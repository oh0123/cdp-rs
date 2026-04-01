// Auto-generated from Chrome at version 146.0.7680.165 domain: Accessibility
use super::dom;
use super::page;
use super::runtime;
#[allow(unused_imports)]
use super::types::*;
#[allow(unused_imports)]
use derive_builder::Builder;
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
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "camelCase")]
#[doc = "A single source for a computed AX property."]
pub struct AxValueSource {
    #[doc = "What type of source this is."]
    pub r#type: AxValueSourceType,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[doc = "The value of this property source."]
    pub value: Option<AxValue>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[doc = "The name of the relevant attribute, if any."]
    pub attribute: Option<String>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[doc = "The value of the relevant attribute, if any."]
    pub attribute_value: Option<AxValue>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[doc = "Whether this source is superseded by a higher priority source."]
    pub superseded: Option<bool>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[doc = "The native markup source for this value, e.g. a `<label>` element."]
    pub native_source: Option<AxValueNativeSourceType>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[doc = "The value, such as a node or node list, of the native source."]
    pub native_source_value: Option<AxValue>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[doc = "Whether the value for this property is invalid."]
    pub invalid: Option<bool>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[doc = "Reason for the value being invalid, if it is."]
    pub invalid_reason: Option<String>,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "camelCase")]
pub struct AxRelatedNode {
    #[doc = "The BackendNodeId of the related DOM node."]
    #[serde(rename = "backendDOMNodeId")]
    pub backend_dom_node_id: dom::BackendNodeId,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[doc = "The IDRef value provided, if any."]
    pub idref: Option<String>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[doc = "The text alternative of this node in the current context."]
    pub text: Option<String>,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "camelCase")]
pub struct AxProperty {
    #[doc = "The name of this property."]
    pub name: AxPropertyName,
    #[doc = "The value of this property."]
    pub value: AxValue,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "camelCase")]
#[doc = "A single computed AX property."]
pub struct AxValue {
    #[doc = "The type of this value."]
    pub r#type: AxValueType,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[doc = "The computed value of this property."]
    pub value: Option<Json>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[doc = "One or more related nodes, if applicable."]
    pub related_nodes: Option<Vec<AxRelatedNode>>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[doc = "The sources which contributed to the computation of this property."]
    pub sources: Option<Vec<AxValueSource>>,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "camelCase")]
#[doc = "A node in the accessibility tree."]
pub struct AxNode {
    #[doc = "Unique identifier for this node."]
    pub node_id: AxNodeId,
    #[serde(default)]
    #[doc = "Whether this node is ignored for accessibility"]
    pub ignored: bool,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[doc = "Collection of reasons why this node is hidden."]
    pub ignored_reasons: Option<Vec<AxProperty>>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[doc = "This `Node`'s role, whether explicit or implicit."]
    pub role: Option<AxValue>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[doc = "This `Node`'s Chrome raw role."]
    pub chrome_role: Option<AxValue>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[doc = "The accessible name for this `Node`."]
    pub name: Option<AxValue>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[doc = "The accessible description for this `Node`."]
    pub description: Option<AxValue>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[doc = "The value for this `Node`."]
    pub value: Option<AxValue>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[doc = "All other properties"]
    pub properties: Option<Vec<AxProperty>>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[doc = "ID for this node's parent."]
    pub parent_id: Option<AxNodeId>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[doc = "IDs for each of this node's child nodes."]
    pub child_ids: Option<Vec<AxNodeId>>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[doc = "The backend ID for the associated DOM node, if any."]
    #[serde(rename = "backendDOMNodeId")]
    pub backend_dom_node_id: Option<dom::BackendNodeId>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[doc = "The frame ID for the frame associated with this nodes document."]
    pub frame_id: Option<page::FrameId>,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct Disable(pub Option<Json>);
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct Enable(pub Option<Json>);
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "camelCase")]
#[doc = "Fetches the accessibility node and partial accessibility tree for this DOM node, if it exists."]
pub struct GetPartialAXTree {
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[doc = "Identifier of the node to get the partial accessibility tree for."]
    pub node_id: Option<dom::NodeId>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[doc = "Identifier of the backend node to get the partial accessibility tree for."]
    pub backend_node_id: Option<dom::BackendNodeId>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[doc = "JavaScript object id of the node wrapper to get the partial accessibility tree for."]
    pub object_id: Option<runtime::RemoteObjectId>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[doc = "Whether to fetch this node's ancestors, siblings and children. Defaults to true."]
    pub fetch_relatives: Option<bool>,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "camelCase")]
#[doc = "Fetches the entire accessibility tree for the root Document"]
pub struct GetFullAXTree {
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[doc = "The maximum depth at which descendants of the root node should be retrieved.\n If omitted, the full tree is returned."]
    pub depth: Option<JsUInt>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[doc = "The frame for whose document the AX tree should be retrieved.\n If omitted, the root frame is used."]
    pub frame_id: Option<page::FrameId>,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "camelCase")]
