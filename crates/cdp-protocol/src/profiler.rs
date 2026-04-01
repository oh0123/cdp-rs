// Auto-generated from Chrome at version 146.0.7680.165 domain: Profiler
use super::runtime;
#[allow(unused_imports)]
use super::types::*;
#[allow(unused_imports)]
use derive_builder::Builder;
#[allow(unused_imports)]
use serde::{Deserialize, Serialize};
#[allow(unused_imports)]
use serde_json::Value as Json;
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "camelCase")]
#[doc = "Profile node. Holds callsite information, execution statistics and child nodes."]
pub struct ProfileNode {
    #[serde(default)]
    #[doc = "Unique id of the node."]
    pub id: JsUInt,
    #[doc = "Function location."]
    pub call_frame: runtime::CallFrame,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[doc = "Number of samples where this node was on top of the call stack."]
    pub hit_count: Option<JsUInt>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[doc = "Child node ids."]
    pub children: Option<Vec<JsUInt>>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[doc = "The reason of being not optimized. The function may be deoptimized or marked as don't\n optimize."]
    pub deopt_reason: Option<String>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[doc = "An array of source position ticks."]
    pub position_ticks: Option<Vec<PositionTickInfo>>,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "camelCase")]
#[doc = "Profile."]
pub struct Profile {
    #[doc = "The list of profile nodes. First item is the root node."]
    pub nodes: Vec<ProfileNode>,
    #[serde(default)]
    #[doc = "Profiling start timestamp in microseconds."]
    pub start_time: JsFloat,
    #[serde(default)]
    #[doc = "Profiling end timestamp in microseconds."]
    pub end_time: JsFloat,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[doc = "Ids of samples top nodes."]
    pub samples: Option<Vec<JsUInt>>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[doc = "Time intervals between adjacent samples in microseconds. The first delta is relative to the\n profile startTime."]
    pub time_deltas: Option<Vec<JsUInt>>,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "camelCase")]
#[doc = "Specifies a number of samples attributed to a certain source position."]
pub struct PositionTickInfo {
    #[serde(default)]
    #[doc = "Source line number (1-based)."]
    pub line: JsUInt,
    #[serde(default)]
    #[doc = "Number of samples attributed to the source line."]
    pub ticks: JsUInt,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "camelCase")]
#[doc = "Coverage data for a source range."]
pub struct CoverageRange {
    #[serde(default)]
    #[doc = "JavaScript script source offset for the range start."]
    pub start_offset: JsUInt,
    #[serde(default)]
    #[doc = "JavaScript script source offset for the range end."]
    pub end_offset: JsUInt,
    #[serde(default)]
    #[doc = "Collected execution count of the source range."]
    pub count: JsUInt,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "camelCase")]
#[doc = "Coverage data for a JavaScript function."]
pub struct FunctionCoverage {
    #[serde(default)]
    #[doc = "JavaScript function name."]
    pub function_name: String,
    #[doc = "Source ranges inside the function with coverage data."]
    pub ranges: Vec<CoverageRange>,
    #[serde(default)]
    #[doc = "Whether coverage data for this function has block granularity."]
    pub is_block_coverage: bool,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "camelCase")]
