// Auto-generated from Chrome at version 146.0.7680.165 domain: BackgroundService
use super::network;
use super::service_worker;
#[allow(unused_imports)]
use super::types::*;
#[allow(unused_imports)]
use derive_builder::Builder;
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
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "camelCase")]
#[doc = "A key-value pair for additional event information to pass along."]
pub struct EventMetadata {
    #[serde(default)]
    pub key: String,
    #[serde(default)]
    pub value: String,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "camelCase")]
pub struct BackgroundServiceEvent {
    #[doc = "Timestamp of the event (in seconds)."]
    pub timestamp: network::TimeSinceEpoch,
    #[serde(default)]
    #[doc = "The origin this event belongs to."]
    pub origin: String,
    #[doc = "The Service Worker ID that initiated the event."]
    pub service_worker_registration_id: service_worker::RegistrationId,
    #[doc = "The Background Service this event belongs to."]
    pub service: ServiceName,
    #[serde(default)]
    #[doc = "A description of the event."]
    pub event_name: String,
    #[serde(default)]
    #[doc = "An identifier that groups related events together."]
    pub instance_id: String,
    #[doc = "A list of event-specific information."]
    pub event_metadata: Vec<EventMetadata>,
    #[serde(default)]
    #[doc = "Storage key this event belongs to."]
    pub storage_key: String,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "camelCase")]
#[doc = "Enables event updates for the service."]
pub struct StartObserving {
    pub service: ServiceName,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "camelCase")]
#[doc = "Disables event updates for the service."]
pub struct StopObserving {
    pub service: ServiceName,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "camelCase")]
#[doc = "Set the recording state for the service."]
pub struct SetRecording {
    #[serde(default)]
    pub should_record: bool,
    pub service: ServiceName,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "camelCase")]
#[doc = "Clears all stored data for the service."]
pub struct ClearEvents {
    pub service: ServiceName,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[doc = "Enables event updates for the service."]
pub struct StartObservingReturnObject(pub Option<Json>);
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[doc = "Disables event updates for the service."]
pub struct StopObservingReturnObject(pub Option<Json>);
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[doc = "Set the recording state for the service."]
pub struct SetRecordingReturnObject(pub Option<Json>);
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[doc = "Clears all stored data for the service."]
pub struct ClearEventsReturnObject(pub Option<Json>);
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
    use derive_builder::Builder;
    #[allow(unused_imports)]
    use serde::{Deserialize, Serialize};
    #[allow(unused_imports)]
    use serde_json::Value as Json;
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
    pub struct RecordingStateChangedEvent {
        pub params: RecordingStateChangedEventParams,
    }
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
    #[serde(rename_all = "camelCase")]
    pub struct RecordingStateChangedEventParams {
        #[serde(default)]
        pub is_recording: bool,
        pub service: super::ServiceName,
    }
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
    pub struct BackgroundServiceEventReceivedEvent {
        pub params: BackgroundServiceEventReceivedEventParams,
    }
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
    #[serde(rename_all = "camelCase")]
    pub struct BackgroundServiceEventReceivedEventParams {
        pub background_service_event: super::BackgroundServiceEvent,
    }
}
