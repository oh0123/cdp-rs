// Auto-generated from Chrome at version 146.0.7680.165 domain: DOMStorage
#[allow(unused_imports)]
use super::types::*;
#[allow(unused_imports)]
use derive_builder::Builder;
#[allow(unused_imports)]
use serde::{Deserialize, Serialize};
#[allow(unused_imports)]
use serde_json::Value as Json;
pub type SerializedStorageKey = String;
pub type Item = Vec<String>;
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "camelCase")]
#[doc = "DOM Storage identifier."]
pub struct StorageId {
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[doc = "Security origin for the storage."]
    pub security_origin: Option<String>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[doc = "Represents a key by which DOM Storage keys its CachedStorageAreas"]
    pub storage_key: Option<SerializedStorageKey>,
    #[serde(default)]
    #[doc = "Whether the storage is local storage (not session storage)."]
    pub is_local_storage: bool,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "camelCase")]
pub struct Clear {
    pub storage_id: StorageId,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct Disable(pub Option<Json>);
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct Enable(pub Option<Json>);
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "camelCase")]
pub struct GetDOMStorageItems {
    pub storage_id: StorageId,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "camelCase")]
pub struct RemoveDOMStorageItem {
    pub storage_id: StorageId,
    #[serde(default)]
    pub key: String,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "camelCase")]
pub struct SetDOMStorageItem {
    pub storage_id: StorageId,
    #[serde(default)]
    pub key: String,
    #[serde(default)]
    pub value: String,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct ClearReturnObject(pub Option<Json>);
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[doc = "Disables storage tracking, prevents storage events from being sent to the client."]
pub struct DisableReturnObject(pub Option<Json>);
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[doc = "Enables storage tracking, storage events will now be delivered to the client."]
pub struct EnableReturnObject(pub Option<Json>);
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[serde(rename_all = "camelCase")]
pub struct GetDOMStorageItemsReturnObject {
    pub entries: Vec<Item>,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct RemoveDOMStorageItemReturnObject(pub Option<Json>);
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct SetDOMStorageItemReturnObject(pub Option<Json>);
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
    use derive_builder::Builder;
    #[allow(unused_imports)]
    use serde::{Deserialize, Serialize};
    #[allow(unused_imports)]
    use serde_json::Value as Json;
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
    pub struct DomStorageItemAddedEvent {
        pub params: DomStorageItemAddedEventParams,
    }
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
    #[serde(rename_all = "camelCase")]
    pub struct DomStorageItemAddedEventParams {
        pub storage_id: super::StorageId,
        #[serde(default)]
        pub key: String,
        #[serde(default)]
        pub new_value: String,
    }
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
    pub struct DomStorageItemRemovedEvent {
        pub params: DomStorageItemRemovedEventParams,
    }
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
    #[serde(rename_all = "camelCase")]
    pub struct DomStorageItemRemovedEventParams {
        pub storage_id: super::StorageId,
        #[serde(default)]
        pub key: String,
    }
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
    pub struct DomStorageItemUpdatedEvent {
        pub params: DomStorageItemUpdatedEventParams,
    }
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
    #[serde(rename_all = "camelCase")]
    pub struct DomStorageItemUpdatedEventParams {
        pub storage_id: super::StorageId,
        #[serde(default)]
        pub key: String,
        #[serde(default)]
        pub old_value: String,
        #[serde(default)]
        pub new_value: String,
    }
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
    pub struct DomStorageItemsClearedEvent {
        pub params: DomStorageItemsClearedEventParams,
    }
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
    #[serde(rename_all = "camelCase")]
    pub struct DomStorageItemsClearedEventParams {
        pub storage_id: super::StorageId,
    }
}
