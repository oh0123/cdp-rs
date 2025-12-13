// Auto-generated from Chrome at version 143.0.7499.110 domain: IndexedDB
use super::runtime;
use super::storage;
#[allow(unused_imports)]
use super::types::*;
#[allow(unused_imports)]
use serde::{Deserialize, Serialize};
#[allow(unused_imports)]
use serde_json::Value as Json;
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
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub enum KeyPathType {
    #[serde(rename = "null")]
    Null,
    #[serde(rename = "string")]
    String,
    #[serde(rename = "array")]
    Array,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct DatabaseWithObjectStores {
    #[serde(default)]
    #[serde(rename = "name")]
    pub name: String,
    #[serde(default)]
    #[serde(rename = "version")]
    pub version: JsFloat,
    #[serde(rename = "objectStores")]
    pub object_stores: Vec<ObjectStore>,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct ObjectStore {
    #[serde(default)]
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "keyPath")]
    pub key_path: KeyPath,
    #[serde(default)]
    #[serde(rename = "autoIncrement")]
    pub auto_increment: bool,
    #[serde(rename = "indexes")]
    pub indexes: Vec<ObjectStoreIndex>,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct ObjectStoreIndex {
    #[serde(default)]
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "keyPath")]
    pub key_path: KeyPath,
    #[serde(default)]
    #[serde(rename = "unique")]
    pub unique: bool,
    #[serde(default)]
    #[serde(rename = "multiEntry")]
    pub multi_entry: bool,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct Key {
    #[serde(rename = "type")]
    pub r#type: KeyType,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(rename = "number")]
    pub number: Option<JsFloat>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(rename = "string")]
    pub string: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(rename = "date")]
    pub date: Option<JsFloat>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "array")]
    pub array: Option<Vec<Key>>,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct KeyRange {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "lower")]
    pub lower: Option<Key>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "upper")]
    pub upper: Option<Key>,
    #[serde(default)]
    #[serde(rename = "lowerOpen")]
    pub lower_open: bool,
    #[serde(default)]
    #[serde(rename = "upperOpen")]
    pub upper_open: bool,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct DataEntry {
    #[serde(rename = "key")]
    pub key: runtime::RemoteObject,
    #[serde(rename = "primaryKey")]
    pub primary_key: runtime::RemoteObject,
    #[serde(rename = "value")]
    pub value: runtime::RemoteObject,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct KeyPath {
    #[serde(rename = "type")]
    pub r#type: KeyPathType,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(rename = "string")]
    pub string: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(rename = "array")]
    pub array: Option<Vec<String>>,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct ClearObjectStore {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(rename = "securityOrigin")]
    pub security_origin: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(rename = "storageKey")]
    pub storage_key: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "storageBucket")]
    pub storage_bucket: Option<storage::StorageBucket>,
    #[serde(default)]
    #[serde(rename = "databaseName")]
    pub database_name: String,
    #[serde(default)]
    #[serde(rename = "objectStoreName")]
    pub object_store_name: String,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct DeleteDatabase {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(rename = "securityOrigin")]
    pub security_origin: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(rename = "storageKey")]
    pub storage_key: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "storageBucket")]
    pub storage_bucket: Option<storage::StorageBucket>,
    #[serde(default)]
    #[serde(rename = "databaseName")]
    pub database_name: String,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct DeleteObjectStoreEntries {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(rename = "securityOrigin")]
    pub security_origin: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(rename = "storageKey")]
    pub storage_key: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "storageBucket")]
    pub storage_bucket: Option<storage::StorageBucket>,
    #[serde(default)]
    #[serde(rename = "databaseName")]
    pub database_name: String,
    #[serde(default)]
    #[serde(rename = "objectStoreName")]
    pub object_store_name: String,
    #[serde(rename = "keyRange")]
    pub key_range: KeyRange,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Disable(pub Option<serde_json::Value>);
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Enable(pub Option<serde_json::Value>);
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct RequestData {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(rename = "securityOrigin")]
    pub security_origin: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(rename = "storageKey")]
    pub storage_key: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "storageBucket")]
    pub storage_bucket: Option<storage::StorageBucket>,
    #[serde(default)]
    #[serde(rename = "databaseName")]
    pub database_name: String,
    #[serde(default)]
    #[serde(rename = "objectStoreName")]
    pub object_store_name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(rename = "indexName")]
    pub index_name: Option<String>,
    #[serde(default)]
    #[serde(rename = "skipCount")]
    pub skip_count: JsUInt,
    #[serde(default)]
    #[serde(rename = "pageSize")]
    pub page_size: JsUInt,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "keyRange")]
    pub key_range: Option<KeyRange>,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct GetMetadata {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(rename = "securityOrigin")]
    pub security_origin: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(rename = "storageKey")]
    pub storage_key: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "storageBucket")]
    pub storage_bucket: Option<storage::StorageBucket>,
    #[serde(default)]
    #[serde(rename = "databaseName")]
    pub database_name: String,
    #[serde(default)]
    #[serde(rename = "objectStoreName")]
    pub object_store_name: String,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct RequestDatabase {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(rename = "securityOrigin")]
    pub security_origin: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(rename = "storageKey")]
    pub storage_key: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "storageBucket")]
    pub storage_bucket: Option<storage::StorageBucket>,
    #[serde(default)]
    #[serde(rename = "databaseName")]
    pub database_name: String,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct RequestDatabaseNames {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(rename = "securityOrigin")]
    pub security_origin: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(rename = "storageKey")]
    pub storage_key: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "storageBucket")]
    pub storage_bucket: Option<storage::StorageBucket>,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct ClearObjectStoreReturnObject {}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct DeleteDatabaseReturnObject {}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct DeleteObjectStoreEntriesReturnObject {}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct DisableReturnObject {}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct EnableReturnObject {}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct RequestDataReturnObject {
    #[serde(rename = "objectStoreDataEntries")]
    pub object_store_data_entries: Vec<DataEntry>,
    #[serde(default)]
    #[serde(rename = "hasMore")]
    pub has_more: bool,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct GetMetadataReturnObject {
    #[serde(default)]
    #[serde(rename = "entriesCount")]
    pub entries_count: JsFloat,
    #[serde(default)]
    #[serde(rename = "keyGeneratorValue")]
    pub key_generator_value: JsFloat,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct RequestDatabaseReturnObject {
    #[serde(rename = "databaseWithObjectStores")]
    pub database_with_object_stores: DatabaseWithObjectStores,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct RequestDatabaseNamesReturnObject {
    #[serde(rename = "databaseNames")]
    pub database_names: Vec<String>,
}
impl Method for ClearObjectStore {
    const NAME: &'static str = "IndexedDB.clearObjectStore";
    type ReturnObject = ClearObjectStoreReturnObject;
}
impl Method for DeleteDatabase {
    const NAME: &'static str = "IndexedDB.deleteDatabase";
    type ReturnObject = DeleteDatabaseReturnObject;
}
impl Method for DeleteObjectStoreEntries {
    const NAME: &'static str = "IndexedDB.deleteObjectStoreEntries";
    type ReturnObject = DeleteObjectStoreEntriesReturnObject;
}
impl Method for Disable {
    const NAME: &'static str = "IndexedDB.disable";
    type ReturnObject = DisableReturnObject;
}
impl Method for Enable {
    const NAME: &'static str = "IndexedDB.enable";
    type ReturnObject = EnableReturnObject;
}
impl Method for RequestData {
    const NAME: &'static str = "IndexedDB.requestData";
    type ReturnObject = RequestDataReturnObject;
}
impl Method for GetMetadata {
    const NAME: &'static str = "IndexedDB.getMetadata";
    type ReturnObject = GetMetadataReturnObject;
}
impl Method for RequestDatabase {
    const NAME: &'static str = "IndexedDB.requestDatabase";
    type ReturnObject = RequestDatabaseReturnObject;
}
impl Method for RequestDatabaseNames {
    const NAME: &'static str = "IndexedDB.requestDatabaseNames";
    type ReturnObject = RequestDatabaseNamesReturnObject;
}
pub mod events {
    #[allow(unused_imports)]
    use super::super::types::*;
    #[allow(unused_imports)]
    use serde::{Deserialize, Serialize};
}
