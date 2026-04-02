// Auto-generated from Chrome at version 146.0.7680.165 domain: EventBreakpoints
#![allow(dead_code)]
#[allow(unused_imports)]
use super::types::*;
#[allow(unused_imports)]
use derive_builder::Builder;
#[allow(unused_imports)]
use serde::{Deserialize, Serialize};
#[allow(unused_imports)]
use serde_json::Value as Json;
#[allow(deprecated)]
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "camelCase")]
#[doc = "Sets breakpoint on particular native event."]
pub struct SetInstrumentationBreakpoint {
    #[serde(default)]
    #[doc = "Instrumentation name to stop on."]
    pub event_name: String,
}
#[allow(deprecated)]
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "camelCase")]
#[doc = "Removes breakpoint on particular native event."]
pub struct RemoveInstrumentationBreakpoint {
    #[serde(default)]
    #[doc = "Instrumentation name to stop on."]
    pub event_name: String,
}
#[allow(deprecated)]
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct Disable(pub Option<Json>);
#[allow(deprecated)]
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[doc = "Sets breakpoint on particular native event."]
pub struct SetInstrumentationBreakpointReturnObject(pub Option<Json>);
#[allow(deprecated)]
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[doc = "Removes breakpoint on particular native event."]
pub struct RemoveInstrumentationBreakpointReturnObject(pub Option<Json>);
#[allow(deprecated)]
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[doc = "Removes all breakpoints"]
pub struct DisableReturnObject(pub Option<Json>);
#[allow(deprecated)]
impl Method for SetInstrumentationBreakpoint {
    const NAME: &'static str = "EventBreakpoints.setInstrumentationBreakpoint";
    type ReturnObject = SetInstrumentationBreakpointReturnObject;
}
#[allow(deprecated)]
impl Method for RemoveInstrumentationBreakpoint {
    const NAME: &'static str = "EventBreakpoints.removeInstrumentationBreakpoint";
    type ReturnObject = RemoveInstrumentationBreakpointReturnObject;
}
#[allow(deprecated)]
impl Method for Disable {
    const NAME: &'static str = "EventBreakpoints.disable";
    type ReturnObject = DisableReturnObject;
}
#[allow(dead_code)]
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
