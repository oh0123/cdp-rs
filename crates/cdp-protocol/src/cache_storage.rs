// Auto-generated from Chrome at version 140.0.7339.186 domain: CacheStorage
use super::storage;
#[allow(unused_imports)]
use super::types::*;
#[allow(unused_imports)]
use serde::{Deserialize, Serialize};
#[allow(unused_imports)]
use serde_json::Value as Json;
pub type CacheId = String;
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub enum CachedResponseType {
    #[serde(rename = "basic")]
    Basic,
    #[serde(rename = "cors")]
    Cors,
    #[serde(rename = "default")]
    Default,
    #[serde(rename = "error")]
    Error,
    #[serde(rename = "opaqueResponse")]
    OpaqueResponse,
    #[serde(rename = "opaqueRedirect")]
    OpaqueRedirect,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct DataEntry {
    #[serde(default)]
    #[serde(rename = "requestURL")]
    pub request_url: String,
    #[serde(default)]
    #[serde(rename = "requestMethod")]
    pub request_method: String,
    #[serde(rename = "requestHeaders")]
    pub request_headers: Vec<Header>,
    #[serde(default)]
    #[serde(rename = "responseTime")]
    pub response_time: JsFloat,
    #[serde(default)]
    #[serde(rename = "responseStatus")]
    pub response_status: JsUInt,
    #[serde(default)]
    #[serde(rename = "responseStatusText")]
    pub response_status_text: String,
    #[serde(rename = "responseType")]
    pub response_type: CachedResponseType,
    #[serde(rename = "responseHeaders")]
    pub response_headers: Vec<Header>,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct Cache {
    #[serde(rename = "cacheId")]
    pub cache_id: CacheId,
    #[serde(default)]
    #[serde(rename = "securityOrigin")]
    pub security_origin: String,
    #[serde(default)]
    #[serde(rename = "storageKey")]
    pub storage_key: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "storageBucket")]
    pub storage_bucket: Option<storage::StorageBucket>,
    #[serde(default)]
    #[serde(rename = "cacheName")]
    pub cache_name: String,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct Header {
    #[serde(default)]
    #[serde(rename = "name")]
    pub name: String,
    #[serde(default)]
    #[serde(rename = "value")]
    pub value: String,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct CachedResponse {
    #[serde(rename = "body")]
    pub body: Vec<u8>,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct DeleteCache {
    #[serde(rename = "cacheId")]
    pub cache_id: CacheId,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct DeleteEntry {
    #[serde(rename = "cacheId")]
    pub cache_id: CacheId,
    #[serde(default)]
    #[serde(rename = "request")]
    pub request: String,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct RequestCacheNames {
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
pub struct RequestCachedResponse {
    #[serde(rename = "cacheId")]
    pub cache_id: CacheId,
    #[serde(default)]
    #[serde(rename = "requestURL")]
    pub request_url: String,
    #[serde(rename = "requestHeaders")]
    pub request_headers: Vec<Header>,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct RequestEntries {
    #[serde(rename = "cacheId")]
    pub cache_id: CacheId,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(rename = "skipCount")]
    pub skip_count: Option<JsUInt>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(rename = "pageSize")]
    pub page_size: Option<JsUInt>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(rename = "pathFilter")]
    pub path_filter: Option<String>,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct DeleteCacheReturnObject {}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct DeleteEntryReturnObject {}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct RequestCacheNamesReturnObject {
    #[serde(rename = "caches")]
    pub caches: Vec<Cache>,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct RequestCachedResponseReturnObject {
    #[serde(rename = "response")]
    pub response: CachedResponse,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct RequestEntriesReturnObject {
    #[serde(rename = "cacheDataEntries")]
    pub cache_data_entries: Vec<DataEntry>,
    #[serde(default)]
    #[serde(rename = "returnCount")]
    pub return_count: JsFloat,
}
impl Method for DeleteCache {
    const NAME: &'static str = "CacheStorage.deleteCache";
    type ReturnObject = DeleteCacheReturnObject;
}
impl Method for DeleteEntry {
    const NAME: &'static str = "CacheStorage.deleteEntry";
    type ReturnObject = DeleteEntryReturnObject;
}
impl Method for RequestCacheNames {
    const NAME: &'static str = "CacheStorage.requestCacheNames";
    type ReturnObject = RequestCacheNamesReturnObject;
}
impl Method for RequestCachedResponse {
    const NAME: &'static str = "CacheStorage.requestCachedResponse";
    type ReturnObject = RequestCachedResponseReturnObject;
}
impl Method for RequestEntries {
    const NAME: &'static str = "CacheStorage.requestEntries";
    type ReturnObject = RequestEntriesReturnObject;
}
pub mod events {
    #[allow(unused_imports)]
    use super::super::types::*;
    #[allow(unused_imports)]
    use serde::{Deserialize, Serialize};
}
