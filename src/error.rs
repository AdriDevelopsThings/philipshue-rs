use serde::Deserialize;

#[derive(Debug)]
pub enum HueError {
    RequestError(reqwest::Error),
    /// The request was successful but the server replied with an error, you can find information about the error in the `ApiError` struct
    ApiError(ApiError),
    /// You tried to discover a bridge but no bridge was found
    NoBridgeFound,
    /// The server should reply with an `success` or an `error` object but none of both were found.
    NoData,
}

#[derive(Debug, Deserialize)]
pub struct ApiError {
    #[serde(rename = "type")]
    pub error_type: u16,
    /// I don't know what this field is used for
    pub address: String,
    pub description: String,
}

impl From<reqwest::Error> for HueError {
    fn from(value: reqwest::Error) -> Self {
        Self::RequestError(value)
    }
}
