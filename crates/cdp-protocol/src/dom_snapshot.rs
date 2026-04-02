// Auto-generated from Chrome at version 146.0.7680.165 domain: DOMSnapshot
#![allow(dead_code)]
use super::dom;
use super::dom_debugger;
use super::page;
#[allow(unused_imports)]
use super::types::*;
#[allow(unused_imports)]
use derive_builder::Builder;
#[allow(unused_imports)]
use serde::{Deserialize, Serialize};
#[allow(unused_imports)]
use serde_json::Value as Json;
pub type StringIndex = JsUInt;
pub type ArrayOfStrings = Vec<StringIndex>;
pub type Rectangle = Vec<JsFloat>;
#[allow(deprecated)]
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "camelCase")]
#[doc = "A Node in the DOM tree."]
pub struct DomNode {
    #[serde(default)]
    #[doc = "`Node`'s nodeType."]
    pub node_type: JsUInt,
    #[serde(default)]
    #[doc = "`Node`'s nodeName."]
    pub node_name: String,
    #[serde(default)]
    #[doc = "`Node`'s nodeValue."]
    pub node_value: String,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[doc = "Only set for textarea elements, contains the text value."]
    pub text_value: Option<String>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[doc = "Only set for input elements, contains the input's associated text value."]
    pub input_value: Option<String>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[doc = "Only set for radio and checkbox input elements, indicates if the element has been checked"]
    pub input_checked: Option<bool>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[doc = "Only set for option elements, indicates if the element has been selected"]
    pub option_selected: Option<bool>,
    #[doc = "`Node`'s id, corresponds to DOM.Node.backendNodeId."]
    pub backend_node_id: dom::BackendNodeId,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[doc = "The indexes of the node's child nodes in the `domNodes` array returned by `getSnapshot`, if\n any."]
    pub child_node_indexes: Option<Vec<JsUInt>>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[doc = "Attributes of an `Element` node."]
    pub attributes: Option<Vec<NameValue>>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[doc = "Indexes of pseudo elements associated with this node in the `domNodes` array returned by\n `getSnapshot`, if any."]
    pub pseudo_element_indexes: Option<Vec<JsUInt>>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[doc = "The index of the node's related layout tree node in the `layoutTreeNodes` array returned by\n `getSnapshot`, if any."]
    pub layout_node_index: Option<JsUInt>,
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
    #[doc = "Only set for documents, contains the document's content language."]
    pub content_language: Option<String>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[doc = "Only set for documents, contains the document's character set encoding."]
    pub document_encoding: Option<String>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[doc = "`DocumentType` node's publicId."]
    pub public_id: Option<String>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[doc = "`DocumentType` node's systemId."]
    pub system_id: Option<String>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[doc = "Frame ID for frame owner elements and also for the document node."]
    pub frame_id: Option<page::FrameId>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[doc = "The index of a frame owner element's content document in the `domNodes` array returned by\n `getSnapshot`, if any."]
    pub content_document_index: Option<JsUInt>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[doc = "Type of a pseudo element node."]
    pub pseudo_type: Option<dom::PseudoType>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[doc = "Shadow root type."]
    pub shadow_root_type: Option<dom::ShadowRootType>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[doc = "Whether this DOM node responds to mouse clicks. This includes nodes that have had click\n event listeners attached via JavaScript as well as anchor tags that naturally navigate when\n clicked."]
    pub is_clickable: Option<bool>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[doc = "Details of the node's event listeners, if any."]
    pub event_listeners: Option<Vec<dom_debugger::EventListener>>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[doc = "The selected url for nodes with a srcset attribute."]
    #[serde(rename = "currentSourceURL")]
    pub current_source_url: Option<String>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[doc = "The url of the script (if any) that generates this node."]
    #[serde(rename = "originURL")]
    pub origin_url: Option<String>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[doc = "Scroll offsets, set when this node is a Document."]
    pub scroll_offset_x: Option<JsFloat>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub scroll_offset_y: Option<JsFloat>,
}
#[allow(deprecated)]
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "camelCase")]
#[doc = "Details of post layout rendered text positions. The exact layout should not be regarded as\n stable and may change between versions."]
pub struct InlineTextBox {
    #[doc = "The bounding box in document coordinates. Note that scroll offset of the document is ignored."]
    pub bounding_box: dom::Rect,
    #[serde(default)]
    #[doc = "The starting index in characters, for this post layout textbox substring. Characters that\n would be represented as a surrogate pair in UTF-16 have length 2."]
    pub start_character_index: JsUInt,
    #[serde(default)]
    #[doc = "The number of characters in this post layout textbox substring. Characters that would be\n represented as a surrogate pair in UTF-16 have length 2."]
    pub num_characters: JsUInt,
}
#[allow(deprecated)]
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "camelCase")]
#[doc = "Details of an element in the DOM tree with a LayoutObject."]
pub struct LayoutTreeNode {
    #[serde(default)]
    #[doc = "The index of the related DOM node in the `domNodes` array returned by `getSnapshot`."]
    pub dom_node_index: JsUInt,
    #[doc = "The bounding box in document coordinates. Note that scroll offset of the document is ignored."]
    pub bounding_box: dom::Rect,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[doc = "Contents of the LayoutText, if any."]
    pub layout_text: Option<String>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[doc = "The post-layout inline text nodes, if any."]
    pub inline_text_nodes: Option<Vec<InlineTextBox>>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[doc = "Index into the `computedStyles` array returned by `getSnapshot`."]
    pub style_index: Option<JsUInt>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[doc = "Global paint order index, which is determined by the stacking order of the nodes. Nodes\n that are painted together will have the same index. Only provided if includePaintOrder in\n getSnapshot was true."]
    pub paint_order: Option<JsUInt>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[doc = "Set to true to indicate the element begins a new stacking context."]
    pub is_stacking_context: Option<bool>,
}
#[allow(deprecated)]
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "camelCase")]
#[doc = "A subset of the full ComputedStyle as defined by the request whitelist."]
pub struct ComputedStyle {
    #[doc = "Name/value pairs of computed style properties."]
    pub properties: Vec<NameValue>,
}
#[allow(deprecated)]
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "camelCase")]
#[doc = "A name/value pair."]
pub struct NameValue {
    #[serde(default)]
    #[doc = "Attribute/property name."]
    pub name: String,
    #[serde(default)]
    #[doc = "Attribute/property value."]
    pub value: String,
}
#[allow(deprecated)]
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "camelCase")]
#[doc = "Data that is only present on rare nodes."]
pub struct RareStringData {
    #[serde(default)]
    pub index: Vec<JsUInt>,
    pub value: Vec<StringIndex>,
}
#[allow(deprecated)]
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "camelCase")]
pub struct RareBooleanData {
    #[serde(default)]
    pub index: Vec<JsUInt>,
}
#[allow(deprecated)]
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "camelCase")]
pub struct RareIntegerData {
    #[serde(default)]
    pub index: Vec<JsUInt>,
    #[serde(default)]
    pub value: Vec<JsUInt>,
}
#[allow(deprecated)]
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "camelCase")]
#[doc = "Document snapshot."]
pub struct DocumentSnapshot {
    #[doc = "Document URL that `Document` or `FrameOwner` node points to."]
    #[serde(rename = "documentURL")]
    pub document_url: StringIndex,
    #[doc = "Document title."]
    pub title: StringIndex,
    #[doc = "Base URL that `Document` or `FrameOwner` node uses for URL completion."]
    #[serde(rename = "baseURL")]
    pub base_url: StringIndex,
    #[doc = "Contains the document's content language."]
    pub content_language: StringIndex,
    #[doc = "Contains the document's character set encoding."]
    pub encoding_name: StringIndex,
    #[doc = "`DocumentType` node's publicId."]
    pub public_id: StringIndex,
    #[doc = "`DocumentType` node's systemId."]
    pub system_id: StringIndex,
    #[doc = "Frame ID for frame owner elements and also for the document node."]
    pub frame_id: StringIndex,
    #[doc = "A table with dom nodes."]
    pub nodes: NodeTreeSnapshot,
    #[doc = "The nodes in the layout tree."]
    pub layout: LayoutTreeSnapshot,
    #[doc = "The post-layout inline text nodes."]
    pub text_boxes: TextBoxSnapshot,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[doc = "Horizontal scroll offset."]
    pub scroll_offset_x: Option<JsFloat>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[doc = "Vertical scroll offset."]
    pub scroll_offset_y: Option<JsFloat>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[doc = "Document content width."]
    pub content_width: Option<JsFloat>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[doc = "Document content height."]
    pub content_height: Option<JsFloat>,
}
#[allow(deprecated)]
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "camelCase")]
#[doc = "Table containing nodes."]
pub struct NodeTreeSnapshot {
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[doc = "Parent node index."]
    pub parent_index: Option<Vec<JsUInt>>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[doc = "`Node`'s nodeType."]
    pub node_type: Option<Vec<JsUInt>>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[doc = "Type of the shadow root the `Node` is in. String values are equal to the `ShadowRootType` enum."]
    pub shadow_root_type: Option<RareStringData>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[doc = "`Node`'s nodeName."]
    pub node_name: Option<Vec<StringIndex>>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[doc = "`Node`'s nodeValue."]
    pub node_value: Option<Vec<StringIndex>>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[doc = "`Node`'s id, corresponds to DOM.Node.backendNodeId."]
    pub backend_node_id: Option<Vec<dom::BackendNodeId>>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[doc = "Attributes of an `Element` node. Flatten name, value pairs."]
    pub attributes: Option<Vec<ArrayOfStrings>>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[doc = "Only set for textarea elements, contains the text value."]
    pub text_value: Option<RareStringData>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[doc = "Only set for input elements, contains the input's associated text value."]
    pub input_value: Option<RareStringData>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[doc = "Only set for radio and checkbox input elements, indicates if the element has been checked"]
    pub input_checked: Option<RareBooleanData>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[doc = "Only set for option elements, indicates if the element has been selected"]
    pub option_selected: Option<RareBooleanData>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[doc = "The index of the document in the list of the snapshot documents."]
    pub content_document_index: Option<RareIntegerData>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[doc = "Type of a pseudo element node."]
    pub pseudo_type: Option<RareStringData>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[doc = "Pseudo element identifier for this node. Only present if there is a\n valid pseudoType."]
    pub pseudo_identifier: Option<RareStringData>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[doc = "Whether this DOM node responds to mouse clicks. This includes nodes that have had click\n event listeners attached via JavaScript as well as anchor tags that naturally navigate when\n clicked."]
    pub is_clickable: Option<RareBooleanData>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[doc = "The selected url for nodes with a srcset attribute."]
    #[serde(rename = "currentSourceURL")]
    pub current_source_url: Option<RareStringData>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[doc = "The url of the script (if any) that generates this node."]
    #[serde(rename = "originURL")]
    pub origin_url: Option<RareStringData>,
}
#[allow(deprecated)]
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "camelCase")]
#[doc = "Table of details of an element in the DOM tree with a LayoutObject."]
pub struct LayoutTreeSnapshot {
    #[serde(default)]
    #[doc = "Index of the corresponding node in the `NodeTreeSnapshot` array returned by `captureSnapshot`."]
    pub node_index: Vec<JsUInt>,
    #[doc = "Array of indexes specifying computed style strings, filtered according to the `computedStyles` parameter passed to `captureSnapshot`."]
    pub styles: Vec<ArrayOfStrings>,
    #[doc = "The absolute position bounding box."]
    pub bounds: Vec<Rectangle>,
    #[doc = "Contents of the LayoutText, if any."]
    pub text: Vec<StringIndex>,
    #[doc = "Stacking context information."]
    pub stacking_contexts: RareBooleanData,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[doc = "Global paint order index, which is determined by the stacking order of the nodes. Nodes\n that are painted together will have the same index. Only provided if includePaintOrder in\n captureSnapshot was true."]
    pub paint_orders: Option<Vec<JsUInt>>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[doc = "The offset rect of nodes. Only available when includeDOMRects is set to true"]
    pub offset_rects: Option<Vec<Rectangle>>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[doc = "The scroll rect of nodes. Only available when includeDOMRects is set to true"]
    pub scroll_rects: Option<Vec<Rectangle>>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[doc = "The client rect of nodes. Only available when includeDOMRects is set to true"]
    pub client_rects: Option<Vec<Rectangle>>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[doc = "The list of background colors that are blended with colors of overlapping elements."]
    pub blended_background_colors: Option<Vec<StringIndex>>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[doc = "The list of computed text opacities."]
    pub text_color_opacities: Option<Vec<JsFloat>>,
}
#[allow(deprecated)]
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "camelCase")]
#[doc = "Table of details of the post layout rendered text positions. The exact layout should not be regarded as\n stable and may change between versions."]
pub struct TextBoxSnapshot {
    #[serde(default)]
    #[doc = "Index of the layout tree node that owns this box collection."]
    pub layout_index: Vec<JsUInt>,
    #[doc = "The absolute position bounding box."]
    pub bounds: Vec<Rectangle>,
    #[serde(default)]
    #[doc = "The starting index in characters, for this post layout textbox substring. Characters that\n would be represented as a surrogate pair in UTF-16 have length 2."]
    pub start: Vec<JsUInt>,
    #[serde(default)]
    #[doc = "The number of characters in this post layout textbox substring. Characters that would be\n represented as a surrogate pair in UTF-16 have length 2."]
    pub length: Vec<JsUInt>,
}
#[allow(deprecated)]
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct Disable(pub Option<Json>);
#[allow(deprecated)]
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct Enable(pub Option<Json>);
#[allow(deprecated)]
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "camelCase")]
#[doc = "Returns a document snapshot, including the full DOM tree of the root node (including iframes,\n template contents, and imported documents) in a flattened array, as well as layout and\n white-listed computed style information for the nodes. Shadow DOM in the returned DOM tree is\n flattened."]
#[deprecated]
pub struct GetSnapshot {
    #[serde(default)]
    #[doc = "Whitelist of computed styles to return."]
    pub computed_style_whitelist: Vec<String>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[doc = "Whether or not to retrieve details of DOM listeners (default false)."]
    pub include_event_listeners: Option<bool>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[doc = "Whether to determine and include the paint order index of LayoutTreeNodes (default false)."]
    pub include_paint_order: Option<bool>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[doc = "Whether to include UA shadow tree in the snapshot (default false)."]
    pub include_user_agent_shadow_tree: Option<bool>,
}
#[allow(deprecated)]
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "camelCase")]
#[doc = "Returns a document snapshot, including the full DOM tree of the root node (including iframes,\n template contents, and imported documents) in a flattened array, as well as layout and\n white-listed computed style information for the nodes. Shadow DOM in the returned DOM tree is\n flattened."]
pub struct CaptureSnapshot {
    #[serde(default)]
    #[doc = "Whitelist of computed styles to return."]
    pub computed_styles: Vec<String>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[doc = "Whether to include layout object paint orders into the snapshot."]
    pub include_paint_order: Option<bool>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[doc = "Whether to include DOM rectangles (offsetRects, clientRects, scrollRects) into the snapshot"]
    #[serde(rename = "includeDOMRects")]
    pub include_dom_rects: Option<bool>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[doc = "Whether to include blended background colors in the snapshot (default: false).\n Blended background color is achieved by blending background colors of all elements\n that overlap with the current element."]
    pub include_blended_background_colors: Option<bool>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[doc = "Whether to include text color opacity in the snapshot (default: false).\n An element might have the opacity property set that affects the text color of the element.\n The final text color opacity is computed based on the opacity of all overlapping elements."]
    pub include_text_color_opacities: Option<bool>,
}
#[allow(deprecated)]
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[doc = "Disables DOM snapshot agent for the given page."]
pub struct DisableReturnObject(pub Option<Json>);
#[allow(deprecated)]
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[doc = "Enables DOM snapshot agent for the given page."]
pub struct EnableReturnObject(pub Option<Json>);
#[allow(deprecated)]
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[serde(rename_all = "camelCase")]
#[doc = "Returns a document snapshot, including the full DOM tree of the root node (including iframes,\n template contents, and imported documents) in a flattened array, as well as layout and\n white-listed computed style information for the nodes. Shadow DOM in the returned DOM tree is\n flattened."]
#[deprecated]
pub struct GetSnapshotReturnObject {
    #[doc = "The nodes in the DOM tree. The DOMNode at index 0 corresponds to the root document."]
    pub dom_nodes: Vec<DomNode>,
    #[doc = "The nodes in the layout tree."]
    pub layout_tree_nodes: Vec<LayoutTreeNode>,
    #[doc = "Whitelisted ComputedStyle properties for each node in the layout tree."]
    pub computed_styles: Vec<ComputedStyle>,
}
#[allow(deprecated)]
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[serde(rename_all = "camelCase")]
#[doc = "Returns a document snapshot, including the full DOM tree of the root node (including iframes,\n template contents, and imported documents) in a flattened array, as well as layout and\n white-listed computed style information for the nodes. Shadow DOM in the returned DOM tree is\n flattened."]
pub struct CaptureSnapshotReturnObject {
    #[doc = "The nodes in the DOM tree. The DOMNode at index 0 corresponds to the root document."]
    pub documents: Vec<DocumentSnapshot>,
    #[doc = "Shared string table that all string properties refer to with indexes."]
    pub strings: Vec<String>,
}
#[allow(deprecated)]
impl Method for Disable {
    const NAME: &'static str = "DOMSnapshot.disable";
    type ReturnObject = DisableReturnObject;
}
#[allow(deprecated)]
impl Method for Enable {
    const NAME: &'static str = "DOMSnapshot.enable";
    type ReturnObject = EnableReturnObject;
}
#[allow(deprecated)]
impl Method for GetSnapshot {
    const NAME: &'static str = "DOMSnapshot.getSnapshot";
    type ReturnObject = GetSnapshotReturnObject;
}
#[allow(deprecated)]
impl Method for CaptureSnapshot {
    const NAME: &'static str = "DOMSnapshot.captureSnapshot";
    type ReturnObject = CaptureSnapshotReturnObject;
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
