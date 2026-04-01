// Auto-generated from Chrome at version 146.0.7680.165 domain: Extensions
#[allow(unused_imports)]
use super::types::*;
#[allow(unused_imports)]
use derive_builder::Builder;
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
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "camelCase")]
#[doc = "Runs an extension default action.\n Available if the client is connected using the --remote-debugging-pipe\n flag and the --enable-unsafe-extension-debugging flag is set."]
pub struct TriggerAction {
    #[serde(default)]
    #[doc = "Extension id."]
    pub id: String,
    #[serde(default)]
    #[doc = "A tab target ID to trigger the default extension action on."]
    pub target_id: String,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "camelCase")]
#[doc = "Installs an unpacked extension from the filesystem similar to\n --load-extension CLI flags. Returns extension ID once the extension\n has been installed. Available if the client is connected using the\n --remote-debugging-pipe flag and the --enable-unsafe-extension-debugging\n flag is set."]
pub struct LoadUnpacked {
    #[serde(default)]
    #[doc = "Absolute file path."]
    pub path: String,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[doc = "Enable the extension in incognito"]
    pub enable_in_incognito: Option<bool>,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "camelCase")]
#[doc = "Uninstalls an unpacked extension (others not supported) from the profile.\n Available if the client is connected using the --remote-debugging-pipe flag\n and the --enable-unsafe-extension-debugging."]
pub struct Uninstall {
    #[serde(default)]
    #[doc = "Extension id."]
    pub id: String,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "camelCase")]
#[doc = "Gets data from extension storage in the given `storageArea`. If `keys` is\n specified, these are used to filter the result."]
pub struct GetStorageItems {
    #[serde(default)]
    #[doc = "ID of extension."]
    pub id: String,
    #[doc = "StorageArea to retrieve data from."]
    pub storage_area: StorageArea,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[doc = "Keys to retrieve."]
    pub keys: Option<Vec<String>>,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "camelCase")]
#[doc = "Removes `keys` from extension storage in the given `storageArea`."]
pub struct RemoveStorageItems {
    #[serde(default)]
    #[doc = "ID of extension."]
    pub id: String,
    #[doc = "StorageArea to remove data from."]
    pub storage_area: StorageArea,
    #[serde(default)]
    #[doc = "Keys to remove."]
    pub keys: Vec<String>,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "camelCase")]
#[doc = "Clears extension storage in the given `storageArea`."]
pub struct ClearStorageItems {
    #[serde(default)]
    #[doc = "ID of extension."]
    pub id: String,
    #[doc = "StorageArea to remove data from."]
    pub storage_area: StorageArea,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "camelCase")]
#[doc = "Sets `values` in extension storage in the given `storageArea`. The provided `values`\n will be merged with existing values in the storage area."]
pub struct SetStorageItems {
    #[serde(default)]
    #[doc = "ID of extension."]
    pub id: String,
    #[doc = "StorageArea to set data in."]
    pub storage_area: StorageArea,
    #[serde(default)]
    #[doc = "Values to set."]
    pub values: Json,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[doc = "Runs an extension default action.\n Available if the client is connected using the --remote-debugging-pipe\n flag and the --enable-unsafe-extension-debugging flag is set."]
pub struct TriggerActionReturnObject(pub Option<Json>);
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[serde(rename_all = "camelCase")]
#[doc = "Installs an unpacked extension from the filesystem similar to\n --load-extension CLI flags. Returns extension ID once the extension\n has been installed. Available if the client is connected using the\n --remote-debugging-pipe flag and the --enable-unsafe-extension-debugging\n flag is set."]
pub struct LoadUnpackedReturnObject {
    #[serde(default)]
    #[doc = "Extension id."]
    pub id: String,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[doc = "Uninstalls an unpacked extension (others not supported) from the profile.\n Available if the client is connected using the --remote-debugging-pipe flag\n and the --enable-unsafe-extension-debugging."]
pub struct UninstallReturnObject(pub Option<Json>);
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[serde(rename_all = "camelCase")]
#[doc = "Gets data from extension storage in the given `storageArea`. If `keys` is\n specified, these are used to filter the result."]
pub struct GetStorageItemsReturnObject {
    #[serde(default)]
    pub data: Json,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[doc = "Removes `keys` from extension storage in the given `storageArea`."]
pub struct RemoveStorageItemsReturnObject(pub Option<Json>);
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[doc = "Clears extension storage in the given `storageArea`."]
pub struct ClearStorageItemsReturnObject(pub Option<Json>);
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[doc = "Sets `values` in extension storage in the given `storageArea`. The provided `values`\n will be merged with existing values in the storage area."]
pub struct SetStorageItemsReturnObject(pub Option<Json>);
impl Method for TriggerAction {
    const NAME: &'static str = "Extensions.triggerAction";
    type ReturnObject = TriggerActionReturnObject;
}
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
    use derive_builder::Builder;
    #[allow(unused_imports)]
    use serde::{Deserialize, Serialize};
    #[allow(unused_imports)]
    use serde_json::Value as Json;
}
