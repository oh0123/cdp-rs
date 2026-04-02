// Auto-generated from Chrome at version 146.0.7680.165 domain: IndexedDB
#![allow(dead_code)]
use super::runtime;
use super::storage;
#[allow(unused_imports)]
use super::types::*;
#[allow(unused_imports)]
use derive_builder::Builder;
#[allow(unused_imports)]
use serde::{Deserialize, Serialize};
#[allow(unused_imports)]
use serde_json::Value as Json;
#[allow(deprecated)]
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub enum KeyType {
    #[serde(rename = "number")]
    Number,
    #[serde(rename = "string")]
    String,
    #[serde(rename = "date")]
    Date,
    #[serde(rename = "array")]
    Array,
}
#[allow(deprecated)]
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub enum KeyPathType {
    #[serde(rename = "null")]
    Null,
    #[serde(rename = "string")]
    String,
    #[serde(rename = "array")]
    Array,
}
#[allow(deprecated)]
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "camelCase")]
#[doc = "Database with an array of object stores."]
pub struct DatabaseWithObjectStores {
    #[serde(default)]
    #[doc = "Database name."]
    pub name: String,
    #[serde(default)]
    #[doc = "Database version (type is not 'integer', as the standard\n requires the version number to be 'unsigned long long')"]
    pub version: JsFloat,
    #[doc = "Object stores in this database."]
    pub object_stores: Vec<ObjectStore>,
}
#[allow(deprecated)]
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "camelCase")]
#[doc = "Object store."]
pub struct ObjectStore {
    #[serde(default)]
    #[doc = "Object store name."]
    pub name: String,
    #[doc = "Object store key path."]
    pub key_path: KeyPath,
    #[serde(default)]
    #[doc = "If true, object store has auto increment flag set."]
    pub auto_increment: bool,
    #[doc = "Indexes in this object store."]
    pub indexes: Vec<ObjectStoreIndex>,
}
#[allow(deprecated)]
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "camelCase")]
#[doc = "Object store index."]
pub struct ObjectStoreIndex {
    #[serde(default)]
    #[doc = "Index name."]
    pub name: String,
    #[doc = "Index key path."]
    pub key_path: KeyPath,
    #[serde(default)]
    #[doc = "If true, index is unique."]
    pub unique: bool,
    #[serde(default)]
    #[doc = "If true, index allows multiple entries for a key."]
    pub multi_entry: bool,
}
#[allow(deprecated)]
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "camelCase")]
#[doc = "Key."]
pub struct Key {
    #[doc = "Key type."]
    pub r#type: KeyType,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[doc = "Number value."]
    pub number: Option<JsFloat>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[doc = "String value."]
    pub string: Option<String>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[doc = "Date value."]
    pub date: Option<JsFloat>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[doc = "Array value."]
    pub array: Option<Vec<Key>>,
}
#[allow(deprecated)]
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "camelCase")]
#[doc = "Key range."]
pub struct KeyRange {
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[doc = "Lower bound."]
    pub lower: Option<Key>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[doc = "Upper bound."]
    pub upper: Option<Key>,
    #[serde(default)]
    #[doc = "If true lower bound is open."]
    pub lower_open: bool,
    #[serde(default)]
    #[doc = "If true upper bound is open."]
    pub upper_open: bool,
}
#[allow(deprecated)]
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "camelCase")]
#[doc = "Data entry."]
pub struct DataEntry {
    #[doc = "Key object."]
    pub key: runtime::RemoteObject,
    #[doc = "Primary key object."]
    pub primary_key: runtime::RemoteObject,
    #[doc = "Value object."]
    pub value: runtime::RemoteObject,
}
#[allow(deprecated)]
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "camelCase")]
#[doc = "Key path."]
pub struct KeyPath {
    #[doc = "Key path type."]
    pub r#type: KeyPathType,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[doc = "String value."]
    pub string: Option<String>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[doc = "Array value."]
    pub array: Option<Vec<String>>,
}
#[allow(deprecated)]
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "camelCase")]
#[doc = "Clears all entries from an object store."]
pub struct ClearObjectStore {
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[doc = "At least and at most one of securityOrigin, storageKey, or storageBucket must be specified.\n Security origin."]
    pub security_origin: Option<String>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[doc = "Storage key."]
    pub storage_key: Option<String>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[doc = "Storage bucket. If not specified, it uses the default bucket."]
    pub storage_bucket: Option<storage::StorageBucket>,
    #[serde(default)]
    #[doc = "Database name."]
    pub database_name: String,
    #[serde(default)]
    #[doc = "Object store name."]
    pub object_store_name: String,
}
#[allow(deprecated)]
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "camelCase")]
#[doc = "Deletes a database."]
pub struct DeleteDatabase {
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[doc = "At least and at most one of securityOrigin, storageKey, or storageBucket must be specified.\n Security origin."]
    pub security_origin: Option<String>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[doc = "Storage key."]
    pub storage_key: Option<String>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[doc = "Storage bucket. If not specified, it uses the default bucket."]
    pub storage_bucket: Option<storage::StorageBucket>,
    #[serde(default)]
    #[doc = "Database name."]
    pub database_name: String,
}
#[allow(deprecated)]
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "camelCase")]
#[doc = "Delete a range of entries from an object store"]
pub struct DeleteObjectStoreEntries {
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[doc = "At least and at most one of securityOrigin, storageKey, or storageBucket must be specified.\n Security origin."]
    pub security_origin: Option<String>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[doc = "Storage key."]
    pub storage_key: Option<String>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[doc = "Storage bucket. If not specified, it uses the default bucket."]
    pub storage_bucket: Option<storage::StorageBucket>,
    #[serde(default)]
    pub database_name: String,
    #[serde(default)]
    pub object_store_name: String,
    #[doc = "Range of entry keys to delete"]
    pub key_range: KeyRange,
}
#[allow(deprecated)]
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct Disable(pub Option<Json>);
#[allow(deprecated)]
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct Enable(pub Option<Json>);
#[allow(deprecated)]
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "camelCase")]
#[doc = "Requests data from object store or index."]
pub struct RequestData {
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[doc = "At least and at most one of securityOrigin, storageKey, or storageBucket must be specified.\n Security origin."]
    pub security_origin: Option<String>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[doc = "Storage key."]
    pub storage_key: Option<String>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[doc = "Storage bucket. If not specified, it uses the default bucket."]
    pub storage_bucket: Option<storage::StorageBucket>,
    #[serde(default)]
    #[doc = "Database name."]
    pub database_name: String,
    #[serde(default)]
    #[doc = "Object store name."]
    pub object_store_name: String,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[doc = "Index name. If not specified, it performs an object store data request."]
    pub index_name: Option<String>,
    #[serde(default)]
    #[doc = "Number of records to skip."]
    pub skip_count: JsUInt,
    #[serde(default)]
    #[doc = "Number of records to fetch."]
    pub page_size: JsUInt,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[doc = "Key range."]
    pub key_range: Option<KeyRange>,
}
#[allow(deprecated)]
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "camelCase")]
#[doc = "Gets metadata of an object store."]
pub struct GetMetadata {
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[doc = "At least and at most one of securityOrigin, storageKey, or storageBucket must be specified.\n Security origin."]
    pub security_origin: Option<String>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[doc = "Storage key."]
    pub storage_key: Option<String>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[doc = "Storage bucket. If not specified, it uses the default bucket."]
    pub storage_bucket: Option<storage::StorageBucket>,
    #[serde(default)]
    #[doc = "Database name."]
    pub database_name: String,
    #[serde(default)]
    #[doc = "Object store name."]
    pub object_store_name: String,
}
#[allow(deprecated)]
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "camelCase")]
#[doc = "Requests database with given name in given frame."]
pub struct RequestDatabase {
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[doc = "At least and at most one of securityOrigin, storageKey, or storageBucket must be specified.\n Security origin."]
    pub security_origin: Option<String>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[doc = "Storage key."]
    pub storage_key: Option<String>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[doc = "Storage bucket. If not specified, it uses the default bucket."]
    pub storage_bucket: Option<storage::StorageBucket>,
    #[serde(default)]
    #[doc = "Database name."]
    pub database_name: String,
}
#[allow(deprecated)]
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "camelCase")]
#[doc = "Requests database names for given security origin."]
pub struct RequestDatabaseNames {
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[doc = "At least and at most one of securityOrigin, storageKey, or storageBucket must be specified.\n Security origin."]
    pub security_origin: Option<String>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[doc = "Storage key."]
    pub storage_key: Option<String>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[doc = "Storage bucket. If not specified, it uses the default bucket."]
    pub storage_bucket: Option<storage::StorageBucket>,
}
#[allow(deprecated)]
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[doc = "Clears all entries from an object store."]
pub struct ClearObjectStoreReturnObject(pub Option<Json>);
#[allow(deprecated)]
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[doc = "Deletes a database."]
pub struct DeleteDatabaseReturnObject(pub Option<Json>);
#[allow(deprecated)]
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[doc = "Delete a range of entries from an object store"]
pub struct DeleteObjectStoreEntriesReturnObject(pub Option<Json>);
#[allow(deprecated)]
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[doc = "Disables events from backend."]
pub struct DisableReturnObject(pub Option<Json>);
#[allow(deprecated)]
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[doc = "Enables events from backend."]
pub struct EnableReturnObject(pub Option<Json>);
#[allow(deprecated)]
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[serde(rename_all = "camelCase")]
#[doc = "Requests data from object store or index."]
pub struct RequestDataReturnObject {
    #[doc = "Array of object store data entries."]
    pub object_store_data_entries: Vec<DataEntry>,
    #[serde(default)]
    #[doc = "If true, there are more entries to fetch in the given range."]
    pub has_more: bool,
}
#[allow(deprecated)]
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[serde(rename_all = "camelCase")]
#[doc = "Gets metadata of an object store."]
pub struct GetMetadataReturnObject {
    #[serde(default)]
    #[doc = "the entries count"]
    pub entries_count: JsFloat,
    #[serde(default)]
    #[doc = "the current value of key generator, to become the next inserted\n key into the object store. Valid if objectStore.autoIncrement\n is true."]
    pub key_generator_value: JsFloat,
}
#[allow(deprecated)]
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[serde(rename_all = "camelCase")]
#[doc = "Requests database with given name in given frame."]
pub struct RequestDatabaseReturnObject {
    #[doc = "Database with an array of object stores."]
    pub database_with_object_stores: DatabaseWithObjectStores,
}
#[allow(deprecated)]
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[serde(rename_all = "camelCase")]
#[doc = "Requests database names for given security origin."]
pub struct RequestDatabaseNamesReturnObject {
    #[doc = "Database names for origin."]
    pub database_names: Vec<String>,
}
#[allow(deprecated)]
impl Method for ClearObjectStore {
    const NAME: &'static str = "IndexedDB.clearObjectStore";
    type ReturnObject = ClearObjectStoreReturnObject;
}
#[allow(deprecated)]
impl Method for DeleteDatabase {
    const NAME: &'static str = "IndexedDB.deleteDatabase";
    type ReturnObject = DeleteDatabaseReturnObject;
}
#[allow(deprecated)]
impl Method for DeleteObjectStoreEntries {
    const NAME: &'static str = "IndexedDB.deleteObjectStoreEntries";
    type ReturnObject = DeleteObjectStoreEntriesReturnObject;
}
#[allow(deprecated)]
impl Method for Disable {
    const NAME: &'static str = "IndexedDB.disable";
    type ReturnObject = DisableReturnObject;
}
#[allow(deprecated)]
impl Method for Enable {
    const NAME: &'static str = "IndexedDB.enable";
    type ReturnObject = EnableReturnObject;
}
#[allow(deprecated)]
impl Method for RequestData {
    const NAME: &'static str = "IndexedDB.requestData";
    type ReturnObject = RequestDataReturnObject;
}
#[allow(deprecated)]
impl Method for GetMetadata {
    const NAME: &'static str = "IndexedDB.getMetadata";
    type ReturnObject = GetMetadataReturnObject;
}
#[allow(deprecated)]
impl Method for RequestDatabase {
    const NAME: &'static str = "IndexedDB.requestDatabase";
    type ReturnObject = RequestDatabaseReturnObject;
}
#[allow(deprecated)]
impl Method for RequestDatabaseNames {
    const NAME: &'static str = "IndexedDB.requestDatabaseNames";
    type ReturnObject = RequestDatabaseNamesReturnObject;
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
}
