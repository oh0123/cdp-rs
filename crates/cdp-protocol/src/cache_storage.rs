// Auto-generated from Chrome at version 146.0.7680.165 domain: CacheStorage
use super::storage;
#[allow(unused_imports)]
use super::types::*;
#[allow(unused_imports)]
use derive_builder::Builder;
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
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "camelCase")]
#[doc = "Data entry."]
pub struct DataEntry {
    #[serde(default)]
    #[doc = "Request URL."]
    #[serde(rename = "requestURL")]
    pub request_url: String,
    #[serde(default)]
    #[doc = "Request method."]
    pub request_method: String,
    #[doc = "Request headers"]
    pub request_headers: Vec<Header>,
    #[serde(default)]
    #[doc = "Number of seconds since epoch."]
    pub response_time: JsFloat,
    #[serde(default)]
    #[doc = "HTTP response status code."]
    pub response_status: JsUInt,
    #[serde(default)]
    #[doc = "HTTP response status text."]
    pub response_status_text: String,
    #[doc = "HTTP response type"]
    pub response_type: CachedResponseType,
    #[doc = "Response headers"]
    pub response_headers: Vec<Header>,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "camelCase")]
#[doc = "Cache identifier."]
pub struct Cache {
    #[doc = "An opaque unique id of the cache."]
    pub cache_id: CacheId,
    #[serde(default)]
    #[doc = "Security origin of the cache."]
    pub security_origin: String,
    #[serde(default)]
    #[doc = "Storage key of the cache."]
    pub storage_key: String,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[doc = "Storage bucket of the cache."]
    pub storage_bucket: Option<storage::StorageBucket>,
    #[serde(default)]
    #[doc = "The name of the cache."]
    pub cache_name: String,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "camelCase")]
pub struct Header {
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    pub value: String,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "camelCase")]
#[doc = "Cached response"]
pub struct CachedResponse {
    #[doc = "Entry content, base64-encoded."]
    pub body: Vec<u8>,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "camelCase")]
#[doc = "Deletes a cache."]
pub struct DeleteCache {
    #[doc = "Id of cache for deletion."]
    pub cache_id: CacheId,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "camelCase")]
#[doc = "Deletes a cache entry."]
pub struct DeleteEntry {
    #[doc = "Id of cache where the entry will be deleted."]
    pub cache_id: CacheId,
    #[serde(default)]
    #[doc = "URL spec of the request."]
    pub request: String,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "camelCase")]
#[doc = "Requests cache names."]
pub struct RequestCacheNames {
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[doc = "At least and at most one of securityOrigin, storageKey, storageBucket must be specified.\n Security origin."]
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
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "camelCase")]
#[doc = "Fetches cache entry."]
pub struct RequestCachedResponse {
    #[doc = "Id of cache that contains the entry."]
    pub cache_id: CacheId,
    #[serde(default)]
    #[doc = "URL spec of the request."]
    #[serde(rename = "requestURL")]
    pub request_url: String,
    #[doc = "headers of the request."]
    pub request_headers: Vec<Header>,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "camelCase")]
#[doc = "Requests data from cache."]
pub struct RequestEntries {
    #[doc = "ID of cache to get entries from."]
    pub cache_id: CacheId,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[doc = "Number of records to skip."]
    pub skip_count: Option<JsUInt>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[doc = "Number of records to fetch."]
    pub page_size: Option<JsUInt>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[doc = "If present, only return the entries containing this substring in the path"]
    pub path_filter: Option<String>,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[doc = "Deletes a cache."]
pub struct DeleteCacheReturnObject(pub Option<Json>);
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[doc = "Deletes a cache entry."]
pub struct DeleteEntryReturnObject(pub Option<Json>);
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[serde(rename_all = "camelCase")]
#[doc = "Requests cache names."]
pub struct RequestCacheNamesReturnObject {
    #[doc = "Caches for the security origin."]
    pub caches: Vec<Cache>,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[serde(rename_all = "camelCase")]
#[doc = "Fetches cache entry."]
pub struct RequestCachedResponseReturnObject {
    #[doc = "Response read from the cache."]
    pub response: CachedResponse,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[serde(rename_all = "camelCase")]
#[doc = "Requests data from cache."]
pub struct RequestEntriesReturnObject {
    #[doc = "Array of object store data entries."]
    pub cache_data_entries: Vec<DataEntry>,
    #[serde(default)]
    #[doc = "Count of returned entries from this storage. If pathFilter is empty, it\n is the count of all entries from this storage."]
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
    use derive_builder::Builder;
    #[allow(unused_imports)]
    use serde::{Deserialize, Serialize};
    #[allow(unused_imports)]
    use serde_json::Value as Json;
}
