// Auto-generated from Chrome at version 146.0.7680.165 domain: LayerTree
use super::dom;
#[allow(unused_imports)]
use super::types::*;
#[allow(unused_imports)]
use derive_builder::Builder;
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
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "camelCase")]
#[doc = "Rectangle where scrolling happens on the main thread."]
pub struct ScrollRect {
    #[doc = "Rectangle itself."]
    pub rect: dom::Rect,
    #[doc = "Reason for rectangle to force scrolling on the main thread"]
    pub r#type: ScrollRectType,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "camelCase")]
#[doc = "Sticky position constraints."]
pub struct StickyPositionConstraint {
    #[doc = "Layout rectangle of the sticky element before being shifted"]
    pub sticky_box_rect: dom::Rect,
    #[doc = "Layout rectangle of the containing block of the sticky element"]
    pub containing_block_rect: dom::Rect,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[doc = "The nearest sticky layer that shifts the sticky box"]
    pub nearest_layer_shifting_sticky_box: Option<LayerId>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[doc = "The nearest sticky layer that shifts the containing block"]
    pub nearest_layer_shifting_containing_block: Option<LayerId>,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "camelCase")]
#[doc = "Serialized fragment of layer picture along with its offset within the layer."]
pub struct PictureTile {
    #[serde(default)]
    #[doc = "Offset from owning layer left boundary"]
    pub x: JsFloat,
    #[serde(default)]
    #[doc = "Offset from owning layer top boundary"]
    pub y: JsFloat,
    #[doc = "Base64-encoded snapshot data."]
    pub picture: Vec<u8>,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "camelCase")]
#[doc = "Information about a compositing layer."]
pub struct Layer {
    #[doc = "The unique id for this layer."]
    pub layer_id: LayerId,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[doc = "The id of parent (not present for root)."]
    pub parent_layer_id: Option<LayerId>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[doc = "The backend id for the node associated with this layer."]
    pub backend_node_id: Option<dom::BackendNodeId>,
    #[serde(default)]
    #[doc = "Offset from parent layer, X coordinate."]
    pub offset_x: JsFloat,
    #[serde(default)]
    #[doc = "Offset from parent layer, Y coordinate."]
    pub offset_y: JsFloat,
    #[serde(default)]
    #[doc = "Layer width."]
    pub width: JsFloat,
    #[serde(default)]
    #[doc = "Layer height."]
    pub height: JsFloat,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[doc = "Transformation matrix for layer, default is identity matrix"]
    pub transform: Option<Vec<JsFloat>>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[doc = "Transform anchor point X, absent if no transform specified"]
    pub anchor_x: Option<JsFloat>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[doc = "Transform anchor point Y, absent if no transform specified"]
    pub anchor_y: Option<JsFloat>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[doc = "Transform anchor point Z, absent if no transform specified"]
    pub anchor_z: Option<JsFloat>,
    #[serde(default)]
    #[doc = "Indicates how many time this layer has painted."]
    pub paint_count: JsUInt,
    #[serde(default)]
    #[doc = "Indicates whether this layer hosts any content, rather than being used for\n transform/scrolling purposes only."]
    pub draws_content: bool,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[doc = "Set if layer is not visible."]
    pub invisible: Option<bool>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[doc = "Rectangles scrolling on main thread only."]
    pub scroll_rects: Option<Vec<ScrollRect>>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[doc = "Sticky position constraint information"]
    pub sticky_position_constraint: Option<StickyPositionConstraint>,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "camelCase")]
#[doc = "Provides the reasons why the given layer was composited."]
pub struct CompositingReasons {
    #[doc = "The id of the layer for which we want to get the reasons it was composited."]
    pub layer_id: LayerId,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct Disable(pub Option<Json>);
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct Enable(pub Option<Json>);
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "camelCase")]
#[doc = "Returns the snapshot identifier."]
pub struct LoadSnapshot {
    #[doc = "An array of tiles composing the snapshot."]
    pub tiles: Vec<PictureTile>,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "camelCase")]
#[doc = "Returns the layer snapshot identifier."]
pub struct MakeSnapshot {
    #[doc = "The id of the layer."]
    pub layer_id: LayerId,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "camelCase")]
