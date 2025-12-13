// Auto-generated from Chrome at version 143.0.7499.110 domain: Browser
use super::target;
#[allow(unused_imports)]
use super::types::*;
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
    #[serde(rename = "localNetworkAccess")]
    LocalNetworkAccess,
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
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct Bounds {
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
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct PermissionDescriptor {
    #[serde(default)]
    #[serde(rename = "name")]
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(rename = "sysex")]
    pub sysex: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(rename = "userVisibleOnly")]
    pub user_visible_only: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(rename = "allowWithoutSanitization")]
    pub allow_without_sanitization: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(rename = "allowWithoutGesture")]
    pub allow_without_gesture: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(rename = "panTiltZoom")]
    pub pan_tilt_zoom: Option<bool>,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct Bucket {
    #[serde(default)]
    #[serde(rename = "low")]
    pub low: JsUInt,
    #[serde(default)]
    #[serde(rename = "high")]
    pub high: JsUInt,
    #[serde(default)]
    #[serde(rename = "count")]
    pub count: JsUInt,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct Histogram {
    #[serde(default)]
    #[serde(rename = "name")]
    pub name: String,
    #[serde(default)]
    #[serde(rename = "sum")]
    pub sum: JsUInt,
    #[serde(default)]
    #[serde(rename = "count")]
    pub count: JsUInt,
    #[serde(rename = "buckets")]
    pub buckets: Vec<Bucket>,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct SetPermission {
    #[serde(rename = "permission")]
    pub permission: PermissionDescriptor,
    #[serde(rename = "setting")]
    pub setting: PermissionSetting,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(rename = "origin")]
    pub origin: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(rename = "embeddedOrigin")]
    pub embedded_origin: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "browserContextId")]
    pub browser_context_id: Option<BrowserContextId>,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct GrantPermissions {
    #[serde(rename = "permissions")]
    pub permissions: Vec<PermissionType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(rename = "origin")]
    pub origin: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "browserContextId")]
    pub browser_context_id: Option<BrowserContextId>,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct ResetPermissions {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "browserContextId")]
    pub browser_context_id: Option<BrowserContextId>,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct SetDownloadBehavior {
    #[serde(rename = "behavior")]
    pub behavior: SetDownloadBehaviorBehaviorOption,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "browserContextId")]
    pub browser_context_id: Option<BrowserContextId>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(rename = "downloadPath")]
    pub download_path: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(rename = "eventsEnabled")]
    pub events_enabled: Option<bool>,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct CancelDownload {
    #[serde(default)]
    #[serde(rename = "guid")]
    pub guid: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "browserContextId")]
    pub browser_context_id: Option<BrowserContextId>,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Close(pub Option<serde_json::Value>);
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Crash(pub Option<serde_json::Value>);
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct CrashGpuProcess(pub Option<serde_json::Value>);
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct GetVersion(pub Option<serde_json::Value>);
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct GetBrowserCommandLine(pub Option<serde_json::Value>);
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct GetHistograms {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(rename = "query")]
    pub query: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(rename = "delta")]
    pub delta: Option<bool>,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct GetHistogram {
    #[serde(default)]
    #[serde(rename = "name")]
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(rename = "delta")]
    pub delta: Option<bool>,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct GetWindowBounds {
    #[serde(rename = "windowId")]
    pub window_id: WindowId,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct GetWindowForTarget {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "targetId")]
    pub target_id: Option<target::TargetId>,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct SetWindowBounds {
    #[serde(rename = "windowId")]
    pub window_id: WindowId,
    #[serde(rename = "bounds")]
    pub bounds: Bounds,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct SetContentsSize {
    #[serde(rename = "windowId")]
    pub window_id: WindowId,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(rename = "width")]
    pub width: Option<JsUInt>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(rename = "height")]
    pub height: Option<JsUInt>,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct SetDockTile {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(rename = "badgeLabel")]
    pub badge_label: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "image")]
    pub image: Option<Vec<u8>>,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct ExecuteBrowserCommand {
    #[serde(rename = "commandId")]
    pub command_id: BrowserCommandId,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct AddPrivacySandboxEnrollmentOverride {
    #[serde(default)]
    #[serde(rename = "url")]
    pub url: String,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct AddPrivacySandboxCoordinatorKeyConfig {
    #[serde(rename = "api")]
    pub api: PrivacySandboxApi,
    #[serde(default)]
    #[serde(rename = "coordinatorOrigin")]
    pub coordinator_origin: String,
    #[serde(default)]
    #[serde(rename = "keyConfig")]
    pub key_config: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "browserContextId")]
    pub browser_context_id: Option<BrowserContextId>,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct SetPermissionReturnObject {}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct GrantPermissionsReturnObject {}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct ResetPermissionsReturnObject {}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct SetDownloadBehaviorReturnObject {}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct CancelDownloadReturnObject {}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct CloseReturnObject {}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct CrashReturnObject {}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct CrashGpuProcessReturnObject {}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct GetVersionReturnObject {
    #[serde(default)]
    #[serde(rename = "protocolVersion")]
    pub protocol_version: String,
    #[serde(default)]
    #[serde(rename = "product")]
    pub product: String,
    #[serde(default)]
    #[serde(rename = "revision")]
    pub revision: String,
    #[serde(default)]
    #[serde(rename = "userAgent")]
    pub user_agent: String,
    #[serde(default)]
    #[serde(rename = "jsVersion")]
    pub js_version: String,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct GetBrowserCommandLineReturnObject {
    #[serde(rename = "arguments")]
    pub arguments: Vec<String>,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct GetHistogramsReturnObject {
    #[serde(rename = "histograms")]
    pub histograms: Vec<Histogram>,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct GetHistogramReturnObject {
    #[serde(rename = "histogram")]
    pub histogram: Histogram,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct GetWindowBoundsReturnObject {
    #[serde(rename = "bounds")]
    pub bounds: Bounds,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct GetWindowForTargetReturnObject {
    #[serde(rename = "windowId")]
    pub window_id: WindowId,
    #[serde(rename = "bounds")]
    pub bounds: Bounds,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct SetWindowBoundsReturnObject {}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct SetContentsSizeReturnObject {}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct SetDockTileReturnObject {}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct ExecuteBrowserCommandReturnObject {}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct AddPrivacySandboxEnrollmentOverrideReturnObject {}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct AddPrivacySandboxCoordinatorKeyConfigReturnObject {}
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
    use serde::{Deserialize, Serialize};
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
    pub struct DownloadWillBeginEvent {
        pub params: DownloadWillBeginEventParams,
    }
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
    pub struct DownloadWillBeginEventParams {
        #[serde(rename = "frameId")]
        pub frame_id: super::super::page::FrameId,
        #[serde(default)]
        #[serde(rename = "guid")]
        pub guid: String,
        #[serde(default)]
        #[serde(rename = "url")]
        pub url: String,
        #[serde(default)]
        #[serde(rename = "suggestedFilename")]
        pub suggested_filename: String,
    }
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
    pub struct DownloadProgressEvent {
        pub params: DownloadProgressEventParams,
    }
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
    pub struct DownloadProgressEventParams {
        #[serde(default)]
        #[serde(rename = "guid")]
        pub guid: String,
        #[serde(default)]
        #[serde(rename = "totalBytes")]
        pub total_bytes: JsFloat,
        #[serde(default)]
        #[serde(rename = "receivedBytes")]
        pub received_bytes: JsFloat,
        #[serde(rename = "state")]
        pub state: super::DownloadProgressStateOption,
        #[serde(skip_serializing_if = "Option::is_none")]
        #[serde(default)]
        #[serde(rename = "filePath")]
        pub file_path: Option<String>,
    }
}
