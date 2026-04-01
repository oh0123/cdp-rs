// Auto-generated from Chrome at version 146.0.7680.165 domain: Performance
#[allow(unused_imports)]
use super::types::*;
#[allow(unused_imports)]
use derive_builder::Builder;
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
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "camelCase")]
#[doc = "Run-time execution metric."]
pub struct Metric {
    #[serde(default)]
    #[doc = "Metric name."]
    pub name: String,
    #[serde(default)]
    #[doc = "Metric value."]
    pub value: JsFloat,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct Disable(pub Option<Json>);
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "camelCase")]
#[doc = "Enable collecting and reporting metrics."]
pub struct Enable {
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[doc = "Time domain to use for collecting and reporting duration metrics."]
    pub time_domain: Option<EnableTimeDomainOption>,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "camelCase")]
#[doc = "Sets time domain to use for collecting and reporting duration metrics.\n Note that this must be called before enabling metrics collection. Calling\n this method while metrics collection is enabled returns an error."]
#[deprecated]
pub struct SetTimeDomain {
    #[doc = "Time domain"]
    pub time_domain: SetTimeDomainTimeDomainOption,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct GetMetrics(pub Option<Json>);
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[doc = "Disable collecting and reporting metrics."]
pub struct DisableReturnObject(pub Option<Json>);
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[doc = "Enable collecting and reporting metrics."]
pub struct EnableReturnObject(pub Option<Json>);
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[doc = "Sets time domain to use for collecting and reporting duration metrics.\n Note that this must be called before enabling metrics collection. Calling\n this method while metrics collection is enabled returns an error."]
#[deprecated]
pub struct SetTimeDomainReturnObject(pub Option<Json>);
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[serde(rename_all = "camelCase")]
#[doc = "Retrieve current values of run-time metrics."]
pub struct GetMetricsReturnObject {
    #[doc = "Current values for run-time metrics."]
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
    use derive_builder::Builder;
    #[allow(unused_imports)]
    use serde::{Deserialize, Serialize};
    #[allow(unused_imports)]
    use serde_json::Value as Json;
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
    pub struct MetricsEvent {
        pub params: MetricsEventParams,
    }
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
    pub struct MetricsEventParams {
        #[doc = "Current values of the metrics."]
        pub metrics: Vec<super::Metric>,
        #[serde(default)]
        #[doc = "Timestamp title."]
        pub title: String,
    }
}
