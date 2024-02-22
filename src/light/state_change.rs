use serde::Serialize;

use crate::{error::HueError, Hue};

impl Hue {
    /// Change the state of a light
    /// ```
    /// use philipshue::{Hue, HueBridge, light::StateChange};
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let hue = Hue::new(HueBridge::new("url".to_string()), "username".to_string());
    ///     hue.set_light_state(
    ///         "1", // we know the light number from the lights request for example
    ///         StateChange::new()
    ///             .on(true) // turn the light on
    ///             .bri(254) // this is the maximal brightness
    ///     ).await.unwrap();
    /// }
    /// ```
    pub async fn set_light_state(
        &self,
        light_number: &str,
        state_change: StateChange,
    ) -> Result<(), HueError> {
        self.bridge
            .parse_response::<serde_json::Value>(
                self.bridge
                    .client()
                    .put(format!(
                        "{}/lights/{light_number}/state",
                        self.get_username_url()
                    ))
                    .json(&state_change)
                    .send()
                    .await?,
            )
            .await?;
        Ok(())
    }
}

/// Build `StateChange` objects to change the state of a `HueLight`
/// ```
/// use philipshue::light::StateChange;
///
/// let change = StateChange::new()
///     .on(true) // turn the light on
///     .bri(254) // set the brightness to 254 (maximum)
///     .sat(123) // set the saturation to 123
///     .hue(123); // set the hue to 123
/// ```
#[derive(Default, Clone, Serialize)]
pub struct StateChange {
    #[serde(rename = "on", skip_serializing_if = "Option::is_none")]
    value_on: Option<bool>,
    #[serde(rename = "sat", skip_serializing_if = "Option::is_none")]
    value_sat: Option<u8>,
    #[serde(rename = "bri", skip_serializing_if = "Option::is_none")]
    value_bri: Option<u8>,
    #[serde(rename = "hue", skip_serializing_if = "Option::is_none")]
    value_hue: Option<u16>,
    #[serde(rename = "transitiontime", skip_serializing_if = "Option::is_none")]
    value_transition_time: Option<u16>,
}

impl StateChange {
    pub fn new() -> Self {
        Self::default()
    }

    /// Returns `true` if no value was set (every value is `None`)
    pub fn is_empty(&self) -> bool {
        self.value_on.is_none()
            && self.value_sat.is_none()
            && self.value_bri.is_none()
            && self.value_hue.is_none()
            && self.value_transition_time.is_none()
    }

    pub fn on(mut self, on: bool) -> Self {
        self.value_on = Some(on);
        self
    }

    pub fn sat(mut self, sat: u8) -> Self {
        self.value_sat = Some(sat);
        self
    }

    pub fn bri(mut self, bri: u8) -> Self {
        self.value_bri = Some(bri);
        self
    }

    pub fn hue(mut self, hue: u16) -> Self {
        self.value_hue = Some(hue);
        self
    }

    pub fn transition_time(mut self, transition_time: u16) -> Self {
        self.value_transition_time = Some(transition_time);
        self
    }
}
