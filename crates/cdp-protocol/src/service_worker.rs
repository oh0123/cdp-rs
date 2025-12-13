// Auto-generated from Chrome at version 143.0.7499.110 domain: ServiceWorker
use super::target;
#[allow(unused_imports)]
use super::types::*;
#[allow(unused_imports)]
use serde::{Deserialize, Serialize};
#[allow(unused_imports)]
use serde_json::Value as Json;
pub type RegistrationId = String;
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub enum ServiceWorkerVersionRunningStatus {
    #[serde(rename = "stopped")]
    Stopped,
    #[serde(rename = "starting")]
    Starting,
    #[serde(rename = "running")]
    Running,
    #[serde(rename = "stopping")]
    Stopping,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub enum ServiceWorkerVersionStatus {
    #[serde(rename = "new")]
    New,
    #[serde(rename = "installing")]
    Installing,
    #[serde(rename = "installed")]
    Installed,
    #[serde(rename = "activating")]
    Activating,
    #[serde(rename = "activated")]
    Activated,
    #[serde(rename = "redundant")]
    Redundant,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct ServiceWorkerRegistration {
    #[serde(rename = "registrationId")]
    pub registration_id: RegistrationId,
    #[serde(default)]
    #[serde(rename = "scopeURL")]
    pub scope_url: String,
    #[serde(default)]
    #[serde(rename = "isDeleted")]
    pub is_deleted: bool,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct ServiceWorkerVersion {
    #[serde(default)]
    #[serde(rename = "versionId")]
    pub version_id: String,
    #[serde(rename = "registrationId")]
    pub registration_id: RegistrationId,
    #[serde(default)]
    #[serde(rename = "scriptURL")]
    pub script_url: String,
    #[serde(rename = "runningStatus")]
    pub running_status: ServiceWorkerVersionRunningStatus,
    #[serde(rename = "status")]
    pub status: ServiceWorkerVersionStatus,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(rename = "scriptLastModified")]
    pub script_last_modified: Option<JsFloat>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(rename = "scriptResponseTime")]
    pub script_response_time: Option<JsFloat>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "controlledClients")]
    pub controlled_clients: Option<Vec<target::TargetId>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "targetId")]
    pub target_id: Option<target::TargetId>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(rename = "routerRules")]
    pub router_rules: Option<String>,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct ServiceWorkerErrorMessage {
    #[serde(default)]
    #[serde(rename = "errorMessage")]
    pub error_message: String,
    #[serde(rename = "registrationId")]
    pub registration_id: RegistrationId,
    #[serde(default)]
    #[serde(rename = "versionId")]
    pub version_id: String,
    #[serde(default)]
    #[serde(rename = "sourceURL")]
    pub source_url: String,
    #[serde(default)]
    #[serde(rename = "lineNumber")]
    pub line_number: JsUInt,
    #[serde(default)]
    #[serde(rename = "columnNumber")]
    pub column_number: JsUInt,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct DeliverPushMessage {
    #[serde(default)]
    #[serde(rename = "origin")]
    pub origin: String,
    #[serde(rename = "registrationId")]
    pub registration_id: RegistrationId,
    #[serde(default)]
    #[serde(rename = "data")]
    pub data: String,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Disable(pub Option<serde_json::Value>);
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct DispatchSyncEvent {
    #[serde(default)]
    #[serde(rename = "origin")]
    pub origin: String,
    #[serde(rename = "registrationId")]
    pub registration_id: RegistrationId,
    #[serde(default)]
    #[serde(rename = "tag")]
    pub tag: String,
    #[serde(default)]
    #[serde(rename = "lastChance")]
    pub last_chance: bool,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct DispatchPeriodicSyncEvent {
    #[serde(default)]
    #[serde(rename = "origin")]
    pub origin: String,
    #[serde(rename = "registrationId")]
    pub registration_id: RegistrationId,
    #[serde(default)]
    #[serde(rename = "tag")]
    pub tag: String,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Enable(pub Option<serde_json::Value>);
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct SetForceUpdateOnPageLoad {
    #[serde(default)]
    #[serde(rename = "forceUpdateOnPageLoad")]
    pub force_update_on_page_load: bool,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct SkipWaiting {
    #[serde(default)]
    #[serde(rename = "scopeURL")]
    pub scope_url: String,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct StartWorker {
    #[serde(default)]
    #[serde(rename = "scopeURL")]
    pub scope_url: String,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct StopAllWorkers(pub Option<serde_json::Value>);
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct StopWorker {
    #[serde(default)]
    #[serde(rename = "versionId")]
    pub version_id: String,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct Unregister {
    #[serde(default)]
    #[serde(rename = "scopeURL")]
    pub scope_url: String,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct UpdateRegistration {
    #[serde(default)]
    #[serde(rename = "scopeURL")]
    pub scope_url: String,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct DeliverPushMessageReturnObject {}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct DisableReturnObject {}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct DispatchSyncEventReturnObject {}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct DispatchPeriodicSyncEventReturnObject {}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct EnableReturnObject {}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct SetForceUpdateOnPageLoadReturnObject {}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct SkipWaitingReturnObject {}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct StartWorkerReturnObject {}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct StopAllWorkersReturnObject {}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct StopWorkerReturnObject {}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct UnregisterReturnObject {}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct UpdateRegistrationReturnObject {}
impl Method for DeliverPushMessage {
    const NAME: &'static str = "ServiceWorker.deliverPushMessage";
    type ReturnObject = DeliverPushMessageReturnObject;
}
impl Method for Disable {
    const NAME: &'static str = "ServiceWorker.disable";
    type ReturnObject = DisableReturnObject;
}
impl Method for DispatchSyncEvent {
    const NAME: &'static str = "ServiceWorker.dispatchSyncEvent";
    type ReturnObject = DispatchSyncEventReturnObject;
}
impl Method for DispatchPeriodicSyncEvent {
    const NAME: &'static str = "ServiceWorker.dispatchPeriodicSyncEvent";
    type ReturnObject = DispatchPeriodicSyncEventReturnObject;
}
impl Method for Enable {
    const NAME: &'static str = "ServiceWorker.enable";
    type ReturnObject = EnableReturnObject;
}
impl Method for SetForceUpdateOnPageLoad {
    const NAME: &'static str = "ServiceWorker.setForceUpdateOnPageLoad";
    type ReturnObject = SetForceUpdateOnPageLoadReturnObject;
}
impl Method for SkipWaiting {
    const NAME: &'static str = "ServiceWorker.skipWaiting";
    type ReturnObject = SkipWaitingReturnObject;
}
impl Method for StartWorker {
    const NAME: &'static str = "ServiceWorker.startWorker";
    type ReturnObject = StartWorkerReturnObject;
}
impl Method for StopAllWorkers {
    const NAME: &'static str = "ServiceWorker.stopAllWorkers";
    type ReturnObject = StopAllWorkersReturnObject;
}
impl Method for StopWorker {
    const NAME: &'static str = "ServiceWorker.stopWorker";
    type ReturnObject = StopWorkerReturnObject;
}
impl Method for Unregister {
    const NAME: &'static str = "ServiceWorker.unregister";
    type ReturnObject = UnregisterReturnObject;
}
impl Method for UpdateRegistration {
    const NAME: &'static str = "ServiceWorker.updateRegistration";
    type ReturnObject = UpdateRegistrationReturnObject;
}
pub mod events {
    #[allow(unused_imports)]
    use super::super::types::*;
    #[allow(unused_imports)]
    use serde::{Deserialize, Serialize};
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
    pub struct WorkerErrorReportedEvent {
        pub params: WorkerErrorReportedEventParams,
    }
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
    pub struct WorkerErrorReportedEventParams {
        #[serde(rename = "errorMessage")]
        pub error_message: super::ServiceWorkerErrorMessage,
    }
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
    pub struct WorkerRegistrationUpdatedEvent {
        pub params: WorkerRegistrationUpdatedEventParams,
    }
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
    pub struct WorkerRegistrationUpdatedEventParams {
        #[serde(rename = "registrations")]
        pub registrations: Vec<super::ServiceWorkerRegistration>,
    }
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
    pub struct WorkerVersionUpdatedEvent {
        pub params: WorkerVersionUpdatedEventParams,
    }
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
    pub struct WorkerVersionUpdatedEventParams {
        #[serde(rename = "versions")]
        pub versions: Vec<super::ServiceWorkerVersion>,
    }
}
