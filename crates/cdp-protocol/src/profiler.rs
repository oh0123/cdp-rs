// Auto-generated from Chrome at version 143.0.7499.110 domain: Profiler
use super::runtime;
#[allow(unused_imports)]
use super::types::*;
#[allow(unused_imports)]
use serde::{Deserialize, Serialize};
#[allow(unused_imports)]
use serde_json::Value as Json;
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct ProfileNode {
    #[serde(default)]
    #[serde(rename = "id")]
    pub id: JsUInt,
    #[serde(rename = "callFrame")]
    pub call_frame: runtime::CallFrame,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(rename = "hitCount")]
    pub hit_count: Option<JsUInt>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(rename = "children")]
    pub children: Option<Vec<JsUInt>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(rename = "deoptReason")]
    pub deopt_reason: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "positionTicks")]
    pub position_ticks: Option<Vec<PositionTickInfo>>,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct Profile {
    #[serde(rename = "nodes")]
    pub nodes: Vec<ProfileNode>,
    #[serde(default)]
    #[serde(rename = "startTime")]
    pub start_time: JsFloat,
    #[serde(default)]
    #[serde(rename = "endTime")]
    pub end_time: JsFloat,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(rename = "samples")]
    pub samples: Option<Vec<JsUInt>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(rename = "timeDeltas")]
    pub time_deltas: Option<Vec<JsUInt>>,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct PositionTickInfo {
    #[serde(default)]
    #[serde(rename = "line")]
    pub line: JsUInt,
    #[serde(default)]
    #[serde(rename = "ticks")]
    pub ticks: JsUInt,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct CoverageRange {
    #[serde(default)]
    #[serde(rename = "startOffset")]
    pub start_offset: JsUInt,
    #[serde(default)]
    #[serde(rename = "endOffset")]
    pub end_offset: JsUInt,
    #[serde(default)]
    #[serde(rename = "count")]
    pub count: JsUInt,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct FunctionCoverage {
    #[serde(default)]
    #[serde(rename = "functionName")]
    pub function_name: String,
    #[serde(rename = "ranges")]
    pub ranges: Vec<CoverageRange>,
    #[serde(default)]
    #[serde(rename = "isBlockCoverage")]
    pub is_block_coverage: bool,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct ScriptCoverage {
    #[serde(rename = "scriptId")]
    pub script_id: runtime::ScriptId,
    #[serde(default)]
    #[serde(rename = "url")]
    pub url: String,
    #[serde(rename = "functions")]
    pub functions: Vec<FunctionCoverage>,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Disable(pub Option<serde_json::Value>);
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Enable(pub Option<serde_json::Value>);
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct GetBestEffortCoverage(pub Option<serde_json::Value>);
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct SetSamplingInterval {
    #[serde(default)]
    #[serde(rename = "interval")]
    pub interval: JsUInt,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Start(pub Option<serde_json::Value>);
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct StartPreciseCoverage {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(rename = "callCount")]
    pub call_count: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(rename = "detailed")]
    pub detailed: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(rename = "allowTriggeredUpdates")]
    pub allow_triggered_updates: Option<bool>,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Stop(pub Option<serde_json::Value>);
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct StopPreciseCoverage(pub Option<serde_json::Value>);
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct TakePreciseCoverage(pub Option<serde_json::Value>);
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct DisableReturnObject {}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct EnableReturnObject {}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct GetBestEffortCoverageReturnObject {
    #[serde(rename = "result")]
    pub result: Vec<ScriptCoverage>,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct SetSamplingIntervalReturnObject {}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct StartReturnObject {}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct StartPreciseCoverageReturnObject {
    #[serde(default)]
    #[serde(rename = "timestamp")]
    pub timestamp: JsFloat,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct StopReturnObject {
    #[serde(rename = "profile")]
    pub profile: Profile,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct StopPreciseCoverageReturnObject {}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct TakePreciseCoverageReturnObject {
    #[serde(rename = "result")]
    pub result: Vec<ScriptCoverage>,
    #[serde(default)]
    #[serde(rename = "timestamp")]
    pub timestamp: JsFloat,
}
impl Method for Disable {
    const NAME: &'static str = "Profiler.disable";
    type ReturnObject = DisableReturnObject;
}
impl Method for Enable {
    const NAME: &'static str = "Profiler.enable";
    type ReturnObject = EnableReturnObject;
}
impl Method for GetBestEffortCoverage {
    const NAME: &'static str = "Profiler.getBestEffortCoverage";
    type ReturnObject = GetBestEffortCoverageReturnObject;
}
impl Method for SetSamplingInterval {
    const NAME: &'static str = "Profiler.setSamplingInterval";
    type ReturnObject = SetSamplingIntervalReturnObject;
}
impl Method for Start {
    const NAME: &'static str = "Profiler.start";
    type ReturnObject = StartReturnObject;
}
impl Method for StartPreciseCoverage {
    const NAME: &'static str = "Profiler.startPreciseCoverage";
    type ReturnObject = StartPreciseCoverageReturnObject;
}
impl Method for Stop {
    const NAME: &'static str = "Profiler.stop";
    type ReturnObject = StopReturnObject;
}
impl Method for StopPreciseCoverage {
    const NAME: &'static str = "Profiler.stopPreciseCoverage";
    type ReturnObject = StopPreciseCoverageReturnObject;
}
impl Method for TakePreciseCoverage {
    const NAME: &'static str = "Profiler.takePreciseCoverage";
    type ReturnObject = TakePreciseCoverageReturnObject;
}
pub mod events {
    #[allow(unused_imports)]
    use super::super::types::*;
    #[allow(unused_imports)]
    use serde::{Deserialize, Serialize};
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
    pub struct ConsoleProfileFinishedEvent {
        pub params: ConsoleProfileFinishedEventParams,
    }
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
    pub struct ConsoleProfileFinishedEventParams {
        #[serde(default)]
        #[serde(rename = "id")]
        pub id: String,
        #[serde(rename = "location")]
        pub location: super::super::debugger::Location,
        #[serde(rename = "profile")]
        pub profile: super::Profile,
        #[serde(skip_serializing_if = "Option::is_none")]
        #[serde(default)]
        #[serde(rename = "title")]
        pub title: Option<String>,
    }
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
    pub struct ConsoleProfileStartedEvent {
        pub params: ConsoleProfileStartedEventParams,
    }
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
    pub struct ConsoleProfileStartedEventParams {
        #[serde(default)]
        #[serde(rename = "id")]
        pub id: String,
        #[serde(rename = "location")]
        pub location: super::super::debugger::Location,
        #[serde(skip_serializing_if = "Option::is_none")]
        #[serde(default)]
        #[serde(rename = "title")]
        pub title: Option<String>,
    }
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
    pub struct PreciseCoverageDeltaUpdateEvent {
        pub params: PreciseCoverageDeltaUpdateEventParams,
    }
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
    pub struct PreciseCoverageDeltaUpdateEventParams {
        #[serde(default)]
        #[serde(rename = "timestamp")]
        pub timestamp: JsFloat,
        #[serde(default)]
        #[serde(rename = "occasion")]
        pub occasion: String,
        #[serde(rename = "result")]
        pub result: Vec<super::ScriptCoverage>,
    }
}
