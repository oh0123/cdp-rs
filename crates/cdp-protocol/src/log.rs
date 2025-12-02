// Auto-generated from Chrome at version 140.0.7339.186 domain: Log
use super::network;
use super::runtime;
#[allow(unused_imports)]
use super::types::*;
#[allow(unused_imports)]
use serde::{Deserialize, Serialize};
#[allow(unused_imports)]
use serde_json::Value as Json;
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
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub enum LogEntryCategory {
    #[serde(rename = "cors")]
    Cors,
}
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
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct LogEntry {
    #[serde(rename = "source")]
    pub source: LogEntrySource,
    #[serde(rename = "level")]
    pub level: LogEntryLevel,
    #[serde(default)]
    #[serde(rename = "text")]
    pub text: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "category")]
    pub category: Option<LogEntryCategory>,
    #[serde(rename = "timestamp")]
    pub timestamp: runtime::Timestamp,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(rename = "url")]
    pub url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(rename = "lineNumber")]
    pub line_number: Option<JsUInt>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "stackTrace")]
    pub stack_trace: Option<runtime::StackTrace>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "networkRequestId")]
    pub network_request_id: Option<network::RequestId>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(rename = "workerId")]
    pub worker_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "args")]
    pub args: Option<Vec<runtime::RemoteObject>>,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct ViolationSetting {
    #[serde(rename = "name")]
    pub name: ViolationSettingName,
    #[serde(default)]
    #[serde(rename = "threshold")]
    pub threshold: JsFloat,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Clear(pub Option<serde_json::Value>);
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Disable(pub Option<serde_json::Value>);
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Enable(pub Option<serde_json::Value>);
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct StartViolationsReport {
    #[serde(rename = "config")]
    pub config: Vec<ViolationSetting>,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct StopViolationsReport(pub Option<serde_json::Value>);
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct ClearReturnObject {}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct DisableReturnObject {}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct EnableReturnObject {}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct StartViolationsReportReturnObject {}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct StopViolationsReportReturnObject {}
impl Method for Clear {
    const NAME: &'static str = "Log.clear";
    type ReturnObject = ClearReturnObject;
}
impl Method for Disable {
    const NAME: &'static str = "Log.disable";
    type ReturnObject = DisableReturnObject;
}
impl Method for Enable {
    const NAME: &'static str = "Log.enable";
    type ReturnObject = EnableReturnObject;
}
impl Method for StartViolationsReport {
    const NAME: &'static str = "Log.startViolationsReport";
    type ReturnObject = StartViolationsReportReturnObject;
}
impl Method for StopViolationsReport {
    const NAME: &'static str = "Log.stopViolationsReport";
    type ReturnObject = StopViolationsReportReturnObject;
}
pub mod events {
    #[allow(unused_imports)]
    use super::super::types::*;
    #[allow(unused_imports)]
    use serde::{Deserialize, Serialize};
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
    pub struct EntryAddedEvent {
        pub params: EntryAddedEventParams,
    }
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
    pub struct EntryAddedEventParams {
        #[serde(rename = "entry")]
        pub entry: super::LogEntry,
    }
}
