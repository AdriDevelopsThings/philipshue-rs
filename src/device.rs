use serde::Serialize;

/// It's necessary to use the `DeviceType` object to identify you. Just create one with any string.
/// ```
/// use philipshue::DeviceType;
///
/// let device_type = DeviceType::new("my_device_name".to_string());
/// ```
#[derive(Serialize)]
pub struct DeviceType {
    #[serde(rename = "devicetype")]
    pub device_type: String,
}

impl DeviceType {
    pub fn new(device_type: String) -> Self {
        Self { device_type }
    }
}
