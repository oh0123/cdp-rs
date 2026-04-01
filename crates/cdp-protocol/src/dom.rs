// Auto-generated from Chrome at version 146.0.7680.165 domain: DOM
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
pub type NodeId = JsUInt;
pub type BackendNodeId = JsUInt;
pub type StyleSheetId = String;
pub type Quad = Vec<JsFloat>;
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub enum PseudoType {
    #[serde(rename = "first-line")]
    FirstLine,
    #[serde(rename = "first-letter")]
    FirstLetter,
    #[serde(rename = "checkmark")]
    Checkmark,
    #[serde(rename = "before")]
    Before,
    #[serde(rename = "after")]
    After,
    #[serde(rename = "picker-icon")]
    PickerIcon,
    #[serde(rename = "interest-hint")]
    InterestHint,
    #[serde(rename = "marker")]
    Marker,
    #[serde(rename = "backdrop")]
    Backdrop,
    #[serde(rename = "column")]
    Column,
    #[serde(rename = "selection")]
    Selection,
    #[serde(rename = "search-text")]
    SearchText,
    #[serde(rename = "target-text")]
    TargetText,
    #[serde(rename = "spelling-error")]
    SpellingError,
    #[serde(rename = "grammar-error")]
    GrammarError,
    #[serde(rename = "highlight")]
    Highlight,
    #[serde(rename = "first-line-inherited")]
    FirstLineInherited,
    #[serde(rename = "scroll-marker")]
    ScrollMarker,
    #[serde(rename = "scroll-marker-group")]
    ScrollMarkerGroup,
    #[serde(rename = "scroll-button")]
    ScrollButton,
    #[serde(rename = "scrollbar")]
    Scrollbar,
    #[serde(rename = "scrollbar-thumb")]
    ScrollbarThumb,
    #[serde(rename = "scrollbar-button")]
    ScrollbarButton,
    #[serde(rename = "scrollbar-track")]
    ScrollbarTrack,
    #[serde(rename = "scrollbar-track-piece")]
    ScrollbarTrackPiece,
    #[serde(rename = "scrollbar-corner")]
    ScrollbarCorner,
    #[serde(rename = "resizer")]
    Resizer,
    #[serde(rename = "input-list-button")]
    InputListButton,
    #[serde(rename = "view-transition")]
    ViewTransition,
    #[serde(rename = "view-transition-group")]
    ViewTransitionGroup,
    #[serde(rename = "view-transition-image-pair")]
    ViewTransitionImagePair,
    #[serde(rename = "view-transition-group-children")]
    ViewTransitionGroupChildren,
    #[serde(rename = "view-transition-old")]
    ViewTransitionOld,
    #[serde(rename = "view-transition-new")]
    ViewTransitionNew,
    #[serde(rename = "placeholder")]
    Placeholder,
    #[serde(rename = "file-selector-button")]
    FileSelectorButton,
    #[serde(rename = "details-content")]
    DetailsContent,
    #[serde(rename = "picker")]
    Picker,
    #[serde(rename = "permission-icon")]
    PermissionIcon,
    #[serde(rename = "overscroll-area-parent")]
    OverscrollAreaParent,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub enum ShadowRootType {
    #[serde(rename = "user-agent")]
    UserAgent,
    #[serde(rename = "open")]
    Open,
    #[serde(rename = "closed")]
    Closed,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub enum CompatibilityMode {
    #[serde(rename = "QuirksMode")]
    QuirksMode,
    #[serde(rename = "LimitedQuirksMode")]
    LimitedQuirksMode,
    #[serde(rename = "NoQuirksMode")]
    NoQuirksMode,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub enum PhysicalAxes {
    #[serde(rename = "Horizontal")]
    Horizontal,
    #[serde(rename = "Vertical")]
    Vertical,
    #[serde(rename = "Both")]
    Both,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub enum LogicalAxes {
    #[serde(rename = "Inline")]
    Inline,
    #[serde(rename = "Block")]
    Block,
    #[serde(rename = "Both")]
    Both,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub enum ScrollOrientation {
    #[serde(rename = "horizontal")]
    Horizontal,
    #[serde(rename = "vertical")]
    Vertical,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub enum EnableIncludeWhitespaceOption {
    #[serde(rename = "none")]
    None,
    #[serde(rename = "all")]
    All,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub enum GetElementByRelationRelationOption {
    #[serde(rename = "PopoverTarget")]
    PopoverTarget,
    #[serde(rename = "InterestTarget")]
    InterestTarget,
    #[serde(rename = "CommandFor")]
    CommandFor,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "camelCase")]
#[doc = "Backend node with a friendly name."]
pub struct BackendNode {
    #[serde(default)]
    #[doc = "`Node`'s nodeType."]
    pub node_type: JsUInt,
    #[serde(default)]
    #[doc = "`Node`'s nodeName."]
    pub node_name: String,
    pub backend_node_id: BackendNodeId,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "camelCase")]
#[doc = "DOM interaction is implemented in terms of mirror objects that represent the actual DOM nodes.\n DOMNode is a base node mirror type."]
pub struct Node {
    #[doc = "Node identifier that is passed into the rest of the DOM messages as the `nodeId`. Backend\n will only push node with given `id` once. It is aware of all requested nodes and will only\n fire DOM events for nodes known to the client."]
    pub node_id: NodeId,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[doc = "The id of the parent node if any."]
    pub parent_id: Option<NodeId>,
    #[doc = "The BackendNodeId for this node."]
    pub backend_node_id: BackendNodeId,
    #[serde(default)]
    #[doc = "`Node`'s nodeType."]
    pub node_type: JsUInt,
    #[serde(default)]
    #[doc = "`Node`'s nodeName."]
    pub node_name: String,
    #[serde(default)]
    #[doc = "`Node`'s localName."]
    pub local_name: String,
    #[serde(default)]
    #[doc = "`Node`'s nodeValue."]
    pub node_value: String,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[doc = "Child count for `Container` nodes."]
    pub child_node_count: Option<JsUInt>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[doc = "Child nodes of this node when requested with children."]
    pub children: Option<Vec<Node>>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[doc = "Attributes of the `Element` node in the form of flat array `[name1, value1, name2, value2]`."]
    pub attributes: Option<Vec<String>>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[doc = "Document URL that `Document` or `FrameOwner` node points to."]
    #[serde(rename = "documentURL")]
    pub document_url: Option<String>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[doc = "Base URL that `Document` or `FrameOwner` node uses for URL completion."]
    #[serde(rename = "baseURL")]
    pub base_url: Option<String>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[doc = "`DocumentType`'s publicId."]
    pub public_id: Option<String>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[doc = "`DocumentType`'s systemId."]
    pub system_id: Option<String>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[doc = "`DocumentType`'s internalSubset."]
    pub internal_subset: Option<String>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[doc = "`Document`'s XML version in case of XML documents."]
    pub xml_version: Option<String>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[doc = "`Attr`'s name."]
    pub name: Option<String>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[doc = "`Attr`'s value."]
    pub value: Option<String>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[doc = "Pseudo element type for this node."]
    pub pseudo_type: Option<PseudoType>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[doc = "Pseudo element identifier for this node. Only present if there is a\n valid pseudoType."]
    pub pseudo_identifier: Option<String>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[doc = "Shadow root type."]
    pub shadow_root_type: Option<ShadowRootType>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[doc = "Frame ID for frame owner elements."]
    pub frame_id: Option<page::FrameId>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[doc = "Content document for frame owner elements."]
    pub content_document: Option<Box<Node>>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[doc = "Shadow root list for given element host."]
    pub shadow_roots: Option<Vec<Node>>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[doc = "Content document fragment for template elements."]
    pub template_content: Option<Box<Node>>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[doc = "Pseudo elements associated with this node."]
    pub pseudo_elements: Option<Vec<Node>>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[doc = "Deprecated, as the HTML Imports API has been removed (crbug.com/937746).\n This property used to return the imported document for the HTMLImport links.\n The property is always undefined now."]
    #[deprecated]
    pub imported_document: Option<Box<Node>>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[doc = "Distributed nodes for given insertion point."]
    pub distributed_nodes: Option<Vec<BackendNode>>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[doc = "Whether the node is SVG."]
    #[serde(rename = "isSVG")]
    pub is_svg: Option<bool>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub compatibility_mode: Option<CompatibilityMode>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub assigned_slot: Option<BackendNode>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub is_scrollable: Option<bool>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub affected_by_starting_styles: Option<bool>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub adopted_style_sheets: Option<Vec<StyleSheetId>>,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "camelCase")]
#[doc = "A structure to hold the top-level node of a detached tree and an array of its retained descendants."]
pub struct DetachedElementInfo {
    pub tree_node: Node,
    pub retained_node_ids: Vec<NodeId>,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "camelCase")]
#[doc = "A structure holding an RGBA color."]
pub struct Rgba {
    #[serde(default)]
    #[doc = "The red component, in the [0-255] range."]
    pub r: JsUInt,
    #[serde(default)]
    #[doc = "The green component, in the [0-255] range."]
    pub g: JsUInt,
    #[serde(default)]
    #[doc = "The blue component, in the [0-255] range."]
    pub b: JsUInt,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[doc = "The alpha component, in the [0-1] range (default: 1)."]
    pub a: Option<JsFloat>,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "camelCase")]
#[doc = "Box model."]
pub struct BoxModel {
    #[doc = "Content box"]
    pub content: Quad,
    #[doc = "Padding box"]
    pub padding: Quad,
    #[doc = "Border box"]
    pub border: Quad,
    #[doc = "Margin box"]
    pub margin: Quad,
    #[serde(default)]
    #[doc = "Node width"]
    pub width: JsUInt,
    #[serde(default)]
    #[doc = "Node height"]
    pub height: JsUInt,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[doc = "Shape outside coordinates"]
    pub shape_outside: Option<ShapeOutsideInfo>,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "camelCase")]
#[doc = "CSS Shape Outside details."]
pub struct ShapeOutsideInfo {
    #[doc = "Shape bounds"]
    pub bounds: Quad,
    #[serde(default)]
    #[doc = "Shape coordinate details"]
    pub shape: Vec<Json>,
    #[serde(default)]
    #[doc = "Margin shape bounds"]
    pub margin_shape: Vec<Json>,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "camelCase")]
