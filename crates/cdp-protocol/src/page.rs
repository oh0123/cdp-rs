// Auto-generated from Chrome at version 143.0.7499.110 domain: Page
use super::debugger;
use super::dom;
use super::emulation;
use super::io;
use super::network;
use super::runtime;
#[allow(unused_imports)]
use super::types::*;
#[allow(unused_imports)]
use serde::{Deserialize, Serialize};
#[allow(unused_imports)]
use serde_json::Value as Json;
pub type FrameId = String;
pub type ScriptIdentifier = String;
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub enum AdFrameType {
    #[serde(rename = "none")]
    None,
    #[serde(rename = "child")]
    Child,
    #[serde(rename = "root")]
    Root,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub enum AdFrameExplanation {
    #[serde(rename = "ParentIsAd")]
    ParentIsAd,
    #[serde(rename = "CreatedByAdScript")]
    CreatedByAdScript,
    #[serde(rename = "MatchedBlockingRule")]
    MatchedBlockingRule,
}
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
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub enum CrossOriginIsolatedContextType {
    #[serde(rename = "Isolated")]
    Isolated,
    #[serde(rename = "NotIsolated")]
    NotIsolated,
    #[serde(rename = "NotIsolatedFeatureDisabled")]
    NotIsolatedFeatureDisabled,
}
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
    #[serde(rename = "local-network-access")]
    LocalNetworkAccess,
    #[serde(rename = "magnetometer")]
    Magnetometer,
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
    #[serde(rename = "popins")]
    Popins,
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
    #[serde(rename = "shared-autofill")]
    SharedAutofill,
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
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub enum OriginTrialUsageRestriction {
    #[serde(rename = "None")]
    None,
    #[serde(rename = "Subset")]
    Subset,
}
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
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub enum NavigationType {
    #[serde(rename = "Navigation")]
    Navigation,
    #[serde(rename = "BackForwardCacheRestore")]
    BackForwardCacheRestore,
}
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
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub enum BackForwardCacheNotRestoredReasonType {
    #[serde(rename = "SupportPending")]
    SupportPending,
    #[serde(rename = "PageSupportNeeded")]
    PageSupportNeeded,
    #[serde(rename = "Circumstantial")]
    Circumstantial,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub enum CaptureScreenshotFormatOption {
    #[serde(rename = "jpeg")]
    Jpeg,
    #[serde(rename = "png")]
    Png,
    #[serde(rename = "webp")]
    Webp,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub enum CaptureSnapshotFormatOption {
    #[serde(rename = "mhtml")]
    Mhtml,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub enum PrintToPdfTransferModeOption {
    #[serde(rename = "ReturnAsBase64")]
    ReturnAsBase64,
    #[serde(rename = "ReturnAsStream")]
    ReturnAsStream,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub enum SetDownloadBehaviorBehaviorOption {
    #[serde(rename = "deny")]
    Deny,
    #[serde(rename = "allow")]
    Allow,
    #[serde(rename = "default")]
    Default,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub enum SetTouchEmulationEnabledConfigurationOption {
    #[serde(rename = "mobile")]
    Mobile,
    #[serde(rename = "desktop")]
    Desktop,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub enum StartScreencastFormatOption {
    #[serde(rename = "jpeg")]
    Jpeg,
    #[serde(rename = "png")]
    Png,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub enum SetWebLifecycleStateStateOption {
    #[serde(rename = "frozen")]
    Frozen,
    #[serde(rename = "active")]
    Active,
}
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
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub enum SetRphRegistrationModeModeOption {
    #[serde(rename = "none")]
    None,
    #[serde(rename = "autoAccept")]
    AutoAccept,
    #[serde(rename = "autoReject")]
    AutoReject,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub enum FileChooserOpenedModeOption {
    #[serde(rename = "selectSingle")]
    SelectSingle,
    #[serde(rename = "selectMultiple")]
    SelectMultiple,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub enum FrameDetachedReasonOption {
    #[serde(rename = "remove")]
    Remove,
    #[serde(rename = "swap")]
    Swap,
}
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
pub enum NavigatedWithinDocumentNavigationTypeOption {
    #[serde(rename = "fragment")]
    Fragment,
    #[serde(rename = "historyApi")]
    HistoryApi,
    #[serde(rename = "other")]
    Other,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct AdFrameStatus {
    #[serde(rename = "adFrameType")]
    pub ad_frame_type: AdFrameType,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "explanations")]
    pub explanations: Option<Vec<AdFrameExplanation>>,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct AdScriptId {
    #[serde(rename = "scriptId")]
    pub script_id: runtime::ScriptId,
    #[serde(rename = "debuggerId")]
    pub debugger_id: runtime::UniqueDebuggerId,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct AdScriptAncestry {
    #[serde(rename = "ancestryChain")]
    pub ancestry_chain: Vec<AdScriptId>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(rename = "rootScriptFilterlistRule")]
    pub root_script_filterlist_rule: Option<String>,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct PermissionsPolicyBlockLocator {
    #[serde(rename = "frameId")]
    pub frame_id: FrameId,
    #[serde(rename = "blockReason")]
    pub block_reason: PermissionsPolicyBlockReason,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct PermissionsPolicyFeatureState {
    #[serde(rename = "feature")]
    pub feature: PermissionsPolicyFeature,
    #[serde(default)]
    #[serde(rename = "allowed")]
    pub allowed: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "locator")]
    pub locator: Option<PermissionsPolicyBlockLocator>,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct OriginTrialToken {
    #[serde(default)]
    #[serde(rename = "origin")]
    pub origin: String,
    #[serde(default)]
    #[serde(rename = "matchSubDomains")]
    pub match_sub_domains: bool,
    #[serde(default)]
    #[serde(rename = "trialName")]
    pub trial_name: String,
    #[serde(rename = "expiryTime")]
    pub expiry_time: network::TimeSinceEpoch,
    #[serde(default)]
    #[serde(rename = "isThirdParty")]
    pub is_third_party: bool,
    #[serde(rename = "usageRestriction")]
    pub usage_restriction: OriginTrialUsageRestriction,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct OriginTrialTokenWithStatus {
    #[serde(default)]
    #[serde(rename = "rawTokenText")]
    pub raw_token_text: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "parsedToken")]
    pub parsed_token: Option<OriginTrialToken>,
    #[serde(rename = "status")]
    pub status: OriginTrialTokenStatus,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct OriginTrial {
    #[serde(default)]
    #[serde(rename = "trialName")]
    pub trial_name: String,
    #[serde(rename = "status")]
    pub status: OriginTrialStatus,
    #[serde(rename = "tokensWithStatus")]
    pub tokens_with_status: Vec<OriginTrialTokenWithStatus>,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct SecurityOriginDetails {
    #[serde(default)]
    #[serde(rename = "isLocalhost")]
    pub is_localhost: bool,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct Frame {
    #[serde(rename = "id")]
    pub id: FrameId,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "parentId")]
    pub parent_id: Option<FrameId>,
    #[serde(rename = "loaderId")]
    pub loader_id: network::LoaderId,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(rename = "name")]
    pub name: Option<String>,
    #[serde(default)]
    #[serde(rename = "url")]
    pub url: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(rename = "urlFragment")]
    pub url_fragment: Option<String>,
    #[serde(default)]
    #[serde(rename = "domainAndRegistry")]
    pub domain_and_registry: String,
    #[serde(default)]
    #[serde(rename = "securityOrigin")]
    pub security_origin: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "securityOriginDetails")]
    pub security_origin_details: Option<SecurityOriginDetails>,
    #[serde(default)]
    #[serde(rename = "mimeType")]
    pub mime_type: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(rename = "unreachableUrl")]
    pub unreachable_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "adFrameStatus")]
    pub ad_frame_status: Option<AdFrameStatus>,
    #[serde(rename = "secureContextType")]
    pub secure_context_type: SecureContextType,
    #[serde(rename = "crossOriginIsolatedContextType")]
    pub cross_origin_isolated_context_type: CrossOriginIsolatedContextType,
    #[serde(rename = "gatedAPIFeatures")]
    pub gated_api_features: Vec<GatedApiFeatures>,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct FrameResource {
    #[serde(default)]
    #[serde(rename = "url")]
    pub url: String,
    #[serde(rename = "type")]
    pub r#type: network::ResourceType,
    #[serde(default)]
    #[serde(rename = "mimeType")]
    pub mime_type: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "lastModified")]
    pub last_modified: Option<network::TimeSinceEpoch>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(rename = "contentSize")]
    pub content_size: Option<JsFloat>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(rename = "failed")]
    pub failed: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(rename = "canceled")]
    pub canceled: Option<bool>,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct FrameResourceTree {
    #[serde(rename = "frame")]
    pub frame: Frame,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "childFrames")]
    pub child_frames: Option<Vec<FrameResourceTree>>,
    #[serde(rename = "resources")]
    pub resources: Vec<FrameResource>,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct FrameTree {
    #[serde(rename = "frame")]
    pub frame: Frame,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "childFrames")]
    pub child_frames: Option<Vec<FrameTree>>,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct NavigationEntry {
    #[serde(default)]
    #[serde(rename = "id")]
    pub id: JsUInt,
    #[serde(default)]
    #[serde(rename = "url")]
    pub url: String,
    #[serde(default)]
    #[serde(rename = "userTypedURL")]
    pub user_typed_url: String,
    #[serde(default)]
    #[serde(rename = "title")]
    pub title: String,
    #[serde(rename = "transitionType")]
    pub transition_type: TransitionType,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct ScreencastFrameMetadata {
    #[serde(default)]
    #[serde(rename = "offsetTop")]
    pub offset_top: JsFloat,
    #[serde(default)]
    #[serde(rename = "pageScaleFactor")]
    pub page_scale_factor: JsFloat,
    #[serde(default)]
    #[serde(rename = "deviceWidth")]
    pub device_width: JsFloat,
    #[serde(default)]
    #[serde(rename = "deviceHeight")]
    pub device_height: JsFloat,
    #[serde(default)]
    #[serde(rename = "scrollOffsetX")]
    pub scroll_offset_x: JsFloat,
    #[serde(default)]
    #[serde(rename = "scrollOffsetY")]
    pub scroll_offset_y: JsFloat,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "timestamp")]
    pub timestamp: Option<network::TimeSinceEpoch>,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct AppManifestError {
    #[serde(default)]
    #[serde(rename = "message")]
    pub message: String,
    #[serde(default)]
    #[serde(rename = "critical")]
    pub critical: JsUInt,
    #[serde(default)]
    #[serde(rename = "line")]
    pub line: JsUInt,
    #[serde(default)]
    #[serde(rename = "column")]
    pub column: JsUInt,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct AppManifestParsedProperties {
    #[serde(default)]
    #[serde(rename = "scope")]
    pub scope: String,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct LayoutViewport {
    #[serde(default)]
    #[serde(rename = "pageX")]
    pub page_x: JsUInt,
    #[serde(default)]
    #[serde(rename = "pageY")]
    pub page_y: JsUInt,
    #[serde(default)]
    #[serde(rename = "clientWidth")]
    pub client_width: JsUInt,
    #[serde(default)]
    #[serde(rename = "clientHeight")]
    pub client_height: JsUInt,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct VisualViewport {
    #[serde(default)]
    #[serde(rename = "offsetX")]
    pub offset_x: JsFloat,
    #[serde(default)]
    #[serde(rename = "offsetY")]
    pub offset_y: JsFloat,
    #[serde(default)]
    #[serde(rename = "pageX")]
    pub page_x: JsFloat,
    #[serde(default)]
    #[serde(rename = "pageY")]
    pub page_y: JsFloat,
    #[serde(default)]
    #[serde(rename = "clientWidth")]
    pub client_width: JsFloat,
    #[serde(default)]
    #[serde(rename = "clientHeight")]
    pub client_height: JsFloat,
    #[serde(default)]
    #[serde(rename = "scale")]
    pub scale: JsFloat,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(rename = "zoom")]
    pub zoom: Option<JsFloat>,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct Viewport {
    #[serde(default)]
    #[serde(rename = "x")]
    pub x: JsFloat,
    #[serde(default)]
    #[serde(rename = "y")]
    pub y: JsFloat,
    #[serde(default)]
    #[serde(rename = "width")]
    pub width: JsFloat,
    #[serde(default)]
    #[serde(rename = "height")]
    pub height: JsFloat,
    #[serde(default)]
    #[serde(rename = "scale")]
    pub scale: JsFloat,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct FontFamilies {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(rename = "standard")]
    pub standard: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(rename = "fixed")]
    pub fixed: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(rename = "serif")]
    pub serif: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(rename = "sansSerif")]
    pub sans_serif: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(rename = "cursive")]
    pub cursive: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(rename = "fantasy")]
    pub fantasy: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(rename = "math")]
    pub math: Option<String>,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct ScriptFontFamilies {
    #[serde(default)]
    #[serde(rename = "script")]
    pub script: String,
    #[serde(rename = "fontFamilies")]
    pub font_families: FontFamilies,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct FontSizes {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(rename = "standard")]
    pub standard: Option<JsUInt>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(rename = "fixed")]
    pub fixed: Option<JsUInt>,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct InstallabilityErrorArgument {
    #[serde(default)]
    #[serde(rename = "name")]
    pub name: String,
    #[serde(default)]
    #[serde(rename = "value")]
    pub value: String,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct InstallabilityError {
    #[serde(default)]
    #[serde(rename = "errorId")]
    pub error_id: String,
    #[serde(rename = "errorArguments")]
    pub error_arguments: Vec<InstallabilityErrorArgument>,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct CompilationCacheParams {
    #[serde(default)]
    #[serde(rename = "url")]
    pub url: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(rename = "eager")]
    pub eager: Option<bool>,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct FileFilter {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(rename = "name")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(rename = "accepts")]
    pub accepts: Option<Vec<String>>,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct FileHandler {
    #[serde(default)]
    #[serde(rename = "action")]
    pub action: String,
    #[serde(default)]
    #[serde(rename = "name")]
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "icons")]
    pub icons: Option<Vec<ImageResource>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "accepts")]
    pub accepts: Option<Vec<FileFilter>>,
    #[serde(default)]
    #[serde(rename = "launchType")]
    pub launch_type: String,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct ImageResource {
    #[serde(default)]
    #[serde(rename = "url")]
    pub url: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(rename = "sizes")]
    pub sizes: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(rename = "type")]
    pub r#type: Option<String>,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct LaunchHandler {
    #[serde(default)]
    #[serde(rename = "clientMode")]
    pub client_mode: String,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct ProtocolHandler {
    #[serde(default)]
    #[serde(rename = "protocol")]
    pub protocol: String,
    #[serde(default)]
    #[serde(rename = "url")]
    pub url: String,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct RelatedApplication {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(rename = "id")]
    pub id: Option<String>,
    #[serde(default)]
    #[serde(rename = "url")]
    pub url: String,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct ScopeExtension {
    #[serde(default)]
    #[serde(rename = "origin")]
    pub origin: String,
    #[serde(default)]
    #[serde(rename = "hasOriginWildcard")]
    pub has_origin_wildcard: bool,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct Screenshot {
    #[serde(rename = "image")]
    pub image: ImageResource,
    #[serde(default)]
    #[serde(rename = "formFactor")]
    pub form_factor: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(rename = "label")]
    pub label: Option<String>,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct ShareTarget {
    #[serde(default)]
    #[serde(rename = "action")]
    pub action: String,
    #[serde(default)]
    #[serde(rename = "method")]
    pub method: String,
    #[serde(default)]
    #[serde(rename = "enctype")]
    pub enctype: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(rename = "title")]
    pub title: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(rename = "text")]
    pub text: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(rename = "url")]
    pub url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "files")]
    pub files: Option<Vec<FileFilter>>,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct Shortcut {
    #[serde(default)]
    #[serde(rename = "name")]
    pub name: String,
    #[serde(default)]
    #[serde(rename = "url")]
    pub url: String,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct WebAppManifest {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(rename = "backgroundColor")]
    pub background_color: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(rename = "description")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(rename = "dir")]
    pub dir: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(rename = "display")]
    pub display: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(rename = "displayOverrides")]
    pub display_overrides: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "fileHandlers")]
    pub file_handlers: Option<Vec<FileHandler>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "icons")]
    pub icons: Option<Vec<ImageResource>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(rename = "id")]
    pub id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(rename = "lang")]
    pub lang: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "launchHandler")]
    pub launch_handler: Option<LaunchHandler>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(rename = "name")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(rename = "orientation")]
    pub orientation: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(rename = "preferRelatedApplications")]
    pub prefer_related_applications: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "protocolHandlers")]
    pub protocol_handlers: Option<Vec<ProtocolHandler>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "relatedApplications")]
    pub related_applications: Option<Vec<RelatedApplication>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(rename = "scope")]
    pub scope: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "scopeExtensions")]
    pub scope_extensions: Option<Vec<ScopeExtension>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "screenshots")]
    pub screenshots: Option<Vec<Screenshot>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "shareTarget")]
    pub share_target: Option<ShareTarget>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(rename = "shortName")]
    pub short_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "shortcuts")]
    pub shortcuts: Option<Vec<Shortcut>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(rename = "startUrl")]
    pub start_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(rename = "themeColor")]
    pub theme_color: Option<String>,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct BackForwardCacheBlockingDetails {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(rename = "url")]
    pub url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(rename = "function")]
    pub function: Option<String>,
    #[serde(default)]
    #[serde(rename = "lineNumber")]
    pub line_number: JsUInt,
    #[serde(default)]
    #[serde(rename = "columnNumber")]
    pub column_number: JsUInt,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct BackForwardCacheNotRestoredExplanation {
    #[serde(rename = "type")]
    pub r#type: BackForwardCacheNotRestoredReasonType,
    #[serde(rename = "reason")]
    pub reason: BackForwardCacheNotRestoredReason,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(rename = "context")]
    pub context: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "details")]
    pub details: Option<Vec<BackForwardCacheBlockingDetails>>,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct BackForwardCacheNotRestoredExplanationTree {
    #[serde(default)]
    #[serde(rename = "url")]
    pub url: String,
    #[serde(rename = "explanations")]
    pub explanations: Vec<BackForwardCacheNotRestoredExplanation>,
    #[serde(rename = "children")]
    pub children: Vec<BackForwardCacheNotRestoredExplanationTree>,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct AddScriptToEvaluateOnLoad {
    #[serde(default)]
    #[serde(rename = "scriptSource")]
    pub script_source: String,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct AddScriptToEvaluateOnNewDocument {
    #[serde(default)]
    #[serde(rename = "source")]
    pub source: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(rename = "worldName")]
    pub world_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(rename = "includeCommandLineAPI")]
    pub include_command_line_api: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(rename = "runImmediately")]
    pub run_immediately: Option<bool>,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct BringToFront(pub Option<serde_json::Value>);
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct CaptureScreenshot {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "format")]
    pub format: Option<CaptureScreenshotFormatOption>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(rename = "quality")]
    pub quality: Option<JsUInt>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "clip")]
    pub clip: Option<Viewport>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(rename = "fromSurface")]
    pub from_surface: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(rename = "captureBeyondViewport")]
    pub capture_beyond_viewport: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(rename = "optimizeForSpeed")]
    pub optimize_for_speed: Option<bool>,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct CaptureSnapshot {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "format")]
    pub format: Option<CaptureSnapshotFormatOption>,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct ClearDeviceMetricsOverride(pub Option<serde_json::Value>);
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct ClearDeviceOrientationOverride(pub Option<serde_json::Value>);
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct ClearGeolocationOverride(pub Option<serde_json::Value>);
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct CreateIsolatedWorld {
    #[serde(rename = "frameId")]
    pub frame_id: FrameId,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(rename = "worldName")]
    pub world_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(rename = "grantUniveralAccess")]
    pub grant_univeral_access: Option<bool>,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct DeleteCookie {
    #[serde(default)]
    #[serde(rename = "cookieName")]
    pub cookie_name: String,
    #[serde(default)]
    #[serde(rename = "url")]
    pub url: String,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Disable(pub Option<serde_json::Value>);
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct Enable {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(rename = "enableFileChooserOpenedEvent")]
    pub enable_file_chooser_opened_event: Option<bool>,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct GetAppManifest {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(rename = "manifestId")]
    pub manifest_id: Option<String>,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct GetInstallabilityErrors(pub Option<serde_json::Value>);
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct GetManifestIcons(pub Option<serde_json::Value>);
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct GetAppId(pub Option<serde_json::Value>);
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct GetAdScriptAncestry {
    #[serde(rename = "frameId")]
    pub frame_id: FrameId,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct GetFrameTree(pub Option<serde_json::Value>);
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct GetLayoutMetrics(pub Option<serde_json::Value>);
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct GetNavigationHistory(pub Option<serde_json::Value>);
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct ResetNavigationHistory(pub Option<serde_json::Value>);
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct GetResourceContent {
    #[serde(rename = "frameId")]
    pub frame_id: FrameId,
    #[serde(default)]
    #[serde(rename = "url")]
    pub url: String,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct GetResourceTree(pub Option<serde_json::Value>);
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct HandleJavaScriptDialog {
    #[serde(default)]
    #[serde(rename = "accept")]
    pub accept: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(rename = "promptText")]
    pub prompt_text: Option<String>,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct Navigate {
    #[serde(default)]
    #[serde(rename = "url")]
    pub url: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(rename = "referrer")]
    pub referrer: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "transitionType")]
    pub transition_type: Option<TransitionType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "frameId")]
    pub frame_id: Option<FrameId>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "referrerPolicy")]
    pub referrer_policy: Option<ReferrerPolicy>,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct NavigateToHistoryEntry {
    #[serde(default)]
    #[serde(rename = "entryId")]
    pub entry_id: JsUInt,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct PrintToPDF {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(rename = "landscape")]
    pub landscape: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(rename = "displayHeaderFooter")]
    pub display_header_footer: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(rename = "printBackground")]
    pub print_background: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(rename = "scale")]
    pub scale: Option<JsFloat>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(rename = "paperWidth")]
    pub paper_width: Option<JsFloat>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(rename = "paperHeight")]
    pub paper_height: Option<JsFloat>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(rename = "marginTop")]
    pub margin_top: Option<JsFloat>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(rename = "marginBottom")]
    pub margin_bottom: Option<JsFloat>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(rename = "marginLeft")]
    pub margin_left: Option<JsFloat>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(rename = "marginRight")]
    pub margin_right: Option<JsFloat>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(rename = "pageRanges")]
    pub page_ranges: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(rename = "headerTemplate")]
    pub header_template: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(rename = "footerTemplate")]
    pub footer_template: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(rename = "preferCSSPageSize")]
    pub prefer_css_page_size: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "transferMode")]
    pub transfer_mode: Option<PrintToPdfTransferModeOption>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(rename = "generateTaggedPDF")]
    pub generate_tagged_pdf: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(rename = "generateDocumentOutline")]
    pub generate_document_outline: Option<bool>,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct Reload {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(rename = "ignoreCache")]
    pub ignore_cache: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(rename = "scriptToEvaluateOnLoad")]
    pub script_to_evaluate_on_load: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "loaderId")]
    pub loader_id: Option<network::LoaderId>,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct RemoveScriptToEvaluateOnLoad {
    #[serde(rename = "identifier")]
    pub identifier: ScriptIdentifier,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct RemoveScriptToEvaluateOnNewDocument {
    #[serde(rename = "identifier")]
    pub identifier: ScriptIdentifier,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct ScreencastFrameAck {
    #[serde(default)]
    #[serde(rename = "sessionId")]
    pub session_id: JsUInt,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct SearchInResource {
    #[serde(rename = "frameId")]
    pub frame_id: FrameId,
    #[serde(default)]
    #[serde(rename = "url")]
    pub url: String,
    #[serde(default)]
    #[serde(rename = "query")]
    pub query: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(rename = "caseSensitive")]
    pub case_sensitive: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(rename = "isRegex")]
    pub is_regex: Option<bool>,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct SetAdBlockingEnabled {
    #[serde(default)]
    #[serde(rename = "enabled")]
    pub enabled: bool,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct SetBypassCSP {
    #[serde(default)]
    #[serde(rename = "enabled")]
    pub enabled: bool,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct GetPermissionsPolicyState {
    #[serde(rename = "frameId")]
    pub frame_id: FrameId,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct GetOriginTrials {
    #[serde(rename = "frameId")]
    pub frame_id: FrameId,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct SetDeviceMetricsOverride {
    #[serde(default)]
    #[serde(rename = "width")]
    pub width: JsUInt,
    #[serde(default)]
    #[serde(rename = "height")]
    pub height: JsUInt,
    #[serde(default)]
    #[serde(rename = "deviceScaleFactor")]
    pub device_scale_factor: JsFloat,
    #[serde(default)]
    #[serde(rename = "mobile")]
    pub mobile: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(rename = "scale")]
    pub scale: Option<JsFloat>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(rename = "screenWidth")]
    pub screen_width: Option<JsUInt>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(rename = "screenHeight")]
    pub screen_height: Option<JsUInt>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(rename = "positionX")]
    pub position_x: Option<JsUInt>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(rename = "positionY")]
    pub position_y: Option<JsUInt>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(rename = "dontSetVisibleSize")]
    pub dont_set_visible_size: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "screenOrientation")]
    pub screen_orientation: Option<emulation::ScreenOrientation>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "viewport")]
    pub viewport: Option<Viewport>,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct SetDeviceOrientationOverride {
    #[serde(default)]
    #[serde(rename = "alpha")]
    pub alpha: JsFloat,
    #[serde(default)]
    #[serde(rename = "beta")]
    pub beta: JsFloat,
    #[serde(default)]
    #[serde(rename = "gamma")]
    pub gamma: JsFloat,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct SetFontFamilies {
    #[serde(rename = "fontFamilies")]
    pub font_families: FontFamilies,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "forScripts")]
    pub for_scripts: Option<Vec<ScriptFontFamilies>>,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct SetFontSizes {
    #[serde(rename = "fontSizes")]
    pub font_sizes: FontSizes,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct SetDocumentContent {
    #[serde(rename = "frameId")]
    pub frame_id: FrameId,
    #[serde(default)]
    #[serde(rename = "html")]
    pub html: String,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct SetDownloadBehavior {
    #[serde(rename = "behavior")]
    pub behavior: SetDownloadBehaviorBehaviorOption,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(rename = "downloadPath")]
    pub download_path: Option<String>,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct SetGeolocationOverride {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(rename = "latitude")]
    pub latitude: Option<JsFloat>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(rename = "longitude")]
    pub longitude: Option<JsFloat>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(rename = "accuracy")]
    pub accuracy: Option<JsFloat>,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct SetLifecycleEventsEnabled {
    #[serde(default)]
    #[serde(rename = "enabled")]
    pub enabled: bool,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct SetTouchEmulationEnabled {
    #[serde(default)]
    #[serde(rename = "enabled")]
    pub enabled: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "configuration")]
    pub configuration: Option<SetTouchEmulationEnabledConfigurationOption>,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct StartScreencast {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "format")]
    pub format: Option<StartScreencastFormatOption>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(rename = "quality")]
    pub quality: Option<JsUInt>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(rename = "maxWidth")]
    pub max_width: Option<JsUInt>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(rename = "maxHeight")]
    pub max_height: Option<JsUInt>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(rename = "everyNthFrame")]
    pub every_nth_frame: Option<JsUInt>,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct StopLoading(pub Option<serde_json::Value>);
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Crash(pub Option<serde_json::Value>);
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Close(pub Option<serde_json::Value>);
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct SetWebLifecycleState {
    #[serde(rename = "state")]
    pub state: SetWebLifecycleStateStateOption,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct StopScreencast(pub Option<serde_json::Value>);
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct ProduceCompilationCache {
    #[serde(rename = "scripts")]
    pub scripts: Vec<CompilationCacheParams>,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct AddCompilationCache {
    #[serde(default)]
    #[serde(rename = "url")]
    pub url: String,
    #[serde(rename = "data")]
    pub data: Vec<u8>,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct ClearCompilationCache(pub Option<serde_json::Value>);
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct SetSPCTransactionMode {
    #[serde(rename = "mode")]
    pub mode: SetSpcTransactionModeModeOption,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct SetRPHRegistrationMode {
    #[serde(rename = "mode")]
    pub mode: SetRphRegistrationModeModeOption,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct GenerateTestReport {
    #[serde(default)]
    #[serde(rename = "message")]
    pub message: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(rename = "group")]
    pub group: Option<String>,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct WaitForDebugger(pub Option<serde_json::Value>);
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct SetInterceptFileChooserDialog {
    #[serde(default)]
    #[serde(rename = "enabled")]
    pub enabled: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(rename = "cancel")]
    pub cancel: Option<bool>,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct SetPrerenderingAllowed {
    #[serde(default)]
    #[serde(rename = "isAllowed")]
    pub is_allowed: bool,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct AddScriptToEvaluateOnLoadReturnObject {
    #[serde(rename = "identifier")]
    pub identifier: ScriptIdentifier,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct AddScriptToEvaluateOnNewDocumentReturnObject {
    #[serde(rename = "identifier")]
    pub identifier: ScriptIdentifier,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct BringToFrontReturnObject {}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct CaptureScreenshotReturnObject {
    #[serde(rename = "data")]
    pub data: Vec<u8>,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct CaptureSnapshotReturnObject {
    #[serde(default)]
    #[serde(rename = "data")]
    pub data: String,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct ClearDeviceMetricsOverrideReturnObject {}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct ClearDeviceOrientationOverrideReturnObject {}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct ClearGeolocationOverrideReturnObject {}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct CreateIsolatedWorldReturnObject {
    #[serde(rename = "executionContextId")]
    pub execution_context_id: runtime::ExecutionContextId,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct DeleteCookieReturnObject {}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct DisableReturnObject {}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct EnableReturnObject {}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct GetAppManifestReturnObject {
    #[serde(default)]
    #[serde(rename = "url")]
    pub url: String,
    #[serde(rename = "errors")]
    pub errors: Vec<AppManifestError>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(rename = "data")]
    pub data: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "parsed")]
    pub parsed: Option<AppManifestParsedProperties>,
    #[serde(rename = "manifest")]
    pub manifest: WebAppManifest,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct GetInstallabilityErrorsReturnObject {
    #[serde(rename = "installabilityErrors")]
    pub installability_errors: Vec<InstallabilityError>,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct GetManifestIconsReturnObject {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "primaryIcon")]
    pub primary_icon: Option<Vec<u8>>,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct GetAppIdReturnObject {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(rename = "appId")]
    pub app_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(rename = "recommendedId")]
    pub recommended_id: Option<String>,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct GetAdScriptAncestryReturnObject {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "adScriptAncestry")]
    pub ad_script_ancestry: Option<AdScriptAncestry>,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct GetFrameTreeReturnObject {
    #[serde(rename = "frameTree")]
    pub frame_tree: FrameTree,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct GetLayoutMetricsReturnObject {
    #[serde(rename = "layoutViewport")]
    pub layout_viewport: LayoutViewport,
    #[serde(rename = "visualViewport")]
    pub visual_viewport: VisualViewport,
    #[serde(rename = "contentSize")]
    pub content_size: dom::Rect,
    #[serde(rename = "cssLayoutViewport")]
    pub css_layout_viewport: LayoutViewport,
    #[serde(rename = "cssVisualViewport")]
    pub css_visual_viewport: VisualViewport,
    #[serde(rename = "cssContentSize")]
    pub css_content_size: dom::Rect,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct GetNavigationHistoryReturnObject {
    #[serde(default)]
    #[serde(rename = "currentIndex")]
    pub current_index: JsUInt,
    #[serde(rename = "entries")]
    pub entries: Vec<NavigationEntry>,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct ResetNavigationHistoryReturnObject {}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct GetResourceContentReturnObject {
    #[serde(default)]
    #[serde(rename = "content")]
    pub content: String,
    #[serde(default)]
    #[serde(rename = "base64Encoded")]
    pub base_64_encoded: bool,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct GetResourceTreeReturnObject {
    #[serde(rename = "frameTree")]
    pub frame_tree: FrameResourceTree,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct HandleJavaScriptDialogReturnObject {}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct NavigateReturnObject {
    #[serde(rename = "frameId")]
    pub frame_id: FrameId,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "loaderId")]
    pub loader_id: Option<network::LoaderId>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(rename = "errorText")]
    pub error_text: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(rename = "isDownload")]
    pub is_download: Option<bool>,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct NavigateToHistoryEntryReturnObject {}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct PrintToPDFReturnObject {
    #[serde(rename = "data")]
    pub data: Vec<u8>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "stream")]
    pub stream: Option<io::StreamHandle>,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct ReloadReturnObject {}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct RemoveScriptToEvaluateOnLoadReturnObject {}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct RemoveScriptToEvaluateOnNewDocumentReturnObject {}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct ScreencastFrameAckReturnObject {}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct SearchInResourceReturnObject {
    #[serde(rename = "result")]
    pub result: debugger::SearchMatch,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct SetAdBlockingEnabledReturnObject {}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct SetBypassCSPReturnObject {}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct GetPermissionsPolicyStateReturnObject {
    #[serde(rename = "states")]
    pub states: Vec<PermissionsPolicyFeatureState>,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct GetOriginTrialsReturnObject {
    #[serde(rename = "originTrials")]
    pub origin_trials: Vec<OriginTrial>,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct SetDeviceMetricsOverrideReturnObject {}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct SetDeviceOrientationOverrideReturnObject {}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct SetFontFamiliesReturnObject {}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct SetFontSizesReturnObject {}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct SetDocumentContentReturnObject {}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct SetDownloadBehaviorReturnObject {}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct SetGeolocationOverrideReturnObject {}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct SetLifecycleEventsEnabledReturnObject {}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct SetTouchEmulationEnabledReturnObject {}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct StartScreencastReturnObject {}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct StopLoadingReturnObject {}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct CrashReturnObject {}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct CloseReturnObject {}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct SetWebLifecycleStateReturnObject {}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct StopScreencastReturnObject {}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct ProduceCompilationCacheReturnObject {}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct AddCompilationCacheReturnObject {}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct ClearCompilationCacheReturnObject {}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct SetSPCTransactionModeReturnObject {}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct SetRPHRegistrationModeReturnObject {}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct GenerateTestReportReturnObject {}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct WaitForDebuggerReturnObject {}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct SetInterceptFileChooserDialogReturnObject {}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct SetPrerenderingAllowedReturnObject {}
impl Method for AddScriptToEvaluateOnLoad {
    const NAME: &'static str = "Page.addScriptToEvaluateOnLoad";
    type ReturnObject = AddScriptToEvaluateOnLoadReturnObject;
}
impl Method for AddScriptToEvaluateOnNewDocument {
    const NAME: &'static str = "Page.addScriptToEvaluateOnNewDocument";
    type ReturnObject = AddScriptToEvaluateOnNewDocumentReturnObject;
}
impl Method for BringToFront {
    const NAME: &'static str = "Page.bringToFront";
    type ReturnObject = BringToFrontReturnObject;
}
impl Method for CaptureScreenshot {
    const NAME: &'static str = "Page.captureScreenshot";
    type ReturnObject = CaptureScreenshotReturnObject;
}
impl Method for CaptureSnapshot {
    const NAME: &'static str = "Page.captureSnapshot";
    type ReturnObject = CaptureSnapshotReturnObject;
}
impl Method for ClearDeviceMetricsOverride {
    const NAME: &'static str = "Page.clearDeviceMetricsOverride";
    type ReturnObject = ClearDeviceMetricsOverrideReturnObject;
}
impl Method for ClearDeviceOrientationOverride {
    const NAME: &'static str = "Page.clearDeviceOrientationOverride";
    type ReturnObject = ClearDeviceOrientationOverrideReturnObject;
}
impl Method for ClearGeolocationOverride {
    const NAME: &'static str = "Page.clearGeolocationOverride";
    type ReturnObject = ClearGeolocationOverrideReturnObject;
}
impl Method for CreateIsolatedWorld {
    const NAME: &'static str = "Page.createIsolatedWorld";
    type ReturnObject = CreateIsolatedWorldReturnObject;
}
impl Method for DeleteCookie {
    const NAME: &'static str = "Page.deleteCookie";
    type ReturnObject = DeleteCookieReturnObject;
}
impl Method for Disable {
    const NAME: &'static str = "Page.disable";
    type ReturnObject = DisableReturnObject;
}
impl Method for Enable {
    const NAME: &'static str = "Page.enable";
    type ReturnObject = EnableReturnObject;
}
impl Method for GetAppManifest {
    const NAME: &'static str = "Page.getAppManifest";
    type ReturnObject = GetAppManifestReturnObject;
}
impl Method for GetInstallabilityErrors {
    const NAME: &'static str = "Page.getInstallabilityErrors";
    type ReturnObject = GetInstallabilityErrorsReturnObject;
}
impl Method for GetManifestIcons {
    const NAME: &'static str = "Page.getManifestIcons";
    type ReturnObject = GetManifestIconsReturnObject;
}
impl Method for GetAppId {
    const NAME: &'static str = "Page.getAppId";
    type ReturnObject = GetAppIdReturnObject;
}
impl Method for GetAdScriptAncestry {
    const NAME: &'static str = "Page.getAdScriptAncestry";
    type ReturnObject = GetAdScriptAncestryReturnObject;
}
impl Method for GetFrameTree {
    const NAME: &'static str = "Page.getFrameTree";
    type ReturnObject = GetFrameTreeReturnObject;
}
impl Method for GetLayoutMetrics {
    const NAME: &'static str = "Page.getLayoutMetrics";
    type ReturnObject = GetLayoutMetricsReturnObject;
}
impl Method for GetNavigationHistory {
    const NAME: &'static str = "Page.getNavigationHistory";
    type ReturnObject = GetNavigationHistoryReturnObject;
}
impl Method for ResetNavigationHistory {
    const NAME: &'static str = "Page.resetNavigationHistory";
    type ReturnObject = ResetNavigationHistoryReturnObject;
}
impl Method for GetResourceContent {
    const NAME: &'static str = "Page.getResourceContent";
    type ReturnObject = GetResourceContentReturnObject;
}
impl Method for GetResourceTree {
    const NAME: &'static str = "Page.getResourceTree";
    type ReturnObject = GetResourceTreeReturnObject;
}
impl Method for HandleJavaScriptDialog {
    const NAME: &'static str = "Page.handleJavaScriptDialog";
    type ReturnObject = HandleJavaScriptDialogReturnObject;
}
impl Method for Navigate {
    const NAME: &'static str = "Page.navigate";
    type ReturnObject = NavigateReturnObject;
}
impl Method for NavigateToHistoryEntry {
    const NAME: &'static str = "Page.navigateToHistoryEntry";
    type ReturnObject = NavigateToHistoryEntryReturnObject;
}
impl Method for PrintToPDF {
    const NAME: &'static str = "Page.printToPDF";
    type ReturnObject = PrintToPDFReturnObject;
}
impl Method for Reload {
    const NAME: &'static str = "Page.reload";
    type ReturnObject = ReloadReturnObject;
}
impl Method for RemoveScriptToEvaluateOnLoad {
    const NAME: &'static str = "Page.removeScriptToEvaluateOnLoad";
    type ReturnObject = RemoveScriptToEvaluateOnLoadReturnObject;
}
impl Method for RemoveScriptToEvaluateOnNewDocument {
    const NAME: &'static str = "Page.removeScriptToEvaluateOnNewDocument";
    type ReturnObject = RemoveScriptToEvaluateOnNewDocumentReturnObject;
}
impl Method for ScreencastFrameAck {
    const NAME: &'static str = "Page.screencastFrameAck";
    type ReturnObject = ScreencastFrameAckReturnObject;
}
impl Method for SearchInResource {
    const NAME: &'static str = "Page.searchInResource";
    type ReturnObject = SearchInResourceReturnObject;
}
impl Method for SetAdBlockingEnabled {
    const NAME: &'static str = "Page.setAdBlockingEnabled";
    type ReturnObject = SetAdBlockingEnabledReturnObject;
}
impl Method for SetBypassCSP {
    const NAME: &'static str = "Page.setBypassCSP";
    type ReturnObject = SetBypassCSPReturnObject;
}
impl Method for GetPermissionsPolicyState {
    const NAME: &'static str = "Page.getPermissionsPolicyState";
    type ReturnObject = GetPermissionsPolicyStateReturnObject;
}
impl Method for GetOriginTrials {
    const NAME: &'static str = "Page.getOriginTrials";
    type ReturnObject = GetOriginTrialsReturnObject;
}
impl Method for SetDeviceMetricsOverride {
    const NAME: &'static str = "Page.setDeviceMetricsOverride";
    type ReturnObject = SetDeviceMetricsOverrideReturnObject;
}
impl Method for SetDeviceOrientationOverride {
    const NAME: &'static str = "Page.setDeviceOrientationOverride";
    type ReturnObject = SetDeviceOrientationOverrideReturnObject;
}
impl Method for SetFontFamilies {
    const NAME: &'static str = "Page.setFontFamilies";
    type ReturnObject = SetFontFamiliesReturnObject;
}
impl Method for SetFontSizes {
    const NAME: &'static str = "Page.setFontSizes";
    type ReturnObject = SetFontSizesReturnObject;
}
impl Method for SetDocumentContent {
    const NAME: &'static str = "Page.setDocumentContent";
    type ReturnObject = SetDocumentContentReturnObject;
}
impl Method for SetDownloadBehavior {
    const NAME: &'static str = "Page.setDownloadBehavior";
    type ReturnObject = SetDownloadBehaviorReturnObject;
}
impl Method for SetGeolocationOverride {
    const NAME: &'static str = "Page.setGeolocationOverride";
    type ReturnObject = SetGeolocationOverrideReturnObject;
}
impl Method for SetLifecycleEventsEnabled {
    const NAME: &'static str = "Page.setLifecycleEventsEnabled";
    type ReturnObject = SetLifecycleEventsEnabledReturnObject;
}
impl Method for SetTouchEmulationEnabled {
    const NAME: &'static str = "Page.setTouchEmulationEnabled";
    type ReturnObject = SetTouchEmulationEnabledReturnObject;
}
impl Method for StartScreencast {
    const NAME: &'static str = "Page.startScreencast";
    type ReturnObject = StartScreencastReturnObject;
}
impl Method for StopLoading {
    const NAME: &'static str = "Page.stopLoading";
    type ReturnObject = StopLoadingReturnObject;
}
impl Method for Crash {
    const NAME: &'static str = "Page.crash";
    type ReturnObject = CrashReturnObject;
}
impl Method for Close {
    const NAME: &'static str = "Page.close";
    type ReturnObject = CloseReturnObject;
}
impl Method for SetWebLifecycleState {
    const NAME: &'static str = "Page.setWebLifecycleState";
    type ReturnObject = SetWebLifecycleStateReturnObject;
}
impl Method for StopScreencast {
    const NAME: &'static str = "Page.stopScreencast";
    type ReturnObject = StopScreencastReturnObject;
}
impl Method for ProduceCompilationCache {
    const NAME: &'static str = "Page.produceCompilationCache";
    type ReturnObject = ProduceCompilationCacheReturnObject;
}
impl Method for AddCompilationCache {
    const NAME: &'static str = "Page.addCompilationCache";
    type ReturnObject = AddCompilationCacheReturnObject;
}
impl Method for ClearCompilationCache {
    const NAME: &'static str = "Page.clearCompilationCache";
    type ReturnObject = ClearCompilationCacheReturnObject;
}
impl Method for SetSPCTransactionMode {
    const NAME: &'static str = "Page.setSPCTransactionMode";
    type ReturnObject = SetSPCTransactionModeReturnObject;
}
impl Method for SetRPHRegistrationMode {
    const NAME: &'static str = "Page.setRPHRegistrationMode";
    type ReturnObject = SetRPHRegistrationModeReturnObject;
}
impl Method for GenerateTestReport {
    const NAME: &'static str = "Page.generateTestReport";
    type ReturnObject = GenerateTestReportReturnObject;
}
impl Method for WaitForDebugger {
    const NAME: &'static str = "Page.waitForDebugger";
    type ReturnObject = WaitForDebuggerReturnObject;
}
impl Method for SetInterceptFileChooserDialog {
    const NAME: &'static str = "Page.setInterceptFileChooserDialog";
    type ReturnObject = SetInterceptFileChooserDialogReturnObject;
}
impl Method for SetPrerenderingAllowed {
    const NAME: &'static str = "Page.setPrerenderingAllowed";
    type ReturnObject = SetPrerenderingAllowedReturnObject;
}
pub mod events {
    #[allow(unused_imports)]
    use super::super::types::*;
    #[allow(unused_imports)]
    use serde::{Deserialize, Serialize};
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
    pub struct DomContentEventFiredEvent {
        pub params: DomContentEventFiredEventParams,
    }
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
    pub struct DomContentEventFiredEventParams {
        #[serde(rename = "timestamp")]
        pub timestamp: super::super::network::MonotonicTime,
    }
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
    pub struct FileChooserOpenedEvent {
        pub params: FileChooserOpenedEventParams,
    }
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
    pub struct FileChooserOpenedEventParams {
        #[serde(rename = "frameId")]
        pub frame_id: super::FrameId,
        #[serde(rename = "mode")]
        pub mode: super::FileChooserOpenedModeOption,
        #[serde(skip_serializing_if = "Option::is_none")]
        #[serde(rename = "backendNodeId")]
        pub backend_node_id: Option<super::super::dom::BackendNodeId>,
    }
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
    pub struct FrameAttachedEvent {
        pub params: FrameAttachedEventParams,
    }
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
    pub struct FrameAttachedEventParams {
        #[serde(rename = "frameId")]
        pub frame_id: super::FrameId,
        #[serde(rename = "parentFrameId")]
        pub parent_frame_id: super::FrameId,
        #[serde(skip_serializing_if = "Option::is_none")]
        #[serde(rename = "stack")]
        pub stack: Option<super::super::runtime::StackTrace>,
    }
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
    pub struct FrameClearedScheduledNavigationEvent {
        pub params: FrameClearedScheduledNavigationEventParams,
    }
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
    pub struct FrameClearedScheduledNavigationEventParams {
        #[serde(rename = "frameId")]
        pub frame_id: super::FrameId,
    }
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
    pub struct FrameDetachedEvent {
        pub params: FrameDetachedEventParams,
    }
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
    pub struct FrameDetachedEventParams {
        #[serde(rename = "frameId")]
        pub frame_id: super::FrameId,
        #[serde(rename = "reason")]
        pub reason: super::FrameDetachedReasonOption,
    }
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
    pub struct FrameSubtreeWillBeDetachedEvent {
        pub params: FrameSubtreeWillBeDetachedEventParams,
    }
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
    pub struct FrameSubtreeWillBeDetachedEventParams {
        #[serde(rename = "frameId")]
        pub frame_id: super::FrameId,
    }
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
    pub struct FrameNavigatedEvent {
        pub params: FrameNavigatedEventParams,
    }
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
    pub struct FrameNavigatedEventParams {
        #[serde(rename = "frame")]
        pub frame: super::Frame,
        #[serde(rename = "type")]
        pub r#type: super::NavigationType,
    }
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
    pub struct DocumentOpenedEvent {
        pub params: DocumentOpenedEventParams,
    }
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
    pub struct DocumentOpenedEventParams {
        #[serde(rename = "frame")]
        pub frame: super::Frame,
    }
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
    #[serde(rename_all = "camelCase")]
    pub struct FrameResizedEvent(pub Option<serde_json::Value>);
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
    pub struct FrameStartedNavigatingEvent {
        pub params: FrameStartedNavigatingEventParams,
    }
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
    pub struct FrameStartedNavigatingEventParams {
        #[serde(rename = "frameId")]
        pub frame_id: super::FrameId,
        #[serde(default)]
        #[serde(rename = "url")]
        pub url: String,
        #[serde(rename = "loaderId")]
        pub loader_id: super::super::network::LoaderId,
        #[serde(rename = "navigationType")]
        pub navigation_type: super::FrameStartedNavigatingNavigationTypeOption,
    }
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
    pub struct FrameRequestedNavigationEvent {
        pub params: FrameRequestedNavigationEventParams,
    }
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
    pub struct FrameRequestedNavigationEventParams {
        #[serde(rename = "frameId")]
        pub frame_id: super::FrameId,
        #[serde(rename = "reason")]
        pub reason: super::ClientNavigationReason,
        #[serde(default)]
        #[serde(rename = "url")]
        pub url: String,
        #[serde(rename = "disposition")]
        pub disposition: super::ClientNavigationDisposition,
    }
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
    pub struct FrameScheduledNavigationEvent {
        pub params: FrameScheduledNavigationEventParams,
    }
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
    pub struct FrameScheduledNavigationEventParams {
        #[serde(rename = "frameId")]
        pub frame_id: super::FrameId,
        #[serde(default)]
        #[serde(rename = "delay")]
        pub delay: JsFloat,
        #[serde(rename = "reason")]
        pub reason: super::ClientNavigationReason,
        #[serde(default)]
        #[serde(rename = "url")]
        pub url: String,
    }
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
    pub struct FrameStartedLoadingEvent {
        pub params: FrameStartedLoadingEventParams,
    }
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
    pub struct FrameStartedLoadingEventParams {
        #[serde(rename = "frameId")]
        pub frame_id: super::FrameId,
    }
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
    pub struct FrameStoppedLoadingEvent {
        pub params: FrameStoppedLoadingEventParams,
    }
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
    pub struct FrameStoppedLoadingEventParams {
        #[serde(rename = "frameId")]
        pub frame_id: super::FrameId,
    }
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
    pub struct DownloadWillBeginEvent {
        pub params: DownloadWillBeginEventParams,
    }
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
    pub struct DownloadWillBeginEventParams {
        #[serde(rename = "frameId")]
        pub frame_id: super::FrameId,
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
    }
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
    #[serde(rename_all = "camelCase")]
    pub struct InterstitialHiddenEvent(pub Option<serde_json::Value>);
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
    #[serde(rename_all = "camelCase")]
    pub struct InterstitialShownEvent(pub Option<serde_json::Value>);
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
    pub struct JavascriptDialogClosedEvent {
        pub params: JavascriptDialogClosedEventParams,
    }
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
    pub struct JavascriptDialogClosedEventParams {
        #[serde(rename = "frameId")]
        pub frame_id: super::FrameId,
        #[serde(default)]
        #[serde(rename = "result")]
        pub result: bool,
        #[serde(default)]
        #[serde(rename = "userInput")]
        pub user_input: String,
    }
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
    pub struct JavascriptDialogOpeningEvent {
        pub params: JavascriptDialogOpeningEventParams,
    }
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
    pub struct JavascriptDialogOpeningEventParams {
        #[serde(default)]
        #[serde(rename = "url")]
        pub url: String,
        #[serde(rename = "frameId")]
        pub frame_id: super::FrameId,
        #[serde(default)]
        #[serde(rename = "message")]
        pub message: String,
        #[serde(rename = "type")]
        pub r#type: super::DialogType,
        #[serde(default)]
        #[serde(rename = "hasBrowserHandler")]
        pub has_browser_handler: bool,
        #[serde(skip_serializing_if = "Option::is_none")]
        #[serde(default)]
        #[serde(rename = "defaultPrompt")]
        pub default_prompt: Option<String>,
    }
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
    pub struct LifecycleEventEvent {
        pub params: LifecycleEventEventParams,
    }
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
    pub struct LifecycleEventEventParams {
        #[serde(rename = "frameId")]
        pub frame_id: super::FrameId,
        #[serde(rename = "loaderId")]
        pub loader_id: super::super::network::LoaderId,
        #[serde(default)]
        #[serde(rename = "name")]
        pub name: String,
        #[serde(rename = "timestamp")]
        pub timestamp: super::super::network::MonotonicTime,
    }
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
    pub struct BackForwardCacheNotUsedEvent {
        pub params: BackForwardCacheNotUsedEventParams,
    }
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
    pub struct BackForwardCacheNotUsedEventParams {
        #[serde(rename = "loaderId")]
        pub loader_id: super::super::network::LoaderId,
        #[serde(rename = "frameId")]
        pub frame_id: super::FrameId,
        #[serde(rename = "notRestoredExplanations")]
        pub not_restored_explanations: Vec<super::BackForwardCacheNotRestoredExplanation>,
        #[serde(skip_serializing_if = "Option::is_none")]
        #[serde(rename = "notRestoredExplanationsTree")]
        pub not_restored_explanations_tree:
            Option<super::BackForwardCacheNotRestoredExplanationTree>,
    }
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
    pub struct LoadEventFiredEvent {
        pub params: LoadEventFiredEventParams,
    }
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
    pub struct LoadEventFiredEventParams {
        #[serde(rename = "timestamp")]
        pub timestamp: super::super::network::MonotonicTime,
    }
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
    pub struct NavigatedWithinDocumentEvent {
        pub params: NavigatedWithinDocumentEventParams,
    }
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
    pub struct NavigatedWithinDocumentEventParams {
        #[serde(rename = "frameId")]
        pub frame_id: super::FrameId,
        #[serde(default)]
        #[serde(rename = "url")]
        pub url: String,
        #[serde(rename = "navigationType")]
        pub navigation_type: super::NavigatedWithinDocumentNavigationTypeOption,
    }
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
    pub struct ScreencastFrameEvent {
        pub params: ScreencastFrameEventParams,
    }
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
    pub struct ScreencastFrameEventParams {
        #[serde(default)]
        #[serde(rename = "data")]
        pub data: u8,
        #[serde(rename = "metadata")]
        pub metadata: super::ScreencastFrameMetadata,
        #[serde(default)]
        #[serde(rename = "sessionId")]
        pub session_id: JsUInt,
    }
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
    pub struct ScreencastVisibilityChangedEvent {
        pub params: ScreencastVisibilityChangedEventParams,
    }
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
    pub struct ScreencastVisibilityChangedEventParams {
        #[serde(default)]
        #[serde(rename = "visible")]
        pub visible: bool,
    }
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
    pub struct WindowOpenEvent {
        pub params: WindowOpenEventParams,
    }
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
    pub struct WindowOpenEventParams {
        #[serde(default)]
        #[serde(rename = "url")]
        pub url: String,
        #[serde(default)]
        #[serde(rename = "windowName")]
        pub window_name: String,
        #[serde(default)]
        #[serde(rename = "windowFeatures")]
        pub window_features: Vec<String>,
        #[serde(default)]
        #[serde(rename = "userGesture")]
        pub user_gesture: bool,
    }
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
    pub struct CompilationCacheProducedEvent {
        pub params: CompilationCacheProducedEventParams,
    }
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
    pub struct CompilationCacheProducedEventParams {
        #[serde(default)]
        #[serde(rename = "url")]
        pub url: String,
        #[serde(default)]
        #[serde(rename = "data")]
        pub data: u8,
    }
}
