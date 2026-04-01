// Auto-generated from Chrome at version 146.0.7680.165 domain: Console
#[allow(unused_imports)]
use super::types::*;
#[allow(unused_imports)]
use derive_builder::Builder;
#[allow(unused_imports)]
use serde::{Deserialize, Serialize};
#[allow(unused_imports)]
use serde_json::Value as Json;
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub enum ConsoleMessageSource {
    #[serde(rename = "xml")]
    Xml,
    #[serde(rename = "javascript")]
    Javascript,
    #[serde(rename = "network")]
    Network,
    #[serde(rename = "console-api")]
    ConsoleApi,
    #[serde(rename = "storage")]
    Storage,
    #[serde(rename = "appcache")]
    Appcache,
    #[serde(rename = "rendering")]
    Rendering,
    #[serde(rename = "security")]
    Security,
    #[serde(rename = "other")]
    Other,
    #[serde(rename = "deprecation")]
    Deprecation,
    #[serde(rename = "worker")]
    Worker,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub enum ConsoleMessageLevel {
    #[serde(rename = "log")]
    Log,
    #[serde(rename = "warning")]
    Warning,
    #[serde(rename = "error")]
    Error,
    #[serde(rename = "debug")]
    Debug,
    #[serde(rename = "info")]
    Info,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "camelCase")]
#[doc = "Console message."]
pub struct ConsoleMessage {
    #[doc = "Message source."]
    pub source: ConsoleMessageSource,
    #[doc = "Message severity."]
    pub level: ConsoleMessageLevel,
    #[serde(default)]
    #[doc = "Message text."]
    pub text: String,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[doc = "URL of the message origin."]
    pub url: Option<String>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[doc = "Line number in the resource that generated this message (1-based)."]
    pub line: Option<JsUInt>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[doc = "Column number in the resource that generated this message (1-based)."]
    pub column: Option<JsUInt>,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct ClearMessages(pub Option<Json>);
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct Disable(pub Option<Json>);
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct Enable(pub Option<Json>);
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[doc = "Does nothing."]
pub struct ClearMessagesReturnObject(pub Option<Json>);
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[doc = "Disables console domain, prevents further console messages from being reported to the client."]
pub struct DisableReturnObject(pub Option<Json>);
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[doc = "Enables console domain, sends the messages collected so far to the client by means of the\n `messageAdded` notification."]
pub struct EnableReturnObject(pub Option<Json>);
impl Method for ClearMessages {
    const NAME: &'static str = "Console.clearMessages";
    type ReturnObject = ClearMessagesReturnObject;
}
impl Method for Disable {
    const NAME: &'static str = "Console.disable";
    type ReturnObject = DisableReturnObject;
}
impl Method for Enable {
    const NAME: &'static str = "Console.enable";
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
    pub struct MessageAddedEvent {
        pub params: MessageAddedEventParams,
    }
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
    #[serde(rename_all = "camelCase")]
    pub struct MessageAddedEventParams {
        #[doc = "Console message that has been added."]
        pub message: super::ConsoleMessage,
    }
}
