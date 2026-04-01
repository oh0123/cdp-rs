// Auto-generated from Chrome at version 146.0.7680.165 domain: DeviceAccess
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
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct Enable(pub Option<Json>);
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct Disable(pub Option<Json>);
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "camelCase")]
#[doc = "Select a device in response to a DeviceAccess.deviceRequestPrompted event."]
pub struct SelectPrompt {
    pub id: RequestId,
    pub device_id: DeviceId,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "camelCase")]
#[doc = "Cancel a prompt in response to a DeviceAccess.deviceRequestPrompted event."]
pub struct CancelPrompt {
    pub id: RequestId,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[doc = "Enable events in this domain."]
pub struct EnableReturnObject(pub Option<Json>);
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[doc = "Disable events in this domain."]
pub struct DisableReturnObject(pub Option<Json>);
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[doc = "Select a device in response to a DeviceAccess.deviceRequestPrompted event."]
pub struct SelectPromptReturnObject(pub Option<Json>);
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[doc = "Cancel a prompt in response to a DeviceAccess.deviceRequestPrompted event."]
pub struct CancelPromptReturnObject(pub Option<Json>);
impl Method for Enable {
    const NAME: &'static str = "DeviceAccess.enable";
    type ReturnObject = EnableReturnObject;
}
impl Method for Disable {
    const NAME: &'static str = "DeviceAccess.disable";
    type ReturnObject = DisableReturnObject;
}
impl Method for SelectPrompt {
    const NAME: &'static str = "DeviceAccess.selectPrompt";
    type ReturnObject = SelectPromptReturnObject;
}
impl Method for CancelPrompt {
    const NAME: &'static str = "DeviceAccess.cancelPrompt";
    type ReturnObject = CancelPromptReturnObject;
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
    pub struct DeviceRequestPromptedEvent {
        pub params: DeviceRequestPromptedEventParams,
    }
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
    pub struct DeviceRequestPromptedEventParams {
        pub id: super::RequestId,
        pub devices: Vec<super::PromptDevice>,
    }
}