#[doc = "Rectangle."]
pub struct Rect {
    #[serde(default)]
    #[doc = "X coordinate"]
    pub x: JsFloat,
    #[serde(default)]
    #[doc = "Y coordinate"]
    pub y: JsFloat,
    #[serde(default)]
    #[doc = "Rectangle width"]
    pub width: JsFloat,
    #[serde(default)]
    #[doc = "Rectangle height"]
    pub height: JsFloat,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "camelCase")]
pub struct CssComputedStyleProperty {
    #[serde(default)]
    #[doc = "Computed style property name."]
    pub name: String,
    #[serde(default)]
    #[doc = "Computed style property value."]
    pub value: String,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "camelCase")]
#[doc = "Collects class names for the node with given id and all of it's child nodes."]
pub struct CollectClassNamesFromSubtree {
    #[doc = "Id of the node to collect class names."]
    pub node_id: NodeId,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "camelCase")]
#[doc = "Creates a deep copy of the specified node and places it into the target container before the\n given anchor."]
pub struct CopyTo {
    #[doc = "Id of the node to copy."]
    pub node_id: NodeId,
    #[doc = "Id of the element to drop the copy into."]
    pub target_node_id: NodeId,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[doc = "Drop the copy before this node (if absent, the copy becomes the last child of\n `targetNodeId`)."]
    pub insert_before_node_id: Option<NodeId>,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "camelCase")]
#[doc = "Describes node given its id, does not require domain to be enabled. Does not start tracking any\n objects, can be used for automation."]
pub struct DescribeNode {
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[doc = "Identifier of the node."]
    pub node_id: Option<NodeId>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[doc = "Identifier of the backend node."]
    pub backend_node_id: Option<BackendNodeId>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[doc = "JavaScript object id of the node wrapper."]
    pub object_id: Option<runtime::RemoteObjectId>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[doc = "The maximum depth at which children should be retrieved, defaults to 1. Use -1 for the\n entire subtree or provide an integer larger than 0."]
    pub depth: Option<JsUInt>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[doc = "Whether or not iframes and shadow roots should be traversed when returning the subtree\n (default is false)."]
    pub pierce: Option<bool>,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "camelCase")]
#[doc = "Scrolls the specified rect of the given node into view if not already visible.\n Note: exactly one between nodeId, backendNodeId and objectId should be passed\n to identify the node."]
pub struct ScrollIntoViewIfNeeded {
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[doc = "Identifier of the node."]
    pub node_id: Option<NodeId>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[doc = "Identifier of the backend node."]
    pub backend_node_id: Option<BackendNodeId>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[doc = "JavaScript object id of the node wrapper."]
    pub object_id: Option<runtime::RemoteObjectId>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[doc = "The rect to be scrolled into view, relative to the node's border box, in CSS pixels.\n When omitted, center of the node will be used, similar to Element.scrollIntoView."]
    pub rect: Option<Rect>,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct Disable(pub Option<Json>);
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "camelCase")]
#[doc = "Discards search results from the session with the given id. `getSearchResults` should no longer\n be called for that search."]
pub struct DiscardSearchResults {
    #[serde(default)]
    #[doc = "Unique search session identifier."]
    pub search_id: String,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "camelCase")]
#[doc = "Enables DOM agent for the given page."]
pub struct Enable {
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[doc = "Whether to include whitespaces in the children array of returned Nodes."]
    pub include_whitespace: Option<EnableIncludeWhitespaceOption>,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "camelCase")]
#[doc = "Focuses the given element."]
pub struct Focus {
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[doc = "Identifier of the node."]
    pub node_id: Option<NodeId>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[doc = "Identifier of the backend node."]
    pub backend_node_id: Option<BackendNodeId>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[doc = "JavaScript object id of the node wrapper."]
    pub object_id: Option<runtime::RemoteObjectId>,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "camelCase")]
