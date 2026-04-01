// Auto-generated from Chrome at version 146.0.7680.165 domain: Browser
use super::target;
#[allow(unused_imports)]
use super::types::*;
#[allow(unused_imports)]
use derive_builder::Builder;
#[allow(unused_imports)]
use serde::{Deserialize, Serialize};
#[allow(unused_imports)]
use serde_json::Value as Json;
pub type BrowserContextId = String;
pub type WindowId = JsUInt;
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
pub enum PermissionType {
    #[serde(rename = "ar")]
    Ar,
    #[serde(rename = "audioCapture")]
    AudioCapture,
    #[serde(rename = "automaticFullscreen")]
    AutomaticFullscreen,
    #[serde(rename = "backgroundFetch")]
    BackgroundFetch,
    #[serde(rename = "backgroundSync")]
    BackgroundSync,
    #[serde(rename = "cameraPanTiltZoom")]
    CameraPanTiltZoom,
    #[serde(rename = "capturedSurfaceControl")]
    CapturedSurfaceControl,
    #[serde(rename = "clipboardReadWrite")]
    ClipboardReadWrite,
    #[serde(rename = "clipboardSanitizedWrite")]
    ClipboardSanitizedWrite,
    #[serde(rename = "displayCapture")]
    DisplayCapture,
    #[serde(rename = "durableStorage")]
    DurableStorage,
    #[serde(rename = "geolocation")]
    Geolocation,
    #[serde(rename = "handTracking")]
    HandTracking,
    #[serde(rename = "idleDetection")]
    IdleDetection,
    #[serde(rename = "keyboardLock")]
    KeyboardLock,
    #[serde(rename = "localFonts")]
    LocalFonts,
    #[serde(rename = "localNetwork")]
    LocalNetwork,
    #[serde(rename = "localNetworkAccess")]
    LocalNetworkAccess,
    #[serde(rename = "loopbackNetwork")]
    LoopbackNetwork,
    #[serde(rename = "midi")]
    Midi,
    #[serde(rename = "midiSysex")]
    MidiSysex,
    #[serde(rename = "nfc")]
    Nfc,
    #[serde(rename = "notifications")]
    Notifications,
    #[serde(rename = "paymentHandler")]
    PaymentHandler,
    #[serde(rename = "periodicBackgroundSync")]
    PeriodicBackgroundSync,
    #[serde(rename = "pointerLock")]
    PointerLock,
    #[serde(rename = "protectedMediaIdentifier")]
    ProtectedMediaIdentifier,
    #[serde(rename = "sensors")]
    Sensors,
    #[serde(rename = "smartCard")]
    SmartCard,
    #[serde(rename = "speakerSelection")]
    SpeakerSelection,
    #[serde(rename = "storageAccess")]
    StorageAccess,
    #[serde(rename = "topLevelStorageAccess")]
    TopLevelStorageAccess,
    #[serde(rename = "videoCapture")]
    VideoCapture,
    #[serde(rename = "vr")]
    Vr,
    #[serde(rename = "wakeLockScreen")]
    WakeLockScreen,
    #[serde(rename = "wakeLockSystem")]
    WakeLockSystem,
    #[serde(rename = "webAppInstallation")]
    WebAppInstallation,
    #[serde(rename = "webPrinting")]
    WebPrinting,
    #[serde(rename = "windowManagement")]
    WindowManagement,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub enum PermissionSetting {
    #[serde(rename = "granted")]
    Granted,
    #[serde(rename = "denied")]
    Denied,
    #[serde(rename = "prompt")]
    Prompt,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub enum BrowserCommandId {
    #[serde(rename = "openTabSearch")]
    OpenTabSearch,
    #[serde(rename = "closeTabSearch")]
    CloseTabSearch,
    #[serde(rename = "openGlic")]
    OpenGlic,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub enum PrivacySandboxApi {
    #[serde(rename = "BiddingAndAuctionServices")]
    BiddingAndAuctionServices,
    #[serde(rename = "TrustedKeyValue")]
    TrustedKeyValue,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub enum SetDownloadBehaviorBehaviorOption {
    #[serde(rename = "deny")]
    Deny,
    #[serde(rename = "allow")]
    Allow,
    #[serde(rename = "allowAndName")]
    AllowAndName,
    #[serde(rename = "default")]
    Default,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub enum DownloadProgressStateOption {
    #[serde(rename = "inProgress")]
    InProgress,
    #[serde(rename = "completed")]
    Completed,
    #[serde(rename = "canceled")]
    Canceled,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "camelCase")]
#[doc = "Browser window bounds information"]
pub struct Bounds {
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[doc = "The offset from the left edge of the screen to the window in pixels."]
    pub left: Option<JsUInt>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[doc = "The offset from the top edge of the screen to the window in pixels."]
    pub top: Option<JsUInt>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[doc = "The window width in pixels."]
    pub width: Option<JsUInt>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[doc = "The window height in pixels."]
    pub height: Option<JsUInt>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[doc = "The window state. Default to normal."]
    pub window_state: Option<WindowState>,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "camelCase")]
#[doc = "Definition of PermissionDescriptor defined in the Permissions API:\n https://w3c.github.io/permissions/#dom-permissiondescriptor."]
pub struct PermissionDescriptor {
    #[serde(default)]
    #[doc = "Name of permission.\n See https://cs.chromium.org/chromium/src/third_party/blink/renderer/modules/permissions/permission_descriptor.idl for valid permission names."]
    pub name: String,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[doc = "For \"midi\" permission, may also specify sysex control."]
    pub sysex: Option<bool>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[doc = "For \"push\" permission, may specify userVisibleOnly.\n Note that userVisibleOnly = true is the only currently supported type."]
    pub user_visible_only: Option<bool>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[doc = "For \"clipboard\" permission, may specify allowWithoutSanitization."]
    pub allow_without_sanitization: Option<bool>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[doc = "For \"fullscreen\" permission, must specify allowWithoutGesture:true."]
    pub allow_without_gesture: Option<bool>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[doc = "For \"camera\" permission, may specify panTiltZoom."]
    pub pan_tilt_zoom: Option<bool>,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "camelCase")]
#[doc = "Chrome histogram bucket."]
pub struct Bucket {
    #[serde(default)]
    #[doc = "Minimum value (inclusive)."]
    pub low: JsUInt,
    #[serde(default)]
    #[doc = "Maximum value (exclusive)."]
    pub high: JsUInt,
    #[serde(default)]
    #[doc = "Number of samples."]
    pub count: JsUInt,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "camelCase")]
#[doc = "Chrome histogram."]
pub struct Histogram {
    #[serde(default)]
    #[doc = "Name."]
    pub name: String,
    #[serde(default)]
    #[doc = "Sum of sample values."]
    pub sum: JsUInt,
    #[serde(default)]
    #[doc = "Total number of samples."]
    pub count: JsUInt,
    #[doc = "Buckets."]
    pub buckets: Vec<Bucket>,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "camelCase")]
