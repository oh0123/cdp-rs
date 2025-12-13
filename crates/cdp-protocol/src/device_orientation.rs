// Auto-generated from Chrome at version 143.0.7499.110 domain: DeviceOrientation
#[allow(unused_imports)]
use super::types::*;
#[allow(unused_imports)]
use serde::{Deserialize, Serialize};
#[allow(unused_imports)]
use serde_json::Value as Json;
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct ClearDeviceOrientationOverride(pub Option<serde_json::Value>);
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct SetDeviceOrientationOverride {
    #[serde(default)]
    #[serde(rename = "alpha")]
    pub alpha: JsFloat,
    #[serde(default)]
    #[serde(rename = "beta")]
    pub beta: JsFloat,
    #[serde(default)]
    #[serde(rename = "gamma")]
    pub gamma: JsFloat,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct ClearDeviceOrientationOverrideReturnObject {}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct SetDeviceOrientationOverrideReturnObject {}
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
    use serde::{Deserialize, Serialize};
}
