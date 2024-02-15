use crate::error::HueError;

mod discovery_url;

pub use discovery_url::DiscoveryUrl;

/// This struct identifies a discovered hue bridge.
/// You can convert this struct easly to a `HueBridge`:
/// ```
/// let discovered_hue_bridge = DiscoveredHueBridge {
///     id: "id".to_string(),
///     url: "url".to_string()
/// };
/// let hue_bridge: HueBridge = discovered_hue_bridge.into();
/// ```
#[derive(Clone)]
pub struct DiscoveredHueBridge {
    pub id: String,
    pub url: String,
}

/// If you don't know the bridge url of your `HueBridge` it's possible to discover it. There are several methods to discover a hue bridge. Currently just one is implemented:
/// - `DiscoveryUrl`
#[async_trait::async_trait]
pub trait Discover {
    /// Discover multiple hue bridges
    async fn discover() -> Result<Vec<DiscoveredHueBridge>, HueError>;
    /// Discover just one `HueBridge` and throw an `HueError::NoBridgeFound` error if no bridge was found
    async fn discover_one() -> Result<DiscoveredHueBridge, HueError> {
        Self::discover()
            .await?
            .first()
            .cloned()
            .ok_or(HueError::NoBridgeFound)
    }
}