#[doc = "Set permission settings for given embedding and embedded origins."]
pub struct SetPermission {
    #[doc = "Descriptor of permission to override."]
    pub permission: PermissionDescriptor,
    #[doc = "Setting of the permission."]
    pub setting: PermissionSetting,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[doc = "Embedding origin the permission applies to, all origins if not specified."]
    pub origin: Option<String>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[doc = "Embedded origin the permission applies to. It is ignored unless the embedding origin is\n present and valid. If the embedding origin is provided but the embedded origin isn't, the\n embedding origin is used as the embedded origin."]
    pub embedded_origin: Option<String>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[doc = "Context to override. When omitted, default browser context is used."]
    pub browser_context_id: Option<BrowserContextId>,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "camelCase")]
#[doc = "Grant specific permissions to the given origin and reject all others. Deprecated. Use\n setPermission instead."]
#[deprecated]
pub struct GrantPermissions {
    pub permissions: Vec<PermissionType>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[doc = "Origin the permission applies to, all origins if not specified."]
    pub origin: Option<String>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[doc = "BrowserContext to override permissions. When omitted, default browser context is used."]
    pub browser_context_id: Option<BrowserContextId>,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "camelCase")]
#[doc = "Reset all permission management for all origins."]
pub struct ResetPermissions {
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[doc = "BrowserContext to reset permissions. When omitted, default browser context is used."]
    pub browser_context_id: Option<BrowserContextId>,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "camelCase")]
