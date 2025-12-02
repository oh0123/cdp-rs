// Auto-generated from Chrome at version 140.0.7339.186 domain: BackgroundService
use super::network;
use super::service_worker;
#[allow(unused_imports)]
use super::types::*;
#[allow(unused_imports)]
use serde::{Deserialize, Serialize};
#[allow(unused_imports)]
use serde_json::Value as Json;
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub enum ServiceName {
    #[serde(rename = "backgroundFetch")]
    BackgroundFetch,
    #[serde(rename = "backgroundSync")]
    BackgroundSync,
    #[serde(rename = "pushMessaging")]
    PushMessaging,
    #[serde(rename = "notifications")]
    Notifications,
    #[serde(rename = "paymentHandler")]
    PaymentHandler,
    #[serde(rename = "periodicBackgroundSync")]
    PeriodicBackgroundSync,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct EventMetadata {
    #[serde(default)]
    #[serde(rename = "key")]
    pub key: String,
    #[serde(default)]
    #[serde(rename = "value")]
    pub value: String,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct BackgroundServiceEvent {
    #[serde(rename = "timestamp")]
    pub timestamp: network::TimeSinceEpoch,
    #[serde(default)]
    #[serde(rename = "origin")]
    pub origin: String,
    #[serde(rename = "serviceWorkerRegistrationId")]
    pub service_worker_registration_id: service_worker::RegistrationId,
    #[serde(rename = "service")]
    pub service: ServiceName,
    #[serde(default)]
    #[serde(rename = "eventName")]
    pub event_name: String,
    #[serde(default)]
    #[serde(rename = "instanceId")]
    pub instance_id: String,
    #[serde(rename = "eventMetadata")]
    pub event_metadata: Vec<EventMetadata>,
    #[serde(default)]
    #[serde(rename = "storageKey")]
    pub storage_key: String,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct StartObserving {
    #[serde(rename = "service")]
    pub service: ServiceName,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct StopObserving {
    #[serde(rename = "service")]
    pub service: ServiceName,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct SetRecording {
    #[serde(default)]
    #[serde(rename = "shouldRecord")]
    pub should_record: bool,
    #[serde(rename = "service")]
    pub service: ServiceName,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct ClearEvents {
    #[serde(rename = "service")]
    pub service: ServiceName,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct StartObservingReturnObject {}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct StopObservingReturnObject {}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct SetRecordingReturnObject {}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct ClearEventsReturnObject {}
impl Method for StartObserving {
    const NAME: &'static str = "BackgroundService.startObserving";
    type ReturnObject = StartObservingReturnObject;
}
impl Method for StopObserving {
    const NAME: &'static str = "BackgroundService.stopObserving";
    type ReturnObject = StopObservingReturnObject;
}
impl Method for SetRecording {
    const NAME: &'static str = "BackgroundService.setRecording";
    type ReturnObject = SetRecordingReturnObject;
}
impl Method for ClearEvents {
    const NAME: &'static str = "BackgroundService.clearEvents";
    type ReturnObject = ClearEventsReturnObject;
}
pub mod events {
    #[allow(unused_imports)]
    use super::super::types::*;
    #[allow(unused_imports)]
    use serde::{Deserialize, Serialize};
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
    pub struct RecordingStateChangedEvent {
        pub params: RecordingStateChangedEventParams,
    }
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
    pub struct RecordingStateChangedEventParams {
        #[serde(default)]
        #[serde(rename = "isRecording")]
        pub is_recording: bool,
        #[serde(rename = "service")]
        pub service: super::ServiceName,
    }
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
    pub struct BackgroundServiceEventReceivedEvent {
        pub params: BackgroundServiceEventReceivedEventParams,
    }
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
    pub struct BackgroundServiceEventReceivedEventParams {
        #[serde(rename = "backgroundServiceEvent")]
        pub background_service_event: super::BackgroundServiceEvent,
    }
}
