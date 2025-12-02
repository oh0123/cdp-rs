// Auto-generated from Chrome at version 140.0.7339.186 domain: Tethering
#[allow(unused_imports)]
use super::types::*;
#[allow(unused_imports)]
use serde::{Deserialize, Serialize};
#[allow(unused_imports)]
use serde_json::Value as Json;
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct Bind {
    #[serde(default)]
    #[serde(rename = "port")]
    pub port: JsUInt,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct Unbind {
    #[serde(default)]
    #[serde(rename = "port")]
    pub port: JsUInt,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct BindReturnObject {}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct UnbindReturnObject {}
impl Method for Bind {
    const NAME: &'static str = "Tethering.bind";
    type ReturnObject = BindReturnObject;
}
impl Method for Unbind {
    const NAME: &'static str = "Tethering.unbind";
    type ReturnObject = UnbindReturnObject;
}
pub mod events {
    #[allow(unused_imports)]
    use super::super::types::*;
    #[allow(unused_imports)]
    use serde::{Deserialize, Serialize};
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
    pub struct AcceptedEvent {
        pub params: AcceptedEventParams,
    }
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
    pub struct AcceptedEventParams {
        #[serde(default)]
        #[serde(rename = "port")]
        pub port: JsUInt,
        #[serde(default)]
        #[serde(rename = "connectionId")]
        pub connection_id: String,
    }
}