#[doc = "Set the behavior when downloading a file."]
pub struct SetDownloadBehavior {
    #[doc = "Whether to allow all or deny all download requests, or use default Chrome behavior if\n available (otherwise deny). |allowAndName| allows download and names files according to\n their download guids."]
    pub behavior: SetDownloadBehaviorBehaviorOption,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[doc = "BrowserContext to set download behavior. When omitted, default browser context is used."]
    pub browser_context_id: Option<BrowserContextId>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[doc = "The default path to save downloaded files to. This is required if behavior is set to 'allow'\n or 'allowAndName'."]
    pub download_path: Option<String>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[doc = "Whether to emit download events (defaults to false)."]
    pub events_enabled: Option<bool>,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "camelCase")]
#[doc = "Cancel a download if in progress"]
pub struct CancelDownload {
    #[serde(default)]
    #[doc = "Global unique identifier of the download."]
    pub guid: String,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[doc = "BrowserContext to perform the action in. When omitted, default browser context is used."]
    pub browser_context_id: Option<BrowserContextId>,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct Close(pub Option<Json>);
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct Crash(pub Option<Json>);
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct CrashGpuProcess(pub Option<Json>);
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct GetVersion(pub Option<Json>);
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct GetBrowserCommandLine(pub Option<Json>);
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "camelCase")]
#[doc = "Get Chrome histograms."]
pub struct GetHistograms {
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[doc = "Requested substring in name. Only histograms which have query as a\n substring in their name are extracted. An empty or absent query returns\n all histograms."]
    pub query: Option<String>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[doc = "If true, retrieve delta since last delta call."]
    pub delta: Option<bool>,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "camelCase")]
#[doc = "Get a Chrome histogram by name."]
pub struct GetHistogram {
    #[serde(default)]
    #[doc = "Requested histogram name."]
    pub name: String,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[doc = "If true, retrieve delta since last delta call."]
    pub delta: Option<bool>,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "camelCase")]
#[doc = "Get position and size of the browser window."]
pub struct GetWindowBounds {
    #[doc = "Browser window id."]
    pub window_id: WindowId,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "camelCase")]
#[doc = "Get the browser window that contains the devtools target."]
pub struct GetWindowForTarget {
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[doc = "Devtools agent host id. If called as a part of the session, associated targetId is used."]
    pub target_id: Option<target::TargetId>,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "camelCase")]
#[doc = "Set position and/or size of the browser window."]
pub struct SetWindowBounds {
    #[doc = "Browser window id."]
    pub window_id: WindowId,
    #[doc = "New window bounds. The 'minimized', 'maximized' and 'fullscreen' states cannot be combined\n with 'left', 'top', 'width' or 'height'. Leaves unspecified fields unchanged."]
    pub bounds: Bounds,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "camelCase")]
#[doc = "Set size of the browser contents resizing browser window as necessary."]
pub struct SetContentsSize {
    #[doc = "Browser window id."]
    pub window_id: WindowId,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[doc = "The window contents width in DIP. Assumes current width if omitted.\n Must be specified if 'height' is omitted."]
    pub width: Option<JsUInt>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[doc = "The window contents height in DIP. Assumes current height if omitted.\n Must be specified if 'width' is omitted."]
    pub height: Option<JsUInt>,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "camelCase")]
#[doc = "Set dock tile details, platform-specific."]
pub struct SetDockTile {
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub badge_label: Option<String>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[doc = "Png encoded image."]
    pub image: Option<Vec<u8>>,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "camelCase")]
#[doc = "Invoke custom browser commands used by telemetry."]
pub struct ExecuteBrowserCommand {
    pub command_id: BrowserCommandId,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "camelCase")]
#[doc = "Allows a site to use privacy sandbox features that require enrollment\n without the site actually being enrolled. Only supported on page targets."]
pub struct AddPrivacySandboxEnrollmentOverride {
    #[serde(default)]
    pub url: String,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "camelCase")]
