// Auto-generated from Chrome at version 140.0.7339.186 domain: Tracing
#[allow(unused_imports)]
use super::types::*;
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
#[serde(rename_all = "camelCase")]
pub struct MemoryDumpConfig(pub Option<serde_json::Value>);
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct TraceConfig {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "recordMode")]
    pub record_mode: Option<TraceConfigRecordMode>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(rename = "traceBufferSizeInKb")]
    pub trace_buffer_size_in_kb: Option<JsFloat>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(rename = "enableSampling")]
    pub enable_sampling: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(rename = "enableSystrace")]
    pub enable_systrace: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(rename = "enableArgumentFilter")]
    pub enable_argument_filter: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(rename = "includedCategories")]
    pub included_categories: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(rename = "excludedCategories")]
    pub excluded_categories: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(rename = "syntheticDelays")]
    pub synthetic_delays: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "memoryDumpConfig")]
    pub memory_dump_config: Option<MemoryDumpConfig>,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct End(pub Option<serde_json::Value>);
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct GetCategories(pub Option<serde_json::Value>);
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct RecordClockSyncMarker {
    #[serde(default)]
    #[serde(rename = "syncId")]
    pub sync_id: String,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct RequestMemoryDump {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(rename = "deterministic")]
    pub deterministic: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "levelOfDetail")]
    pub level_of_detail: Option<MemoryDumpLevelOfDetail>,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct Start {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(rename = "categories")]
    pub categories: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(rename = "options")]
    pub options: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(rename = "bufferUsageReportingInterval")]
    pub buffer_usage_reporting_interval: Option<JsFloat>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "transferMode")]
    pub transfer_mode: Option<StartTransferModeOption>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "streamFormat")]
    pub stream_format: Option<StreamFormat>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "streamCompression")]
    pub stream_compression: Option<StreamCompression>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "traceConfig")]
    pub trace_config: Option<TraceConfig>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "perfettoConfig")]
    pub perfetto_config: Option<Vec<u8>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "tracingBackend")]
    pub tracing_backend: Option<TracingBackend>,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct EndReturnObject {}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct GetCategoriesReturnObject {
    #[serde(rename = "categories")]
    pub categories: Vec<String>,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct RecordClockSyncMarkerReturnObject {}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct RequestMemoryDumpReturnObject {
    #[serde(default)]
    #[serde(rename = "dumpGuid")]
    pub dump_guid: String,
    #[serde(default)]
    #[serde(rename = "success")]
    pub success: bool,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct StartReturnObject {}
impl Method for End {
    const NAME: &'static str = "Tracing.end";
    type ReturnObject = EndReturnObject;
}
impl Method for GetCategories {
    const NAME: &'static str = "Tracing.getCategories";
    type ReturnObject = GetCategoriesReturnObject;
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
    use serde::{Deserialize, Serialize};
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
    pub struct BufferUsageEvent {
        pub params: BufferUsageEventParams,
    }
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
    pub struct BufferUsageEventParams {
        #[serde(skip_serializing_if = "Option::is_none")]
        #[serde(default)]
        #[serde(rename = "percentFull")]
        pub percent_full: Option<JsFloat>,
        #[serde(skip_serializing_if = "Option::is_none")]
        #[serde(default)]
        #[serde(rename = "eventCount")]
        pub event_count: Option<JsFloat>,
        #[serde(skip_serializing_if = "Option::is_none")]
        #[serde(default)]
        #[serde(rename = "value")]
        pub value: Option<JsFloat>,
    }
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
    pub struct DataCollectedEvent {
        pub params: DataCollectedEventParams,
    }
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
    pub struct DataCollectedEventParams {}
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
    pub struct TracingCompleteEvent {
        pub params: TracingCompleteEventParams,
    }
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
    pub struct TracingCompleteEventParams {
        #[serde(default)]
        #[serde(rename = "dataLossOccurred")]
        pub data_loss_occurred: bool,
        #[serde(skip_serializing_if = "Option::is_none")]
        #[serde(rename = "stream")]
        pub stream: Option<super::super::io::StreamHandle>,
        #[serde(skip_serializing_if = "Option::is_none")]
        #[serde(rename = "traceFormat")]
        pub trace_format: Option<super::StreamFormat>,
        #[serde(skip_serializing_if = "Option::is_none")]
        #[serde(rename = "streamCompression")]
        pub stream_compression: Option<super::StreamCompression>,
    }
}
