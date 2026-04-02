// Auto-generated from Chrome at version 146.0.7680.165 domain: Page
#![allow(dead_code)]
use super::debugger;
use super::dom;
use super::emulation;
use super::io;
use super::network;
use super::runtime;
#[allow(unused_imports)]
use super::types::*;
#[allow(unused_imports)]
use derive_builder::Builder;
#[allow(unused_imports)]
use serde::{Deserialize, Serialize};
#[allow(unused_imports)]
use serde_json::Value as Json;
pub type FrameId = String;
pub type ScriptIdentifier = String;
#[allow(deprecated)]
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub enum AdFrameType {
    #[serde(rename = "none")]
    None,
    #[serde(rename = "child")]
    Child,
    #[serde(rename = "root")]
    Root,
}
#[allow(deprecated)]
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub enum AdFrameExplanation {
    #[serde(rename = "ParentIsAd")]
    ParentIsAd,
    #[serde(rename = "CreatedByAdScript")]
    CreatedByAdScript,
    #[serde(rename = "MatchedBlockingRule")]
    MatchedBlockingRule,
}
#[allow(deprecated)]
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub enum SecureContextType {
    #[serde(rename = "Secure")]
    Secure,
    #[serde(rename = "SecureLocalhost")]
    SecureLocalhost,
    #[serde(rename = "InsecureScheme")]
    InsecureScheme,
    #[serde(rename = "InsecureAncestor")]
    InsecureAncestor,
}
#[allow(deprecated)]
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub enum CrossOriginIsolatedContextType {
    #[serde(rename = "Isolated")]
    Isolated,
    #[serde(rename = "NotIsolated")]
    NotIsolated,
    #[serde(rename = "NotIsolatedFeatureDisabled")]
    NotIsolatedFeatureDisabled,
}
#[allow(deprecated)]
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub enum GatedApiFeatures {
    #[serde(rename = "SharedArrayBuffers")]
    SharedArrayBuffers,
    #[serde(rename = "SharedArrayBuffersTransferAllowed")]
    SharedArrayBuffersTransferAllowed,
    #[serde(rename = "PerformanceMeasureMemory")]
    PerformanceMeasureMemory,
    #[serde(rename = "PerformanceProfile")]
    PerformanceProfile,
}
#[allow(deprecated)]
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub enum PermissionsPolicyFeature {
    #[serde(rename = "accelerometer")]
    Accelerometer,
    #[serde(rename = "all-screens-capture")]
    AllScreensCapture,
    #[serde(rename = "ambient-light-sensor")]
    AmbientLightSensor,
    #[serde(rename = "aria-notify")]
    AriaNotify,
    #[serde(rename = "attribution-reporting")]
    AttributionReporting,
    #[serde(rename = "autofill")]
    Autofill,
    #[serde(rename = "autoplay")]
    Autoplay,
    #[serde(rename = "bluetooth")]
    Bluetooth,
    #[serde(rename = "browsing-topics")]
    BrowsingTopics,
    #[serde(rename = "camera")]
    Camera,
    #[serde(rename = "captured-surface-control")]
    CapturedSurfaceControl,
    #[serde(rename = "ch-dpr")]
    ChDpr,
    #[serde(rename = "ch-device-memory")]
    ChDeviceMemory,
    #[serde(rename = "ch-downlink")]
    ChDownlink,
    #[serde(rename = "ch-ect")]
    ChEct,
    #[serde(rename = "ch-prefers-color-scheme")]
    ChPrefersColorScheme,
    #[serde(rename = "ch-prefers-reduced-motion")]
    ChPrefersReducedMotion,
    #[serde(rename = "ch-prefers-reduced-transparency")]
    ChPrefersReducedTransparency,
    #[serde(rename = "ch-rtt")]
    ChRtt,
    #[serde(rename = "ch-save-data")]
    ChSaveData,
    #[serde(rename = "ch-ua")]
    ChUa,
    #[serde(rename = "ch-ua-arch")]
    ChUaArch,
    #[serde(rename = "ch-ua-bitness")]
    ChUaBitness,
    #[serde(rename = "ch-ua-high-entropy-values")]
    ChUaHighEntropyValues,
    #[serde(rename = "ch-ua-platform")]
    ChUaPlatform,
    #[serde(rename = "ch-ua-model")]
    ChUaModel,
    #[serde(rename = "ch-ua-mobile")]
    ChUaMobile,
    #[serde(rename = "ch-ua-form-factors")]
    ChUaFormFactors,
    #[serde(rename = "ch-ua-full-version")]
    ChUaFullVersion,
    #[serde(rename = "ch-ua-full-version-list")]
    ChUaFullVersionList,
    #[serde(rename = "ch-ua-platform-version")]
    ChUaPlatformVersion,
    #[serde(rename = "ch-ua-wow64")]
    ChUaWow64,
    #[serde(rename = "ch-viewport-height")]
    ChViewportHeight,
    #[serde(rename = "ch-viewport-width")]
    ChViewportWidth,
    #[serde(rename = "ch-width")]
    ChWidth,
    #[serde(rename = "clipboard-read")]
    ClipboardRead,
    #[serde(rename = "clipboard-write")]
    ClipboardWrite,
    #[serde(rename = "compute-pressure")]
    ComputePressure,
    #[serde(rename = "controlled-frame")]
    ControlledFrame,
    #[serde(rename = "cross-origin-isolated")]
    CrossOriginIsolated,
    #[serde(rename = "deferred-fetch")]
    DeferredFetch,
    #[serde(rename = "deferred-fetch-minimal")]
    DeferredFetchMinimal,
    #[serde(rename = "device-attributes")]
    DeviceAttributes,
    #[serde(rename = "digital-credentials-create")]
    DigitalCredentialsCreate,
    #[serde(rename = "digital-credentials-get")]
    DigitalCredentialsGet,
    #[serde(rename = "direct-sockets")]
    DirectSockets,
    #[serde(rename = "direct-sockets-multicast")]
    DirectSocketsMulticast,
    #[serde(rename = "direct-sockets-private")]
    DirectSocketsPrivate,
    #[serde(rename = "display-capture")]
    DisplayCapture,
    #[serde(rename = "document-domain")]
    DocumentDomain,
    #[serde(rename = "encrypted-media")]
    EncryptedMedia,
    #[serde(rename = "execution-while-out-of-viewport")]
    ExecutionWhileOutOfViewport,
    #[serde(rename = "execution-while-not-rendered")]
    ExecutionWhileNotRendered,
    #[serde(rename = "fenced-unpartitioned-storage-read")]
    FencedUnpartitionedStorageRead,
    #[serde(rename = "focus-without-user-activation")]
    FocusWithoutUserActivation,
    #[serde(rename = "fullscreen")]
    Fullscreen,
    #[serde(rename = "frobulate")]
    Frobulate,
    #[serde(rename = "gamepad")]
    Gamepad,
    #[serde(rename = "geolocation")]
    Geolocation,
    #[serde(rename = "gyroscope")]
    Gyroscope,
    #[serde(rename = "hid")]
    Hid,
    #[serde(rename = "identity-credentials-get")]
    IdentityCredentialsGet,
    #[serde(rename = "idle-detection")]
    IdleDetection,
    #[serde(rename = "interest-cohort")]
    InterestCohort,
    #[serde(rename = "join-ad-interest-group")]
    JoinAdInterestGroup,
    #[serde(rename = "keyboard-map")]
    KeyboardMap,
    #[serde(rename = "language-detector")]
    LanguageDetector,
    #[serde(rename = "language-model")]
    LanguageModel,
    #[serde(rename = "local-fonts")]
    LocalFonts,
    #[serde(rename = "local-network")]
    LocalNetwork,
    #[serde(rename = "local-network-access")]
    LocalNetworkAccess,
    #[serde(rename = "loopback-network")]
    LoopbackNetwork,
    #[serde(rename = "magnetometer")]
    Magnetometer,
    #[serde(rename = "manual-text")]
    ManualText,
    #[serde(rename = "media-playback-while-not-visible")]
    MediaPlaybackWhileNotVisible,
    #[serde(rename = "microphone")]
    Microphone,
    #[serde(rename = "midi")]
    Midi,
    #[serde(rename = "on-device-speech-recognition")]
    OnDeviceSpeechRecognition,
    #[serde(rename = "otp-credentials")]
    OtpCredentials,
    #[serde(rename = "payment")]
    Payment,
    #[serde(rename = "picture-in-picture")]
    PictureInPicture,
    #[serde(rename = "private-aggregation")]
    PrivateAggregation,
    #[serde(rename = "private-state-token-issuance")]
    PrivateStateTokenIssuance,
    #[serde(rename = "private-state-token-redemption")]
    PrivateStateTokenRedemption,
    #[serde(rename = "publickey-credentials-create")]
    PublickeyCredentialsCreate,
    #[serde(rename = "publickey-credentials-get")]
    PublickeyCredentialsGet,
    #[serde(rename = "record-ad-auction-events")]
    RecordAdAuctionEvents,
    #[serde(rename = "rewriter")]
    Rewriter,
    #[serde(rename = "run-ad-auction")]
    RunAdAuction,
    #[serde(rename = "screen-wake-lock")]
    ScreenWakeLock,
    #[serde(rename = "serial")]
    Serial,
    #[serde(rename = "shared-storage")]
    SharedStorage,
    #[serde(rename = "shared-storage-select-url")]
    SharedStorageSelectUrl,
    #[serde(rename = "smart-card")]
    SmartCard,
    #[serde(rename = "speaker-selection")]
    SpeakerSelection,
    #[serde(rename = "storage-access")]
    StorageAccess,
    #[serde(rename = "sub-apps")]
    SubApps,
    #[serde(rename = "summarizer")]
    Summarizer,
    #[serde(rename = "sync-xhr")]
    SyncXhr,
    #[serde(rename = "translator")]
    Translator,
    #[serde(rename = "unload")]
    Unload,
    #[serde(rename = "usb")]
    Usb,
    #[serde(rename = "usb-unrestricted")]
    UsbUnrestricted,
    #[serde(rename = "vertical-scroll")]
    VerticalScroll,
    #[serde(rename = "web-app-installation")]
    WebAppInstallation,
    #[serde(rename = "web-printing")]
    WebPrinting,
    #[serde(rename = "web-share")]
    WebShare,
    #[serde(rename = "window-management")]
    WindowManagement,
    #[serde(rename = "writer")]
    Writer,
    #[serde(rename = "xr-spatial-tracking")]
    XrSpatialTracking,
}
#[allow(deprecated)]
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub enum PermissionsPolicyBlockReason {
    #[serde(rename = "Header")]
    Header,
    #[serde(rename = "IframeAttribute")]
    IframeAttribute,
    #[serde(rename = "InFencedFrameTree")]
    InFencedFrameTree,
    #[serde(rename = "InIsolatedApp")]
    InIsolatedApp,
}
#[allow(deprecated)]
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub enum OriginTrialTokenStatus {
    #[serde(rename = "Success")]
    Success,
    #[serde(rename = "NotSupported")]
    NotSupported,
    #[serde(rename = "Insecure")]
    Insecure,
    #[serde(rename = "Expired")]
    Expired,
    #[serde(rename = "WrongOrigin")]
    WrongOrigin,
    #[serde(rename = "InvalidSignature")]
    InvalidSignature,
    #[serde(rename = "Malformed")]
    Malformed,
    #[serde(rename = "WrongVersion")]
    WrongVersion,
    #[serde(rename = "FeatureDisabled")]
    FeatureDisabled,
    #[serde(rename = "TokenDisabled")]
    TokenDisabled,
    #[serde(rename = "FeatureDisabledForUser")]
    FeatureDisabledForUser,
    #[serde(rename = "UnknownTrial")]
    UnknownTrial,
}
#[allow(deprecated)]
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub enum OriginTrialStatus {
    #[serde(rename = "Enabled")]
    Enabled,
    #[serde(rename = "ValidTokenNotProvided")]
    ValidTokenNotProvided,
    #[serde(rename = "OSNotSupported")]
    OsNotSupported,
    #[serde(rename = "TrialNotAllowed")]
    TrialNotAllowed,
}
#[allow(deprecated)]
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub enum OriginTrialUsageRestriction {
    #[serde(rename = "None")]
    None,
    #[serde(rename = "Subset")]
    Subset,
}
#[allow(deprecated)]
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub enum TransitionType {
    #[serde(rename = "link")]
    Link,
    #[serde(rename = "typed")]
    Typed,
    #[serde(rename = "address_bar")]
    AddressBar,
    #[serde(rename = "auto_bookmark")]
    AutoBookmark,
    #[serde(rename = "auto_subframe")]
    AutoSubframe,
    #[serde(rename = "manual_subframe")]
    ManualSubframe,
    #[serde(rename = "generated")]
    Generated,
    #[serde(rename = "auto_toplevel")]
    AutoToplevel,
    #[serde(rename = "form_submit")]
    FormSubmit,
    #[serde(rename = "reload")]
    Reload,
    #[serde(rename = "keyword")]
    Keyword,
    #[serde(rename = "keyword_generated")]
    KeywordGenerated,
    #[serde(rename = "other")]
    Other,
}
#[allow(deprecated)]
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub enum DialogType {
    #[serde(rename = "alert")]
    Alert,
    #[serde(rename = "confirm")]
    Confirm,
    #[serde(rename = "prompt")]
    Prompt,
    #[serde(rename = "beforeunload")]
    Beforeunload,
}
#[allow(deprecated)]
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub enum ClientNavigationReason {
    #[serde(rename = "anchorClick")]
    AnchorClick,
    #[serde(rename = "formSubmissionGet")]
    FormSubmissionGet,
    #[serde(rename = "formSubmissionPost")]
    FormSubmissionPost,
    #[serde(rename = "httpHeaderRefresh")]
    HttpHeaderRefresh,
    #[serde(rename = "initialFrameNavigation")]
    InitialFrameNavigation,
    #[serde(rename = "metaTagRefresh")]
    MetaTagRefresh,
    #[serde(rename = "other")]
    Other,
    #[serde(rename = "pageBlockInterstitial")]
    PageBlockInterstitial,
    #[serde(rename = "reload")]
    Reload,
    #[serde(rename = "scriptInitiated")]
    ScriptInitiated,
}
#[allow(deprecated)]
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub enum ClientNavigationDisposition {
    #[serde(rename = "currentTab")]
    CurrentTab,
    #[serde(rename = "newTab")]
    NewTab,
    #[serde(rename = "newWindow")]
    NewWindow,
    #[serde(rename = "download")]
    Download,
}
#[allow(deprecated)]
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub enum ReferrerPolicy {
    #[serde(rename = "noReferrer")]
    NoReferrer,
    #[serde(rename = "noReferrerWhenDowngrade")]
    NoReferrerWhenDowngrade,
    #[serde(rename = "origin")]
    Origin,
    #[serde(rename = "originWhenCrossOrigin")]
    OriginWhenCrossOrigin,
    #[serde(rename = "sameOrigin")]
    SameOrigin,
    #[serde(rename = "strictOrigin")]
    StrictOrigin,
    #[serde(rename = "strictOriginWhenCrossOrigin")]
    StrictOriginWhenCrossOrigin,
    #[serde(rename = "unsafeUrl")]
    UnsafeUrl,
}
#[allow(deprecated)]
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub enum NavigationType {
    #[serde(rename = "Navigation")]
    Navigation,
    #[serde(rename = "BackForwardCacheRestore")]
    BackForwardCacheRestore,
}
#[allow(deprecated)]
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub enum BackForwardCacheNotRestoredReason {
    #[serde(rename = "NotPrimaryMainFrame")]
    NotPrimaryMainFrame,
    #[serde(rename = "BackForwardCacheDisabled")]
    BackForwardCacheDisabled,
    #[serde(rename = "RelatedActiveContentsExist")]
    RelatedActiveContentsExist,
    #[serde(rename = "HTTPStatusNotOK")]
    HttpStatusNotOk,
    #[serde(rename = "SchemeNotHTTPOrHTTPS")]
    SchemeNotHttpOrHttps,
    #[serde(rename = "Loading")]
    Loading,
    #[serde(rename = "WasGrantedMediaAccess")]
    WasGrantedMediaAccess,
    #[serde(rename = "DisableForRenderFrameHostCalled")]
    DisableForRenderFrameHostCalled,
    #[serde(rename = "DomainNotAllowed")]
    DomainNotAllowed,
    #[serde(rename = "HTTPMethodNotGET")]
    HttpMethodNotGet,
    #[serde(rename = "SubframeIsNavigating")]
    SubframeIsNavigating,
    #[serde(rename = "Timeout")]
    Timeout,
    #[serde(rename = "CacheLimit")]
    CacheLimit,
    #[serde(rename = "JavaScriptExecution")]
    JavaScriptExecution,
    #[serde(rename = "RendererProcessKilled")]
    RendererProcessKilled,
    #[serde(rename = "RendererProcessCrashed")]
    RendererProcessCrashed,
    #[serde(rename = "SchedulerTrackedFeatureUsed")]
    SchedulerTrackedFeatureUsed,
    #[serde(rename = "ConflictingBrowsingInstance")]
    ConflictingBrowsingInstance,
    #[serde(rename = "CacheFlushed")]
    CacheFlushed,
    #[serde(rename = "ServiceWorkerVersionActivation")]
    ServiceWorkerVersionActivation,
    #[serde(rename = "SessionRestored")]
    SessionRestored,
    #[serde(rename = "ServiceWorkerPostMessage")]
    ServiceWorkerPostMessage,
    #[serde(rename = "EnteredBackForwardCacheBeforeServiceWorkerHostAdded")]
    EnteredBackForwardCacheBeforeServiceWorkerHostAdded,
    #[serde(rename = "RenderFrameHostReused_SameSite")]
    RenderFrameHostReusedSameSite,
    #[serde(rename = "RenderFrameHostReused_CrossSite")]
    RenderFrameHostReusedCrossSite,
    #[serde(rename = "ServiceWorkerClaim")]
    ServiceWorkerClaim,
    #[serde(rename = "IgnoreEventAndEvict")]
    IgnoreEventAndEvict,
    #[serde(rename = "HaveInnerContents")]
    HaveInnerContents,
    #[serde(rename = "TimeoutPuttingInCache")]
    TimeoutPuttingInCache,
    #[serde(rename = "BackForwardCacheDisabledByLowMemory")]
    BackForwardCacheDisabledByLowMemory,
    #[serde(rename = "BackForwardCacheDisabledByCommandLine")]
    BackForwardCacheDisabledByCommandLine,
    #[serde(rename = "NetworkRequestDatapipeDrainedAsBytesConsumer")]
    NetworkRequestDatapipeDrainedAsBytesConsumer,
    #[serde(rename = "NetworkRequestRedirected")]
    NetworkRequestRedirected,
    #[serde(rename = "NetworkRequestTimeout")]
    NetworkRequestTimeout,
    #[serde(rename = "NetworkExceedsBufferLimit")]
    NetworkExceedsBufferLimit,
    #[serde(rename = "NavigationCancelledWhileRestoring")]
    NavigationCancelledWhileRestoring,
    #[serde(rename = "NotMostRecentNavigationEntry")]
    NotMostRecentNavigationEntry,
    #[serde(rename = "BackForwardCacheDisabledForPrerender")]
    BackForwardCacheDisabledForPrerender,
    #[serde(rename = "UserAgentOverrideDiffers")]
    UserAgentOverrideDiffers,
    #[serde(rename = "ForegroundCacheLimit")]
    ForegroundCacheLimit,
    #[serde(rename = "BrowsingInstanceNotSwapped")]
    BrowsingInstanceNotSwapped,
    #[serde(rename = "BackForwardCacheDisabledForDelegate")]
    BackForwardCacheDisabledForDelegate,
    #[serde(rename = "UnloadHandlerExistsInMainFrame")]
    UnloadHandlerExistsInMainFrame,
    #[serde(rename = "UnloadHandlerExistsInSubFrame")]
    UnloadHandlerExistsInSubFrame,
    #[serde(rename = "ServiceWorkerUnregistration")]
    ServiceWorkerUnregistration,
    #[serde(rename = "CacheControlNoStore")]
    CacheControlNoStore,
    #[serde(rename = "CacheControlNoStoreCookieModified")]
    CacheControlNoStoreCookieModified,
    #[serde(rename = "CacheControlNoStoreHTTPOnlyCookieModified")]
    CacheControlNoStoreHttpOnlyCookieModified,
    #[serde(rename = "NoResponseHead")]
    NoResponseHead,
    #[serde(rename = "Unknown")]
    Unknown,
    #[serde(rename = "ActivationNavigationsDisallowedForBug1234857")]
    ActivationNavigationsDisallowedForBug1234857,
    #[serde(rename = "ErrorDocument")]
    ErrorDocument,
    #[serde(rename = "FencedFramesEmbedder")]
    FencedFramesEmbedder,
    #[serde(rename = "CookieDisabled")]
    CookieDisabled,
    #[serde(rename = "HTTPAuthRequired")]
    HttpAuthRequired,
    #[serde(rename = "CookieFlushed")]
    CookieFlushed,
    #[serde(rename = "BroadcastChannelOnMessage")]
    BroadcastChannelOnMessage,
    #[serde(rename = "WebViewSettingsChanged")]
    WebViewSettingsChanged,
    #[serde(rename = "WebViewJavaScriptObjectChanged")]
    WebViewJavaScriptObjectChanged,
    #[serde(rename = "WebViewMessageListenerInjected")]
    WebViewMessageListenerInjected,
    #[serde(rename = "WebViewSafeBrowsingAllowlistChanged")]
    WebViewSafeBrowsingAllowlistChanged,
    #[serde(rename = "WebViewDocumentStartJavascriptChanged")]
    WebViewDocumentStartJavascriptChanged,
    #[serde(rename = "WebSocket")]
    WebSocket,
    #[serde(rename = "WebTransport")]
    WebTransport,
    #[serde(rename = "WebRTC")]
    WebRtc,
    #[serde(rename = "MainResourceHasCacheControlNoStore")]
    MainResourceHasCacheControlNoStore,
    #[serde(rename = "MainResourceHasCacheControlNoCache")]
    MainResourceHasCacheControlNoCache,
    #[serde(rename = "SubresourceHasCacheControlNoStore")]
    SubresourceHasCacheControlNoStore,
    #[serde(rename = "SubresourceHasCacheControlNoCache")]
    SubresourceHasCacheControlNoCache,
    #[serde(rename = "ContainsPlugins")]
    ContainsPlugins,
    #[serde(rename = "DocumentLoaded")]
    DocumentLoaded,
    #[serde(rename = "OutstandingNetworkRequestOthers")]
    OutstandingNetworkRequestOthers,
    #[serde(rename = "RequestedMIDIPermission")]
    RequestedMidiPermission,
    #[serde(rename = "RequestedAudioCapturePermission")]
    RequestedAudioCapturePermission,
    #[serde(rename = "RequestedVideoCapturePermission")]
    RequestedVideoCapturePermission,
    #[serde(rename = "RequestedBackForwardCacheBlockedSensors")]
    RequestedBackForwardCacheBlockedSensors,
    #[serde(rename = "RequestedBackgroundWorkPermission")]
    RequestedBackgroundWorkPermission,
    #[serde(rename = "BroadcastChannel")]
    BroadcastChannel,
    #[serde(rename = "WebXR")]
    WebXr,
    #[serde(rename = "SharedWorker")]
    SharedWorker,
    #[serde(rename = "SharedWorkerMessage")]
    SharedWorkerMessage,
    #[serde(rename = "SharedWorkerWithNoActiveClient")]
    SharedWorkerWithNoActiveClient,
    #[serde(rename = "WebLocks")]
    WebLocks,
    #[serde(rename = "WebLocksContention")]
    WebLocksContention,
    #[serde(rename = "WebHID")]
    WebHid,
    #[serde(rename = "WebBluetooth")]
    WebBluetooth,
    #[serde(rename = "WebShare")]
    WebShare,
    #[serde(rename = "RequestedStorageAccessGrant")]
    RequestedStorageAccessGrant,
    #[serde(rename = "WebNfc")]
    WebNfc,
    #[serde(rename = "OutstandingNetworkRequestFetch")]
    OutstandingNetworkRequestFetch,
    #[serde(rename = "OutstandingNetworkRequestXHR")]
    OutstandingNetworkRequestXhr,
    #[serde(rename = "AppBanner")]
    AppBanner,
    #[serde(rename = "Printing")]
    Printing,
    #[serde(rename = "WebDatabase")]
    WebDatabase,
    #[serde(rename = "PictureInPicture")]
    PictureInPicture,
    #[serde(rename = "SpeechRecognizer")]
    SpeechRecognizer,
    #[serde(rename = "IdleManager")]
    IdleManager,
    #[serde(rename = "PaymentManager")]
    PaymentManager,
    #[serde(rename = "SpeechSynthesis")]
    SpeechSynthesis,
    #[serde(rename = "KeyboardLock")]
    KeyboardLock,
    #[serde(rename = "WebOTPService")]
    WebOtpService,
    #[serde(rename = "OutstandingNetworkRequestDirectSocket")]
    OutstandingNetworkRequestDirectSocket,
    #[serde(rename = "InjectedJavascript")]
    InjectedJavascript,
    #[serde(rename = "InjectedStyleSheet")]
    InjectedStyleSheet,
    #[serde(rename = "KeepaliveRequest")]
    KeepaliveRequest,
    #[serde(rename = "IndexedDBEvent")]
    IndexedDbEvent,
    #[serde(rename = "Dummy")]
    Dummy,
    #[serde(rename = "JsNetworkRequestReceivedCacheControlNoStoreResource")]
    JsNetworkRequestReceivedCacheControlNoStoreResource,
    #[serde(rename = "WebRTCUsedWithCCNS")]
    WebRtcUsedWithCcns,
    #[serde(rename = "WebTransportUsedWithCCNS")]
    WebTransportUsedWithCcns,
    #[serde(rename = "WebSocketUsedWithCCNS")]
    WebSocketUsedWithCcns,
    #[serde(rename = "SmartCard")]
    SmartCard,
    #[serde(rename = "LiveMediaStreamTrack")]
    LiveMediaStreamTrack,
    #[serde(rename = "UnloadHandler")]
    UnloadHandler,
    #[serde(rename = "ParserAborted")]
    ParserAborted,
    #[serde(rename = "ContentSecurityHandler")]
    ContentSecurityHandler,
    #[serde(rename = "ContentWebAuthenticationAPI")]
    ContentWebAuthenticationApi,
    #[serde(rename = "ContentFileChooser")]
    ContentFileChooser,
    #[serde(rename = "ContentSerial")]
    ContentSerial,
    #[serde(rename = "ContentFileSystemAccess")]
    ContentFileSystemAccess,
    #[serde(rename = "ContentMediaDevicesDispatcherHost")]
    ContentMediaDevicesDispatcherHost,
    #[serde(rename = "ContentWebBluetooth")]
    ContentWebBluetooth,
    #[serde(rename = "ContentWebUSB")]
    ContentWebUsb,
    #[serde(rename = "ContentMediaSessionService")]
    ContentMediaSessionService,
    #[serde(rename = "ContentScreenReader")]
    ContentScreenReader,
    #[serde(rename = "ContentDiscarded")]
    ContentDiscarded,
    #[serde(rename = "EmbedderPopupBlockerTabHelper")]
    EmbedderPopupBlockerTabHelper,
    #[serde(rename = "EmbedderSafeBrowsingTriggeredPopupBlocker")]
    EmbedderSafeBrowsingTriggeredPopupBlocker,
    #[serde(rename = "EmbedderSafeBrowsingThreatDetails")]
    EmbedderSafeBrowsingThreatDetails,
    #[serde(rename = "EmbedderAppBannerManager")]
    EmbedderAppBannerManager,
    #[serde(rename = "EmbedderDomDistillerViewerSource")]
    EmbedderDomDistillerViewerSource,
    #[serde(rename = "EmbedderDomDistillerSelfDeletingRequestDelegate")]
    EmbedderDomDistillerSelfDeletingRequestDelegate,
    #[serde(rename = "EmbedderOomInterventionTabHelper")]
    EmbedderOomInterventionTabHelper,
    #[serde(rename = "EmbedderOfflinePage")]
    EmbedderOfflinePage,
    #[serde(rename = "EmbedderChromePasswordManagerClientBindCredentialManager")]
    EmbedderChromePasswordManagerClientBindCredentialManager,
    #[serde(rename = "EmbedderPermissionRequestManager")]
    EmbedderPermissionRequestManager,
    #[serde(rename = "EmbedderModalDialog")]
    EmbedderModalDialog,
    #[serde(rename = "EmbedderExtensions")]
    EmbedderExtensions,
    #[serde(rename = "EmbedderExtensionMessaging")]
    EmbedderExtensionMessaging,
    #[serde(rename = "EmbedderExtensionMessagingForOpenPort")]
    EmbedderExtensionMessagingForOpenPort,
    #[serde(rename = "EmbedderExtensionSentMessageToCachedFrame")]
    EmbedderExtensionSentMessageToCachedFrame,
    #[serde(rename = "RequestedByWebViewClient")]
    RequestedByWebViewClient,
    #[serde(rename = "PostMessageByWebViewClient")]
    PostMessageByWebViewClient,
    #[serde(rename = "CacheControlNoStoreDeviceBoundSessionTerminated")]
    CacheControlNoStoreDeviceBoundSessionTerminated,
    #[serde(rename = "CacheLimitPrunedOnModerateMemoryPressure")]
    CacheLimitPrunedOnModerateMemoryPressure,
    #[serde(rename = "CacheLimitPrunedOnCriticalMemoryPressure")]
    CacheLimitPrunedOnCriticalMemoryPressure,
}
#[allow(deprecated)]
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub enum BackForwardCacheNotRestoredReasonType {
    #[serde(rename = "SupportPending")]
    SupportPending,
    #[serde(rename = "PageSupportNeeded")]
    PageSupportNeeded,
    #[serde(rename = "Circumstantial")]
    Circumstantial,
}
#[allow(deprecated)]
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub enum CaptureScreenshotFormatOption {
    #[serde(rename = "jpeg")]
    Jpeg,
    #[serde(rename = "png")]
    Png,
    #[serde(rename = "webp")]
    Webp,
}
#[allow(deprecated)]
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub enum CaptureSnapshotFormatOption {
    #[serde(rename = "mhtml")]
    Mhtml,
}
#[allow(deprecated)]
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub enum PrintToPdfTransferModeOption {
    #[serde(rename = "ReturnAsBase64")]
    ReturnAsBase64,
    #[serde(rename = "ReturnAsStream")]
    ReturnAsStream,
}
#[allow(deprecated)]
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub enum SetDownloadBehaviorBehaviorOption {
    #[serde(rename = "deny")]
    Deny,
    #[serde(rename = "allow")]
    Allow,
    #[serde(rename = "default")]
    Default,
}
#[allow(deprecated)]
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub enum SetTouchEmulationEnabledConfigurationOption {
    #[serde(rename = "mobile")]
    Mobile,
    #[serde(rename = "desktop")]
    Desktop,
}
#[allow(deprecated)]
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub enum StartScreencastFormatOption {
    #[serde(rename = "jpeg")]
    Jpeg,
    #[serde(rename = "png")]
    Png,
}
#[allow(deprecated)]
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub enum SetWebLifecycleStateStateOption {
    #[serde(rename = "frozen")]
    Frozen,
    #[serde(rename = "active")]
    Active,
}
#[allow(deprecated)]
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub enum SetSpcTransactionModeModeOption {
    #[serde(rename = "none")]
    None,
    #[serde(rename = "autoAccept")]
    AutoAccept,
    #[serde(rename = "autoChooseToAuthAnotherWay")]
    AutoChooseToAuthAnotherWay,
    #[serde(rename = "autoReject")]
    AutoReject,
    #[serde(rename = "autoOptOut")]
    AutoOptOut,
}
#[allow(deprecated)]
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub enum SetRphRegistrationModeModeOption {
    #[serde(rename = "none")]
    None,
    #[serde(rename = "autoAccept")]
    AutoAccept,
    #[serde(rename = "autoReject")]
    AutoReject,
}
#[allow(deprecated)]
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub enum FileChooserOpenedModeOption {
    #[serde(rename = "selectSingle")]
    SelectSingle,
    #[serde(rename = "selectMultiple")]
    SelectMultiple,
}
#[allow(deprecated)]
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub enum FrameDetachedReasonOption {
    #[serde(rename = "remove")]
    Remove,
    #[serde(rename = "swap")]
    Swap,
}
#[allow(deprecated)]
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub enum FrameStartedNavigatingNavigationTypeOption {
    #[serde(rename = "reload")]
    Reload,
    #[serde(rename = "reloadBypassingCache")]
    ReloadBypassingCache,
    #[serde(rename = "restore")]
    Restore,
    #[serde(rename = "restoreWithPost")]
    RestoreWithPost,
    #[serde(rename = "historySameDocument")]
    HistorySameDocument,
    #[serde(rename = "historyDifferentDocument")]
    HistoryDifferentDocument,
    #[serde(rename = "sameDocument")]
    SameDocument,
    #[serde(rename = "differentDocument")]
    DifferentDocument,
}
#[allow(deprecated)]
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub enum DownloadProgressStateOption {
    #[serde(rename = "inProgress")]
    InProgress,
    #[serde(rename = "completed")]
    Completed,
    #[serde(rename = "canceled")]
    Canceled,
}
#[allow(deprecated)]
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub enum NavigatedWithinDocumentNavigationTypeOption {
    #[serde(rename = "fragment")]
    Fragment,
    #[serde(rename = "historyApi")]
    HistoryApi,
    #[serde(rename = "other")]
    Other,
}
#[allow(deprecated)]
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "camelCase")]
#[doc = "Indicates whether a frame has been identified as an ad and why."]
pub struct AdFrameStatus {
    pub ad_frame_type: AdFrameType,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub explanations: Option<Vec<AdFrameExplanation>>,
}
#[allow(deprecated)]
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "camelCase")]
#[doc = "Identifies the script which caused a script or frame to be labelled as an\n ad."]
pub struct AdScriptId {
    #[doc = "Script Id of the script which caused a script or frame to be labelled as\n an ad."]
    pub script_id: runtime::ScriptId,
    #[doc = "Id of scriptId's debugger."]
    pub debugger_id: runtime::UniqueDebuggerId,
}
#[allow(deprecated)]
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "camelCase")]
#[doc = "Encapsulates the script ancestry and the root script filterlist rule that\n caused the frame to be labelled as an ad. Only created when `ancestryChain`\n is not empty."]
pub struct AdScriptAncestry {
    #[doc = "A chain of `AdScriptId`s representing the ancestry of an ad script that\n led to the creation of a frame. The chain is ordered from the script\n itself (lower level) up to its root ancestor that was flagged by\n filterlist."]
    pub ancestry_chain: Vec<AdScriptId>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[doc = "The filterlist rule that caused the root (last) script in\n `ancestryChain` to be ad-tagged. Only populated if the rule is\n available."]
    pub root_script_filterlist_rule: Option<String>,
}
#[allow(deprecated)]
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "camelCase")]
pub struct PermissionsPolicyBlockLocator {
    pub frame_id: FrameId,
    pub block_reason: PermissionsPolicyBlockReason,
}
#[allow(deprecated)]
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "camelCase")]
pub struct PermissionsPolicyFeatureState {
    pub feature: PermissionsPolicyFeature,
    #[serde(default)]
    pub allowed: bool,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub locator: Option<PermissionsPolicyBlockLocator>,
}
#[allow(deprecated)]
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "camelCase")]
pub struct OriginTrialToken {
    #[serde(default)]
    pub origin: String,
    #[serde(default)]
    pub match_sub_domains: bool,
    #[serde(default)]
    pub trial_name: String,
    pub expiry_time: network::TimeSinceEpoch,
    #[serde(default)]
    pub is_third_party: bool,
    pub usage_restriction: OriginTrialUsageRestriction,
}
#[allow(deprecated)]
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "camelCase")]
pub struct OriginTrialTokenWithStatus {
    #[serde(default)]
    pub raw_token_text: String,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[doc = "`parsedToken` is present only when the token is extractable and\n parsable."]
    pub parsed_token: Option<OriginTrialToken>,
    pub status: OriginTrialTokenStatus,
}
#[allow(deprecated)]
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "camelCase")]
pub struct OriginTrial {
    #[serde(default)]
    pub trial_name: String,
    pub status: OriginTrialStatus,
    pub tokens_with_status: Vec<OriginTrialTokenWithStatus>,
}
#[allow(deprecated)]
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "camelCase")]
#[doc = "Additional information about the frame document's security origin."]
pub struct SecurityOriginDetails {
    #[serde(default)]
    #[doc = "Indicates whether the frame document's security origin is one\n of the local hostnames (e.g. \"localhost\") or IP addresses (IPv4\n 127.0.0.0/8 or IPv6 ::1)."]
    pub is_localhost: bool,
}
#[allow(deprecated)]
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "camelCase")]
#[doc = "Information about the Frame on the page."]
pub struct Frame {
    #[doc = "Frame unique identifier."]
    pub id: FrameId,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[doc = "Parent frame identifier."]
    pub parent_id: Option<FrameId>,
    #[doc = "Identifier of the loader associated with this frame."]
    pub loader_id: network::LoaderId,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[doc = "Frame's name as specified in the tag."]
    pub name: Option<String>,
    #[serde(default)]
    #[doc = "Frame document's URL without fragment."]
    pub url: String,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[doc = "Frame document's URL fragment including the '#'."]
    pub url_fragment: Option<String>,
    #[serde(default)]
    #[doc = "Frame document's registered domain, taking the public suffixes list into account.\n Extracted from the Frame's url.\n Example URLs: <http://www.google.com/file.html> -\\> \"google.com\"\n               <http://a.b.co.uk/file.html>      -\\> \"b.co.uk\""]
    pub domain_and_registry: String,
    #[serde(default)]
    #[doc = "Frame document's security origin."]
    pub security_origin: String,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[doc = "Additional details about the frame document's security origin."]
    pub security_origin_details: Option<SecurityOriginDetails>,
    #[serde(default)]
    #[doc = "Frame document's mimeType as determined by the browser."]
    pub mime_type: String,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[doc = "If the frame failed to load, this contains the URL that could not be loaded. Note that unlike url above, this URL may contain a fragment."]
    pub unreachable_url: Option<String>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[doc = "Indicates whether this frame was tagged as an ad and why."]
    pub ad_frame_status: Option<AdFrameStatus>,
    #[doc = "Indicates whether the main document is a secure context and explains why that is the case."]
    pub secure_context_type: SecureContextType,
    #[doc = "Indicates whether this is a cross origin isolated context."]
    pub cross_origin_isolated_context_type: CrossOriginIsolatedContextType,
    #[doc = "Indicated which gated APIs / features are available."]
    #[serde(rename = "gatedAPIFeatures")]
    pub gated_api_features: Vec<GatedApiFeatures>,
}
#[allow(deprecated)]
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "camelCase")]
#[doc = "Information about the Resource on the page."]
pub struct FrameResource {
    #[serde(default)]
    #[doc = "Resource URL."]
    pub url: String,
    #[doc = "Type of this resource."]
    pub r#type: network::ResourceType,
    #[serde(default)]
    #[doc = "Resource mimeType as determined by the browser."]
    pub mime_type: String,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[doc = "last-modified timestamp as reported by server."]
    pub last_modified: Option<network::TimeSinceEpoch>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[doc = "Resource content size."]
    pub content_size: Option<JsFloat>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[doc = "True if the resource failed to load."]
    pub failed: Option<bool>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[doc = "True if the resource was canceled during loading."]
    pub canceled: Option<bool>,
}
#[allow(deprecated)]
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "camelCase")]
#[doc = "Information about the Frame hierarchy along with their cached resources."]
pub struct FrameResourceTree {
    #[doc = "Frame information for this tree item."]
    pub frame: Frame,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[doc = "Child frames."]
    pub child_frames: Option<Vec<FrameResourceTree>>,
    #[doc = "Information about frame resources."]
    pub resources: Vec<FrameResource>,
}
#[allow(deprecated)]
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "camelCase")]
#[doc = "Information about the Frame hierarchy."]
pub struct FrameTree {
    #[doc = "Frame information for this tree item."]
    pub frame: Frame,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[doc = "Child frames."]
    pub child_frames: Option<Vec<FrameTree>>,
}
#[allow(deprecated)]
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "camelCase")]
#[doc = "Navigation history entry."]
pub struct NavigationEntry {
    #[serde(default)]
    #[doc = "Unique id of the navigation history entry."]
    pub id: JsUInt,
    #[serde(default)]
    #[doc = "URL of the navigation history entry."]
    pub url: String,
    #[serde(default)]
    #[doc = "URL that the user typed in the url bar."]
    #[serde(rename = "userTypedURL")]
    pub user_typed_url: String,
    #[serde(default)]
    #[doc = "Title of the navigation history entry."]
    pub title: String,
    #[doc = "Transition type."]
    pub transition_type: TransitionType,
}
#[allow(deprecated)]
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "camelCase")]
#[doc = "Screencast frame metadata."]
pub struct ScreencastFrameMetadata {
    #[serde(default)]
    #[doc = "Top offset in DIP."]
    pub offset_top: JsFloat,
    #[serde(default)]
    #[doc = "Page scale factor."]
    pub page_scale_factor: JsFloat,
    #[serde(default)]
    #[doc = "Device screen width in DIP."]
    pub device_width: JsFloat,
    #[serde(default)]
    #[doc = "Device screen height in DIP."]
    pub device_height: JsFloat,
    #[serde(default)]
    #[doc = "Position of horizontal scroll in CSS pixels."]
    pub scroll_offset_x: JsFloat,
    #[serde(default)]
    #[doc = "Position of vertical scroll in CSS pixels."]
    pub scroll_offset_y: JsFloat,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[doc = "Frame swap timestamp."]
    pub timestamp: Option<network::TimeSinceEpoch>,
}
#[allow(deprecated)]
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "camelCase")]
#[doc = "Error while paring app manifest."]
pub struct AppManifestError {
    #[serde(default)]
    #[doc = "Error message."]
    pub message: String,
    #[serde(default)]
    #[doc = "If critical, this is a non-recoverable parse error."]
    pub critical: JsUInt,
    #[serde(default)]
    #[doc = "Error line."]
    pub line: JsUInt,
    #[serde(default)]
    #[doc = "Error column."]
    pub column: JsUInt,
}
#[allow(deprecated)]
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "camelCase")]
#[doc = "Parsed app manifest properties."]
pub struct AppManifestParsedProperties {
    #[serde(default)]
    #[doc = "Computed scope value"]
    pub scope: String,
}
#[allow(deprecated)]
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "camelCase")]
#[doc = "Layout viewport position and dimensions."]
pub struct LayoutViewport {
    #[serde(default)]
    #[doc = "Horizontal offset relative to the document (CSS pixels)."]
    pub page_x: JsUInt,
    #[serde(default)]
    #[doc = "Vertical offset relative to the document (CSS pixels)."]
    pub page_y: JsUInt,
    #[serde(default)]
    #[doc = "Width (CSS pixels), excludes scrollbar if present."]
    pub client_width: JsUInt,
    #[serde(default)]
    #[doc = "Height (CSS pixels), excludes scrollbar if present."]
    pub client_height: JsUInt,
}
#[allow(deprecated)]
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "camelCase")]
#[doc = "Visual viewport position, dimensions, and scale."]
pub struct VisualViewport {
    #[serde(default)]
    #[doc = "Horizontal offset relative to the layout viewport (CSS pixels)."]
    pub offset_x: JsFloat,
    #[serde(default)]
    #[doc = "Vertical offset relative to the layout viewport (CSS pixels)."]
    pub offset_y: JsFloat,
    #[serde(default)]
    #[doc = "Horizontal offset relative to the document (CSS pixels)."]
    pub page_x: JsFloat,
    #[serde(default)]
    #[doc = "Vertical offset relative to the document (CSS pixels)."]
    pub page_y: JsFloat,
    #[serde(default)]
    #[doc = "Width (CSS pixels), excludes scrollbar if present."]
    pub client_width: JsFloat,
    #[serde(default)]
    #[doc = "Height (CSS pixels), excludes scrollbar if present."]
    pub client_height: JsFloat,
    #[serde(default)]
    #[doc = "Scale relative to the ideal viewport (size at width=device-width)."]
    pub scale: JsFloat,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[doc = "Page zoom factor (CSS to device independent pixels ratio)."]
    pub zoom: Option<JsFloat>,
}
#[allow(deprecated)]
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "camelCase")]
#[doc = "Viewport for capturing screenshot."]
pub struct Viewport {
    #[serde(default)]
    #[doc = "X offset in device independent pixels (dip)."]
    pub x: JsFloat,
    #[serde(default)]
    #[doc = "Y offset in device independent pixels (dip)."]
    pub y: JsFloat,
    #[serde(default)]
    #[doc = "Rectangle width in device independent pixels (dip)."]
    pub width: JsFloat,
    #[serde(default)]
    #[doc = "Rectangle height in device independent pixels (dip)."]
    pub height: JsFloat,
    #[serde(default)]
    #[doc = "Page scale factor."]
    pub scale: JsFloat,
}
#[allow(deprecated)]
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "camelCase")]
#[doc = "Generic font families collection."]
pub struct FontFamilies {
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[doc = "The standard font-family."]
    pub standard: Option<String>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[doc = "The fixed font-family."]
    pub fixed: Option<String>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[doc = "The serif font-family."]
    pub serif: Option<String>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[doc = "The sansSerif font-family."]
    pub sans_serif: Option<String>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[doc = "The cursive font-family."]
    pub cursive: Option<String>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[doc = "The fantasy font-family."]
    pub fantasy: Option<String>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[doc = "The math font-family."]
    pub math: Option<String>,
}
#[allow(deprecated)]
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "camelCase")]
#[doc = "Font families collection for a script."]
pub struct ScriptFontFamilies {
    #[serde(default)]
    #[doc = "Name of the script which these font families are defined for."]
    pub script: String,
    #[doc = "Generic font families collection for the script."]
    pub font_families: FontFamilies,
}
#[allow(deprecated)]
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "camelCase")]
#[doc = "Default font sizes."]
pub struct FontSizes {
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[doc = "Default standard font size."]
    pub standard: Option<JsUInt>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[doc = "Default fixed font size."]
    pub fixed: Option<JsUInt>,
}
#[allow(deprecated)]
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "camelCase")]
pub struct InstallabilityErrorArgument {
    #[serde(default)]
    #[doc = "Argument name (e.g. name:'minimum-icon-size-in-pixels')."]
    pub name: String,
    #[serde(default)]
    #[doc = "Argument value (e.g. value:'64')."]
    pub value: String,
}
#[allow(deprecated)]
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "camelCase")]
#[doc = "The installability error"]
pub struct InstallabilityError {
    #[serde(default)]
    #[doc = "The error id (e.g. 'manifest-missing-suitable-icon')."]
    pub error_id: String,
    #[doc = "The list of error arguments (e.g. {name:'minimum-icon-size-in-pixels', value:'64'})."]
    pub error_arguments: Vec<InstallabilityErrorArgument>,
}
#[allow(deprecated)]
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "camelCase")]
#[doc = "Per-script compilation cache parameters for `Page.produceCompilationCache`"]
pub struct CompilationCacheParams {
    #[serde(default)]
    #[doc = "The URL of the script to produce a compilation cache entry for."]
    pub url: String,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[doc = "A hint to the backend whether eager compilation is recommended.\n (the actual compilation mode used is upon backend discretion)."]
    pub eager: Option<bool>,
}
#[allow(deprecated)]
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "camelCase")]
pub struct FileFilter {
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub name: Option<String>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub accepts: Option<Vec<String>>,
}
#[allow(deprecated)]
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "camelCase")]
pub struct FileHandler {
    #[serde(default)]
    pub action: String,
    #[serde(default)]
    pub name: String,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub icons: Option<Vec<ImageResource>>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[doc = "Mimic a map, name is the key, accepts is the value."]
    pub accepts: Option<Vec<FileFilter>>,
    #[serde(default)]
    #[doc = "Won't repeat the enums, using string for easy comparison. Same as the\n other enums below."]
    pub launch_type: String,
}
#[allow(deprecated)]
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "camelCase")]
#[doc = "The image definition used in both icon and screenshot."]
pub struct ImageResource {
    #[serde(default)]
    #[doc = "The src field in the definition, but changing to url in favor of\n consistency."]
    pub url: String,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub sizes: Option<String>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub r#type: Option<String>,
}
#[allow(deprecated)]
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "camelCase")]
pub struct LaunchHandler {
    #[serde(default)]
    pub client_mode: String,
}
#[allow(deprecated)]
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "camelCase")]
pub struct ProtocolHandler {
    #[serde(default)]
    pub protocol: String,
    #[serde(default)]
    pub url: String,
}
#[allow(deprecated)]
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "camelCase")]
pub struct RelatedApplication {
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub id: Option<String>,
    #[serde(default)]
    pub url: String,
}
#[allow(deprecated)]
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "camelCase")]
pub struct ScopeExtension {
    #[serde(default)]
    #[doc = "Instead of using tuple, this field always returns the serialized string\n for easy understanding and comparison."]
    pub origin: String,
    #[serde(default)]
    pub has_origin_wildcard: bool,
}
#[allow(deprecated)]
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "camelCase")]
pub struct Screenshot {
    pub image: ImageResource,
    #[serde(default)]
    pub form_factor: String,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub label: Option<String>,
}
#[allow(deprecated)]
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "camelCase")]
pub struct ShareTarget {
    #[serde(default)]
    pub action: String,
    #[serde(default)]
    pub method: String,
    #[serde(default)]
    pub enctype: String,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[doc = "Embed the ShareTargetParams"]
    pub title: Option<String>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub text: Option<String>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub url: Option<String>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub files: Option<Vec<FileFilter>>,
}
#[allow(deprecated)]
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "camelCase")]
pub struct Shortcut {
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    pub url: String,
}
#[allow(deprecated)]
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "camelCase")]
pub struct WebAppManifest {
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub background_color: Option<String>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[doc = "The extra description provided by the manifest."]
    pub description: Option<String>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub dir: Option<String>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub display: Option<String>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[doc = "The overrided display mode controlled by the user."]
    pub display_overrides: Option<Vec<String>>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[doc = "The handlers to open files."]
    pub file_handlers: Option<Vec<FileHandler>>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub icons: Option<Vec<ImageResource>>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub id: Option<String>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub lang: Option<String>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[doc = "TODO(crbug.com/1231886): This field is non-standard and part of a Chrome\n experiment. See:\n <https://github.com/WICG/web-app-launch/blob/main/launch_handler.md>"]
    pub launch_handler: Option<LaunchHandler>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub name: Option<String>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub orientation: Option<String>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub prefer_related_applications: Option<bool>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[doc = "The handlers to open protocols."]
    pub protocol_handlers: Option<Vec<ProtocolHandler>>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub related_applications: Option<Vec<RelatedApplication>>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub scope: Option<String>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[doc = "Non-standard, see\n <https://github.com/WICG/manifest-incubations/blob/gh-pages/scope_extensions-explainer.md>"]
    pub scope_extensions: Option<Vec<ScopeExtension>>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[doc = "The screenshots used by chromium."]
    pub screenshots: Option<Vec<Screenshot>>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub share_target: Option<ShareTarget>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub short_name: Option<String>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shortcuts: Option<Vec<Shortcut>>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub start_url: Option<String>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub theme_color: Option<String>,
}
#[allow(deprecated)]
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "camelCase")]
pub struct BackForwardCacheBlockingDetails {
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[doc = "Url of the file where blockage happened. Optional because of tests."]
    pub url: Option<String>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[doc = "Function name where blockage happened. Optional because of anonymous functions and tests."]
    pub function: Option<String>,
    #[serde(default)]
    #[doc = "Line number in the script (0-based)."]
    pub line_number: JsUInt,
    #[serde(default)]
    #[doc = "Column number in the script (0-based)."]
    pub column_number: JsUInt,
}
#[allow(deprecated)]
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "camelCase")]
pub struct BackForwardCacheNotRestoredExplanation {
    #[doc = "Type of the reason"]
    pub r#type: BackForwardCacheNotRestoredReasonType,
    #[doc = "Not restored reason"]
    pub reason: BackForwardCacheNotRestoredReason,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[doc = "Context associated with the reason. The meaning of this context is\n dependent on the reason:\n - EmbedderExtensionSentMessageToCachedFrame: the extension ID."]
    pub context: Option<String>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub details: Option<Vec<BackForwardCacheBlockingDetails>>,
}
#[allow(deprecated)]
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "camelCase")]
pub struct BackForwardCacheNotRestoredExplanationTree {
    #[serde(default)]
    #[doc = "URL of each frame"]
    pub url: String,
    #[doc = "Not restored reasons of each frame"]
    pub explanations: Vec<BackForwardCacheNotRestoredExplanation>,
    #[doc = "Array of children frame"]
    pub children: Vec<BackForwardCacheNotRestoredExplanationTree>,
}
#[allow(deprecated)]
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "camelCase")]
#[doc = "Deprecated, please use addScriptToEvaluateOnNewDocument instead."]
#[deprecated]
pub struct AddScriptToEvaluateOnLoad {
    #[serde(default)]
    pub script_source: String,
}
#[allow(deprecated)]
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "camelCase")]
#[doc = "Evaluates given script in every frame upon creation (before loading frame's scripts)."]
pub struct AddScriptToEvaluateOnNewDocument {
    #[serde(default)]
    pub source: String,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[doc = "If specified, creates an isolated world with the given name and evaluates given script in it.\n This world name will be used as the ExecutionContextDescription::name when the corresponding\n event is emitted."]
    pub world_name: Option<String>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[doc = "Specifies whether command line API should be available to the script, defaults\n to false."]
    #[serde(rename = "includeCommandLineAPI")]
    pub include_command_line_api: Option<bool>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[doc = "If true, runs the script immediately on existing execution contexts or worlds.\n Default: false."]
    pub run_immediately: Option<bool>,
}
#[allow(deprecated)]
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct BringToFront(pub Option<Json>);
#[allow(deprecated)]
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "camelCase")]
#[doc = "Capture page screenshot."]
pub struct CaptureScreenshot {
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[doc = "Image compression format (defaults to png)."]
    pub format: Option<CaptureScreenshotFormatOption>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[doc = "Compression quality from range \\[0..100\\] (jpeg only)."]
    pub quality: Option<JsUInt>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[doc = "Capture the screenshot of a given region only."]
    pub clip: Option<Viewport>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[doc = "Capture the screenshot from the surface, rather than the view. Defaults to true."]
    pub from_surface: Option<bool>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[doc = "Capture the screenshot beyond the viewport. Defaults to false."]
    pub capture_beyond_viewport: Option<bool>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[doc = "Optimize image encoding for speed, not for resulting size (defaults to false)"]
    pub optimize_for_speed: Option<bool>,
}
#[allow(deprecated)]
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "camelCase")]
#[doc = "Returns a snapshot of the page as a string. For MHTML format, the serialization includes\n iframes, shadow DOM, external resources, and element-inline styles."]
pub struct CaptureSnapshot {
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[doc = "Format (defaults to mhtml)."]
    pub format: Option<CaptureSnapshotFormatOption>,
}
#[allow(deprecated)]
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct ClearDeviceMetricsOverride(pub Option<Json>);
#[allow(deprecated)]
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct ClearDeviceOrientationOverride(pub Option<Json>);
#[allow(deprecated)]
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct ClearGeolocationOverride(pub Option<Json>);
#[allow(deprecated)]
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "camelCase")]
#[doc = "Creates an isolated world for the given frame."]
pub struct CreateIsolatedWorld {
    #[doc = "Id of the frame in which the isolated world should be created."]
    pub frame_id: FrameId,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[doc = "An optional name which is reported in the Execution Context."]
    pub world_name: Option<String>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[doc = "Whether or not universal access should be granted to the isolated world. This is a powerful\n option, use with caution."]
    pub grant_univeral_access: Option<bool>,
}
#[allow(deprecated)]
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "camelCase")]
#[doc = "Deletes browser cookie with given name, domain and path."]
#[deprecated]
pub struct DeleteCookie {
    #[serde(default)]
    #[doc = "Name of the cookie to remove."]
    pub cookie_name: String,
    #[serde(default)]
    #[doc = "URL to match cooke domain and path."]
    pub url: String,
}
#[allow(deprecated)]
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct Disable(pub Option<Json>);
#[allow(deprecated)]
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "camelCase")]
#[doc = "Enables page domain notifications."]
pub struct Enable {
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[doc = "If true, the `Page.fileChooserOpened` event will be emitted regardless of the state set by\n `Page.setInterceptFileChooserDialog` command (default: false)."]
    pub enable_file_chooser_opened_event: Option<bool>,
}
#[allow(deprecated)]
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "camelCase")]
#[doc = "Gets the processed manifest for this current document.\n   This API always waits for the manifest to be loaded.\n   If manifestId is provided, and it does not match the manifest of the\n     current document, this API errors out.\n   If there is not a loaded page, this API errors out immediately."]
pub struct GetAppManifest {
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub manifest_id: Option<String>,
}
#[allow(deprecated)]
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct GetInstallabilityErrors(pub Option<Json>);
#[allow(deprecated)]
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct GetManifestIcons(pub Option<Json>);
#[allow(deprecated)]
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct GetAppId(pub Option<Json>);
#[allow(deprecated)]
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "camelCase")]
pub struct GetAdScriptAncestry {
    pub frame_id: FrameId,
}
#[allow(deprecated)]
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct GetFrameTree(pub Option<Json>);
#[allow(deprecated)]
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct GetLayoutMetrics(pub Option<Json>);
#[allow(deprecated)]
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct GetNavigationHistory(pub Option<Json>);
#[allow(deprecated)]
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct ResetNavigationHistory(pub Option<Json>);
#[allow(deprecated)]
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "camelCase")]
#[doc = "Returns content of the given resource."]
pub struct GetResourceContent {
    #[doc = "Frame id to get resource for."]
    pub frame_id: FrameId,
    #[serde(default)]
    #[doc = "URL of the resource to get content for."]
    pub url: String,
}
#[allow(deprecated)]
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct GetResourceTree(pub Option<Json>);
#[allow(deprecated)]
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "camelCase")]
#[doc = "Accepts or dismisses a JavaScript initiated dialog (alert, confirm, prompt, or onbeforeunload)."]
pub struct HandleJavaScriptDialog {
    #[serde(default)]
    #[doc = "Whether to accept or dismiss the dialog."]
    pub accept: bool,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[doc = "The text to enter into the dialog prompt before accepting. Used only if this is a prompt\n dialog."]
    pub prompt_text: Option<String>,
}
#[allow(deprecated)]
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "camelCase")]
#[doc = "Navigates current page to the given URL."]
pub struct Navigate {
    #[serde(default)]
    #[doc = "URL to navigate the page to."]
    pub url: String,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[doc = "Referrer URL."]
    pub referrer: Option<String>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[doc = "Intended transition type."]
    pub transition_type: Option<TransitionType>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[doc = "Frame id to navigate, if not specified navigates the top frame."]
    pub frame_id: Option<FrameId>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[doc = "Referrer-policy used for the navigation."]
    pub referrer_policy: Option<ReferrerPolicy>,
}
#[allow(deprecated)]
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "camelCase")]
#[doc = "Navigates current page to the given history entry."]
pub struct NavigateToHistoryEntry {
    #[serde(default)]
    #[doc = "Unique id of the entry to navigate to."]
    pub entry_id: JsUInt,
}
#[allow(deprecated)]
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "camelCase")]
#[doc = "Print page as PDF."]
pub struct PrintToPDF {
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[doc = "Paper orientation. Defaults to false."]
    pub landscape: Option<bool>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[doc = "Display header and footer. Defaults to false."]
    pub display_header_footer: Option<bool>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[doc = "Print background graphics. Defaults to false."]
    pub print_background: Option<bool>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[doc = "Scale of the webpage rendering. Defaults to 1."]
    pub scale: Option<JsFloat>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[doc = "Paper width in inches. Defaults to 8.5 inches."]
    pub paper_width: Option<JsFloat>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[doc = "Paper height in inches. Defaults to 11 inches."]
    pub paper_height: Option<JsFloat>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[doc = "Top margin in inches. Defaults to 1cm (~0.4 inches)."]
    pub margin_top: Option<JsFloat>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[doc = "Bottom margin in inches. Defaults to 1cm (~0.4 inches)."]
    pub margin_bottom: Option<JsFloat>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[doc = "Left margin in inches. Defaults to 1cm (~0.4 inches)."]
    pub margin_left: Option<JsFloat>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[doc = "Right margin in inches. Defaults to 1cm (~0.4 inches)."]
    pub margin_right: Option<JsFloat>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[doc = "Paper ranges to print, one based, e.g., '1-5, 8, 11-13'. Pages are\n printed in the document order, not in the order specified, and no\n more than once.\n Defaults to empty string, which implies the entire document is printed.\n The page numbers are quietly capped to actual page count of the\n document, and ranges beyond the end of the document are ignored.\n If this results in no pages to print, an error is reported.\n It is an error to specify a range with start greater than end."]
    pub page_ranges: Option<String>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[doc = "HTML template for the print header. Should be valid HTML markup with following\n classes used to inject printing values into them:\n - `date`: formatted print date\n - `title`: document title\n - `url`: document location\n - `pageNumber`: current page number\n - `totalPages`: total pages in the document\n \n For example, `\\<span class=title\\>\\</span\\>` would generate span containing the title."]
    pub header_template: Option<String>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[doc = "HTML template for the print footer. Should use the same format as the `headerTemplate`."]
    pub footer_template: Option<String>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[doc = "Whether or not to prefer page size as defined by css. Defaults to false,\n in which case the content will be scaled to fit the paper size."]
    #[serde(rename = "preferCSSPageSize")]
    pub prefer_css_page_size: Option<bool>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[doc = "return as stream"]
    pub transfer_mode: Option<PrintToPdfTransferModeOption>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[doc = "Whether or not to generate tagged (accessible) PDF. Defaults to embedder choice."]
    #[serde(rename = "generateTaggedPDF")]
    pub generate_tagged_pdf: Option<bool>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[doc = "Whether or not to embed the document outline into the PDF."]
    pub generate_document_outline: Option<bool>,
}
#[allow(deprecated)]
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "camelCase")]
#[doc = "Reloads given page optionally ignoring the cache."]
pub struct Reload {
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[doc = "If true, browser cache is ignored (as if the user pressed Shift+refresh)."]
    pub ignore_cache: Option<bool>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[doc = "If set, the script will be injected into all frames of the inspected page after reload.\n Argument will be ignored if reloading dataURL origin."]
    pub script_to_evaluate_on_load: Option<String>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[doc = "If set, an error will be thrown if the target page's main frame's\n loader id does not match the provided id. This prevents accidentally\n reloading an unintended target in case there's a racing navigation."]
    pub loader_id: Option<network::LoaderId>,
}
#[allow(deprecated)]
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "camelCase")]
#[doc = "Deprecated, please use removeScriptToEvaluateOnNewDocument instead."]
#[deprecated]
pub struct RemoveScriptToEvaluateOnLoad {
    pub identifier: ScriptIdentifier,
}
#[allow(deprecated)]
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "camelCase")]
#[doc = "Removes given script from the list."]
pub struct RemoveScriptToEvaluateOnNewDocument {
    pub identifier: ScriptIdentifier,
}
#[allow(deprecated)]
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "camelCase")]
#[doc = "Acknowledges that a screencast frame has been received by the frontend."]
pub struct ScreencastFrameAck {
    #[serde(default)]
    #[doc = "Frame number."]
    pub session_id: JsUInt,
}
#[allow(deprecated)]
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "camelCase")]
#[doc = "Searches for given string in resource content."]
pub struct SearchInResource {
    #[doc = "Frame id for resource to search in."]
    pub frame_id: FrameId,
    #[serde(default)]
    #[doc = "URL of the resource to search in."]
    pub url: String,
    #[serde(default)]
    #[doc = "String to search for."]
    pub query: String,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[doc = "If true, search is case sensitive."]
    pub case_sensitive: Option<bool>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[doc = "If true, treats string parameter as regex."]
    pub is_regex: Option<bool>,
}
#[allow(deprecated)]
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "camelCase")]
#[doc = "Enable Chrome's experimental ad filter on all sites."]
pub struct SetAdBlockingEnabled {
    #[serde(default)]
    #[doc = "Whether to block ads."]
    pub enabled: bool,
}
#[allow(deprecated)]
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "camelCase")]
#[doc = "Enable page Content Security Policy by-passing."]
pub struct SetBypassCSP {
    #[serde(default)]
    #[doc = "Whether to bypass page CSP."]
    pub enabled: bool,
}
#[allow(deprecated)]
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "camelCase")]
#[doc = "Get Permissions Policy state on given frame."]
pub struct GetPermissionsPolicyState {
    pub frame_id: FrameId,
}
#[allow(deprecated)]
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "camelCase")]
#[doc = "Get Origin Trials on given frame."]
pub struct GetOriginTrials {
    pub frame_id: FrameId,
}
#[allow(deprecated)]
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "camelCase")]
#[doc = "Overrides the values of device screen dimensions (window.screen.width, window.screen.height,\n window.innerWidth, window.innerHeight, and \"device-width\"/\"device-height\"-related CSS media\n query results)."]
#[deprecated]
pub struct SetDeviceMetricsOverride {
    #[serde(default)]
    #[doc = "Overriding width value in pixels (minimum 0, maximum 10000000). 0 disables the override."]
    pub width: JsUInt,
    #[serde(default)]
    #[doc = "Overriding height value in pixels (minimum 0, maximum 10000000). 0 disables the override."]
    pub height: JsUInt,
    #[serde(default)]
    #[doc = "Overriding device scale factor value. 0 disables the override."]
    pub device_scale_factor: JsFloat,
    #[serde(default)]
    #[doc = "Whether to emulate mobile device. This includes viewport meta tag, overlay scrollbars, text\n autosizing and more."]
    pub mobile: bool,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[doc = "Scale to apply to resulting view image."]
    pub scale: Option<JsFloat>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[doc = "Overriding screen width value in pixels (minimum 0, maximum 10000000)."]
    pub screen_width: Option<JsUInt>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[doc = "Overriding screen height value in pixels (minimum 0, maximum 10000000)."]
    pub screen_height: Option<JsUInt>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[doc = "Overriding view X position on screen in pixels (minimum 0, maximum 10000000)."]
    pub position_x: Option<JsUInt>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[doc = "Overriding view Y position on screen in pixels (minimum 0, maximum 10000000)."]
    pub position_y: Option<JsUInt>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[doc = "Do not set visible view size, rely upon explicit setVisibleSize call."]
    pub dont_set_visible_size: Option<bool>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[doc = "Screen orientation override."]
    pub screen_orientation: Option<emulation::ScreenOrientation>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[doc = "The viewport dimensions and scale. If not set, the override is cleared."]
    pub viewport: Option<Viewport>,
}
#[allow(deprecated)]
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "camelCase")]
#[doc = "Overrides the Device Orientation."]
#[deprecated]
pub struct SetDeviceOrientationOverride {
    #[serde(default)]
    #[doc = "Mock alpha"]
    pub alpha: JsFloat,
    #[serde(default)]
    #[doc = "Mock beta"]
    pub beta: JsFloat,
    #[serde(default)]
    #[doc = "Mock gamma"]
    pub gamma: JsFloat,
}
#[allow(deprecated)]
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "camelCase")]
#[doc = "Set generic font families."]
pub struct SetFontFamilies {
    #[doc = "Specifies font families to set. If a font family is not specified, it won't be changed."]
    pub font_families: FontFamilies,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[doc = "Specifies font families to set for individual scripts."]
    pub for_scripts: Option<Vec<ScriptFontFamilies>>,
}
#[allow(deprecated)]
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "camelCase")]
#[doc = "Set default font sizes."]
pub struct SetFontSizes {
    #[doc = "Specifies font sizes to set. If a font size is not specified, it won't be changed."]
    pub font_sizes: FontSizes,
}
#[allow(deprecated)]
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "camelCase")]
#[doc = "Sets given markup as the document's HTML."]
pub struct SetDocumentContent {
    #[doc = "Frame id to set HTML for."]
    pub frame_id: FrameId,
    #[serde(default)]
    #[doc = "HTML content to set."]
    pub html: String,
}
#[allow(deprecated)]
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "camelCase")]
#[doc = "Set the behavior when downloading a file."]
#[deprecated]
pub struct SetDownloadBehavior {
    #[doc = "Whether to allow all or deny all download requests, or use default Chrome behavior if\n available (otherwise deny)."]
    pub behavior: SetDownloadBehaviorBehaviorOption,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[doc = "The default path to save downloaded files to. This is required if behavior is set to 'allow'"]
    pub download_path: Option<String>,
}
#[allow(deprecated)]
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "camelCase")]
#[doc = "Overrides the Geolocation Position or Error. Omitting any of the parameters emulates position\n unavailable."]
#[deprecated]
pub struct SetGeolocationOverride {
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[doc = "Mock latitude"]
    pub latitude: Option<JsFloat>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[doc = "Mock longitude"]
    pub longitude: Option<JsFloat>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[doc = "Mock accuracy"]
    pub accuracy: Option<JsFloat>,
}
#[allow(deprecated)]
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "camelCase")]
#[doc = "Controls whether page will emit lifecycle events."]
pub struct SetLifecycleEventsEnabled {
    #[serde(default)]
    #[doc = "If true, starts emitting lifecycle events."]
    pub enabled: bool,
}
#[allow(deprecated)]
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "camelCase")]
#[doc = "Toggles mouse event-based touch event emulation."]
#[deprecated]
pub struct SetTouchEmulationEnabled {
    #[serde(default)]
    #[doc = "Whether the touch event emulation should be enabled."]
    pub enabled: bool,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[doc = "Touch/gesture events configuration. Default: current platform."]
    pub configuration: Option<SetTouchEmulationEnabledConfigurationOption>,
}
#[allow(deprecated)]
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "camelCase")]
#[doc = "Starts sending each frame using the `screencastFrame` event."]
pub struct StartScreencast {
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[doc = "Image compression format."]
    pub format: Option<StartScreencastFormatOption>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[doc = "Compression quality from range \\[0..100\\]."]
    pub quality: Option<JsUInt>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[doc = "Maximum screenshot width."]
    pub max_width: Option<JsUInt>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[doc = "Maximum screenshot height."]
    pub max_height: Option<JsUInt>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[doc = "Send every n-th frame."]
    pub every_nth_frame: Option<JsUInt>,
}
#[allow(deprecated)]
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct StopLoading(pub Option<Json>);
#[allow(deprecated)]
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct Crash(pub Option<Json>);
#[allow(deprecated)]
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct Close(pub Option<Json>);
#[allow(deprecated)]
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "camelCase")]
#[doc = "Tries to update the web lifecycle state of the page.\n It will transition the page to the given state according to:\n <https://github.com/WICG/web-lifecycle/>"]
pub struct SetWebLifecycleState {
    #[doc = "Target lifecycle state"]
    pub state: SetWebLifecycleStateStateOption,
}
#[allow(deprecated)]
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct StopScreencast(pub Option<Json>);
#[allow(deprecated)]
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "camelCase")]
#[doc = "Requests backend to produce compilation cache for the specified scripts.\n `scripts` are appended to the list of scripts for which the cache\n would be produced. The list may be reset during page navigation.\n When script with a matching URL is encountered, the cache is optionally\n produced upon backend discretion, based on internal heuristics.\n See also: `Page.compilationCacheProduced`."]
pub struct ProduceCompilationCache {
    pub scripts: Vec<CompilationCacheParams>,
}
#[allow(deprecated)]
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "camelCase")]
#[doc = "Seeds compilation cache for given url. Compilation cache does not survive\n cross-process navigation."]
pub struct AddCompilationCache {
    #[serde(default)]
    pub url: String,
    #[doc = "Base64-encoded data"]
    pub data: Vec<u8>,
}
#[allow(deprecated)]
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct ClearCompilationCache(pub Option<Json>);
#[allow(deprecated)]
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "camelCase")]
#[doc = "Sets the Secure Payment Confirmation transaction mode.\n <https://w3c.github.io/secure-payment-confirmation/#sctn-automation-set-spc-transaction-mode>"]
pub struct SetSPCTransactionMode {
    pub mode: SetSpcTransactionModeModeOption,
}
#[allow(deprecated)]
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "camelCase")]
#[doc = "Extensions for Custom Handlers API:\n <https://html.spec.whatwg.org/multipage/system-state.html#rph-automation>"]
pub struct SetRPHRegistrationMode {
    pub mode: SetRphRegistrationModeModeOption,
}
#[allow(deprecated)]
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "camelCase")]
#[doc = "Generates a report for testing."]
pub struct GenerateTestReport {
    #[serde(default)]
    #[doc = "Message to be displayed in the report."]
    pub message: String,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[doc = "Specifies the endpoint group to deliver the report to."]
    pub group: Option<String>,
}
#[allow(deprecated)]
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct WaitForDebugger(pub Option<Json>);
#[allow(deprecated)]
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "camelCase")]
#[doc = "Intercept file chooser requests and transfer control to protocol clients.\n When file chooser interception is enabled, native file chooser dialog is not shown.\n Instead, a protocol event `Page.fileChooserOpened` is emitted."]
pub struct SetInterceptFileChooserDialog {
    #[serde(default)]
    pub enabled: bool,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[doc = "If true, cancels the dialog by emitting relevant events (if any)\n in addition to not showing it if the interception is enabled\n (default: false)."]
    pub cancel: Option<bool>,
}
#[allow(deprecated)]
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "camelCase")]
#[doc = "Enable/disable prerendering manually.\n \n This command is a short-term solution for <https://crbug.com/1440085>.\n See <https://docs.google.com/document/d/12HVmFxYj5Jc-eJr5OmWsa2bqTJsbgGLKI6ZIyx0_wpA>\n for more details.\n \n TODO(<https://crbug.com/1440085>): Remove this once Puppeteer supports tab targets."]
pub struct SetPrerenderingAllowed {
    #[serde(default)]
    pub is_allowed: bool,
}
#[allow(deprecated)]
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "camelCase")]
#[doc = "Get the annotated page content for the main frame.\n This is an experimental command that is subject to change."]
pub struct GetAnnotatedPageContent {
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[doc = "Whether to include actionable information. Defaults to true."]
    pub include_actionable_information: Option<bool>,
}
#[allow(deprecated)]
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[serde(rename_all = "camelCase")]
#[doc = "Deprecated, please use addScriptToEvaluateOnNewDocument instead."]
#[deprecated]
pub struct AddScriptToEvaluateOnLoadReturnObject {
    #[doc = "Identifier of the added script."]
    pub identifier: ScriptIdentifier,
}
#[allow(deprecated)]
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[serde(rename_all = "camelCase")]
#[doc = "Evaluates given script in every frame upon creation (before loading frame's scripts)."]
pub struct AddScriptToEvaluateOnNewDocumentReturnObject {
    #[doc = "Identifier of the added script."]
    pub identifier: ScriptIdentifier,
}
#[allow(deprecated)]
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[doc = "Brings page to front (activates tab)."]
pub struct BringToFrontReturnObject(pub Option<Json>);
#[allow(deprecated)]
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[serde(rename_all = "camelCase")]
#[doc = "Capture page screenshot."]
pub struct CaptureScreenshotReturnObject {
    #[doc = "Base64-encoded image data."]
    pub data: String,
}
#[allow(deprecated)]
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[serde(rename_all = "camelCase")]
#[doc = "Returns a snapshot of the page as a string. For MHTML format, the serialization includes\n iframes, shadow DOM, external resources, and element-inline styles."]
pub struct CaptureSnapshotReturnObject {
    #[serde(default)]
    #[doc = "Serialized page data."]
    pub data: String,
}
#[allow(deprecated)]
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[doc = "Clears the overridden device metrics."]
#[deprecated]
pub struct ClearDeviceMetricsOverrideReturnObject(pub Option<Json>);
#[allow(deprecated)]
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[doc = "Clears the overridden Device Orientation."]
#[deprecated]
pub struct ClearDeviceOrientationOverrideReturnObject(pub Option<Json>);
#[allow(deprecated)]
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[doc = "Clears the overridden Geolocation Position and Error."]
#[deprecated]
pub struct ClearGeolocationOverrideReturnObject(pub Option<Json>);
#[allow(deprecated)]
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[serde(rename_all = "camelCase")]
#[doc = "Creates an isolated world for the given frame."]
pub struct CreateIsolatedWorldReturnObject {
    #[doc = "Execution context of the isolated world."]
    pub execution_context_id: runtime::ExecutionContextId,
}
#[allow(deprecated)]
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[doc = "Deletes browser cookie with given name, domain and path."]
#[deprecated]
pub struct DeleteCookieReturnObject(pub Option<Json>);
#[allow(deprecated)]
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[doc = "Disables page domain notifications."]
pub struct DisableReturnObject(pub Option<Json>);
#[allow(deprecated)]
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[doc = "Enables page domain notifications."]
pub struct EnableReturnObject(pub Option<Json>);
#[allow(deprecated)]
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[serde(rename_all = "camelCase")]
#[doc = "Gets the processed manifest for this current document.\n   This API always waits for the manifest to be loaded.\n   If manifestId is provided, and it does not match the manifest of the\n     current document, this API errors out.\n   If there is not a loaded page, this API errors out immediately."]
pub struct GetAppManifestReturnObject {
    #[serde(default)]
    #[doc = "Manifest location."]
    pub url: String,
    pub errors: Vec<AppManifestError>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[doc = "Manifest content."]
    pub data: Option<String>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[doc = "Parsed manifest properties. Deprecated, use manifest instead."]
    #[deprecated]
    pub parsed: Option<AppManifestParsedProperties>,
    pub manifest: WebAppManifest,
}
#[allow(deprecated)]
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[serde(rename_all = "camelCase")]
pub struct GetInstallabilityErrorsReturnObject {
    pub installability_errors: Vec<InstallabilityError>,
}
#[allow(deprecated)]
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[serde(rename_all = "camelCase")]
#[doc = "Deprecated because it's not guaranteed that the returned icon is in fact the one used for PWA installation."]
#[deprecated]
pub struct GetManifestIconsReturnObject {
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub primary_icon: Option<String>,
}
#[allow(deprecated)]
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[serde(rename_all = "camelCase")]
#[doc = "Returns the unique (PWA) app id.\n Only returns values if the feature flag 'WebAppEnableManifestId' is enabled"]
pub struct GetAppIdReturnObject {
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[doc = "App id, either from manifest's id attribute or computed from start_url"]
    pub app_id: Option<String>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[doc = "Recommendation for manifest's id attribute to match current id computed from start_url"]
    pub recommended_id: Option<String>,
}
#[allow(deprecated)]
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[serde(rename_all = "camelCase")]
pub struct GetAdScriptAncestryReturnObject {
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[doc = "The ancestry chain of ad script identifiers leading to this frame's\n creation, along with the root script's filterlist rule. The ancestry\n chain is ordered from the most immediate script (in the frame creation\n stack) to more distant ancestors (that created the immediately preceding\n script). Only sent if frame is labelled as an ad and ids are available."]
    pub ad_script_ancestry: Option<AdScriptAncestry>,
}
#[allow(deprecated)]
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[serde(rename_all = "camelCase")]
#[doc = "Returns present frame tree structure."]
pub struct GetFrameTreeReturnObject {
    #[doc = "Present frame tree structure."]
    pub frame_tree: FrameTree,
}
#[allow(deprecated)]
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[serde(rename_all = "camelCase")]
#[doc = "Returns metrics relating to the layouting of the page, such as viewport bounds/scale."]
pub struct GetLayoutMetricsReturnObject {
    #[doc = "Deprecated metrics relating to the layout viewport. Is in device pixels. Use `cssLayoutViewport` instead."]
    #[deprecated]
    pub layout_viewport: LayoutViewport,
    #[doc = "Deprecated metrics relating to the visual viewport. Is in device pixels. Use `cssVisualViewport` instead."]
    #[deprecated]
    pub visual_viewport: VisualViewport,
    #[doc = "Deprecated size of scrollable area. Is in DP. Use `cssContentSize` instead."]
    #[deprecated]
    pub content_size: dom::Rect,
    #[doc = "Metrics relating to the layout viewport in CSS pixels."]
    pub css_layout_viewport: LayoutViewport,
    #[doc = "Metrics relating to the visual viewport in CSS pixels."]
    pub css_visual_viewport: VisualViewport,
    #[doc = "Size of scrollable area in CSS pixels."]
    pub css_content_size: dom::Rect,
}
#[allow(deprecated)]
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[serde(rename_all = "camelCase")]
#[doc = "Returns navigation history for the current page."]
pub struct GetNavigationHistoryReturnObject {
    #[serde(default)]
    #[doc = "Index of the current navigation history entry."]
    pub current_index: JsUInt,
    #[doc = "Array of navigation history entries."]
    pub entries: Vec<NavigationEntry>,
}
#[allow(deprecated)]
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[doc = "Resets navigation history for the current page."]
pub struct ResetNavigationHistoryReturnObject(pub Option<Json>);
#[allow(deprecated)]
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[serde(rename_all = "camelCase")]
#[doc = "Returns content of the given resource."]
pub struct GetResourceContentReturnObject {
    #[serde(default)]
    #[doc = "Resource content."]
    pub content: String,
    #[serde(default)]
    #[doc = "True, if content was served as base64."]
    pub base_64_encoded: bool,
}
#[allow(deprecated)]
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[serde(rename_all = "camelCase")]
#[doc = "Returns present frame / resource tree structure."]
pub struct GetResourceTreeReturnObject {
    #[doc = "Present frame / resource tree structure."]
    pub frame_tree: FrameResourceTree,
}
#[allow(deprecated)]
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[doc = "Accepts or dismisses a JavaScript initiated dialog (alert, confirm, prompt, or onbeforeunload)."]
pub struct HandleJavaScriptDialogReturnObject(pub Option<Json>);
#[allow(deprecated)]
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[serde(rename_all = "camelCase")]
#[doc = "Navigates current page to the given URL."]
pub struct NavigateReturnObject {
    #[doc = "Frame id that has navigated (or failed to navigate)"]
    pub frame_id: FrameId,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[doc = "Loader identifier. This is omitted in case of same-document navigation,\n as the previously committed loaderId would not change."]
    pub loader_id: Option<network::LoaderId>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[doc = "User friendly error message, present if and only if navigation has failed."]
    pub error_text: Option<String>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[doc = "Whether the navigation resulted in a download."]
    pub is_download: Option<bool>,
}
#[allow(deprecated)]
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[doc = "Navigates current page to the given history entry."]
pub struct NavigateToHistoryEntryReturnObject(pub Option<Json>);
#[allow(deprecated)]
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[serde(rename_all = "camelCase")]
#[doc = "Print page as PDF."]
pub struct PrintToPDFReturnObject {
    #[doc = "Base64-encoded pdf data. Empty if |returnAsStream| is specified."]
    pub data: String,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[doc = "A handle of the stream that holds resulting PDF data."]
    pub stream: Option<io::StreamHandle>,
}
#[allow(deprecated)]
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[doc = "Reloads given page optionally ignoring the cache."]
pub struct ReloadReturnObject(pub Option<Json>);
#[allow(deprecated)]
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[doc = "Deprecated, please use removeScriptToEvaluateOnNewDocument instead."]
#[deprecated]
pub struct RemoveScriptToEvaluateOnLoadReturnObject(pub Option<Json>);
#[allow(deprecated)]
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[doc = "Removes given script from the list."]
pub struct RemoveScriptToEvaluateOnNewDocumentReturnObject(pub Option<Json>);
#[allow(deprecated)]
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[doc = "Acknowledges that a screencast frame has been received by the frontend."]
pub struct ScreencastFrameAckReturnObject(pub Option<Json>);
#[allow(deprecated)]
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[serde(rename_all = "camelCase")]
#[doc = "Searches for given string in resource content."]
pub struct SearchInResourceReturnObject {
    #[doc = "List of search matches."]
    pub result: debugger::SearchMatch,
}
#[allow(deprecated)]
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[doc = "Enable Chrome's experimental ad filter on all sites."]
pub struct SetAdBlockingEnabledReturnObject(pub Option<Json>);
#[allow(deprecated)]
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[doc = "Enable page Content Security Policy by-passing."]
pub struct SetBypassCSPReturnObject(pub Option<Json>);
#[allow(deprecated)]
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[serde(rename_all = "camelCase")]
#[doc = "Get Permissions Policy state on given frame."]
pub struct GetPermissionsPolicyStateReturnObject {
    pub states: Vec<PermissionsPolicyFeatureState>,
}
#[allow(deprecated)]
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[serde(rename_all = "camelCase")]
#[doc = "Get Origin Trials on given frame."]
pub struct GetOriginTrialsReturnObject {
    pub origin_trials: Vec<OriginTrial>,
}
#[allow(deprecated)]
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[doc = "Overrides the values of device screen dimensions (window.screen.width, window.screen.height,\n window.innerWidth, window.innerHeight, and \"device-width\"/\"device-height\"-related CSS media\n query results)."]
#[deprecated]
pub struct SetDeviceMetricsOverrideReturnObject(pub Option<Json>);
#[allow(deprecated)]
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[doc = "Overrides the Device Orientation."]
#[deprecated]
pub struct SetDeviceOrientationOverrideReturnObject(pub Option<Json>);
#[allow(deprecated)]
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[doc = "Set generic font families."]
pub struct SetFontFamiliesReturnObject(pub Option<Json>);
#[allow(deprecated)]
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[doc = "Set default font sizes."]
pub struct SetFontSizesReturnObject(pub Option<Json>);
#[allow(deprecated)]
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[doc = "Sets given markup as the document's HTML."]
pub struct SetDocumentContentReturnObject(pub Option<Json>);
#[allow(deprecated)]
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[doc = "Set the behavior when downloading a file."]
#[deprecated]
pub struct SetDownloadBehaviorReturnObject(pub Option<Json>);
#[allow(deprecated)]
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[doc = "Overrides the Geolocation Position or Error. Omitting any of the parameters emulates position\n unavailable."]
#[deprecated]
pub struct SetGeolocationOverrideReturnObject(pub Option<Json>);
#[allow(deprecated)]
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[doc = "Controls whether page will emit lifecycle events."]
pub struct SetLifecycleEventsEnabledReturnObject(pub Option<Json>);
#[allow(deprecated)]
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[doc = "Toggles mouse event-based touch event emulation."]
#[deprecated]
pub struct SetTouchEmulationEnabledReturnObject(pub Option<Json>);
#[allow(deprecated)]
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[doc = "Starts sending each frame using the `screencastFrame` event."]
pub struct StartScreencastReturnObject(pub Option<Json>);
#[allow(deprecated)]
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[doc = "Force the page stop all navigations and pending resource fetches."]
pub struct StopLoadingReturnObject(pub Option<Json>);
#[allow(deprecated)]
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[doc = "Crashes renderer on the IO thread, generates minidumps."]
pub struct CrashReturnObject(pub Option<Json>);
#[allow(deprecated)]
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[doc = "Tries to close page, running its beforeunload hooks, if any."]
pub struct CloseReturnObject(pub Option<Json>);
#[allow(deprecated)]
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[doc = "Tries to update the web lifecycle state of the page.\n It will transition the page to the given state according to:\n <https://github.com/WICG/web-lifecycle/>"]
pub struct SetWebLifecycleStateReturnObject(pub Option<Json>);
#[allow(deprecated)]
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[doc = "Stops sending each frame in the `screencastFrame`."]
pub struct StopScreencastReturnObject(pub Option<Json>);
#[allow(deprecated)]
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[doc = "Requests backend to produce compilation cache for the specified scripts.\n `scripts` are appended to the list of scripts for which the cache\n would be produced. The list may be reset during page navigation.\n When script with a matching URL is encountered, the cache is optionally\n produced upon backend discretion, based on internal heuristics.\n See also: `Page.compilationCacheProduced`."]
pub struct ProduceCompilationCacheReturnObject(pub Option<Json>);
#[allow(deprecated)]
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[doc = "Seeds compilation cache for given url. Compilation cache does not survive\n cross-process navigation."]
pub struct AddCompilationCacheReturnObject(pub Option<Json>);
#[allow(deprecated)]
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[doc = "Clears seeded compilation cache."]
pub struct ClearCompilationCacheReturnObject(pub Option<Json>);
#[allow(deprecated)]
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[doc = "Sets the Secure Payment Confirmation transaction mode.\n <https://w3c.github.io/secure-payment-confirmation/#sctn-automation-set-spc-transaction-mode>"]
pub struct SetSPCTransactionModeReturnObject(pub Option<Json>);
#[allow(deprecated)]
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[doc = "Extensions for Custom Handlers API:\n <https://html.spec.whatwg.org/multipage/system-state.html#rph-automation>"]
pub struct SetRPHRegistrationModeReturnObject(pub Option<Json>);
#[allow(deprecated)]
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[doc = "Generates a report for testing."]
pub struct GenerateTestReportReturnObject(pub Option<Json>);
#[allow(deprecated)]
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[doc = "Pauses page execution. Can be resumed using generic Runtime.runIfWaitingForDebugger."]
pub struct WaitForDebuggerReturnObject(pub Option<Json>);
#[allow(deprecated)]
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[doc = "Intercept file chooser requests and transfer control to protocol clients.\n When file chooser interception is enabled, native file chooser dialog is not shown.\n Instead, a protocol event `Page.fileChooserOpened` is emitted."]
pub struct SetInterceptFileChooserDialogReturnObject(pub Option<Json>);
#[allow(deprecated)]
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[doc = "Enable/disable prerendering manually.\n \n This command is a short-term solution for <https://crbug.com/1440085>.\n See <https://docs.google.com/document/d/12HVmFxYj5Jc-eJr5OmWsa2bqTJsbgGLKI6ZIyx0_wpA>\n for more details.\n \n TODO(<https://crbug.com/1440085>): Remove this once Puppeteer supports tab targets."]
pub struct SetPrerenderingAllowedReturnObject(pub Option<Json>);
#[allow(deprecated)]
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[serde(rename_all = "camelCase")]
#[doc = "Get the annotated page content for the main frame.\n This is an experimental command that is subject to change."]
pub struct GetAnnotatedPageContentReturnObject {
    #[doc = "The annotated page content as a base64 encoded protobuf.\n The format is defined by the `AnnotatedPageContent` message in\n components/optimization_guide/proto/features/common_quality_data.proto"]
    pub content: String,
}
#[allow(deprecated)]
impl Method for AddScriptToEvaluateOnLoad {
    const NAME: &'static str = "Page.addScriptToEvaluateOnLoad";
    type ReturnObject = AddScriptToEvaluateOnLoadReturnObject;
}
#[allow(deprecated)]
impl Method for AddScriptToEvaluateOnNewDocument {
    const NAME: &'static str = "Page.addScriptToEvaluateOnNewDocument";
    type ReturnObject = AddScriptToEvaluateOnNewDocumentReturnObject;
}
#[allow(deprecated)]
impl Method for BringToFront {
    const NAME: &'static str = "Page.bringToFront";
    type ReturnObject = BringToFrontReturnObject;
}
#[allow(deprecated)]
impl Method for CaptureScreenshot {
    const NAME: &'static str = "Page.captureScreenshot";
    type ReturnObject = CaptureScreenshotReturnObject;
}
#[allow(deprecated)]
impl Method for CaptureSnapshot {
    const NAME: &'static str = "Page.captureSnapshot";
    type ReturnObject = CaptureSnapshotReturnObject;
}
#[allow(deprecated)]
impl Method for ClearDeviceMetricsOverride {
    const NAME: &'static str = "Page.clearDeviceMetricsOverride";
    type ReturnObject = ClearDeviceMetricsOverrideReturnObject;
}
#[allow(deprecated)]
impl Method for ClearDeviceOrientationOverride {
    const NAME: &'static str = "Page.clearDeviceOrientationOverride";
    type ReturnObject = ClearDeviceOrientationOverrideReturnObject;
}
#[allow(deprecated)]
impl Method for ClearGeolocationOverride {
    const NAME: &'static str = "Page.clearGeolocationOverride";
    type ReturnObject = ClearGeolocationOverrideReturnObject;
}
#[allow(deprecated)]
impl Method for CreateIsolatedWorld {
    const NAME: &'static str = "Page.createIsolatedWorld";
    type ReturnObject = CreateIsolatedWorldReturnObject;
}
#[allow(deprecated)]
impl Method for DeleteCookie {
    const NAME: &'static str = "Page.deleteCookie";
    type ReturnObject = DeleteCookieReturnObject;
}
#[allow(deprecated)]
impl Method for Disable {
    const NAME: &'static str = "Page.disable";
    type ReturnObject = DisableReturnObject;
}
#[allow(deprecated)]
impl Method for Enable {
    const NAME: &'static str = "Page.enable";
    type ReturnObject = EnableReturnObject;
}
#[allow(deprecated)]
impl Method for GetAppManifest {
    const NAME: &'static str = "Page.getAppManifest";
    type ReturnObject = GetAppManifestReturnObject;
}
#[allow(deprecated)]
impl Method for GetInstallabilityErrors {
    const NAME: &'static str = "Page.getInstallabilityErrors";
    type ReturnObject = GetInstallabilityErrorsReturnObject;
}
#[allow(deprecated)]
impl Method for GetManifestIcons {
    const NAME: &'static str = "Page.getManifestIcons";
    type ReturnObject = GetManifestIconsReturnObject;
}
#[allow(deprecated)]
impl Method for GetAppId {
    const NAME: &'static str = "Page.getAppId";
    type ReturnObject = GetAppIdReturnObject;
}
#[allow(deprecated)]
impl Method for GetAdScriptAncestry {
    const NAME: &'static str = "Page.getAdScriptAncestry";
    type ReturnObject = GetAdScriptAncestryReturnObject;
}
#[allow(deprecated)]
impl Method for GetFrameTree {
    const NAME: &'static str = "Page.getFrameTree";
    type ReturnObject = GetFrameTreeReturnObject;
}
#[allow(deprecated)]
impl Method for GetLayoutMetrics {
    const NAME: &'static str = "Page.getLayoutMetrics";
    type ReturnObject = GetLayoutMetricsReturnObject;
}
#[allow(deprecated)]
impl Method for GetNavigationHistory {
    const NAME: &'static str = "Page.getNavigationHistory";
    type ReturnObject = GetNavigationHistoryReturnObject;
}
#[allow(deprecated)]
impl Method for ResetNavigationHistory {
    const NAME: &'static str = "Page.resetNavigationHistory";
    type ReturnObject = ResetNavigationHistoryReturnObject;
}
#[allow(deprecated)]
impl Method for GetResourceContent {
    const NAME: &'static str = "Page.getResourceContent";
    type ReturnObject = GetResourceContentReturnObject;
}
#[allow(deprecated)]
impl Method for GetResourceTree {
    const NAME: &'static str = "Page.getResourceTree";
    type ReturnObject = GetResourceTreeReturnObject;
}
#[allow(deprecated)]
impl Method for HandleJavaScriptDialog {
    const NAME: &'static str = "Page.handleJavaScriptDialog";
    type ReturnObject = HandleJavaScriptDialogReturnObject;
}
#[allow(deprecated)]
impl Method for Navigate {
    const NAME: &'static str = "Page.navigate";
    type ReturnObject = NavigateReturnObject;
}
#[allow(deprecated)]
impl Method for NavigateToHistoryEntry {
    const NAME: &'static str = "Page.navigateToHistoryEntry";
    type ReturnObject = NavigateToHistoryEntryReturnObject;
}
#[allow(deprecated)]
impl Method for PrintToPDF {
    const NAME: &'static str = "Page.printToPDF";
    type ReturnObject = PrintToPDFReturnObject;
}
#[allow(deprecated)]
impl Method for Reload {
    const NAME: &'static str = "Page.reload";
    type ReturnObject = ReloadReturnObject;
}
#[allow(deprecated)]
impl Method for RemoveScriptToEvaluateOnLoad {
    const NAME: &'static str = "Page.removeScriptToEvaluateOnLoad";
    type ReturnObject = RemoveScriptToEvaluateOnLoadReturnObject;
}
#[allow(deprecated)]
impl Method for RemoveScriptToEvaluateOnNewDocument {
    const NAME: &'static str = "Page.removeScriptToEvaluateOnNewDocument";
    type ReturnObject = RemoveScriptToEvaluateOnNewDocumentReturnObject;
}
#[allow(deprecated)]
impl Method for ScreencastFrameAck {
    const NAME: &'static str = "Page.screencastFrameAck";
    type ReturnObject = ScreencastFrameAckReturnObject;
}
#[allow(deprecated)]
impl Method for SearchInResource {
    const NAME: &'static str = "Page.searchInResource";
    type ReturnObject = SearchInResourceReturnObject;
}
#[allow(deprecated)]
impl Method for SetAdBlockingEnabled {
    const NAME: &'static str = "Page.setAdBlockingEnabled";
    type ReturnObject = SetAdBlockingEnabledReturnObject;
}
#[allow(deprecated)]
impl Method for SetBypassCSP {
    const NAME: &'static str = "Page.setBypassCSP";
    type ReturnObject = SetBypassCSPReturnObject;
}
#[allow(deprecated)]
impl Method for GetPermissionsPolicyState {
    const NAME: &'static str = "Page.getPermissionsPolicyState";
    type ReturnObject = GetPermissionsPolicyStateReturnObject;
}
#[allow(deprecated)]
impl Method for GetOriginTrials {
    const NAME: &'static str = "Page.getOriginTrials";
    type ReturnObject = GetOriginTrialsReturnObject;
}
#[allow(deprecated)]
impl Method for SetDeviceMetricsOverride {
    const NAME: &'static str = "Page.setDeviceMetricsOverride";
    type ReturnObject = SetDeviceMetricsOverrideReturnObject;
}
#[allow(deprecated)]
impl Method for SetDeviceOrientationOverride {
    const NAME: &'static str = "Page.setDeviceOrientationOverride";
    type ReturnObject = SetDeviceOrientationOverrideReturnObject;
}
#[allow(deprecated)]
impl Method for SetFontFamilies {
    const NAME: &'static str = "Page.setFontFamilies";
    type ReturnObject = SetFontFamiliesReturnObject;
}
#[allow(deprecated)]
impl Method for SetFontSizes {
    const NAME: &'static str = "Page.setFontSizes";
    type ReturnObject = SetFontSizesReturnObject;
}
#[allow(deprecated)]
impl Method for SetDocumentContent {
    const NAME: &'static str = "Page.setDocumentContent";
    type ReturnObject = SetDocumentContentReturnObject;
}
#[allow(deprecated)]
impl Method for SetDownloadBehavior {
    const NAME: &'static str = "Page.setDownloadBehavior";
    type ReturnObject = SetDownloadBehaviorReturnObject;
}
#[allow(deprecated)]
impl Method for SetGeolocationOverride {
    const NAME: &'static str = "Page.setGeolocationOverride";
    type ReturnObject = SetGeolocationOverrideReturnObject;
}
#[allow(deprecated)]
impl Method for SetLifecycleEventsEnabled {
    const NAME: &'static str = "Page.setLifecycleEventsEnabled";
    type ReturnObject = SetLifecycleEventsEnabledReturnObject;
}
#[allow(deprecated)]
impl Method for SetTouchEmulationEnabled {
    const NAME: &'static str = "Page.setTouchEmulationEnabled";
    type ReturnObject = SetTouchEmulationEnabledReturnObject;
}
#[allow(deprecated)]
impl Method for StartScreencast {
    const NAME: &'static str = "Page.startScreencast";
    type ReturnObject = StartScreencastReturnObject;
}
#[allow(deprecated)]
impl Method for StopLoading {
    const NAME: &'static str = "Page.stopLoading";
    type ReturnObject = StopLoadingReturnObject;
}
#[allow(deprecated)]
impl Method for Crash {
    const NAME: &'static str = "Page.crash";
    type ReturnObject = CrashReturnObject;
}
#[allow(deprecated)]
impl Method for Close {
    const NAME: &'static str = "Page.close";
    type ReturnObject = CloseReturnObject;
}
#[allow(deprecated)]
impl Method for SetWebLifecycleState {
    const NAME: &'static str = "Page.setWebLifecycleState";
    type ReturnObject = SetWebLifecycleStateReturnObject;
}
#[allow(deprecated)]
impl Method for StopScreencast {
    const NAME: &'static str = "Page.stopScreencast";
    type ReturnObject = StopScreencastReturnObject;
}
#[allow(deprecated)]
impl Method for ProduceCompilationCache {
    const NAME: &'static str = "Page.produceCompilationCache";
    type ReturnObject = ProduceCompilationCacheReturnObject;
}
#[allow(deprecated)]
impl Method for AddCompilationCache {
    const NAME: &'static str = "Page.addCompilationCache";
    type ReturnObject = AddCompilationCacheReturnObject;
}
#[allow(deprecated)]
impl Method for ClearCompilationCache {
    const NAME: &'static str = "Page.clearCompilationCache";
    type ReturnObject = ClearCompilationCacheReturnObject;
}
#[allow(deprecated)]
impl Method for SetSPCTransactionMode {
    const NAME: &'static str = "Page.setSPCTransactionMode";
    type ReturnObject = SetSPCTransactionModeReturnObject;
}
#[allow(deprecated)]
impl Method for SetRPHRegistrationMode {
    const NAME: &'static str = "Page.setRPHRegistrationMode";
    type ReturnObject = SetRPHRegistrationModeReturnObject;
}
#[allow(deprecated)]
impl Method for GenerateTestReport {
    const NAME: &'static str = "Page.generateTestReport";
    type ReturnObject = GenerateTestReportReturnObject;
}
#[allow(deprecated)]
impl Method for WaitForDebugger {
    const NAME: &'static str = "Page.waitForDebugger";
    type ReturnObject = WaitForDebuggerReturnObject;
}
#[allow(deprecated)]
impl Method for SetInterceptFileChooserDialog {
    const NAME: &'static str = "Page.setInterceptFileChooserDialog";
    type ReturnObject = SetInterceptFileChooserDialogReturnObject;
}
#[allow(deprecated)]
impl Method for SetPrerenderingAllowed {
    const NAME: &'static str = "Page.setPrerenderingAllowed";
    type ReturnObject = SetPrerenderingAllowedReturnObject;
}
#[allow(deprecated)]
impl Method for GetAnnotatedPageContent {
    const NAME: &'static str = "Page.getAnnotatedPageContent";
    type ReturnObject = GetAnnotatedPageContentReturnObject;
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
    pub struct DomContentEventFiredEvent {
        pub params: DomContentEventFiredEventParams,
    }
    #[allow(deprecated)]
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
    #[serde(rename_all = "camelCase")]
    pub struct DomContentEventFiredEventParams {
        pub timestamp: super::super::network::MonotonicTime,
    }
    #[allow(deprecated)]
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
    pub struct FileChooserOpenedEvent {
        pub params: FileChooserOpenedEventParams,
    }
    #[allow(deprecated)]
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
    #[serde(rename_all = "camelCase")]
    pub struct FileChooserOpenedEventParams {
        #[doc = "Id of the frame containing input node."]
        pub frame_id: super::FrameId,
        #[doc = "Input mode."]
        pub mode: super::FileChooserOpenedModeOption,
        #[builder(default)]
        #[serde(skip_serializing_if = "Option::is_none")]
        #[doc = "Input node id. Only present for file choosers opened via an `\\<input type=\"file\"\\>` element."]
        pub backend_node_id: Option<super::super::dom::BackendNodeId>,
    }
    #[allow(deprecated)]
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
    pub struct FrameAttachedEvent {
        pub params: FrameAttachedEventParams,
    }
    #[allow(deprecated)]
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
    #[serde(rename_all = "camelCase")]
    pub struct FrameAttachedEventParams {
        #[doc = "Id of the frame that has been attached."]
        pub frame_id: super::FrameId,
        #[doc = "Parent frame identifier."]
        pub parent_frame_id: super::FrameId,
        #[builder(default)]
        #[serde(skip_serializing_if = "Option::is_none")]
        #[doc = "JavaScript stack trace of when frame was attached, only set if frame initiated from script."]
        pub stack: Option<super::super::runtime::StackTrace>,
    }
    #[allow(deprecated)]
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
    pub struct FrameClearedScheduledNavigationEvent {
        pub params: FrameClearedScheduledNavigationEventParams,
    }
    #[allow(deprecated)]
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
    #[serde(rename_all = "camelCase")]
    pub struct FrameClearedScheduledNavigationEventParams {
        #[doc = "Id of the frame that has cleared its scheduled navigation."]
        pub frame_id: super::FrameId,
    }
    #[allow(deprecated)]
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
    pub struct FrameDetachedEvent {
        pub params: FrameDetachedEventParams,
    }
    #[allow(deprecated)]
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
    #[serde(rename_all = "camelCase")]
    pub struct FrameDetachedEventParams {
        #[doc = "Id of the frame that has been detached."]
        pub frame_id: super::FrameId,
        pub reason: super::FrameDetachedReasonOption,
    }
    #[allow(deprecated)]
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
    pub struct FrameSubtreeWillBeDetachedEvent {
        pub params: FrameSubtreeWillBeDetachedEventParams,
    }
    #[allow(deprecated)]
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
    #[serde(rename_all = "camelCase")]
    pub struct FrameSubtreeWillBeDetachedEventParams {
        #[doc = "Id of the frame that is the root of the subtree that will be detached."]
        pub frame_id: super::FrameId,
    }
    #[allow(deprecated)]
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
    pub struct FrameNavigatedEvent {
        pub params: FrameNavigatedEventParams,
    }
    #[allow(deprecated)]
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
    #[serde(rename_all = "camelCase")]
    pub struct FrameNavigatedEventParams {
        #[doc = "Frame object."]
        pub frame: super::Frame,
        pub r#type: super::NavigationType,
    }
    #[allow(deprecated)]
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
    pub struct DocumentOpenedEvent {
        pub params: DocumentOpenedEventParams,
    }
    #[allow(deprecated)]
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
    #[serde(rename_all = "camelCase")]
    pub struct DocumentOpenedEventParams {
        #[doc = "Frame object."]
        pub frame: super::Frame,
    }
    #[allow(deprecated)]
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
    pub struct FrameResizedEvent(pub Option<Json>);
    #[allow(deprecated)]
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
    pub struct FrameStartedNavigatingEvent {
        pub params: FrameStartedNavigatingEventParams,
    }
    #[allow(deprecated)]
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
    #[serde(rename_all = "camelCase")]
    pub struct FrameStartedNavigatingEventParams {
        #[doc = "ID of the frame that is being navigated."]
        pub frame_id: super::FrameId,
        #[serde(default)]
        #[doc = "The URL the navigation started with. The final URL can be different."]
        pub url: String,
        #[doc = "Loader identifier. Even though it is present in case of same-document\n navigation, the previously committed loaderId would not change unless\n the navigation changes from a same-document to a cross-document\n navigation."]
        pub loader_id: super::super::network::LoaderId,
        pub navigation_type: super::FrameStartedNavigatingNavigationTypeOption,
    }
    #[allow(deprecated)]
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
    pub struct FrameRequestedNavigationEvent {
        pub params: FrameRequestedNavigationEventParams,
    }
    #[allow(deprecated)]
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
    #[serde(rename_all = "camelCase")]
    pub struct FrameRequestedNavigationEventParams {
        #[doc = "Id of the frame that is being navigated."]
        pub frame_id: super::FrameId,
        #[doc = "The reason for the navigation."]
        pub reason: super::ClientNavigationReason,
        #[serde(default)]
        #[doc = "The destination URL for the requested navigation."]
        pub url: String,
        #[doc = "The disposition for the navigation."]
        pub disposition: super::ClientNavigationDisposition,
    }
    #[allow(deprecated)]
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
    pub struct FrameScheduledNavigationEvent {
        pub params: FrameScheduledNavigationEventParams,
    }
    #[allow(deprecated)]
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
    #[serde(rename_all = "camelCase")]
    pub struct FrameScheduledNavigationEventParams {
        #[doc = "Id of the frame that has scheduled a navigation."]
        pub frame_id: super::FrameId,
        #[serde(default)]
        #[doc = "Delay (in seconds) until the navigation is scheduled to begin. The navigation is not\n guaranteed to start."]
        pub delay: JsFloat,
        #[doc = "The reason for the navigation."]
        pub reason: super::ClientNavigationReason,
        #[serde(default)]
        #[doc = "The destination URL for the scheduled navigation."]
        pub url: String,
    }
    #[allow(deprecated)]
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
    pub struct FrameStartedLoadingEvent {
        pub params: FrameStartedLoadingEventParams,
    }
    #[allow(deprecated)]
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
    #[serde(rename_all = "camelCase")]
    pub struct FrameStartedLoadingEventParams {
        #[doc = "Id of the frame that has started loading."]
        pub frame_id: super::FrameId,
    }
    #[allow(deprecated)]
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
    pub struct FrameStoppedLoadingEvent {
        pub params: FrameStoppedLoadingEventParams,
    }
    #[allow(deprecated)]
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
    #[serde(rename_all = "camelCase")]
    pub struct FrameStoppedLoadingEventParams {
        #[doc = "Id of the frame that has stopped loading."]
        pub frame_id: super::FrameId,
    }
    #[allow(deprecated)]
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
    pub struct DownloadWillBeginEvent {
        pub params: DownloadWillBeginEventParams,
    }
    #[allow(deprecated)]
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
    #[serde(rename_all = "camelCase")]
    pub struct DownloadWillBeginEventParams {
        #[doc = "Id of the frame that caused download to begin."]
        pub frame_id: super::FrameId,
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
    #[allow(deprecated)]
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
    pub struct DownloadProgressEvent {
        pub params: DownloadProgressEventParams,
    }
    #[allow(deprecated)]
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
    #[serde(rename_all = "camelCase")]
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
    }
    #[allow(deprecated)]
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
    pub struct InterstitialHiddenEvent(pub Option<Json>);
    #[allow(deprecated)]
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
    pub struct InterstitialShownEvent(pub Option<Json>);
    #[allow(deprecated)]
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
    pub struct JavascriptDialogClosedEvent {
        pub params: JavascriptDialogClosedEventParams,
    }
    #[allow(deprecated)]
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
    #[serde(rename_all = "camelCase")]
    pub struct JavascriptDialogClosedEventParams {
        #[doc = "Frame id."]
        pub frame_id: super::FrameId,
        #[serde(default)]
        #[doc = "Whether dialog was confirmed."]
        pub result: bool,
        #[serde(default)]
        #[doc = "User input in case of prompt."]
        pub user_input: String,
    }
    #[allow(deprecated)]
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
    pub struct JavascriptDialogOpeningEvent {
        pub params: JavascriptDialogOpeningEventParams,
    }
    #[allow(deprecated)]
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
    #[serde(rename_all = "camelCase")]
    pub struct JavascriptDialogOpeningEventParams {
        #[serde(default)]
        #[doc = "Frame url."]
        pub url: String,
        #[doc = "Frame id."]
        pub frame_id: super::FrameId,
        #[serde(default)]
        #[doc = "Message that will be displayed by the dialog."]
        pub message: String,
        #[doc = "Dialog type."]
        pub r#type: super::DialogType,
        #[serde(default)]
        #[doc = "True iff browser is capable showing or acting on the given dialog. When browser has no\n dialog handler for given target, calling alert while Page domain is engaged will stall\n the page execution. Execution can be resumed via calling Page.handleJavaScriptDialog."]
        pub has_browser_handler: bool,
        #[builder(default)]
        #[serde(skip_serializing_if = "Option::is_none")]
        #[serde(default)]
        #[doc = "Default dialog prompt."]
        pub default_prompt: Option<String>,
    }
    #[allow(deprecated)]
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
    pub struct LifecycleEventEvent {
        pub params: LifecycleEventEventParams,
    }
    #[allow(deprecated)]
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
    #[serde(rename_all = "camelCase")]
    pub struct LifecycleEventEventParams {
        #[doc = "Id of the frame."]
        pub frame_id: super::FrameId,
        #[doc = "Loader identifier. Empty string if the request is fetched from worker."]
        pub loader_id: super::super::network::LoaderId,
        #[serde(default)]
        pub name: String,
        pub timestamp: super::super::network::MonotonicTime,
    }
    #[allow(deprecated)]
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
    pub struct BackForwardCacheNotUsedEvent {
        pub params: BackForwardCacheNotUsedEventParams,
    }
    #[allow(deprecated)]
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
    #[serde(rename_all = "camelCase")]
    pub struct BackForwardCacheNotUsedEventParams {
        #[doc = "The loader id for the associated navigation."]
        pub loader_id: super::super::network::LoaderId,
        #[doc = "The frame id of the associated frame."]
        pub frame_id: super::FrameId,
        #[doc = "Array of reasons why the page could not be cached. This must not be empty."]
        pub not_restored_explanations: Vec<super::BackForwardCacheNotRestoredExplanation>,
        #[builder(default)]
        #[serde(skip_serializing_if = "Option::is_none")]
        #[doc = "Tree structure of reasons why the page could not be cached for each frame."]
        pub not_restored_explanations_tree:
            Option<super::BackForwardCacheNotRestoredExplanationTree>,
    }
    #[allow(deprecated)]
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
    pub struct LoadEventFiredEvent {
        pub params: LoadEventFiredEventParams,
    }
    #[allow(deprecated)]
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
    #[serde(rename_all = "camelCase")]
    pub struct LoadEventFiredEventParams {
        pub timestamp: super::super::network::MonotonicTime,
    }
    #[allow(deprecated)]
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
    pub struct NavigatedWithinDocumentEvent {
        pub params: NavigatedWithinDocumentEventParams,
    }
    #[allow(deprecated)]
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
    #[serde(rename_all = "camelCase")]
    pub struct NavigatedWithinDocumentEventParams {
        #[doc = "Id of the frame."]
        pub frame_id: super::FrameId,
        #[serde(default)]
        #[doc = "Frame's new url."]
        pub url: String,
        #[doc = "Navigation type"]
        pub navigation_type: super::NavigatedWithinDocumentNavigationTypeOption,
    }
    #[allow(deprecated)]
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
    pub struct ScreencastFrameEvent {
        pub params: ScreencastFrameEventParams,
    }
    #[allow(deprecated)]
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
    #[serde(rename_all = "camelCase")]
    pub struct ScreencastFrameEventParams {
        #[serde(default)]
        #[doc = "Base64-encoded compressed image."]
        pub data: String,
        #[doc = "Screencast frame metadata."]
        pub metadata: super::ScreencastFrameMetadata,
        #[serde(default)]
        #[doc = "Frame number."]
        pub session_id: JsUInt,
    }
    #[allow(deprecated)]
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
    pub struct ScreencastVisibilityChangedEvent {
        pub params: ScreencastVisibilityChangedEventParams,
    }
    #[allow(deprecated)]
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
    #[serde(rename_all = "camelCase")]
    pub struct ScreencastVisibilityChangedEventParams {
        #[serde(default)]
        #[doc = "True if the page is visible."]
        pub visible: bool,
    }
    #[allow(deprecated)]
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
    pub struct WindowOpenEvent {
        pub params: WindowOpenEventParams,
    }
    #[allow(deprecated)]
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
    #[serde(rename_all = "camelCase")]
    pub struct WindowOpenEventParams {
        #[serde(default)]
        #[doc = "The URL for the new window."]
        pub url: String,
        #[serde(default)]
        #[doc = "Window name."]
        pub window_name: String,
        #[serde(default)]
        #[doc = "An array of enabled window features."]
        pub window_features: Vec<String>,
        #[serde(default)]
        #[doc = "Whether or not it was triggered by user gesture."]
        pub user_gesture: bool,
    }
    #[allow(deprecated)]
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
    pub struct CompilationCacheProducedEvent {
        pub params: CompilationCacheProducedEventParams,
    }
    #[allow(deprecated)]
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
    #[serde(rename_all = "camelCase")]
    pub struct CompilationCacheProducedEventParams {
        #[serde(default)]
        pub url: String,
        #[serde(default)]
        #[doc = "Base64-encoded data"]
        pub data: String,
    }
}