#[doc = "Configures encryption keys used with a given privacy sandbox API to talk\n to a trusted coordinator.  Since this is intended for test automation only,\n coordinatorOrigin must be a .test domain. No existing coordinator\n configuration for the origin may exist."]
pub struct AddPrivacySandboxCoordinatorKeyConfig {
    pub api: PrivacySandboxApi,
    #[serde(default)]
    pub coordinator_origin: String,
    #[serde(default)]
    pub key_config: String,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[doc = "BrowserContext to perform the action in. When omitted, default browser\n context is used."]
    pub browser_context_id: Option<BrowserContextId>,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[doc = "Set permission settings for given embedding and embedded origins."]
pub struct SetPermissionReturnObject(pub Option<Json>);
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[doc = "Grant specific permissions to the given origin and reject all others. Deprecated. Use\n setPermission instead."]
#[deprecated]
pub struct GrantPermissionsReturnObject(pub Option<Json>);
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[doc = "Reset all permission management for all origins."]
pub struct ResetPermissionsReturnObject(pub Option<Json>);
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[doc = "Set the behavior when downloading a file."]
pub struct SetDownloadBehaviorReturnObject(pub Option<Json>);
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[doc = "Cancel a download if in progress"]
pub struct CancelDownloadReturnObject(pub Option<Json>);
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[doc = "Close browser gracefully."]
pub struct CloseReturnObject(pub Option<Json>);
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[doc = "Crashes browser on the main thread."]
pub struct CrashReturnObject(pub Option<Json>);
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[doc = "Crashes GPU process."]
pub struct CrashGpuProcessReturnObject(pub Option<Json>);
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[serde(rename_all = "camelCase")]
#[doc = "Returns version information."]
pub struct GetVersionReturnObject {
    #[serde(default)]
    #[doc = "Protocol version."]
    pub protocol_version: String,
    #[serde(default)]
    #[doc = "Product name."]
    pub product: String,
    #[serde(default)]
    #[doc = "Product revision."]
    pub revision: String,
    #[serde(default)]
    #[doc = "User-Agent."]
    pub user_agent: String,
    #[serde(default)]
    #[doc = "V8 version."]
    pub js_version: String,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[serde(rename_all = "camelCase")]
#[doc = "Returns the command line switches for the browser process if, and only if\n --enable-automation is on the commandline."]
pub struct GetBrowserCommandLineReturnObject {
    #[doc = "Commandline parameters"]
    pub arguments: Vec<String>,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[serde(rename_all = "camelCase")]
#[doc = "Get Chrome histograms."]
pub struct GetHistogramsReturnObject {
    #[doc = "Histograms."]
    pub histograms: Vec<Histogram>,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[serde(rename_all = "camelCase")]
#[doc = "Get a Chrome histogram by name."]
pub struct GetHistogramReturnObject {
    #[doc = "Histogram."]
    pub histogram: Histogram,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[serde(rename_all = "camelCase")]
#[doc = "Get position and size of the browser window."]
pub struct GetWindowBoundsReturnObject {
    #[doc = "Bounds information of the window. When window state is 'minimized', the restored window\n position and size are returned."]
    pub bounds: Bounds,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[serde(rename_all = "camelCase")]
#[doc = "Get the browser window that contains the devtools target."]
pub struct GetWindowForTargetReturnObject {
    #[doc = "Browser window id."]
    pub window_id: WindowId,
    #[doc = "Bounds information of the window. When window state is 'minimized', the restored window\n position and size are returned."]
    pub bounds: Bounds,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[doc = "Set position and/or size of the browser window."]
pub struct SetWindowBoundsReturnObject(pub Option<Json>);
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[doc = "Set size of the browser contents resizing browser window as necessary."]
pub struct SetContentsSizeReturnObject(pub Option<Json>);
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[doc = "Set dock tile details, platform-specific."]
pub struct SetDockTileReturnObject(pub Option<Json>);
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[doc = "Invoke custom browser commands used by telemetry."]
pub struct ExecuteBrowserCommandReturnObject(pub Option<Json>);
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[doc = "Allows a site to use privacy sandbox features that require enrollment\n without the site actually being enrolled. Only supported on page targets."]
pub struct AddPrivacySandboxEnrollmentOverrideReturnObject(pub Option<Json>);
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[doc = "Configures encryption keys used with a given privacy sandbox API to talk\n to a trusted coordinator.  Since this is intended for test automation only,\n coordinatorOrigin must be a .test domain. No existing coordinator\n configuration for the origin may exist."]
pub struct AddPrivacySandboxCoordinatorKeyConfigReturnObject(pub Option<Json>);
impl Method for SetPermission {
    const NAME: &'static str = "Browser.setPermission";
    type ReturnObject = SetPermissionReturnObject;
}
impl Method for GrantPermissions {
    const NAME: &'static str = "Browser.grantPermissions";
    type ReturnObject = GrantPermissionsReturnObject;
}
impl Method for ResetPermissions {
    const NAME: &'static str = "Browser.resetPermissions";
    type ReturnObject = ResetPermissionsReturnObject;
}
impl Method for SetDownloadBehavior {
    const NAME: &'static str = "Browser.setDownloadBehavior";
    type ReturnObject = SetDownloadBehaviorReturnObject;
}
impl Method for CancelDownload {
    const NAME: &'static str = "Browser.cancelDownload";
    type ReturnObject = CancelDownloadReturnObject;
}
impl Method for Close {
    const NAME: &'static str = "Browser.close";
    type ReturnObject = CloseReturnObject;
}
impl Method for Crash {
    const NAME: &'static str = "Browser.crash";
    type ReturnObject = CrashReturnObject;
}
impl Method for CrashGpuProcess {
    const NAME: &'static str = "Browser.crashGpuProcess";
    type ReturnObject = CrashGpuProcessReturnObject;
}
impl Method for GetVersion {
    const NAME: &'static str = "Browser.getVersion";
    type ReturnObject = GetVersionReturnObject;
}
impl Method for GetBrowserCommandLine {
    const NAME: &'static str = "Browser.getBrowserCommandLine";
    type ReturnObject = GetBrowserCommandLineReturnObject;
}
impl Method for GetHistograms {
    const NAME: &'static str = "Browser.getHistograms";
    type ReturnObject = GetHistogramsReturnObject;
}
impl Method for GetHistogram {
    const NAME: &'static str = "Browser.getHistogram";
    type ReturnObject = GetHistogramReturnObject;
}
impl Method for GetWindowBounds {
    const NAME: &'static str = "Browser.getWindowBounds";
    type ReturnObject = GetWindowBoundsReturnObject;
}
impl Method for GetWindowForTarget {
    const NAME: &'static str = "Browser.getWindowForTarget";
    type ReturnObject = GetWindowForTargetReturnObject;
}
impl Method for SetWindowBounds {
    const NAME: &'static str = "Browser.setWindowBounds";
    type ReturnObject = SetWindowBoundsReturnObject;
}
impl Method for SetContentsSize {
    const NAME: &'static str = "Browser.setContentsSize";
    type ReturnObject = SetContentsSizeReturnObject;
}
impl Method for SetDockTile {
    const NAME: &'static str = "Browser.setDockTile";
    type ReturnObject = SetDockTileReturnObject;
}
impl Method for ExecuteBrowserCommand {
    const NAME: &'static str = "Browser.executeBrowserCommand";
    type ReturnObject = ExecuteBrowserCommandReturnObject;
}
impl Method for AddPrivacySandboxEnrollmentOverride {
    const NAME: &'static str = "Browser.addPrivacySandboxEnrollmentOverride";
    type ReturnObject = AddPrivacySandboxEnrollmentOverrideReturnObject;
}
impl Method for AddPrivacySandboxCoordinatorKeyConfig {
    const NAME: &'static str = "Browser.addPrivacySandboxCoordinatorKeyConfig";
    type ReturnObject = AddPrivacySandboxCoordinatorKeyConfigReturnObject;
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
    pub struct DownloadWillBeginEvent {
        pub params: DownloadWillBeginEventParams,
    }
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
    pub struct DownloadWillBeginEventParams {
        #[doc = "Id of the frame that caused the download to begin."]
        pub frame_id: super::super::page::FrameId,
        #[serde(default)]
        #[doc = "Global unique identifier of the download."]
        pub guid: String,
        #[serde(default)]
        #[doc = "URL of the resource being downloaded."]
        pub url: String,
        #[serde(default)]
        #[doc = "Suggested file name of the resource (the actual name of the file saved on disk may differ)."]
        pub suggested_filename: String,
    }
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
    pub struct DownloadProgressEvent {
        pub params: DownloadProgressEventParams,
    }
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
    pub struct DownloadProgressEventParams {
        #[serde(default)]
        #[doc = "Global unique identifier of the download."]
        pub guid: String,
        #[serde(default)]
        #[doc = "Total expected bytes to download."]
        pub total_bytes: JsFloat,
        #[serde(default)]
        #[doc = "Total bytes received."]
        pub received_bytes: JsFloat,
        #[doc = "Download status."]
        pub state: super::DownloadProgressStateOption,
        #[builder(default)]
        #[serde(skip_serializing_if = "Option::is_none")]
        #[serde(default)]
        #[doc = "If download is \"completed\", provides the path of the downloaded file.\n Depending on the platform, it is not guaranteed to be set, nor the file\n is guaranteed to exist."]
        pub file_path: Option<String>,
    }
}
