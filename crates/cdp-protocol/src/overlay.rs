// Auto-generated from Chrome at version 146.0.7680.165 domain: Overlay
#![allow(dead_code)]
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
#[allow(deprecated)]
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub enum LineStylePattern {
    #[serde(rename = "dashed")]
    Dashed,
    #[serde(rename = "dotted")]
    Dotted,
}
#[allow(deprecated)]
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub enum ContrastAlgorithm {
    #[serde(rename = "aa")]
    Aa,
    #[serde(rename = "aaa")]
    Aaa,
    #[serde(rename = "apca")]
    Apca,
}
#[allow(deprecated)]
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub enum ColorFormat {
    #[serde(rename = "rgb")]
    Rgb,
    #[serde(rename = "hsl")]
    Hsl,
    #[serde(rename = "hwb")]
    Hwb,
    #[serde(rename = "hex")]
    Hex,
}
#[allow(deprecated)]
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub enum InspectMode {
    #[serde(rename = "searchForNode")]
    SearchForNode,
    #[serde(rename = "searchForUAShadowDOM")]
    SearchForUaShadowDom,
    #[serde(rename = "captureAreaScreenshot")]
    CaptureAreaScreenshot,
    #[serde(rename = "none")]
    None,
}
#[allow(deprecated)]
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "camelCase")]
#[doc = "Configuration data for drawing the source order of an elements children."]
pub struct SourceOrderConfig {
    #[doc = "the color to outline the given element in."]
    pub parent_outline_color: dom::Rgba,
    #[doc = "the color to outline the child elements in."]
    pub child_outline_color: dom::Rgba,
}
#[allow(deprecated)]
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "camelCase")]
#[doc = "Configuration data for the highlighting of Grid elements."]
pub struct GridHighlightConfig {
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[doc = "Whether the extension lines from grid cells to the rulers should be shown (default: false)."]
    pub show_grid_extension_lines: Option<bool>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[doc = "Show Positive line number labels (default: false)."]
    pub show_positive_line_numbers: Option<bool>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[doc = "Show Negative line number labels (default: false)."]
    pub show_negative_line_numbers: Option<bool>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[doc = "Show area name labels (default: false)."]
    pub show_area_names: Option<bool>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[doc = "Show line name labels (default: false)."]
    pub show_line_names: Option<bool>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[doc = "Show track size labels (default: false)."]
    pub show_track_sizes: Option<bool>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[doc = "The grid container border highlight color (default: transparent)."]
    pub grid_border_color: Option<dom::Rgba>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[doc = "The cell border color (default: transparent). Deprecated, please use rowLineColor and columnLineColor instead."]
    #[deprecated]
    pub cell_border_color: Option<dom::Rgba>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[doc = "The row line color (default: transparent)."]
    pub row_line_color: Option<dom::Rgba>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[doc = "The column line color (default: transparent)."]
    pub column_line_color: Option<dom::Rgba>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[doc = "Whether the grid border is dashed (default: false)."]
    pub grid_border_dash: Option<bool>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[doc = "Whether the cell border is dashed (default: false). Deprecated, please us rowLineDash and columnLineDash instead."]
    #[deprecated]
    pub cell_border_dash: Option<bool>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[doc = "Whether row lines are dashed (default: false)."]
    pub row_line_dash: Option<bool>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[doc = "Whether column lines are dashed (default: false)."]
    pub column_line_dash: Option<bool>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[doc = "The row gap highlight fill color (default: transparent)."]
    pub row_gap_color: Option<dom::Rgba>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[doc = "The row gap hatching fill color (default: transparent)."]
    pub row_hatch_color: Option<dom::Rgba>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[doc = "The column gap highlight fill color (default: transparent)."]
    pub column_gap_color: Option<dom::Rgba>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[doc = "The column gap hatching fill color (default: transparent)."]
    pub column_hatch_color: Option<dom::Rgba>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[doc = "The named grid areas border color (Default: transparent)."]
    pub area_border_color: Option<dom::Rgba>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[doc = "The grid container background color (Default: transparent)."]
    pub grid_background_color: Option<dom::Rgba>,
}
#[allow(deprecated)]
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "camelCase")]
#[doc = "Configuration data for the highlighting of Flex container elements."]
pub struct FlexContainerHighlightConfig {
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[doc = "The style of the container border"]
    pub container_border: Option<LineStyle>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[doc = "The style of the separator between lines"]
    pub line_separator: Option<LineStyle>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[doc = "The style of the separator between items"]
    pub item_separator: Option<LineStyle>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[doc = "Style of content-distribution space on the main axis (justify-content)."]
    pub main_distributed_space: Option<BoxStyle>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[doc = "Style of content-distribution space on the cross axis (align-content)."]
    pub cross_distributed_space: Option<BoxStyle>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[doc = "Style of empty space caused by row gaps (gap/row-gap)."]
    pub row_gap_space: Option<BoxStyle>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[doc = "Style of empty space caused by columns gaps (gap/column-gap)."]
    pub column_gap_space: Option<BoxStyle>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[doc = "Style of the self-alignment line (align-items)."]
    pub cross_alignment: Option<LineStyle>,
}
#[allow(deprecated)]
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "camelCase")]
#[doc = "Configuration data for the highlighting of Flex item elements."]
pub struct FlexItemHighlightConfig {
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[doc = "Style of the box representing the item's base size"]
    pub base_size_box: Option<BoxStyle>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[doc = "Style of the border around the box representing the item's base size"]
    pub base_size_border: Option<LineStyle>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[doc = "Style of the arrow representing if the item grew or shrank"]
    pub flexibility_arrow: Option<LineStyle>,
}
#[allow(deprecated)]
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "camelCase")]
#[doc = "Style information for drawing a line."]
pub struct LineStyle {
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[doc = "The color of the line (default: transparent)"]
    pub color: Option<dom::Rgba>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[doc = "The line pattern (default: solid)"]
    pub pattern: Option<LineStylePattern>,
}
#[allow(deprecated)]
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "camelCase")]
#[doc = "Style information for drawing a box."]
pub struct BoxStyle {
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[doc = "The background color for the box (default: transparent)"]
    pub fill_color: Option<dom::Rgba>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[doc = "The hatching color for the box (default: transparent)"]
    pub hatch_color: Option<dom::Rgba>,
}
#[allow(deprecated)]
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "camelCase")]
#[doc = "Configuration data for the highlighting of page elements."]
pub struct HighlightConfig {
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[doc = "Whether the node info tooltip should be shown (default: false)."]
    pub show_info: Option<bool>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[doc = "Whether the node styles in the tooltip (default: false)."]
    pub show_styles: Option<bool>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[doc = "Whether the rulers should be shown (default: false)."]
    pub show_rulers: Option<bool>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[doc = "Whether the a11y info should be shown (default: true)."]
    pub show_accessibility_info: Option<bool>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[doc = "Whether the extension lines from node to the rulers should be shown (default: false)."]
    pub show_extension_lines: Option<bool>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[doc = "The content box highlight fill color (default: transparent)."]
    pub content_color: Option<dom::Rgba>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[doc = "The padding highlight fill color (default: transparent)."]
    pub padding_color: Option<dom::Rgba>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[doc = "The border highlight fill color (default: transparent)."]
    pub border_color: Option<dom::Rgba>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[doc = "The margin highlight fill color (default: transparent)."]
    pub margin_color: Option<dom::Rgba>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[doc = "The event target element highlight fill color (default: transparent)."]
    pub event_target_color: Option<dom::Rgba>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[doc = "The shape outside fill color (default: transparent)."]
    pub shape_color: Option<dom::Rgba>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[doc = "The shape margin fill color (default: transparent)."]
    pub shape_margin_color: Option<dom::Rgba>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[doc = "The grid layout color (default: transparent)."]
    pub css_grid_color: Option<dom::Rgba>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[doc = "The color format used to format color styles (default: hex)."]
    pub color_format: Option<ColorFormat>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[doc = "The grid layout highlight configuration (default: all transparent)."]
    pub grid_highlight_config: Option<GridHighlightConfig>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[doc = "The flex container highlight configuration (default: all transparent)."]
    pub flex_container_highlight_config: Option<FlexContainerHighlightConfig>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[doc = "The flex item highlight configuration (default: all transparent)."]
    pub flex_item_highlight_config: Option<FlexItemHighlightConfig>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[doc = "The contrast algorithm to use for the contrast ratio (default: aa)."]
    pub contrast_algorithm: Option<ContrastAlgorithm>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[doc = "The container query container highlight configuration (default: all transparent)."]
    pub container_query_container_highlight_config: Option<ContainerQueryContainerHighlightConfig>,
}
#[allow(deprecated)]
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "camelCase")]
#[doc = "Configurations for Persistent Grid Highlight"]
pub struct GridNodeHighlightConfig {
    #[doc = "A descriptor for the highlight appearance."]
    pub grid_highlight_config: GridHighlightConfig,
    #[doc = "Identifier of the node to highlight."]
    pub node_id: dom::NodeId,
}
#[allow(deprecated)]
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "camelCase")]
pub struct FlexNodeHighlightConfig {
    #[doc = "A descriptor for the highlight appearance of flex containers."]
    pub flex_container_highlight_config: FlexContainerHighlightConfig,
    #[doc = "Identifier of the node to highlight."]
    pub node_id: dom::NodeId,
}
#[allow(deprecated)]
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "camelCase")]
pub struct ScrollSnapContainerHighlightConfig {
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[doc = "The style of the snapport border (default: transparent)"]
    pub snapport_border: Option<LineStyle>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[doc = "The style of the snap area border (default: transparent)"]
    pub snap_area_border: Option<LineStyle>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[doc = "The margin highlight fill color (default: transparent)."]
    pub scroll_margin_color: Option<dom::Rgba>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[doc = "The padding highlight fill color (default: transparent)."]
    pub scroll_padding_color: Option<dom::Rgba>,
}
#[allow(deprecated)]
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "camelCase")]
pub struct ScrollSnapHighlightConfig {
    #[doc = "A descriptor for the highlight appearance of scroll snap containers."]
    pub scroll_snap_container_highlight_config: ScrollSnapContainerHighlightConfig,
    #[doc = "Identifier of the node to highlight."]
    pub node_id: dom::NodeId,
}
#[allow(deprecated)]
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "camelCase")]
#[doc = "Configuration for dual screen hinge"]
pub struct HingeConfig {
    #[doc = "A rectangle represent hinge"]
    pub rect: dom::Rect,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[doc = "The content box highlight fill color (default: a dark color)."]
    pub content_color: Option<dom::Rgba>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[doc = "The content box highlight outline color (default: transparent)."]
    pub outline_color: Option<dom::Rgba>,
}
#[allow(deprecated)]
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "camelCase")]
#[doc = "Configuration for Window Controls Overlay"]
pub struct WindowControlsOverlayConfig {
    #[serde(default)]
    #[doc = "Whether the title bar CSS should be shown when emulating the Window Controls Overlay."]
    #[serde(rename = "showCSS")]
    pub show_css: bool,
    #[serde(default)]
    #[doc = "Selected platforms to show the overlay."]
    pub selected_platform: String,
    #[serde(default)]
    #[doc = "The theme color defined in app manifest."]
    pub theme_color: String,
}
#[allow(deprecated)]
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "camelCase")]
pub struct ContainerQueryHighlightConfig {
    #[doc = "A descriptor for the highlight appearance of container query containers."]
    pub container_query_container_highlight_config: ContainerQueryContainerHighlightConfig,
    #[doc = "Identifier of the container node to highlight."]
    pub node_id: dom::NodeId,
}
#[allow(deprecated)]
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "camelCase")]
pub struct ContainerQueryContainerHighlightConfig {
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[doc = "The style of the container border."]
    pub container_border: Option<LineStyle>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[doc = "The style of the descendants' borders."]
    pub descendant_border: Option<LineStyle>,
}
#[allow(deprecated)]
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "camelCase")]
pub struct IsolatedElementHighlightConfig {
    #[doc = "A descriptor for the highlight appearance of an element in isolation mode."]
    pub isolation_mode_highlight_config: IsolationModeHighlightConfig,
    #[doc = "Identifier of the isolated element to highlight."]
    pub node_id: dom::NodeId,
}
#[allow(deprecated)]
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "camelCase")]
pub struct IsolationModeHighlightConfig {
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[doc = "The fill color of the resizers (default: transparent)."]
    pub resizer_color: Option<dom::Rgba>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[doc = "The fill color for resizer handles (default: transparent)."]
    pub resizer_handle_color: Option<dom::Rgba>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[doc = "The fill color for the mask covering non-isolated elements (default: transparent)."]
    pub mask_color: Option<dom::Rgba>,
}
#[allow(deprecated)]
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "camelCase")]
pub struct InspectedElementAnchorConfig {
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[doc = "Identifier of the node to highlight."]
    pub node_id: Option<dom::NodeId>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[doc = "Identifier of the backend node to highlight."]
    pub backend_node_id: Option<dom::BackendNodeId>,
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
#[doc = "For testing."]
pub struct GetHighlightObjectForTest {
    #[doc = "Id of the node to get highlight object for."]
    pub node_id: dom::NodeId,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[doc = "Whether to include distance info."]
    pub include_distance: Option<bool>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[doc = "Whether to include style info."]
    pub include_style: Option<bool>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[doc = "The color format to get config with (default: hex)."]
    pub color_format: Option<ColorFormat>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[doc = "Whether to show accessibility info (default: true)."]
    pub show_accessibility_info: Option<bool>,
}
#[allow(deprecated)]
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "camelCase")]
#[doc = "For Persistent Grid testing."]
pub struct GetGridHighlightObjectsForTest {
    #[doc = "Ids of the node to get highlight object for."]
    pub node_ids: dom::NodeId,
}
#[allow(deprecated)]
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "camelCase")]
#[doc = "For Source Order Viewer testing."]
pub struct GetSourceOrderHighlightObjectForTest {
    #[doc = "Id of the node to highlight."]
    pub node_id: dom::NodeId,
}
#[allow(deprecated)]
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct HideHighlight(pub Option<Json>);
#[allow(deprecated)]
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "camelCase")]
#[doc = "Highlights owner element of the frame with given id.\n Deprecated: Doesn't work reliably and cannot be fixed due to process\n separation (the owner node might be in a different process). Determine\n the owner node in the client and use highlightNode."]
#[deprecated]
pub struct HighlightFrame {
    #[doc = "Identifier of the frame to highlight."]
    pub frame_id: page::FrameId,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[doc = "The content box highlight fill color (default: transparent)."]
    pub content_color: Option<dom::Rgba>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[doc = "The content box highlight outline color (default: transparent)."]
    pub content_outline_color: Option<dom::Rgba>,
}
#[allow(deprecated)]
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "camelCase")]
#[doc = "Highlights DOM node with given id or with the given JavaScript object wrapper. Either nodeId or\n objectId must be specified."]
pub struct HighlightNode {
    #[doc = "A descriptor for the highlight appearance."]
    pub highlight_config: HighlightConfig,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[doc = "Identifier of the node to highlight."]
    pub node_id: Option<dom::NodeId>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[doc = "Identifier of the backend node to highlight."]
    pub backend_node_id: Option<dom::BackendNodeId>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[doc = "JavaScript object id of the node to be highlighted."]
    pub object_id: Option<runtime::RemoteObjectId>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[doc = "Selectors to highlight relevant nodes."]
    pub selector: Option<String>,
}
#[allow(deprecated)]
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "camelCase")]
#[doc = "Highlights given quad. Coordinates are absolute with respect to the main frame viewport."]
pub struct HighlightQuad {
    #[doc = "Quad to highlight"]
    pub quad: dom::Quad,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[doc = "The highlight fill color (default: transparent)."]
    pub color: Option<dom::Rgba>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[doc = "The highlight outline color (default: transparent)."]
    pub outline_color: Option<dom::Rgba>,
}
#[allow(deprecated)]
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "camelCase")]
#[doc = "Highlights given rectangle. Coordinates are absolute with respect to the main frame viewport.\n Issue: the method does not handle device pixel ratio (DPR) correctly.\n The coordinates currently have to be adjusted by the client\n if DPR is not 1 (see crbug.com/437807128)."]
pub struct HighlightRect {
    #[serde(default)]
    #[doc = "X coordinate"]
    pub x: JsUInt,
    #[serde(default)]
    #[doc = "Y coordinate"]
    pub y: JsUInt,
    #[serde(default)]
    #[doc = "Rectangle width"]
    pub width: JsUInt,
    #[serde(default)]
    #[doc = "Rectangle height"]
    pub height: JsUInt,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[doc = "The highlight fill color (default: transparent)."]
    pub color: Option<dom::Rgba>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[doc = "The highlight outline color (default: transparent)."]
    pub outline_color: Option<dom::Rgba>,
}
#[allow(deprecated)]
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "camelCase")]
#[doc = "Highlights the source order of the children of the DOM node with given id or with the given\n JavaScript object wrapper. Either nodeId or objectId must be specified."]
pub struct HighlightSourceOrder {
    #[doc = "A descriptor for the appearance of the overlay drawing."]
    pub source_order_config: SourceOrderConfig,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[doc = "Identifier of the node to highlight."]
    pub node_id: Option<dom::NodeId>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[doc = "Identifier of the backend node to highlight."]
    pub backend_node_id: Option<dom::BackendNodeId>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[doc = "JavaScript object id of the node to be highlighted."]
    pub object_id: Option<runtime::RemoteObjectId>,
}
#[allow(deprecated)]
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "camelCase")]
#[doc = "Enters the 'inspect' mode. In this mode, elements that user is hovering over are highlighted.\n Backend then generates 'inspectNodeRequested' event upon element selection."]
pub struct SetInspectMode {
    #[doc = "Set an inspection mode."]
    pub mode: InspectMode,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[doc = "A descriptor for the highlight appearance of hovered-over nodes. May be omitted if `enabled\n == false`."]
    pub highlight_config: Option<HighlightConfig>,
}
#[allow(deprecated)]
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "camelCase")]
#[doc = "Highlights owner element of all frames detected to be ads."]
pub struct SetShowAdHighlights {
    #[serde(default)]
    #[doc = "True for showing ad highlights"]
    pub show: bool,
}
#[allow(deprecated)]
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "camelCase")]
pub struct SetPausedInDebuggerMessage {
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[doc = "The message to display, also triggers resume and step over controls."]
    pub message: Option<String>,
}
#[allow(deprecated)]
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "camelCase")]
#[doc = "Requests that backend shows debug borders on layers"]
pub struct SetShowDebugBorders {
    #[serde(default)]
    #[doc = "True for showing debug borders"]
    pub show: bool,
}
#[allow(deprecated)]
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "camelCase")]
#[doc = "Requests that backend shows the FPS counter"]
pub struct SetShowFPSCounter {
    #[serde(default)]
    #[doc = "True for showing the FPS counter"]
    pub show: bool,
}
#[allow(deprecated)]
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "camelCase")]
#[doc = "Highlight multiple elements with the CSS Grid overlay."]
pub struct SetShowGridOverlays {
    #[doc = "An array of node identifiers and descriptors for the highlight appearance."]
    pub grid_node_highlight_configs: Vec<GridNodeHighlightConfig>,
}
#[allow(deprecated)]
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "camelCase")]
pub struct SetShowFlexOverlays {
    #[doc = "An array of node identifiers and descriptors for the highlight appearance."]
    pub flex_node_highlight_configs: Vec<FlexNodeHighlightConfig>,
}
#[allow(deprecated)]
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "camelCase")]
pub struct SetShowScrollSnapOverlays {
    #[doc = "An array of node identifiers and descriptors for the highlight appearance."]
    pub scroll_snap_highlight_configs: Vec<ScrollSnapHighlightConfig>,
}
#[allow(deprecated)]
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "camelCase")]
pub struct SetShowContainerQueryOverlays {
    #[doc = "An array of node identifiers and descriptors for the highlight appearance."]
    pub container_query_highlight_configs: Vec<ContainerQueryHighlightConfig>,
}
#[allow(deprecated)]
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "camelCase")]
pub struct SetShowInspectedElementAnchor {
    #[doc = "Node identifier for which to show an anchor for."]
    pub inspected_element_anchor_config: InspectedElementAnchorConfig,
}
#[allow(deprecated)]
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "camelCase")]
#[doc = "Requests that backend shows paint rectangles"]
pub struct SetShowPaintRects {
    #[serde(default)]
    #[doc = "True for showing paint rectangles"]
    pub result: bool,
}
#[allow(deprecated)]
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "camelCase")]
#[doc = "Requests that backend shows layout shift regions"]
pub struct SetShowLayoutShiftRegions {
    #[serde(default)]
    #[doc = "True for showing layout shift regions"]
    pub result: bool,
}
#[allow(deprecated)]
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "camelCase")]
#[doc = "Requests that backend shows scroll bottleneck rects"]
pub struct SetShowScrollBottleneckRects {
    #[serde(default)]
    #[doc = "True for showing scroll bottleneck rects"]
    pub show: bool,
}
#[allow(deprecated)]
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "camelCase")]
#[doc = "Deprecated, no longer has any effect."]
#[deprecated]
pub struct SetShowHitTestBorders {
    #[serde(default)]
    #[doc = "True for showing hit-test borders"]
    pub show: bool,
}
#[allow(deprecated)]
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "camelCase")]
#[doc = "Deprecated, no longer has any effect."]
#[deprecated]
pub struct SetShowWebVitals {
    #[serde(default)]
    pub show: bool,
}
#[allow(deprecated)]
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "camelCase")]
#[doc = "Paints viewport size upon main frame resize."]
pub struct SetShowViewportSizeOnResize {
    #[serde(default)]
    #[doc = "Whether to paint size or not."]
    pub show: bool,
}
#[allow(deprecated)]
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "camelCase")]
#[doc = "Add a dual screen device hinge"]
pub struct SetShowHinge {
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[doc = "hinge data, null means hideHinge"]
    pub hinge_config: Option<HingeConfig>,
}
#[allow(deprecated)]
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "camelCase")]
#[doc = "Show elements in isolation mode with overlays."]
pub struct SetShowIsolatedElements {
    #[doc = "An array of node identifiers and descriptors for the highlight appearance."]
    pub isolated_element_highlight_configs: Vec<IsolatedElementHighlightConfig>,
}
#[allow(deprecated)]
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "camelCase")]
#[doc = "Show Window Controls Overlay for PWA"]
pub struct SetShowWindowControlsOverlay {
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[doc = "Window Controls Overlay data, null means hide Window Controls Overlay"]
    pub window_controls_overlay_config: Option<WindowControlsOverlayConfig>,
}
#[allow(deprecated)]
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[doc = "Disables domain notifications."]
pub struct DisableReturnObject(pub Option<Json>);
#[allow(deprecated)]
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[doc = "Enables domain notifications."]
pub struct EnableReturnObject(pub Option<Json>);
#[allow(deprecated)]
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[serde(rename_all = "camelCase")]
#[doc = "For testing."]
pub struct GetHighlightObjectForTestReturnObject {
    #[serde(default)]
    #[doc = "Highlight data for the node."]
    pub highlight: Json,
}
#[allow(deprecated)]
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[serde(rename_all = "camelCase")]
#[doc = "For Persistent Grid testing."]
pub struct GetGridHighlightObjectsForTestReturnObject {
    #[serde(default)]
    #[doc = "Grid Highlight data for the node ids provided."]
    pub highlights: Json,
}
#[allow(deprecated)]
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[serde(rename_all = "camelCase")]
#[doc = "For Source Order Viewer testing."]
pub struct GetSourceOrderHighlightObjectForTestReturnObject {
    #[serde(default)]
    #[doc = "Source order highlight data for the node id provided."]
    pub highlight: Json,
}
#[allow(deprecated)]
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[doc = "Hides any highlight."]
pub struct HideHighlightReturnObject(pub Option<Json>);
#[allow(deprecated)]
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[doc = "Highlights owner element of the frame with given id.\n Deprecated: Doesn't work reliably and cannot be fixed due to process\n separation (the owner node might be in a different process). Determine\n the owner node in the client and use highlightNode."]
#[deprecated]
pub struct HighlightFrameReturnObject(pub Option<Json>);
#[allow(deprecated)]
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[doc = "Highlights DOM node with given id or with the given JavaScript object wrapper. Either nodeId or\n objectId must be specified."]
pub struct HighlightNodeReturnObject(pub Option<Json>);
#[allow(deprecated)]
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[doc = "Highlights given quad. Coordinates are absolute with respect to the main frame viewport."]
pub struct HighlightQuadReturnObject(pub Option<Json>);
#[allow(deprecated)]
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[doc = "Highlights given rectangle. Coordinates are absolute with respect to the main frame viewport.\n Issue: the method does not handle device pixel ratio (DPR) correctly.\n The coordinates currently have to be adjusted by the client\n if DPR is not 1 (see crbug.com/437807128)."]
pub struct HighlightRectReturnObject(pub Option<Json>);
#[allow(deprecated)]
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[doc = "Highlights the source order of the children of the DOM node with given id or with the given\n JavaScript object wrapper. Either nodeId or objectId must be specified."]
pub struct HighlightSourceOrderReturnObject(pub Option<Json>);
#[allow(deprecated)]
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[doc = "Enters the 'inspect' mode. In this mode, elements that user is hovering over are highlighted.\n Backend then generates 'inspectNodeRequested' event upon element selection."]
pub struct SetInspectModeReturnObject(pub Option<Json>);
#[allow(deprecated)]
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[doc = "Highlights owner element of all frames detected to be ads."]
pub struct SetShowAdHighlightsReturnObject(pub Option<Json>);
#[allow(deprecated)]
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct SetPausedInDebuggerMessageReturnObject(pub Option<Json>);
#[allow(deprecated)]
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[doc = "Requests that backend shows debug borders on layers"]
pub struct SetShowDebugBordersReturnObject(pub Option<Json>);
#[allow(deprecated)]
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[doc = "Requests that backend shows the FPS counter"]
pub struct SetShowFPSCounterReturnObject(pub Option<Json>);
#[allow(deprecated)]
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[doc = "Highlight multiple elements with the CSS Grid overlay."]
pub struct SetShowGridOverlaysReturnObject(pub Option<Json>);
#[allow(deprecated)]
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct SetShowFlexOverlaysReturnObject(pub Option<Json>);
#[allow(deprecated)]
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct SetShowScrollSnapOverlaysReturnObject(pub Option<Json>);
#[allow(deprecated)]
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct SetShowContainerQueryOverlaysReturnObject(pub Option<Json>);
#[allow(deprecated)]
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct SetShowInspectedElementAnchorReturnObject(pub Option<Json>);
#[allow(deprecated)]
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[doc = "Requests that backend shows paint rectangles"]
pub struct SetShowPaintRectsReturnObject(pub Option<Json>);
#[allow(deprecated)]
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[doc = "Requests that backend shows layout shift regions"]
pub struct SetShowLayoutShiftRegionsReturnObject(pub Option<Json>);
#[allow(deprecated)]
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[doc = "Requests that backend shows scroll bottleneck rects"]
pub struct SetShowScrollBottleneckRectsReturnObject(pub Option<Json>);
#[allow(deprecated)]
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[doc = "Deprecated, no longer has any effect."]
#[deprecated]
pub struct SetShowHitTestBordersReturnObject(pub Option<Json>);
#[allow(deprecated)]
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[doc = "Deprecated, no longer has any effect."]
#[deprecated]
pub struct SetShowWebVitalsReturnObject(pub Option<Json>);
#[allow(deprecated)]
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[doc = "Paints viewport size upon main frame resize."]
pub struct SetShowViewportSizeOnResizeReturnObject(pub Option<Json>);
#[allow(deprecated)]
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[doc = "Add a dual screen device hinge"]
pub struct SetShowHingeReturnObject(pub Option<Json>);
#[allow(deprecated)]
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[doc = "Show elements in isolation mode with overlays."]
pub struct SetShowIsolatedElementsReturnObject(pub Option<Json>);
#[allow(deprecated)]
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[doc = "Show Window Controls Overlay for PWA"]
pub struct SetShowWindowControlsOverlayReturnObject(pub Option<Json>);
#[allow(deprecated)]
impl Method for Disable {
    const NAME: &'static str = "Overlay.disable";
    type ReturnObject = DisableReturnObject;
}
#[allow(deprecated)]
impl Method for Enable {
    const NAME: &'static str = "Overlay.enable";
    type ReturnObject = EnableReturnObject;
}
#[allow(deprecated)]
impl Method for GetHighlightObjectForTest {
    const NAME: &'static str = "Overlay.getHighlightObjectForTest";
    type ReturnObject = GetHighlightObjectForTestReturnObject;
}
#[allow(deprecated)]
impl Method for GetGridHighlightObjectsForTest {
    const NAME: &'static str = "Overlay.getGridHighlightObjectsForTest";
    type ReturnObject = GetGridHighlightObjectsForTestReturnObject;
}
#[allow(deprecated)]
impl Method for GetSourceOrderHighlightObjectForTest {
    const NAME: &'static str = "Overlay.getSourceOrderHighlightObjectForTest";
    type ReturnObject = GetSourceOrderHighlightObjectForTestReturnObject;
}
#[allow(deprecated)]
impl Method for HideHighlight {
    const NAME: &'static str = "Overlay.hideHighlight";
    type ReturnObject = HideHighlightReturnObject;
}
#[allow(deprecated)]
impl Method for HighlightFrame {
    const NAME: &'static str = "Overlay.highlightFrame";
    type ReturnObject = HighlightFrameReturnObject;
}
#[allow(deprecated)]
impl Method for HighlightNode {
    const NAME: &'static str = "Overlay.highlightNode";
    type ReturnObject = HighlightNodeReturnObject;
}
#[allow(deprecated)]
impl Method for HighlightQuad {
    const NAME: &'static str = "Overlay.highlightQuad";
    type ReturnObject = HighlightQuadReturnObject;
}
#[allow(deprecated)]
impl Method for HighlightRect {
    const NAME: &'static str = "Overlay.highlightRect";
    type ReturnObject = HighlightRectReturnObject;
}
#[allow(deprecated)]
impl Method for HighlightSourceOrder {
    const NAME: &'static str = "Overlay.highlightSourceOrder";
    type ReturnObject = HighlightSourceOrderReturnObject;
}
#[allow(deprecated)]
impl Method for SetInspectMode {
    const NAME: &'static str = "Overlay.setInspectMode";
    type ReturnObject = SetInspectModeReturnObject;
}
#[allow(deprecated)]
impl Method for SetShowAdHighlights {
    const NAME: &'static str = "Overlay.setShowAdHighlights";
    type ReturnObject = SetShowAdHighlightsReturnObject;
}
#[allow(deprecated)]
impl Method for SetPausedInDebuggerMessage {
    const NAME: &'static str = "Overlay.setPausedInDebuggerMessage";
    type ReturnObject = SetPausedInDebuggerMessageReturnObject;
}
#[allow(deprecated)]
impl Method for SetShowDebugBorders {
    const NAME: &'static str = "Overlay.setShowDebugBorders";
    type ReturnObject = SetShowDebugBordersReturnObject;
}
#[allow(deprecated)]
impl Method for SetShowFPSCounter {
    const NAME: &'static str = "Overlay.setShowFPSCounter";
    type ReturnObject = SetShowFPSCounterReturnObject;
}
#[allow(deprecated)]
impl Method for SetShowGridOverlays {
    const NAME: &'static str = "Overlay.setShowGridOverlays";
    type ReturnObject = SetShowGridOverlaysReturnObject;
}
#[allow(deprecated)]
impl Method for SetShowFlexOverlays {
    const NAME: &'static str = "Overlay.setShowFlexOverlays";
    type ReturnObject = SetShowFlexOverlaysReturnObject;
}
#[allow(deprecated)]
impl Method for SetShowScrollSnapOverlays {
    const NAME: &'static str = "Overlay.setShowScrollSnapOverlays";
    type ReturnObject = SetShowScrollSnapOverlaysReturnObject;
}
#[allow(deprecated)]
impl Method for SetShowContainerQueryOverlays {
    const NAME: &'static str = "Overlay.setShowContainerQueryOverlays";
    type ReturnObject = SetShowContainerQueryOverlaysReturnObject;
}
#[allow(deprecated)]
impl Method for SetShowInspectedElementAnchor {
    const NAME: &'static str = "Overlay.setShowInspectedElementAnchor";
    type ReturnObject = SetShowInspectedElementAnchorReturnObject;
}
#[allow(deprecated)]
impl Method for SetShowPaintRects {
    const NAME: &'static str = "Overlay.setShowPaintRects";
    type ReturnObject = SetShowPaintRectsReturnObject;
}
#[allow(deprecated)]
impl Method for SetShowLayoutShiftRegions {
    const NAME: &'static str = "Overlay.setShowLayoutShiftRegions";
    type ReturnObject = SetShowLayoutShiftRegionsReturnObject;
}
#[allow(deprecated)]
impl Method for SetShowScrollBottleneckRects {
    const NAME: &'static str = "Overlay.setShowScrollBottleneckRects";
    type ReturnObject = SetShowScrollBottleneckRectsReturnObject;
}
#[allow(deprecated)]
impl Method for SetShowHitTestBorders {
    const NAME: &'static str = "Overlay.setShowHitTestBorders";
    type ReturnObject = SetShowHitTestBordersReturnObject;
}
#[allow(deprecated)]
impl Method for SetShowWebVitals {
    const NAME: &'static str = "Overlay.setShowWebVitals";
    type ReturnObject = SetShowWebVitalsReturnObject;
}
#[allow(deprecated)]
impl Method for SetShowViewportSizeOnResize {
    const NAME: &'static str = "Overlay.setShowViewportSizeOnResize";
    type ReturnObject = SetShowViewportSizeOnResizeReturnObject;
}
#[allow(deprecated)]
impl Method for SetShowHinge {
    const NAME: &'static str = "Overlay.setShowHinge";
    type ReturnObject = SetShowHingeReturnObject;
}
#[allow(deprecated)]
impl Method for SetShowIsolatedElements {
    const NAME: &'static str = "Overlay.setShowIsolatedElements";
    type ReturnObject = SetShowIsolatedElementsReturnObject;
}
#[allow(deprecated)]
impl Method for SetShowWindowControlsOverlay {
    const NAME: &'static str = "Overlay.setShowWindowControlsOverlay";
    type ReturnObject = SetShowWindowControlsOverlayReturnObject;
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
    #[allow(deprecated)]
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
    pub struct InspectNodeRequestedEvent {
        pub params: InspectNodeRequestedEventParams,
    }
    #[allow(deprecated)]
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
    #[serde(rename_all = "camelCase")]
    pub struct InspectNodeRequestedEventParams {
        #[doc = "Id of the node to inspect."]
        pub backend_node_id: super::super::dom::BackendNodeId,
    }
    #[allow(deprecated)]
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
    pub struct NodeHighlightRequestedEvent {
        pub params: NodeHighlightRequestedEventParams,
    }
    #[allow(deprecated)]
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
    #[serde(rename_all = "camelCase")]
    pub struct NodeHighlightRequestedEventParams {
        pub node_id: super::super::dom::NodeId,
    }
    #[allow(deprecated)]
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
    pub struct ScreenshotRequestedEvent {
        pub params: ScreenshotRequestedEventParams,
    }
    #[allow(deprecated)]
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
    #[serde(rename_all = "camelCase")]
    pub struct ScreenshotRequestedEventParams {
        #[doc = "Viewport to capture, in device independent pixels (dip)."]
        pub viewport: super::super::page::Viewport,
    }
    #[allow(deprecated)]
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
    pub struct InspectPanelShowRequestedEvent {
        pub params: InspectPanelShowRequestedEventParams,
    }
    #[allow(deprecated)]
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
    #[serde(rename_all = "camelCase")]
    pub struct InspectPanelShowRequestedEventParams {
        #[doc = "Id of the node to show in the panel."]
        pub backend_node_id: super::super::dom::BackendNodeId,
    }
    #[allow(deprecated)]
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
    pub struct InspectedElementWindowRestoredEvent {
        pub params: InspectedElementWindowRestoredEventParams,
    }
    #[allow(deprecated)]
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
    #[serde(rename_all = "camelCase")]
    pub struct InspectedElementWindowRestoredEventParams {
        #[doc = "Id of the node to restore the floating window for."]
        pub backend_node_id: super::super::dom::BackendNodeId,
    }
    #[allow(deprecated)]
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
    pub struct InspectModeCanceledEvent(pub Option<Json>);
}
