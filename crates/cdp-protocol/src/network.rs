// Auto-generated from Chrome at version 140.0.7339.186 domain: Network
use super::debugger;
use super::emulation;
use super::io;
use super::network;
use super::page;
use super::runtime;
use super::security;
#[allow(unused_imports)]
use super::types::*;
#[allow(unused_imports)]
use serde::{Deserialize, Serialize};
#[allow(unused_imports)]
use serde_json::Value as Json;
pub type LoaderId = String;
pub type RequestId = String;
pub type InterceptionId = String;
pub type TimeSinceEpoch = JsFloat;
pub type MonotonicTime = JsFloat;
pub type ReportId = String;
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub enum ResourceType {
    #[serde(rename = "Document")]
    Document,
    #[serde(rename = "Stylesheet")]
    Stylesheet,
    #[serde(rename = "Image")]
    Image,
    #[serde(rename = "Media")]
    Media,
    #[serde(rename = "Font")]
    Font,
    #[serde(rename = "Script")]
    Script,
    #[serde(rename = "TextTrack")]
    TextTrack,
    #[serde(rename = "XHR")]
    Xhr,
    #[serde(rename = "Fetch")]
    Fetch,
    #[serde(rename = "Prefetch")]
    Prefetch,
    #[serde(rename = "EventSource")]
    EventSource,
    #[serde(rename = "WebSocket")]
    WebSocket,
    #[serde(rename = "Manifest")]
    Manifest,
    #[serde(rename = "SignedExchange")]
    SignedExchange,
    #[serde(rename = "Ping")]
    Ping,
    #[serde(rename = "CSPViolationReport")]
    CspViolationReport,
    #[serde(rename = "Preflight")]
    Preflight,
    #[serde(rename = "FedCM")]
    FedCm,
    #[serde(rename = "Other")]
    Other,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub enum ErrorReason {
    #[serde(rename = "Failed")]
    Failed,
    #[serde(rename = "Aborted")]
    Aborted,
    #[serde(rename = "TimedOut")]
    TimedOut,
    #[serde(rename = "AccessDenied")]
    AccessDenied,
    #[serde(rename = "ConnectionClosed")]
    ConnectionClosed,
    #[serde(rename = "ConnectionReset")]
    ConnectionReset,
    #[serde(rename = "ConnectionRefused")]
    ConnectionRefused,
    #[serde(rename = "ConnectionAborted")]
    ConnectionAborted,
    #[serde(rename = "ConnectionFailed")]
    ConnectionFailed,
    #[serde(rename = "NameNotResolved")]
    NameNotResolved,
    #[serde(rename = "InternetDisconnected")]
    InternetDisconnected,
    #[serde(rename = "AddressUnreachable")]
    AddressUnreachable,
    #[serde(rename = "BlockedByClient")]
    BlockedByClient,
    #[serde(rename = "BlockedByResponse")]
    BlockedByResponse,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub enum ConnectionType {
    #[serde(rename = "none")]
    None,
    #[serde(rename = "cellular2g")]
    Cellular2G,
    #[serde(rename = "cellular3g")]
    Cellular3G,
    #[serde(rename = "cellular4g")]
    Cellular4G,
    #[serde(rename = "bluetooth")]
    Bluetooth,
    #[serde(rename = "ethernet")]
    Ethernet,
    #[serde(rename = "wifi")]
    Wifi,
    #[serde(rename = "wimax")]
    Wimax,
    #[serde(rename = "other")]
    Other,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub enum CookieSameSite {
    #[serde(rename = "Strict")]
    Strict,
    #[serde(rename = "Lax")]
    Lax,
    #[serde(rename = "None")]
    None,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub enum CookiePriority {
    #[serde(rename = "Low")]
    Low,
    #[serde(rename = "Medium")]
    Medium,
    #[serde(rename = "High")]
    High,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub enum CookieSourceScheme {
    #[serde(rename = "Unset")]
    Unset,
    #[serde(rename = "NonSecure")]
    NonSecure,
    #[serde(rename = "Secure")]
    Secure,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub enum ResourcePriority {
    #[serde(rename = "VeryLow")]
    VeryLow,
    #[serde(rename = "Low")]
    Low,
    #[serde(rename = "Medium")]
    Medium,
    #[serde(rename = "High")]
    High,
    #[serde(rename = "VeryHigh")]
    VeryHigh,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub enum RequestReferrerPolicy {
    #[serde(rename = "unsafe-url")]
    UnsafeUrl,
    #[serde(rename = "no-referrer-when-downgrade")]
    NoReferrerWhenDowngrade,
    #[serde(rename = "no-referrer")]
    NoReferrer,
    #[serde(rename = "origin")]
    Origin,
    #[serde(rename = "origin-when-cross-origin")]
    OriginWhenCrossOrigin,
    #[serde(rename = "same-origin")]
    SameOrigin,
    #[serde(rename = "strict-origin")]
    StrictOrigin,
    #[serde(rename = "strict-origin-when-cross-origin")]
    StrictOriginWhenCrossOrigin,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub enum CertificateTransparencyCompliance {
    #[serde(rename = "unknown")]
    Unknown,
    #[serde(rename = "not-compliant")]
    NotCompliant,
    #[serde(rename = "compliant")]
    Compliant,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub enum BlockedReason {
    #[serde(rename = "other")]
    Other,
    #[serde(rename = "csp")]
    Csp,
    #[serde(rename = "mixed-content")]
    MixedContent,
    #[serde(rename = "origin")]
    Origin,
    #[serde(rename = "inspector")]
    Inspector,
    #[serde(rename = "integrity")]
    Integrity,
    #[serde(rename = "subresource-filter")]
    SubresourceFilter,
    #[serde(rename = "content-type")]
    ContentType,
    #[serde(rename = "coep-frame-resource-needs-coep-header")]
    CoepFrameResourceNeedsCoepHeader,
    #[serde(rename = "coop-sandboxed-iframe-cannot-navigate-to-coop-page")]
    CoopSandboxedIframeCannotNavigateToCoopPage,
    #[serde(rename = "corp-not-same-origin")]
    CorpNotSameOrigin,
    #[serde(rename = "corp-not-same-origin-after-defaulted-to-same-origin-by-coep")]
    CorpNotSameOriginAfterDefaultedToSameOriginByCoep,
    #[serde(rename = "corp-not-same-origin-after-defaulted-to-same-origin-by-dip")]
    CorpNotSameOriginAfterDefaultedToSameOriginByDip,
    #[serde(rename = "corp-not-same-origin-after-defaulted-to-same-origin-by-coep-and-dip")]
    CorpNotSameOriginAfterDefaultedToSameOriginByCoepAndDip,
    #[serde(rename = "corp-not-same-site")]
    CorpNotSameSite,
    #[serde(rename = "sri-message-signature-mismatch")]
    SriMessageSignatureMismatch,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub enum CorsError {
    #[serde(rename = "DisallowedByMode")]
    DisallowedByMode,
    #[serde(rename = "InvalidResponse")]
    InvalidResponse,
    #[serde(rename = "WildcardOriginNotAllowed")]
    WildcardOriginNotAllowed,
    #[serde(rename = "MissingAllowOriginHeader")]
    MissingAllowOriginHeader,
    #[serde(rename = "MultipleAllowOriginValues")]
    MultipleAllowOriginValues,
    #[serde(rename = "InvalidAllowOriginValue")]
    InvalidAllowOriginValue,
    #[serde(rename = "AllowOriginMismatch")]
    AllowOriginMismatch,
    #[serde(rename = "InvalidAllowCredentials")]
    InvalidAllowCredentials,
    #[serde(rename = "CorsDisabledScheme")]
    CorsDisabledScheme,
    #[serde(rename = "PreflightInvalidStatus")]
    PreflightInvalidStatus,
    #[serde(rename = "PreflightDisallowedRedirect")]
    PreflightDisallowedRedirect,
    #[serde(rename = "PreflightWildcardOriginNotAllowed")]
    PreflightWildcardOriginNotAllowed,
    #[serde(rename = "PreflightMissingAllowOriginHeader")]
    PreflightMissingAllowOriginHeader,
    #[serde(rename = "PreflightMultipleAllowOriginValues")]
    PreflightMultipleAllowOriginValues,
    #[serde(rename = "PreflightInvalidAllowOriginValue")]
    PreflightInvalidAllowOriginValue,
    #[serde(rename = "PreflightAllowOriginMismatch")]
    PreflightAllowOriginMismatch,
    #[serde(rename = "PreflightInvalidAllowCredentials")]
    PreflightInvalidAllowCredentials,
    #[serde(rename = "PreflightMissingAllowExternal")]
    PreflightMissingAllowExternal,
    #[serde(rename = "PreflightInvalidAllowExternal")]
    PreflightInvalidAllowExternal,
    #[serde(rename = "PreflightMissingAllowPrivateNetwork")]
    PreflightMissingAllowPrivateNetwork,
    #[serde(rename = "PreflightInvalidAllowPrivateNetwork")]
    PreflightInvalidAllowPrivateNetwork,
    #[serde(rename = "InvalidAllowMethodsPreflightResponse")]
    InvalidAllowMethodsPreflightResponse,
    #[serde(rename = "InvalidAllowHeadersPreflightResponse")]
    InvalidAllowHeadersPreflightResponse,
    #[serde(rename = "MethodDisallowedByPreflightResponse")]
    MethodDisallowedByPreflightResponse,
    #[serde(rename = "HeaderDisallowedByPreflightResponse")]
    HeaderDisallowedByPreflightResponse,
    #[serde(rename = "RedirectContainsCredentials")]
    RedirectContainsCredentials,
    #[serde(rename = "InsecurePrivateNetwork")]
    InsecurePrivateNetwork,
    #[serde(rename = "InvalidPrivateNetworkAccess")]
    InvalidPrivateNetworkAccess,
    #[serde(rename = "UnexpectedPrivateNetworkAccess")]
    UnexpectedPrivateNetworkAccess,
    #[serde(rename = "NoCorsRedirectModeNotFollow")]
    NoCorsRedirectModeNotFollow,
    #[serde(rename = "PreflightMissingPrivateNetworkAccessId")]
    PreflightMissingPrivateNetworkAccessId,
    #[serde(rename = "PreflightMissingPrivateNetworkAccessName")]
    PreflightMissingPrivateNetworkAccessName,
    #[serde(rename = "PrivateNetworkAccessPermissionUnavailable")]
    PrivateNetworkAccessPermissionUnavailable,
    #[serde(rename = "PrivateNetworkAccessPermissionDenied")]
    PrivateNetworkAccessPermissionDenied,
    #[serde(rename = "LocalNetworkAccessPermissionDenied")]
    LocalNetworkAccessPermissionDenied,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub enum ServiceWorkerResponseSource {
    #[serde(rename = "cache-storage")]
    CacheStorage,
    #[serde(rename = "http-cache")]
    HttpCache,
    #[serde(rename = "fallback-code")]
    FallbackCode,
    #[serde(rename = "network")]
    Network,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub enum TrustTokenParamsRefreshPolicy {
    #[serde(rename = "UseCached")]
    UseCached,
    #[serde(rename = "Refresh")]
    Refresh,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub enum TrustTokenOperationType {
    #[serde(rename = "Issuance")]
    Issuance,
    #[serde(rename = "Redemption")]
    Redemption,
    #[serde(rename = "Signing")]
    Signing,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub enum AlternateProtocolUsage {
    #[serde(rename = "alternativeJobWonWithoutRace")]
    AlternativeJobWonWithoutRace,
    #[serde(rename = "alternativeJobWonRace")]
    AlternativeJobWonRace,
    #[serde(rename = "mainJobWonRace")]
    MainJobWonRace,
    #[serde(rename = "mappingMissing")]
    MappingMissing,
    #[serde(rename = "broken")]
    Broken,
    #[serde(rename = "dnsAlpnH3JobWonWithoutRace")]
    DnsAlpnH3JobWonWithoutRace,
    #[serde(rename = "dnsAlpnH3JobWonRace")]
    DnsAlpnH3JobWonRace,
    #[serde(rename = "unspecifiedReason")]
    UnspecifiedReason,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub enum ServiceWorkerRouterSource {
    #[serde(rename = "network")]
    Network,
    #[serde(rename = "cache")]
    Cache,
    #[serde(rename = "fetch-event")]
    FetchEvent,
    #[serde(rename = "race-network-and-fetch-handler")]
    RaceNetworkAndFetchHandler,
    #[serde(rename = "race-network-and-cache")]
    RaceNetworkAndCache,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub enum InitiatorType {
    #[serde(rename = "parser")]
    Parser,
    #[serde(rename = "script")]
    Script,
    #[serde(rename = "preload")]
    Preload,
    #[serde(rename = "SignedExchange")]
    SignedExchange,
    #[serde(rename = "preflight")]
    Preflight,
    #[serde(rename = "other")]
    Other,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub enum SetCookieBlockedReason {
    #[serde(rename = "SecureOnly")]
    SecureOnly,
    #[serde(rename = "SameSiteStrict")]
    SameSiteStrict,
    #[serde(rename = "SameSiteLax")]
    SameSiteLax,
    #[serde(rename = "SameSiteUnspecifiedTreatedAsLax")]
    SameSiteUnspecifiedTreatedAsLax,
    #[serde(rename = "SameSiteNoneInsecure")]
    SameSiteNoneInsecure,
    #[serde(rename = "UserPreferences")]
    UserPreferences,
    #[serde(rename = "ThirdPartyPhaseout")]
    ThirdPartyPhaseout,
    #[serde(rename = "ThirdPartyBlockedInFirstPartySet")]
    ThirdPartyBlockedInFirstPartySet,
    #[serde(rename = "SyntaxError")]
    SyntaxError,
    #[serde(rename = "SchemeNotSupported")]
    SchemeNotSupported,
    #[serde(rename = "OverwriteSecure")]
    OverwriteSecure,
    #[serde(rename = "InvalidDomain")]
    InvalidDomain,
    #[serde(rename = "InvalidPrefix")]
    InvalidPrefix,
    #[serde(rename = "UnknownError")]
    UnknownError,
    #[serde(rename = "SchemefulSameSiteStrict")]
    SchemefulSameSiteStrict,
    #[serde(rename = "SchemefulSameSiteLax")]
    SchemefulSameSiteLax,
    #[serde(rename = "SchemefulSameSiteUnspecifiedTreatedAsLax")]
    SchemefulSameSiteUnspecifiedTreatedAsLax,
    #[serde(rename = "SamePartyFromCrossPartyContext")]
    SamePartyFromCrossPartyContext,
    #[serde(rename = "SamePartyConflictsWithOtherAttributes")]
    SamePartyConflictsWithOtherAttributes,
    #[serde(rename = "NameValuePairExceedsMaxSize")]
    NameValuePairExceedsMaxSize,
    #[serde(rename = "DisallowedCharacter")]
    DisallowedCharacter,
    #[serde(rename = "NoCookieContent")]
    NoCookieContent,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub enum CookieBlockedReason {
    #[serde(rename = "SecureOnly")]
    SecureOnly,
    #[serde(rename = "NotOnPath")]
    NotOnPath,
    #[serde(rename = "DomainMismatch")]
    DomainMismatch,
    #[serde(rename = "SameSiteStrict")]
    SameSiteStrict,
    #[serde(rename = "SameSiteLax")]
    SameSiteLax,
    #[serde(rename = "SameSiteUnspecifiedTreatedAsLax")]
    SameSiteUnspecifiedTreatedAsLax,
    #[serde(rename = "SameSiteNoneInsecure")]
    SameSiteNoneInsecure,
    #[serde(rename = "UserPreferences")]
    UserPreferences,
    #[serde(rename = "ThirdPartyPhaseout")]
    ThirdPartyPhaseout,
    #[serde(rename = "ThirdPartyBlockedInFirstPartySet")]
    ThirdPartyBlockedInFirstPartySet,
    #[serde(rename = "UnknownError")]
    UnknownError,
    #[serde(rename = "SchemefulSameSiteStrict")]
    SchemefulSameSiteStrict,
    #[serde(rename = "SchemefulSameSiteLax")]
    SchemefulSameSiteLax,
    #[serde(rename = "SchemefulSameSiteUnspecifiedTreatedAsLax")]
    SchemefulSameSiteUnspecifiedTreatedAsLax,
    #[serde(rename = "SamePartyFromCrossPartyContext")]
    SamePartyFromCrossPartyContext,
    #[serde(rename = "NameValuePairExceedsMaxSize")]
    NameValuePairExceedsMaxSize,
    #[serde(rename = "PortMismatch")]
    PortMismatch,
    #[serde(rename = "SchemeMismatch")]
    SchemeMismatch,
    #[serde(rename = "AnonymousContext")]
    AnonymousContext,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub enum CookieExemptionReason {
    #[serde(rename = "None")]
    None,
    #[serde(rename = "UserSetting")]
    UserSetting,
    #[serde(rename = "TPCDMetadata")]
    TpcdMetadata,
    #[serde(rename = "TPCDDeprecationTrial")]
    TpcdDeprecationTrial,
    #[serde(rename = "TopLevelTPCDDeprecationTrial")]
    TopLevelTpcdDeprecationTrial,
    #[serde(rename = "TPCDHeuristics")]
    TpcdHeuristics,
    #[serde(rename = "EnterprisePolicy")]
    EnterprisePolicy,
    #[serde(rename = "StorageAccess")]
    StorageAccess,
    #[serde(rename = "TopLevelStorageAccess")]
    TopLevelStorageAccess,
    #[serde(rename = "Scheme")]
    Scheme,
    #[serde(rename = "SameSiteNoneCookiesInSandbox")]
    SameSiteNoneCookiesInSandbox,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub enum AuthChallengeSource {
    #[serde(rename = "Server")]
    Server,
    #[serde(rename = "Proxy")]
    Proxy,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub enum AuthChallengeResponseResponse {
    #[serde(rename = "Default")]
    Default,
    #[serde(rename = "CancelAuth")]
    CancelAuth,
    #[serde(rename = "ProvideCredentials")]
    ProvideCredentials,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub enum InterceptionStage {
    #[serde(rename = "Request")]
    Request,
    #[serde(rename = "HeadersReceived")]
    HeadersReceived,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub enum SignedExchangeErrorField {
    #[serde(rename = "signatureSig")]
    SignatureSig,
    #[serde(rename = "signatureIntegrity")]
    SignatureIntegrity,
    #[serde(rename = "signatureCertUrl")]
    SignatureCertUrl,
    #[serde(rename = "signatureCertSha256")]
    SignatureCertSha256,
    #[serde(rename = "signatureValidityUrl")]
    SignatureValidityUrl,
    #[serde(rename = "signatureTimestamps")]
    SignatureTimestamps,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub enum ContentEncoding {
    #[serde(rename = "deflate")]
    Deflate,
    #[serde(rename = "gzip")]
    Gzip,
    #[serde(rename = "br")]
    Br,
    #[serde(rename = "zstd")]
    Zstd,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub enum DirectSocketDnsQueryType {
    #[serde(rename = "ipv4")]
    Ipv4,
    #[serde(rename = "ipv6")]
    Ipv6,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub enum PrivateNetworkRequestPolicy {
    #[serde(rename = "Allow")]
    Allow,
    #[serde(rename = "BlockFromInsecureToMorePrivate")]
    BlockFromInsecureToMorePrivate,
    #[serde(rename = "WarnFromInsecureToMorePrivate")]
    WarnFromInsecureToMorePrivate,
    #[serde(rename = "PreflightBlock")]
    PreflightBlock,
    #[serde(rename = "PreflightWarn")]
    PreflightWarn,
    #[serde(rename = "PermissionBlock")]
    PermissionBlock,
    #[serde(rename = "PermissionWarn")]
    PermissionWarn,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub enum IpAddressSpace {
    #[serde(rename = "Loopback")]
    Loopback,
    #[serde(rename = "Local")]
    Local,
    #[serde(rename = "Public")]
    Public,
    #[serde(rename = "Unknown")]
    Unknown,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub enum CrossOriginOpenerPolicyValue {
    #[serde(rename = "SameOrigin")]
    SameOrigin,
    #[serde(rename = "SameOriginAllowPopups")]
    SameOriginAllowPopups,
    #[serde(rename = "RestrictProperties")]
    RestrictProperties,
    #[serde(rename = "UnsafeNone")]
    UnsafeNone,
    #[serde(rename = "SameOriginPlusCoep")]
    SameOriginPlusCoep,
    #[serde(rename = "RestrictPropertiesPlusCoep")]
    RestrictPropertiesPlusCoep,
    #[serde(rename = "NoopenerAllowPopups")]
    NoopenerAllowPopups,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub enum CrossOriginEmbedderPolicyValue {
    #[serde(rename = "None")]
    None,
    #[serde(rename = "Credentialless")]
    Credentialless,
    #[serde(rename = "RequireCorp")]
    RequireCorp,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub enum ContentSecurityPolicySource {
    #[serde(rename = "HTTP")]
    Http,
    #[serde(rename = "Meta")]
    Meta,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub enum ReportStatus {
    #[serde(rename = "Queued")]
    Queued,
    #[serde(rename = "Pending")]
    Pending,
    #[serde(rename = "MarkedForRemoval")]
    MarkedForRemoval,
    #[serde(rename = "Success")]
    Success,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub enum TrustTokenOperationDoneStatusOption {
    #[serde(rename = "Ok")]
    Ok,
    #[serde(rename = "InvalidArgument")]
    InvalidArgument,
    #[serde(rename = "MissingIssuerKeys")]
    MissingIssuerKeys,
    #[serde(rename = "FailedPrecondition")]
    FailedPrecondition,
    #[serde(rename = "ResourceExhausted")]
    ResourceExhausted,
    #[serde(rename = "AlreadyExists")]
    AlreadyExists,
    #[serde(rename = "ResourceLimited")]
    ResourceLimited,
    #[serde(rename = "Unauthorized")]
    Unauthorized,
    #[serde(rename = "BadResponse")]
    BadResponse,
    #[serde(rename = "InternalError")]
    InternalError,
    #[serde(rename = "UnknownError")]
    UnknownError,
    #[serde(rename = "FulfilledLocally")]
    FulfilledLocally,
    #[serde(rename = "SiteIssuerLimit")]
    SiteIssuerLimit,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Headers(pub Option<serde_json::Value>);
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct ResourceTiming {
    #[serde(default)]
    #[serde(rename = "requestTime")]
    pub request_time: JsFloat,
    #[serde(default)]
    #[serde(rename = "proxyStart")]
    pub proxy_start: JsFloat,
    #[serde(default)]
    #[serde(rename = "proxyEnd")]
    pub proxy_end: JsFloat,
    #[serde(default)]
    #[serde(rename = "dnsStart")]
    pub dns_start: JsFloat,
    #[serde(default)]
    #[serde(rename = "dnsEnd")]
    pub dns_end: JsFloat,
    #[serde(default)]
    #[serde(rename = "connectStart")]
    pub connect_start: JsFloat,
    #[serde(default)]
    #[serde(rename = "connectEnd")]
    pub connect_end: JsFloat,
    #[serde(default)]
    #[serde(rename = "sslStart")]
    pub ssl_start: JsFloat,
    #[serde(default)]
    #[serde(rename = "sslEnd")]
    pub ssl_end: JsFloat,
    #[serde(default)]
    #[serde(rename = "workerStart")]
    pub worker_start: JsFloat,
    #[serde(default)]
    #[serde(rename = "workerReady")]
    pub worker_ready: JsFloat,
    #[serde(default)]
    #[serde(rename = "workerFetchStart")]
    pub worker_fetch_start: JsFloat,
    #[serde(default)]
    #[serde(rename = "workerRespondWithSettled")]
    pub worker_respond_with_settled: JsFloat,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(rename = "workerRouterEvaluationStart")]
    pub worker_router_evaluation_start: Option<JsFloat>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(rename = "workerCacheLookupStart")]
    pub worker_cache_lookup_start: Option<JsFloat>,
    #[serde(default)]
    #[serde(rename = "sendStart")]
    pub send_start: JsFloat,
    #[serde(default)]
    #[serde(rename = "sendEnd")]
    pub send_end: JsFloat,
    #[serde(default)]
    #[serde(rename = "pushStart")]
    pub push_start: JsFloat,
    #[serde(default)]
    #[serde(rename = "pushEnd")]
    pub push_end: JsFloat,
    #[serde(default)]
    #[serde(rename = "receiveHeadersStart")]
    pub receive_headers_start: JsFloat,
    #[serde(default)]
    #[serde(rename = "receiveHeadersEnd")]
    pub receive_headers_end: JsFloat,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct PostDataEntry {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(rename = "bytes")]
    pub bytes: Option<String>,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct Request {
    #[serde(default)]
    #[serde(rename = "url")]
    pub url: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(rename = "urlFragment")]
    pub url_fragment: Option<String>,
    #[serde(default)]
    #[serde(rename = "method")]
    pub method: String,
    #[serde(rename = "headers")]
    pub headers: Headers,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(rename = "postData")]
    pub post_data: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(rename = "hasPostData")]
    pub has_post_data: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "postDataEntries")]
    pub post_data_entries: Option<Vec<PostDataEntry>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "mixedContentType")]
    pub mixed_content_type: Option<security::MixedContentType>,
    #[serde(rename = "initialPriority")]
    pub initial_priority: ResourcePriority,
    #[serde(rename = "referrerPolicy")]
    pub referrer_policy: RequestReferrerPolicy,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(rename = "isLinkPreload")]
    pub is_link_preload: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "trustTokenParams")]
    pub trust_token_params: Option<TrustTokenParams>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(rename = "isSameSite")]
    pub is_same_site: Option<bool>,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct SignedCertificateTimestamp {
    #[serde(default)]
    #[serde(rename = "status")]
    pub status: String,
    #[serde(default)]
    #[serde(rename = "origin")]
    pub origin: String,
    #[serde(default)]
    #[serde(rename = "logDescription")]
    pub log_description: String,
    #[serde(default)]
    #[serde(rename = "logId")]
    pub log_id: String,
    #[serde(default)]
    #[serde(rename = "timestamp")]
    pub timestamp: JsFloat,
    #[serde(default)]
    #[serde(rename = "hashAlgorithm")]
    pub hash_algorithm: String,
    #[serde(default)]
    #[serde(rename = "signatureAlgorithm")]
    pub signature_algorithm: String,
    #[serde(default)]
    #[serde(rename = "signatureData")]
    pub signature_data: String,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct SecurityDetails {
    #[serde(default)]
    #[serde(rename = "protocol")]
    pub protocol: String,
    #[serde(default)]
    #[serde(rename = "keyExchange")]
    pub key_exchange: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(rename = "keyExchangeGroup")]
    pub key_exchange_group: Option<String>,
    #[serde(default)]
    #[serde(rename = "cipher")]
    pub cipher: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(rename = "mac")]
    pub mac: Option<String>,
    #[serde(rename = "certificateId")]
    pub certificate_id: security::CertificateId,
    #[serde(default)]
    #[serde(rename = "subjectName")]
    pub subject_name: String,
    #[serde(default)]
    #[serde(rename = "sanList")]
    pub san_list: Vec<String>,
    #[serde(default)]
    #[serde(rename = "issuer")]
    pub issuer: String,
    #[serde(rename = "validFrom")]
    pub valid_from: TimeSinceEpoch,
    #[serde(rename = "validTo")]
    pub valid_to: TimeSinceEpoch,
    #[serde(rename = "signedCertificateTimestampList")]
    pub signed_certificate_timestamp_list: Vec<SignedCertificateTimestamp>,
    #[serde(rename = "certificateTransparencyCompliance")]
    pub certificate_transparency_compliance: CertificateTransparencyCompliance,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(rename = "serverSignatureAlgorithm")]
    pub server_signature_algorithm: Option<JsUInt>,
    #[serde(default)]
    #[serde(rename = "encryptedClientHello")]
    pub encrypted_client_hello: bool,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct CorsErrorStatus {
    #[serde(rename = "corsError")]
    pub cors_error: CorsError,
    #[serde(default)]
    #[serde(rename = "failedParameter")]
    pub failed_parameter: String,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct TrustTokenParams {
    #[serde(rename = "operation")]
    pub operation: TrustTokenOperationType,
    #[serde(rename = "refreshPolicy")]
    pub refresh_policy: TrustTokenParamsRefreshPolicy,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(rename = "issuers")]
    pub issuers: Option<Vec<String>>,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct ServiceWorkerRouterInfo {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(rename = "ruleIdMatched")]
    pub rule_id_matched: Option<JsUInt>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "matchedSourceType")]
    pub matched_source_type: Option<ServiceWorkerRouterSource>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "actualSourceType")]
    pub actual_source_type: Option<ServiceWorkerRouterSource>,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct Response {
    #[serde(default)]
    #[serde(rename = "url")]
    pub url: String,
    #[serde(default)]
    #[serde(rename = "status")]
    pub status: JsUInt,
    #[serde(default)]
    #[serde(rename = "statusText")]
    pub status_text: String,
    #[serde(rename = "headers")]
    pub headers: Headers,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(rename = "headersText")]
    pub headers_text: Option<String>,
    #[serde(default)]
    #[serde(rename = "mimeType")]
    pub mime_type: String,
    #[serde(default)]
    #[serde(rename = "charset")]
    pub charset: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "requestHeaders")]
    pub request_headers: Option<Headers>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(rename = "requestHeadersText")]
    pub request_headers_text: Option<String>,
    #[serde(default)]
    #[serde(rename = "connectionReused")]
    pub connection_reused: bool,
    #[serde(default)]
    #[serde(rename = "connectionId")]
    pub connection_id: JsFloat,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(rename = "remoteIPAddress")]
    pub remote_ip_address: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(rename = "remotePort")]
    pub remote_port: Option<JsUInt>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(rename = "fromDiskCache")]
    pub from_disk_cache: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(rename = "fromServiceWorker")]
    pub from_service_worker: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(rename = "fromPrefetchCache")]
    pub from_prefetch_cache: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(rename = "fromEarlyHints")]
    pub from_early_hints: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "serviceWorkerRouterInfo")]
    pub service_worker_router_info: Option<ServiceWorkerRouterInfo>,
    #[serde(default)]
    #[serde(rename = "encodedDataLength")]
    pub encoded_data_length: JsFloat,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "timing")]
    pub timing: Option<ResourceTiming>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "serviceWorkerResponseSource")]
    pub service_worker_response_source: Option<ServiceWorkerResponseSource>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "responseTime")]
    pub response_time: Option<TimeSinceEpoch>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(rename = "cacheStorageCacheName")]
    pub cache_storage_cache_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(rename = "protocol")]
    pub protocol: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "alternateProtocolUsage")]
    pub alternate_protocol_usage: Option<AlternateProtocolUsage>,
    #[serde(rename = "securityState")]
    pub security_state: security::SecurityState,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "securityDetails")]
    pub security_details: Option<SecurityDetails>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(rename = "isIpProtectionUsed")]
    pub is_ip_protection_used: Option<bool>,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct WebSocketRequest {
    #[serde(rename = "headers")]
    pub headers: Headers,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct WebSocketResponse {
    #[serde(default)]
    #[serde(rename = "status")]
    pub status: JsUInt,
    #[serde(default)]
    #[serde(rename = "statusText")]
    pub status_text: String,
    #[serde(rename = "headers")]
    pub headers: Headers,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(rename = "headersText")]
    pub headers_text: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "requestHeaders")]
    pub request_headers: Option<Headers>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(rename = "requestHeadersText")]
    pub request_headers_text: Option<String>,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct WebSocketFrame {
    #[serde(default)]
    #[serde(rename = "opcode")]
    pub opcode: JsFloat,
    #[serde(default)]
    #[serde(rename = "mask")]
    pub mask: bool,
    #[serde(default)]
    #[serde(rename = "payloadData")]
    pub payload_data: String,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct CachedResource {
    #[serde(default)]
    #[serde(rename = "url")]
    pub url: String,
    #[serde(rename = "type")]
    pub r#type: ResourceType,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "response")]
    pub response: Option<Response>,
    #[serde(default)]
    #[serde(rename = "bodySize")]
    pub body_size: JsFloat,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct Initiator {
    #[serde(rename = "type")]
    pub r#type: InitiatorType,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "stack")]
    pub stack: Option<runtime::StackTrace>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(rename = "url")]
    pub url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(rename = "lineNumber")]
    pub line_number: Option<JsFloat>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(rename = "columnNumber")]
    pub column_number: Option<JsFloat>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "requestId")]
    pub request_id: Option<RequestId>,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct CookiePartitionKey {
    #[serde(default)]
    #[serde(rename = "topLevelSite")]
    pub top_level_site: String,
    #[serde(default)]
    #[serde(rename = "hasCrossSiteAncestor")]
    pub has_cross_site_ancestor: bool,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct Cookie {
    #[serde(default)]
    #[serde(rename = "name")]
    pub name: String,
    #[serde(default)]
    #[serde(rename = "value")]
    pub value: String,
    #[serde(default)]
    #[serde(rename = "domain")]
    pub domain: String,
    #[serde(default)]
    #[serde(rename = "path")]
    pub path: String,
    #[serde(default)]
    #[serde(rename = "expires")]
    pub expires: JsFloat,
    #[serde(default)]
    #[serde(rename = "size")]
    pub size: JsUInt,
    #[serde(default)]
    #[serde(rename = "httpOnly")]
    pub http_only: bool,
    #[serde(default)]
    #[serde(rename = "secure")]
    pub secure: bool,
    #[serde(default)]
    #[serde(rename = "session")]
    pub session: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "sameSite")]
    pub same_site: Option<CookieSameSite>,
    #[serde(rename = "priority")]
    pub priority: CookiePriority,
    #[serde(default)]
    #[serde(rename = "sameParty")]
    pub same_party: bool,
    #[serde(rename = "sourceScheme")]
    pub source_scheme: CookieSourceScheme,
    #[serde(default)]
    #[serde(rename = "sourcePort")]
    pub source_port: JsUInt,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "partitionKey")]
    pub partition_key: Option<CookiePartitionKey>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(rename = "partitionKeyOpaque")]
    pub partition_key_opaque: Option<bool>,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct BlockedSetCookieWithReason {
    #[serde(rename = "blockedReasons")]
    pub blocked_reasons: Vec<SetCookieBlockedReason>,
    #[serde(default)]
    #[serde(rename = "cookieLine")]
    pub cookie_line: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "cookie")]
    pub cookie: Option<Cookie>,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct ExemptedSetCookieWithReason {
    #[serde(rename = "exemptionReason")]
    pub exemption_reason: CookieExemptionReason,
    #[serde(default)]
    #[serde(rename = "cookieLine")]
    pub cookie_line: String,
    #[serde(rename = "cookie")]
    pub cookie: Cookie,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct AssociatedCookie {
    #[serde(rename = "cookie")]
    pub cookie: Cookie,
    #[serde(rename = "blockedReasons")]
    pub blocked_reasons: Vec<CookieBlockedReason>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "exemptionReason")]
    pub exemption_reason: Option<CookieExemptionReason>,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct CookieParam {
    #[serde(default)]
    #[serde(rename = "name")]
    pub name: String,
    #[serde(default)]
    #[serde(rename = "value")]
    pub value: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(rename = "url")]
    pub url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(rename = "domain")]
    pub domain: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(rename = "path")]
    pub path: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(rename = "secure")]
    pub secure: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(rename = "httpOnly")]
    pub http_only: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "sameSite")]
    pub same_site: Option<CookieSameSite>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "expires")]
    pub expires: Option<TimeSinceEpoch>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "priority")]
    pub priority: Option<CookiePriority>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(rename = "sameParty")]
    pub same_party: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "sourceScheme")]
    pub source_scheme: Option<CookieSourceScheme>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(rename = "sourcePort")]
    pub source_port: Option<JsUInt>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "partitionKey")]
    pub partition_key: Option<CookiePartitionKey>,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct AuthChallenge {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "source")]
    pub source: Option<AuthChallengeSource>,
    #[serde(default)]
    #[serde(rename = "origin")]
    pub origin: String,
    #[serde(default)]
    #[serde(rename = "scheme")]
    pub scheme: String,
    #[serde(default)]
    #[serde(rename = "realm")]
    pub realm: String,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct AuthChallengeResponse {
    #[serde(rename = "response")]
    pub response: AuthChallengeResponseResponse,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(rename = "username")]
    pub username: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(rename = "password")]
    pub password: Option<String>,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct RequestPattern {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(rename = "urlPattern")]
    pub url_pattern: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "resourceType")]
    pub resource_type: Option<ResourceType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "interceptionStage")]
    pub interception_stage: Option<InterceptionStage>,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct SignedExchangeSignature {
    #[serde(default)]
    #[serde(rename = "label")]
    pub label: String,
    #[serde(default)]
    #[serde(rename = "signature")]
    pub signature: String,
    #[serde(default)]
    #[serde(rename = "integrity")]
    pub integrity: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(rename = "certUrl")]
    pub cert_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(rename = "certSha256")]
    pub cert_sha_256: Option<String>,
    #[serde(default)]
    #[serde(rename = "validityUrl")]
    pub validity_url: String,
    #[serde(default)]
    #[serde(rename = "date")]
    pub date: JsUInt,
    #[serde(default)]
    #[serde(rename = "expires")]
    pub expires: JsUInt,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(rename = "certificates")]
    pub certificates: Option<Vec<String>>,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct SignedExchangeHeader {
    #[serde(default)]
    #[serde(rename = "requestUrl")]
    pub request_url: String,
    #[serde(default)]
    #[serde(rename = "responseCode")]
    pub response_code: JsUInt,
    #[serde(rename = "responseHeaders")]
    pub response_headers: Headers,
    #[serde(rename = "signatures")]
    pub signatures: Vec<SignedExchangeSignature>,
    #[serde(default)]
    #[serde(rename = "headerIntegrity")]
    pub header_integrity: String,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct SignedExchangeError {
    #[serde(default)]
    #[serde(rename = "message")]
    pub message: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(rename = "signatureIndex")]
    pub signature_index: Option<JsUInt>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "errorField")]
    pub error_field: Option<SignedExchangeErrorField>,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct SignedExchangeInfo {
    #[serde(rename = "outerResponse")]
    pub outer_response: Response,
    #[serde(default)]
    #[serde(rename = "hasExtraInfo")]
    pub has_extra_info: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "header")]
    pub header: Option<SignedExchangeHeader>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "securityDetails")]
    pub security_details: Option<SecurityDetails>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "errors")]
    pub errors: Option<Vec<SignedExchangeError>>,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct DirectTcpSocketOptions {
    #[serde(default)]
    #[serde(rename = "noDelay")]
    pub no_delay: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(rename = "keepAliveDelay")]
    pub keep_alive_delay: Option<JsFloat>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(rename = "sendBufferSize")]
    pub send_buffer_size: Option<JsFloat>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(rename = "receiveBufferSize")]
    pub receive_buffer_size: Option<JsFloat>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "dnsQueryType")]
    pub dns_query_type: Option<DirectSocketDnsQueryType>,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct DirectUdpSocketOptions {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(rename = "remoteAddr")]
    pub remote_addr: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(rename = "remotePort")]
    pub remote_port: Option<JsUInt>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(rename = "localAddr")]
    pub local_addr: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(rename = "localPort")]
    pub local_port: Option<JsUInt>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "dnsQueryType")]
    pub dns_query_type: Option<DirectSocketDnsQueryType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(rename = "sendBufferSize")]
    pub send_buffer_size: Option<JsFloat>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(rename = "receiveBufferSize")]
    pub receive_buffer_size: Option<JsFloat>,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct DirectUdpMessage {
    #[serde(rename = "data")]
    pub data: Vec<u8>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(rename = "remoteAddr")]
    pub remote_addr: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(rename = "remotePort")]
    pub remote_port: Option<JsUInt>,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct ConnectTiming {
    #[serde(default)]
    #[serde(rename = "requestTime")]
    pub request_time: JsFloat,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct ClientSecurityState {
    #[serde(default)]
    #[serde(rename = "initiatorIsSecureContext")]
    pub initiator_is_secure_context: bool,
    #[serde(rename = "initiatorIPAddressSpace")]
    pub initiator_ip_address_space: IpAddressSpace,
    #[serde(rename = "privateNetworkRequestPolicy")]
    pub private_network_request_policy: PrivateNetworkRequestPolicy,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct CrossOriginOpenerPolicyStatus {
    #[serde(rename = "value")]
    pub value: CrossOriginOpenerPolicyValue,
    #[serde(rename = "reportOnlyValue")]
    pub report_only_value: CrossOriginOpenerPolicyValue,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(rename = "reportingEndpoint")]
    pub reporting_endpoint: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(rename = "reportOnlyReportingEndpoint")]
    pub report_only_reporting_endpoint: Option<String>,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct CrossOriginEmbedderPolicyStatus {
    #[serde(rename = "value")]
    pub value: CrossOriginEmbedderPolicyValue,
    #[serde(rename = "reportOnlyValue")]
    pub report_only_value: CrossOriginEmbedderPolicyValue,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(rename = "reportingEndpoint")]
    pub reporting_endpoint: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(rename = "reportOnlyReportingEndpoint")]
    pub report_only_reporting_endpoint: Option<String>,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct ContentSecurityPolicyStatus {
    #[serde(default)]
    #[serde(rename = "effectiveDirectives")]
    pub effective_directives: String,
    #[serde(default)]
    #[serde(rename = "isEnforced")]
    pub is_enforced: bool,
    #[serde(rename = "source")]
    pub source: ContentSecurityPolicySource,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct SecurityIsolationStatus {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "coop")]
    pub coop: Option<CrossOriginOpenerPolicyStatus>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "coep")]
    pub coep: Option<CrossOriginEmbedderPolicyStatus>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "csp")]
    pub csp: Option<Vec<ContentSecurityPolicyStatus>>,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct ReportingApiReport {
    #[serde(rename = "id")]
    pub id: ReportId,
    #[serde(default)]
    #[serde(rename = "initiatorUrl")]
    pub initiator_url: String,
    #[serde(default)]
    #[serde(rename = "destination")]
    pub destination: String,
    #[serde(default)]
    #[serde(rename = "type")]
    pub r#type: String,
    #[serde(rename = "timestamp")]
    pub timestamp: network::TimeSinceEpoch,
    #[serde(default)]
    #[serde(rename = "depth")]
    pub depth: JsUInt,
    #[serde(default)]
    #[serde(rename = "completedAttempts")]
    pub completed_attempts: JsUInt,
    #[serde(rename = "status")]
    pub status: ReportStatus,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct ReportingApiEndpoint {
    #[serde(default)]
    #[serde(rename = "url")]
    pub url: String,
    #[serde(default)]
    #[serde(rename = "groupName")]
    pub group_name: String,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct LoadNetworkResourcePageResult {
    #[serde(default)]
    #[serde(rename = "success")]
    pub success: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(rename = "netError")]
    pub net_error: Option<JsFloat>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(rename = "netErrorName")]
    pub net_error_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(rename = "httpStatusCode")]
    pub http_status_code: Option<JsFloat>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "stream")]
    pub stream: Option<io::StreamHandle>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "headers")]
    pub headers: Option<network::Headers>,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct LoadNetworkResourceOptions {
    #[serde(default)]
    #[serde(rename = "disableCache")]
    pub disable_cache: bool,
    #[serde(default)]
    #[serde(rename = "includeCredentials")]
    pub include_credentials: bool,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct SetAcceptedEncodings {
    #[serde(rename = "encodings")]
    pub encodings: Vec<ContentEncoding>,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct ClearAcceptedEncodingsOverride(pub Option<serde_json::Value>);
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct CanClearBrowserCache(pub Option<serde_json::Value>);
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct CanClearBrowserCookies(pub Option<serde_json::Value>);
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct CanEmulateNetworkConditions(pub Option<serde_json::Value>);
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct ClearBrowserCache(pub Option<serde_json::Value>);
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct ClearBrowserCookies(pub Option<serde_json::Value>);
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct ContinueInterceptedRequest {
    #[serde(rename = "interceptionId")]
    pub interception_id: InterceptionId,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "errorReason")]
    pub error_reason: Option<ErrorReason>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "rawResponse")]
    pub raw_response: Option<Vec<u8>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(rename = "url")]
    pub url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(rename = "method")]
    pub method: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(rename = "postData")]
    pub post_data: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "headers")]
    pub headers: Option<Headers>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "authChallengeResponse")]
    pub auth_challenge_response: Option<AuthChallengeResponse>,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct DeleteCookies {
    #[serde(default)]
    #[serde(rename = "name")]
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(rename = "url")]
    pub url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(rename = "domain")]
    pub domain: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(rename = "path")]
    pub path: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "partitionKey")]
    pub partition_key: Option<CookiePartitionKey>,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Disable(pub Option<serde_json::Value>);
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct EmulateNetworkConditions {
    #[serde(default)]
    #[serde(rename = "offline")]
    pub offline: bool,
    #[serde(default)]
    #[serde(rename = "latency")]
    pub latency: JsFloat,
    #[serde(default)]
    #[serde(rename = "downloadThroughput")]
    pub download_throughput: JsFloat,
    #[serde(default)]
    #[serde(rename = "uploadThroughput")]
    pub upload_throughput: JsFloat,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "connectionType")]
    pub connection_type: Option<ConnectionType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(rename = "packetLoss")]
    pub packet_loss: Option<JsFloat>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(rename = "packetQueueLength")]
    pub packet_queue_length: Option<JsUInt>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(rename = "packetReordering")]
    pub packet_reordering: Option<bool>,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct Enable {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(rename = "maxTotalBufferSize")]
    pub max_total_buffer_size: Option<JsUInt>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(rename = "maxResourceBufferSize")]
    pub max_resource_buffer_size: Option<JsUInt>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(rename = "maxPostDataSize")]
    pub max_post_data_size: Option<JsUInt>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(rename = "reportDirectSocketTraffic")]
    pub report_direct_socket_traffic: Option<bool>,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct GetAllCookies(pub Option<serde_json::Value>);
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct GetCertificate {
    #[serde(default)]
    #[serde(rename = "origin")]
    pub origin: String,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct GetCookies {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(rename = "urls")]
    pub urls: Option<Vec<String>>,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct GetResponseBody {
    #[serde(rename = "requestId")]
    pub request_id: RequestId,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct GetRequestPostData {
    #[serde(rename = "requestId")]
    pub request_id: RequestId,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct GetResponseBodyForInterception {
    #[serde(rename = "interceptionId")]
    pub interception_id: InterceptionId,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct TakeResponseBodyForInterceptionAsStream {
    #[serde(rename = "interceptionId")]
    pub interception_id: InterceptionId,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct ReplayXHR {
    #[serde(rename = "requestId")]
    pub request_id: RequestId,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct SearchInResponseBody {
    #[serde(rename = "requestId")]
    pub request_id: RequestId,
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
pub struct SetBlockedURLs {
    #[serde(default)]
    #[serde(rename = "urls")]
    pub urls: Vec<String>,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct SetBypassServiceWorker {
    #[serde(default)]
    #[serde(rename = "bypass")]
    pub bypass: bool,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct SetCacheDisabled {
    #[serde(default)]
    #[serde(rename = "cacheDisabled")]
    pub cache_disabled: bool,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct SetCookie {
    #[serde(default)]
    #[serde(rename = "name")]
    pub name: String,
    #[serde(default)]
    #[serde(rename = "value")]
    pub value: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(rename = "url")]
    pub url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(rename = "domain")]
    pub domain: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(rename = "path")]
    pub path: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(rename = "secure")]
    pub secure: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(rename = "httpOnly")]
    pub http_only: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "sameSite")]
    pub same_site: Option<CookieSameSite>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "expires")]
    pub expires: Option<TimeSinceEpoch>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "priority")]
    pub priority: Option<CookiePriority>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(rename = "sameParty")]
    pub same_party: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "sourceScheme")]
    pub source_scheme: Option<CookieSourceScheme>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(rename = "sourcePort")]
    pub source_port: Option<JsUInt>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "partitionKey")]
    pub partition_key: Option<CookiePartitionKey>,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct SetCookies {
    #[serde(rename = "cookies")]
    pub cookies: Vec<CookieParam>,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct SetExtraHTTPHeaders {
    #[serde(rename = "headers")]
    pub headers: Headers,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct SetAttachDebugStack {
    #[serde(default)]
    #[serde(rename = "enabled")]
    pub enabled: bool,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct SetRequestInterception {
    #[serde(rename = "patterns")]
    pub patterns: Vec<RequestPattern>,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct SetUserAgentOverride {
    #[serde(default)]
    #[serde(rename = "userAgent")]
    pub user_agent: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(rename = "acceptLanguage")]
    pub accept_language: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(rename = "platform")]
    pub platform: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "userAgentMetadata")]
    pub user_agent_metadata: Option<emulation::UserAgentMetadata>,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct StreamResourceContent {
    #[serde(rename = "requestId")]
    pub request_id: RequestId,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct GetSecurityIsolationStatus {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "frameId")]
    pub frame_id: Option<page::FrameId>,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct EnableReportingApi {
    #[serde(default)]
    #[serde(rename = "enable")]
    pub enable: bool,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct LoadNetworkResource {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "frameId")]
    pub frame_id: Option<page::FrameId>,
    #[serde(default)]
    #[serde(rename = "url")]
    pub url: String,
    #[serde(rename = "options")]
    pub options: LoadNetworkResourceOptions,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct SetCookieControls {
    #[serde(default)]
    #[serde(rename = "enableThirdPartyCookieRestriction")]
    pub enable_third_party_cookie_restriction: bool,
    #[serde(default)]
    #[serde(rename = "disableThirdPartyCookieMetadata")]
    pub disable_third_party_cookie_metadata: bool,
    #[serde(default)]
    #[serde(rename = "disableThirdPartyCookieHeuristics")]
    pub disable_third_party_cookie_heuristics: bool,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct SetAcceptedEncodingsReturnObject {}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct ClearAcceptedEncodingsOverrideReturnObject {}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct CanClearBrowserCacheReturnObject {
    #[serde(default)]
    #[serde(rename = "result")]
    pub result: bool,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct CanClearBrowserCookiesReturnObject {
    #[serde(default)]
    #[serde(rename = "result")]
    pub result: bool,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct CanEmulateNetworkConditionsReturnObject {
    #[serde(default)]
    #[serde(rename = "result")]
    pub result: bool,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct ClearBrowserCacheReturnObject {}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct ClearBrowserCookiesReturnObject {}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct ContinueInterceptedRequestReturnObject {}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct DeleteCookiesReturnObject {}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct DisableReturnObject {}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct EmulateNetworkConditionsReturnObject {}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct EnableReturnObject {}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct GetAllCookiesReturnObject {
    #[serde(rename = "cookies")]
    pub cookies: Vec<Cookie>,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct GetCertificateReturnObject {
    #[serde(rename = "tableNames")]
    pub table_names: Vec<String>,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct GetCookiesReturnObject {
    #[serde(rename = "cookies")]
    pub cookies: Vec<Cookie>,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct GetResponseBodyReturnObject {
    #[serde(default)]
    #[serde(rename = "body")]
    pub body: String,
    #[serde(default)]
    #[serde(rename = "base64Encoded")]
    pub base_64_encoded: bool,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct GetRequestPostDataReturnObject {
    #[serde(default)]
    #[serde(rename = "postData")]
    pub post_data: String,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct GetResponseBodyForInterceptionReturnObject {
    #[serde(default)]
    #[serde(rename = "body")]
    pub body: String,
    #[serde(default)]
    #[serde(rename = "base64Encoded")]
    pub base_64_encoded: bool,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct TakeResponseBodyForInterceptionAsStreamReturnObject {
    #[serde(rename = "stream")]
    pub stream: io::StreamHandle,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct ReplayXHRReturnObject {}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct SearchInResponseBodyReturnObject {
    #[serde(rename = "result")]
    pub result: debugger::SearchMatch,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct SetBlockedURLsReturnObject {}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct SetBypassServiceWorkerReturnObject {}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct SetCacheDisabledReturnObject {}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct SetCookieReturnObject {
    #[serde(default)]
    #[serde(rename = "success")]
    pub success: bool,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct SetCookiesReturnObject {}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct SetExtraHTTPHeadersReturnObject {}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct SetAttachDebugStackReturnObject {}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct SetRequestInterceptionReturnObject {}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct SetUserAgentOverrideReturnObject {}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct StreamResourceContentReturnObject {
    #[serde(rename = "bufferedData")]
    pub buffered_data: Vec<u8>,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct GetSecurityIsolationStatusReturnObject {
    #[serde(rename = "status")]
    pub status: SecurityIsolationStatus,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct EnableReportingApiReturnObject {}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct LoadNetworkResourceReturnObject {
    #[serde(rename = "resource")]
    pub resource: LoadNetworkResourcePageResult,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct SetCookieControlsReturnObject {}
impl Method for SetAcceptedEncodings {
    const NAME: &'static str = "Network.setAcceptedEncodings";
    type ReturnObject = SetAcceptedEncodingsReturnObject;
}
impl Method for ClearAcceptedEncodingsOverride {
    const NAME: &'static str = "Network.clearAcceptedEncodingsOverride";
    type ReturnObject = ClearAcceptedEncodingsOverrideReturnObject;
}
impl Method for CanClearBrowserCache {
    const NAME: &'static str = "Network.canClearBrowserCache";
    type ReturnObject = CanClearBrowserCacheReturnObject;
}
impl Method for CanClearBrowserCookies {
    const NAME: &'static str = "Network.canClearBrowserCookies";
    type ReturnObject = CanClearBrowserCookiesReturnObject;
}
impl Method for CanEmulateNetworkConditions {
    const NAME: &'static str = "Network.canEmulateNetworkConditions";
    type ReturnObject = CanEmulateNetworkConditionsReturnObject;
}
impl Method for ClearBrowserCache {
    const NAME: &'static str = "Network.clearBrowserCache";
    type ReturnObject = ClearBrowserCacheReturnObject;
}
impl Method for ClearBrowserCookies {
    const NAME: &'static str = "Network.clearBrowserCookies";
    type ReturnObject = ClearBrowserCookiesReturnObject;
}
impl Method for ContinueInterceptedRequest {
    const NAME: &'static str = "Network.continueInterceptedRequest";
    type ReturnObject = ContinueInterceptedRequestReturnObject;
}
impl Method for DeleteCookies {
    const NAME: &'static str = "Network.deleteCookies";
    type ReturnObject = DeleteCookiesReturnObject;
}
impl Method for Disable {
    const NAME: &'static str = "Network.disable";
    type ReturnObject = DisableReturnObject;
}
impl Method for EmulateNetworkConditions {
    const NAME: &'static str = "Network.emulateNetworkConditions";
    type ReturnObject = EmulateNetworkConditionsReturnObject;
}
impl Method for Enable {
    const NAME: &'static str = "Network.enable";
    type ReturnObject = EnableReturnObject;
}
impl Method for GetAllCookies {
    const NAME: &'static str = "Network.getAllCookies";
    type ReturnObject = GetAllCookiesReturnObject;
}
impl Method for GetCertificate {
    const NAME: &'static str = "Network.getCertificate";
    type ReturnObject = GetCertificateReturnObject;
}
impl Method for GetCookies {
    const NAME: &'static str = "Network.getCookies";
    type ReturnObject = GetCookiesReturnObject;
}
impl Method for GetResponseBody {
    const NAME: &'static str = "Network.getResponseBody";
    type ReturnObject = GetResponseBodyReturnObject;
}
impl Method for GetRequestPostData {
    const NAME: &'static str = "Network.getRequestPostData";
    type ReturnObject = GetRequestPostDataReturnObject;
}
impl Method for GetResponseBodyForInterception {
    const NAME: &'static str = "Network.getResponseBodyForInterception";
    type ReturnObject = GetResponseBodyForInterceptionReturnObject;
}
impl Method for TakeResponseBodyForInterceptionAsStream {
    const NAME: &'static str = "Network.takeResponseBodyForInterceptionAsStream";
    type ReturnObject = TakeResponseBodyForInterceptionAsStreamReturnObject;
}
impl Method for ReplayXHR {
    const NAME: &'static str = "Network.replayXHR";
    type ReturnObject = ReplayXHRReturnObject;
}
impl Method for SearchInResponseBody {
    const NAME: &'static str = "Network.searchInResponseBody";
    type ReturnObject = SearchInResponseBodyReturnObject;
}
impl Method for SetBlockedURLs {
    const NAME: &'static str = "Network.setBlockedURLs";
    type ReturnObject = SetBlockedURLsReturnObject;
}
impl Method for SetBypassServiceWorker {
    const NAME: &'static str = "Network.setBypassServiceWorker";
    type ReturnObject = SetBypassServiceWorkerReturnObject;
}
impl Method for SetCacheDisabled {
    const NAME: &'static str = "Network.setCacheDisabled";
    type ReturnObject = SetCacheDisabledReturnObject;
}
impl Method for SetCookie {
    const NAME: &'static str = "Network.setCookie";
    type ReturnObject = SetCookieReturnObject;
}
impl Method for SetCookies {
    const NAME: &'static str = "Network.setCookies";
    type ReturnObject = SetCookiesReturnObject;
}
impl Method for SetExtraHTTPHeaders {
    const NAME: &'static str = "Network.setExtraHTTPHeaders";
    type ReturnObject = SetExtraHTTPHeadersReturnObject;
}
impl Method for SetAttachDebugStack {
    const NAME: &'static str = "Network.setAttachDebugStack";
    type ReturnObject = SetAttachDebugStackReturnObject;
}
impl Method for SetRequestInterception {
    const NAME: &'static str = "Network.setRequestInterception";
    type ReturnObject = SetRequestInterceptionReturnObject;
}
impl Method for SetUserAgentOverride {
    const NAME: &'static str = "Network.setUserAgentOverride";
    type ReturnObject = SetUserAgentOverrideReturnObject;
}
impl Method for StreamResourceContent {
    const NAME: &'static str = "Network.streamResourceContent";
    type ReturnObject = StreamResourceContentReturnObject;
}
impl Method for GetSecurityIsolationStatus {
    const NAME: &'static str = "Network.getSecurityIsolationStatus";
    type ReturnObject = GetSecurityIsolationStatusReturnObject;
}
impl Method for EnableReportingApi {
    const NAME: &'static str = "Network.enableReportingApi";
    type ReturnObject = EnableReportingApiReturnObject;
}
impl Method for LoadNetworkResource {
    const NAME: &'static str = "Network.loadNetworkResource";
    type ReturnObject = LoadNetworkResourceReturnObject;
}
impl Method for SetCookieControls {
    const NAME: &'static str = "Network.setCookieControls";
    type ReturnObject = SetCookieControlsReturnObject;
}
pub mod events {
    #[allow(unused_imports)]
    use super::super::types::*;
    #[allow(unused_imports)]
    use serde::{Deserialize, Serialize};
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
    pub struct DataReceivedEvent {
        pub params: DataReceivedEventParams,
    }
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
    pub struct DataReceivedEventParams {
        #[serde(rename = "requestId")]
        pub request_id: super::RequestId,
        #[serde(rename = "timestamp")]
        pub timestamp: super::MonotonicTime,
        #[serde(default)]
        #[serde(rename = "dataLength")]
        pub data_length: JsUInt,
        #[serde(default)]
        #[serde(rename = "encodedDataLength")]
        pub encoded_data_length: JsUInt,
        #[serde(skip_serializing_if = "Option::is_none")]
        #[serde(default)]
        #[serde(rename = "data")]
        pub data: Option<u8>,
    }
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
    pub struct EventSourceMessageReceivedEvent {
        pub params: EventSourceMessageReceivedEventParams,
    }
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
    pub struct EventSourceMessageReceivedEventParams {
        #[serde(rename = "requestId")]
        pub request_id: super::RequestId,
        #[serde(rename = "timestamp")]
        pub timestamp: super::MonotonicTime,
        #[serde(default)]
        #[serde(rename = "eventName")]
        pub event_name: String,
        #[serde(default)]
        #[serde(rename = "eventId")]
        pub event_id: String,
        #[serde(default)]
        #[serde(rename = "data")]
        pub data: String,
    }
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
    pub struct LoadingFailedEvent {
        pub params: LoadingFailedEventParams,
    }
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
    pub struct LoadingFailedEventParams {
        #[serde(rename = "requestId")]
        pub request_id: super::RequestId,
        #[serde(rename = "timestamp")]
        pub timestamp: super::MonotonicTime,
        #[serde(rename = "type")]
        pub r#type: super::ResourceType,
        #[serde(default)]
        #[serde(rename = "errorText")]
        pub error_text: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        #[serde(default)]
        #[serde(rename = "canceled")]
        pub canceled: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none")]
        #[serde(rename = "blockedReason")]
        pub blocked_reason: Option<super::BlockedReason>,
        #[serde(skip_serializing_if = "Option::is_none")]
        #[serde(rename = "corsErrorStatus")]
        pub cors_error_status: Option<super::CorsErrorStatus>,
    }
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
    pub struct LoadingFinishedEvent {
        pub params: LoadingFinishedEventParams,
    }
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
    pub struct LoadingFinishedEventParams {
        #[serde(rename = "requestId")]
        pub request_id: super::RequestId,
        #[serde(rename = "timestamp")]
        pub timestamp: super::MonotonicTime,
        #[serde(default)]
        #[serde(rename = "encodedDataLength")]
        pub encoded_data_length: JsFloat,
    }
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
    pub struct RequestInterceptedEvent {
        pub params: RequestInterceptedEventParams,
    }
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
    pub struct RequestInterceptedEventParams {
        #[serde(rename = "interceptionId")]
        pub interception_id: super::InterceptionId,
        #[serde(rename = "request")]
        pub request: super::Request,
        #[serde(rename = "frameId")]
        pub frame_id: super::super::page::FrameId,
        #[serde(rename = "resourceType")]
        pub resource_type: super::ResourceType,
        #[serde(default)]
        #[serde(rename = "isNavigationRequest")]
        pub is_navigation_request: bool,
        #[serde(skip_serializing_if = "Option::is_none")]
        #[serde(default)]
        #[serde(rename = "isDownload")]
        pub is_download: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none")]
        #[serde(default)]
        #[serde(rename = "redirectUrl")]
        pub redirect_url: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        #[serde(rename = "authChallenge")]
        pub auth_challenge: Option<super::AuthChallenge>,
        #[serde(skip_serializing_if = "Option::is_none")]
        #[serde(rename = "responseErrorReason")]
        pub response_error_reason: Option<super::ErrorReason>,
        #[serde(skip_serializing_if = "Option::is_none")]
        #[serde(default)]
        #[serde(rename = "responseStatusCode")]
        pub response_status_code: Option<JsUInt>,
        #[serde(skip_serializing_if = "Option::is_none")]
        #[serde(rename = "responseHeaders")]
        pub response_headers: Option<super::Headers>,
        #[serde(skip_serializing_if = "Option::is_none")]
        #[serde(rename = "requestId")]
        pub request_id: Option<super::RequestId>,
    }
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
    pub struct RequestServedFromCacheEvent {
        pub params: RequestServedFromCacheEventParams,
    }
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
    pub struct RequestServedFromCacheEventParams {
        #[serde(rename = "requestId")]
        pub request_id: super::RequestId,
    }
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
    pub struct RequestWillBeSentEvent {
        pub params: RequestWillBeSentEventParams,
    }
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
    pub struct RequestWillBeSentEventParams {
        #[serde(rename = "requestId")]
        pub request_id: super::RequestId,
        #[serde(rename = "loaderId")]
        pub loader_id: super::LoaderId,
        #[serde(default)]
        #[serde(rename = "documentURL")]
        pub document_url: String,
        #[serde(rename = "request")]
        pub request: super::Request,
        #[serde(rename = "timestamp")]
        pub timestamp: super::MonotonicTime,
        #[serde(rename = "wallTime")]
        pub wall_time: super::TimeSinceEpoch,
        #[serde(rename = "initiator")]
        pub initiator: super::Initiator,
        #[serde(default)]
        #[serde(rename = "redirectHasExtraInfo")]
        pub redirect_has_extra_info: bool,
        #[serde(skip_serializing_if = "Option::is_none")]
        #[serde(rename = "redirectResponse")]
        pub redirect_response: Option<super::Response>,
        #[serde(skip_serializing_if = "Option::is_none")]
        #[serde(rename = "type")]
        pub r#type: Option<super::ResourceType>,
        #[serde(skip_serializing_if = "Option::is_none")]
        #[serde(rename = "frameId")]
        pub frame_id: Option<super::super::page::FrameId>,
        #[serde(skip_serializing_if = "Option::is_none")]
        #[serde(default)]
        #[serde(rename = "hasUserGesture")]
        pub has_user_gesture: Option<bool>,
    }
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
    pub struct ResourceChangedPriorityEvent {
        pub params: ResourceChangedPriorityEventParams,
    }
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
    pub struct ResourceChangedPriorityEventParams {
        #[serde(rename = "requestId")]
        pub request_id: super::RequestId,
        #[serde(rename = "newPriority")]
        pub new_priority: super::ResourcePriority,
        #[serde(rename = "timestamp")]
        pub timestamp: super::MonotonicTime,
    }
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
    pub struct SignedExchangeReceivedEvent {
        pub params: SignedExchangeReceivedEventParams,
    }
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
    pub struct SignedExchangeReceivedEventParams {
        #[serde(rename = "requestId")]
        pub request_id: super::RequestId,
        #[serde(rename = "info")]
        pub info: super::SignedExchangeInfo,
    }
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
    pub struct ResponseReceivedEvent {
        pub params: ResponseReceivedEventParams,
    }
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
    pub struct ResponseReceivedEventParams {
        #[serde(rename = "requestId")]
        pub request_id: super::RequestId,
        #[serde(rename = "loaderId")]
        pub loader_id: super::LoaderId,
        #[serde(rename = "timestamp")]
        pub timestamp: super::MonotonicTime,
        #[serde(rename = "type")]
        pub r#type: super::ResourceType,
        #[serde(rename = "response")]
        pub response: super::Response,
        #[serde(default)]
        #[serde(rename = "hasExtraInfo")]
        pub has_extra_info: bool,
        #[serde(skip_serializing_if = "Option::is_none")]
        #[serde(rename = "frameId")]
        pub frame_id: Option<super::super::page::FrameId>,
    }
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
    pub struct WebSocketClosedEvent {
        pub params: WebSocketClosedEventParams,
    }
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
    pub struct WebSocketClosedEventParams {
        #[serde(rename = "requestId")]
        pub request_id: super::RequestId,
        #[serde(rename = "timestamp")]
        pub timestamp: super::MonotonicTime,
    }
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
    pub struct WebSocketCreatedEvent {
        pub params: WebSocketCreatedEventParams,
    }
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
    pub struct WebSocketCreatedEventParams {
        #[serde(rename = "requestId")]
        pub request_id: super::RequestId,
        #[serde(default)]
        #[serde(rename = "url")]
        pub url: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        #[serde(rename = "initiator")]
        pub initiator: Option<super::Initiator>,
    }
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
    pub struct WebSocketFrameErrorEvent {
        pub params: WebSocketFrameErrorEventParams,
    }
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
    pub struct WebSocketFrameErrorEventParams {
        #[serde(rename = "requestId")]
        pub request_id: super::RequestId,
        #[serde(rename = "timestamp")]
        pub timestamp: super::MonotonicTime,
        #[serde(default)]
        #[serde(rename = "errorMessage")]
        pub error_message: String,
    }
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
    pub struct WebSocketFrameReceivedEvent {
        pub params: WebSocketFrameReceivedEventParams,
    }
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
    pub struct WebSocketFrameReceivedEventParams {
        #[serde(rename = "requestId")]
        pub request_id: super::RequestId,
        #[serde(rename = "timestamp")]
        pub timestamp: super::MonotonicTime,
        #[serde(rename = "response")]
        pub response: super::WebSocketFrame,
    }
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
    pub struct WebSocketFrameSentEvent {
        pub params: WebSocketFrameSentEventParams,
    }
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
    pub struct WebSocketFrameSentEventParams {
        #[serde(rename = "requestId")]
        pub request_id: super::RequestId,
        #[serde(rename = "timestamp")]
        pub timestamp: super::MonotonicTime,
        #[serde(rename = "response")]
        pub response: super::WebSocketFrame,
    }
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
    pub struct WebSocketHandshakeResponseReceivedEvent {
        pub params: WebSocketHandshakeResponseReceivedEventParams,
    }
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
    pub struct WebSocketHandshakeResponseReceivedEventParams {
        #[serde(rename = "requestId")]
        pub request_id: super::RequestId,
        #[serde(rename = "timestamp")]
        pub timestamp: super::MonotonicTime,
        #[serde(rename = "response")]
        pub response: super::WebSocketResponse,
    }
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
    pub struct WebSocketWillSendHandshakeRequestEvent {
        pub params: WebSocketWillSendHandshakeRequestEventParams,
    }
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
    pub struct WebSocketWillSendHandshakeRequestEventParams {
        #[serde(rename = "requestId")]
        pub request_id: super::RequestId,
        #[serde(rename = "timestamp")]
        pub timestamp: super::MonotonicTime,
        #[serde(rename = "wallTime")]
        pub wall_time: super::TimeSinceEpoch,
        #[serde(rename = "request")]
        pub request: super::WebSocketRequest,
    }
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
    pub struct WebTransportCreatedEvent {
        pub params: WebTransportCreatedEventParams,
    }
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
    pub struct WebTransportCreatedEventParams {
        #[serde(rename = "transportId")]
        pub transport_id: super::RequestId,
        #[serde(default)]
        #[serde(rename = "url")]
        pub url: String,
        #[serde(rename = "timestamp")]
        pub timestamp: super::MonotonicTime,
        #[serde(skip_serializing_if = "Option::is_none")]
        #[serde(rename = "initiator")]
        pub initiator: Option<super::Initiator>,
    }
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
    pub struct WebTransportConnectionEstablishedEvent {
        pub params: WebTransportConnectionEstablishedEventParams,
    }
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
    pub struct WebTransportConnectionEstablishedEventParams {
        #[serde(rename = "transportId")]
        pub transport_id: super::RequestId,
        #[serde(rename = "timestamp")]
        pub timestamp: super::MonotonicTime,
    }
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
    pub struct WebTransportClosedEvent {
        pub params: WebTransportClosedEventParams,
    }
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
    pub struct WebTransportClosedEventParams {
        #[serde(rename = "transportId")]
        pub transport_id: super::RequestId,
        #[serde(rename = "timestamp")]
        pub timestamp: super::MonotonicTime,
    }
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
    pub struct DirectTCPSocketCreatedEvent {
        pub params: DirectTCPSocketCreatedEventParams,
    }
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
    pub struct DirectTCPSocketCreatedEventParams {
        #[serde(rename = "identifier")]
        pub identifier: super::RequestId,
        #[serde(default)]
        #[serde(rename = "remoteAddr")]
        pub remote_addr: String,
        #[serde(default)]
        #[serde(rename = "remotePort")]
        pub remote_port: JsUInt,
        #[serde(rename = "options")]
        pub options: super::DirectTcpSocketOptions,
        #[serde(rename = "timestamp")]
        pub timestamp: super::MonotonicTime,
        #[serde(skip_serializing_if = "Option::is_none")]
        #[serde(rename = "initiator")]
        pub initiator: Option<super::Initiator>,
    }
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
    pub struct DirectTCPSocketOpenedEvent {
        pub params: DirectTCPSocketOpenedEventParams,
    }
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
    pub struct DirectTCPSocketOpenedEventParams {
        #[serde(rename = "identifier")]
        pub identifier: super::RequestId,
        #[serde(default)]
        #[serde(rename = "remoteAddr")]
        pub remote_addr: String,
        #[serde(default)]
        #[serde(rename = "remotePort")]
        pub remote_port: JsUInt,
        #[serde(rename = "timestamp")]
        pub timestamp: super::MonotonicTime,
        #[serde(skip_serializing_if = "Option::is_none")]
        #[serde(default)]
        #[serde(rename = "localAddr")]
        pub local_addr: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        #[serde(default)]
        #[serde(rename = "localPort")]
        pub local_port: Option<JsUInt>,
    }
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
    pub struct DirectTCPSocketAbortedEvent {
        pub params: DirectTCPSocketAbortedEventParams,
    }
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
    pub struct DirectTCPSocketAbortedEventParams {
        #[serde(rename = "identifier")]
        pub identifier: super::RequestId,
        #[serde(default)]
        #[serde(rename = "errorMessage")]
        pub error_message: String,
        #[serde(rename = "timestamp")]
        pub timestamp: super::MonotonicTime,
    }
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
    pub struct DirectTCPSocketClosedEvent {
        pub params: DirectTCPSocketClosedEventParams,
    }
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
    pub struct DirectTCPSocketClosedEventParams {
        #[serde(rename = "identifier")]
        pub identifier: super::RequestId,
        #[serde(rename = "timestamp")]
        pub timestamp: super::MonotonicTime,
    }
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
    pub struct DirectTCPSocketChunkSentEvent {
        pub params: DirectTCPSocketChunkSentEventParams,
    }
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
    pub struct DirectTCPSocketChunkSentEventParams {
        #[serde(rename = "identifier")]
        pub identifier: super::RequestId,
        #[serde(default)]
        #[serde(rename = "data")]
        pub data: u8,
        #[serde(rename = "timestamp")]
        pub timestamp: super::MonotonicTime,
    }
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
    pub struct DirectTCPSocketChunkReceivedEvent {
        pub params: DirectTCPSocketChunkReceivedEventParams,
    }
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
    pub struct DirectTCPSocketChunkReceivedEventParams {
        #[serde(rename = "identifier")]
        pub identifier: super::RequestId,
        #[serde(default)]
        #[serde(rename = "data")]
        pub data: u8,
        #[serde(rename = "timestamp")]
        pub timestamp: super::MonotonicTime,
    }
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
    pub struct DirectUDPSocketCreatedEvent {
        pub params: DirectUDPSocketCreatedEventParams,
    }
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
    pub struct DirectUDPSocketCreatedEventParams {
        #[serde(rename = "identifier")]
        pub identifier: super::RequestId,
        #[serde(rename = "options")]
        pub options: super::DirectUdpSocketOptions,
        #[serde(rename = "timestamp")]
        pub timestamp: super::MonotonicTime,
        #[serde(skip_serializing_if = "Option::is_none")]
        #[serde(rename = "initiator")]
        pub initiator: Option<super::Initiator>,
    }
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
    pub struct DirectUDPSocketOpenedEvent {
        pub params: DirectUDPSocketOpenedEventParams,
    }
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
    pub struct DirectUDPSocketOpenedEventParams {
        #[serde(rename = "identifier")]
        pub identifier: super::RequestId,
        #[serde(default)]
        #[serde(rename = "localAddr")]
        pub local_addr: String,
        #[serde(default)]
        #[serde(rename = "localPort")]
        pub local_port: JsUInt,
        #[serde(rename = "timestamp")]
        pub timestamp: super::MonotonicTime,
        #[serde(skip_serializing_if = "Option::is_none")]
        #[serde(default)]
        #[serde(rename = "remoteAddr")]
        pub remote_addr: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        #[serde(default)]
        #[serde(rename = "remotePort")]
        pub remote_port: Option<JsUInt>,
    }
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
    pub struct DirectUDPSocketAbortedEvent {
        pub params: DirectUDPSocketAbortedEventParams,
    }
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
    pub struct DirectUDPSocketAbortedEventParams {
        #[serde(rename = "identifier")]
        pub identifier: super::RequestId,
        #[serde(default)]
        #[serde(rename = "errorMessage")]
        pub error_message: String,
        #[serde(rename = "timestamp")]
        pub timestamp: super::MonotonicTime,
    }
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
    pub struct DirectUDPSocketClosedEvent {
        pub params: DirectUDPSocketClosedEventParams,
    }
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
    pub struct DirectUDPSocketClosedEventParams {
        #[serde(rename = "identifier")]
        pub identifier: super::RequestId,
        #[serde(rename = "timestamp")]
        pub timestamp: super::MonotonicTime,
    }
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
    pub struct DirectUDPSocketChunkSentEvent {
        pub params: DirectUDPSocketChunkSentEventParams,
    }
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
    pub struct DirectUDPSocketChunkSentEventParams {
        #[serde(rename = "identifier")]
        pub identifier: super::RequestId,
        #[serde(rename = "message")]
        pub message: super::DirectUdpMessage,
        #[serde(rename = "timestamp")]
        pub timestamp: super::MonotonicTime,
    }
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
    pub struct DirectUDPSocketChunkReceivedEvent {
        pub params: DirectUDPSocketChunkReceivedEventParams,
    }
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
    pub struct DirectUDPSocketChunkReceivedEventParams {
        #[serde(rename = "identifier")]
        pub identifier: super::RequestId,
        #[serde(rename = "message")]
        pub message: super::DirectUdpMessage,
        #[serde(rename = "timestamp")]
        pub timestamp: super::MonotonicTime,
    }
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
    pub struct RequestWillBeSentExtraInfoEvent {
        pub params: RequestWillBeSentExtraInfoEventParams,
    }
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
    pub struct RequestWillBeSentExtraInfoEventParams {
        #[serde(rename = "requestId")]
        pub request_id: super::RequestId,
        #[serde(rename = "associatedCookies")]
        pub associated_cookies: Vec<super::AssociatedCookie>,
        #[serde(rename = "headers")]
        pub headers: super::Headers,
        #[serde(rename = "connectTiming")]
        pub connect_timing: super::ConnectTiming,
        #[serde(skip_serializing_if = "Option::is_none")]
        #[serde(rename = "clientSecurityState")]
        pub client_security_state: Option<super::ClientSecurityState>,
        #[serde(skip_serializing_if = "Option::is_none")]
        #[serde(default)]
        #[serde(rename = "siteHasCookieInOtherPartition")]
        pub site_has_cookie_in_other_partition: Option<bool>,
    }
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
    pub struct ResponseReceivedExtraInfoEvent {
        pub params: ResponseReceivedExtraInfoEventParams,
    }
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
    pub struct ResponseReceivedExtraInfoEventParams {
        #[serde(rename = "requestId")]
        pub request_id: super::RequestId,
        #[serde(rename = "blockedCookies")]
        pub blocked_cookies: Vec<super::BlockedSetCookieWithReason>,
        #[serde(rename = "headers")]
        pub headers: super::Headers,
        #[serde(rename = "resourceIPAddressSpace")]
        pub resource_ip_address_space: super::IpAddressSpace,
        #[serde(default)]
        #[serde(rename = "statusCode")]
        pub status_code: JsUInt,
        #[serde(skip_serializing_if = "Option::is_none")]
        #[serde(default)]
        #[serde(rename = "headersText")]
        pub headers_text: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        #[serde(rename = "cookiePartitionKey")]
        pub cookie_partition_key: Option<super::CookiePartitionKey>,
        #[serde(skip_serializing_if = "Option::is_none")]
        #[serde(default)]
        #[serde(rename = "cookiePartitionKeyOpaque")]
        pub cookie_partition_key_opaque: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none")]
        #[serde(rename = "exemptedCookies")]
        pub exempted_cookies: Option<Vec<super::ExemptedSetCookieWithReason>>,
    }
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
    pub struct ResponseReceivedEarlyHintsEvent {
        pub params: ResponseReceivedEarlyHintsEventParams,
    }
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
    pub struct ResponseReceivedEarlyHintsEventParams {
        #[serde(rename = "requestId")]
        pub request_id: super::RequestId,
        #[serde(rename = "headers")]
        pub headers: super::Headers,
    }
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
    pub struct TrustTokenOperationDoneEvent {
        pub params: TrustTokenOperationDoneEventParams,
    }
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
    pub struct TrustTokenOperationDoneEventParams {
        #[serde(rename = "status")]
        pub status: super::TrustTokenOperationDoneStatusOption,
        #[serde(rename = "type")]
        pub r#type: super::TrustTokenOperationType,
        #[serde(rename = "requestId")]
        pub request_id: super::RequestId,
        #[serde(skip_serializing_if = "Option::is_none")]
        #[serde(default)]
        #[serde(rename = "topLevelOrigin")]
        pub top_level_origin: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        #[serde(default)]
        #[serde(rename = "issuerOrigin")]
        pub issuer_origin: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        #[serde(default)]
        #[serde(rename = "issuedTokenCount")]
        pub issued_token_count: Option<JsUInt>,
    }
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
    #[serde(rename_all = "camelCase")]
    pub struct PolicyUpdatedEvent(pub Option<serde_json::Value>);
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
    pub struct SubresourceWebBundleMetadataReceivedEvent {
        pub params: SubresourceWebBundleMetadataReceivedEventParams,
    }
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
    pub struct SubresourceWebBundleMetadataReceivedEventParams {
        #[serde(rename = "requestId")]
        pub request_id: super::RequestId,
        #[serde(default)]
        #[serde(rename = "urls")]
        pub urls: Vec<String>,
    }
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
    pub struct SubresourceWebBundleMetadataErrorEvent {
        pub params: SubresourceWebBundleMetadataErrorEventParams,
    }
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
    pub struct SubresourceWebBundleMetadataErrorEventParams {
        #[serde(rename = "requestId")]
        pub request_id: super::RequestId,
        #[serde(default)]
        #[serde(rename = "errorMessage")]
        pub error_message: String,
    }
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
    pub struct SubresourceWebBundleInnerResponseParsedEvent {
        pub params: SubresourceWebBundleInnerResponseParsedEventParams,
    }
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
    pub struct SubresourceWebBundleInnerResponseParsedEventParams {
        #[serde(rename = "innerRequestId")]
        pub inner_request_id: super::RequestId,
        #[serde(default)]
        #[serde(rename = "innerRequestURL")]
        pub inner_request_url: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        #[serde(rename = "bundleRequestId")]
        pub bundle_request_id: Option<super::RequestId>,
    }
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
    pub struct SubresourceWebBundleInnerResponseErrorEvent {
        pub params: SubresourceWebBundleInnerResponseErrorEventParams,
    }
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
    pub struct SubresourceWebBundleInnerResponseErrorEventParams {
        #[serde(rename = "innerRequestId")]
        pub inner_request_id: super::RequestId,
        #[serde(default)]
        #[serde(rename = "innerRequestURL")]
        pub inner_request_url: String,
        #[serde(default)]
        #[serde(rename = "errorMessage")]
        pub error_message: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        #[serde(rename = "bundleRequestId")]
        pub bundle_request_id: Option<super::RequestId>,
    }
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
    pub struct ReportingApiReportAddedEvent {
        pub params: ReportingApiReportAddedEventParams,
    }
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
    pub struct ReportingApiReportAddedEventParams {
        #[serde(rename = "report")]
        pub report: super::ReportingApiReport,
    }
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
    pub struct ReportingApiReportUpdatedEvent {
        pub params: ReportingApiReportUpdatedEventParams,
    }
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
    pub struct ReportingApiReportUpdatedEventParams {
        #[serde(rename = "report")]
        pub report: super::ReportingApiReport,
    }
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
    pub struct ReportingApiEndpointsChangedForOriginEvent {
        pub params: ReportingApiEndpointsChangedForOriginEventParams,
    }
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
    pub struct ReportingApiEndpointsChangedForOriginEventParams {
        #[serde(default)]
        #[serde(rename = "origin")]
        pub origin: String,
        #[serde(rename = "endpoints")]
        pub endpoints: Vec<super::ReportingApiEndpoint>,
    }
}