#[doc = "Returns attributes for the specified node."]
pub struct GetAttributes {
    #[doc = "Id of the node to retrieve attributes for."]
    pub node_id: NodeId,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "camelCase")]
#[doc = "Returns boxes for the given node."]
pub struct GetBoxModel {
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[doc = "Identifier of the node."]
    pub node_id: Option<NodeId>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[doc = "Identifier of the backend node."]
    pub backend_node_id: Option<BackendNodeId>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[doc = "JavaScript object id of the node wrapper."]
    pub object_id: Option<runtime::RemoteObjectId>,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "camelCase")]
#[doc = "Returns quads that describe node position on the page. This method\n might return multiple quads for inline nodes."]
pub struct GetContentQuads {
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[doc = "Identifier of the node."]
    pub node_id: Option<NodeId>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[doc = "Identifier of the backend node."]
    pub backend_node_id: Option<BackendNodeId>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[doc = "JavaScript object id of the node wrapper."]
    pub object_id: Option<runtime::RemoteObjectId>,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "camelCase")]
#[doc = "Returns the root DOM node (and optionally the subtree) to the caller.\n Implicitly enables the DOM domain events for the current target."]
pub struct GetDocument {
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[doc = "The maximum depth at which children should be retrieved, defaults to 1. Use -1 for the\n entire subtree or provide an integer larger than 0."]
    pub depth: Option<JsUInt>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[doc = "Whether or not iframes and shadow roots should be traversed when returning the subtree\n (default is false)."]
    pub pierce: Option<bool>,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "camelCase")]
#[doc = "Returns the root DOM node (and optionally the subtree) to the caller.\n Deprecated, as it is not designed to work well with the rest of the DOM agent.\n Use DOMSnapshot.captureSnapshot instead."]
#[deprecated]
pub struct GetFlattenedDocument {
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[doc = "The maximum depth at which children should be retrieved, defaults to 1. Use -1 for the\n entire subtree or provide an integer larger than 0."]
    pub depth: Option<JsUInt>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[doc = "Whether or not iframes and shadow roots should be traversed when returning the subtree\n (default is false)."]
    pub pierce: Option<bool>,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "camelCase")]
#[doc = "Finds nodes with a given computed style in a subtree."]
pub struct GetNodesForSubtreeByStyle {
    #[doc = "Node ID pointing to the root of a subtree."]
    pub node_id: NodeId,
    #[doc = "The style to filter nodes by (includes nodes if any of properties matches)."]
    pub computed_styles: Vec<CssComputedStyleProperty>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[doc = "Whether or not iframes and shadow roots in the same target should be traversed when returning the\n results (default is false)."]
    pub pierce: Option<bool>,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "camelCase")]
#[doc = "Returns node id at given location. Depending on whether DOM domain is enabled, nodeId is\n either returned or not."]
pub struct GetNodeForLocation {
    #[serde(default)]
    #[doc = "X coordinate."]
    pub x: JsUInt,
    #[serde(default)]
    #[doc = "Y coordinate."]
    pub y: JsUInt,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[doc = "False to skip to the nearest non-UA shadow root ancestor (default: false)."]
    #[serde(rename = "includeUserAgentShadowDOM")]
    pub include_user_agent_shadow_dom: Option<bool>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[doc = "Whether to ignore pointer-events: none on elements and hit test them."]
    pub ignore_pointer_events_none: Option<bool>,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "camelCase")]
#[doc = "Returns node's HTML markup."]
pub struct GetOuterHTML {
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[doc = "Identifier of the node."]
    pub node_id: Option<NodeId>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[doc = "Identifier of the backend node."]
    pub backend_node_id: Option<BackendNodeId>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[doc = "JavaScript object id of the node wrapper."]
    pub object_id: Option<runtime::RemoteObjectId>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[doc = "Include all shadow roots. Equals to false if not specified."]
    #[serde(rename = "includeShadowDOM")]
    pub include_shadow_dom: Option<bool>,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "camelCase")]
#[doc = "Returns the id of the nearest ancestor that is a relayout boundary."]
pub struct GetRelayoutBoundary {
    #[doc = "Id of the node."]
    pub node_id: NodeId,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "camelCase")]
#[doc = "Returns search results from given `fromIndex` to given `toIndex` from the search with the given\n identifier."]
pub struct GetSearchResults {
    #[serde(default)]
    #[doc = "Unique search session identifier."]
    pub search_id: String,
    #[serde(default)]
    #[doc = "Start index of the search result to be returned."]
    pub from_index: JsUInt,
    #[serde(default)]
    #[doc = "End index of the search result to be returned."]
    pub to_index: JsUInt,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct HideHighlight(pub Option<Json>);
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct HighlightNode(pub Option<Json>);
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct HighlightRect(pub Option<Json>);
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct MarkUndoableState(pub Option<Json>);
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "camelCase")]
#[doc = "Moves node into the new container, places it before the given anchor."]
pub struct MoveTo {
    #[doc = "Id of the node to move."]
    pub node_id: NodeId,
    #[doc = "Id of the element to drop the moved node into."]
    pub target_node_id: NodeId,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[doc = "Drop node before this one (if absent, the moved node becomes the last child of\n `targetNodeId`)."]
    pub insert_before_node_id: Option<NodeId>,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "camelCase")]
#[doc = "Searches for a given string in the DOM tree. Use `getSearchResults` to access search results or\n `cancelSearch` to end this search session."]
pub struct PerformSearch {
    #[serde(default)]
    #[doc = "Plain text or query selector or XPath search query."]
    pub query: String,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[doc = "True to search in user agent shadow DOM."]
    #[serde(rename = "includeUserAgentShadowDOM")]
    pub include_user_agent_shadow_dom: Option<bool>,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "camelCase")]
#[doc = "Requests that the node is sent to the caller given its path. // FIXME, use XPath"]
pub struct PushNodeByPathToFrontend {
    #[serde(default)]
    #[doc = "Path to node in the proprietary format."]
    pub path: String,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "camelCase")]
#[doc = "Requests that a batch of nodes is sent to the caller given their backend node ids."]
pub struct PushNodesByBackendIdsToFrontend {
    #[doc = "The array of backend node ids."]
    pub backend_node_ids: Vec<BackendNodeId>,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "camelCase")]
#[doc = "Executes `querySelector` on a given node."]
pub struct QuerySelector {
    #[doc = "Id of the node to query upon."]
    pub node_id: NodeId,
    #[serde(default)]
    #[doc = "Selector string."]
    pub selector: String,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "camelCase")]
#[doc = "Executes `querySelectorAll` on a given node."]
pub struct QuerySelectorAll {
    #[doc = "Id of the node to query upon."]
    pub node_id: NodeId,
    #[serde(default)]
    #[doc = "Selector string."]
    pub selector: String,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct GetTopLayerElements(pub Option<Json>);
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "camelCase")]
#[doc = "Returns the NodeId of the matched element according to certain relations."]
pub struct GetElementByRelation {
    #[doc = "Id of the node from which to query the relation."]
    pub node_id: NodeId,
    #[doc = "Type of relation to get."]
    pub relation: GetElementByRelationRelationOption,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct Redo(pub Option<Json>);
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "camelCase")]
#[doc = "Removes attribute with given name from an element with given id."]
pub struct RemoveAttribute {
    #[doc = "Id of the element to remove attribute from."]
    pub node_id: NodeId,
    #[serde(default)]
    #[doc = "Name of the attribute to remove."]
    pub name: String,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "camelCase")]
