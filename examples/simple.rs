use std::env;

use philipshue::{
    light::StateChange, DeviceType, Discover, DiscoveryUrl, Hue, HueBridge, HueError,
};

#[tokio::main]
async fn main() {
    // get the device_type from the environment variable, this value shouldn't be changed
    let device_type =
        DeviceType::new(env::var("HUE_DEVICE_TYPE").unwrap_or_else(|_| "testdevice".to_string()));

    // get the bridge by it's bridge url saved in the environment or discover it
    let bridge: HueBridge = match env::var("HUE_BRIDGE_URL") {
        Ok(bridge_url) => HueBridge::new(bridge_url),
        Err(_) => {
            // discover bridge; CAUTION: This discovery method has a rate limit so just discover a hue bridge once
            let bridge: HueBridge = DiscoveryUrl::discover_one()
                .await
                .expect("Error while discovering bridge")
                .into();
            println!("Discovered hue bridge: Please set the HUE_BRIDGE_URL environment variable to {} the next time you run this", bridge.bridge_url);
            bridge
        }
    };

    // now authenticate the hue bridge by the username (something like a authorization token)
    let hue = if let Ok(username) = env::var("HUE_USERNAME") {
        Hue::new(bridge, username)
    } else {
        // login
        let hue = match bridge.login_to_hue(device_type).await {
            Ok(hue) => hue, // login request was successful
            Err(HueError::ApiError(api_error)) => {
                // there was an api error while login
                if api_error.error_type == 101 {
                    // this is the error_type if the user didn't press the link button, just rerun the login after he pressed the button
                    panic!(
                        "You have to press the link button of the bridge and restart this script."
                    )
                } else {
                    panic!("Unknown api error: {api_error:?}"); // there was another api error
                }
            }
            Err(e) => panic!("{e:?}"), // there was another error
        };
        println!("Login was successful: Please set the HUE_USERNAME environment variable to {} the next time you run this", hue.username);
        hue
    };

    // get all lights the bridge knows -> HashMap<LightNumber / String, HueLight>
    let lights = hue.lights().await.expect("Error while getting lights");
    for (light_number, light) in lights {
        println!(
            "Light ({}) {} is {}",
            light_number,
            light.name,
            if light.state.on { "on" } else { "off" } // light.state.on is true if the light is on
        );
    }

    // now let us switch a light
    if let Ok(light_number) = env::var("HUE_SWITCH_LIGHT_NUMBER") {
        // get the light to get it's old state
        let light = hue
            .get_light(&light_number)
            .await
            .expect("Error while getting light");
        // now change the state to the negative of the old state (`!light.state.on`)
        hue.set_light_state(
            &light_number,
            StateChange::new()
                .on(!light.state.on)
                .bri(254)
                .transition_time(30),
        )
        .await
        .expect("Error while switching light state");
    } else {
        println!(
            "You can set the HUE_SWITCH_LIGHT_NUMBER enviroment to switch the state of the light."
        )
    }
}
