// Auto-generated from Chrome at version 143.0.7499.110 domain: Media
use super::dom;
#[allow(unused_imports)]
use super::types::*;
#[allow(unused_imports)]
use serde::{Deserialize, Serialize};
#[allow(unused_imports)]
use serde_json::Value as Json;
pub type PlayerId = String;
pub type Timestamp = JsFloat;
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
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct PlayerMessage {
    #[serde(rename = "level")]
    pub level: PlayerMessageLevel,
    #[serde(default)]
    #[serde(rename = "message")]
    pub message: String,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct PlayerProperty {
    #[serde(default)]
    #[serde(rename = "name")]
    pub name: String,
    #[serde(default)]
    #[serde(rename = "value")]
    pub value: String,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct PlayerEvent {
    #[serde(rename = "timestamp")]
    pub timestamp: Timestamp,
    #[serde(default)]
    #[serde(rename = "value")]
    pub value: String,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct PlayerErrorSourceLocation {
    #[serde(default)]
    #[serde(rename = "file")]
    pub file: String,
    #[serde(default)]
    #[serde(rename = "line")]
    pub line: JsUInt,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct PlayerError {
    #[serde(default)]
    #[serde(rename = "errorType")]
    pub error_type: String,
    #[serde(default)]
    #[serde(rename = "code")]
    pub code: JsUInt,
    #[serde(rename = "stack")]
    pub stack: Vec<PlayerErrorSourceLocation>,
    #[serde(rename = "cause")]
    pub cause: Vec<PlayerError>,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct Player {
    #[serde(rename = "playerId")]
    pub player_id: PlayerId,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "domNodeId")]
    pub dom_node_id: Option<dom::BackendNodeId>,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Enable(pub Option<serde_json::Value>);
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Disable(pub Option<serde_json::Value>);
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct EnableReturnObject {}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct DisableReturnObject {}
impl Method for Enable {
    const NAME: &'static str = "Media.enable";
    type ReturnObject = EnableReturnObject;
}
impl Method for Disable {
    const NAME: &'static str = "Media.disable";
    type ReturnObject = DisableReturnObject;
}
pub mod events {
    #[allow(unused_imports)]
    use super::super::types::*;
    #[allow(unused_imports)]
    use serde::{Deserialize, Serialize};
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
    pub struct PlayerPropertiesChangedEvent {
        pub params: PlayerPropertiesChangedEventParams,
    }
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
    pub struct PlayerPropertiesChangedEventParams {
        #[serde(rename = "playerId")]
        pub player_id: super::PlayerId,
        #[serde(rename = "properties")]
        pub properties: Vec<super::PlayerProperty>,
    }
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
    pub struct PlayerEventsAddedEvent {
        pub params: PlayerEventsAddedEventParams,
    }
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
    pub struct PlayerEventsAddedEventParams {
        #[serde(rename = "playerId")]
        pub player_id: super::PlayerId,
        #[serde(rename = "events")]
        pub events: Vec<super::PlayerEvent>,
    }
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
    pub struct PlayerMessagesLoggedEvent {
        pub params: PlayerMessagesLoggedEventParams,
    }
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
    pub struct PlayerMessagesLoggedEventParams {
        #[serde(rename = "playerId")]
        pub player_id: super::PlayerId,
        #[serde(rename = "messages")]
        pub messages: Vec<super::PlayerMessage>,
    }
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
    pub struct PlayerErrorsRaisedEvent {
        pub params: PlayerErrorsRaisedEventParams,
    }
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
    pub struct PlayerErrorsRaisedEventParams {
        #[serde(rename = "playerId")]
        pub player_id: super::PlayerId,
        #[serde(rename = "errors")]
        pub errors: Vec<super::PlayerError>,
    }
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
    pub struct PlayerCreatedEvent {
        pub params: PlayerCreatedEventParams,
    }
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
    pub struct PlayerCreatedEventParams {
        #[serde(rename = "player")]
        pub player: super::Player,
    }
}
