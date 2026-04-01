// Auto-generated from Chrome at version 146.0.7680.165 domain: FileSystem
use super::network;
use super::storage;
#[allow(unused_imports)]
use super::types::*;
#[allow(unused_imports)]
use derive_builder::Builder;
#[allow(unused_imports)]
use serde::{Deserialize, Serialize};
#[allow(unused_imports)]
use serde_json::Value as Json;
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "camelCase")]
pub struct File {
    #[serde(default)]
    pub name: String,
    #[doc = "Timestamp"]
    pub last_modified: network::TimeSinceEpoch,
    #[serde(default)]
    #[doc = "Size in bytes"]
    pub size: JsFloat,
    #[serde(default)]
    pub r#type: String,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "camelCase")]
pub struct Directory {
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    pub nested_directories: Vec<String>,
    #[doc = "Files that are directly nested under this directory."]
    pub nested_files: Vec<File>,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "camelCase")]
pub struct BucketFileSystemLocator {
    #[doc = "Storage key"]
    pub storage_key: storage::SerializedStorageKey,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[doc = "Bucket name. Not passing a `bucketName` will retrieve the default Bucket. (https://developer.mozilla.org/en-US/docs/Web/API/Storage_API#storage_buckets)"]
    pub bucket_name: Option<String>,
    #[serde(default)]
    #[doc = "Path to the directory using each path component as an array item."]
    pub path_components: Vec<String>,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "camelCase")]
pub struct GetDirectory {
    pub bucket_file_system_locator: BucketFileSystemLocator,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[serde(rename_all = "camelCase")]
pub struct GetDirectoryReturnObject {
    #[doc = "Returns the directory object at the path."]
    pub directory: Directory,
}
impl Method for GetDirectory {
    const NAME: &'static str = "FileSystem.getDirectory";
    type ReturnObject = GetDirectoryReturnObject;
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
