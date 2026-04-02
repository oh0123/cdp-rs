// Auto-generated from Chrome at version 146.0.7680.165 domain: Log
#![allow(dead_code)]
use super::network;
use super::runtime;
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
pub enum LogEntrySource {
    #[serde(rename = "xml")]
    Xml,
    #[serde(rename = "javascript")]
    Javascript,
    #[serde(rename = "network")]
    Network,
    #[serde(rename = "storage")]
    Storage,
    #[serde(rename = "appcache")]
    Appcache,
    #[serde(rename = "rendering")]
    Rendering,
    #[serde(rename = "security")]
    Security,
    #[serde(rename = "deprecation")]
    Deprecation,
    #[serde(rename = "worker")]
    Worker,
    #[serde(rename = "violation")]
    Violation,
    #[serde(rename = "intervention")]
    Intervention,
    #[serde(rename = "recommendation")]
    Recommendation,
    #[serde(rename = "other")]
    Other,
}
#[allow(deprecated)]
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub enum LogEntryLevel {
    #[serde(rename = "verbose")]
    Verbose,
    #[serde(rename = "info")]
    Info,
    #[serde(rename = "warning")]
    Warning,
    #[serde(rename = "error")]
    Error,
}
#[allow(deprecated)]
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub enum LogEntryCategory {
    #[serde(rename = "cors")]
    Cors,
}
#[allow(deprecated)]
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub enum ViolationSettingName {
    #[serde(rename = "longTask")]
    LongTask,
    #[serde(rename = "longLayout")]
    LongLayout,
    #[serde(rename = "blockedEvent")]
    BlockedEvent,
    #[serde(rename = "blockedParser")]
    BlockedParser,
    #[serde(rename = "discouragedAPIUse")]
    DiscouragedApiUse,
    #[serde(rename = "handler")]
    Handler,
    #[serde(rename = "recurringHandler")]
    RecurringHandler,
}
#[allow(deprecated)]
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "camelCase")]
#[doc = "Log entry."]
pub struct LogEntry {
    #[doc = "Log entry source."]
    pub source: LogEntrySource,
    #[doc = "Log entry severity."]
    pub level: LogEntryLevel,
    #[serde(default)]
    #[doc = "Logged text."]
    pub text: String,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub category: Option<LogEntryCategory>,
    #[doc = "Timestamp when this entry was added."]
    pub timestamp: runtime::Timestamp,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[doc = "URL of the resource if known."]
    pub url: Option<String>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[doc = "Line number in the resource."]
    pub line_number: Option<JsUInt>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[doc = "JavaScript stack trace."]
    pub stack_trace: Option<runtime::StackTrace>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[doc = "Identifier of the network request associated with this entry."]
    pub network_request_id: Option<network::RequestId>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[doc = "Identifier of the worker associated with this entry."]
    pub worker_id: Option<String>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[doc = "Call arguments."]
    pub args: Option<Vec<runtime::RemoteObject>>,
}
#[allow(deprecated)]
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "camelCase")]
#[doc = "Violation configuration setting."]
pub struct ViolationSetting {
    #[doc = "Violation type."]
    pub name: ViolationSettingName,
    #[serde(default)]
    #[doc = "Time threshold to trigger upon."]
    pub threshold: JsFloat,
}
#[allow(deprecated)]
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct Clear(pub Option<Json>);
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
#[doc = "start violation reporting."]
pub struct StartViolationsReport {
    #[doc = "Configuration for violations."]
    pub config: Vec<ViolationSetting>,
}
#[allow(deprecated)]
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct StopViolationsReport(pub Option<Json>);
#[allow(deprecated)]
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[doc = "Clears the log."]
pub struct ClearReturnObject(pub Option<Json>);
#[allow(deprecated)]
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[doc = "Disables log domain, prevents further log entries from being reported to the client."]
pub struct DisableReturnObject(pub Option<Json>);
#[allow(deprecated)]
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[doc = "Enables log domain, sends the entries collected so far to the client by means of the\n `entryAdded` notification."]
pub struct EnableReturnObject(pub Option<Json>);
#[allow(deprecated)]
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[doc = "start violation reporting."]
pub struct StartViolationsReportReturnObject(pub Option<Json>);
#[allow(deprecated)]
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[doc = "Stop violation reporting."]
pub struct StopViolationsReportReturnObject(pub Option<Json>);
#[allow(deprecated)]
impl Method for Clear {
    const NAME: &'static str = "Log.clear";
    type ReturnObject = ClearReturnObject;
}
#[allow(deprecated)]
impl Method for Disable {
    const NAME: &'static str = "Log.disable";
    type ReturnObject = DisableReturnObject;
}
#[allow(deprecated)]
impl Method for Enable {
    const NAME: &'static str = "Log.enable";
    type ReturnObject = EnableReturnObject;
}
#[allow(deprecated)]
impl Method for StartViolationsReport {
    const NAME: &'static str = "Log.startViolationsReport";
    type ReturnObject = StartViolationsReportReturnObject;
}
#[allow(deprecated)]
impl Method for StopViolationsReport {
    const NAME: &'static str = "Log.stopViolationsReport";
    type ReturnObject = StopViolationsReportReturnObject;
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
    pub struct EntryAddedEvent {
        pub params: EntryAddedEventParams,
    }
    #[allow(deprecated)]
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
    #[serde(rename_all = "camelCase")]
    pub struct EntryAddedEventParams {
        #[doc = "The entry."]
        pub entry: super::LogEntry,
    }
}
