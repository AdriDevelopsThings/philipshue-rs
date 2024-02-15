use serde::Deserialize;

use crate::{Discover, DiscoveredHueBridge, HueError};

const DISCOVER_ADDRESS: &str = "https://discovery.meethue.com";

#[derive(Deserialize)]
struct DiscoveryUrlRespones {
    pub id: String,
    #[serde(rename = "internalipaddress")]
    pub internal_ip_address: String,
    pub port: u16,
}

impl From<DiscoveryUrlRespones> for DiscoveredHueBridge {
    fn from(value: DiscoveryUrlRespones) -> Self {
        Self {
            id: value.id,
            url: match value.port {
                443 => format!("https://{}", value.internal_ip_address), // tls encrypted
                _ => format!("http://{}:{}", value.internal_ip_address, value.port),
            },
        }
    }
}

/// Discover a hue bridge using the `https://discovery.meethue.com` url.
/// CAUTION: This url has an rate limit. So run this discovery just one time and save the `bridge_url` of `HueBridge`.
/// ```
/// use philipshue::{DiscoveredHueBridge, DiscoveryUrl, Discover};
///
/// #[tokio::main]
/// async fn main() {
///     let discovered_hue_bridges: Vec<DiscoveredHueBridge> = DiscoveryUrl::discover().await.unwrap();
/// }
/// ```
pub struct DiscoveryUrl;

#[async_trait::async_trait]
impl Discover for DiscoveryUrl {
    /// This discover function may throw an `HueError::RequestError` error.
    async fn discover() -> Result<Vec<DiscoveredHueBridge>, HueError> {
        Ok(reqwest::get(DISCOVER_ADDRESS)
            .await?
            .error_for_status()?
            .json::<Vec<DiscoveryUrlRespones>>()
            .await?
            .into_iter()
            .map(|response| response.into())
            .collect())
    }
}
