// Auto-generated from Chrome at version 140.0.7339.186 domain: HeapProfiler
use super::runtime;
#[allow(unused_imports)]
use super::types::*;
#[allow(unused_imports)]
use serde::{Deserialize, Serialize};
#[allow(unused_imports)]
use serde_json::Value as Json;
pub type HeapSnapshotObjectId = String;
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct SamplingHeapProfileNode {
    #[serde(rename = "callFrame")]
    pub call_frame: runtime::CallFrame,
    #[serde(default)]
    #[serde(rename = "selfSize")]
    pub self_size: JsFloat,
    #[serde(default)]
    #[serde(rename = "id")]
    pub id: JsUInt,
    #[serde(rename = "children")]
    pub children: Vec<SamplingHeapProfileNode>,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct SamplingHeapProfileSample {
    #[serde(default)]
    #[serde(rename = "size")]
    pub size: JsFloat,
    #[serde(default)]
    #[serde(rename = "nodeId")]
    pub node_id: JsUInt,
    #[serde(default)]
    #[serde(rename = "ordinal")]
    pub ordinal: JsFloat,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct SamplingHeapProfile {
    #[serde(rename = "head")]
    pub head: SamplingHeapProfileNode,
    #[serde(rename = "samples")]
    pub samples: Vec<SamplingHeapProfileSample>,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct AddInspectedHeapObject {
    #[serde(rename = "heapObjectId")]
    pub heap_object_id: HeapSnapshotObjectId,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct CollectGarbage(pub Option<serde_json::Value>);
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Disable(pub Option<serde_json::Value>);
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Enable(pub Option<serde_json::Value>);
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct GetHeapObjectId {
    #[serde(rename = "objectId")]
    pub object_id: runtime::RemoteObjectId,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct GetObjectByHeapObjectId {
    #[serde(rename = "objectId")]
    pub object_id: HeapSnapshotObjectId,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(rename = "objectGroup")]
    pub object_group: Option<String>,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct GetSamplingProfile(pub Option<serde_json::Value>);
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct StartSampling {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(rename = "samplingInterval")]
    pub sampling_interval: Option<JsFloat>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(rename = "includeObjectsCollectedByMajorGC")]
    pub include_objects_collected_by_major_gc: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(rename = "includeObjectsCollectedByMinorGC")]
    pub include_objects_collected_by_minor_gc: Option<bool>,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct StartTrackingHeapObjects {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(rename = "trackAllocations")]
    pub track_allocations: Option<bool>,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct StopSampling(pub Option<serde_json::Value>);
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct StopTrackingHeapObjects {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(rename = "reportProgress")]
    pub report_progress: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(rename = "treatGlobalObjectsAsRoots")]
    pub treat_global_objects_as_roots: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(rename = "captureNumericValue")]
    pub capture_numeric_value: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(rename = "exposeInternals")]
    pub expose_internals: Option<bool>,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct TakeHeapSnapshot {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(rename = "reportProgress")]
    pub report_progress: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(rename = "treatGlobalObjectsAsRoots")]
    pub treat_global_objects_as_roots: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(rename = "captureNumericValue")]
    pub capture_numeric_value: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(rename = "exposeInternals")]
    pub expose_internals: Option<bool>,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct AddInspectedHeapObjectReturnObject {}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct CollectGarbageReturnObject {}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct DisableReturnObject {}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct EnableReturnObject {}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct GetHeapObjectIdReturnObject {
    #[serde(rename = "heapSnapshotObjectId")]
    pub heap_snapshot_object_id: HeapSnapshotObjectId,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct GetObjectByHeapObjectIdReturnObject {
    #[serde(rename = "result")]
    pub result: runtime::RemoteObject,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct GetSamplingProfileReturnObject {
    #[serde(rename = "profile")]
    pub profile: SamplingHeapProfile,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct StartSamplingReturnObject {}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct StartTrackingHeapObjectsReturnObject {}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct StopSamplingReturnObject {
    #[serde(rename = "profile")]
    pub profile: SamplingHeapProfile,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct StopTrackingHeapObjectsReturnObject {}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct TakeHeapSnapshotReturnObject {}
impl Method for AddInspectedHeapObject {
    const NAME: &'static str = "HeapProfiler.addInspectedHeapObject";
    type ReturnObject = AddInspectedHeapObjectReturnObject;
}
impl Method for CollectGarbage {
    const NAME: &'static str = "HeapProfiler.collectGarbage";
    type ReturnObject = CollectGarbageReturnObject;
}
impl Method for Disable {
    const NAME: &'static str = "HeapProfiler.disable";
    type ReturnObject = DisableReturnObject;
}
impl Method for Enable {
    const NAME: &'static str = "HeapProfiler.enable";
    type ReturnObject = EnableReturnObject;
}
impl Method for GetHeapObjectId {
    const NAME: &'static str = "HeapProfiler.getHeapObjectId";
    type ReturnObject = GetHeapObjectIdReturnObject;
}
impl Method for GetObjectByHeapObjectId {
    const NAME: &'static str = "HeapProfiler.getObjectByHeapObjectId";
    type ReturnObject = GetObjectByHeapObjectIdReturnObject;
}
impl Method for GetSamplingProfile {
    const NAME: &'static str = "HeapProfiler.getSamplingProfile";
    type ReturnObject = GetSamplingProfileReturnObject;
}
impl Method for StartSampling {
    const NAME: &'static str = "HeapProfiler.startSampling";
    type ReturnObject = StartSamplingReturnObject;
}
impl Method for StartTrackingHeapObjects {
    const NAME: &'static str = "HeapProfiler.startTrackingHeapObjects";
    type ReturnObject = StartTrackingHeapObjectsReturnObject;
}
impl Method for StopSampling {
    const NAME: &'static str = "HeapProfiler.stopSampling";
    type ReturnObject = StopSamplingReturnObject;
}
impl Method for StopTrackingHeapObjects {
    const NAME: &'static str = "HeapProfiler.stopTrackingHeapObjects";
    type ReturnObject = StopTrackingHeapObjectsReturnObject;
}
impl Method for TakeHeapSnapshot {
    const NAME: &'static str = "HeapProfiler.takeHeapSnapshot";
    type ReturnObject = TakeHeapSnapshotReturnObject;
}
pub mod events {
    #[allow(unused_imports)]
    use super::super::types::*;
    #[allow(unused_imports)]
    use serde::{Deserialize, Serialize};
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
    pub struct AddHeapSnapshotChunkEvent {
        pub params: AddHeapSnapshotChunkEventParams,
    }
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
    pub struct AddHeapSnapshotChunkEventParams {
        #[serde(default)]
        #[serde(rename = "chunk")]
        pub chunk: String,
    }
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
    pub struct HeapStatsUpdateEvent {
        pub params: HeapStatsUpdateEventParams,
    }
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
    pub struct HeapStatsUpdateEventParams {
        #[serde(default)]
        #[serde(rename = "statsUpdate")]
        pub stats_update: Vec<JsUInt>,
    }
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
    pub struct LastSeenObjectIdEvent {
        pub params: LastSeenObjectIdEventParams,
    }
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
    pub struct LastSeenObjectIdEventParams {
        #[serde(default)]
        #[serde(rename = "lastSeenObjectId")]
        pub last_seen_object_id: JsUInt,
        #[serde(default)]
        #[serde(rename = "timestamp")]
        pub timestamp: JsFloat,
    }
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
    pub struct ReportHeapSnapshotProgressEvent {
        pub params: ReportHeapSnapshotProgressEventParams,
    }
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
    pub struct ReportHeapSnapshotProgressEventParams {
        #[serde(default)]
        #[serde(rename = "done")]
        pub done: JsUInt,
        #[serde(default)]
        #[serde(rename = "total")]
        pub total: JsUInt,
        #[serde(skip_serializing_if = "Option::is_none")]
        #[serde(default)]
        #[serde(rename = "finished")]
        pub finished: Option<bool>,
    }
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
    #[serde(rename_all = "camelCase")]
    pub struct ResetProfilesEvent(pub Option<serde_json::Value>);
}
