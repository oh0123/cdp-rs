// Auto-generated from Chrome at version 146.0.7680.165 domain: DOMDebugger
#![allow(dead_code)]
use super::dom;
use super::runtime;
#[allow(unused_imports)]
use super::types::*;
#[allow(unused_imports)]
use derive_builder::Builder;
#[allow(unused_imports)]
use serde::{Deserialize, Serialize};
#[allow(unused_imports)]
use serde_json::Value as Json;
#[allow(deprecated)]
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub enum DomBreakpointType {
    #[serde(rename = "subtree-modified")]
    SubtreeModified,
    #[serde(rename = "attribute-modified")]
    AttributeModified,
    #[serde(rename = "node-removed")]
    NodeRemoved,
}
#[allow(deprecated)]
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub enum CspViolationType {
    #[serde(rename = "trustedtype-sink-violation")]
    TrustedtypeSinkViolation,
    #[serde(rename = "trustedtype-policy-violation")]
    TrustedtypePolicyViolation,
}
#[allow(deprecated)]
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "camelCase")]
#[doc = "Object event listener."]
pub struct EventListener {
    #[serde(default)]
    #[doc = "`EventListener`'s type."]
    pub r#type: String,
    #[serde(default)]
    #[doc = "`EventListener`'s useCapture."]
    pub use_capture: bool,
    #[serde(default)]
    #[doc = "`EventListener`'s passive flag."]
    pub passive: bool,
    #[serde(default)]
    #[doc = "`EventListener`'s once flag."]
    pub once: bool,
    #[doc = "Script id of the handler code."]
    pub script_id: runtime::ScriptId,
    #[serde(default)]
    #[doc = "Line number in the script (0-based)."]
    pub line_number: JsUInt,
    #[serde(default)]
    #[doc = "Column number in the script (0-based)."]
    pub column_number: JsUInt,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[doc = "Event handler function value."]
    pub handler: Option<runtime::RemoteObject>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[doc = "Event original handler function value."]
    pub original_handler: Option<runtime::RemoteObject>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[doc = "Node the listener is added to (if any)."]
    pub backend_node_id: Option<dom::BackendNodeId>,
}
#[allow(deprecated)]
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "camelCase")]
#[doc = "Returns event listeners of the given object."]
pub struct GetEventListeners {
    #[doc = "Identifier of the object to return listeners for."]
    pub object_id: runtime::RemoteObjectId,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[doc = "The maximum depth at which Node children should be retrieved, defaults to 1. Use -1 for the\n entire subtree or provide an integer larger than 0."]
    pub depth: Option<JsUInt>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[doc = "Whether or not iframes and shadow roots should be traversed when returning the subtree\n (default is false). Reports listeners for all contexts if pierce is enabled."]
    pub pierce: Option<bool>,
}
#[allow(deprecated)]
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "camelCase")]
#[doc = "Removes DOM breakpoint that was set using `setDOMBreakpoint`."]
pub struct RemoveDOMBreakpoint {
    #[doc = "Identifier of the node to remove breakpoint from."]
    pub node_id: dom::NodeId,
    #[doc = "Type of the breakpoint to remove."]
    pub r#type: DomBreakpointType,
}
#[allow(deprecated)]
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "camelCase")]
#[doc = "Removes breakpoint on particular DOM event."]
pub struct RemoveEventListenerBreakpoint {
    #[serde(default)]
    #[doc = "Event name."]
    pub event_name: String,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[doc = "EventTarget interface name."]
    pub target_name: Option<String>,
}
#[allow(deprecated)]
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "camelCase")]
#[doc = "Removes breakpoint on particular native event."]
#[deprecated]
pub struct RemoveInstrumentationBreakpoint {
    #[serde(default)]
    #[doc = "Instrumentation name to stop on."]
    pub event_name: String,
}
#[allow(deprecated)]
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "camelCase")]
#[doc = "Removes breakpoint from XMLHttpRequest."]
pub struct RemoveXHRBreakpoint {
    #[serde(default)]
    #[doc = "Resource URL substring."]
    pub url: String,
}
#[allow(deprecated)]
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "camelCase")]
#[doc = "Sets breakpoint on particular CSP violations."]
pub struct SetBreakOnCSPViolation {
    #[doc = "CSP Violations to stop upon."]
    pub violation_types: Vec<CspViolationType>,
}
#[allow(deprecated)]
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "camelCase")]
#[doc = "Sets breakpoint on particular operation with DOM."]
pub struct SetDOMBreakpoint {
    #[doc = "Identifier of the node to set breakpoint on."]
    pub node_id: dom::NodeId,
    #[doc = "Type of the operation to stop upon."]
    pub r#type: DomBreakpointType,
}
#[allow(deprecated)]
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "camelCase")]
#[doc = "Sets breakpoint on particular DOM event."]
pub struct SetEventListenerBreakpoint {
    #[serde(default)]
    #[doc = "DOM Event name to stop on (any DOM event will do)."]
    pub event_name: String,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[doc = "EventTarget interface name to stop on. If equal to `\"*\"` or not provided, will stop on any\n EventTarget."]
    pub target_name: Option<String>,
}
#[allow(deprecated)]
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "camelCase")]
#[doc = "Sets breakpoint on particular native event."]
#[deprecated]
pub struct SetInstrumentationBreakpoint {
    #[serde(default)]
    #[doc = "Instrumentation name to stop on."]
    pub event_name: String,
}
#[allow(deprecated)]
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "camelCase")]
#[doc = "Sets breakpoint on XMLHttpRequest."]
pub struct SetXHRBreakpoint {
    #[serde(default)]
    #[doc = "Resource URL substring. All XHRs having this substring in the URL will get stopped upon."]
    pub url: String,
}
#[allow(deprecated)]
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[serde(rename_all = "camelCase")]
#[doc = "Returns event listeners of the given object."]
pub struct GetEventListenersReturnObject {
    #[doc = "Array of relevant listeners."]
    pub listeners: Vec<EventListener>,
}
#[allow(deprecated)]
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[doc = "Removes DOM breakpoint that was set using `setDOMBreakpoint`."]
pub struct RemoveDOMBreakpointReturnObject(pub Option<Json>);
#[allow(deprecated)]
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[doc = "Removes breakpoint on particular DOM event."]
pub struct RemoveEventListenerBreakpointReturnObject(pub Option<Json>);
#[allow(deprecated)]
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[doc = "Removes breakpoint on particular native event."]
#[deprecated]
pub struct RemoveInstrumentationBreakpointReturnObject(pub Option<Json>);
#[allow(deprecated)]
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[doc = "Removes breakpoint from XMLHttpRequest."]
pub struct RemoveXHRBreakpointReturnObject(pub Option<Json>);
#[allow(deprecated)]
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[doc = "Sets breakpoint on particular CSP violations."]
pub struct SetBreakOnCSPViolationReturnObject(pub Option<Json>);
#[allow(deprecated)]
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[doc = "Sets breakpoint on particular operation with DOM."]
pub struct SetDOMBreakpointReturnObject(pub Option<Json>);
#[allow(deprecated)]
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[doc = "Sets breakpoint on particular DOM event."]
pub struct SetEventListenerBreakpointReturnObject(pub Option<Json>);
#[allow(deprecated)]
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[doc = "Sets breakpoint on particular native event."]
#[deprecated]
pub struct SetInstrumentationBreakpointReturnObject(pub Option<Json>);
#[allow(deprecated)]
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[doc = "Sets breakpoint on XMLHttpRequest."]
pub struct SetXHRBreakpointReturnObject(pub Option<Json>);
#[allow(deprecated)]
impl Method for GetEventListeners {
    const NAME: &'static str = "DOMDebugger.getEventListeners";
    type ReturnObject = GetEventListenersReturnObject;
}
#[allow(deprecated)]
impl Method for RemoveDOMBreakpoint {
    const NAME: &'static str = "DOMDebugger.removeDOMBreakpoint";
    type ReturnObject = RemoveDOMBreakpointReturnObject;
}
#[allow(deprecated)]
impl Method for RemoveEventListenerBreakpoint {
    const NAME: &'static str = "DOMDebugger.removeEventListenerBreakpoint";
    type ReturnObject = RemoveEventListenerBreakpointReturnObject;
}
#[allow(deprecated)]
impl Method for RemoveInstrumentationBreakpoint {
    const NAME: &'static str = "DOMDebugger.removeInstrumentationBreakpoint";
    type ReturnObject = RemoveInstrumentationBreakpointReturnObject;
}
#[allow(deprecated)]
impl Method for RemoveXHRBreakpoint {
    const NAME: &'static str = "DOMDebugger.removeXHRBreakpoint";
    type ReturnObject = RemoveXHRBreakpointReturnObject;
}
#[allow(deprecated)]
impl Method for SetBreakOnCSPViolation {
    const NAME: &'static str = "DOMDebugger.setBreakOnCSPViolation";
    type ReturnObject = SetBreakOnCSPViolationReturnObject;
}
#[allow(deprecated)]
impl Method for SetDOMBreakpoint {
    const NAME: &'static str = "DOMDebugger.setDOMBreakpoint";
    type ReturnObject = SetDOMBreakpointReturnObject;
}
#[allow(deprecated)]
impl Method for SetEventListenerBreakpoint {
    const NAME: &'static str = "DOMDebugger.setEventListenerBreakpoint";
    type ReturnObject = SetEventListenerBreakpointReturnObject;
}
#[allow(deprecated)]
impl Method for SetInstrumentationBreakpoint {
    const NAME: &'static str = "DOMDebugger.setInstrumentationBreakpoint";
    type ReturnObject = SetInstrumentationBreakpointReturnObject;
}
#[allow(deprecated)]
impl Method for SetXHRBreakpoint {
    const NAME: &'static str = "DOMDebugger.setXHRBreakpoint";
    type ReturnObject = SetXHRBreakpointReturnObject;
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
