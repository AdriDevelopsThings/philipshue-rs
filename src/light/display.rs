use std::fmt;

use super::{
    Capabilities, CapabilitiesControl, CapabilitiesControlCT, CapabilitiesStreaming, HueLight,
    HueLightConfig, HueLightSoftwareUpdate, HueLightState,
};

impl fmt::Display for HueLight {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // add root elements
        writeln!(f, "Name: {}", self.name)?;
        writeln!(f, "Type: {}", self._type)?;
        writeln!(f, "Model id: {}", self.model_id)?;
        writeln!(f, "Manufacturer: {}", self.manufacturer_name)?;
        writeln!(f, "Product: {}", self.product_name)?;
        writeln!(f, "Unique id: {}", self.unique_id)?;
        writeln!(f, "Software version: {}", self.software_version)?;

        // software update
        writeln!(f, "Software update:")?;
        self.software_update.fmt(f)?;
        writeln!(f)?;

        // config
        writeln!(f, "Config:")?;
        self.config.fmt(f)?;
        writeln!(f)?;

        // light state
        writeln!(f, "--- Light state: --- ")?;
        self.state.fmt(f)?;
        Ok(())
    }
}

impl fmt::Display for HueLightSoftwareUpdate {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "State: {}", self.state)?;
        writeln!(f, "Last install: {}", self.last_install)?;
        Ok(())
    }
}

impl fmt::Display for HueLightConfig {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "Archetype: {}", self.archetype)?;
        writeln!(f, "Function: {}", self.function)?;
        writeln!(f, "Direction: {}", self.direction)?;
        Ok(())
    }
}

impl fmt::Display for HueLightState {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "On: {}", self.on)?;
        if let Some(bri) = self.bri {
            writeln!(f, "Brightness: {}", bri)?;
        }
        if let Some(hue) = self.hue {
            writeln!(f, "Hue: {}", hue)?;
        }
        if let Some(sat) = self.sat {
            writeln!(f, "Saturation: {}", sat)?;
        }
        if let Some(ct) = self.ct {
            writeln!(f, "Color temperature: {}", ct)?;
        }
        writeln!(f, "Alert: {}", self.alert)?;
        if let Some(colormode) = &self.colormode {
            writeln!(f, "Colormode: {}", colormode)?;
        }
        writeln!(f, "Mode: {}", self.mode)?;
        writeln!(f, "Reachable: {}", self.reachable)?;
        Ok(())
    }
}

impl fmt::Display for Capabilities {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "Certified: {}", self.certified)?;
        if self.control.ct.is_some() {
            writeln!(f, "Control capabilities:")?;
            self.control.fmt(f)?;
            writeln!(f)?;
        }
        writeln!(f, "Streaming capabilities:")?;
        self.streaming.fmt(f)?;
        Ok(())
    }
}

impl fmt::Display for CapabilitiesControl {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if let Some(CapabilitiesControlCT { min, max }) = self.ct {
            writeln!(f, "Color temperature min: {}, max: {}", min, max)?;
        }
        Ok(())
    }
}

impl fmt::Display for CapabilitiesStreaming {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "Renderer: {}", self.renderer)?;
        writeln!(f, "Proxy: {}", self.proxy)?;
        Ok(())
    }
}