#[doc = "Fetches the root node.\n Requires `enable()` to have been called previously."]
pub struct GetRootAXNode {
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[doc = "The frame in whose document the node resides.\n If omitted, the root frame is used."]
    pub frame_id: Option<page::FrameId>,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "camelCase")]
#[doc = "Fetches a node and all ancestors up to and including the root.\n Requires `enable()` to have been called previously."]
pub struct GetAXNodeAndAncestors {
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[doc = "Identifier of the node to get."]
    pub node_id: Option<dom::NodeId>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[doc = "Identifier of the backend node to get."]
    pub backend_node_id: Option<dom::BackendNodeId>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[doc = "JavaScript object id of the node wrapper to get."]
    pub object_id: Option<runtime::RemoteObjectId>,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "camelCase")]
#[doc = "Fetches a particular accessibility node by AXNodeId.\n Requires `enable()` to have been called previously."]
pub struct GetChildAXNodes {
    pub id: AxNodeId,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[doc = "The frame in whose document the node resides.\n If omitted, the root frame is used."]
    pub frame_id: Option<page::FrameId>,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "camelCase")]
#[doc = "Query a DOM node's accessibility subtree for accessible name and role.\n This command computes the name and role for all nodes in the subtree, including those that are\n ignored for accessibility, and returns those that match the specified name and role. If no DOM\n node is specified, or the DOM node does not exist, the command returns an error. If neither\n `accessibleName` or `role` is specified, it returns all the accessibility nodes in the subtree."]
pub struct QueryAXTree {
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[doc = "Identifier of the node for the root to query."]
    pub node_id: Option<dom::NodeId>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[doc = "Identifier of the backend node for the root to query."]
    pub backend_node_id: Option<dom::BackendNodeId>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[doc = "JavaScript object id of the node wrapper for the root to query."]
    pub object_id: Option<runtime::RemoteObjectId>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[doc = "Find nodes with this computed name."]
    pub accessible_name: Option<String>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[doc = "Find nodes with this computed role."]
    pub role: Option<String>,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[doc = "Disables the accessibility domain."]
pub struct DisableReturnObject(pub Option<Json>);
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[doc = "Enables the accessibility domain which causes `AXNodeId`s to remain consistent between method calls.\n This turns on accessibility for the page, which can impact performance until accessibility is disabled."]
pub struct EnableReturnObject(pub Option<Json>);
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[serde(rename_all = "camelCase")]
#[doc = "Fetches the accessibility node and partial accessibility tree for this DOM node, if it exists."]
pub struct GetPartialAXTreeReturnObject {
    #[doc = "The `Accessibility.AXNode` for this DOM node, if it exists, plus its ancestors, siblings and\n children, if requested."]
    pub nodes: Vec<AxNode>,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[serde(rename_all = "camelCase")]
#[doc = "Fetches the entire accessibility tree for the root Document"]
pub struct GetFullAXTreeReturnObject {
    pub nodes: Vec<AxNode>,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[serde(rename_all = "camelCase")]
#[doc = "Fetches the root node.\n Requires `enable()` to have been called previously."]
pub struct GetRootAXNodeReturnObject {
    pub node: AxNode,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[serde(rename_all = "camelCase")]
#[doc = "Fetches a node and all ancestors up to and including the root.\n Requires `enable()` to have been called previously."]
pub struct GetAXNodeAndAncestorsReturnObject {
    pub nodes: Vec<AxNode>,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[serde(rename_all = "camelCase")]
#[doc = "Fetches a particular accessibility node by AXNodeId.\n Requires `enable()` to have been called previously."]
pub struct GetChildAXNodesReturnObject {
    pub nodes: Vec<AxNode>,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[serde(rename_all = "camelCase")]
#[doc = "Query a DOM node's accessibility subtree for accessible name and role.\n This command computes the name and role for all nodes in the subtree, including those that are\n ignored for accessibility, and returns those that match the specified name and role. If no DOM\n node is specified, or the DOM node does not exist, the command returns an error. If neither\n `accessibleName` or `role` is specified, it returns all the accessibility nodes in the subtree."]
pub struct QueryAXTreeReturnObject {
    #[doc = "A list of `Accessibility.AXNode` matching the specified attributes,\n including nodes that are ignored for accessibility."]
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
    use derive_builder::Builder;
    #[allow(unused_imports)]
    use serde::{Deserialize, Serialize};
    #[allow(unused_imports)]
    use serde_json::Value as Json;
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
    pub struct LoadCompleteEvent {
        pub params: LoadCompleteEventParams,
    }
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
    #[serde(rename_all = "camelCase")]
    pub struct LoadCompleteEventParams {
        #[doc = "New document root node."]
        pub root: super::AxNode,
    }
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
    pub struct NodesUpdatedEvent {
        pub params: NodesUpdatedEventParams,
    }
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
    #[serde(rename_all = "camelCase")]
    pub struct NodesUpdatedEventParams {
        #[doc = "Updated node data."]
        pub nodes: Vec<super::AxNode>,
    }
}
