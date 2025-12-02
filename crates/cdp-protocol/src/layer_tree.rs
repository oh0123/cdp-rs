// Auto-generated from Chrome at version 140.0.7339.186 domain: LayerTree
use super::dom;
#[allow(unused_imports)]
use super::types::*;
#[allow(unused_imports)]
use serde::{Deserialize, Serialize};
#[allow(unused_imports)]
use serde_json::Value as Json;
pub type LayerId = String;
pub type SnapshotId = String;
pub type PaintProfile = Vec<JsFloat>;
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub enum ScrollRectType {
    #[serde(rename = "RepaintsOnScroll")]
    RepaintsOnScroll,
    #[serde(rename = "TouchEventHandler")]
    TouchEventHandler,
    #[serde(rename = "WheelEventHandler")]
    WheelEventHandler,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct ScrollRect {
    #[serde(rename = "rect")]
    pub rect: dom::Rect,
    #[serde(rename = "type")]
    pub r#type: ScrollRectType,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct StickyPositionConstraint {
    #[serde(rename = "stickyBoxRect")]
    pub sticky_box_rect: dom::Rect,
    #[serde(rename = "containingBlockRect")]
    pub containing_block_rect: dom::Rect,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "nearestLayerShiftingStickyBox")]
    pub nearest_layer_shifting_sticky_box: Option<LayerId>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "nearestLayerShiftingContainingBlock")]
    pub nearest_layer_shifting_containing_block: Option<LayerId>,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct PictureTile {
    #[serde(default)]
    #[serde(rename = "x")]
    pub x: JsFloat,
    #[serde(default)]
    #[serde(rename = "y")]
    pub y: JsFloat,
    #[serde(rename = "picture")]
    pub picture: Vec<u8>,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct Layer {
    #[serde(rename = "layerId")]
    pub layer_id: LayerId,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "parentLayerId")]
    pub parent_layer_id: Option<LayerId>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "backendNodeId")]
    pub backend_node_id: Option<dom::BackendNodeId>,
    #[serde(default)]
    #[serde(rename = "offsetX")]
    pub offset_x: JsFloat,
    #[serde(default)]
    #[serde(rename = "offsetY")]
    pub offset_y: JsFloat,
    #[serde(default)]
    #[serde(rename = "width")]
    pub width: JsFloat,
    #[serde(default)]
    #[serde(rename = "height")]
    pub height: JsFloat,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(rename = "transform")]
    pub transform: Option<Vec<JsFloat>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(rename = "anchorX")]
    pub anchor_x: Option<JsFloat>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(rename = "anchorY")]
    pub anchor_y: Option<JsFloat>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(rename = "anchorZ")]
    pub anchor_z: Option<JsFloat>,
    #[serde(default)]
    #[serde(rename = "paintCount")]
    pub paint_count: JsUInt,
    #[serde(default)]
    #[serde(rename = "drawsContent")]
    pub draws_content: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(rename = "invisible")]
    pub invisible: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "scrollRects")]
    pub scroll_rects: Option<Vec<ScrollRect>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "stickyPositionConstraint")]
    pub sticky_position_constraint: Option<StickyPositionConstraint>,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct CompositingReasons {
    #[serde(rename = "layerId")]
    pub layer_id: LayerId,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Disable(pub Option<serde_json::Value>);
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Enable(pub Option<serde_json::Value>);
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct LoadSnapshot {
    #[serde(rename = "tiles")]
    pub tiles: Vec<PictureTile>,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct MakeSnapshot {
    #[serde(rename = "layerId")]
    pub layer_id: LayerId,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct ProfileSnapshot {
    #[serde(rename = "snapshotId")]
    pub snapshot_id: SnapshotId,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(rename = "minRepeatCount")]
    pub min_repeat_count: Option<JsUInt>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(rename = "minDuration")]
    pub min_duration: Option<JsFloat>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "clipRect")]
    pub clip_rect: Option<dom::Rect>,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct ReleaseSnapshot {
    #[serde(rename = "snapshotId")]
    pub snapshot_id: SnapshotId,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct ReplaySnapshot {
    #[serde(rename = "snapshotId")]
    pub snapshot_id: SnapshotId,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(rename = "fromStep")]
    pub from_step: Option<JsUInt>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(rename = "toStep")]
    pub to_step: Option<JsUInt>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(rename = "scale")]
    pub scale: Option<JsFloat>,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct SnapshotCommandLog {
    #[serde(rename = "snapshotId")]
    pub snapshot_id: SnapshotId,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct CompositingReasonsReturnObject {
    #[serde(rename = "compositingReasons")]
    pub compositing_reasons: Vec<String>,
    #[serde(rename = "compositingReasonIds")]
    pub compositing_reason_ids: Vec<String>,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct DisableReturnObject {}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct EnableReturnObject {}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct LoadSnapshotReturnObject {
    #[serde(rename = "snapshotId")]
    pub snapshot_id: SnapshotId,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct MakeSnapshotReturnObject {
    #[serde(rename = "snapshotId")]
    pub snapshot_id: SnapshotId,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct ProfileSnapshotReturnObject {
    #[serde(rename = "timings")]
    pub timings: Vec<PaintProfile>,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct ReleaseSnapshotReturnObject {}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct ReplaySnapshotReturnObject {
    #[serde(default)]
    #[serde(rename = "dataURL")]
    pub data_url: String,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct SnapshotCommandLogReturnObject {}
impl Method for CompositingReasons {
    const NAME: &'static str = "LayerTree.compositingReasons";
    type ReturnObject = CompositingReasonsReturnObject;
}
impl Method for Disable {
    const NAME: &'static str = "LayerTree.disable";
    type ReturnObject = DisableReturnObject;
}
impl Method for Enable {
    const NAME: &'static str = "LayerTree.enable";
    type ReturnObject = EnableReturnObject;
}
impl Method for LoadSnapshot {
    const NAME: &'static str = "LayerTree.loadSnapshot";
    type ReturnObject = LoadSnapshotReturnObject;
}
impl Method for MakeSnapshot {
    const NAME: &'static str = "LayerTree.makeSnapshot";
    type ReturnObject = MakeSnapshotReturnObject;
}
impl Method for ProfileSnapshot {
    const NAME: &'static str = "LayerTree.profileSnapshot";
    type ReturnObject = ProfileSnapshotReturnObject;
}
impl Method for ReleaseSnapshot {
    const NAME: &'static str = "LayerTree.releaseSnapshot";
    type ReturnObject = ReleaseSnapshotReturnObject;
}
impl Method for ReplaySnapshot {
    const NAME: &'static str = "LayerTree.replaySnapshot";
    type ReturnObject = ReplaySnapshotReturnObject;
}
impl Method for SnapshotCommandLog {
    const NAME: &'static str = "LayerTree.snapshotCommandLog";
    type ReturnObject = SnapshotCommandLogReturnObject;
}
pub mod events {
    #[allow(unused_imports)]
    use super::super::types::*;
    #[allow(unused_imports)]
    use serde::{Deserialize, Serialize};
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
    pub struct LayerPaintedEvent {
        pub params: LayerPaintedEventParams,
    }
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
    pub struct LayerPaintedEventParams {
        #[serde(rename = "layerId")]
        pub layer_id: super::LayerId,
        #[serde(rename = "clip")]
        pub clip: super::super::dom::Rect,
    }
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
    pub struct LayerTreeDidChangeEvent {
        pub params: LayerTreeDidChangeEventParams,
    }
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
    pub struct LayerTreeDidChangeEventParams {
        #[serde(skip_serializing_if = "Option::is_none")]
        #[serde(rename = "layers")]
        pub layers: Option<Vec<super::Layer>>,
    }
}
