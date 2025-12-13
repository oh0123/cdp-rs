// Auto-generated from Chrome at version 143.0.7499.110 domain: Extensions
#[allow(unused_imports)]
use super::types::*;
#[allow(unused_imports)]
use serde::{Deserialize, Serialize};
#[allow(unused_imports)]
use serde_json::Value as Json;
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub enum StorageArea {
    #[serde(rename = "session")]
    Session,
    #[serde(rename = "local")]
    Local,
    #[serde(rename = "sync")]
    Sync,
    #[serde(rename = "managed")]
    Managed,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct LoadUnpacked {
    #[serde(default)]
    #[serde(rename = "path")]
    pub path: String,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct Uninstall {
    #[serde(default)]
    #[serde(rename = "id")]
    pub id: String,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct GetStorageItems {
    #[serde(default)]
    #[serde(rename = "id")]
    pub id: String,
    #[serde(rename = "storageArea")]
    pub storage_area: StorageArea,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(rename = "keys")]
    pub keys: Option<Vec<String>>,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct RemoveStorageItems {
    #[serde(default)]
    #[serde(rename = "id")]
    pub id: String,
    #[serde(rename = "storageArea")]
    pub storage_area: StorageArea,
    #[serde(default)]
    #[serde(rename = "keys")]
    pub keys: Vec<String>,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct ClearStorageItems {
    #[serde(default)]
    #[serde(rename = "id")]
    pub id: String,
    #[serde(rename = "storageArea")]
    pub storage_area: StorageArea,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct SetStorageItems {
    #[serde(default)]
    #[serde(rename = "id")]
    pub id: String,
    #[serde(rename = "storageArea")]
    pub storage_area: StorageArea,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct LoadUnpackedReturnObject {
    #[serde(default)]
    #[serde(rename = "id")]
    pub id: String,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct UninstallReturnObject {}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct GetStorageItemsReturnObject {}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct RemoveStorageItemsReturnObject {}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct ClearStorageItemsReturnObject {}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct SetStorageItemsReturnObject {}
impl Method for LoadUnpacked {
    const NAME: &'static str = "Extensions.loadUnpacked";
    type ReturnObject = LoadUnpackedReturnObject;
}
impl Method for Uninstall {
    const NAME: &'static str = "Extensions.uninstall";
    type ReturnObject = UninstallReturnObject;
}
impl Method for GetStorageItems {
    const NAME: &'static str = "Extensions.getStorageItems";
    type ReturnObject = GetStorageItemsReturnObject;
}
impl Method for RemoveStorageItems {
    const NAME: &'static str = "Extensions.removeStorageItems";
    type ReturnObject = RemoveStorageItemsReturnObject;
}
impl Method for ClearStorageItems {
    const NAME: &'static str = "Extensions.clearStorageItems";
    type ReturnObject = ClearStorageItemsReturnObject;
}
impl Method for SetStorageItems {
    const NAME: &'static str = "Extensions.setStorageItems";
    type ReturnObject = SetStorageItemsReturnObject;
}
pub mod events {
    #[allow(unused_imports)]
    use super::super::types::*;
    #[allow(unused_imports)]
    use serde::{Deserialize, Serialize};
}
