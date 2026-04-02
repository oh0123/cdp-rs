// Auto-generated from Chrome at version 146.0.7680.165 domain: HeapProfiler
#![allow(dead_code)]
use super::runtime;
#[allow(unused_imports)]
use super::types::*;
#[allow(unused_imports)]
use derive_builder::Builder;
#[allow(unused_imports)]
use serde::{Deserialize, Serialize};
#[allow(unused_imports)]
use serde_json::Value as Json;
pub type HeapSnapshotObjectId = String;
#[allow(deprecated)]
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "camelCase")]
#[doc = "Sampling Heap Profile node. Holds callsite information, allocation statistics and child nodes."]
pub struct SamplingHeapProfileNode {
    #[doc = "Function location."]
    pub call_frame: runtime::CallFrame,
    #[serde(default)]
    #[doc = "Allocations size in bytes for the node excluding children."]
    pub self_size: JsFloat,
    #[serde(default)]
    #[doc = "Node id. Ids are unique across all profiles collected between startSampling and stopSampling."]
    pub id: JsUInt,
    #[doc = "Child nodes."]
    pub children: Vec<SamplingHeapProfileNode>,
}
#[allow(deprecated)]
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "camelCase")]
#[doc = "A single sample from a sampling profile."]
pub struct SamplingHeapProfileSample {
    #[serde(default)]
    #[doc = "Allocation size in bytes attributed to the sample."]
    pub size: JsFloat,
    #[serde(default)]
    #[doc = "Id of the corresponding profile tree node."]
    pub node_id: JsUInt,
    #[serde(default)]
    #[doc = "Time-ordered sample ordinal number. It is unique across all profiles retrieved\n between startSampling and stopSampling."]
    pub ordinal: JsFloat,
}
#[allow(deprecated)]
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "camelCase")]
#[doc = "Sampling profile."]
pub struct SamplingHeapProfile {
    pub head: SamplingHeapProfileNode,
    pub samples: Vec<SamplingHeapProfileSample>,
}
#[allow(deprecated)]
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "camelCase")]
#[doc = "Enables console to refer to the node with given id via $x (see Command Line API for more details\n $x functions)."]
pub struct AddInspectedHeapObject {
    #[doc = "Heap snapshot object id to be accessible by means of $x command line API."]
    pub heap_object_id: HeapSnapshotObjectId,
}
#[allow(deprecated)]
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct CollectGarbage(pub Option<Json>);
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
pub struct GetHeapObjectId {
    #[doc = "Identifier of the object to get heap object id for."]
    pub object_id: runtime::RemoteObjectId,
}
#[allow(deprecated)]
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "camelCase")]
pub struct GetObjectByHeapObjectId {
    pub object_id: HeapSnapshotObjectId,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[doc = "Symbolic group name that can be used to release multiple objects."]
    pub object_group: Option<String>,
}
#[allow(deprecated)]
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct GetSamplingProfile(pub Option<Json>);
#[allow(deprecated)]
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "camelCase")]
pub struct StartSampling {
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[doc = "Average sample interval in bytes. Poisson distribution is used for the intervals. The\n default value is 32768 bytes."]
    pub sampling_interval: Option<JsFloat>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[doc = "Maximum stack depth. The default value is 128."]
    pub stack_depth: Option<JsFloat>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[doc = "By default, the sampling heap profiler reports only objects which are\n still alive when the profile is returned via getSamplingProfile or\n stopSampling, which is useful for determining what functions contribute\n the most to steady-state memory usage. This flag instructs the sampling\n heap profiler to also include information about objects discarded by\n major GC, which will show which functions cause large temporary memory\n usage or long GC pauses."]
    #[serde(rename = "includeObjectsCollectedByMajorGC")]
    pub include_objects_collected_by_major_gc: Option<bool>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[doc = "By default, the sampling heap profiler reports only objects which are\n still alive when the profile is returned via getSamplingProfile or\n stopSampling, which is useful for determining what functions contribute\n the most to steady-state memory usage. This flag instructs the sampling\n heap profiler to also include information about objects discarded by\n minor GC, which is useful when tuning a latency-sensitive application\n for minimal GC activity."]
    #[serde(rename = "includeObjectsCollectedByMinorGC")]
    pub include_objects_collected_by_minor_gc: Option<bool>,
}
#[allow(deprecated)]
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "camelCase")]
pub struct StartTrackingHeapObjects {
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub track_allocations: Option<bool>,
}
#[allow(deprecated)]
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct StopSampling(pub Option<Json>);
#[allow(deprecated)]
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "camelCase")]
pub struct StopTrackingHeapObjects {
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[doc = "If true 'reportHeapSnapshotProgress' events will be generated while snapshot is being taken\n when the tracking is stopped."]
    pub report_progress: Option<bool>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[doc = "Deprecated in favor of `exposeInternals`."]
    #[deprecated]
    pub treat_global_objects_as_roots: Option<bool>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[doc = "If true, numerical values are included in the snapshot"]
    pub capture_numeric_value: Option<bool>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[doc = "If true, exposes internals of the snapshot."]
    pub expose_internals: Option<bool>,
}
#[allow(deprecated)]
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "camelCase")]
pub struct TakeHeapSnapshot {
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[doc = "If true 'reportHeapSnapshotProgress' events will be generated while snapshot is being taken."]
    pub report_progress: Option<bool>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[doc = "If true, a raw snapshot without artificial roots will be generated.\n Deprecated in favor of `exposeInternals`."]
    #[deprecated]
    pub treat_global_objects_as_roots: Option<bool>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[doc = "If true, numerical values are included in the snapshot"]
    pub capture_numeric_value: Option<bool>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[doc = "If true, exposes internals of the snapshot."]
    pub expose_internals: Option<bool>,
}
#[allow(deprecated)]
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[doc = "Enables console to refer to the node with given id via $x (see Command Line API for more details\n $x functions)."]
pub struct AddInspectedHeapObjectReturnObject(pub Option<Json>);
#[allow(deprecated)]
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct CollectGarbageReturnObject(pub Option<Json>);
#[allow(deprecated)]
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct DisableReturnObject(pub Option<Json>);
#[allow(deprecated)]
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct EnableReturnObject(pub Option<Json>);
#[allow(deprecated)]
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[serde(rename_all = "camelCase")]
pub struct GetHeapObjectIdReturnObject {
    #[doc = "Id of the heap snapshot object corresponding to the passed remote object id."]
    pub heap_snapshot_object_id: HeapSnapshotObjectId,
}
#[allow(deprecated)]
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[serde(rename_all = "camelCase")]
pub struct GetObjectByHeapObjectIdReturnObject {
    #[doc = "Evaluation result."]
    pub result: runtime::RemoteObject,
}
#[allow(deprecated)]
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[serde(rename_all = "camelCase")]
pub struct GetSamplingProfileReturnObject {
    #[doc = "Return the sampling profile being collected."]
    pub profile: SamplingHeapProfile,
}
#[allow(deprecated)]
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct StartSamplingReturnObject(pub Option<Json>);
#[allow(deprecated)]
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct StartTrackingHeapObjectsReturnObject(pub Option<Json>);
#[allow(deprecated)]
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[serde(rename_all = "camelCase")]
pub struct StopSamplingReturnObject {
    #[doc = "Recorded sampling heap profile."]
    pub profile: SamplingHeapProfile,
}
#[allow(deprecated)]
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct StopTrackingHeapObjectsReturnObject(pub Option<Json>);
#[allow(deprecated)]
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct TakeHeapSnapshotReturnObject(pub Option<Json>);
#[allow(deprecated)]
impl Method for AddInspectedHeapObject {
    const NAME: &'static str = "HeapProfiler.addInspectedHeapObject";
    type ReturnObject = AddInspectedHeapObjectReturnObject;
}
#[allow(deprecated)]
impl Method for CollectGarbage {
    const NAME: &'static str = "HeapProfiler.collectGarbage";
    type ReturnObject = CollectGarbageReturnObject;
}
#[allow(deprecated)]
impl Method for Disable {
    const NAME: &'static str = "HeapProfiler.disable";
    type ReturnObject = DisableReturnObject;
}
#[allow(deprecated)]
impl Method for Enable {
    const NAME: &'static str = "HeapProfiler.enable";
    type ReturnObject = EnableReturnObject;
}
#[allow(deprecated)]
impl Method for GetHeapObjectId {
    const NAME: &'static str = "HeapProfiler.getHeapObjectId";
    type ReturnObject = GetHeapObjectIdReturnObject;
}
#[allow(deprecated)]
impl Method for GetObjectByHeapObjectId {
    const NAME: &'static str = "HeapProfiler.getObjectByHeapObjectId";
    type ReturnObject = GetObjectByHeapObjectIdReturnObject;
}
#[allow(deprecated)]
impl Method for GetSamplingProfile {
    const NAME: &'static str = "HeapProfiler.getSamplingProfile";
    type ReturnObject = GetSamplingProfileReturnObject;
}
#[allow(deprecated)]
impl Method for StartSampling {
    const NAME: &'static str = "HeapProfiler.startSampling";
    type ReturnObject = StartSamplingReturnObject;
}
#[allow(deprecated)]
impl Method for StartTrackingHeapObjects {
    const NAME: &'static str = "HeapProfiler.startTrackingHeapObjects";
    type ReturnObject = StartTrackingHeapObjectsReturnObject;
}
#[allow(deprecated)]
impl Method for StopSampling {
    const NAME: &'static str = "HeapProfiler.stopSampling";
    type ReturnObject = StopSamplingReturnObject;
}
#[allow(deprecated)]
impl Method for StopTrackingHeapObjects {
    const NAME: &'static str = "HeapProfiler.stopTrackingHeapObjects";
    type ReturnObject = StopTrackingHeapObjectsReturnObject;
}
#[allow(deprecated)]
impl Method for TakeHeapSnapshot {
    const NAME: &'static str = "HeapProfiler.takeHeapSnapshot";
    type ReturnObject = TakeHeapSnapshotReturnObject;
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
    pub struct AddHeapSnapshotChunkEvent {
        pub params: AddHeapSnapshotChunkEventParams,
    }
    #[allow(deprecated)]
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
    #[serde(rename_all = "camelCase")]
    pub struct AddHeapSnapshotChunkEventParams {
        #[serde(default)]
        pub chunk: String,
    }
    #[allow(deprecated)]
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
    pub struct HeapStatsUpdateEvent {
        pub params: HeapStatsUpdateEventParams,
    }
    #[allow(deprecated)]
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
    #[serde(rename_all = "camelCase")]
    pub struct HeapStatsUpdateEventParams {
        #[serde(default)]
        #[doc = "An array of triplets. Each triplet describes a fragment. The first integer is the fragment\n index, the second integer is a total count of objects for the fragment, the third integer is\n a total size of the objects for the fragment."]
        pub stats_update: Vec<JsUInt>,
    }
    #[allow(deprecated)]
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
    pub struct LastSeenObjectIdEvent {
        pub params: LastSeenObjectIdEventParams,
    }
    #[allow(deprecated)]
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
    #[serde(rename_all = "camelCase")]
    pub struct LastSeenObjectIdEventParams {
        #[serde(default)]
        pub last_seen_object_id: JsUInt,
        #[serde(default)]
        pub timestamp: JsFloat,
    }
    #[allow(deprecated)]
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
    pub struct ReportHeapSnapshotProgressEvent {
        pub params: ReportHeapSnapshotProgressEventParams,
    }
    #[allow(deprecated)]
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
    #[serde(rename_all = "camelCase")]
    pub struct ReportHeapSnapshotProgressEventParams {
        #[serde(default)]
        pub done: JsUInt,
        #[serde(default)]
        pub total: JsUInt,
        #[builder(default)]
        #[serde(skip_serializing_if = "Option::is_none")]
        #[serde(default)]
        pub finished: Option<bool>,
    }
    #[allow(deprecated)]
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
    pub struct ResetProfilesEvent(pub Option<Json>);
}
