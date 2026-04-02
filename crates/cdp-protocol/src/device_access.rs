// Auto-generated from Chrome at version 146.0.7680.165 domain: DeviceAccess
#![allow(dead_code)]
#[allow(unused_imports)]
use super::types::*;
#[allow(unused_imports)]
use derive_builder::Builder;
#[allow(unused_imports)]
use serde::{Deserialize, Serialize};
#[allow(unused_imports)]
use serde_json::Value as Json;
pub type RequestId = String;
pub type DeviceId = String;
#[allow(deprecated)]
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "camelCase")]
#[doc = "Device information displayed in a user prompt to select a device."]
pub struct PromptDevice {
    pub id: DeviceId,
    #[serde(default)]
    #[doc = "Display name as it appears in a device request user prompt."]
    pub name: String,
}
#[allow(deprecated)]
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct Enable(pub Option<Json>);
#[allow(deprecated)]
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct Disable(pub Option<Json>);
#[allow(deprecated)]
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "camelCase")]
#[doc = "Select a device in response to a DeviceAccess.deviceRequestPrompted event."]
pub struct SelectPrompt {
    pub id: RequestId,
    pub device_id: DeviceId,
}
#[allow(deprecated)]
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "camelCase")]
#[doc = "Cancel a prompt in response to a DeviceAccess.deviceRequestPrompted event."]
pub struct CancelPrompt {
    pub id: RequestId,
}
#[allow(deprecated)]
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[doc = "Enable events in this domain."]
pub struct EnableReturnObject(pub Option<Json>);
#[allow(deprecated)]
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[doc = "Disable events in this domain."]
pub struct DisableReturnObject(pub Option<Json>);
#[allow(deprecated)]
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[doc = "Select a device in response to a DeviceAccess.deviceRequestPrompted event."]
pub struct SelectPromptReturnObject(pub Option<Json>);
#[allow(deprecated)]
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[doc = "Cancel a prompt in response to a DeviceAccess.deviceRequestPrompted event."]
pub struct CancelPromptReturnObject(pub Option<Json>);
#[allow(deprecated)]
impl Method for Enable {
    const NAME: &'static str = "DeviceAccess.enable";
    type ReturnObject = EnableReturnObject;
}
#[allow(deprecated)]
impl Method for Disable {
    const NAME: &'static str = "DeviceAccess.disable";
    type ReturnObject = DisableReturnObject;
}
#[allow(deprecated)]
impl Method for SelectPrompt {
    const NAME: &'static str = "DeviceAccess.selectPrompt";
    type ReturnObject = SelectPromptReturnObject;
}
#[allow(deprecated)]
impl Method for CancelPrompt {
    const NAME: &'static str = "DeviceAccess.cancelPrompt";
    type ReturnObject = CancelPromptReturnObject;
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
    pub struct DeviceRequestPromptedEvent {
        pub params: DeviceRequestPromptedEventParams,
    }
    #[allow(deprecated)]
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
    #[serde(rename_all = "camelCase")]
    pub struct DeviceRequestPromptedEventParams {
        pub id: super::RequestId,
        pub devices: Vec<super::PromptDevice>,
    }
}
