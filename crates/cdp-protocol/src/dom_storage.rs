// Auto-generated from Chrome at version 140.0.7339.186 domain: DOMStorage
#[allow(unused_imports)]
use super::types::*;
#[allow(unused_imports)]
use serde::{Deserialize, Serialize};
#[allow(unused_imports)]
use serde_json::Value as Json;
pub type SerializedStorageKey = String;
pub type Item = Vec<String>;
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct StorageId {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(rename = "securityOrigin")]
    pub security_origin: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "storageKey")]
    pub storage_key: Option<SerializedStorageKey>,
    #[serde(default)]
    #[serde(rename = "isLocalStorage")]
    pub is_local_storage: bool,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct Clear {
    #[serde(rename = "storageId")]
    pub storage_id: StorageId,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Disable(pub Option<serde_json::Value>);
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Enable(pub Option<serde_json::Value>);
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct GetDOMStorageItems {
    #[serde(rename = "storageId")]
    pub storage_id: StorageId,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct RemoveDOMStorageItem {
    #[serde(rename = "storageId")]
    pub storage_id: StorageId,
    #[serde(default)]
    #[serde(rename = "key")]
    pub key: String,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct SetDOMStorageItem {
    #[serde(rename = "storageId")]
    pub storage_id: StorageId,
    #[serde(default)]
    #[serde(rename = "key")]
    pub key: String,
    #[serde(default)]
    #[serde(rename = "value")]
    pub value: String,
}
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
pub struct GetDOMStorageItemsReturnObject {
    #[serde(rename = "entries")]
    pub entries: Vec<Item>,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct RemoveDOMStorageItemReturnObject {}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct SetDOMStorageItemReturnObject {}
impl Method for Clear {
    const NAME: &'static str = "DOMStorage.clear";
    type ReturnObject = ClearReturnObject;
}
impl Method for Disable {
    const NAME: &'static str = "DOMStorage.disable";
    type ReturnObject = DisableReturnObject;
}
impl Method for Enable {
    const NAME: &'static str = "DOMStorage.enable";
    type ReturnObject = EnableReturnObject;
}
impl Method for GetDOMStorageItems {
    const NAME: &'static str = "DOMStorage.getDOMStorageItems";
    type ReturnObject = GetDOMStorageItemsReturnObject;
}
impl Method for RemoveDOMStorageItem {
    const NAME: &'static str = "DOMStorage.removeDOMStorageItem";
    type ReturnObject = RemoveDOMStorageItemReturnObject;
}
impl Method for SetDOMStorageItem {
    const NAME: &'static str = "DOMStorage.setDOMStorageItem";
    type ReturnObject = SetDOMStorageItemReturnObject;
}
pub mod events {
    #[allow(unused_imports)]
    use super::super::types::*;
    #[allow(unused_imports)]
    use serde::{Deserialize, Serialize};
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
    pub struct DomStorageItemAddedEvent {
        pub params: DomStorageItemAddedEventParams,
    }
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
    pub struct DomStorageItemAddedEventParams {
        #[serde(rename = "storageId")]
        pub storage_id: super::StorageId,
        #[serde(default)]
        #[serde(rename = "key")]
        pub key: String,
        #[serde(default)]
        #[serde(rename = "newValue")]
        pub new_value: String,
    }
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
    pub struct DomStorageItemRemovedEvent {
        pub params: DomStorageItemRemovedEventParams,
    }
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
    pub struct DomStorageItemRemovedEventParams {
        #[serde(rename = "storageId")]
        pub storage_id: super::StorageId,
        #[serde(default)]
        #[serde(rename = "key")]
        pub key: String,
    }
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
    pub struct DomStorageItemUpdatedEvent {
        pub params: DomStorageItemUpdatedEventParams,
    }
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
    pub struct DomStorageItemUpdatedEventParams {
        #[serde(rename = "storageId")]
        pub storage_id: super::StorageId,
        #[serde(default)]
        #[serde(rename = "key")]
        pub key: String,
        #[serde(default)]
        #[serde(rename = "oldValue")]
        pub old_value: String,
        #[serde(default)]
        #[serde(rename = "newValue")]
        pub new_value: String,
    }
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
    pub struct DomStorageItemsClearedEvent {
        pub params: DomStorageItemsClearedEventParams,
    }
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
    pub struct DomStorageItemsClearedEventParams {
        #[serde(rename = "storageId")]
        pub storage_id: super::StorageId,
    }
}
