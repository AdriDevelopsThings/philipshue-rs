use serde::Deserialize;

#[derive(Deserialize)]
pub struct HueLight {
    pub state: HueLightState,
    #[serde(rename = "type")]
    pub _type: String,
    pub name: String,
    #[serde(rename = "modelid")]
    pub model_id: String,
    #[serde(rename = "manufacturername")]
    pub manufacturer_name: String,
    #[serde(rename = "productname")]
    pub product_name: String,
    #[serde(rename = "swupdate")]
    pub software_update: HueLightSoftwareUpdate,
    #[serde(rename = "uniqueid")]
    pub unique_id: String,
    #[serde(rename = "swversion")]
    pub software_version: String,
    pub capabilities: Capabilities,
    pub config: HueLightConfig,
}

#[derive(Deserialize)]
pub struct HueLightState {
    pub on: bool,
    pub bri: u8,
    pub hue: Option<u16>,
    pub sat: Option<u8>,
    pub ct: Option<u16>,
    pub alert: String,
    pub colormode: Option<String>,
    pub mode: String,
    pub reachable: bool,
}

#[derive(Deserialize)]
pub struct HueLightSoftwareUpdate {
    pub state: String,
    #[serde(rename = "lastinstall")]
    pub last_install: String,
}

#[derive(Deserialize)]
pub struct Capabilities {
    pub certified: bool,
    pub control: CapabilitiesControl,
    pub streaming: CapabilitiesStreaming,
}

#[derive(Deserialize)]
pub struct CapabilitiesControl {
    pub ct: Option<CapabilitiesControlCT>,
}

#[derive(Deserialize)]
pub struct CapabilitiesControlCT {
    pub min: u16,
    pub max: u16,
}

#[derive(Deserialize)]
pub struct CapabilitiesStreaming {
    pub renderer: bool,
    pub proxy: bool,
}

#[derive(Deserialize)]
pub struct HueLightConfig {
    pub archetype: String,
    pub function: String,
    pub direction: String,
}
