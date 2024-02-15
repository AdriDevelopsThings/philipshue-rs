use std::collections::HashMap;

use crate::{error::HueError, Hue};

mod model;
mod state_change;

pub use model::*;
pub use state_change::*;

impl Hue {
    /// Get a `HashMap` of all lights the hue bridge know. The key of the `HashMap` identifies the id of the `HueLight`
    pub async fn lights(&self) -> Result<HashMap<String, HueLight>, HueError> {
        Ok(self
            .bridge
            .client()
            .get(format!("{}/lights", self.get_username_url()))
            .send()
            .await?
            .error_for_status()?
            .json()
            .await?)
    }

    /// Get one specific light from it's `light_number`
    pub async fn get_light(&self, light_number: &str) -> Result<HueLight, HueError> {
        Ok(self
            .bridge
            .client()
            .get(format!("{}/lights/{light_number}", self.get_username_url()))
            .send()
            .await?
            .error_for_status()?
            .json()
            .await?)
    }
}
