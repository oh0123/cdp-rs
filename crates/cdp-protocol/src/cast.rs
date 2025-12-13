// Auto-generated from Chrome at version 143.0.7499.110 domain: Cast
#[allow(unused_imports)]
use super::types::*;
#[allow(unused_imports)]
use serde::{Deserialize, Serialize};
#[allow(unused_imports)]
use serde_json::Value as Json;
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct Sink {
    #[serde(default)]
    #[serde(rename = "name")]
    pub name: String,
    #[serde(default)]
    #[serde(rename = "id")]
    pub id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(rename = "session")]
    pub session: Option<String>,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct Enable {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(rename = "presentationUrl")]
    pub presentation_url: Option<String>,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Disable(pub Option<serde_json::Value>);
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct SetSinkToUse {
    #[serde(default)]
    #[serde(rename = "sinkName")]
    pub sink_name: String,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct StartDesktopMirroring {
    #[serde(default)]
    #[serde(rename = "sinkName")]
    pub sink_name: String,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct StartTabMirroring {
    #[serde(default)]
    #[serde(rename = "sinkName")]
    pub sink_name: String,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct StopCasting {
    #[serde(default)]
    #[serde(rename = "sinkName")]
    pub sink_name: String,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct EnableReturnObject {}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct DisableReturnObject {}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct SetSinkToUseReturnObject {}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct StartDesktopMirroringReturnObject {}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct StartTabMirroringReturnObject {}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct StopCastingReturnObject {}
impl Method for Enable {
    const NAME: &'static str = "Cast.enable";
    type ReturnObject = EnableReturnObject;
}
impl Method for Disable {
    const NAME: &'static str = "Cast.disable";
    type ReturnObject = DisableReturnObject;
}
impl Method for SetSinkToUse {
    const NAME: &'static str = "Cast.setSinkToUse";
    type ReturnObject = SetSinkToUseReturnObject;
}
impl Method for StartDesktopMirroring {
    const NAME: &'static str = "Cast.startDesktopMirroring";
    type ReturnObject = StartDesktopMirroringReturnObject;
}
impl Method for StartTabMirroring {
    const NAME: &'static str = "Cast.startTabMirroring";
    type ReturnObject = StartTabMirroringReturnObject;
}
impl Method for StopCasting {
    const NAME: &'static str = "Cast.stopCasting";
    type ReturnObject = StopCastingReturnObject;
}
pub mod events {
    #[allow(unused_imports)]
    use super::super::types::*;
    #[allow(unused_imports)]
    use serde::{Deserialize, Serialize};
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
    pub struct SinksUpdatedEvent {
        pub params: SinksUpdatedEventParams,
    }
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
    pub struct SinksUpdatedEventParams {
        #[serde(rename = "sinks")]
        pub sinks: Vec<super::Sink>,
    }
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
    pub struct IssueUpdatedEvent {
        pub params: IssueUpdatedEventParams,
    }
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
    pub struct IssueUpdatedEventParams {
        #[serde(default)]
        #[serde(rename = "issueMessage")]
        pub issue_message: String,
    }
}