pub struct ProfileSnapshot {
    #[doc = "The id of the layer snapshot."]
    pub snapshot_id: SnapshotId,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[doc = "The maximum number of times to replay the snapshot (1, if not specified)."]
    pub min_repeat_count: Option<JsUInt>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[doc = "The minimum duration (in seconds) to replay the snapshot."]
    pub min_duration: Option<JsFloat>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[doc = "The clip rectangle to apply when replaying the snapshot."]
    pub clip_rect: Option<dom::Rect>,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "camelCase")]
#[doc = "Releases layer snapshot captured by the back-end."]
pub struct ReleaseSnapshot {
    #[doc = "The id of the layer snapshot."]
    pub snapshot_id: SnapshotId,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "camelCase")]
#[doc = "Replays the layer snapshot and returns the resulting bitmap."]
pub struct ReplaySnapshot {
    #[doc = "The id of the layer snapshot."]
    pub snapshot_id: SnapshotId,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[doc = "The first step to replay from (replay from the very start if not specified)."]
    pub from_step: Option<JsUInt>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[doc = "The last step to replay to (replay till the end if not specified)."]
    pub to_step: Option<JsUInt>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[doc = "The scale to apply while replaying (defaults to 1)."]
    pub scale: Option<JsFloat>,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "camelCase")]
#[doc = "Replays the layer snapshot and returns canvas log."]
pub struct SnapshotCommandLog {
    #[doc = "The id of the layer snapshot."]
    pub snapshot_id: SnapshotId,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[serde(rename_all = "camelCase")]
#[doc = "Provides the reasons why the given layer was composited."]
pub struct CompositingReasonsReturnObject {
    #[doc = "A list of strings specifying reasons for the given layer to become composited."]
    pub compositing_reasons: Vec<String>,
    #[doc = "A list of strings specifying reason IDs for the given layer to become composited."]
    pub compositing_reason_ids: Vec<String>,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[doc = "Disables compositing tree inspection."]
pub struct DisableReturnObject(pub Option<Json>);
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[doc = "Enables compositing tree inspection."]
pub struct EnableReturnObject(pub Option<Json>);
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[serde(rename_all = "camelCase")]
#[doc = "Returns the snapshot identifier."]
pub struct LoadSnapshotReturnObject {
    #[doc = "The id of the snapshot."]
    pub snapshot_id: SnapshotId,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[serde(rename_all = "camelCase")]
#[doc = "Returns the layer snapshot identifier."]
pub struct MakeSnapshotReturnObject {
    #[doc = "The id of the layer snapshot."]
    pub snapshot_id: SnapshotId,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[serde(rename_all = "camelCase")]
pub struct ProfileSnapshotReturnObject {
    #[doc = "The array of paint profiles, one per run."]
    pub timings: Vec<PaintProfile>,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[doc = "Releases layer snapshot captured by the back-end."]
pub struct ReleaseSnapshotReturnObject(pub Option<Json>);
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[serde(rename_all = "camelCase")]
#[doc = "Replays the layer snapshot and returns the resulting bitmap."]
pub struct ReplaySnapshotReturnObject {
    #[serde(default)]
    #[doc = "A data: URL for resulting image."]
    #[serde(rename = "dataURL")]
    pub data_url: String,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[serde(rename_all = "camelCase")]
#[doc = "Replays the layer snapshot and returns canvas log."]
pub struct SnapshotCommandLogReturnObject {
    #[doc = "The array of canvas function calls."]
    pub command_log: Vec<Json>,
}
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
    use derive_builder::Builder;
    #[allow(unused_imports)]
    use serde::{Deserialize, Serialize};
    #[allow(unused_imports)]
    use serde_json::Value as Json;
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
    pub struct LayerPaintedEvent {
        pub params: LayerPaintedEventParams,
    }
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
    pub struct LayerPaintedEventParams {
        #[doc = "The id of the painted layer."]
        pub layer_id: super::LayerId,
        #[doc = "Clip rectangle."]
        pub clip: super::super::dom::Rect,
    }
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
    pub struct LayerTreeDidChangeEvent {
        pub params: LayerTreeDidChangeEventParams,
    }
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
    pub struct LayerTreeDidChangeEventParams {
        #[builder(default)]
        #[serde(skip_serializing_if = "Option::is_none")]
        #[doc = "Layer tree, absent if not in the compositing mode."]
        pub layers: Option<Vec<super::Layer>>,
    }
}
