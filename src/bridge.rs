use reqwest::Response;
use serde::{de::DeserializeOwned, Deserialize};

use crate::{
    discover::DiscoveredHueBridge,
    error::{ApiError, HueError},
};

/// The `HueBridge` struct identifes a hue bridge (without `username` or authorization)
/// If you know the bridge url just create the struct like this
/// ```
/// use philipshue::HueBridge;
///
/// let bridge = HueBridge::new("https://philips-hue.local".to_string());
/// ```
#[derive(Clone, Debug)]
pub struct HueBridge {
    pub bridge_url: String,
}

/// Some responses do include a `success` object or an `error` object containing an `ApiError`
#[derive(Deserialize)]
pub(crate) enum DataResponse<T> {
    #[serde(rename = "success")]
    Success(T),
    #[serde(rename = "error")]
    Error(ApiError),
}

/// A `DataResponse<T>` can be transformed to `Result<T, HueError>`
impl<T> From<DataResponse<T>> for Result<T, HueError> {
    fn from(value: DataResponse<T>) -> Self {
        match value {
            DataResponse::Success(value) => Ok(value),
            DataResponse::Error(value) => Err(HueError::ApiError(value)),
        }
    }
}

impl HueBridge {
    pub fn new(bridge_url: String) -> Self {
        Self { bridge_url }
    }

    /// Parse the response from the `Response` struct which includes json which includes a `success` or an `error` object
    pub(crate) async fn parse_response<'a, T>(&self, response: Response) -> Result<T, HueError>
    where
        T: DeserializeOwned,
    {
        response
            .json::<Vec<DataResponse<T>>>()
            .await?
            .into_iter()
            .next()
            .ok_or(HueError::NoData)?
            .into()
    }

    /// We need a custom client that *disables invalid SSL certs* because the hue bridge has a self signed certifcate
    /// The client will have `{cargo package name}/{cargo package version}` as user agent
    pub(crate) fn client(&self) -> reqwest::Client {
        reqwest::ClientBuilder::new()
            .danger_accept_invalid_certs(true)
            .user_agent(concat!(
                env!("CARGO_PKG_NAME"),
                "/",
                env!("CARGO_PKG_VERSION")
            ))
            .build()
            .unwrap()
    }
}

/// A discovered hue bridge can be easily transformed into a `HueBridge`
impl From<DiscoveredHueBridge> for HueBridge {
    fn from(value: DiscoveredHueBridge) -> Self {
        Self::new(value.url)
    }
}