#[doc = "Removes node with given id."]
pub struct RemoveNode {
    #[doc = "Id of the node to remove."]
    pub node_id: NodeId,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "camelCase")]
#[doc = "Requests that children of the node with given id are returned to the caller in form of\n `setChildNodes` events where not only immediate children are retrieved, but all children down to\n the specified depth."]
pub struct RequestChildNodes {
    #[doc = "Id of the node to get children for."]
    pub node_id: NodeId,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[doc = "The maximum depth at which children should be retrieved, defaults to 1. Use -1 for the\n entire subtree or provide an integer larger than 0."]
    pub depth: Option<JsUInt>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[doc = "Whether or not iframes and shadow roots should be traversed when returning the sub-tree\n (default is false)."]
    pub pierce: Option<bool>,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "camelCase")]
#[doc = "Requests that the node is sent to the caller given the JavaScript node object reference. All\n nodes that form the path from the node to the root are also sent to the client as a series of\n `setChildNodes` notifications."]
pub struct RequestNode {
    #[doc = "JavaScript object id to convert into node."]
    pub object_id: runtime::RemoteObjectId,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "camelCase")]
#[doc = "Resolves the JavaScript node object for a given NodeId or BackendNodeId."]
pub struct ResolveNode {
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[doc = "Id of the node to resolve."]
    pub node_id: Option<NodeId>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[doc = "Backend identifier of the node to resolve."]
    pub backend_node_id: Option<dom::BackendNodeId>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[doc = "Symbolic group name that can be used to release multiple objects."]
    pub object_group: Option<String>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[doc = "Execution context in which to resolve the node."]
    pub execution_context_id: Option<runtime::ExecutionContextId>,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "camelCase")]
#[doc = "Sets attribute for an element with given id."]
pub struct SetAttributeValue {
    #[doc = "Id of the element to set attribute for."]
    pub node_id: NodeId,
    #[serde(default)]
    #[doc = "Attribute name."]
    pub name: String,
    #[serde(default)]
    #[doc = "Attribute value."]
    pub value: String,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "camelCase")]
#[doc = "Sets attributes on element with given id. This method is useful when user edits some existing\n attribute value and types in several attribute name/value pairs."]
pub struct SetAttributesAsText {
    #[doc = "Id of the element to set attributes for."]
    pub node_id: NodeId,
    #[serde(default)]
    #[doc = "Text with a number of attributes. Will parse this text using HTML parser."]
    pub text: String,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[doc = "Attribute name to replace with new attributes derived from text in case text parsed\n successfully."]
    pub name: Option<String>,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "camelCase")]
#[doc = "Sets files for the given file input element."]
pub struct SetFileInputFiles {
    #[serde(default)]
    #[doc = "Array of file paths to set."]
    pub files: Vec<String>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[doc = "Identifier of the node."]
    pub node_id: Option<NodeId>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[doc = "Identifier of the backend node."]
    pub backend_node_id: Option<BackendNodeId>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[doc = "JavaScript object id of the node wrapper."]
    pub object_id: Option<runtime::RemoteObjectId>,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "camelCase")]
#[doc = "Sets if stack traces should be captured for Nodes. See `Node.getNodeStackTraces`. Default is disabled."]
pub struct SetNodeStackTracesEnabled {
    #[serde(default)]
    #[doc = "Enable or disable."]
    pub enable: bool,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "camelCase")]
#[doc = "Gets stack traces associated with a Node. As of now, only provides stack trace for Node creation."]
pub struct GetNodeStackTraces {
    #[doc = "Id of the node to get stack traces for."]
    pub node_id: NodeId,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "camelCase")]
#[doc = "Returns file information for the given\n File wrapper."]
pub struct GetFileInfo {
    #[doc = "JavaScript object id of the node wrapper."]
    pub object_id: runtime::RemoteObjectId,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct GetDetachedDomNodes(pub Option<Json>);
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "camelCase")]
#[doc = "Enables console to refer to the node with given id via $x (see Command Line API for more details\n $x functions)."]
pub struct SetInspectedNode {
    #[doc = "DOM node id to be accessible by means of $x command line API."]
    pub node_id: NodeId,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "camelCase")]
#[doc = "Sets node name for a node with given id."]
pub struct SetNodeName {
    #[doc = "Id of the node to set name for."]
    pub node_id: NodeId,
    #[serde(default)]
    #[doc = "New node's name."]
    pub name: String,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "camelCase")]
#[doc = "Sets node value for a node with given id."]
pub struct SetNodeValue {
    #[doc = "Id of the node to set value for."]
    pub node_id: NodeId,
    #[serde(default)]
    #[doc = "New node's value."]
    pub value: String,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "camelCase")]
#[doc = "Sets node HTML markup, returns new node id."]
pub struct SetOuterHTML {
    #[doc = "Id of the node to set markup for."]
    pub node_id: NodeId,
    #[serde(default)]
    #[doc = "Outer HTML markup to set."]
    #[serde(rename = "outerHTML")]
    pub outer_html: String,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct Undo(pub Option<Json>);
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "camelCase")]
#[doc = "Returns iframe node that owns iframe with the given domain."]
pub struct GetFrameOwner {
    pub frame_id: page::FrameId,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "camelCase")]
#[doc = "Returns the query container of the given node based on container query\n conditions: containerName, physical and logical axes, and whether it queries\n scroll-state or anchored elements. If no axes are provided and\n queriesScrollState is false, the style container is returned, which is the\n direct parent or the closest element with a matching container-name."]
pub struct GetContainerForNode {
    pub node_id: NodeId,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub container_name: Option<String>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub physical_axes: Option<PhysicalAxes>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub logical_axes: Option<LogicalAxes>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub queries_scroll_state: Option<bool>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub queries_anchored: Option<bool>,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "camelCase")]
#[doc = "Returns the descendants of a container query container that have\n container queries against this container."]
pub struct GetQueryingDescendantsForContainer {
    #[doc = "Id of the container node to find querying descendants from."]
    pub node_id: NodeId,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "camelCase")]
#[doc = "Returns the target anchor element of the given anchor query according to\n https://www.w3.org/TR/css-anchor-position-1/#target."]
pub struct GetAnchorElement {
    #[doc = "Id of the positioned element from which to find the anchor."]
    pub node_id: NodeId,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[doc = "An optional anchor specifier, as defined in\n https://www.w3.org/TR/css-anchor-position-1/#anchor-specifier.\n If not provided, it will return the implicit anchor element for\n the given positioned element."]
    pub anchor_specifier: Option<String>,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "camelCase")]
