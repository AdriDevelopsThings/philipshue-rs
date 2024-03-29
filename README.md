# philipshue
Just another philipshue implementation in rust

## Creating the bridge struct
If you know the philips hue url just use this method.
```rust
use philipshue::HueBridge;

let bridge = HueBridge::new("https://philips-hue.local".to_string());
```
If you don't know the url you can discover the bridge, but be careful the http endpoint is ratelimited so discover the bridge just one time and save the bridge url after the discovery.
```rust
use philipshue::{HueBridge, Discover, DiscoveryUrl};

#[tokio::main]
async fn main() {
    let bridge: HueBridge = DiscoveryUrl::discover_one().await.unwrap().into();
}
```

## Authorization
First define your device type. This is a freely selectable string that identifies your device. After this creating you can login.
```rust
use philipshue::{HueBridge, Hue, DeviceType};

#[tokio::main]
async fn main() {
    let device_type = DeviceType::new("your_device_name".to_string());
    let bridge = HueBridge::new("https://philips-hue.local".to_string());
    let hue: Hue = bridge.login_to_hue(device_type).await.unwrap();
}
```
The login statement may run into an error: `HueError::ApiError(ApiError)`. If `ApiError::error_type` is `101` the error means that the user didn't press the link button. You can easly rerun this function after the user clicked the button. The hue bridge will recognise you from the `DeviceType`. If your login was successful you get a `Hue` struct. Save the `hue.username` (it's something like an authorization token) for the next time.

## Getting information about lights
```rust
use philipshue::{HueBridge, Hue, light::HueLight};
use std::collections::HashMap;

#[tokio::main]
async fn main() {
    let hue = Hue::new(HueBridge::new("https://philips-hue.local".to_string()), "username".to_string());
    let lights: HashMap<String, HueLight> = hue.lights().await.unwrap();
    for (light_number, light) in lights {
        println!("The light number {} and name {} is {}", light_number, light.name, if light.state.on { "on "} else { "off" });
    }
}
```

Or get information just about one light by using `Hue::get_light(light_number)`.

If you need more information about the data of a light take a look to the documentation.

## Changing the state of a light
```rust
use philipshue::{HueBridge, Hue, light::StateChange};

#[tokio::main]
async fn main() {
    let light_number = "1";
    let hue = Hue::new(HueBridge::new("https://philips-hue.local".to_string()), "username".to_string());
    hue.set_light_state(light_number, StateChange::new().on(true)).await.unwrap(); // turn the light on
}
```

It's also possible to change the brightness, saturation and hue of the light by using the `StateChange` builder.

# License
Licensed under either of

 * Apache License, Version 2.0
   ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license
   ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)


at your option.