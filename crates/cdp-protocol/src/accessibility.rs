// Auto-generated from Chrome at version 143.0.7499.110 domain: Accessibility
use super::dom;
use super::page;
use super::runtime;
#[allow(unused_imports)]
use super::types::*;
#[allow(unused_imports)]
use serde::{Deserialize, Serialize};
#[allow(unused_imports)]
use serde_json::Value as Json;
pub type AxNodeId = String;
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub enum AxValueType {
    #[serde(rename = "boolean")]
    Boolean,
    #[serde(rename = "tristate")]
    Tristate,
    #[serde(rename = "booleanOrUndefined")]
    BooleanOrUndefined,
    #[serde(rename = "idref")]
    Idref,
    #[serde(rename = "idrefList")]
    IdrefList,
    #[serde(rename = "integer")]
    Integer,
    #[serde(rename = "node")]
    Node,
    #[serde(rename = "nodeList")]
    NodeList,
    #[serde(rename = "number")]
    Number,
    #[serde(rename = "string")]
    String,
    #[serde(rename = "computedString")]
    ComputedString,
    #[serde(rename = "token")]
    Token,
    #[serde(rename = "tokenList")]
    TokenList,
    #[serde(rename = "domRelation")]
    DomRelation,
    #[serde(rename = "role")]
    Role,
    #[serde(rename = "internalRole")]
    InternalRole,
    #[serde(rename = "valueUndefined")]
    ValueUndefined,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub enum AxValueSourceType {
    #[serde(rename = "attribute")]
    Attribute,
    #[serde(rename = "implicit")]
    Implicit,
    #[serde(rename = "style")]
    Style,
    #[serde(rename = "contents")]
    Contents,
    #[serde(rename = "placeholder")]
    Placeholder,
    #[serde(rename = "relatedElement")]
    RelatedElement,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub enum AxValueNativeSourceType {
    #[serde(rename = "description")]
    Description,
    #[serde(rename = "figcaption")]
    Figcaption,
    #[serde(rename = "label")]
    Label,
    #[serde(rename = "labelfor")]
    Labelfor,
    #[serde(rename = "labelwrapped")]
    Labelwrapped,
    #[serde(rename = "legend")]
    Legend,
    #[serde(rename = "rubyannotation")]
    Rubyannotation,
    #[serde(rename = "tablecaption")]
    Tablecaption,
    #[serde(rename = "title")]
    Title,
    #[serde(rename = "other")]
    Other,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub enum AxPropertyName {
    #[serde(rename = "actions")]
    Actions,
    #[serde(rename = "busy")]
    Busy,
    #[serde(rename = "disabled")]
    Disabled,
    #[serde(rename = "editable")]
    Editable,
    #[serde(rename = "focusable")]
    Focusable,
    #[serde(rename = "focused")]
    Focused,
    #[serde(rename = "hidden")]
    Hidden,
    #[serde(rename = "hiddenRoot")]
    HiddenRoot,
    #[serde(rename = "invalid")]
    Invalid,
    #[serde(rename = "keyshortcuts")]
    Keyshortcuts,
    #[serde(rename = "settable")]
    Settable,
    #[serde(rename = "roledescription")]
    Roledescription,
    #[serde(rename = "live")]
    Live,
    #[serde(rename = "atomic")]
    Atomic,
    #[serde(rename = "relevant")]
    Relevant,
    #[serde(rename = "root")]
    Root,
    #[serde(rename = "autocomplete")]
    Autocomplete,
    #[serde(rename = "hasPopup")]
    HasPopup,
    #[serde(rename = "level")]
    Level,
    #[serde(rename = "multiselectable")]
    Multiselectable,
    #[serde(rename = "orientation")]
    Orientation,
    #[serde(rename = "multiline")]
    Multiline,
    #[serde(rename = "readonly")]
    Readonly,
    #[serde(rename = "required")]
    Required,
    #[serde(rename = "valuemin")]
    Valuemin,
    #[serde(rename = "valuemax")]
    Valuemax,
    #[serde(rename = "valuetext")]
    Valuetext,
    #[serde(rename = "checked")]
    Checked,
    #[serde(rename = "expanded")]
    Expanded,
    #[serde(rename = "modal")]
    Modal,
    #[serde(rename = "pressed")]
    Pressed,
    #[serde(rename = "selected")]
    Selected,
    #[serde(rename = "activedescendant")]
    Activedescendant,
    #[serde(rename = "controls")]
    Controls,
    #[serde(rename = "describedby")]
    Describedby,
    #[serde(rename = "details")]
    Details,
    #[serde(rename = "errormessage")]
    Errormessage,
    #[serde(rename = "flowto")]
    Flowto,
    #[serde(rename = "labelledby")]
    Labelledby,
    #[serde(rename = "owns")]
    Owns,
    #[serde(rename = "url")]
    Url,
    #[serde(rename = "activeFullscreenElement")]
    ActiveFullscreenElement,
    #[serde(rename = "activeModalDialog")]
    ActiveModalDialog,
    #[serde(rename = "activeAriaModalDialog")]
    ActiveAriaModalDialog,
    #[serde(rename = "ariaHiddenElement")]
    AriaHiddenElement,
    #[serde(rename = "ariaHiddenSubtree")]
    AriaHiddenSubtree,
    #[serde(rename = "emptyAlt")]
    EmptyAlt,
    #[serde(rename = "emptyText")]
    EmptyText,
    #[serde(rename = "inertElement")]
    InertElement,
    #[serde(rename = "inertSubtree")]
    InertSubtree,
    #[serde(rename = "labelContainer")]
    LabelContainer,
    #[serde(rename = "labelFor")]
    LabelFor,
    #[serde(rename = "notRendered")]
    NotRendered,
    #[serde(rename = "notVisible")]
    NotVisible,
    #[serde(rename = "presentationalRole")]
    PresentationalRole,
    #[serde(rename = "probablyPresentational")]
    ProbablyPresentational,
    #[serde(rename = "inactiveCarouselTabContent")]
    InactiveCarouselTabContent,
    #[serde(rename = "uninteresting")]
    Uninteresting,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct AxValueSource {
    #[serde(rename = "type")]
    pub r#type: AxValueSourceType,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "value")]
    pub value: Option<AxValue>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(rename = "attribute")]
    pub attribute: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "attributeValue")]
    pub attribute_value: Option<AxValue>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(rename = "superseded")]
    pub superseded: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "nativeSource")]
    pub native_source: Option<AxValueNativeSourceType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "nativeSourceValue")]
    pub native_source_value: Option<AxValue>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(rename = "invalid")]
    pub invalid: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(rename = "invalidReason")]
    pub invalid_reason: Option<String>,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct AxRelatedNode {
    #[serde(rename = "backendDOMNodeId")]
    pub backend_dom_node_id: dom::BackendNodeId,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(rename = "idref")]
    pub idref: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(rename = "text")]
    pub text: Option<String>,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct AxProperty {
    #[serde(rename = "name")]
    pub name: AxPropertyName,
    #[serde(rename = "value")]
    pub value: AxValue,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct AxValue {
    #[serde(rename = "type")]
    pub r#type: AxValueType,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(rename = "value")]
    pub value: Option<Json>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "relatedNodes")]
    pub related_nodes: Option<Vec<AxRelatedNode>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "sources")]
    pub sources: Option<Vec<AxValueSource>>,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct AxNode {
    #[serde(rename = "nodeId")]
    pub node_id: AxNodeId,
    #[serde(default)]
    #[serde(rename = "ignored")]
    pub ignored: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "ignoredReasons")]
    pub ignored_reasons: Option<Vec<AxProperty>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "role")]
    pub role: Option<AxValue>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "chromeRole")]
    pub chrome_role: Option<AxValue>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "name")]
    pub name: Option<AxValue>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "description")]
    pub description: Option<AxValue>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "value")]
    pub value: Option<AxValue>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "properties")]
    pub properties: Option<Vec<AxProperty>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "parentId")]
    pub parent_id: Option<AxNodeId>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "childIds")]
    pub child_ids: Option<Vec<AxNodeId>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "backendDOMNodeId")]
    pub backend_dom_node_id: Option<dom::BackendNodeId>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "frameId")]
    pub frame_id: Option<page::FrameId>,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Disable(pub Option<serde_json::Value>);
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Enable(pub Option<serde_json::Value>);
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct GetPartialAXTree {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "nodeId")]
    pub node_id: Option<dom::NodeId>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "backendNodeId")]
    pub backend_node_id: Option<dom::BackendNodeId>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "objectId")]
    pub object_id: Option<runtime::RemoteObjectId>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(rename = "fetchRelatives")]
    pub fetch_relatives: Option<bool>,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct GetFullAXTree {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(rename = "depth")]
    pub depth: Option<JsUInt>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "frameId")]
    pub frame_id: Option<page::FrameId>,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct GetRootAXNode {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "frameId")]
    pub frame_id: Option<page::FrameId>,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct GetAXNodeAndAncestors {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "nodeId")]
    pub node_id: Option<dom::NodeId>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "backendNodeId")]
    pub backend_node_id: Option<dom::BackendNodeId>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "objectId")]
    pub object_id: Option<runtime::RemoteObjectId>,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct GetChildAXNodes {
    #[serde(rename = "id")]
    pub id: AxNodeId,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "frameId")]
    pub frame_id: Option<page::FrameId>,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct QueryAXTree {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "nodeId")]
    pub node_id: Option<dom::NodeId>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "backendNodeId")]
    pub backend_node_id: Option<dom::BackendNodeId>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "objectId")]
    pub object_id: Option<runtime::RemoteObjectId>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(rename = "accessibleName")]
    pub accessible_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(rename = "role")]
    pub role: Option<String>,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct DisableReturnObject {}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct EnableReturnObject {}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct GetPartialAXTreeReturnObject {
    #[serde(rename = "nodes")]
    pub nodes: Vec<AxNode>,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct GetFullAXTreeReturnObject {
    #[serde(rename = "nodes")]
    pub nodes: Vec<AxNode>,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct GetRootAXNodeReturnObject {
    #[serde(rename = "node")]
    pub node: AxNode,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct GetAXNodeAndAncestorsReturnObject {
    #[serde(rename = "nodes")]
    pub nodes: Vec<AxNode>,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct GetChildAXNodesReturnObject {
    #[serde(rename = "nodes")]
    pub nodes: Vec<AxNode>,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct QueryAXTreeReturnObject {
    #[serde(rename = "nodes")]
    pub nodes: Vec<AxNode>,
}
impl Method for Disable {
    const NAME: &'static str = "Accessibility.disable";
    type ReturnObject = DisableReturnObject;
}
impl Method for Enable {
    const NAME: &'static str = "Accessibility.enable";
    type ReturnObject = EnableReturnObject;
}
impl Method for GetPartialAXTree {
    const NAME: &'static str = "Accessibility.getPartialAXTree";
    type ReturnObject = GetPartialAXTreeReturnObject;
}
impl Method for GetFullAXTree {
    const NAME: &'static str = "Accessibility.getFullAXTree";
    type ReturnObject = GetFullAXTreeReturnObject;
}
impl Method for GetRootAXNode {
    const NAME: &'static str = "Accessibility.getRootAXNode";
    type ReturnObject = GetRootAXNodeReturnObject;
}
impl Method for GetAXNodeAndAncestors {
    const NAME: &'static str = "Accessibility.getAXNodeAndAncestors";
    type ReturnObject = GetAXNodeAndAncestorsReturnObject;
}
impl Method for GetChildAXNodes {
    const NAME: &'static str = "Accessibility.getChildAXNodes";
    type ReturnObject = GetChildAXNodesReturnObject;
}
impl Method for QueryAXTree {
    const NAME: &'static str = "Accessibility.queryAXTree";
    type ReturnObject = QueryAXTreeReturnObject;
}
pub mod events {
    #[allow(unused_imports)]
    use super::super::types::*;
    #[allow(unused_imports)]
    use serde::{Deserialize, Serialize};
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
    pub struct LoadCompleteEvent {
        pub params: LoadCompleteEventParams,
    }
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
    pub struct LoadCompleteEventParams {
        #[serde(rename = "root")]
        pub root: super::AxNode,
    }
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
    pub struct NodesUpdatedEvent {
        pub params: NodesUpdatedEventParams,
    }
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
    pub struct NodesUpdatedEventParams {
        #[serde(rename = "nodes")]
        pub nodes: Vec<super::AxNode>,
    }
}
