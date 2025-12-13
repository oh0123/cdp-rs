// Auto-generated from Chrome at version 143.0.7499.110 domain: Performance
#[allow(unused_imports)]
use super::types::*;
#[allow(unused_imports)]
use serde::{Deserialize, Serialize};
#[allow(unused_imports)]
use serde_json::Value as Json;
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub enum EnableTimeDomainOption {
    #[serde(rename = "timeTicks")]
    TimeTicks,
    #[serde(rename = "threadTicks")]
    ThreadTicks,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub enum SetTimeDomainTimeDomainOption {
    #[serde(rename = "timeTicks")]
    TimeTicks,
    #[serde(rename = "threadTicks")]
    ThreadTicks,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct Metric {
    #[serde(default)]
    #[serde(rename = "name")]
    pub name: String,
    #[serde(default)]
    #[serde(rename = "value")]
    pub value: JsFloat,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Disable(pub Option<serde_json::Value>);
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct Enable {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "timeDomain")]
    pub time_domain: Option<EnableTimeDomainOption>,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct SetTimeDomain {
    #[serde(rename = "timeDomain")]
    pub time_domain: SetTimeDomainTimeDomainOption,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct GetMetrics(pub Option<serde_json::Value>);
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct DisableReturnObject {}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct EnableReturnObject {}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct SetTimeDomainReturnObject {}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct GetMetricsReturnObject {
    #[serde(rename = "metrics")]
    pub metrics: Vec<Metric>,
}
impl Method for Disable {
    const NAME: &'static str = "Performance.disable";
    type ReturnObject = DisableReturnObject;
}
impl Method for Enable {
    const NAME: &'static str = "Performance.enable";
    type ReturnObject = EnableReturnObject;
}
impl Method for SetTimeDomain {
    const NAME: &'static str = "Performance.setTimeDomain";
    type ReturnObject = SetTimeDomainReturnObject;
}
impl Method for GetMetrics {
    const NAME: &'static str = "Performance.getMetrics";
    type ReturnObject = GetMetricsReturnObject;
}
pub mod events {
    #[allow(unused_imports)]
    use super::super::types::*;
    #[allow(unused_imports)]
    use serde::{Deserialize, Serialize};
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
    pub struct MetricsEvent {
        pub params: MetricsEventParams,
    }
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
    pub struct MetricsEventParams {
        #[serde(rename = "metrics")]
        pub metrics: Vec<super::Metric>,
        #[serde(default)]
        #[serde(rename = "title")]
        pub title: String,
    }
}
