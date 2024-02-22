use std::fmt::Display;

use serde::Deserialize;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum HueError {
    #[error("error while doing the request to the philipshue bridge: {0}")]
    RequestError(#[from] reqwest::Error),
    /// The request was successful but the server replied with an error, you can find information about the error in the `ApiError` struct
    #[error("the philipshue bridge replied with an error: {0}")]
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

impl Display for ApiError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "error_type = {}, ", self.error_type)?;
        if !self.address.is_empty() {
            write!(f, "address = {}, ", self.address)?;
        }
        write!(f, "description = {}", self.description)?;
        Ok(())
    }
}
