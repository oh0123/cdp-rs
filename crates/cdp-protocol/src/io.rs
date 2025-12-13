// Auto-generated from Chrome at version 143.0.7499.110 domain: IO
use super::runtime;
#[allow(unused_imports)]
use super::types::*;
#[allow(unused_imports)]
use serde::{Deserialize, Serialize};
#[allow(unused_imports)]
use serde_json::Value as Json;
pub type StreamHandle = String;
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct Close {
    #[serde(rename = "handle")]
    pub handle: StreamHandle,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct Read {
    #[serde(rename = "handle")]
    pub handle: StreamHandle,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(rename = "offset")]
    pub offset: Option<JsUInt>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(rename = "size")]
    pub size: Option<JsUInt>,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct ResolveBlob {
    #[serde(rename = "objectId")]
    pub object_id: runtime::RemoteObjectId,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct CloseReturnObject {}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct ReadReturnObject {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(rename = "base64Encoded")]
    pub base_64_encoded: Option<bool>,
    #[serde(default)]
    #[serde(rename = "data")]
    pub data: String,
    #[serde(default)]
    #[serde(rename = "eof")]
    pub eof: bool,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct ResolveBlobReturnObject {
    #[serde(default)]
    #[serde(rename = "uuid")]
    pub uuid: String,
}
impl Method for Close {
    const NAME: &'static str = "IO.close";
    type ReturnObject = CloseReturnObject;
}
impl Method for Read {
    const NAME: &'static str = "IO.read";
    type ReturnObject = ReadReturnObject;
}
impl Method for ResolveBlob {
    const NAME: &'static str = "IO.resolveBlob";
    type ReturnObject = ResolveBlobReturnObject;
}
pub mod events {
    #[allow(unused_imports)]
    use super::super::types::*;
    #[allow(unused_imports)]
    use serde::{Deserialize, Serialize};
}