#[doc = "When enabling, this API force-opens the popover identified by nodeId\n and keeps it open until disabled."]
pub struct ForceShowPopover {
    #[doc = "Id of the popover HTMLElement"]
    pub node_id: NodeId,
    #[serde(default)]
    #[doc = "If true, opens the popover and keeps it open. If false, closes the\n popover if it was previously force-opened."]
    pub enable: bool,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[serde(rename_all = "camelCase")]
#[doc = "Collects class names for the node with given id and all of it's child nodes."]
pub struct CollectClassNamesFromSubtreeReturnObject {
    #[doc = "Class name list."]
    pub class_names: Vec<String>,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[serde(rename_all = "camelCase")]
#[doc = "Creates a deep copy of the specified node and places it into the target container before the\n given anchor."]
pub struct CopyToReturnObject {
    #[doc = "Id of the node clone."]
    pub node_id: NodeId,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[serde(rename_all = "camelCase")]
#[doc = "Describes node given its id, does not require domain to be enabled. Does not start tracking any\n objects, can be used for automation."]
pub struct DescribeNodeReturnObject {
    #[doc = "Node description."]
    pub node: Node,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[doc = "Scrolls the specified rect of the given node into view if not already visible.\n Note: exactly one between nodeId, backendNodeId and objectId should be passed\n to identify the node."]
pub struct ScrollIntoViewIfNeededReturnObject(pub Option<Json>);
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[doc = "Disables DOM agent for the given page."]
pub struct DisableReturnObject(pub Option<Json>);
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[doc = "Discards search results from the session with the given id. `getSearchResults` should no longer\n be called for that search."]
pub struct DiscardSearchResultsReturnObject(pub Option<Json>);
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[doc = "Enables DOM agent for the given page."]
pub struct EnableReturnObject(pub Option<Json>);
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[doc = "Focuses the given element."]
pub struct FocusReturnObject(pub Option<Json>);
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[serde(rename_all = "camelCase")]
#[doc = "Returns attributes for the specified node."]
pub struct GetAttributesReturnObject {
    #[doc = "An interleaved array of node attribute names and values."]
    pub attributes: Vec<String>,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[serde(rename_all = "camelCase")]
#[doc = "Returns boxes for the given node."]
pub struct GetBoxModelReturnObject {
    #[doc = "Box model for the node."]
    pub model: BoxModel,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[serde(rename_all = "camelCase")]
#[doc = "Returns quads that describe node position on the page. This method\n might return multiple quads for inline nodes."]
pub struct GetContentQuadsReturnObject {
    #[doc = "Quads that describe node layout relative to viewport."]
    pub quads: Vec<Quad>,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[serde(rename_all = "camelCase")]
#[doc = "Returns the root DOM node (and optionally the subtree) to the caller.\n Implicitly enables the DOM domain events for the current target."]
pub struct GetDocumentReturnObject {
    #[doc = "Resulting node."]
    pub root: Node,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[serde(rename_all = "camelCase")]
#[doc = "Returns the root DOM node (and optionally the subtree) to the caller.\n Deprecated, as it is not designed to work well with the rest of the DOM agent.\n Use DOMSnapshot.captureSnapshot instead."]
#[deprecated]
pub struct GetFlattenedDocumentReturnObject {
    #[doc = "Resulting node."]
    pub nodes: Vec<Node>,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[serde(rename_all = "camelCase")]
#[doc = "Finds nodes with a given computed style in a subtree."]
pub struct GetNodesForSubtreeByStyleReturnObject {
    #[doc = "Resulting nodes."]
    pub node_ids: Vec<NodeId>,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[serde(rename_all = "camelCase")]
#[doc = "Returns node id at given location. Depending on whether DOM domain is enabled, nodeId is\n either returned or not."]
pub struct GetNodeForLocationReturnObject {
    #[doc = "Resulting node."]
    pub backend_node_id: BackendNodeId,
    #[doc = "Frame this node belongs to."]
    pub frame_id: page::FrameId,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[doc = "Id of the node at given coordinates, only when enabled and requested document."]
    pub node_id: Option<NodeId>,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[serde(rename_all = "camelCase")]
#[doc = "Returns node's HTML markup."]
pub struct GetOuterHTMLReturnObject {
    #[serde(default)]
    #[doc = "Outer HTML markup."]
    #[serde(rename = "outerHTML")]
    pub outer_html: String,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[serde(rename_all = "camelCase")]
#[doc = "Returns the id of the nearest ancestor that is a relayout boundary."]
pub struct GetRelayoutBoundaryReturnObject {
    #[doc = "Relayout boundary node id for the given node."]
    pub node_id: NodeId,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[serde(rename_all = "camelCase")]
#[doc = "Returns search results from given `fromIndex` to given `toIndex` from the search with the given\n identifier."]
pub struct GetSearchResultsReturnObject {
    #[doc = "Ids of the search result nodes."]
    pub node_ids: Vec<NodeId>,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[doc = "Hides any highlight."]
pub struct HideHighlightReturnObject(pub Option<Json>);
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[doc = "Highlights DOM node."]
pub struct HighlightNodeReturnObject(pub Option<Json>);
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[doc = "Highlights given rectangle."]
pub struct HighlightRectReturnObject(pub Option<Json>);
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[doc = "Marks last undoable state."]
pub struct MarkUndoableStateReturnObject(pub Option<Json>);
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[serde(rename_all = "camelCase")]
#[doc = "Moves node into the new container, places it before the given anchor."]
pub struct MoveToReturnObject {
    #[doc = "New id of the moved node."]
    pub node_id: NodeId,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[serde(rename_all = "camelCase")]
#[doc = "Searches for a given string in the DOM tree. Use `getSearchResults` to access search results or\n `cancelSearch` to end this search session."]
pub struct PerformSearchReturnObject {
    #[serde(default)]
    #[doc = "Unique search session identifier."]
    pub search_id: String,
    #[serde(default)]
    #[doc = "Number of search results."]
    pub result_count: JsUInt,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[serde(rename_all = "camelCase")]
#[doc = "Requests that the node is sent to the caller given its path. // FIXME, use XPath"]
pub struct PushNodeByPathToFrontendReturnObject {
    #[doc = "Id of the node for given path."]
    pub node_id: NodeId,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[serde(rename_all = "camelCase")]
#[doc = "Requests that a batch of nodes is sent to the caller given their backend node ids."]
pub struct PushNodesByBackendIdsToFrontendReturnObject {
    #[doc = "The array of ids of pushed nodes that correspond to the backend ids specified in\n backendNodeIds."]
    pub node_ids: Vec<NodeId>,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[serde(rename_all = "camelCase")]
#[doc = "Executes `querySelector` on a given node."]
pub struct QuerySelectorReturnObject {
    #[doc = "Query selector result."]
    pub node_id: NodeId,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[serde(rename_all = "camelCase")]
#[doc = "Executes `querySelectorAll` on a given node."]
pub struct QuerySelectorAllReturnObject {
    #[doc = "Query selector result."]
    pub node_ids: Vec<NodeId>,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[serde(rename_all = "camelCase")]
#[doc = "Returns NodeIds of current top layer elements.\n Top layer is rendered closest to the user within a viewport, therefore its elements always\n appear on top of all other content."]
pub struct GetTopLayerElementsReturnObject {
    #[doc = "NodeIds of top layer elements"]
    pub node_ids: Vec<NodeId>,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[serde(rename_all = "camelCase")]
#[doc = "Returns the NodeId of the matched element according to certain relations."]
pub struct GetElementByRelationReturnObject {
    #[doc = "NodeId of the element matching the queried relation."]
    pub node_id: NodeId,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[doc = "Re-does the last undone action."]
pub struct RedoReturnObject(pub Option<Json>);
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[doc = "Removes attribute with given name from an element with given id."]
pub struct RemoveAttributeReturnObject(pub Option<Json>);
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[doc = "Removes node with given id."]
pub struct RemoveNodeReturnObject(pub Option<Json>);
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[doc = "Requests that children of the node with given id are returned to the caller in form of\n `setChildNodes` events where not only immediate children are retrieved, but all children down to\n the specified depth."]
pub struct RequestChildNodesReturnObject(pub Option<Json>);
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[serde(rename_all = "camelCase")]
#[doc = "Requests that the node is sent to the caller given the JavaScript node object reference. All\n nodes that form the path from the node to the root are also sent to the client as a series of\n `setChildNodes` notifications."]
pub struct RequestNodeReturnObject {
    #[doc = "Node id for given object."]
    pub node_id: NodeId,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[serde(rename_all = "camelCase")]
#[doc = "Resolves the JavaScript node object for a given NodeId or BackendNodeId."]
pub struct ResolveNodeReturnObject {
    #[doc = "JavaScript object wrapper for given node."]
    pub object: runtime::RemoteObject,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[doc = "Sets attribute for an element with given id."]
pub struct SetAttributeValueReturnObject(pub Option<Json>);
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[doc = "Sets attributes on element with given id. This method is useful when user edits some existing\n attribute value and types in several attribute name/value pairs."]
pub struct SetAttributesAsTextReturnObject(pub Option<Json>);
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[doc = "Sets files for the given file input element."]
pub struct SetFileInputFilesReturnObject(pub Option<Json>);
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[doc = "Sets if stack traces should be captured for Nodes. See `Node.getNodeStackTraces`. Default is disabled."]
pub struct SetNodeStackTracesEnabledReturnObject(pub Option<Json>);
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[serde(rename_all = "camelCase")]
#[doc = "Gets stack traces associated with a Node. As of now, only provides stack trace for Node creation."]
pub struct GetNodeStackTracesReturnObject {
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[doc = "Creation stack trace, if available."]
    pub creation: Option<runtime::StackTrace>,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[serde(rename_all = "camelCase")]
#[doc = "Returns file information for the given\n File wrapper."]
pub struct GetFileInfoReturnObject {
    #[serde(default)]
    pub path: String,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[serde(rename_all = "camelCase")]
#[doc = "Returns list of detached nodes"]
pub struct GetDetachedDomNodesReturnObject {
    #[doc = "The list of detached nodes"]
    pub detached_nodes: Vec<DetachedElementInfo>,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[doc = "Enables console to refer to the node with given id via $x (see Command Line API for more details\n $x functions)."]
pub struct SetInspectedNodeReturnObject(pub Option<Json>);
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[serde(rename_all = "camelCase")]
#[doc = "Sets node name for a node with given id."]
pub struct SetNodeNameReturnObject {
    #[doc = "New node's id."]
    pub node_id: NodeId,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[doc = "Sets node value for a node with given id."]
pub struct SetNodeValueReturnObject(pub Option<Json>);
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[doc = "Sets node HTML markup, returns new node id."]
pub struct SetOuterHTMLReturnObject(pub Option<Json>);
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[doc = "Undoes the last performed action."]
pub struct UndoReturnObject(pub Option<Json>);
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[serde(rename_all = "camelCase")]
#[doc = "Returns iframe node that owns iframe with the given domain."]
pub struct GetFrameOwnerReturnObject {
    #[doc = "Resulting node."]
    pub backend_node_id: BackendNodeId,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[doc = "Id of the node at given coordinates, only when enabled and requested document."]
    pub node_id: Option<NodeId>,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[serde(rename_all = "camelCase")]
#[doc = "Returns the query container of the given node based on container query\n conditions: containerName, physical and logical axes, and whether it queries\n scroll-state or anchored elements. If no axes are provided and\n queriesScrollState is false, the style container is returned, which is the\n direct parent or the closest element with a matching container-name."]
pub struct GetContainerForNodeReturnObject {
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[doc = "The container node for the given node, or null if not found."]
    pub node_id: Option<NodeId>,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[serde(rename_all = "camelCase")]
#[doc = "Returns the descendants of a container query container that have\n container queries against this container."]
pub struct GetQueryingDescendantsForContainerReturnObject {
    #[doc = "Descendant nodes with container queries against the given container."]
    pub node_ids: Vec<NodeId>,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[serde(rename_all = "camelCase")]
#[doc = "Returns the target anchor element of the given anchor query according to\n https://www.w3.org/TR/css-anchor-position-1/#target."]
pub struct GetAnchorElementReturnObject {
    #[doc = "The anchor element of the given anchor query."]
    pub node_id: NodeId,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[serde(rename_all = "camelCase")]
#[doc = "When enabling, this API force-opens the popover identified by nodeId\n and keeps it open until disabled."]
pub struct ForceShowPopoverReturnObject {
    #[doc = "List of popovers that were closed in order to respect popover stacking order."]
    pub node_ids: Vec<NodeId>,
}
impl Method for CollectClassNamesFromSubtree {
    const NAME: &'static str = "DOM.collectClassNamesFromSubtree";
    type ReturnObject = CollectClassNamesFromSubtreeReturnObject;
}
impl Method for CopyTo {
    const NAME: &'static str = "DOM.copyTo";
    type ReturnObject = CopyToReturnObject;
}
impl Method for DescribeNode {
    const NAME: &'static str = "DOM.describeNode";
    type ReturnObject = DescribeNodeReturnObject;
}
impl Method for ScrollIntoViewIfNeeded {
    const NAME: &'static str = "DOM.scrollIntoViewIfNeeded";
    type ReturnObject = ScrollIntoViewIfNeededReturnObject;
}
impl Method for Disable {
    const NAME: &'static str = "DOM.disable";
    type ReturnObject = DisableReturnObject;
}
impl Method for DiscardSearchResults {
    const NAME: &'static str = "DOM.discardSearchResults";
    type ReturnObject = DiscardSearchResultsReturnObject;
}
impl Method for Enable {
    const NAME: &'static str = "DOM.enable";
    type ReturnObject = EnableReturnObject;
}
impl Method for Focus {
    const NAME: &'static str = "DOM.focus";
    type ReturnObject = FocusReturnObject;
}
impl Method for GetAttributes {
    const NAME: &'static str = "DOM.getAttributes";
    type ReturnObject = GetAttributesReturnObject;
}
impl Method for GetBoxModel {
    const NAME: &'static str = "DOM.getBoxModel";
    type ReturnObject = GetBoxModelReturnObject;
}
impl Method for GetContentQuads {
    const NAME: &'static str = "DOM.getContentQuads";
    type ReturnObject = GetContentQuadsReturnObject;
}
impl Method for GetDocument {
    const NAME: &'static str = "DOM.getDocument";
    type ReturnObject = GetDocumentReturnObject;
}
impl Method for GetFlattenedDocument {
    const NAME: &'static str = "DOM.getFlattenedDocument";
    type ReturnObject = GetFlattenedDocumentReturnObject;
}
impl Method for GetNodesForSubtreeByStyle {
    const NAME: &'static str = "DOM.getNodesForSubtreeByStyle";
    type ReturnObject = GetNodesForSubtreeByStyleReturnObject;
}
impl Method for GetNodeForLocation {
    const NAME: &'static str = "DOM.getNodeForLocation";
    type ReturnObject = GetNodeForLocationReturnObject;
}
impl Method for GetOuterHTML {
    const NAME: &'static str = "DOM.getOuterHTML";
    type ReturnObject = GetOuterHTMLReturnObject;
}
impl Method for GetRelayoutBoundary {
    const NAME: &'static str = "DOM.getRelayoutBoundary";
    type ReturnObject = GetRelayoutBoundaryReturnObject;
}
impl Method for GetSearchResults {
    const NAME: &'static str = "DOM.getSearchResults";
    type ReturnObject = GetSearchResultsReturnObject;
}
impl Method for HideHighlight {
    const NAME: &'static str = "DOM.hideHighlight";
    type ReturnObject = HideHighlightReturnObject;
}
impl Method for HighlightNode {
    const NAME: &'static str = "DOM.highlightNode";
    type ReturnObject = HighlightNodeReturnObject;
}
impl Method for HighlightRect {
    const NAME: &'static str = "DOM.highlightRect";
    type ReturnObject = HighlightRectReturnObject;
}
impl Method for MarkUndoableState {
    const NAME: &'static str = "DOM.markUndoableState";
    type ReturnObject = MarkUndoableStateReturnObject;
}
impl Method for MoveTo {
    const NAME: &'static str = "DOM.moveTo";
    type ReturnObject = MoveToReturnObject;
}
impl Method for PerformSearch {
    const NAME: &'static str = "DOM.performSearch";
    type ReturnObject = PerformSearchReturnObject;
}
impl Method for PushNodeByPathToFrontend {
    const NAME: &'static str = "DOM.pushNodeByPathToFrontend";
    type ReturnObject = PushNodeByPathToFrontendReturnObject;
}
impl Method for PushNodesByBackendIdsToFrontend {
    const NAME: &'static str = "DOM.pushNodesByBackendIdsToFrontend";
    type ReturnObject = PushNodesByBackendIdsToFrontendReturnObject;
}
impl Method for QuerySelector {
    const NAME: &'static str = "DOM.querySelector";
    type ReturnObject = QuerySelectorReturnObject;
}
impl Method for QuerySelectorAll {
    const NAME: &'static str = "DOM.querySelectorAll";
    type ReturnObject = QuerySelectorAllReturnObject;
}
impl Method for GetTopLayerElements {
    const NAME: &'static str = "DOM.getTopLayerElements";
    type ReturnObject = GetTopLayerElementsReturnObject;
}
impl Method for GetElementByRelation {
    const NAME: &'static str = "DOM.getElementByRelation";
    type ReturnObject = GetElementByRelationReturnObject;
}
impl Method for Redo {
    const NAME: &'static str = "DOM.redo";
    type ReturnObject = RedoReturnObject;
}
impl Method for RemoveAttribute {
    const NAME: &'static str = "DOM.removeAttribute";
    type ReturnObject = RemoveAttributeReturnObject;
}
impl Method for RemoveNode {
    const NAME: &'static str = "DOM.removeNode";
    type ReturnObject = RemoveNodeReturnObject;
}
impl Method for RequestChildNodes {
    const NAME: &'static str = "DOM.requestChildNodes";
    type ReturnObject = RequestChildNodesReturnObject;
}
impl Method for RequestNode {
    const NAME: &'static str = "DOM.requestNode";
    type ReturnObject = RequestNodeReturnObject;
}
impl Method for ResolveNode {
    const NAME: &'static str = "DOM.resolveNode";
    type ReturnObject = ResolveNodeReturnObject;
}
impl Method for SetAttributeValue {
    const NAME: &'static str = "DOM.setAttributeValue";
    type ReturnObject = SetAttributeValueReturnObject;
}
impl Method for SetAttributesAsText {
    const NAME: &'static str = "DOM.setAttributesAsText";
    type ReturnObject = SetAttributesAsTextReturnObject;
}
impl Method for SetFileInputFiles {
    const NAME: &'static str = "DOM.setFileInputFiles";
    type ReturnObject = SetFileInputFilesReturnObject;
}
impl Method for SetNodeStackTracesEnabled {
    const NAME: &'static str = "DOM.setNodeStackTracesEnabled";
    type ReturnObject = SetNodeStackTracesEnabledReturnObject;
}
impl Method for GetNodeStackTraces {
    const NAME: &'static str = "DOM.getNodeStackTraces";
    type ReturnObject = GetNodeStackTracesReturnObject;
}
impl Method for GetFileInfo {
    const NAME: &'static str = "DOM.getFileInfo";
    type ReturnObject = GetFileInfoReturnObject;
}
impl Method for GetDetachedDomNodes {
    const NAME: &'static str = "DOM.getDetachedDomNodes";
    type ReturnObject = GetDetachedDomNodesReturnObject;
}
impl Method for SetInspectedNode {
    const NAME: &'static str = "DOM.setInspectedNode";
    type ReturnObject = SetInspectedNodeReturnObject;
}
impl Method for SetNodeName {
    const NAME: &'static str = "DOM.setNodeName";
    type ReturnObject = SetNodeNameReturnObject;
}
impl Method for SetNodeValue {
    const NAME: &'static str = "DOM.setNodeValue";
    type ReturnObject = SetNodeValueReturnObject;
}
impl Method for SetOuterHTML {
    const NAME: &'static str = "DOM.setOuterHTML";
    type ReturnObject = SetOuterHTMLReturnObject;
}
impl Method for Undo {
    const NAME: &'static str = "DOM.undo";
    type ReturnObject = UndoReturnObject;
}
impl Method for GetFrameOwner {
    const NAME: &'static str = "DOM.getFrameOwner";
    type ReturnObject = GetFrameOwnerReturnObject;
}
impl Method for GetContainerForNode {
    const NAME: &'static str = "DOM.getContainerForNode";
    type ReturnObject = GetContainerForNodeReturnObject;
}
impl Method for GetQueryingDescendantsForContainer {
    const NAME: &'static str = "DOM.getQueryingDescendantsForContainer";
    type ReturnObject = GetQueryingDescendantsForContainerReturnObject;
}
impl Method for GetAnchorElement {
    const NAME: &'static str = "DOM.getAnchorElement";
    type ReturnObject = GetAnchorElementReturnObject;
}
impl Method for ForceShowPopover {
    const NAME: &'static str = "DOM.forceShowPopover";
    type ReturnObject = ForceShowPopoverReturnObject;
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
    pub struct AttributeModifiedEvent {
        pub params: AttributeModifiedEventParams,
    }
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
    #[serde(rename_all = "camelCase")]
    pub struct AttributeModifiedEventParams {
        #[doc = "Id of the node that has changed."]
        pub node_id: super::NodeId,
        #[serde(default)]
        #[doc = "Attribute name."]
        pub name: String,
        #[serde(default)]
        #[doc = "Attribute value."]
        pub value: String,
    }
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
    pub struct AdoptedStyleSheetsModifiedEvent {
        pub params: AdoptedStyleSheetsModifiedEventParams,
    }
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
    #[serde(rename_all = "camelCase")]
    pub struct AdoptedStyleSheetsModifiedEventParams {
        #[doc = "Id of the node that has changed."]
        pub node_id: super::NodeId,
        #[doc = "New adoptedStyleSheets array."]
        pub adopted_style_sheets: Vec<super::StyleSheetId>,
    }
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
    pub struct AttributeRemovedEvent {
        pub params: AttributeRemovedEventParams,
    }
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
    #[serde(rename_all = "camelCase")]
    pub struct AttributeRemovedEventParams {
        #[doc = "Id of the node that has changed."]
        pub node_id: super::NodeId,
        #[serde(default)]
        #[doc = "A ttribute name."]
        pub name: String,
    }
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
    pub struct CharacterDataModifiedEvent {
        pub params: CharacterDataModifiedEventParams,
    }
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
    #[serde(rename_all = "camelCase")]
    pub struct CharacterDataModifiedEventParams {
        #[doc = "Id of the node that has changed."]
        pub node_id: super::NodeId,
        #[serde(default)]
        #[doc = "New text value."]
        pub character_data: String,
    }
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
    pub struct ChildNodeCountUpdatedEvent {
        pub params: ChildNodeCountUpdatedEventParams,
    }
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
    #[serde(rename_all = "camelCase")]
    pub struct ChildNodeCountUpdatedEventParams {
        #[doc = "Id of the node that has changed."]
        pub node_id: super::NodeId,
        #[serde(default)]
        #[doc = "New node count."]
        pub child_node_count: JsUInt,
    }
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
    pub struct ChildNodeInsertedEvent {
        pub params: ChildNodeInsertedEventParams,
    }
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
    #[serde(rename_all = "camelCase")]
    pub struct ChildNodeInsertedEventParams {
        #[doc = "Id of the node that has changed."]
        pub parent_node_id: super::NodeId,
        #[doc = "Id of the previous sibling."]
        pub previous_node_id: super::NodeId,
        #[doc = "Inserted node data."]
        pub node: super::Node,
    }
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
    pub struct ChildNodeRemovedEvent {
        pub params: ChildNodeRemovedEventParams,
    }
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
    #[serde(rename_all = "camelCase")]
    pub struct ChildNodeRemovedEventParams {
        #[doc = "Parent id."]
        pub parent_node_id: super::NodeId,
        #[doc = "Id of the node that has been removed."]
        pub node_id: super::NodeId,
    }
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
    pub struct DistributedNodesUpdatedEvent {
        pub params: DistributedNodesUpdatedEventParams,
    }
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
    #[serde(rename_all = "camelCase")]
    pub struct DistributedNodesUpdatedEventParams {
        #[doc = "Insertion point where distributed nodes were updated."]
        pub insertion_point_id: super::NodeId,
        #[doc = "Distributed nodes for given insertion point."]
        pub distributed_nodes: Vec<super::BackendNode>,
    }
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
    pub struct DocumentUpdatedEvent(pub Option<Json>);
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
    pub struct InlineStyleInvalidatedEvent {
        pub params: InlineStyleInvalidatedEventParams,
    }
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
    #[serde(rename_all = "camelCase")]
    pub struct InlineStyleInvalidatedEventParams {
        #[doc = "Ids of the nodes for which the inline styles have been invalidated."]
        pub node_ids: Vec<super::NodeId>,
    }
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
    pub struct PseudoElementAddedEvent {
        pub params: PseudoElementAddedEventParams,
    }
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
    #[serde(rename_all = "camelCase")]
    pub struct PseudoElementAddedEventParams {
        #[doc = "Pseudo element's parent element id."]
        pub parent_id: super::NodeId,
        #[doc = "The added pseudo element."]
        pub pseudo_element: super::Node,
    }
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
    pub struct TopLayerElementsUpdatedEvent(pub Option<Json>);
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
    pub struct ScrollableFlagUpdatedEvent {
        pub params: ScrollableFlagUpdatedEventParams,
    }
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
    #[serde(rename_all = "camelCase")]
    pub struct ScrollableFlagUpdatedEventParams {
        #[doc = "The id of the node."]
        pub node_id: super::super::dom::NodeId,
        #[serde(default)]
        #[doc = "If the node is scrollable."]
        pub is_scrollable: bool,
    }
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
    pub struct AffectedByStartingStylesFlagUpdatedEvent {
        pub params: AffectedByStartingStylesFlagUpdatedEventParams,
    }
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
    #[serde(rename_all = "camelCase")]
    pub struct AffectedByStartingStylesFlagUpdatedEventParams {
        #[doc = "The id of the node."]
        pub node_id: super::super::dom::NodeId,
        #[serde(default)]
        #[doc = "If the node has starting styles."]
        pub affected_by_starting_styles: bool,
    }
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
    pub struct PseudoElementRemovedEvent {
        pub params: PseudoElementRemovedEventParams,
    }
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
    #[serde(rename_all = "camelCase")]
    pub struct PseudoElementRemovedEventParams {
        #[doc = "Pseudo element's parent element id."]
        pub parent_id: super::NodeId,
        #[doc = "The removed pseudo element id."]
        pub pseudo_element_id: super::NodeId,
    }
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
    pub struct SetChildNodesEvent {
        pub params: SetChildNodesEventParams,
    }
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
    #[serde(rename_all = "camelCase")]
    pub struct SetChildNodesEventParams {
        #[doc = "Parent node id to populate with children."]
        pub parent_id: super::NodeId,
        #[doc = "Child nodes array."]
        pub nodes: Vec<super::Node>,
    }
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
    pub struct ShadowRootPoppedEvent {
        pub params: ShadowRootPoppedEventParams,
    }
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
    #[serde(rename_all = "camelCase")]
    pub struct ShadowRootPoppedEventParams {
        #[doc = "Host element id."]
        pub host_id: super::NodeId,
        #[doc = "Shadow root id."]
        pub root_id: super::NodeId,
    }
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
    pub struct ShadowRootPushedEvent {
        pub params: ShadowRootPushedEventParams,
    }
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
    #[serde(rename_all = "camelCase")]
    pub struct ShadowRootPushedEventParams {
        #[doc = "Host element id."]
        pub host_id: super::NodeId,
        #[doc = "Shadow root."]
        pub root: super::Node,
    }
}
