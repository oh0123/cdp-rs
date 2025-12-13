// Auto-generated from Chrome at version 143.0.7499.110 domain: Target
use super::browser;
use super::page;
#[allow(unused_imports)]
use super::types::*;
#[allow(unused_imports)]
use serde::{Deserialize, Serialize};
#[allow(unused_imports)]
use serde_json::Value as Json;
pub type TargetId = String;
pub type SessionId = String;
pub type TargetFilter = Vec<FilterEntry>;
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub enum WindowState {
    #[serde(rename = "normal")]
    Normal,
    #[serde(rename = "minimized")]
    Minimized,
    #[serde(rename = "maximized")]
    Maximized,
    #[serde(rename = "fullscreen")]
    Fullscreen,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct TargetInfo {
    #[serde(rename = "targetId")]
    pub target_id: TargetId,
    #[serde(default)]
    #[serde(rename = "type")]
    pub r#type: String,
    #[serde(default)]
    #[serde(rename = "title")]
    pub title: String,
    #[serde(default)]
    #[serde(rename = "url")]
    pub url: String,
    #[serde(default)]
    #[serde(rename = "attached")]
    pub attached: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "openerId")]
    pub opener_id: Option<TargetId>,
    #[serde(default)]
    #[serde(rename = "canAccessOpener")]
    pub can_access_opener: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "openerFrameId")]
    pub opener_frame_id: Option<page::FrameId>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "parentFrameId")]
    pub parent_frame_id: Option<page::FrameId>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "browserContextId")]
    pub browser_context_id: Option<browser::BrowserContextId>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(rename = "subtype")]
    pub subtype: Option<String>,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct FilterEntry {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(rename = "exclude")]
    pub exclude: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(rename = "type")]
    pub r#type: Option<String>,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct RemoteLocation {
    #[serde(default)]
    #[serde(rename = "host")]
    pub host: String,
    #[serde(default)]
    #[serde(rename = "port")]
    pub port: JsUInt,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct ActivateTarget {
    #[serde(rename = "targetId")]
    pub target_id: TargetId,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct AttachToTarget {
    #[serde(rename = "targetId")]
    pub target_id: TargetId,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(rename = "flatten")]
    pub flatten: Option<bool>,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct AttachToBrowserTarget(pub Option<serde_json::Value>);
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct CloseTarget {
    #[serde(rename = "targetId")]
    pub target_id: TargetId,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct ExposeDevToolsProtocol {
    #[serde(rename = "targetId")]
    pub target_id: TargetId,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(rename = "bindingName")]
    pub binding_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(rename = "inheritPermissions")]
    pub inherit_permissions: Option<bool>,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct CreateBrowserContext {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(rename = "disposeOnDetach")]
    pub dispose_on_detach: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(rename = "proxyServer")]
    pub proxy_server: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(rename = "proxyBypassList")]
    pub proxy_bypass_list: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(rename = "originsWithUniversalNetworkAccess")]
    pub origins_with_universal_network_access: Option<Vec<String>>,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct GetBrowserContexts(pub Option<serde_json::Value>);
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct CreateTarget {
    #[serde(default)]
    #[serde(rename = "url")]
    pub url: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(rename = "left")]
    pub left: Option<JsUInt>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(rename = "top")]
    pub top: Option<JsUInt>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(rename = "width")]
    pub width: Option<JsUInt>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(rename = "height")]
    pub height: Option<JsUInt>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "windowState")]
    pub window_state: Option<WindowState>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "browserContextId")]
    pub browser_context_id: Option<browser::BrowserContextId>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(rename = "enableBeginFrameControl")]
    pub enable_begin_frame_control: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(rename = "newWindow")]
    pub new_window: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(rename = "background")]
    pub background: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(rename = "forTab")]
    pub for_tab: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(rename = "hidden")]
    pub hidden: Option<bool>,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct DetachFromTarget {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "sessionId")]
    pub session_id: Option<SessionId>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "targetId")]
    pub target_id: Option<TargetId>,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct DisposeBrowserContext {
    #[serde(rename = "browserContextId")]
    pub browser_context_id: browser::BrowserContextId,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct GetTargetInfo {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "targetId")]
    pub target_id: Option<TargetId>,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct GetTargets {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "filter")]
    pub filter: Option<TargetFilter>,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct SendMessageToTarget {
    #[serde(default)]
    #[serde(rename = "message")]
    pub message: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "sessionId")]
    pub session_id: Option<SessionId>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "targetId")]
    pub target_id: Option<TargetId>,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct SetAutoAttach {
    #[serde(default)]
    #[serde(rename = "autoAttach")]
    pub auto_attach: bool,
    #[serde(default)]
    #[serde(rename = "waitForDebuggerOnStart")]
    pub wait_for_debugger_on_start: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(rename = "flatten")]
    pub flatten: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "filter")]
    pub filter: Option<TargetFilter>,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct AutoAttachRelated {
    #[serde(rename = "targetId")]
    pub target_id: TargetId,
    #[serde(default)]
    #[serde(rename = "waitForDebuggerOnStart")]
    pub wait_for_debugger_on_start: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "filter")]
    pub filter: Option<TargetFilter>,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct SetDiscoverTargets {
    #[serde(default)]
    #[serde(rename = "discover")]
    pub discover: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "filter")]
    pub filter: Option<TargetFilter>,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct SetRemoteLocations {
    #[serde(rename = "locations")]
    pub locations: Vec<RemoteLocation>,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct OpenDevTools {
    #[serde(rename = "targetId")]
    pub target_id: TargetId,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct ActivateTargetReturnObject {}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct AttachToTargetReturnObject {
    #[serde(rename = "sessionId")]
    pub session_id: SessionId,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct AttachToBrowserTargetReturnObject {
    #[serde(rename = "sessionId")]
    pub session_id: SessionId,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct CloseTargetReturnObject {
    #[serde(default)]
    #[serde(rename = "success")]
    pub success: bool,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct ExposeDevToolsProtocolReturnObject {}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct CreateBrowserContextReturnObject {
    #[serde(rename = "browserContextId")]
    pub browser_context_id: browser::BrowserContextId,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct GetBrowserContextsReturnObject {
    #[serde(rename = "browserContextIds")]
    pub browser_context_ids: browser::BrowserContextId,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct CreateTargetReturnObject {
    #[serde(rename = "targetId")]
    pub target_id: TargetId,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct DetachFromTargetReturnObject {}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct DisposeBrowserContextReturnObject {}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct GetTargetInfoReturnObject {
    #[serde(rename = "targetInfo")]
    pub target_info: TargetInfo,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct GetTargetsReturnObject {
    #[serde(rename = "targetInfos")]
    pub target_infos: Vec<TargetInfo>,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct SendMessageToTargetReturnObject {}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct SetAutoAttachReturnObject {}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct AutoAttachRelatedReturnObject {}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct SetDiscoverTargetsReturnObject {}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct SetRemoteLocationsReturnObject {}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct OpenDevToolsReturnObject {
    #[serde(rename = "targetId")]
    pub target_id: TargetId,
}
impl Method for ActivateTarget {
    const NAME: &'static str = "Target.activateTarget";
    type ReturnObject = ActivateTargetReturnObject;
}
impl Method for AttachToTarget {
    const NAME: &'static str = "Target.attachToTarget";
    type ReturnObject = AttachToTargetReturnObject;
}
impl Method for AttachToBrowserTarget {
    const NAME: &'static str = "Target.attachToBrowserTarget";
    type ReturnObject = AttachToBrowserTargetReturnObject;
}
impl Method for CloseTarget {
    const NAME: &'static str = "Target.closeTarget";
    type ReturnObject = CloseTargetReturnObject;
}
impl Method for ExposeDevToolsProtocol {
    const NAME: &'static str = "Target.exposeDevToolsProtocol";
    type ReturnObject = ExposeDevToolsProtocolReturnObject;
}
impl Method for CreateBrowserContext {
    const NAME: &'static str = "Target.createBrowserContext";
    type ReturnObject = CreateBrowserContextReturnObject;
}
impl Method for GetBrowserContexts {
    const NAME: &'static str = "Target.getBrowserContexts";
    type ReturnObject = GetBrowserContextsReturnObject;
}
impl Method for CreateTarget {
    const NAME: &'static str = "Target.createTarget";
    type ReturnObject = CreateTargetReturnObject;
}
impl Method for DetachFromTarget {
    const NAME: &'static str = "Target.detachFromTarget";
    type ReturnObject = DetachFromTargetReturnObject;
}
impl Method for DisposeBrowserContext {
    const NAME: &'static str = "Target.disposeBrowserContext";
    type ReturnObject = DisposeBrowserContextReturnObject;
}
impl Method for GetTargetInfo {
    const NAME: &'static str = "Target.getTargetInfo";
    type ReturnObject = GetTargetInfoReturnObject;
}
impl Method for GetTargets {
    const NAME: &'static str = "Target.getTargets";
    type ReturnObject = GetTargetsReturnObject;
}
impl Method for SendMessageToTarget {
    const NAME: &'static str = "Target.sendMessageToTarget";
    type ReturnObject = SendMessageToTargetReturnObject;
}
impl Method for SetAutoAttach {
    const NAME: &'static str = "Target.setAutoAttach";
    type ReturnObject = SetAutoAttachReturnObject;
}
impl Method for AutoAttachRelated {
    const NAME: &'static str = "Target.autoAttachRelated";
    type ReturnObject = AutoAttachRelatedReturnObject;
}
impl Method for SetDiscoverTargets {
    const NAME: &'static str = "Target.setDiscoverTargets";
    type ReturnObject = SetDiscoverTargetsReturnObject;
}
impl Method for SetRemoteLocations {
    const NAME: &'static str = "Target.setRemoteLocations";
    type ReturnObject = SetRemoteLocationsReturnObject;
}
impl Method for OpenDevTools {
    const NAME: &'static str = "Target.openDevTools";
    type ReturnObject = OpenDevToolsReturnObject;
}
pub mod events {
    #[allow(unused_imports)]
    use super::super::types::*;
    #[allow(unused_imports)]
    use serde::{Deserialize, Serialize};
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
    pub struct AttachedToTargetEvent {
        pub params: AttachedToTargetEventParams,
    }
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
    pub struct AttachedToTargetEventParams {
        #[serde(rename = "sessionId")]
        pub session_id: super::SessionId,
        #[serde(rename = "targetInfo")]
        pub target_info: super::TargetInfo,
        #[serde(default)]
        #[serde(rename = "waitingForDebugger")]
        pub waiting_for_debugger: bool,
    }
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
    pub struct DetachedFromTargetEvent {
        pub params: DetachedFromTargetEventParams,
    }
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
    pub struct DetachedFromTargetEventParams {
        #[serde(rename = "sessionId")]
        pub session_id: super::SessionId,
        #[serde(skip_serializing_if = "Option::is_none")]
        #[serde(rename = "targetId")]
        pub target_id: Option<super::TargetId>,
    }
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
    pub struct ReceivedMessageFromTargetEvent {
        pub params: ReceivedMessageFromTargetEventParams,
    }
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
    pub struct ReceivedMessageFromTargetEventParams {
        #[serde(rename = "sessionId")]
        pub session_id: super::SessionId,
        #[serde(default)]
        #[serde(rename = "message")]
        pub message: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        #[serde(rename = "targetId")]
        pub target_id: Option<super::TargetId>,
    }
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
    pub struct TargetCreatedEvent {
        pub params: TargetCreatedEventParams,
    }
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
    pub struct TargetCreatedEventParams {
        #[serde(rename = "targetInfo")]
        pub target_info: super::TargetInfo,
    }
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
    pub struct TargetDestroyedEvent {
        pub params: TargetDestroyedEventParams,
    }
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
    pub struct TargetDestroyedEventParams {
        #[serde(rename = "targetId")]
        pub target_id: super::TargetId,
    }
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
    pub struct TargetCrashedEvent {
        pub params: TargetCrashedEventParams,
    }
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
    pub struct TargetCrashedEventParams {
        #[serde(rename = "targetId")]
        pub target_id: super::TargetId,
        #[serde(default)]
        #[serde(rename = "status")]
        pub status: String,
        #[serde(default)]
        #[serde(rename = "errorCode")]
        pub error_code: JsUInt,
    }
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
    pub struct TargetInfoChangedEvent {
        pub params: TargetInfoChangedEventParams,
    }
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
    pub struct TargetInfoChangedEventParams {
        #[serde(rename = "targetInfo")]
        pub target_info: super::TargetInfo,
    }
}
