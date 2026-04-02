// Auto-generated from Chrome at version 146.0.7680.165 domain: Tethering
#![allow(dead_code)]
#[allow(unused_imports)]
use super::types::*;
#[allow(unused_imports)]
use derive_builder::Builder;
#[allow(unused_imports)]
use serde::{Deserialize, Serialize};
#[allow(unused_imports)]
use serde_json::Value as Json;
#[allow(deprecated)]
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "camelCase")]
#[doc = "Request browser port binding."]
pub struct Bind {
    #[serde(default)]
    #[doc = "Port number to bind."]
    pub port: JsUInt,
}
#[allow(deprecated)]
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "camelCase")]
#[doc = "Request browser port unbinding."]
pub struct Unbind {
    #[serde(default)]
    #[doc = "Port number to unbind."]
    pub port: JsUInt,
}
#[allow(deprecated)]
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[doc = "Request browser port binding."]
pub struct BindReturnObject(pub Option<Json>);
#[allow(deprecated)]
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[doc = "Request browser port unbinding."]
pub struct UnbindReturnObject(pub Option<Json>);
#[allow(deprecated)]
impl Method for Bind {
    const NAME: &'static str = "Tethering.bind";
    type ReturnObject = BindReturnObject;
}
#[allow(deprecated)]
impl Method for Unbind {
    const NAME: &'static str = "Tethering.unbind";
    type ReturnObject = UnbindReturnObject;
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
    #[allow(deprecated)]
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
    pub struct AcceptedEvent {
        pub params: AcceptedEventParams,
    }
    #[allow(deprecated)]
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
    #[serde(rename_all = "camelCase")]
    pub struct AcceptedEventParams {
        #[serde(default)]
        #[doc = "Port number that was successfully bound."]
        pub port: JsUInt,
        #[serde(default)]
        #[doc = "Connection id to be used."]
        pub connection_id: String,
    }
}
