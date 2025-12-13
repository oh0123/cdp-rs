// Auto-generated from Chrome at version 143.0.7499.110 domain: EventBreakpoints
#[allow(unused_imports)]
use super::types::*;
#[allow(unused_imports)]
use serde::{Deserialize, Serialize};
#[allow(unused_imports)]
use serde_json::Value as Json;
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct SetInstrumentationBreakpoint {
    #[serde(default)]
    #[serde(rename = "eventName")]
    pub event_name: String,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct RemoveInstrumentationBreakpoint {
    #[serde(default)]
    #[serde(rename = "eventName")]
    pub event_name: String,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Disable(pub Option<serde_json::Value>);
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct SetInstrumentationBreakpointReturnObject {}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct RemoveInstrumentationBreakpointReturnObject {}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct DisableReturnObject {}
impl Method for SetInstrumentationBreakpoint {
    const NAME: &'static str = "EventBreakpoints.setInstrumentationBreakpoint";
    type ReturnObject = SetInstrumentationBreakpointReturnObject;
}
impl Method for RemoveInstrumentationBreakpoint {
    const NAME: &'static str = "EventBreakpoints.removeInstrumentationBreakpoint";
    type ReturnObject = RemoveInstrumentationBreakpointReturnObject;
}
impl Method for Disable {
    const NAME: &'static str = "EventBreakpoints.disable";
    type ReturnObject = DisableReturnObject;
}
pub mod events {
    #[allow(unused_imports)]
    use super::super::types::*;
    #[allow(unused_imports)]
    use serde::{Deserialize, Serialize};
}
