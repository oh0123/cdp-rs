// Auto-generated from Chrome at version 143.0.7499.110 domain: DOM
use super::dom;
use super::page;
use super::runtime;
#[allow(unused_imports)]
use super::types::*;
#[allow(unused_imports)]
use serde::{Deserialize, Serialize};
#[allow(unused_imports)]
use serde_json::Value as Json;
pub type NodeId = JsUInt;
pub type BackendNodeId = JsUInt;
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
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct BackendNode {
    #[serde(default)]
    #[serde(rename = "nodeType")]
    pub node_type: JsUInt,
    #[serde(default)]
    #[serde(rename = "nodeName")]
    pub node_name: String,
    #[serde(rename = "backendNodeId")]
    pub backend_node_id: BackendNodeId,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct Node {
    #[serde(rename = "nodeId")]
    pub node_id: NodeId,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "parentId")]
    pub parent_id: Option<NodeId>,
    #[serde(rename = "backendNodeId")]
    pub backend_node_id: BackendNodeId,
    #[serde(default)]
    #[serde(rename = "nodeType")]
    pub node_type: JsUInt,
    #[serde(default)]
    #[serde(rename = "nodeName")]
    pub node_name: String,
    #[serde(default)]
    #[serde(rename = "localName")]
    pub local_name: String,
    #[serde(default)]
    #[serde(rename = "nodeValue")]
    pub node_value: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(rename = "childNodeCount")]
    pub child_node_count: Option<JsUInt>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "children")]
    pub children: Option<Vec<Node>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(rename = "attributes")]
    pub attributes: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(rename = "documentURL")]
    pub document_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(rename = "baseURL")]
    pub base_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(rename = "publicId")]
    pub public_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(rename = "systemId")]
    pub system_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(rename = "internalSubset")]
    pub internal_subset: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(rename = "xmlVersion")]
    pub xml_version: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(rename = "name")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(rename = "value")]
    pub value: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "pseudoType")]
    pub pseudo_type: Option<PseudoType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(rename = "pseudoIdentifier")]
    pub pseudo_identifier: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "shadowRootType")]
    pub shadow_root_type: Option<ShadowRootType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "frameId")]
    pub frame_id: Option<page::FrameId>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "contentDocument")]
    pub content_document: Option<Box<Node>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "shadowRoots")]
    pub shadow_roots: Option<Vec<Node>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "templateContent")]
    pub template_content: Option<Box<Node>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "pseudoElements")]
    pub pseudo_elements: Option<Vec<Node>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "importedDocument")]
    pub imported_document: Option<Box<Node>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "distributedNodes")]
    pub distributed_nodes: Option<Vec<BackendNode>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(rename = "isSVG")]
    pub is_svg: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "compatibilityMode")]
    pub compatibility_mode: Option<CompatibilityMode>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "assignedSlot")]
    pub assigned_slot: Option<BackendNode>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(rename = "isScrollable")]
    pub is_scrollable: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(rename = "affectedByStartingStyles")]
    pub affected_by_starting_styles: Option<bool>,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct DetachedElementInfo {
    #[serde(rename = "treeNode")]
    pub tree_node: Node,
    #[serde(rename = "retainedNodeIds")]
    pub retained_node_ids: Vec<NodeId>,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct Rgba {
    #[serde(default)]
    #[serde(rename = "r")]
    pub r: JsUInt,
    #[serde(default)]
    #[serde(rename = "g")]
    pub g: JsUInt,
    #[serde(default)]
    #[serde(rename = "b")]
    pub b: JsUInt,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(rename = "a")]
    pub a: Option<JsFloat>,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct BoxModel {
    #[serde(rename = "content")]
    pub content: Quad,
    #[serde(rename = "padding")]
    pub padding: Quad,
    #[serde(rename = "border")]
    pub border: Quad,
    #[serde(rename = "margin")]
    pub margin: Quad,
    #[serde(default)]
    #[serde(rename = "width")]
    pub width: JsUInt,
    #[serde(default)]
    #[serde(rename = "height")]
    pub height: JsUInt,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "shapeOutside")]
    pub shape_outside: Option<ShapeOutsideInfo>,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct ShapeOutsideInfo {
    #[serde(rename = "bounds")]
    pub bounds: Quad,
    #[serde(default)]
    #[serde(rename = "shape")]
    pub shape: Vec<Json>,
    #[serde(default)]
    #[serde(rename = "marginShape")]
    pub margin_shape: Vec<Json>,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct Rect {
    #[serde(default)]
    #[serde(rename = "x")]
    pub x: JsFloat,
    #[serde(default)]
    #[serde(rename = "y")]
    pub y: JsFloat,
    #[serde(default)]
    #[serde(rename = "width")]
    pub width: JsFloat,
    #[serde(default)]
    #[serde(rename = "height")]
    pub height: JsFloat,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct CssComputedStyleProperty {
    #[serde(default)]
    #[serde(rename = "name")]
    pub name: String,
    #[serde(default)]
    #[serde(rename = "value")]
    pub value: String,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct CollectClassNamesFromSubtree {
    #[serde(rename = "nodeId")]
    pub node_id: NodeId,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct CopyTo {
    #[serde(rename = "nodeId")]
    pub node_id: NodeId,
    #[serde(rename = "targetNodeId")]
    pub target_node_id: NodeId,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "insertBeforeNodeId")]
    pub insert_before_node_id: Option<NodeId>,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct DescribeNode {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "nodeId")]
    pub node_id: Option<NodeId>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "backendNodeId")]
    pub backend_node_id: Option<BackendNodeId>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "objectId")]
    pub object_id: Option<runtime::RemoteObjectId>,
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
pub struct ScrollIntoViewIfNeeded {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "nodeId")]
    pub node_id: Option<NodeId>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "backendNodeId")]
    pub backend_node_id: Option<BackendNodeId>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "objectId")]
    pub object_id: Option<runtime::RemoteObjectId>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "rect")]
    pub rect: Option<Rect>,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Disable(pub Option<serde_json::Value>);
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct DiscardSearchResults {
    #[serde(default)]
    #[serde(rename = "searchId")]
    pub search_id: String,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct Enable {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "includeWhitespace")]
    pub include_whitespace: Option<EnableIncludeWhitespaceOption>,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct Focus {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "nodeId")]
    pub node_id: Option<NodeId>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "backendNodeId")]
    pub backend_node_id: Option<BackendNodeId>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "objectId")]
    pub object_id: Option<runtime::RemoteObjectId>,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct GetAttributes {
    #[serde(rename = "nodeId")]
    pub node_id: NodeId,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct GetBoxModel {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "nodeId")]
    pub node_id: Option<NodeId>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "backendNodeId")]
    pub backend_node_id: Option<BackendNodeId>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "objectId")]
    pub object_id: Option<runtime::RemoteObjectId>,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct GetContentQuads {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "nodeId")]
    pub node_id: Option<NodeId>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "backendNodeId")]
    pub backend_node_id: Option<BackendNodeId>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "objectId")]
    pub object_id: Option<runtime::RemoteObjectId>,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct GetDocument {
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
pub struct GetFlattenedDocument {
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
pub struct GetNodesForSubtreeByStyle {
    #[serde(rename = "nodeId")]
    pub node_id: NodeId,
    #[serde(rename = "computedStyles")]
    pub computed_styles: Vec<CssComputedStyleProperty>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(rename = "pierce")]
    pub pierce: Option<bool>,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct GetNodeForLocation {
    #[serde(default)]
    #[serde(rename = "x")]
    pub x: JsUInt,
    #[serde(default)]
    #[serde(rename = "y")]
    pub y: JsUInt,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(rename = "includeUserAgentShadowDOM")]
    pub include_user_agent_shadow_dom: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(rename = "ignorePointerEventsNone")]
    pub ignore_pointer_events_none: Option<bool>,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct GetOuterHTML {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "nodeId")]
    pub node_id: Option<NodeId>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "backendNodeId")]
    pub backend_node_id: Option<BackendNodeId>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "objectId")]
    pub object_id: Option<runtime::RemoteObjectId>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(rename = "includeShadowDOM")]
    pub include_shadow_dom: Option<bool>,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct GetRelayoutBoundary {
    #[serde(rename = "nodeId")]
    pub node_id: NodeId,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct GetSearchResults {
    #[serde(default)]
    #[serde(rename = "searchId")]
    pub search_id: String,
    #[serde(default)]
    #[serde(rename = "fromIndex")]
    pub from_index: JsUInt,
    #[serde(default)]
    #[serde(rename = "toIndex")]
    pub to_index: JsUInt,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct HideHighlight(pub Option<serde_json::Value>);
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct HighlightNode(pub Option<serde_json::Value>);
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct HighlightRect(pub Option<serde_json::Value>);
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct MarkUndoableState(pub Option<serde_json::Value>);
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct MoveTo {
    #[serde(rename = "nodeId")]
    pub node_id: NodeId,
    #[serde(rename = "targetNodeId")]
    pub target_node_id: NodeId,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "insertBeforeNodeId")]
    pub insert_before_node_id: Option<NodeId>,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct PerformSearch {
    #[serde(default)]
    #[serde(rename = "query")]
    pub query: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(rename = "includeUserAgentShadowDOM")]
    pub include_user_agent_shadow_dom: Option<bool>,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct PushNodeByPathToFrontend {
    #[serde(default)]
    #[serde(rename = "path")]
    pub path: String,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct PushNodesByBackendIdsToFrontend {
    #[serde(rename = "backendNodeIds")]
    pub backend_node_ids: Vec<BackendNodeId>,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct QuerySelector {
    #[serde(rename = "nodeId")]
    pub node_id: NodeId,
    #[serde(default)]
    #[serde(rename = "selector")]
    pub selector: String,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct QuerySelectorAll {
    #[serde(rename = "nodeId")]
    pub node_id: NodeId,
    #[serde(default)]
    #[serde(rename = "selector")]
    pub selector: String,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct GetTopLayerElements(pub Option<serde_json::Value>);
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct GetElementByRelation {
    #[serde(rename = "nodeId")]
    pub node_id: NodeId,
    #[serde(rename = "relation")]
    pub relation: GetElementByRelationRelationOption,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Redo(pub Option<serde_json::Value>);
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct RemoveAttribute {
    #[serde(rename = "nodeId")]
    pub node_id: NodeId,
    #[serde(default)]
    #[serde(rename = "name")]
    pub name: String,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct RemoveNode {
    #[serde(rename = "nodeId")]
    pub node_id: NodeId,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct RequestChildNodes {
    #[serde(rename = "nodeId")]
    pub node_id: NodeId,
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
pub struct RequestNode {
    #[serde(rename = "objectId")]
    pub object_id: runtime::RemoteObjectId,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct ResolveNode {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "nodeId")]
    pub node_id: Option<NodeId>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "backendNodeId")]
    pub backend_node_id: Option<dom::BackendNodeId>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(rename = "objectGroup")]
    pub object_group: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "executionContextId")]
    pub execution_context_id: Option<runtime::ExecutionContextId>,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct SetAttributeValue {
    #[serde(rename = "nodeId")]
    pub node_id: NodeId,
    #[serde(default)]
    #[serde(rename = "name")]
    pub name: String,
    #[serde(default)]
    #[serde(rename = "value")]
    pub value: String,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct SetAttributesAsText {
    #[serde(rename = "nodeId")]
    pub node_id: NodeId,
    #[serde(default)]
    #[serde(rename = "text")]
    pub text: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(rename = "name")]
    pub name: Option<String>,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct SetFileInputFiles {
    #[serde(default)]
    #[serde(rename = "files")]
    pub files: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "nodeId")]
    pub node_id: Option<NodeId>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "backendNodeId")]
    pub backend_node_id: Option<BackendNodeId>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "objectId")]
    pub object_id: Option<runtime::RemoteObjectId>,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct SetNodeStackTracesEnabled {
    #[serde(default)]
    #[serde(rename = "enable")]
    pub enable: bool,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct GetNodeStackTraces {
    #[serde(rename = "nodeId")]
    pub node_id: NodeId,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct GetFileInfo {
    #[serde(rename = "objectId")]
    pub object_id: runtime::RemoteObjectId,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct GetDetachedDomNodes(pub Option<serde_json::Value>);
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct SetInspectedNode {
    #[serde(rename = "nodeId")]
    pub node_id: NodeId,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct SetNodeName {
    #[serde(rename = "nodeId")]
    pub node_id: NodeId,
    #[serde(default)]
    #[serde(rename = "name")]
    pub name: String,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct SetNodeValue {
    #[serde(rename = "nodeId")]
    pub node_id: NodeId,
    #[serde(default)]
    #[serde(rename = "value")]
    pub value: String,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct SetOuterHTML {
    #[serde(rename = "nodeId")]
    pub node_id: NodeId,
    #[serde(default)]
    #[serde(rename = "outerHTML")]
    pub outer_html: String,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Undo(pub Option<serde_json::Value>);
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct GetFrameOwner {
    #[serde(rename = "frameId")]
    pub frame_id: page::FrameId,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct GetContainerForNode {
    #[serde(rename = "nodeId")]
    pub node_id: NodeId,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(rename = "containerName")]
    pub container_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "physicalAxes")]
    pub physical_axes: Option<PhysicalAxes>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "logicalAxes")]
    pub logical_axes: Option<LogicalAxes>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(rename = "queriesScrollState")]
    pub queries_scroll_state: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(rename = "queriesAnchored")]
    pub queries_anchored: Option<bool>,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct GetQueryingDescendantsForContainer {
    #[serde(rename = "nodeId")]
    pub node_id: NodeId,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct GetAnchorElement {
    #[serde(rename = "nodeId")]
    pub node_id: NodeId,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(rename = "anchorSpecifier")]
    pub anchor_specifier: Option<String>,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct ForceShowPopover {
    #[serde(rename = "nodeId")]
    pub node_id: NodeId,
    #[serde(default)]
    #[serde(rename = "enable")]
    pub enable: bool,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct CollectClassNamesFromSubtreeReturnObject {
    #[serde(rename = "classNames")]
    pub class_names: Vec<String>,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct CopyToReturnObject {
    #[serde(rename = "nodeId")]
    pub node_id: NodeId,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct DescribeNodeReturnObject {
    #[serde(rename = "node")]
    pub node: Node,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct ScrollIntoViewIfNeededReturnObject {}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct DisableReturnObject {}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct DiscardSearchResultsReturnObject {}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct EnableReturnObject {}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct FocusReturnObject {}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct GetAttributesReturnObject {
    #[serde(rename = "attributes")]
    pub attributes: Vec<String>,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct GetBoxModelReturnObject {
    #[serde(rename = "model")]
    pub model: BoxModel,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct GetContentQuadsReturnObject {
    #[serde(rename = "quads")]
    pub quads: Vec<Quad>,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct GetDocumentReturnObject {
    #[serde(rename = "root")]
    pub root: Node,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct GetFlattenedDocumentReturnObject {
    #[serde(rename = "nodes")]
    pub nodes: Vec<Node>,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct GetNodesForSubtreeByStyleReturnObject {
    #[serde(rename = "nodeIds")]
    pub node_ids: Vec<NodeId>,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct GetNodeForLocationReturnObject {
    #[serde(rename = "backendNodeId")]
    pub backend_node_id: BackendNodeId,
    #[serde(rename = "frameId")]
    pub frame_id: page::FrameId,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "nodeId")]
    pub node_id: Option<NodeId>,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct GetOuterHTMLReturnObject {
    #[serde(default)]
    #[serde(rename = "outerHTML")]
    pub outer_html: String,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct GetRelayoutBoundaryReturnObject {
    #[serde(rename = "nodeId")]
    pub node_id: NodeId,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct GetSearchResultsReturnObject {
    #[serde(rename = "nodeIds")]
    pub node_ids: Vec<NodeId>,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct HideHighlightReturnObject {}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct HighlightNodeReturnObject {}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct HighlightRectReturnObject {}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct MarkUndoableStateReturnObject {}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct MoveToReturnObject {
    #[serde(rename = "nodeId")]
    pub node_id: NodeId,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct PerformSearchReturnObject {
    #[serde(default)]
    #[serde(rename = "searchId")]
    pub search_id: String,
    #[serde(default)]
    #[serde(rename = "resultCount")]
    pub result_count: JsUInt,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct PushNodeByPathToFrontendReturnObject {
    #[serde(rename = "nodeId")]
    pub node_id: NodeId,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct PushNodesByBackendIdsToFrontendReturnObject {
    #[serde(rename = "nodeIds")]
    pub node_ids: Vec<NodeId>,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct QuerySelectorReturnObject {
    #[serde(rename = "nodeId")]
    pub node_id: NodeId,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct QuerySelectorAllReturnObject {
    #[serde(rename = "nodeIds")]
    pub node_ids: Vec<NodeId>,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct GetTopLayerElementsReturnObject {
    #[serde(rename = "nodeIds")]
    pub node_ids: Vec<NodeId>,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct GetElementByRelationReturnObject {
    #[serde(rename = "nodeId")]
    pub node_id: NodeId,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct RedoReturnObject {}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct RemoveAttributeReturnObject {}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct RemoveNodeReturnObject {}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct RequestChildNodesReturnObject {}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct RequestNodeReturnObject {
    #[serde(rename = "nodeId")]
    pub node_id: NodeId,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct ResolveNodeReturnObject {
    #[serde(rename = "object")]
    pub object: runtime::RemoteObject,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct SetAttributeValueReturnObject {}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct SetAttributesAsTextReturnObject {}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct SetFileInputFilesReturnObject {}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct SetNodeStackTracesEnabledReturnObject {}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct GetNodeStackTracesReturnObject {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "creation")]
    pub creation: Option<runtime::StackTrace>,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct GetFileInfoReturnObject {
    #[serde(default)]
    #[serde(rename = "path")]
    pub path: String,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct GetDetachedDomNodesReturnObject {
    #[serde(rename = "detachedNodes")]
    pub detached_nodes: Vec<DetachedElementInfo>,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct SetInspectedNodeReturnObject {}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct SetNodeNameReturnObject {
    #[serde(rename = "nodeId")]
    pub node_id: NodeId,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct SetNodeValueReturnObject {}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct SetOuterHTMLReturnObject {}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct UndoReturnObject {}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct GetFrameOwnerReturnObject {
    #[serde(rename = "backendNodeId")]
    pub backend_node_id: BackendNodeId,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "nodeId")]
    pub node_id: Option<NodeId>,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct GetContainerForNodeReturnObject {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "nodeId")]
    pub node_id: Option<NodeId>,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct GetQueryingDescendantsForContainerReturnObject {
    #[serde(rename = "nodeIds")]
    pub node_ids: Vec<NodeId>,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct GetAnchorElementReturnObject {
    #[serde(rename = "nodeId")]
    pub node_id: NodeId,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct ForceShowPopoverReturnObject {
    #[serde(rename = "nodeIds")]
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
    use serde::{Deserialize, Serialize};
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
    pub struct AttributeModifiedEvent {
        pub params: AttributeModifiedEventParams,
    }
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
    pub struct AttributeModifiedEventParams {
        #[serde(rename = "nodeId")]
        pub node_id: super::NodeId,
        #[serde(default)]
        #[serde(rename = "name")]
        pub name: String,
        #[serde(default)]
        #[serde(rename = "value")]
        pub value: String,
    }
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
    pub struct AttributeRemovedEvent {
        pub params: AttributeRemovedEventParams,
    }
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
    pub struct AttributeRemovedEventParams {
        #[serde(rename = "nodeId")]
        pub node_id: super::NodeId,
        #[serde(default)]
        #[serde(rename = "name")]
        pub name: String,
    }
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
    pub struct CharacterDataModifiedEvent {
        pub params: CharacterDataModifiedEventParams,
    }
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
    pub struct CharacterDataModifiedEventParams {
        #[serde(rename = "nodeId")]
        pub node_id: super::NodeId,
        #[serde(default)]
        #[serde(rename = "characterData")]
        pub character_data: String,
    }
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
    pub struct ChildNodeCountUpdatedEvent {
        pub params: ChildNodeCountUpdatedEventParams,
    }
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
    pub struct ChildNodeCountUpdatedEventParams {
        #[serde(rename = "nodeId")]
        pub node_id: super::NodeId,
        #[serde(default)]
        #[serde(rename = "childNodeCount")]
        pub child_node_count: JsUInt,
    }
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
    pub struct ChildNodeInsertedEvent {
        pub params: ChildNodeInsertedEventParams,
    }
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
    pub struct ChildNodeInsertedEventParams {
        #[serde(rename = "parentNodeId")]
        pub parent_node_id: super::NodeId,
        #[serde(rename = "previousNodeId")]
        pub previous_node_id: super::NodeId,
        #[serde(rename = "node")]
        pub node: super::Node,
    }
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
    pub struct ChildNodeRemovedEvent {
        pub params: ChildNodeRemovedEventParams,
    }
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
    pub struct ChildNodeRemovedEventParams {
        #[serde(rename = "parentNodeId")]
        pub parent_node_id: super::NodeId,
        #[serde(rename = "nodeId")]
        pub node_id: super::NodeId,
    }
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
    pub struct DistributedNodesUpdatedEvent {
        pub params: DistributedNodesUpdatedEventParams,
    }
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
    pub struct DistributedNodesUpdatedEventParams {
        #[serde(rename = "insertionPointId")]
        pub insertion_point_id: super::NodeId,
        #[serde(rename = "distributedNodes")]
        pub distributed_nodes: Vec<super::BackendNode>,
    }
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
    #[serde(rename_all = "camelCase")]
    pub struct DocumentUpdatedEvent(pub Option<serde_json::Value>);
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
    pub struct InlineStyleInvalidatedEvent {
        pub params: InlineStyleInvalidatedEventParams,
    }
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
    pub struct InlineStyleInvalidatedEventParams {
        #[serde(rename = "nodeIds")]
        pub node_ids: Vec<super::NodeId>,
    }
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
    pub struct PseudoElementAddedEvent {
        pub params: PseudoElementAddedEventParams,
    }
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
    pub struct PseudoElementAddedEventParams {
        #[serde(rename = "parentId")]
        pub parent_id: super::NodeId,
        #[serde(rename = "pseudoElement")]
        pub pseudo_element: super::Node,
    }
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
    #[serde(rename_all = "camelCase")]
    pub struct TopLayerElementsUpdatedEvent(pub Option<serde_json::Value>);
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
    pub struct ScrollableFlagUpdatedEvent {
        pub params: ScrollableFlagUpdatedEventParams,
    }
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
    pub struct ScrollableFlagUpdatedEventParams {
        #[serde(rename = "nodeId")]
        pub node_id: super::super::dom::NodeId,
        #[serde(default)]
        #[serde(rename = "isScrollable")]
        pub is_scrollable: bool,
    }
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
    pub struct AffectedByStartingStylesFlagUpdatedEvent {
        pub params: AffectedByStartingStylesFlagUpdatedEventParams,
    }
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
    pub struct AffectedByStartingStylesFlagUpdatedEventParams {
        #[serde(rename = "nodeId")]
        pub node_id: super::super::dom::NodeId,
        #[serde(default)]
        #[serde(rename = "affectedByStartingStyles")]
        pub affected_by_starting_styles: bool,
    }
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
    pub struct PseudoElementRemovedEvent {
        pub params: PseudoElementRemovedEventParams,
    }
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
    pub struct PseudoElementRemovedEventParams {
        #[serde(rename = "parentId")]
        pub parent_id: super::NodeId,
        #[serde(rename = "pseudoElementId")]
        pub pseudo_element_id: super::NodeId,
    }
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
    pub struct SetChildNodesEvent {
        pub params: SetChildNodesEventParams,
    }
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
    pub struct SetChildNodesEventParams {
        #[serde(rename = "parentId")]
        pub parent_id: super::NodeId,
        #[serde(rename = "nodes")]
        pub nodes: Vec<super::Node>,
    }
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
    pub struct ShadowRootPoppedEvent {
        pub params: ShadowRootPoppedEventParams,
    }
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
    pub struct ShadowRootPoppedEventParams {
        #[serde(rename = "hostId")]
        pub host_id: super::NodeId,
        #[serde(rename = "rootId")]
        pub root_id: super::NodeId,
    }
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
    pub struct ShadowRootPushedEvent {
        pub params: ShadowRootPushedEventParams,
    }
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
    pub struct ShadowRootPushedEventParams {
        #[serde(rename = "hostId")]
        pub host_id: super::NodeId,
        #[serde(rename = "root")]
        pub root: super::Node,
    }
}
