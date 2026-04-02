// Auto-generated from Chrome at version 146.0.7680.165 domain: Target
#![allow(dead_code)]
use super::browser;
use super::page;
#[allow(unused_imports)]
use super::types::*;
#[allow(unused_imports)]
use derive_builder::Builder;
#[allow(unused_imports)]
use serde::{Deserialize, Serialize};
#[allow(unused_imports)]
use serde_json::Value as Json;
pub type TargetId = String;
pub type SessionId = String;
pub type TargetFilter = Vec<FilterEntry>;
#[allow(deprecated)]
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
#[allow(deprecated)]
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "camelCase")]
pub struct TargetInfo {
    pub target_id: TargetId,
    #[serde(default)]
    #[doc = "List of types: <https://source.chromium.org/chromium/chromium/src/+/main:content/browser/devtools/devtools_agent_host_impl.cc?ss=chromium&q=f:devtools%20-f:out%20%22::kTypeTab%5B%5D%22>"]
    pub r#type: String,
    #[serde(default)]
    pub title: String,
    #[serde(default)]
    pub url: String,
    #[serde(default)]
    #[doc = "Whether the target has an attached client."]
    pub attached: bool,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[doc = "Opener target Id"]
    pub opener_id: Option<TargetId>,
    #[serde(default)]
    #[doc = "Whether the target has access to the originating window."]
    pub can_access_opener: bool,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[doc = "Frame id of originating window (is only set if target has an opener)."]
    pub opener_frame_id: Option<page::FrameId>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[doc = "Id of the parent frame, only present for the \"iframe\" targets."]
    pub parent_frame_id: Option<page::FrameId>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub browser_context_id: Option<browser::BrowserContextId>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[doc = "Provides additional details for specific target types. For example, for\n the type of \"page\", this may be set to \"prerender\"."]
    pub subtype: Option<String>,
}
#[allow(deprecated)]
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "camelCase")]
#[doc = "A filter used by target query/discovery/auto-attach operations."]
pub struct FilterEntry {
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[doc = "If set, causes exclusion of matching targets from the list."]
    pub exclude: Option<bool>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[doc = "If not present, matches any type."]
    pub r#type: Option<String>,
}
#[allow(deprecated)]
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "camelCase")]
pub struct RemoteLocation {
    #[serde(default)]
    pub host: String,
    #[serde(default)]
    pub port: JsUInt,
}
#[allow(deprecated)]
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "camelCase")]
#[doc = "Activates (focuses) the target."]
pub struct ActivateTarget {
    pub target_id: TargetId,
}
#[allow(deprecated)]
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "camelCase")]
#[doc = "Attaches to the target with given id."]
pub struct AttachToTarget {
    pub target_id: TargetId,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[doc = "Enables \"flat\" access to the session via specifying sessionId attribute in the commands.\n We plan to make this the default, deprecate non-flattened mode,\n and eventually retire it. See crbug.com/991325."]
    pub flatten: Option<bool>,
}
#[allow(deprecated)]
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct AttachToBrowserTarget(pub Option<Json>);
#[allow(deprecated)]
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "camelCase")]
#[doc = "Closes the target. If the target is a page that gets closed too."]
pub struct CloseTarget {
    pub target_id: TargetId,
}
#[allow(deprecated)]
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "camelCase")]
#[doc = "Inject object to the target's main frame that provides a communication\n channel with browser target.\n \n Injected object will be available as `window\\[bindingName\\]`.\n \n The object has the following API:\n - `binding.send(json)` - a method to send messages over the remote debugging protocol\n - `binding.onmessage = json =\\> handleMessage(json)` - a callback that will be called for the protocol notifications and command responses."]
pub struct ExposeDevToolsProtocol {
    pub target_id: TargetId,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[doc = "Binding name, 'cdp' if not specified."]
    pub binding_name: Option<String>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[doc = "If true, inherits the current root session's permissions (default: false)."]
    pub inherit_permissions: Option<bool>,
}
#[allow(deprecated)]
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "camelCase")]
#[doc = "Creates a new empty BrowserContext. Similar to an incognito profile but you can have more than\n one."]
pub struct CreateBrowserContext {
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[doc = "If specified, disposes this context when debugging session disconnects."]
    pub dispose_on_detach: Option<bool>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[doc = "Proxy server, similar to the one passed to --proxy-server"]
    pub proxy_server: Option<String>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[doc = "Proxy bypass list, similar to the one passed to --proxy-bypass-list"]
    pub proxy_bypass_list: Option<String>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[doc = "An optional list of origins to grant unlimited cross-origin access to.\n Parts of the URL other than those constituting origin are ignored."]
    pub origins_with_universal_network_access: Option<Vec<String>>,
}
#[allow(deprecated)]
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct GetBrowserContexts(pub Option<Json>);
#[allow(deprecated)]
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "camelCase")]
#[doc = "Creates a new page."]
pub struct CreateTarget {
    #[serde(default)]
    #[doc = "The initial URL the page will be navigated to. An empty string indicates about:blank."]
    pub url: String,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[doc = "Frame left origin in DIP (requires newWindow to be true or headless shell)."]
    pub left: Option<JsUInt>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[doc = "Frame top origin in DIP (requires newWindow to be true or headless shell)."]
    pub top: Option<JsUInt>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[doc = "Frame width in DIP (requires newWindow to be true or headless shell)."]
    pub width: Option<JsUInt>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[doc = "Frame height in DIP (requires newWindow to be true or headless shell)."]
    pub height: Option<JsUInt>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[doc = "Frame window state (requires newWindow to be true or headless shell).\n Default is normal."]
    pub window_state: Option<WindowState>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[doc = "The browser context to create the page in."]
    pub browser_context_id: Option<browser::BrowserContextId>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[doc = "Whether BeginFrames for this target will be controlled via DevTools (headless shell only,\n not supported on MacOS yet, false by default)."]
    pub enable_begin_frame_control: Option<bool>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[doc = "Whether to create a new Window or Tab (false by default, not supported by headless shell)."]
    pub new_window: Option<bool>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[doc = "Whether to create the target in background or foreground (false by default, not supported\n by headless shell)."]
    pub background: Option<bool>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[doc = "Whether to create the target of type \"tab\"."]
    pub for_tab: Option<bool>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[doc = "Whether to create a hidden target. The hidden target is observable via protocol, but not\n present in the tab UI strip. Cannot be created with `forTab: true`, `newWindow: true` or\n `background: false`. The life-time of the tab is limited to the life-time of the session."]
    pub hidden: Option<bool>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[doc = "If specified, the option is used to determine if the new target should\n be focused or not. By default, the focus behavior depends on the\n value of the background field. For example, background=false and focus=false\n will result in the target tab being opened but the browser window remain\n unchanged (if it was in the background, it will remain in the background)\n and background=false with focus=undefined will result in the window being focused.\n Using background: true and focus: true is not supported and will result in an error."]
    pub focus: Option<bool>,
}
#[allow(deprecated)]
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "camelCase")]
#[doc = "Detaches session with given id."]
pub struct DetachFromTarget {
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[doc = "Session to detach."]
    pub session_id: Option<SessionId>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[doc = "Deprecated."]
    #[deprecated]
    pub target_id: Option<TargetId>,
}
#[allow(deprecated)]
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "camelCase")]
#[doc = "Deletes a BrowserContext. All the belonging pages will be closed without calling their\n beforeunload hooks."]
pub struct DisposeBrowserContext {
    pub browser_context_id: browser::BrowserContextId,
}
#[allow(deprecated)]
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "camelCase")]
#[doc = "Returns information about a target."]
pub struct GetTargetInfo {
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_id: Option<TargetId>,
}
#[allow(deprecated)]
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "camelCase")]
#[doc = "Retrieves a list of available targets."]
pub struct GetTargets {
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[doc = "Only targets matching filter will be reported. If filter is not specified\n and target discovery is currently enabled, a filter used for target discovery\n is used for consistency."]
    pub filter: Option<TargetFilter>,
}
#[allow(deprecated)]
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "camelCase")]
#[doc = "Sends protocol message over session with given id.\n Consider using flat mode instead; see commands attachToTarget, setAutoAttach,\n and crbug.com/991325."]
#[deprecated]
pub struct SendMessageToTarget {
    #[serde(default)]
    pub message: String,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[doc = "Identifier of the session."]
    pub session_id: Option<SessionId>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[doc = "Deprecated."]
    #[deprecated]
    pub target_id: Option<TargetId>,
}
#[allow(deprecated)]
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "camelCase")]
#[doc = "Controls whether to automatically attach to new targets which are considered\n to be directly related to this one (for example, iframes or workers).\n When turned on, attaches to all existing related targets as well. When turned off,\n automatically detaches from all currently attached targets.\n This also clears all targets added by `autoAttachRelated` from the list of targets to watch\n for creation of related targets.\n You might want to call this recursively for auto-attached targets to attach\n to all available targets."]
pub struct SetAutoAttach {
    #[serde(default)]
    #[doc = "Whether to auto-attach to related targets."]
    pub auto_attach: bool,
    #[serde(default)]
    #[doc = "Whether to pause new targets when attaching to them. Use `Runtime.runIfWaitingForDebugger`\n to run paused targets."]
    pub wait_for_debugger_on_start: bool,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[doc = "Enables \"flat\" access to the session via specifying sessionId attribute in the commands.\n We plan to make this the default, deprecate non-flattened mode,\n and eventually retire it. See crbug.com/991325."]
    pub flatten: Option<bool>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[doc = "Only targets matching filter will be attached."]
    pub filter: Option<TargetFilter>,
}
#[allow(deprecated)]
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "camelCase")]
#[doc = "Adds the specified target to the list of targets that will be monitored for any related target\n creation (such as child frames, child workers and new versions of service worker) and reported\n through `attachedToTarget`. The specified target is also auto-attached.\n This cancels the effect of any previous `setAutoAttach` and is also cancelled by subsequent\n `setAutoAttach`. Only available at the Browser target."]
pub struct AutoAttachRelated {
    pub target_id: TargetId,
    #[serde(default)]
    #[doc = "Whether to pause new targets when attaching to them. Use `Runtime.runIfWaitingForDebugger`\n to run paused targets."]
    pub wait_for_debugger_on_start: bool,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[doc = "Only targets matching filter will be attached."]
    pub filter: Option<TargetFilter>,
}
#[allow(deprecated)]
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "camelCase")]
#[doc = "Controls whether to discover available targets and notify via\n `targetCreated/targetInfoChanged/targetDestroyed` events."]
pub struct SetDiscoverTargets {
    #[serde(default)]
    #[doc = "Whether to discover available targets."]
    pub discover: bool,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[doc = "Only targets matching filter will be attached. If `discover` is false,\n `filter` must be omitted or empty."]
    pub filter: Option<TargetFilter>,
}
#[allow(deprecated)]
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "camelCase")]
#[doc = "Enables target discovery for the specified locations, when `setDiscoverTargets` was set to\n `true`."]
pub struct SetRemoteLocations {
    #[doc = "List of remote locations."]
    pub locations: Vec<RemoteLocation>,
}
#[allow(deprecated)]
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "camelCase")]
#[doc = "Gets the targetId of the DevTools page target opened for the given target\n (if any)."]
pub struct GetDevToolsTarget {
    #[doc = "Page or tab target ID."]
    pub target_id: TargetId,
}
#[allow(deprecated)]
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "camelCase")]
#[doc = "Opens a DevTools window for the target."]
pub struct OpenDevTools {
    #[doc = "This can be the page or tab target ID."]
    pub target_id: TargetId,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[doc = "The id of the panel we want DevTools to open initially. Currently\n supported panels are elements, console, network, sources, resources\n and performance."]
    pub panel_id: Option<String>,
}
#[allow(deprecated)]
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[doc = "Activates (focuses) the target."]
pub struct ActivateTargetReturnObject(pub Option<Json>);
#[allow(deprecated)]
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[serde(rename_all = "camelCase")]
#[doc = "Attaches to the target with given id."]
pub struct AttachToTargetReturnObject {
    #[doc = "Id assigned to the session."]
    pub session_id: SessionId,
}
#[allow(deprecated)]
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[serde(rename_all = "camelCase")]
#[doc = "Attaches to the browser target, only uses flat sessionId mode."]
pub struct AttachToBrowserTargetReturnObject {
    #[doc = "Id assigned to the session."]
    pub session_id: SessionId,
}
#[allow(deprecated)]
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[serde(rename_all = "camelCase")]
#[doc = "Closes the target. If the target is a page that gets closed too."]
pub struct CloseTargetReturnObject {
    #[serde(default)]
    #[doc = "Always set to true. If an error occurs, the response indicates protocol error."]
    #[deprecated]
    pub success: bool,
}
#[allow(deprecated)]
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[doc = "Inject object to the target's main frame that provides a communication\n channel with browser target.\n \n Injected object will be available as `window\\[bindingName\\]`.\n \n The object has the following API:\n - `binding.send(json)` - a method to send messages over the remote debugging protocol\n - `binding.onmessage = json =\\> handleMessage(json)` - a callback that will be called for the protocol notifications and command responses."]
pub struct ExposeDevToolsProtocolReturnObject(pub Option<Json>);
#[allow(deprecated)]
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[serde(rename_all = "camelCase")]
#[doc = "Creates a new empty BrowserContext. Similar to an incognito profile but you can have more than\n one."]
pub struct CreateBrowserContextReturnObject {
    #[doc = "The id of the context created."]
    pub browser_context_id: browser::BrowserContextId,
}
#[allow(deprecated)]
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[serde(rename_all = "camelCase")]
#[doc = "Returns all browser contexts created with `Target.createBrowserContext` method."]
pub struct GetBrowserContextsReturnObject {
    #[doc = "An array of browser context ids."]
    pub browser_context_ids: browser::BrowserContextId,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[doc = "The id of the default browser context if available."]
    pub default_browser_context_id: Option<browser::BrowserContextId>,
}
#[allow(deprecated)]
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[serde(rename_all = "camelCase")]
#[doc = "Creates a new page."]
pub struct CreateTargetReturnObject {
    #[doc = "The id of the page opened."]
    pub target_id: TargetId,
}
#[allow(deprecated)]
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[doc = "Detaches session with given id."]
pub struct DetachFromTargetReturnObject(pub Option<Json>);
#[allow(deprecated)]
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[doc = "Deletes a BrowserContext. All the belonging pages will be closed without calling their\n beforeunload hooks."]
pub struct DisposeBrowserContextReturnObject(pub Option<Json>);
#[allow(deprecated)]
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[serde(rename_all = "camelCase")]
#[doc = "Returns information about a target."]
pub struct GetTargetInfoReturnObject {
    pub target_info: TargetInfo,
}
#[allow(deprecated)]
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[serde(rename_all = "camelCase")]
#[doc = "Retrieves a list of available targets."]
pub struct GetTargetsReturnObject {
    #[doc = "The list of targets."]
    pub target_infos: Vec<TargetInfo>,
}
#[allow(deprecated)]
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[doc = "Sends protocol message over session with given id.\n Consider using flat mode instead; see commands attachToTarget, setAutoAttach,\n and crbug.com/991325."]
#[deprecated]
pub struct SendMessageToTargetReturnObject(pub Option<Json>);
#[allow(deprecated)]
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[doc = "Controls whether to automatically attach to new targets which are considered\n to be directly related to this one (for example, iframes or workers).\n When turned on, attaches to all existing related targets as well. When turned off,\n automatically detaches from all currently attached targets.\n This also clears all targets added by `autoAttachRelated` from the list of targets to watch\n for creation of related targets.\n You might want to call this recursively for auto-attached targets to attach\n to all available targets."]
pub struct SetAutoAttachReturnObject(pub Option<Json>);
#[allow(deprecated)]
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[doc = "Adds the specified target to the list of targets that will be monitored for any related target\n creation (such as child frames, child workers and new versions of service worker) and reported\n through `attachedToTarget`. The specified target is also auto-attached.\n This cancels the effect of any previous `setAutoAttach` and is also cancelled by subsequent\n `setAutoAttach`. Only available at the Browser target."]
pub struct AutoAttachRelatedReturnObject(pub Option<Json>);
#[allow(deprecated)]
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[doc = "Controls whether to discover available targets and notify via\n `targetCreated/targetInfoChanged/targetDestroyed` events."]
pub struct SetDiscoverTargetsReturnObject(pub Option<Json>);
#[allow(deprecated)]
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[doc = "Enables target discovery for the specified locations, when `setDiscoverTargets` was set to\n `true`."]
pub struct SetRemoteLocationsReturnObject(pub Option<Json>);
#[allow(deprecated)]
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[serde(rename_all = "camelCase")]
#[doc = "Gets the targetId of the DevTools page target opened for the given target\n (if any)."]
pub struct GetDevToolsTargetReturnObject {
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[doc = "The targetId of DevTools page target if exists."]
    pub target_id: Option<TargetId>,
}
#[allow(deprecated)]
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[serde(rename_all = "camelCase")]
#[doc = "Opens a DevTools window for the target."]
pub struct OpenDevToolsReturnObject {
    #[doc = "The targetId of DevTools page target."]
    pub target_id: TargetId,
}
#[allow(deprecated)]
impl Method for ActivateTarget {
    const NAME: &'static str = "Target.activateTarget";
    type ReturnObject = ActivateTargetReturnObject;
}
#[allow(deprecated)]
impl Method for AttachToTarget {
    const NAME: &'static str = "Target.attachToTarget";
    type ReturnObject = AttachToTargetReturnObject;
}
#[allow(deprecated)]
impl Method for AttachToBrowserTarget {
    const NAME: &'static str = "Target.attachToBrowserTarget";
    type ReturnObject = AttachToBrowserTargetReturnObject;
}
#[allow(deprecated)]
impl Method for CloseTarget {
    const NAME: &'static str = "Target.closeTarget";
    type ReturnObject = CloseTargetReturnObject;
}
#[allow(deprecated)]
impl Method for ExposeDevToolsProtocol {
    const NAME: &'static str = "Target.exposeDevToolsProtocol";
    type ReturnObject = ExposeDevToolsProtocolReturnObject;
}
#[allow(deprecated)]
impl Method for CreateBrowserContext {
    const NAME: &'static str = "Target.createBrowserContext";
    type ReturnObject = CreateBrowserContextReturnObject;
}
#[allow(deprecated)]
impl Method for GetBrowserContexts {
    const NAME: &'static str = "Target.getBrowserContexts";
    type ReturnObject = GetBrowserContextsReturnObject;
}
#[allow(deprecated)]
impl Method for CreateTarget {
    const NAME: &'static str = "Target.createTarget";
    type ReturnObject = CreateTargetReturnObject;
}
#[allow(deprecated)]
impl Method for DetachFromTarget {
    const NAME: &'static str = "Target.detachFromTarget";
    type ReturnObject = DetachFromTargetReturnObject;
}
#[allow(deprecated)]
impl Method for DisposeBrowserContext {
    const NAME: &'static str = "Target.disposeBrowserContext";
    type ReturnObject = DisposeBrowserContextReturnObject;
}
#[allow(deprecated)]
impl Method for GetTargetInfo {
    const NAME: &'static str = "Target.getTargetInfo";
    type ReturnObject = GetTargetInfoReturnObject;
}
#[allow(deprecated)]
impl Method for GetTargets {
    const NAME: &'static str = "Target.getTargets";
    type ReturnObject = GetTargetsReturnObject;
}
#[allow(deprecated)]
impl Method for SendMessageToTarget {
    const NAME: &'static str = "Target.sendMessageToTarget";
    type ReturnObject = SendMessageToTargetReturnObject;
}
#[allow(deprecated)]
impl Method for SetAutoAttach {
    const NAME: &'static str = "Target.setAutoAttach";
    type ReturnObject = SetAutoAttachReturnObject;
}
#[allow(deprecated)]
impl Method for AutoAttachRelated {
    const NAME: &'static str = "Target.autoAttachRelated";
    type ReturnObject = AutoAttachRelatedReturnObject;
}
#[allow(deprecated)]
impl Method for SetDiscoverTargets {
    const NAME: &'static str = "Target.setDiscoverTargets";
    type ReturnObject = SetDiscoverTargetsReturnObject;
}
#[allow(deprecated)]
impl Method for SetRemoteLocations {
    const NAME: &'static str = "Target.setRemoteLocations";
    type ReturnObject = SetRemoteLocationsReturnObject;
}
#[allow(deprecated)]
impl Method for GetDevToolsTarget {
    const NAME: &'static str = "Target.getDevToolsTarget";
    type ReturnObject = GetDevToolsTargetReturnObject;
}
#[allow(deprecated)]
impl Method for OpenDevTools {
    const NAME: &'static str = "Target.openDevTools";
    type ReturnObject = OpenDevToolsReturnObject;
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
    pub struct AttachedToTargetEvent {
        pub params: AttachedToTargetEventParams,
    }
    #[allow(deprecated)]
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
    #[serde(rename_all = "camelCase")]
    pub struct AttachedToTargetEventParams {
        #[doc = "Identifier assigned to the session used to send/receive messages."]
        pub session_id: super::SessionId,
        pub target_info: super::TargetInfo,
        #[serde(default)]
        pub waiting_for_debugger: bool,
    }
    #[allow(deprecated)]
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
    pub struct DetachedFromTargetEvent {
        pub params: DetachedFromTargetEventParams,
    }
    #[allow(deprecated)]
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
    #[serde(rename_all = "camelCase")]
    pub struct DetachedFromTargetEventParams {
        #[doc = "Detached session identifier."]
        pub session_id: super::SessionId,
        #[builder(default)]
        #[serde(skip_serializing_if = "Option::is_none")]
        #[doc = "Deprecated."]
        #[deprecated]
        pub target_id: Option<super::TargetId>,
    }
    #[allow(deprecated)]
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
    pub struct ReceivedMessageFromTargetEvent {
        pub params: ReceivedMessageFromTargetEventParams,
    }
    #[allow(deprecated)]
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
    #[serde(rename_all = "camelCase")]
    pub struct ReceivedMessageFromTargetEventParams {
        #[doc = "Identifier of a session which sends a message."]
        pub session_id: super::SessionId,
        #[serde(default)]
        pub message: String,
        #[builder(default)]
        #[serde(skip_serializing_if = "Option::is_none")]
        #[doc = "Deprecated."]
        #[deprecated]
        pub target_id: Option<super::TargetId>,
    }
    #[allow(deprecated)]
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
    pub struct TargetCreatedEvent {
        pub params: TargetCreatedEventParams,
    }
    #[allow(deprecated)]
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
    #[serde(rename_all = "camelCase")]
    pub struct TargetCreatedEventParams {
        pub target_info: super::TargetInfo,
    }
    #[allow(deprecated)]
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
    pub struct TargetDestroyedEvent {
        pub params: TargetDestroyedEventParams,
    }
    #[allow(deprecated)]
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
    #[serde(rename_all = "camelCase")]
    pub struct TargetDestroyedEventParams {
        pub target_id: super::TargetId,
    }
    #[allow(deprecated)]
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
    pub struct TargetCrashedEvent {
        pub params: TargetCrashedEventParams,
    }
    #[allow(deprecated)]
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
    #[serde(rename_all = "camelCase")]
    pub struct TargetCrashedEventParams {
        pub target_id: super::TargetId,
        #[serde(default)]
        #[doc = "Termination status type."]
        pub status: String,
        #[serde(default)]
        #[doc = "Termination error code."]
        pub error_code: JsUInt,
    }
    #[allow(deprecated)]
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
    pub struct TargetInfoChangedEvent {
        pub params: TargetInfoChangedEventParams,
    }
    #[allow(deprecated)]
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
    #[serde(rename_all = "camelCase")]
    pub struct TargetInfoChangedEventParams {
        pub target_info: super::TargetInfo,
    }
}
