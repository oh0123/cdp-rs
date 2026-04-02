// Auto-generated from Chrome at version 146.0.7680.165 domain: Memory
#![allow(dead_code)]
#[allow(unused_imports)]
use super::types::*;
#[allow(unused_imports)]
use derive_builder::Builder;
#[allow(unused_imports)]
use serde::{Deserialize, Serialize};
#[allow(unused_imports)]
use serde_json::Value as Json;
#[allow(deprecated)]
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub enum PressureLevel {
    #[serde(rename = "moderate")]
    Moderate,
    #[serde(rename = "critical")]
    Critical,
}
#[allow(deprecated)]
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "camelCase")]
#[doc = "Heap profile sample."]
pub struct SamplingProfileNode {
    #[serde(default)]
    #[doc = "Size of the sampled allocation."]
    pub size: JsFloat,
    #[serde(default)]
    #[doc = "Total bytes attributed to this sample."]
    pub total: JsFloat,
    #[serde(default)]
    #[doc = "Execution stack at the point of allocation."]
    pub stack: Vec<String>,
}
#[allow(deprecated)]
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "camelCase")]
#[doc = "Array of heap profile samples."]
pub struct SamplingProfile {
    pub samples: Vec<SamplingProfileNode>,
    pub modules: Vec<Module>,
}
#[allow(deprecated)]
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "camelCase")]
#[doc = "Executable module information"]
pub struct Module {
    #[serde(default)]
    #[doc = "Name of the module."]
    pub name: String,
    #[serde(default)]
    #[doc = "UUID of the module."]
    pub uuid: String,
    #[serde(default)]
    #[doc = "Base address where the module is loaded into memory. Encoded as a decimal\n or hexadecimal (0x prefixed) string."]
    pub base_address: String,
    #[serde(default)]
    #[doc = "Size of the module in bytes."]
    pub size: JsFloat,
}
#[allow(deprecated)]
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "camelCase")]
#[doc = "DOM object counter data."]
pub struct DomCounter {
    #[serde(default)]
    #[doc = "Object name. Note: object names should be presumed volatile and clients should not expect\n the returned names to be consistent across runs."]
    pub name: String,
    #[serde(default)]
    #[doc = "Object count."]
    pub count: JsUInt,
}
#[allow(deprecated)]
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct GetDOMCounters(pub Option<Json>);
#[allow(deprecated)]
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct GetDOMCountersForLeakDetection(pub Option<Json>);
#[allow(deprecated)]
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct PrepareForLeakDetection(pub Option<Json>);
#[allow(deprecated)]
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct ForciblyPurgeJavaScriptMemory(pub Option<Json>);
#[allow(deprecated)]
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "camelCase")]
#[doc = "Enable/disable suppressing memory pressure notifications in all processes."]
pub struct SetPressureNotificationsSuppressed {
    #[serde(default)]
    #[doc = "If true, memory pressure notifications will be suppressed."]
    pub suppressed: bool,
}
#[allow(deprecated)]
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "camelCase")]
#[doc = "Simulate a memory pressure notification in all processes."]
pub struct SimulatePressureNotification {
    #[doc = "Memory pressure level of the notification."]
    pub level: PressureLevel,
}
#[allow(deprecated)]
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "camelCase")]
#[doc = "Start collecting native memory profile."]
pub struct StartSampling {
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[doc = "Average number of bytes between samples."]
    pub sampling_interval: Option<JsUInt>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[doc = "Do not randomize intervals between samples."]
    pub suppress_randomness: Option<bool>,
}
#[allow(deprecated)]
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct StopSampling(pub Option<Json>);
#[allow(deprecated)]
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct GetAllTimeSamplingProfile(pub Option<Json>);
#[allow(deprecated)]
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct GetBrowserSamplingProfile(pub Option<Json>);
#[allow(deprecated)]
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct GetSamplingProfile(pub Option<Json>);
#[allow(deprecated)]
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[serde(rename_all = "camelCase")]
#[doc = "Retruns current DOM object counters."]
pub struct GetDOMCountersReturnObject {
    #[serde(default)]
    pub documents: JsUInt,
    #[serde(default)]
    pub nodes: JsUInt,
    #[serde(default)]
    pub js_event_listeners: JsUInt,
}
#[allow(deprecated)]
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[serde(rename_all = "camelCase")]
#[doc = "Retruns DOM object counters after preparing renderer for leak detection."]
pub struct GetDOMCountersForLeakDetectionReturnObject {
    #[doc = "DOM object counters."]
    pub counters: Vec<DomCounter>,
}
#[allow(deprecated)]
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[doc = "Prepares for leak detection by terminating workers, stopping spellcheckers,\n dropping non-essential internal caches, running garbage collections, etc."]
pub struct PrepareForLeakDetectionReturnObject(pub Option<Json>);
#[allow(deprecated)]
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[doc = "Simulate OomIntervention by purging V8 memory."]
pub struct ForciblyPurgeJavaScriptMemoryReturnObject(pub Option<Json>);
#[allow(deprecated)]
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[doc = "Enable/disable suppressing memory pressure notifications in all processes."]
pub struct SetPressureNotificationsSuppressedReturnObject(pub Option<Json>);
#[allow(deprecated)]
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[doc = "Simulate a memory pressure notification in all processes."]
pub struct SimulatePressureNotificationReturnObject(pub Option<Json>);
#[allow(deprecated)]
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[doc = "Start collecting native memory profile."]
pub struct StartSamplingReturnObject(pub Option<Json>);
#[allow(deprecated)]
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[doc = "Stop collecting native memory profile."]
pub struct StopSamplingReturnObject(pub Option<Json>);
#[allow(deprecated)]
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[serde(rename_all = "camelCase")]
#[doc = "Retrieve native memory allocations profile\n collected since renderer process startup."]
pub struct GetAllTimeSamplingProfileReturnObject {
    pub profile: SamplingProfile,
}
#[allow(deprecated)]
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[serde(rename_all = "camelCase")]
#[doc = "Retrieve native memory allocations profile\n collected since browser process startup."]
pub struct GetBrowserSamplingProfileReturnObject {
    pub profile: SamplingProfile,
}
#[allow(deprecated)]
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[serde(rename_all = "camelCase")]
#[doc = "Retrieve native memory allocations profile collected since last\n `startSampling` call."]
pub struct GetSamplingProfileReturnObject {
    pub profile: SamplingProfile,
}
#[allow(deprecated)]
impl Method for GetDOMCounters {
    const NAME: &'static str = "Memory.getDOMCounters";
    type ReturnObject = GetDOMCountersReturnObject;
}
#[allow(deprecated)]
impl Method for GetDOMCountersForLeakDetection {
    const NAME: &'static str = "Memory.getDOMCountersForLeakDetection";
    type ReturnObject = GetDOMCountersForLeakDetectionReturnObject;
}
#[allow(deprecated)]
impl Method for PrepareForLeakDetection {
    const NAME: &'static str = "Memory.prepareForLeakDetection";
    type ReturnObject = PrepareForLeakDetectionReturnObject;
}
#[allow(deprecated)]
impl Method for ForciblyPurgeJavaScriptMemory {
    const NAME: &'static str = "Memory.forciblyPurgeJavaScriptMemory";
    type ReturnObject = ForciblyPurgeJavaScriptMemoryReturnObject;
}
#[allow(deprecated)]
impl Method for SetPressureNotificationsSuppressed {
    const NAME: &'static str = "Memory.setPressureNotificationsSuppressed";
    type ReturnObject = SetPressureNotificationsSuppressedReturnObject;
}
#[allow(deprecated)]
impl Method for SimulatePressureNotification {
    const NAME: &'static str = "Memory.simulatePressureNotification";
    type ReturnObject = SimulatePressureNotificationReturnObject;
}
#[allow(deprecated)]
impl Method for StartSampling {
    const NAME: &'static str = "Memory.startSampling";
    type ReturnObject = StartSamplingReturnObject;
}
#[allow(deprecated)]
impl Method for StopSampling {
    const NAME: &'static str = "Memory.stopSampling";
    type ReturnObject = StopSamplingReturnObject;
}
#[allow(deprecated)]
impl Method for GetAllTimeSamplingProfile {
    const NAME: &'static str = "Memory.getAllTimeSamplingProfile";
    type ReturnObject = GetAllTimeSamplingProfileReturnObject;
}
#[allow(deprecated)]
impl Method for GetBrowserSamplingProfile {
    const NAME: &'static str = "Memory.getBrowserSamplingProfile";
    type ReturnObject = GetBrowserSamplingProfileReturnObject;
}
#[allow(deprecated)]
impl Method for GetSamplingProfile {
    const NAME: &'static str = "Memory.getSamplingProfile";
    type ReturnObject = GetSamplingProfileReturnObject;
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
}
