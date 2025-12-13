// Auto-generated from Chrome at version 143.0.7499.110 domain: Memory
#[allow(unused_imports)]
use super::types::*;
#[allow(unused_imports)]
use serde::{Deserialize, Serialize};
#[allow(unused_imports)]
use serde_json::Value as Json;
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub enum PressureLevel {
    #[serde(rename = "moderate")]
    Moderate,
    #[serde(rename = "critical")]
    Critical,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct SamplingProfileNode {
    #[serde(default)]
    #[serde(rename = "size")]
    pub size: JsFloat,
    #[serde(default)]
    #[serde(rename = "total")]
    pub total: JsFloat,
    #[serde(default)]
    #[serde(rename = "stack")]
    pub stack: Vec<String>,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct SamplingProfile {
    #[serde(rename = "samples")]
    pub samples: Vec<SamplingProfileNode>,
    #[serde(rename = "modules")]
    pub modules: Vec<Module>,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct Module {
    #[serde(default)]
    #[serde(rename = "name")]
    pub name: String,
    #[serde(default)]
    #[serde(rename = "uuid")]
    pub uuid: String,
    #[serde(default)]
    #[serde(rename = "baseAddress")]
    pub base_address: String,
    #[serde(default)]
    #[serde(rename = "size")]
    pub size: JsFloat,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct DomCounter {
    #[serde(default)]
    #[serde(rename = "name")]
    pub name: String,
    #[serde(default)]
    #[serde(rename = "count")]
    pub count: JsUInt,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct GetDOMCounters(pub Option<serde_json::Value>);
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct GetDOMCountersForLeakDetection(pub Option<serde_json::Value>);
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct PrepareForLeakDetection(pub Option<serde_json::Value>);
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct ForciblyPurgeJavaScriptMemory(pub Option<serde_json::Value>);
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct SetPressureNotificationsSuppressed {
    #[serde(default)]
    #[serde(rename = "suppressed")]
    pub suppressed: bool,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct SimulatePressureNotification {
    #[serde(rename = "level")]
    pub level: PressureLevel,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct StartSampling {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(rename = "samplingInterval")]
    pub sampling_interval: Option<JsUInt>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(rename = "suppressRandomness")]
    pub suppress_randomness: Option<bool>,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct StopSampling(pub Option<serde_json::Value>);
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct GetAllTimeSamplingProfile(pub Option<serde_json::Value>);
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct GetBrowserSamplingProfile(pub Option<serde_json::Value>);
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct GetSamplingProfile(pub Option<serde_json::Value>);
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct GetDOMCountersReturnObject {
    #[serde(default)]
    #[serde(rename = "documents")]
    pub documents: JsUInt,
    #[serde(default)]
    #[serde(rename = "nodes")]
    pub nodes: JsUInt,
    #[serde(default)]
    #[serde(rename = "jsEventListeners")]
    pub js_event_listeners: JsUInt,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct GetDOMCountersForLeakDetectionReturnObject {
    #[serde(rename = "counters")]
    pub counters: Vec<DomCounter>,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct PrepareForLeakDetectionReturnObject {}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct ForciblyPurgeJavaScriptMemoryReturnObject {}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct SetPressureNotificationsSuppressedReturnObject {}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct SimulatePressureNotificationReturnObject {}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct StartSamplingReturnObject {}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct StopSamplingReturnObject {}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct GetAllTimeSamplingProfileReturnObject {
    #[serde(rename = "profile")]
    pub profile: SamplingProfile,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct GetBrowserSamplingProfileReturnObject {
    #[serde(rename = "profile")]
    pub profile: SamplingProfile,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct GetSamplingProfileReturnObject {
    #[serde(rename = "profile")]
    pub profile: SamplingProfile,
}
impl Method for GetDOMCounters {
    const NAME: &'static str = "Memory.getDOMCounters";
    type ReturnObject = GetDOMCountersReturnObject;
}
impl Method for GetDOMCountersForLeakDetection {
    const NAME: &'static str = "Memory.getDOMCountersForLeakDetection";
    type ReturnObject = GetDOMCountersForLeakDetectionReturnObject;
}
impl Method for PrepareForLeakDetection {
    const NAME: &'static str = "Memory.prepareForLeakDetection";
    type ReturnObject = PrepareForLeakDetectionReturnObject;
}
impl Method for ForciblyPurgeJavaScriptMemory {
    const NAME: &'static str = "Memory.forciblyPurgeJavaScriptMemory";
    type ReturnObject = ForciblyPurgeJavaScriptMemoryReturnObject;
}
impl Method for SetPressureNotificationsSuppressed {
    const NAME: &'static str = "Memory.setPressureNotificationsSuppressed";
    type ReturnObject = SetPressureNotificationsSuppressedReturnObject;
}
impl Method for SimulatePressureNotification {
    const NAME: &'static str = "Memory.simulatePressureNotification";
    type ReturnObject = SimulatePressureNotificationReturnObject;
}
impl Method for StartSampling {
    const NAME: &'static str = "Memory.startSampling";
    type ReturnObject = StartSamplingReturnObject;
}
impl Method for StopSampling {
    const NAME: &'static str = "Memory.stopSampling";
    type ReturnObject = StopSamplingReturnObject;
}
impl Method for GetAllTimeSamplingProfile {
    const NAME: &'static str = "Memory.getAllTimeSamplingProfile";
    type ReturnObject = GetAllTimeSamplingProfileReturnObject;
}
impl Method for GetBrowserSamplingProfile {
    const NAME: &'static str = "Memory.getBrowserSamplingProfile";
    type ReturnObject = GetBrowserSamplingProfileReturnObject;
}
impl Method for GetSamplingProfile {
    const NAME: &'static str = "Memory.getSamplingProfile";
    type ReturnObject = GetSamplingProfileReturnObject;
}
pub mod events {
    #[allow(unused_imports)]
    use super::super::types::*;
    #[allow(unused_imports)]
    use serde::{Deserialize, Serialize};
}
