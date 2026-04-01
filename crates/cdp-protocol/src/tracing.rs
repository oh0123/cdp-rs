// Auto-generated from Chrome at version 146.0.7680.165 domain: Tracing
#[allow(unused_imports)]
use super::types::*;
#[allow(unused_imports)]
use derive_builder::Builder;
#[allow(unused_imports)]
use serde::{Deserialize, Serialize};
#[allow(unused_imports)]
use serde_json::Value as Json;
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub enum TraceConfigRecordMode {
    #[serde(rename = "recordUntilFull")]
    RecordUntilFull,
    #[serde(rename = "recordContinuously")]
    RecordContinuously,
    #[serde(rename = "recordAsMuchAsPossible")]
    RecordAsMuchAsPossible,
    #[serde(rename = "echoToConsole")]
    EchoToConsole,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub enum StreamFormat {
    #[serde(rename = "json")]
    Json,
    #[serde(rename = "proto")]
    Proto,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub enum StreamCompression {
    #[serde(rename = "none")]
    None,
    #[serde(rename = "gzip")]
    Gzip,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub enum MemoryDumpLevelOfDetail {
    #[serde(rename = "background")]
    Background,
    #[serde(rename = "light")]
    Light,
    #[serde(rename = "detailed")]
    Detailed,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub enum TracingBackend {
    #[serde(rename = "auto")]
    Auto,
    #[serde(rename = "chrome")]
    Chrome,
    #[serde(rename = "system")]
    System,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub enum StartTransferModeOption {
    #[serde(rename = "ReportEvents")]
    ReportEvents,
    #[serde(rename = "ReturnAsStream")]
    ReturnAsStream,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct MemoryDumpConfig(pub Option<Json>);
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "camelCase")]
pub struct TraceConfig {
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[doc = "Controls how the trace buffer stores data. The default is `recordUntilFull`."]
    pub record_mode: Option<TraceConfigRecordMode>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[doc = "Size of the trace buffer in kilobytes. If not specified or zero is passed, a default value\n of 200 MB would be used."]
    pub trace_buffer_size_in_kb: Option<JsFloat>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[doc = "Turns on JavaScript stack sampling."]
    pub enable_sampling: Option<bool>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[doc = "Turns on system tracing."]
    pub enable_systrace: Option<bool>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[doc = "Turns on argument filter."]
    pub enable_argument_filter: Option<bool>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[doc = "Included category filters."]
    pub included_categories: Option<Vec<String>>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[doc = "Excluded category filters."]
    pub excluded_categories: Option<Vec<String>>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[doc = "Configuration to synthesize the delays in tracing."]
    pub synthetic_delays: Option<Vec<String>>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[doc = "Configuration for memory dump triggers. Used only when \"memory-infra\" category is enabled."]
    pub memory_dump_config: Option<MemoryDumpConfig>,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct End(pub Option<Json>);
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct GetCategories(pub Option<Json>);
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct GetTrackEventDescriptor(pub Option<Json>);
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "camelCase")]
#[doc = "Record a clock sync marker in the trace."]
pub struct RecordClockSyncMarker {
    #[serde(default)]
    #[doc = "The ID of this clock sync marker"]
    pub sync_id: String,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "camelCase")]
#[doc = "Request a global memory dump."]
pub struct RequestMemoryDump {
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[doc = "Enables more deterministic results by forcing garbage collection"]
    pub deterministic: Option<bool>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[doc = "Specifies level of details in memory dump. Defaults to \"detailed\"."]
    pub level_of_detail: Option<MemoryDumpLevelOfDetail>,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "camelCase")]
#[doc = "Start trace events collection."]
pub struct Start {
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[doc = "Category/tag filter"]
    #[deprecated]
    pub categories: Option<String>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[doc = "Tracing options"]
    #[deprecated]
    pub options: Option<String>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[doc = "If set, the agent will issue bufferUsage events at this interval, specified in milliseconds"]
    pub buffer_usage_reporting_interval: Option<JsFloat>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[doc = "Whether to report trace events as series of dataCollected events or to save trace to a\n stream (defaults to `ReportEvents`)."]
    pub transfer_mode: Option<StartTransferModeOption>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[doc = "Trace data format to use. This only applies when using `ReturnAsStream`\n transfer mode (defaults to `json`)."]
    pub stream_format: Option<StreamFormat>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[doc = "Compression format to use. This only applies when using `ReturnAsStream`\n transfer mode (defaults to `none`)"]
    pub stream_compression: Option<StreamCompression>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trace_config: Option<TraceConfig>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[doc = "Base64-encoded serialized perfetto.protos.TraceConfig protobuf message\n When specified, the parameters `categories`, `options`, `traceConfig`\n are ignored."]
    pub perfetto_config: Option<Vec<u8>>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[doc = "Backend type (defaults to `auto`)"]
    pub tracing_backend: Option<TracingBackend>,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[doc = "Stop trace events collection."]
pub struct EndReturnObject(pub Option<Json>);
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[serde(rename_all = "camelCase")]
#[doc = "Gets supported tracing categories."]
pub struct GetCategoriesReturnObject {
    #[doc = "A list of supported tracing categories."]
    pub categories: Vec<String>,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[serde(rename_all = "camelCase")]
#[doc = "Return a descriptor for all available tracing categories."]
pub struct GetTrackEventDescriptorReturnObject {
    #[doc = "Base64-encoded serialized perfetto.protos.TrackEventDescriptor protobuf message."]
    pub descriptor: Vec<u8>,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[doc = "Record a clock sync marker in the trace."]
pub struct RecordClockSyncMarkerReturnObject(pub Option<Json>);
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[serde(rename_all = "camelCase")]
#[doc = "Request a global memory dump."]
pub struct RequestMemoryDumpReturnObject {
    #[serde(default)]
    #[doc = "GUID of the resulting global memory dump."]
    pub dump_guid: String,
    #[serde(default)]
    #[doc = "True iff the global memory dump succeeded."]
    pub success: bool,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[doc = "Start trace events collection."]
pub struct StartReturnObject(pub Option<Json>);
impl Method for End {
    const NAME: &'static str = "Tracing.end";
    type ReturnObject = EndReturnObject;
}
impl Method for GetCategories {
    const NAME: &'static str = "Tracing.getCategories";
    type ReturnObject = GetCategoriesReturnObject;
}
impl Method for GetTrackEventDescriptor {
    const NAME: &'static str = "Tracing.getTrackEventDescriptor";
    type ReturnObject = GetTrackEventDescriptorReturnObject;
}
impl Method for RecordClockSyncMarker {
    const NAME: &'static str = "Tracing.recordClockSyncMarker";
    type ReturnObject = RecordClockSyncMarkerReturnObject;
}
impl Method for RequestMemoryDump {
    const NAME: &'static str = "Tracing.requestMemoryDump";
    type ReturnObject = RequestMemoryDumpReturnObject;
}
impl Method for Start {
    const NAME: &'static str = "Tracing.start";
    type ReturnObject = StartReturnObject;
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
    pub struct BufferUsageEvent {
        pub params: BufferUsageEventParams,
    }
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
    pub struct BufferUsageEventParams {
        #[builder(default)]
        #[serde(skip_serializing_if = "Option::is_none")]
        #[serde(default)]
        #[doc = "A number in range [0..1] that indicates the used size of event buffer as a fraction of its\n total size."]
        pub percent_full: Option<JsFloat>,
        #[builder(default)]
        #[serde(skip_serializing_if = "Option::is_none")]
        #[serde(default)]
        #[doc = "An approximate number of events in the trace log."]
        pub event_count: Option<JsFloat>,
        #[builder(default)]
        #[serde(skip_serializing_if = "Option::is_none")]
        #[serde(default)]
        #[doc = "A number in range [0..1] that indicates the used size of event buffer as a fraction of its\n total size."]
        pub value: Option<JsFloat>,
    }
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
    pub struct DataCollectedEvent {
        pub params: DataCollectedEventParams,
    }
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
    pub struct DataCollectedEventParams {
        #[serde(default)]
        pub value: Vec<Json>,
    }
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
    pub struct TracingCompleteEvent {
        pub params: TracingCompleteEventParams,
    }
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
    pub struct TracingCompleteEventParams {
        #[serde(default)]
        #[doc = "Indicates whether some trace data is known to have been lost, e.g. because the trace ring\n buffer wrapped around."]
        pub data_loss_occurred: bool,
        #[builder(default)]
        #[serde(skip_serializing_if = "Option::is_none")]
        #[doc = "A handle of the stream that holds resulting trace data."]
        pub stream: Option<super::super::io::StreamHandle>,
        #[builder(default)]
        #[serde(skip_serializing_if = "Option::is_none")]
        #[doc = "Trace data format of returned stream."]
        pub trace_format: Option<super::StreamFormat>,
        #[builder(default)]
        #[serde(skip_serializing_if = "Option::is_none")]
        #[doc = "Compression format of returned stream."]
        pub stream_compression: Option<super::StreamCompression>,
    }
}
