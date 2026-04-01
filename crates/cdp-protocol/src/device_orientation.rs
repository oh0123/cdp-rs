// Auto-generated from Chrome at version 146.0.7680.165 domain: DeviceOrientation
#[allow(unused_imports)]
use super::types::*;
#[allow(unused_imports)]
use derive_builder::Builder;
#[allow(unused_imports)]
use serde::{Deserialize, Serialize};
#[allow(unused_imports)]
use serde_json::Value as Json;
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct ClearDeviceOrientationOverride(pub Option<Json>);
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "camelCase")]
#[doc = "Overrides the Device Orientation."]
pub struct SetDeviceOrientationOverride {
    #[serde(default)]
    #[doc = "Mock alpha"]
    pub alpha: JsFloat,
    #[serde(default)]
    #[doc = "Mock beta"]
    pub beta: JsFloat,
    #[serde(default)]
    #[doc = "Mock gamma"]
    pub gamma: JsFloat,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[doc = "Clears the overridden Device Orientation."]
pub struct ClearDeviceOrientationOverrideReturnObject(pub Option<Json>);
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[doc = "Overrides the Device Orientation."]
pub struct SetDeviceOrientationOverrideReturnObject(pub Option<Json>);
impl Method for ClearDeviceOrientationOverride {
    const NAME: &'static str = "DeviceOrientation.clearDeviceOrientationOverride";
    type ReturnObject = ClearDeviceOrientationOverrideReturnObject;
}
impl Method for SetDeviceOrientationOverride {
    const NAME: &'static str = "DeviceOrientation.setDeviceOrientationOverride";
    type ReturnObject = SetDeviceOrientationOverrideReturnObject;
}
pub mod events {
    #[allow(unused_imports)]
    use super::super::types::*;
    #[allow(unused_imports)]
    use derive_builder::Builder;
    #[allow(unused_imports)]
    use serde::{Deserialize, Serialize};
    #[allow(unused_imports)]
    use serde_json::Value as Json;
}
