#![doc = include_str!("../README.md")]

mod bridge;
mod device;
mod discover;
mod error;
pub mod light;
pub mod login;

pub use bridge::HueBridge;
pub use device::DeviceType;
pub use discover::*;
pub use error::*;

/// An authenticated `Hue` api object
pub struct Hue {
    pub bridge: HueBridge,
    /// The username is something like an authorization token
    pub username: String,
}

impl Hue {
    pub fn new(bridge: HueBridge, username: String) -> Self {
        Self { bridge, username }
    }

    /// All authorized request paths are {bridge_url}/api/{username}/{path}
    pub(crate) fn get_username_url(&self) -> String {
        format!("{}/api/{}", self.bridge.bridge_url, self.username)
    }
}
