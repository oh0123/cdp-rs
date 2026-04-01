// Auto-generated from Chrome at version 146.0.7680.165 domain: Inspector
#[allow(unused_imports)]
use super::types::*;
#[allow(unused_imports)]
use derive_builder::Builder;
#[allow(unused_imports)]
use serde::{Deserialize, Serialize};
#[allow(unused_imports)]
use serde_json::Value as Json;
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct Disable(pub Option<Json>);
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct Enable(pub Option<Json>);
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[doc = "Disables inspector domain notifications."]
pub struct DisableReturnObject(pub Option<Json>);
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[doc = "Enables inspector domain notifications."]
pub struct EnableReturnObject(pub Option<Json>);
impl Method for Disable {
    const NAME: &'static str = "Inspector.disable";
    type ReturnObject = DisableReturnObject;
}
impl Method for Enable {
    const NAME: &'static str = "Inspector.enable";
    type ReturnObject = EnableReturnObject;
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
    pub struct DetachedEvent {
        pub params: DetachedEventParams,
    }
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
    pub struct DetachedEventParams {
        #[serde(default)]
        #[doc = "The reason why connection has been terminated."]
        pub reason: String,
    }
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
    pub struct TargetCrashedEvent(pub Option<Json>);
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
    pub struct TargetReloadedAfterCrashEvent(pub Option<Json>);
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
    pub struct WorkerScriptLoadedEvent(pub Option<Json>);
}
