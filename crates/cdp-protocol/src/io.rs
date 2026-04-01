// Auto-generated from Chrome at version 146.0.7680.165 domain: IO
use super::runtime;
#[allow(unused_imports)]
use super::types::*;
#[allow(unused_imports)]
use derive_builder::Builder;
#[allow(unused_imports)]
use serde::{Deserialize, Serialize};
#[allow(unused_imports)]
use serde_json::Value as Json;
pub type StreamHandle = String;
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "camelCase")]
#[doc = "Close the stream, discard any temporary backing storage."]
pub struct Close {
    #[doc = "Handle of the stream to close."]
    pub handle: StreamHandle,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "camelCase")]
#[doc = "Read a chunk of the stream"]
pub struct Read {
    #[doc = "Handle of the stream to read."]
    pub handle: StreamHandle,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[doc = "Seek to the specified offset before reading (if not specified, proceed with offset\n following the last read). Some types of streams may only support sequential reads."]
    pub offset: Option<JsUInt>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[doc = "Maximum number of bytes to read (left upon the agent discretion if not specified)."]
    pub size: Option<JsUInt>,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "camelCase")]
#[doc = "Return UUID of Blob object specified by a remote object id."]
pub struct ResolveBlob {
    #[doc = "Object id of a Blob object wrapper."]
    pub object_id: runtime::RemoteObjectId,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[doc = "Close the stream, discard any temporary backing storage."]
pub struct CloseReturnObject(pub Option<Json>);
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[serde(rename_all = "camelCase")]
#[doc = "Read a chunk of the stream"]
pub struct ReadReturnObject {
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[doc = "Set if the data is base64-encoded"]
    pub base_64_encoded: Option<bool>,
    #[serde(default)]
    #[doc = "Data that were read."]
    pub data: String,
    #[serde(default)]
    #[doc = "Set if the end-of-file condition occurred while reading."]
    pub eof: bool,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[serde(rename_all = "camelCase")]
#[doc = "Return UUID of Blob object specified by a remote object id."]
pub struct ResolveBlobReturnObject {
    #[serde(default)]
    #[doc = "UUID of the specified Blob."]
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
    use derive_builder::Builder;
    #[allow(unused_imports)]
    use serde::{Deserialize, Serialize};
    #[allow(unused_imports)]
    use serde_json::Value as Json;
}
