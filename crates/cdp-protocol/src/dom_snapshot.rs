// Auto-generated from Chrome at version 140.0.7339.186 domain: DOMSnapshot
use super::dom;
use super::dom_debugger;
use super::page;
#[allow(unused_imports)]
use super::types::*;
#[allow(unused_imports)]
use serde::{Deserialize, Serialize};
#[allow(unused_imports)]
use serde_json::Value as Json;
pub type StringIndex = JsUInt;
pub type ArrayOfStrings = Vec<StringIndex>;
pub type Rectangle = Vec<JsFloat>;
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct DomNode {
    #[serde(default)]
    #[serde(rename = "nodeType")]
    pub node_type: JsUInt,
    #[serde(default)]
    #[serde(rename = "nodeName")]
    pub node_name: String,
    #[serde(default)]
    #[serde(rename = "nodeValue")]
    pub node_value: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(rename = "textValue")]
    pub text_value: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(rename = "inputValue")]
    pub input_value: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(rename = "inputChecked")]
    pub input_checked: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(rename = "optionSelected")]
    pub option_selected: Option<bool>,
    #[serde(rename = "backendNodeId")]
    pub backend_node_id: dom::BackendNodeId,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(rename = "childNodeIndexes")]
    pub child_node_indexes: Option<Vec<JsUInt>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "attributes")]
    pub attributes: Option<Vec<NameValue>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(rename = "pseudoElementIndexes")]
    pub pseudo_element_indexes: Option<Vec<JsUInt>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(rename = "layoutNodeIndex")]
    pub layout_node_index: Option<JsUInt>,
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
    #[serde(rename = "contentLanguage")]
    pub content_language: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(rename = "documentEncoding")]
    pub document_encoding: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(rename = "publicId")]
    pub public_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(rename = "systemId")]
    pub system_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "frameId")]
    pub frame_id: Option<page::FrameId>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(rename = "contentDocumentIndex")]
    pub content_document_index: Option<JsUInt>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "pseudoType")]
    pub pseudo_type: Option<dom::PseudoType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "shadowRootType")]
    pub shadow_root_type: Option<dom::ShadowRootType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(rename = "isClickable")]
    pub is_clickable: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "eventListeners")]
    pub event_listeners: Option<Vec<dom_debugger::EventListener>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(rename = "currentSourceURL")]
    pub current_source_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(rename = "originURL")]
    pub origin_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(rename = "scrollOffsetX")]
    pub scroll_offset_x: Option<JsFloat>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(rename = "scrollOffsetY")]
    pub scroll_offset_y: Option<JsFloat>,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct InlineTextBox {
    #[serde(rename = "boundingBox")]
    pub bounding_box: dom::Rect,
    #[serde(default)]
    #[serde(rename = "startCharacterIndex")]
    pub start_character_index: JsUInt,
    #[serde(default)]
    #[serde(rename = "numCharacters")]
    pub num_characters: JsUInt,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct LayoutTreeNode {
    #[serde(default)]
    #[serde(rename = "domNodeIndex")]
    pub dom_node_index: JsUInt,
    #[serde(rename = "boundingBox")]
    pub bounding_box: dom::Rect,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(rename = "layoutText")]
    pub layout_text: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "inlineTextNodes")]
    pub inline_text_nodes: Option<Vec<InlineTextBox>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(rename = "styleIndex")]
    pub style_index: Option<JsUInt>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(rename = "paintOrder")]
    pub paint_order: Option<JsUInt>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(rename = "isStackingContext")]
    pub is_stacking_context: Option<bool>,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct ComputedStyle {
    #[serde(rename = "properties")]
    pub properties: Vec<NameValue>,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct NameValue {
    #[serde(default)]
    #[serde(rename = "name")]
    pub name: String,
    #[serde(default)]
    #[serde(rename = "value")]
    pub value: String,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct RareStringData {
    #[serde(default)]
    #[serde(rename = "index")]
    pub index: Vec<JsUInt>,
    #[serde(rename = "value")]
    pub value: Vec<StringIndex>,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct RareBooleanData {
    #[serde(default)]
    #[serde(rename = "index")]
    pub index: Vec<JsUInt>,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct RareIntegerData {
    #[serde(default)]
    #[serde(rename = "index")]
    pub index: Vec<JsUInt>,
    #[serde(default)]
    #[serde(rename = "value")]
    pub value: Vec<JsUInt>,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct DocumentSnapshot {
    #[serde(rename = "documentURL")]
    pub document_url: StringIndex,
    #[serde(rename = "title")]
    pub title: StringIndex,
    #[serde(rename = "baseURL")]
    pub base_url: StringIndex,
    #[serde(rename = "contentLanguage")]
    pub content_language: StringIndex,
    #[serde(rename = "encodingName")]
    pub encoding_name: StringIndex,
    #[serde(rename = "publicId")]
    pub public_id: StringIndex,
    #[serde(rename = "systemId")]
    pub system_id: StringIndex,
    #[serde(rename = "frameId")]
    pub frame_id: StringIndex,
    #[serde(rename = "nodes")]
    pub nodes: NodeTreeSnapshot,
    #[serde(rename = "layout")]
    pub layout: LayoutTreeSnapshot,
    #[serde(rename = "textBoxes")]
    pub text_boxes: TextBoxSnapshot,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(rename = "scrollOffsetX")]
    pub scroll_offset_x: Option<JsFloat>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(rename = "scrollOffsetY")]
    pub scroll_offset_y: Option<JsFloat>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(rename = "contentWidth")]
    pub content_width: Option<JsFloat>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(rename = "contentHeight")]
    pub content_height: Option<JsFloat>,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct NodeTreeSnapshot {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(rename = "parentIndex")]
    pub parent_index: Option<Vec<JsUInt>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(rename = "nodeType")]
    pub node_type: Option<Vec<JsUInt>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "shadowRootType")]
    pub shadow_root_type: Option<RareStringData>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "nodeName")]
    pub node_name: Option<Vec<StringIndex>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "nodeValue")]
    pub node_value: Option<Vec<StringIndex>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "backendNodeId")]
    pub backend_node_id: Option<Vec<dom::BackendNodeId>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "attributes")]
    pub attributes: Option<Vec<ArrayOfStrings>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "textValue")]
    pub text_value: Option<RareStringData>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "inputValue")]
    pub input_value: Option<RareStringData>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "inputChecked")]
    pub input_checked: Option<RareBooleanData>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "optionSelected")]
    pub option_selected: Option<RareBooleanData>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "contentDocumentIndex")]
    pub content_document_index: Option<RareIntegerData>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "pseudoType")]
    pub pseudo_type: Option<RareStringData>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "pseudoIdentifier")]
    pub pseudo_identifier: Option<RareStringData>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "isClickable")]
    pub is_clickable: Option<RareBooleanData>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "currentSourceURL")]
    pub current_source_url: Option<RareStringData>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "originURL")]
    pub origin_url: Option<RareStringData>,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct LayoutTreeSnapshot {
    #[serde(default)]
    #[serde(rename = "nodeIndex")]
    pub node_index: Vec<JsUInt>,
    #[serde(rename = "styles")]
    pub styles: Vec<ArrayOfStrings>,
    #[serde(rename = "bounds")]
    pub bounds: Vec<Rectangle>,
    #[serde(rename = "text")]
    pub text: Vec<StringIndex>,
    #[serde(rename = "stackingContexts")]
    pub stacking_contexts: RareBooleanData,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(rename = "paintOrders")]
    pub paint_orders: Option<Vec<JsUInt>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "offsetRects")]
    pub offset_rects: Option<Vec<Rectangle>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "scrollRects")]
    pub scroll_rects: Option<Vec<Rectangle>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "clientRects")]
    pub client_rects: Option<Vec<Rectangle>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "blendedBackgroundColors")]
    pub blended_background_colors: Option<Vec<StringIndex>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(rename = "textColorOpacities")]
    pub text_color_opacities: Option<Vec<JsFloat>>,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct TextBoxSnapshot {
    #[serde(default)]
    #[serde(rename = "layoutIndex")]
    pub layout_index: Vec<JsUInt>,
    #[serde(rename = "bounds")]
    pub bounds: Vec<Rectangle>,
    #[serde(default)]
    #[serde(rename = "start")]
    pub start: Vec<JsUInt>,
    #[serde(default)]
    #[serde(rename = "length")]
    pub length: Vec<JsUInt>,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Disable(pub Option<serde_json::Value>);
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Enable(pub Option<serde_json::Value>);
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct GetSnapshot {
    #[serde(default)]
    #[serde(rename = "computedStyleWhitelist")]
    pub computed_style_whitelist: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(rename = "includeEventListeners")]
    pub include_event_listeners: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(rename = "includePaintOrder")]
    pub include_paint_order: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(rename = "includeUserAgentShadowTree")]
    pub include_user_agent_shadow_tree: Option<bool>,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct CaptureSnapshot {
    #[serde(default)]
    #[serde(rename = "computedStyles")]
    pub computed_styles: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(rename = "includePaintOrder")]
    pub include_paint_order: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(rename = "includeDOMRects")]
    pub include_dom_rects: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(rename = "includeBlendedBackgroundColors")]
    pub include_blended_background_colors: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(rename = "includeTextColorOpacities")]
    pub include_text_color_opacities: Option<bool>,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct DisableReturnObject {}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct EnableReturnObject {}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct GetSnapshotReturnObject {
    #[serde(rename = "domNodes")]
    pub dom_nodes: Vec<DomNode>,
    #[serde(rename = "layoutTreeNodes")]
    pub layout_tree_nodes: Vec<LayoutTreeNode>,
    #[serde(rename = "computedStyles")]
    pub computed_styles: Vec<ComputedStyle>,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct CaptureSnapshotReturnObject {
    #[serde(rename = "documents")]
    pub documents: Vec<DocumentSnapshot>,
    #[serde(rename = "strings")]
    pub strings: Vec<String>,
}
impl Method for Disable {
    const NAME: &'static str = "DOMSnapshot.disable";
    type ReturnObject = DisableReturnObject;
}
impl Method for Enable {
    const NAME: &'static str = "DOMSnapshot.enable";
    type ReturnObject = EnableReturnObject;
}
impl Method for GetSnapshot {
    const NAME: &'static str = "DOMSnapshot.getSnapshot";
    type ReturnObject = GetSnapshotReturnObject;
}
impl Method for CaptureSnapshot {
    const NAME: &'static str = "DOMSnapshot.captureSnapshot";
    type ReturnObject = CaptureSnapshotReturnObject;
}
pub mod events {
    #[allow(unused_imports)]
    use super::super::types::*;
    #[allow(unused_imports)]
    use serde::{Deserialize, Serialize};
}
