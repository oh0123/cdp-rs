// Auto-generated from Chrome at version 146.0.7680.165 domain: EventBreakpoints
#[allow(unused_imports)]
use super::types::*;
#[allow(unused_imports)]
use derive_builder::Builder;
#[allow(unused_imports)]
use serde::{Deserialize, Serialize};
#[allow(unused_imports)]
use serde_json::Value as Json;
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "camelCase")]
#[doc = "Sets breakpoint on particular native event."]
pub struct SetInstrumentationBreakpoint {
    #[serde(default)]
    #[doc = "Instrumentation name to stop on."]
    pub event_name: String,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "camelCase")]
#[doc = "Removes breakpoint on particular native event."]
pub struct RemoveInstrumentationBreakpoint {
    #[serde(default)]
    #[doc = "Instrumentation name to stop on."]
    pub event_name: String,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct Disable(pub Option<Json>);
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[doc = "Sets breakpoint on particular native event."]
pub struct SetInstrumentationBreakpointReturnObject(pub Option<Json>);
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[doc = "Removes breakpoint on particular native event."]
pub struct RemoveInstrumentationBreakpointReturnObject(pub Option<Json>);
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[doc = "Removes all breakpoints"]
pub struct DisableReturnObject(pub Option<Json>);
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
    use derive_builder::Builder;
    #[allow(unused_imports)]
    use serde::{Deserialize, Serialize};
    #[allow(unused_imports)]
    use serde_json::Value as Json;
}
