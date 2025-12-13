// Auto-generated from Chrome at version 143.0.7499.110 domain: DOMDebugger
use super::dom;
use super::runtime;
#[allow(unused_imports)]
use super::types::*;
#[allow(unused_imports)]
use serde::{Deserialize, Serialize};
#[allow(unused_imports)]
use serde_json::Value as Json;
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub enum DomBreakpointType {
    #[serde(rename = "subtree-modified")]
    SubtreeModified,
    #[serde(rename = "attribute-modified")]
    AttributeModified,
    #[serde(rename = "node-removed")]
    NodeRemoved,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub enum CspViolationType {
    #[serde(rename = "trustedtype-sink-violation")]
    TrustedtypeSinkViolation,
    #[serde(rename = "trustedtype-policy-violation")]
    TrustedtypePolicyViolation,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct EventListener {
    #[serde(default)]
    #[serde(rename = "type")]
    pub r#type: String,
    #[serde(default)]
    #[serde(rename = "useCapture")]
    pub use_capture: bool,
    #[serde(default)]
    #[serde(rename = "passive")]
    pub passive: bool,
    #[serde(default)]
    #[serde(rename = "once")]
    pub once: bool,
    #[serde(rename = "scriptId")]
    pub script_id: runtime::ScriptId,
    #[serde(default)]
    #[serde(rename = "lineNumber")]
    pub line_number: JsUInt,
    #[serde(default)]
    #[serde(rename = "columnNumber")]
    pub column_number: JsUInt,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "handler")]
    pub handler: Option<runtime::RemoteObject>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "originalHandler")]
    pub original_handler: Option<runtime::RemoteObject>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "backendNodeId")]
    pub backend_node_id: Option<dom::BackendNodeId>,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct GetEventListeners {
    #[serde(rename = "objectId")]
    pub object_id: runtime::RemoteObjectId,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(rename = "depth")]
    pub depth: Option<JsUInt>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(rename = "pierce")]
    pub pierce: Option<bool>,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct RemoveDOMBreakpoint {
    #[serde(rename = "nodeId")]
    pub node_id: dom::NodeId,
    #[serde(rename = "type")]
    pub r#type: DomBreakpointType,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct RemoveEventListenerBreakpoint {
    #[serde(default)]
    #[serde(rename = "eventName")]
    pub event_name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(rename = "targetName")]
    pub target_name: Option<String>,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct RemoveInstrumentationBreakpoint {
    #[serde(default)]
    #[serde(rename = "eventName")]
    pub event_name: String,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct RemoveXHRBreakpoint {
    #[serde(default)]
    #[serde(rename = "url")]
    pub url: String,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct SetBreakOnCSPViolation {
    #[serde(rename = "violationTypes")]
    pub violation_types: Vec<CspViolationType>,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct SetDOMBreakpoint {
    #[serde(rename = "nodeId")]
    pub node_id: dom::NodeId,
    #[serde(rename = "type")]
    pub r#type: DomBreakpointType,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct SetEventListenerBreakpoint {
    #[serde(default)]
    #[serde(rename = "eventName")]
    pub event_name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(rename = "targetName")]
    pub target_name: Option<String>,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct SetInstrumentationBreakpoint {
    #[serde(default)]
    #[serde(rename = "eventName")]
    pub event_name: String,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct SetXHRBreakpoint {
    #[serde(default)]
    #[serde(rename = "url")]
    pub url: String,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct GetEventListenersReturnObject {
    #[serde(rename = "listeners")]
    pub listeners: Vec<EventListener>,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct RemoveDOMBreakpointReturnObject {}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct RemoveEventListenerBreakpointReturnObject {}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct RemoveInstrumentationBreakpointReturnObject {}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct RemoveXHRBreakpointReturnObject {}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct SetBreakOnCSPViolationReturnObject {}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct SetDOMBreakpointReturnObject {}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct SetEventListenerBreakpointReturnObject {}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct SetInstrumentationBreakpointReturnObject {}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct SetXHRBreakpointReturnObject {}
impl Method for GetEventListeners {
    const NAME: &'static str = "DOMDebugger.getEventListeners";
    type ReturnObject = GetEventListenersReturnObject;
}
impl Method for RemoveDOMBreakpoint {
    const NAME: &'static str = "DOMDebugger.removeDOMBreakpoint";
    type ReturnObject = RemoveDOMBreakpointReturnObject;
}
impl Method for RemoveEventListenerBreakpoint {
    const NAME: &'static str = "DOMDebugger.removeEventListenerBreakpoint";
    type ReturnObject = RemoveEventListenerBreakpointReturnObject;
}
impl Method for RemoveInstrumentationBreakpoint {
    const NAME: &'static str = "DOMDebugger.removeInstrumentationBreakpoint";
    type ReturnObject = RemoveInstrumentationBreakpointReturnObject;
}
impl Method for RemoveXHRBreakpoint {
    const NAME: &'static str = "DOMDebugger.removeXHRBreakpoint";
    type ReturnObject = RemoveXHRBreakpointReturnObject;
}
impl Method for SetBreakOnCSPViolation {
    const NAME: &'static str = "DOMDebugger.setBreakOnCSPViolation";
    type ReturnObject = SetBreakOnCSPViolationReturnObject;
}
impl Method for SetDOMBreakpoint {
    const NAME: &'static str = "DOMDebugger.setDOMBreakpoint";
    type ReturnObject = SetDOMBreakpointReturnObject;
}
impl Method for SetEventListenerBreakpoint {
    const NAME: &'static str = "DOMDebugger.setEventListenerBreakpoint";
    type ReturnObject = SetEventListenerBreakpointReturnObject;
}
impl Method for SetInstrumentationBreakpoint {
    const NAME: &'static str = "DOMDebugger.setInstrumentationBreakpoint";
    type ReturnObject = SetInstrumentationBreakpointReturnObject;
}
impl Method for SetXHRBreakpoint {
    const NAME: &'static str = "DOMDebugger.setXHRBreakpoint";
    type ReturnObject = SetXHRBreakpointReturnObject;
}
pub mod events {
    #[allow(unused_imports)]
    use super::super::types::*;
    #[allow(unused_imports)]
    use serde::{Deserialize, Serialize};
}
