// Auto-generated from Chrome at version 146.0.7680.165 domain: Tethering
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
#[doc = "Request browser port binding."]
pub struct Bind {
    #[serde(default)]
    #[doc = "Port number to bind."]
    pub port: JsUInt,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "camelCase")]
#[doc = "Request browser port unbinding."]
pub struct Unbind {
    #[serde(default)]
    #[doc = "Port number to unbind."]
    pub port: JsUInt,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[doc = "Request browser port binding."]
pub struct BindReturnObject(pub Option<Json>);
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[doc = "Request browser port unbinding."]
pub struct UnbindReturnObject(pub Option<Json>);
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
    use derive_builder::Builder;
    #[allow(unused_imports)]
    use serde::{Deserialize, Serialize};
    #[allow(unused_imports)]
    use serde_json::Value as Json;
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
    pub struct AcceptedEvent {
        pub params: AcceptedEventParams,
    }
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
    pub struct AcceptedEventParams {
        #[serde(default)]
        #[doc = "Port number that was successfully bound."]
        pub port: JsUInt,
        #[serde(default)]
        #[doc = "Connection id to be used."]
        pub connection_id: String,
    }
}
