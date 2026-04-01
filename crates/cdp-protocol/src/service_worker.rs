// Auto-generated from Chrome at version 146.0.7680.165 domain: ServiceWorker
use super::target;
#[allow(unused_imports)]
use super::types::*;
#[allow(unused_imports)]
use derive_builder::Builder;
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
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "camelCase")]
#[doc = "ServiceWorker registration."]
pub struct ServiceWorkerRegistration {
    pub registration_id: RegistrationId,
    #[serde(default)]
    #[serde(rename = "scopeURL")]
    pub scope_url: String,
    #[serde(default)]
    pub is_deleted: bool,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "camelCase")]
#[doc = "ServiceWorker version."]
pub struct ServiceWorkerVersion {
    #[serde(default)]
    pub version_id: String,
    pub registration_id: RegistrationId,
    #[serde(default)]
    #[serde(rename = "scriptURL")]
    pub script_url: String,
    pub running_status: ServiceWorkerVersionRunningStatus,
    pub status: ServiceWorkerVersionStatus,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[doc = "The Last-Modified header value of the main script."]
    pub script_last_modified: Option<JsFloat>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[doc = "The time at which the response headers of the main script were received from the server.\n For cached script it is the last time the cache entry was validated."]
    pub script_response_time: Option<JsFloat>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub controlled_clients: Option<Vec<target::TargetId>>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_id: Option<target::TargetId>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub router_rules: Option<String>,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "camelCase")]
#[doc = "ServiceWorker error message."]
pub struct ServiceWorkerErrorMessage {
    #[serde(default)]
    pub error_message: String,
    pub registration_id: RegistrationId,
    #[serde(default)]
    pub version_id: String,
    #[serde(default)]
    #[serde(rename = "sourceURL")]
    pub source_url: String,
    #[serde(default)]
    pub line_number: JsUInt,
    #[serde(default)]
    pub column_number: JsUInt,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "camelCase")]
pub struct DeliverPushMessage {
    #[serde(default)]
    pub origin: String,
    pub registration_id: RegistrationId,
    #[serde(default)]
    pub data: String,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct Disable(pub Option<Json>);
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "camelCase")]
pub struct DispatchSyncEvent {
    #[serde(default)]
    pub origin: String,
    pub registration_id: RegistrationId,
    #[serde(default)]
    pub tag: String,
    #[serde(default)]
    pub last_chance: bool,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "camelCase")]
pub struct DispatchPeriodicSyncEvent {
    #[serde(default)]
    pub origin: String,
    pub registration_id: RegistrationId,
    #[serde(default)]
    pub tag: String,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct Enable(pub Option<Json>);
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "camelCase")]
pub struct SetForceUpdateOnPageLoad {
    #[serde(default)]
    pub force_update_on_page_load: bool,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "camelCase")]
pub struct SkipWaiting {
    #[serde(default)]
    #[serde(rename = "scopeURL")]
    pub scope_url: String,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "camelCase")]
pub struct StartWorker {
    #[serde(default)]
    #[serde(rename = "scopeURL")]
    pub scope_url: String,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct StopAllWorkers(pub Option<Json>);
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "camelCase")]
pub struct StopWorker {
    #[serde(default)]
    pub version_id: String,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "camelCase")]
pub struct Unregister {
    #[serde(default)]
    #[serde(rename = "scopeURL")]
    pub scope_url: String,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "camelCase")]
pub struct UpdateRegistration {
    #[serde(default)]
    #[serde(rename = "scopeURL")]
    pub scope_url: String,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct DeliverPushMessageReturnObject(pub Option<Json>);
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct DisableReturnObject(pub Option<Json>);
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct DispatchSyncEventReturnObject(pub Option<Json>);
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct DispatchPeriodicSyncEventReturnObject(pub Option<Json>);
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct EnableReturnObject(pub Option<Json>);
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct SetForceUpdateOnPageLoadReturnObject(pub Option<Json>);
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct SkipWaitingReturnObject(pub Option<Json>);
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct StartWorkerReturnObject(pub Option<Json>);
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct StopAllWorkersReturnObject(pub Option<Json>);
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct StopWorkerReturnObject(pub Option<Json>);
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct UnregisterReturnObject(pub Option<Json>);
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct UpdateRegistrationReturnObject(pub Option<Json>);
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
    use derive_builder::Builder;
    #[allow(unused_imports)]
    use serde::{Deserialize, Serialize};
    #[allow(unused_imports)]
    use serde_json::Value as Json;
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
    pub struct WorkerErrorReportedEvent {
        pub params: WorkerErrorReportedEventParams,
    }
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
    pub struct WorkerErrorReportedEventParams {
        pub error_message: super::ServiceWorkerErrorMessage,
    }
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
    pub struct WorkerRegistrationUpdatedEvent {
        pub params: WorkerRegistrationUpdatedEventParams,
    }
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
    pub struct WorkerRegistrationUpdatedEventParams {
        pub registrations: Vec<super::ServiceWorkerRegistration>,
    }
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
    pub struct WorkerVersionUpdatedEvent {
        pub params: WorkerVersionUpdatedEventParams,
    }
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
    pub struct WorkerVersionUpdatedEventParams {
        pub versions: Vec<super::ServiceWorkerVersion>,
    }
}
