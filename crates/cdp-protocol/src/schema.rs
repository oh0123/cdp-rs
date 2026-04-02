// Auto-generated from Chrome at version 146.0.7680.165 domain: Schema
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
#[doc = "Description of the protocol domain."]
pub struct Domain {
    #[serde(default)]
    #[doc = "Domain name."]
    pub name: String,
    #[serde(default)]
    #[doc = "Domain version."]
    pub version: String,
}
#[allow(deprecated)]
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct GetDomains(pub Option<Json>);
#[allow(deprecated)]
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[serde(rename_all = "camelCase")]
#[doc = "Returns supported domains."]
pub struct GetDomainsReturnObject {
    #[doc = "List of supported domains."]
    pub domains: Vec<Domain>,
}
#[allow(deprecated)]
impl Method for GetDomains {
    const NAME: &'static str = "Schema.getDomains";
    type ReturnObject = GetDomainsReturnObject;
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
}
