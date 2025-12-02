// Auto-generated from Chrome at version 140.0.7339.186 domain: FileSystem
use super::network;
use super::storage;
#[allow(unused_imports)]
use super::types::*;
#[allow(unused_imports)]
use serde::{Deserialize, Serialize};
#[allow(unused_imports)]
use serde_json::Value as Json;
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct File {
    #[serde(default)]
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "lastModified")]
    pub last_modified: network::TimeSinceEpoch,
    #[serde(default)]
    #[serde(rename = "size")]
    pub size: JsFloat,
    #[serde(default)]
    #[serde(rename = "type")]
    pub r#type: String,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct Directory {
    #[serde(default)]
    #[serde(rename = "name")]
    pub name: String,
    #[serde(default)]
    #[serde(rename = "nestedDirectories")]
    pub nested_directories: Vec<String>,
    #[serde(rename = "nestedFiles")]
    pub nested_files: Vec<File>,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct BucketFileSystemLocator {
    #[serde(rename = "storageKey")]
    pub storage_key: storage::SerializedStorageKey,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(rename = "bucketName")]
    pub bucket_name: Option<String>,
    #[serde(default)]
    #[serde(rename = "pathComponents")]
    pub path_components: Vec<String>,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct GetDirectory {
    #[serde(rename = "bucketFileSystemLocator")]
    pub bucket_file_system_locator: BucketFileSystemLocator,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct GetDirectoryReturnObject {
    #[serde(rename = "directory")]
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
    use serde::{Deserialize, Serialize};
}
