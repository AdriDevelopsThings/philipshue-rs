use serde::Deserialize;

use crate::{bridge::HueBridge, device::DeviceType, error::HueError, Hue};

/// The server just returns the `username` (something like an authorization token)
#[derive(Deserialize)]
pub struct LoginResponse {
    pub username: String,
}

impl HueBridge {
    /// Login with `DeviceType` and return a `Hue` object instead of a `LoginResponse`
    /// `HueError::ApiError` with `ApiError::error_type = 101` (link button not pressed) could be thrown
    pub async fn login_to_hue(self, device_type: DeviceType) -> Result<Hue, HueError> {
        let username = self.login_with_response(device_type).await?.username;
        Ok(Hue::new(self, username))
    }

    /// Login with `DeviceType` and return a `LoginResponse`
    /// `HueError::ApiError` with `ApiError::error_type = 101` (link button not pressed) could be thrown
    pub async fn login_with_response(
        &self,
        device_type: DeviceType,
    ) -> Result<LoginResponse, HueError> {
        self.parse_response(
            self.client()
                .post(format!("{}/api", self.bridge_url))
                .json(&device_type)
                .send()
                .await?,
        )
        .await
    }
}
