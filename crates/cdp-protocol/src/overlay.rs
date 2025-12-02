// Auto-generated from Chrome at version 140.0.7339.186 domain: Overlay
use super::dom;
use super::page;
use super::runtime;
#[allow(unused_imports)]
use super::types::*;
#[allow(unused_imports)]
use serde::{Deserialize, Serialize};
#[allow(unused_imports)]
use serde_json::Value as Json;
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub enum LineStylePattern {
    #[serde(rename = "dashed")]
    Dashed,
    #[serde(rename = "dotted")]
    Dotted,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub enum ContrastAlgorithm {
    #[serde(rename = "aa")]
    Aa,
    #[serde(rename = "aaa")]
    Aaa,
    #[serde(rename = "apca")]
    Apca,
}
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
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct SourceOrderConfig {
    #[serde(rename = "parentOutlineColor")]
    pub parent_outline_color: dom::Rgba,
    #[serde(rename = "childOutlineColor")]
    pub child_outline_color: dom::Rgba,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct GridHighlightConfig {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(rename = "showGridExtensionLines")]
    pub show_grid_extension_lines: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(rename = "showPositiveLineNumbers")]
    pub show_positive_line_numbers: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(rename = "showNegativeLineNumbers")]
    pub show_negative_line_numbers: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(rename = "showAreaNames")]
    pub show_area_names: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(rename = "showLineNames")]
    pub show_line_names: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(rename = "showTrackSizes")]
    pub show_track_sizes: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "gridBorderColor")]
    pub grid_border_color: Option<dom::Rgba>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "cellBorderColor")]
    pub cell_border_color: Option<dom::Rgba>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "rowLineColor")]
    pub row_line_color: Option<dom::Rgba>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "columnLineColor")]
    pub column_line_color: Option<dom::Rgba>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(rename = "gridBorderDash")]
    pub grid_border_dash: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(rename = "cellBorderDash")]
    pub cell_border_dash: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(rename = "rowLineDash")]
    pub row_line_dash: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(rename = "columnLineDash")]
    pub column_line_dash: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "rowGapColor")]
    pub row_gap_color: Option<dom::Rgba>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "rowHatchColor")]
    pub row_hatch_color: Option<dom::Rgba>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "columnGapColor")]
    pub column_gap_color: Option<dom::Rgba>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "columnHatchColor")]
    pub column_hatch_color: Option<dom::Rgba>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "areaBorderColor")]
    pub area_border_color: Option<dom::Rgba>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "gridBackgroundColor")]
    pub grid_background_color: Option<dom::Rgba>,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct FlexContainerHighlightConfig {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "containerBorder")]
    pub container_border: Option<LineStyle>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "lineSeparator")]
    pub line_separator: Option<LineStyle>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "itemSeparator")]
    pub item_separator: Option<LineStyle>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "mainDistributedSpace")]
    pub main_distributed_space: Option<BoxStyle>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "crossDistributedSpace")]
    pub cross_distributed_space: Option<BoxStyle>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "rowGapSpace")]
    pub row_gap_space: Option<BoxStyle>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "columnGapSpace")]
    pub column_gap_space: Option<BoxStyle>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "crossAlignment")]
    pub cross_alignment: Option<LineStyle>,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct FlexItemHighlightConfig {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "baseSizeBox")]
    pub base_size_box: Option<BoxStyle>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "baseSizeBorder")]
    pub base_size_border: Option<LineStyle>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "flexibilityArrow")]
    pub flexibility_arrow: Option<LineStyle>,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct LineStyle {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "color")]
    pub color: Option<dom::Rgba>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "pattern")]
    pub pattern: Option<LineStylePattern>,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct BoxStyle {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "fillColor")]
    pub fill_color: Option<dom::Rgba>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "hatchColor")]
    pub hatch_color: Option<dom::Rgba>,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct HighlightConfig {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(rename = "showInfo")]
    pub show_info: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(rename = "showStyles")]
    pub show_styles: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(rename = "showRulers")]
    pub show_rulers: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(rename = "showAccessibilityInfo")]
    pub show_accessibility_info: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(rename = "showExtensionLines")]
    pub show_extension_lines: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "contentColor")]
    pub content_color: Option<dom::Rgba>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "paddingColor")]
    pub padding_color: Option<dom::Rgba>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "borderColor")]
    pub border_color: Option<dom::Rgba>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "marginColor")]
    pub margin_color: Option<dom::Rgba>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "eventTargetColor")]
    pub event_target_color: Option<dom::Rgba>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "shapeColor")]
    pub shape_color: Option<dom::Rgba>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "shapeMarginColor")]
    pub shape_margin_color: Option<dom::Rgba>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "cssGridColor")]
    pub css_grid_color: Option<dom::Rgba>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "colorFormat")]
    pub color_format: Option<ColorFormat>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "gridHighlightConfig")]
    pub grid_highlight_config: Option<GridHighlightConfig>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "flexContainerHighlightConfig")]
    pub flex_container_highlight_config: Option<FlexContainerHighlightConfig>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "flexItemHighlightConfig")]
    pub flex_item_highlight_config: Option<FlexItemHighlightConfig>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "contrastAlgorithm")]
    pub contrast_algorithm: Option<ContrastAlgorithm>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "containerQueryContainerHighlightConfig")]
    pub container_query_container_highlight_config: Option<ContainerQueryContainerHighlightConfig>,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct GridNodeHighlightConfig {
    #[serde(rename = "gridHighlightConfig")]
    pub grid_highlight_config: GridHighlightConfig,
    #[serde(rename = "nodeId")]
    pub node_id: dom::NodeId,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct FlexNodeHighlightConfig {
    #[serde(rename = "flexContainerHighlightConfig")]
    pub flex_container_highlight_config: FlexContainerHighlightConfig,
    #[serde(rename = "nodeId")]
    pub node_id: dom::NodeId,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct ScrollSnapContainerHighlightConfig {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "snapportBorder")]
    pub snapport_border: Option<LineStyle>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "snapAreaBorder")]
    pub snap_area_border: Option<LineStyle>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "scrollMarginColor")]
    pub scroll_margin_color: Option<dom::Rgba>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "scrollPaddingColor")]
    pub scroll_padding_color: Option<dom::Rgba>,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct ScrollSnapHighlightConfig {
    #[serde(rename = "scrollSnapContainerHighlightConfig")]
    pub scroll_snap_container_highlight_config: ScrollSnapContainerHighlightConfig,
    #[serde(rename = "nodeId")]
    pub node_id: dom::NodeId,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct HingeConfig {
    #[serde(rename = "rect")]
    pub rect: dom::Rect,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "contentColor")]
    pub content_color: Option<dom::Rgba>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "outlineColor")]
    pub outline_color: Option<dom::Rgba>,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct WindowControlsOverlayConfig {
    #[serde(default)]
    #[serde(rename = "showCSS")]
    pub show_css: bool,
    #[serde(default)]
    #[serde(rename = "selectedPlatform")]
    pub selected_platform: String,
    #[serde(default)]
    #[serde(rename = "themeColor")]
    pub theme_color: String,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct ContainerQueryHighlightConfig {
    #[serde(rename = "containerQueryContainerHighlightConfig")]
    pub container_query_container_highlight_config: ContainerQueryContainerHighlightConfig,
    #[serde(rename = "nodeId")]
    pub node_id: dom::NodeId,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct ContainerQueryContainerHighlightConfig {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "containerBorder")]
    pub container_border: Option<LineStyle>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "descendantBorder")]
    pub descendant_border: Option<LineStyle>,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct IsolatedElementHighlightConfig {
    #[serde(rename = "isolationModeHighlightConfig")]
    pub isolation_mode_highlight_config: IsolationModeHighlightConfig,
    #[serde(rename = "nodeId")]
    pub node_id: dom::NodeId,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct IsolationModeHighlightConfig {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "resizerColor")]
    pub resizer_color: Option<dom::Rgba>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "resizerHandleColor")]
    pub resizer_handle_color: Option<dom::Rgba>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "maskColor")]
    pub mask_color: Option<dom::Rgba>,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Disable(pub Option<serde_json::Value>);
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Enable(pub Option<serde_json::Value>);
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct GetHighlightObjectForTest {
    #[serde(rename = "nodeId")]
    pub node_id: dom::NodeId,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(rename = "includeDistance")]
    pub include_distance: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(rename = "includeStyle")]
    pub include_style: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "colorFormat")]
    pub color_format: Option<ColorFormat>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(rename = "showAccessibilityInfo")]
    pub show_accessibility_info: Option<bool>,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct GetGridHighlightObjectsForTest {
    #[serde(rename = "nodeIds")]
    pub node_ids: dom::NodeId,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct GetSourceOrderHighlightObjectForTest {
    #[serde(rename = "nodeId")]
    pub node_id: dom::NodeId,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct HideHighlight(pub Option<serde_json::Value>);
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct HighlightFrame {
    #[serde(rename = "frameId")]
    pub frame_id: page::FrameId,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "contentColor")]
    pub content_color: Option<dom::Rgba>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "contentOutlineColor")]
    pub content_outline_color: Option<dom::Rgba>,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct HighlightNode {
    #[serde(rename = "highlightConfig")]
    pub highlight_config: HighlightConfig,
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
    #[serde(rename = "selector")]
    pub selector: Option<String>,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct HighlightQuad {
    #[serde(rename = "quad")]
    pub quad: dom::Quad,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "color")]
    pub color: Option<dom::Rgba>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "outlineColor")]
    pub outline_color: Option<dom::Rgba>,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct HighlightRect {
    #[serde(default)]
    #[serde(rename = "x")]
    pub x: JsUInt,
    #[serde(default)]
    #[serde(rename = "y")]
    pub y: JsUInt,
    #[serde(default)]
    #[serde(rename = "width")]
    pub width: JsUInt,
    #[serde(default)]
    #[serde(rename = "height")]
    pub height: JsUInt,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "color")]
    pub color: Option<dom::Rgba>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "outlineColor")]
    pub outline_color: Option<dom::Rgba>,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct HighlightSourceOrder {
    #[serde(rename = "sourceOrderConfig")]
    pub source_order_config: SourceOrderConfig,
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
pub struct SetInspectMode {
    #[serde(rename = "mode")]
    pub mode: InspectMode,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "highlightConfig")]
    pub highlight_config: Option<HighlightConfig>,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct SetShowAdHighlights {
    #[serde(default)]
    #[serde(rename = "show")]
    pub show: bool,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct SetPausedInDebuggerMessage {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(rename = "message")]
    pub message: Option<String>,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct SetShowDebugBorders {
    #[serde(default)]
    #[serde(rename = "show")]
    pub show: bool,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct SetShowFPSCounter {
    #[serde(default)]
    #[serde(rename = "show")]
    pub show: bool,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct SetShowGridOverlays {
    #[serde(rename = "gridNodeHighlightConfigs")]
    pub grid_node_highlight_configs: Vec<GridNodeHighlightConfig>,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct SetShowFlexOverlays {
    #[serde(rename = "flexNodeHighlightConfigs")]
    pub flex_node_highlight_configs: Vec<FlexNodeHighlightConfig>,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct SetShowScrollSnapOverlays {
    #[serde(rename = "scrollSnapHighlightConfigs")]
    pub scroll_snap_highlight_configs: Vec<ScrollSnapHighlightConfig>,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct SetShowContainerQueryOverlays {
    #[serde(rename = "containerQueryHighlightConfigs")]
    pub container_query_highlight_configs: Vec<ContainerQueryHighlightConfig>,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct SetShowPaintRects {
    #[serde(default)]
    #[serde(rename = "result")]
    pub result: bool,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct SetShowLayoutShiftRegions {
    #[serde(default)]
    #[serde(rename = "result")]
    pub result: bool,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct SetShowScrollBottleneckRects {
    #[serde(default)]
    #[serde(rename = "show")]
    pub show: bool,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct SetShowHitTestBorders {
    #[serde(default)]
    #[serde(rename = "show")]
    pub show: bool,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct SetShowWebVitals {
    #[serde(default)]
    #[serde(rename = "show")]
    pub show: bool,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct SetShowViewportSizeOnResize {
    #[serde(default)]
    #[serde(rename = "show")]
    pub show: bool,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct SetShowHinge {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "hingeConfig")]
    pub hinge_config: Option<HingeConfig>,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct SetShowIsolatedElements {
    #[serde(rename = "isolatedElementHighlightConfigs")]
    pub isolated_element_highlight_configs: Vec<IsolatedElementHighlightConfig>,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct SetShowWindowControlsOverlay {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "windowControlsOverlayConfig")]
    pub window_controls_overlay_config: Option<WindowControlsOverlayConfig>,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct DisableReturnObject {}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct EnableReturnObject {}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct GetHighlightObjectForTestReturnObject {}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct GetGridHighlightObjectsForTestReturnObject {}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct GetSourceOrderHighlightObjectForTestReturnObject {}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct HideHighlightReturnObject {}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct HighlightFrameReturnObject {}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct HighlightNodeReturnObject {}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct HighlightQuadReturnObject {}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct HighlightRectReturnObject {}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct HighlightSourceOrderReturnObject {}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct SetInspectModeReturnObject {}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct SetShowAdHighlightsReturnObject {}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct SetPausedInDebuggerMessageReturnObject {}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct SetShowDebugBordersReturnObject {}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct SetShowFPSCounterReturnObject {}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct SetShowGridOverlaysReturnObject {}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct SetShowFlexOverlaysReturnObject {}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct SetShowScrollSnapOverlaysReturnObject {}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct SetShowContainerQueryOverlaysReturnObject {}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct SetShowPaintRectsReturnObject {}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct SetShowLayoutShiftRegionsReturnObject {}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct SetShowScrollBottleneckRectsReturnObject {}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct SetShowHitTestBordersReturnObject {}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct SetShowWebVitalsReturnObject {}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct SetShowViewportSizeOnResizeReturnObject {}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct SetShowHingeReturnObject {}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct SetShowIsolatedElementsReturnObject {}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct SetShowWindowControlsOverlayReturnObject {}
impl Method for Disable {
    const NAME: &'static str = "Overlay.disable";
    type ReturnObject = DisableReturnObject;
}
impl Method for Enable {
    const NAME: &'static str = "Overlay.enable";
    type ReturnObject = EnableReturnObject;
}
impl Method for GetHighlightObjectForTest {
    const NAME: &'static str = "Overlay.getHighlightObjectForTest";
    type ReturnObject = GetHighlightObjectForTestReturnObject;
}
impl Method for GetGridHighlightObjectsForTest {
    const NAME: &'static str = "Overlay.getGridHighlightObjectsForTest";
    type ReturnObject = GetGridHighlightObjectsForTestReturnObject;
}
impl Method for GetSourceOrderHighlightObjectForTest {
    const NAME: &'static str = "Overlay.getSourceOrderHighlightObjectForTest";
    type ReturnObject = GetSourceOrderHighlightObjectForTestReturnObject;
}
impl Method for HideHighlight {
    const NAME: &'static str = "Overlay.hideHighlight";
    type ReturnObject = HideHighlightReturnObject;
}
impl Method for HighlightFrame {
    const NAME: &'static str = "Overlay.highlightFrame";
    type ReturnObject = HighlightFrameReturnObject;
}
impl Method for HighlightNode {
    const NAME: &'static str = "Overlay.highlightNode";
    type ReturnObject = HighlightNodeReturnObject;
}
impl Method for HighlightQuad {
    const NAME: &'static str = "Overlay.highlightQuad";
    type ReturnObject = HighlightQuadReturnObject;
}
impl Method for HighlightRect {
    const NAME: &'static str = "Overlay.highlightRect";
    type ReturnObject = HighlightRectReturnObject;
}
impl Method for HighlightSourceOrder {
    const NAME: &'static str = "Overlay.highlightSourceOrder";
    type ReturnObject = HighlightSourceOrderReturnObject;
}
impl Method for SetInspectMode {
    const NAME: &'static str = "Overlay.setInspectMode";
    type ReturnObject = SetInspectModeReturnObject;
}
impl Method for SetShowAdHighlights {
    const NAME: &'static str = "Overlay.setShowAdHighlights";
    type ReturnObject = SetShowAdHighlightsReturnObject;
}
impl Method for SetPausedInDebuggerMessage {
    const NAME: &'static str = "Overlay.setPausedInDebuggerMessage";
    type ReturnObject = SetPausedInDebuggerMessageReturnObject;
}
impl Method for SetShowDebugBorders {
    const NAME: &'static str = "Overlay.setShowDebugBorders";
    type ReturnObject = SetShowDebugBordersReturnObject;
}
impl Method for SetShowFPSCounter {
    const NAME: &'static str = "Overlay.setShowFPSCounter";
    type ReturnObject = SetShowFPSCounterReturnObject;
}
impl Method for SetShowGridOverlays {
    const NAME: &'static str = "Overlay.setShowGridOverlays";
    type ReturnObject = SetShowGridOverlaysReturnObject;
}
impl Method for SetShowFlexOverlays {
    const NAME: &'static str = "Overlay.setShowFlexOverlays";
    type ReturnObject = SetShowFlexOverlaysReturnObject;
}
impl Method for SetShowScrollSnapOverlays {
    const NAME: &'static str = "Overlay.setShowScrollSnapOverlays";
    type ReturnObject = SetShowScrollSnapOverlaysReturnObject;
}
impl Method for SetShowContainerQueryOverlays {
    const NAME: &'static str = "Overlay.setShowContainerQueryOverlays";
    type ReturnObject = SetShowContainerQueryOverlaysReturnObject;
}
impl Method for SetShowPaintRects {
    const NAME: &'static str = "Overlay.setShowPaintRects";
    type ReturnObject = SetShowPaintRectsReturnObject;
}
impl Method for SetShowLayoutShiftRegions {
    const NAME: &'static str = "Overlay.setShowLayoutShiftRegions";
    type ReturnObject = SetShowLayoutShiftRegionsReturnObject;
}
impl Method for SetShowScrollBottleneckRects {
    const NAME: &'static str = "Overlay.setShowScrollBottleneckRects";
    type ReturnObject = SetShowScrollBottleneckRectsReturnObject;
}
impl Method for SetShowHitTestBorders {
    const NAME: &'static str = "Overlay.setShowHitTestBorders";
    type ReturnObject = SetShowHitTestBordersReturnObject;
}
impl Method for SetShowWebVitals {
    const NAME: &'static str = "Overlay.setShowWebVitals";
    type ReturnObject = SetShowWebVitalsReturnObject;
}
impl Method for SetShowViewportSizeOnResize {
    const NAME: &'static str = "Overlay.setShowViewportSizeOnResize";
    type ReturnObject = SetShowViewportSizeOnResizeReturnObject;
}
impl Method for SetShowHinge {
    const NAME: &'static str = "Overlay.setShowHinge";
    type ReturnObject = SetShowHingeReturnObject;
}
impl Method for SetShowIsolatedElements {
    const NAME: &'static str = "Overlay.setShowIsolatedElements";
    type ReturnObject = SetShowIsolatedElementsReturnObject;
}
impl Method for SetShowWindowControlsOverlay {
    const NAME: &'static str = "Overlay.setShowWindowControlsOverlay";
    type ReturnObject = SetShowWindowControlsOverlayReturnObject;
}
pub mod events {
    #[allow(unused_imports)]
    use super::super::types::*;
    #[allow(unused_imports)]
    use serde::{Deserialize, Serialize};
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
    pub struct InspectNodeRequestedEvent {
        pub params: InspectNodeRequestedEventParams,
    }
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
    pub struct InspectNodeRequestedEventParams {
        #[serde(rename = "backendNodeId")]
        pub backend_node_id: super::super::dom::BackendNodeId,
    }
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
    pub struct NodeHighlightRequestedEvent {
        pub params: NodeHighlightRequestedEventParams,
    }
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
    pub struct NodeHighlightRequestedEventParams {
        #[serde(rename = "nodeId")]
        pub node_id: super::super::dom::NodeId,
    }
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
    pub struct ScreenshotRequestedEvent {
        pub params: ScreenshotRequestedEventParams,
    }
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
    pub struct ScreenshotRequestedEventParams {
        #[serde(rename = "viewport")]
        pub viewport: super::super::page::Viewport,
    }
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
    #[serde(rename_all = "camelCase")]
    pub struct InspectModeCanceledEvent(pub Option<serde_json::Value>);
}
