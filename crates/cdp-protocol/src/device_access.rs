// Auto-generated from Chrome at version 143.0.7499.110 domain: DeviceAccess
#[allow(unused_imports)]
use super::types::*;
#[allow(unused_imports)]
use serde::{Deserialize, Serialize};
#[allow(unused_imports)]
use serde_json::Value as Json;
pub type RequestId = String;
pub type DeviceId = String;
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct PromptDevice {
    #[serde(rename = "id")]
    pub id: DeviceId,
    #[serde(default)]
    #[serde(rename = "name")]
    pub name: String,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Enable(pub Option<serde_json::Value>);
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Disable(pub Option<serde_json::Value>);
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct SelectPrompt {
    #[serde(rename = "id")]
    pub id: RequestId,
    #[serde(rename = "deviceId")]
    pub device_id: DeviceId,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct CancelPrompt {
    #[serde(rename = "id")]
    pub id: RequestId,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct EnableReturnObject {}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct DisableReturnObject {}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct SelectPromptReturnObject {}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct CancelPromptReturnObject {}
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
    use serde::{Deserialize, Serialize};
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
    pub struct DeviceRequestPromptedEvent {
        pub params: DeviceRequestPromptedEventParams,
    }
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
    pub struct DeviceRequestPromptedEventParams {
        #[serde(rename = "id")]
        pub id: super::RequestId,
        #[serde(rename = "devices")]
        pub devices: Vec<super::PromptDevice>,
    }
}
