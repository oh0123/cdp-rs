// Auto-generated from Chrome at version 146.0.7680.165 domain: Cast
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
pub struct Sink {
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    pub id: String,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[doc = "Text describing the current session. Present only if there is an active\n session on the sink."]
    pub session: Option<String>,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "camelCase")]
#[doc = "Starts observing for sinks that can be used for tab mirroring, and if set,\n sinks compatible with |presentationUrl| as well. When sinks are found, a\n |sinksUpdated| event is fired.\n Also starts observing for issue messages. When an issue is added or removed,\n an |issueUpdated| event is fired."]
pub struct Enable {
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub presentation_url: Option<String>,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct Disable(pub Option<Json>);
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "camelCase")]
#[doc = "Sets a sink to be used when the web page requests the browser to choose a\n sink via Presentation API, Remote Playback API, or Cast SDK."]
pub struct SetSinkToUse {
    #[serde(default)]
    pub sink_name: String,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "camelCase")]
#[doc = "Starts mirroring the desktop to the sink."]
pub struct StartDesktopMirroring {
    #[serde(default)]
    pub sink_name: String,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "camelCase")]
#[doc = "Starts mirroring the tab to the sink."]
pub struct StartTabMirroring {
    #[serde(default)]
    pub sink_name: String,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "camelCase")]
#[doc = "Stops the active Cast session on the sink."]
pub struct StopCasting {
    #[serde(default)]
    pub sink_name: String,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[doc = "Starts observing for sinks that can be used for tab mirroring, and if set,\n sinks compatible with |presentationUrl| as well. When sinks are found, a\n |sinksUpdated| event is fired.\n Also starts observing for issue messages. When an issue is added or removed,\n an |issueUpdated| event is fired."]
pub struct EnableReturnObject(pub Option<Json>);
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[doc = "Stops observing for sinks and issues."]
pub struct DisableReturnObject(pub Option<Json>);
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[doc = "Sets a sink to be used when the web page requests the browser to choose a\n sink via Presentation API, Remote Playback API, or Cast SDK."]
pub struct SetSinkToUseReturnObject(pub Option<Json>);
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[doc = "Starts mirroring the desktop to the sink."]
pub struct StartDesktopMirroringReturnObject(pub Option<Json>);
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[doc = "Starts mirroring the tab to the sink."]
pub struct StartTabMirroringReturnObject(pub Option<Json>);
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[doc = "Stops the active Cast session on the sink."]
pub struct StopCastingReturnObject(pub Option<Json>);
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
    use derive_builder::Builder;
    #[allow(unused_imports)]
    use serde::{Deserialize, Serialize};
    #[allow(unused_imports)]
    use serde_json::Value as Json;
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
    pub struct SinksUpdatedEvent {
        pub params: SinksUpdatedEventParams,
    }
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
    pub struct SinksUpdatedEventParams {
        pub sinks: Vec<super::Sink>,
    }
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
    pub struct IssueUpdatedEvent {
        pub params: IssueUpdatedEventParams,
    }
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
    pub struct IssueUpdatedEventParams {
        #[serde(default)]
        pub issue_message: String,
    }
}
