// Auto-generated from Chrome at version 146.0.7680.165 domain: Media
#![allow(dead_code)]
use super::dom;
#[allow(unused_imports)]
use super::types::*;
#[allow(unused_imports)]
use derive_builder::Builder;
#[allow(unused_imports)]
use serde::{Deserialize, Serialize};
#[allow(unused_imports)]
use serde_json::Value as Json;
pub type PlayerId = String;
pub type Timestamp = JsFloat;
#[allow(deprecated)]
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub enum PlayerMessageLevel {
    #[serde(rename = "error")]
    Error,
    #[serde(rename = "warning")]
    Warning,
    #[serde(rename = "info")]
    Info,
    #[serde(rename = "debug")]
    Debug,
}
#[allow(deprecated)]
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "camelCase")]
#[doc = "Have one type per entry in MediaLogRecord::Type\n Corresponds to kMessage"]
pub struct PlayerMessage {
    #[doc = "Keep in sync with MediaLogMessageLevel\n We are currently keeping the message level 'error' separate from the\n PlayerError type because right now they represent different things,\n this one being a DVLOG(ERROR) style log message that gets printed\n based on what log level is selected in the UI, and the other is a\n representation of a media::PipelineStatus object. Soon however we're\n going to be moving away from using PipelineStatus for errors and\n introducing a new error type which should hopefully let us integrate\n the error log level into the PlayerError type."]
    pub level: PlayerMessageLevel,
    #[serde(default)]
    pub message: String,
}
#[allow(deprecated)]
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "camelCase")]
#[doc = "Corresponds to kMediaPropertyChange"]
pub struct PlayerProperty {
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    pub value: String,
}
#[allow(deprecated)]
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "camelCase")]
#[doc = "Corresponds to kMediaEventTriggered"]
pub struct PlayerEvent {
    pub timestamp: Timestamp,
    #[serde(default)]
    pub value: String,
}
#[allow(deprecated)]
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "camelCase")]
#[doc = "Represents logged source line numbers reported in an error.\n NOTE: file and line are from chromium c++ implementation code, not js."]
pub struct PlayerErrorSourceLocation {
    #[serde(default)]
    pub file: String,
    #[serde(default)]
    pub line: JsUInt,
}
#[allow(deprecated)]
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "camelCase")]
#[doc = "Corresponds to kMediaError"]
pub struct PlayerError {
    #[serde(default)]
    pub error_type: String,
    #[serde(default)]
    #[doc = "Code is the numeric enum entry for a specific set of error codes, such\n as PipelineStatusCodes in media/base/pipeline_status.h"]
    pub code: JsUInt,
    #[doc = "A trace of where this error was caused / where it passed through."]
    pub stack: Vec<PlayerErrorSourceLocation>,
    #[doc = "Errors potentially have a root cause error, ie, a DecoderError might be\n caused by an WindowsError"]
    pub cause: Vec<PlayerError>,
    #[serde(default)]
    #[doc = "Extra data attached to an error, such as an HRESULT, Video Codec, etc."]
    pub data: Json,
}
#[allow(deprecated)]
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "camelCase")]
pub struct Player {
    pub player_id: PlayerId,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dom_node_id: Option<dom::BackendNodeId>,
}
#[allow(deprecated)]
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct Enable(pub Option<Json>);
#[allow(deprecated)]
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct Disable(pub Option<Json>);
#[allow(deprecated)]
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[doc = "Enables the Media domain"]
pub struct EnableReturnObject(pub Option<Json>);
#[allow(deprecated)]
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[doc = "Disables the Media domain."]
pub struct DisableReturnObject(pub Option<Json>);
#[allow(deprecated)]
impl Method for Enable {
    const NAME: &'static str = "Media.enable";
    type ReturnObject = EnableReturnObject;
}
#[allow(deprecated)]
impl Method for Disable {
    const NAME: &'static str = "Media.disable";
    type ReturnObject = DisableReturnObject;
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
    pub struct PlayerPropertiesChangedEvent {
        pub params: PlayerPropertiesChangedEventParams,
    }
    #[allow(deprecated)]
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
    #[serde(rename_all = "camelCase")]
    pub struct PlayerPropertiesChangedEventParams {
        pub player_id: super::PlayerId,
        pub properties: Vec<super::PlayerProperty>,
    }
    #[allow(deprecated)]
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
    pub struct PlayerEventsAddedEvent {
        pub params: PlayerEventsAddedEventParams,
    }
    #[allow(deprecated)]
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
    #[serde(rename_all = "camelCase")]
    pub struct PlayerEventsAddedEventParams {
        pub player_id: super::PlayerId,
        pub events: Vec<super::PlayerEvent>,
    }
    #[allow(deprecated)]
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
    pub struct PlayerMessagesLoggedEvent {
        pub params: PlayerMessagesLoggedEventParams,
    }
    #[allow(deprecated)]
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
    #[serde(rename_all = "camelCase")]
    pub struct PlayerMessagesLoggedEventParams {
        pub player_id: super::PlayerId,
        pub messages: Vec<super::PlayerMessage>,
    }
    #[allow(deprecated)]
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
    pub struct PlayerErrorsRaisedEvent {
        pub params: PlayerErrorsRaisedEventParams,
    }
    #[allow(deprecated)]
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
    #[serde(rename_all = "camelCase")]
    pub struct PlayerErrorsRaisedEventParams {
        pub player_id: super::PlayerId,
        pub errors: Vec<super::PlayerError>,
    }
    #[allow(deprecated)]
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
    pub struct PlayerCreatedEvent {
        pub params: PlayerCreatedEventParams,
    }
    #[allow(deprecated)]
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
    #[serde(rename_all = "camelCase")]
    pub struct PlayerCreatedEventParams {
        pub player: super::Player,
    }
}