#[doc = "Coverage data for a JavaScript script."]
pub struct ScriptCoverage {
    #[doc = "JavaScript script id."]
    pub script_id: runtime::ScriptId,
    #[serde(default)]
    #[doc = "JavaScript script name or url."]
    pub url: String,
    #[doc = "Functions contained in the script that has coverage data."]
    pub functions: Vec<FunctionCoverage>,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct Disable(pub Option<Json>);
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct Enable(pub Option<Json>);
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct GetBestEffortCoverage(pub Option<Json>);
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "camelCase")]
#[doc = "Changes CPU profiler sampling interval. Must be called before CPU profiles recording started."]
pub struct SetSamplingInterval {
    #[serde(default)]
    #[doc = "New sampling interval in microseconds."]
    pub interval: JsUInt,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct Start(pub Option<Json>);
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "camelCase")]
#[doc = "Enable precise code coverage. Coverage data for JavaScript executed before enabling precise code\n coverage may be incomplete. Enabling prevents running optimized code and resets execution\n counters."]
pub struct StartPreciseCoverage {
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[doc = "Collect accurate call counts beyond simple 'covered' or 'not covered'."]
    pub call_count: Option<bool>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[doc = "Collect block-based coverage."]
    pub detailed: Option<bool>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[doc = "Allow the backend to send updates on its own initiative"]
    pub allow_triggered_updates: Option<bool>,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct Stop(pub Option<Json>);
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct StopPreciseCoverage(pub Option<Json>);
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct TakePreciseCoverage(pub Option<Json>);
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct DisableReturnObject(pub Option<Json>);
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct EnableReturnObject(pub Option<Json>);
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[serde(rename_all = "camelCase")]
#[doc = "Collect coverage data for the current isolate. The coverage data may be incomplete due to\n garbage collection."]
pub struct GetBestEffortCoverageReturnObject {
    #[doc = "Coverage data for the current isolate."]
    pub result: Vec<ScriptCoverage>,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[doc = "Changes CPU profiler sampling interval. Must be called before CPU profiles recording started."]
pub struct SetSamplingIntervalReturnObject(pub Option<Json>);
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct StartReturnObject(pub Option<Json>);
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[serde(rename_all = "camelCase")]
#[doc = "Enable precise code coverage. Coverage data for JavaScript executed before enabling precise code\n coverage may be incomplete. Enabling prevents running optimized code and resets execution\n counters."]
pub struct StartPreciseCoverageReturnObject {
    #[serde(default)]
    #[doc = "Monotonically increasing time (in seconds) when the coverage update was taken in the backend."]
    pub timestamp: JsFloat,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[serde(rename_all = "camelCase")]
pub struct StopReturnObject {
    #[doc = "Recorded profile."]
    pub profile: Profile,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[doc = "Disable precise code coverage. Disabling releases unnecessary execution count records and allows\n executing optimized code."]
pub struct StopPreciseCoverageReturnObject(pub Option<Json>);
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[serde(rename_all = "camelCase")]
#[doc = "Collect coverage data for the current isolate, and resets execution counters. Precise code\n coverage needs to have started."]
pub struct TakePreciseCoverageReturnObject {
    #[doc = "Coverage data for the current isolate."]
    pub result: Vec<ScriptCoverage>,
    #[serde(default)]
    #[doc = "Monotonically increasing time (in seconds) when the coverage update was taken in the backend."]
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
    use derive_builder::Builder;
    #[allow(unused_imports)]
    use serde::{Deserialize, Serialize};
    #[allow(unused_imports)]
    use serde_json::Value as Json;
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
    pub struct ConsoleProfileFinishedEvent {
        pub params: ConsoleProfileFinishedEventParams,
    }
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
    pub struct ConsoleProfileFinishedEventParams {
        #[serde(default)]
        pub id: String,
        #[doc = "Location of console.profileEnd()."]
        pub location: super::super::debugger::Location,
        pub profile: super::Profile,
        #[builder(default)]
        #[serde(skip_serializing_if = "Option::is_none")]
        #[serde(default)]
        #[doc = "Profile title passed as an argument to console.profile()."]
        pub title: Option<String>,
    }
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
    pub struct ConsoleProfileStartedEvent {
        pub params: ConsoleProfileStartedEventParams,
    }
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
    pub struct ConsoleProfileStartedEventParams {
        #[serde(default)]
        pub id: String,
        #[doc = "Location of console.profile()."]
        pub location: super::super::debugger::Location,
        #[builder(default)]
        #[serde(skip_serializing_if = "Option::is_none")]
        #[serde(default)]
        #[doc = "Profile title passed as an argument to console.profile()."]
        pub title: Option<String>,
    }
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
    pub struct PreciseCoverageDeltaUpdateEvent {
        pub params: PreciseCoverageDeltaUpdateEventParams,
    }
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
    pub struct PreciseCoverageDeltaUpdateEventParams {
        #[serde(default)]
        #[doc = "Monotonically increasing time (in seconds) when the coverage update was taken in the backend."]
        pub timestamp: JsFloat,
        #[serde(default)]
        #[doc = "Identifier for distinguishing coverage events."]
        pub occasion: String,
        #[doc = "Coverage data for the current isolate."]
        pub result: Vec<super::ScriptCoverage>,
    }
}
