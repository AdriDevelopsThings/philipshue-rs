use serde::Deserialize;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum HueError {
    #[error("error while doing the request to the philipshue bridge")]
    RequestError(#[from] reqwest::Error),
    /// The request was successful but the server replied with an error, you can find information about the error in the `ApiError` struct
    #[error("the philipshue bridge replied with an error")]
    ApiError(ApiError),
    /// You tried to discover a bridge but no bridge was found
    #[error("no philipshue bridge could be found")]
    NoBridgeFound,
    /// The server should reply with an `success` or an `error` object but none of both were found.
    #[error("the server didn't reply with success or error")]
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
