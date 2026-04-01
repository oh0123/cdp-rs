// Auto-generated from Chrome at version 146.0.7680.165 domain: Network
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
use derive_builder::Builder;
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
pub type DeviceBoundSessionEventId = String;
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
pub enum RenderBlockingBehavior {
    #[serde(rename = "Blocking")]
    Blocking,
    #[serde(rename = "InBodyParserBlocking")]
    InBodyParserBlocking,
    #[serde(rename = "NonBlocking")]
    NonBlocking,
    #[serde(rename = "NonBlockingDynamic")]
    NonBlockingDynamic,
    #[serde(rename = "PotentiallyBlocking")]
    PotentiallyBlocking,
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
    #[serde(rename = "InsecureLocalNetwork")]
    InsecureLocalNetwork,
    #[serde(rename = "InvalidLocalNetworkAccess")]
    InvalidLocalNetworkAccess,
    #[serde(rename = "NoCorsRedirectModeNotFollow")]
    NoCorsRedirectModeNotFollow,
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
    #[serde(rename = "FedCM")]
    FedCm,
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
pub enum LocalNetworkAccessRequestPolicy {
    #[serde(rename = "Allow")]
    Allow,
    #[serde(rename = "BlockFromInsecureToMorePrivate")]
    BlockFromInsecureToMorePrivate,
    #[serde(rename = "WarnFromInsecureToMorePrivate")]
    WarnFromInsecureToMorePrivate,
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
pub enum DeviceBoundSessionWithUsageUsage {
    #[serde(rename = "NotInScope")]
    NotInScope,
    #[serde(rename = "InScopeRefreshNotYetNeeded")]
    InScopeRefreshNotYetNeeded,
    #[serde(rename = "InScopeRefreshNotAllowed")]
    InScopeRefreshNotAllowed,
    #[serde(rename = "ProactiveRefreshNotPossible")]
    ProactiveRefreshNotPossible,
    #[serde(rename = "ProactiveRefreshAttempted")]
    ProactiveRefreshAttempted,
    #[serde(rename = "Deferred")]
    Deferred,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub enum DeviceBoundSessionUrlRuleRuleType {
    #[serde(rename = "Exclude")]
    Exclude,
    #[serde(rename = "Include")]
    Include,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub enum DeviceBoundSessionFetchResult {
    #[serde(rename = "Success")]
    Success,
    #[serde(rename = "KeyError")]
    KeyError,
    #[serde(rename = "SigningError")]
    SigningError,
    #[serde(rename = "ServerRequestedTermination")]
    ServerRequestedTermination,
    #[serde(rename = "InvalidSessionId")]
    InvalidSessionId,
    #[serde(rename = "InvalidChallenge")]
    InvalidChallenge,
    #[serde(rename = "TooManyChallenges")]
    TooManyChallenges,
    #[serde(rename = "InvalidFetcherUrl")]
    InvalidFetcherUrl,
    #[serde(rename = "InvalidRefreshUrl")]
    InvalidRefreshUrl,
    #[serde(rename = "TransientHttpError")]
    TransientHttpError,
    #[serde(rename = "ScopeOriginSameSiteMismatch")]
    ScopeOriginSameSiteMismatch,
    #[serde(rename = "RefreshUrlSameSiteMismatch")]
    RefreshUrlSameSiteMismatch,
    #[serde(rename = "MismatchedSessionId")]
    MismatchedSessionId,
    #[serde(rename = "MissingScope")]
    MissingScope,
    #[serde(rename = "NoCredentials")]
    NoCredentials,
    #[serde(rename = "SubdomainRegistrationWellKnownUnavailable")]
    SubdomainRegistrationWellKnownUnavailable,
    #[serde(rename = "SubdomainRegistrationUnauthorized")]
    SubdomainRegistrationUnauthorized,
    #[serde(rename = "SubdomainRegistrationWellKnownMalformed")]
    SubdomainRegistrationWellKnownMalformed,
    #[serde(rename = "SessionProviderWellKnownUnavailable")]
    SessionProviderWellKnownUnavailable,
    #[serde(rename = "RelyingPartyWellKnownUnavailable")]
    RelyingPartyWellKnownUnavailable,
    #[serde(rename = "FederatedKeyThumbprintMismatch")]
    FederatedKeyThumbprintMismatch,
    #[serde(rename = "InvalidFederatedSessionUrl")]
    InvalidFederatedSessionUrl,
    #[serde(rename = "InvalidFederatedKey")]
    InvalidFederatedKey,
    #[serde(rename = "TooManyRelyingOriginLabels")]
    TooManyRelyingOriginLabels,
    #[serde(rename = "BoundCookieSetForbidden")]
    BoundCookieSetForbidden,
    #[serde(rename = "NetError")]
    NetError,
    #[serde(rename = "ProxyError")]
    ProxyError,
    #[serde(rename = "EmptySessionConfig")]
    EmptySessionConfig,
    #[serde(rename = "InvalidCredentialsConfig")]
    InvalidCredentialsConfig,
    #[serde(rename = "InvalidCredentialsType")]
    InvalidCredentialsType,
    #[serde(rename = "InvalidCredentialsEmptyName")]
    InvalidCredentialsEmptyName,
    #[serde(rename = "InvalidCredentialsCookie")]
    InvalidCredentialsCookie,
    #[serde(rename = "PersistentHttpError")]
    PersistentHttpError,
    #[serde(rename = "RegistrationAttemptedChallenge")]
    RegistrationAttemptedChallenge,
    #[serde(rename = "InvalidScopeOrigin")]
    InvalidScopeOrigin,
    #[serde(rename = "ScopeOriginContainsPath")]
    ScopeOriginContainsPath,
    #[serde(rename = "RefreshInitiatorNotString")]
    RefreshInitiatorNotString,
    #[serde(rename = "RefreshInitiatorInvalidHostPattern")]
    RefreshInitiatorInvalidHostPattern,
    #[serde(rename = "InvalidScopeSpecification")]
    InvalidScopeSpecification,
    #[serde(rename = "MissingScopeSpecificationType")]
    MissingScopeSpecificationType,
    #[serde(rename = "EmptyScopeSpecificationDomain")]
    EmptyScopeSpecificationDomain,
    #[serde(rename = "EmptyScopeSpecificationPath")]
    EmptyScopeSpecificationPath,
    #[serde(rename = "InvalidScopeSpecificationType")]
    InvalidScopeSpecificationType,
    #[serde(rename = "InvalidScopeIncludeSite")]
    InvalidScopeIncludeSite,
    #[serde(rename = "MissingScopeIncludeSite")]
    MissingScopeIncludeSite,
    #[serde(rename = "FederatedNotAuthorizedByProvider")]
    FederatedNotAuthorizedByProvider,
    #[serde(rename = "FederatedNotAuthorizedByRelyingParty")]
    FederatedNotAuthorizedByRelyingParty,
    #[serde(rename = "SessionProviderWellKnownMalformed")]
    SessionProviderWellKnownMalformed,
    #[serde(rename = "SessionProviderWellKnownHasProviderOrigin")]
    SessionProviderWellKnownHasProviderOrigin,
    #[serde(rename = "RelyingPartyWellKnownMalformed")]
    RelyingPartyWellKnownMalformed,
    #[serde(rename = "RelyingPartyWellKnownHasRelyingOrigins")]
    RelyingPartyWellKnownHasRelyingOrigins,
    #[serde(rename = "InvalidFederatedSessionProviderSessionMissing")]
    InvalidFederatedSessionProviderSessionMissing,
    #[serde(rename = "InvalidFederatedSessionWrongProviderOrigin")]
    InvalidFederatedSessionWrongProviderOrigin,
    #[serde(rename = "InvalidCredentialsCookieCreationTime")]
    InvalidCredentialsCookieCreationTime,
    #[serde(rename = "InvalidCredentialsCookieName")]
    InvalidCredentialsCookieName,
    #[serde(rename = "InvalidCredentialsCookieParsing")]
    InvalidCredentialsCookieParsing,
    #[serde(rename = "InvalidCredentialsCookieUnpermittedAttribute")]
    InvalidCredentialsCookieUnpermittedAttribute,
    #[serde(rename = "InvalidCredentialsCookieInvalidDomain")]
    InvalidCredentialsCookieInvalidDomain,
    #[serde(rename = "InvalidCredentialsCookiePrefix")]
    InvalidCredentialsCookiePrefix,
    #[serde(rename = "InvalidScopeRulePath")]
    InvalidScopeRulePath,
    #[serde(rename = "InvalidScopeRuleHostPattern")]
    InvalidScopeRuleHostPattern,
    #[serde(rename = "ScopeRuleOriginScopedHostPatternMismatch")]
    ScopeRuleOriginScopedHostPatternMismatch,
    #[serde(rename = "ScopeRuleSiteScopedHostPatternMismatch")]
    ScopeRuleSiteScopedHostPatternMismatch,
    #[serde(rename = "SigningQuotaExceeded")]
    SigningQuotaExceeded,
    #[serde(rename = "InvalidConfigJson")]
    InvalidConfigJson,
    #[serde(rename = "InvalidFederatedSessionProviderFailedToRestoreKey")]
    InvalidFederatedSessionProviderFailedToRestoreKey,
    #[serde(rename = "FailedToUnwrapKey")]
    FailedToUnwrapKey,
    #[serde(rename = "SessionDeletedDuringRefresh")]
    SessionDeletedDuringRefresh,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub enum RefreshEventDetailsRefreshResult {
    #[serde(rename = "Refreshed")]
    Refreshed,
    #[serde(rename = "InitializedService")]
    InitializedService,
    #[serde(rename = "Unreachable")]
    Unreachable,
    #[serde(rename = "ServerError")]
    ServerError,
    #[serde(rename = "RefreshQuotaExceeded")]
    RefreshQuotaExceeded,
    #[serde(rename = "FatalError")]
    FatalError,
    #[serde(rename = "SigningQuotaExceeded")]
    SigningQuotaExceeded,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub enum TerminationEventDetailsDeletionReason {
    #[serde(rename = "Expired")]
    Expired,
    #[serde(rename = "FailedToRestoreKey")]
    FailedToRestoreKey,
    #[serde(rename = "FailedToUnwrapKey")]
    FailedToUnwrapKey,
    #[serde(rename = "StoragePartitionCleared")]
    StoragePartitionCleared,
    #[serde(rename = "ClearBrowsingData")]
    ClearBrowsingData,
    #[serde(rename = "ServerRequested")]
    ServerRequested,
    #[serde(rename = "InvalidSessionParams")]
    InvalidSessionParams,
    #[serde(rename = "RefreshFatalError")]
    RefreshFatalError,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub enum ChallengeEventDetailsChallengeResult {
    #[serde(rename = "Success")]
    Success,
    #[serde(rename = "NoSessionId")]
    NoSessionId,
    #[serde(rename = "NoSessionMatch")]
    NoSessionMatch,
    #[serde(rename = "CantSetBoundCookie")]
    CantSetBoundCookie,
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
pub struct Headers(pub Option<Json>);
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "camelCase")]
#[doc = "Timing information for the request."]
pub struct ResourceTiming {
    #[serde(default)]
    #[doc = "Timing's requestTime is a baseline in seconds, while the other numbers are ticks in\n milliseconds relatively to this requestTime."]
    pub request_time: JsFloat,
    #[serde(default)]
    #[doc = "Started resolving proxy."]
    pub proxy_start: JsFloat,
    #[serde(default)]
    #[doc = "Finished resolving proxy."]
    pub proxy_end: JsFloat,
    #[serde(default)]
    #[doc = "Started DNS address resolve."]
    pub dns_start: JsFloat,
    #[serde(default)]
    #[doc = "Finished DNS address resolve."]
    pub dns_end: JsFloat,
    #[serde(default)]
    #[doc = "Started connecting to the remote host."]
    pub connect_start: JsFloat,
    #[serde(default)]
    #[doc = "Connected to the remote host."]
    pub connect_end: JsFloat,
    #[serde(default)]
    #[doc = "Started SSL handshake."]
    pub ssl_start: JsFloat,
    #[serde(default)]
    #[doc = "Finished SSL handshake."]
    pub ssl_end: JsFloat,
    #[serde(default)]
    #[doc = "Started running ServiceWorker."]
    pub worker_start: JsFloat,
    #[serde(default)]
    #[doc = "Finished Starting ServiceWorker."]
    pub worker_ready: JsFloat,
    #[serde(default)]
    #[doc = "Started fetch event."]
    pub worker_fetch_start: JsFloat,
    #[serde(default)]
    #[doc = "Settled fetch event respondWith promise."]
    pub worker_respond_with_settled: JsFloat,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[doc = "Started ServiceWorker static routing source evaluation."]
    pub worker_router_evaluation_start: Option<JsFloat>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[doc = "Started cache lookup when the source was evaluated to `cache`."]
    pub worker_cache_lookup_start: Option<JsFloat>,
    #[serde(default)]
    #[doc = "Started sending request."]
    pub send_start: JsFloat,
    #[serde(default)]
    #[doc = "Finished sending request."]
    pub send_end: JsFloat,
    #[serde(default)]
    #[doc = "Time the server started pushing request."]
    pub push_start: JsFloat,
    #[serde(default)]
    #[doc = "Time the server finished pushing request."]
    pub push_end: JsFloat,
    #[serde(default)]
    #[doc = "Started receiving response headers."]
    pub receive_headers_start: JsFloat,
    #[serde(default)]
    #[doc = "Finished receiving response headers."]
    pub receive_headers_end: JsFloat,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "camelCase")]
#[doc = "Post data entry for HTTP request"]
pub struct PostDataEntry {
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bytes: Option<Vec<u8>>,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "camelCase")]
#[doc = "HTTP request data."]
pub struct Request {
    #[serde(default)]
    #[doc = "Request URL (without fragment)."]
    pub url: String,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[doc = "Fragment of the requested URL starting with hash, if present."]
    pub url_fragment: Option<String>,
    #[serde(default)]
    #[doc = "HTTP request method."]
    pub method: String,
    #[doc = "HTTP request headers."]
    pub headers: Headers,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[doc = "HTTP POST request data.\n Use postDataEntries instead."]
    #[deprecated]
    pub post_data: Option<String>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[doc = "True when the request has POST data. Note that postData might still be omitted when this flag is true when the data is too long."]
    pub has_post_data: Option<bool>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[doc = "Request body elements (post data broken into individual entries)."]
    pub post_data_entries: Option<Vec<PostDataEntry>>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[doc = "The mixed content type of the request."]
    pub mixed_content_type: Option<security::MixedContentType>,
    #[doc = "Priority of the resource request at the time request is sent."]
    pub initial_priority: ResourcePriority,
    #[doc = "The referrer policy of the request, as defined in https://www.w3.org/TR/referrer-policy/"]
    pub referrer_policy: RequestReferrerPolicy,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[doc = "Whether is loaded via link preload."]
    pub is_link_preload: Option<bool>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[doc = "Set for requests when the TrustToken API is used. Contains the parameters\n passed by the developer (e.g. via \"fetch\") as understood by the backend."]
    pub trust_token_params: Option<TrustTokenParams>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[doc = "True if this resource request is considered to be the 'same site' as the\n request corresponding to the main frame."]
    pub is_same_site: Option<bool>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[doc = "True when the resource request is ad-related."]
    pub is_ad_related: Option<bool>,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "camelCase")]
#[doc = "Details of a signed certificate timestamp (SCT)."]
pub struct SignedCertificateTimestamp {
    #[serde(default)]
    #[doc = "Validation status."]
    pub status: String,
    #[serde(default)]
    #[doc = "Origin."]
    pub origin: String,
    #[serde(default)]
    #[doc = "Log name / description."]
    pub log_description: String,
    #[serde(default)]
    #[doc = "Log ID."]
    pub log_id: String,
    #[serde(default)]
    #[doc = "Issuance date. Unlike TimeSinceEpoch, this contains the number of\n milliseconds since January 1, 1970, UTC, not the number of seconds."]
    pub timestamp: JsFloat,
    #[serde(default)]
    #[doc = "Hash algorithm."]
    pub hash_algorithm: String,
    #[serde(default)]
    #[doc = "Signature algorithm."]
    pub signature_algorithm: String,
    #[serde(default)]
    #[doc = "Signature data."]
    pub signature_data: String,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "camelCase")]
#[doc = "Security details about a request."]
pub struct SecurityDetails {
    #[serde(default)]
    #[doc = "Protocol name (e.g. \"TLS 1.2\" or \"QUIC\")."]
    pub protocol: String,
    #[serde(default)]
    #[doc = "Key Exchange used by the connection, or the empty string if not applicable."]
    pub key_exchange: String,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[doc = "(EC)DH group used by the connection, if applicable."]
    pub key_exchange_group: Option<String>,
    #[serde(default)]
    #[doc = "Cipher name."]
    pub cipher: String,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[doc = "TLS MAC. Note that AEAD ciphers do not have separate MACs."]
    pub mac: Option<String>,
    #[doc = "Certificate ID value."]
    pub certificate_id: security::CertificateId,
    #[serde(default)]
    #[doc = "Certificate subject name."]
    pub subject_name: String,
    #[serde(default)]
    #[doc = "Subject Alternative Name (SAN) DNS names and IP addresses."]
    pub san_list: Vec<String>,
    #[serde(default)]
    #[doc = "Name of the issuing CA."]
    pub issuer: String,
    #[doc = "Certificate valid from date."]
    pub valid_from: TimeSinceEpoch,
    #[doc = "Certificate valid to (expiration) date"]
    pub valid_to: TimeSinceEpoch,
    #[doc = "List of signed certificate timestamps (SCTs)."]
    pub signed_certificate_timestamp_list: Vec<SignedCertificateTimestamp>,
    #[doc = "Whether the request complied with Certificate Transparency policy"]
    pub certificate_transparency_compliance: CertificateTransparencyCompliance,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[doc = "The signature algorithm used by the server in the TLS server signature,\n represented as a TLS SignatureScheme code point. Omitted if not\n applicable or not known."]
    pub server_signature_algorithm: Option<JsUInt>,
    #[serde(default)]
    #[doc = "Whether the connection used Encrypted ClientHello"]
    pub encrypted_client_hello: bool,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "camelCase")]
pub struct CorsErrorStatus {
    pub cors_error: CorsError,
    #[serde(default)]
    pub failed_parameter: String,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "camelCase")]
#[doc = "Determines what type of Trust Token operation is executed and\n depending on the type, some additional parameters. The values\n are specified in third_party/blink/renderer/core/fetch/trust_token.idl."]
pub struct TrustTokenParams {
    pub operation: TrustTokenOperationType,
    #[doc = "Only set for \"token-redemption\" operation and determine whether\n to request a fresh SRR or use a still valid cached SRR."]
    pub refresh_policy: TrustTokenParamsRefreshPolicy,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[doc = "Origins of issuers from whom to request tokens or redemption\n records."]
    pub issuers: Option<Vec<String>>,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "camelCase")]
pub struct ServiceWorkerRouterInfo {
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[doc = "ID of the rule matched. If there is a matched rule, this field will\n be set, otherwiser no value will be set."]
    pub rule_id_matched: Option<JsUInt>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[doc = "The router source of the matched rule. If there is a matched rule, this\n field will be set, otherwise no value will be set."]
    pub matched_source_type: Option<ServiceWorkerRouterSource>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[doc = "The actual router source used."]
    pub actual_source_type: Option<ServiceWorkerRouterSource>,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "camelCase")]
#[doc = "HTTP response data."]
pub struct Response {
    #[serde(default)]
    #[doc = "Response URL. This URL can be different from CachedResource.url in case of redirect."]
    pub url: String,
    #[serde(default)]
    #[doc = "HTTP response status code."]
    pub status: JsUInt,
    #[serde(default)]
    #[doc = "HTTP response status text."]
    pub status_text: String,
    #[doc = "HTTP response headers."]
    pub headers: Headers,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[doc = "HTTP response headers text. This has been replaced by the headers in Network.responseReceivedExtraInfo."]
    #[deprecated]
    pub headers_text: Option<String>,
    #[serde(default)]
    #[doc = "Resource mimeType as determined by the browser."]
    pub mime_type: String,
    #[serde(default)]
    #[doc = "Resource charset as determined by the browser (if applicable)."]
    pub charset: String,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[doc = "Refined HTTP request headers that were actually transmitted over the network."]
    pub request_headers: Option<Headers>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[doc = "HTTP request headers text. This has been replaced by the headers in Network.requestWillBeSentExtraInfo."]
    #[deprecated]
    pub request_headers_text: Option<String>,
    #[serde(default)]
    #[doc = "Specifies whether physical connection was actually reused for this request."]
    pub connection_reused: bool,
    #[serde(default)]
    #[doc = "Physical connection id that was actually used for this request."]
    pub connection_id: JsFloat,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[doc = "Remote IP address."]
    #[serde(rename = "remoteIPAddress")]
    pub remote_ip_address: Option<String>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[doc = "Remote port."]
    pub remote_port: Option<JsUInt>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[doc = "Specifies that the request was served from the disk cache."]
    pub from_disk_cache: Option<bool>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[doc = "Specifies that the request was served from the ServiceWorker."]
    pub from_service_worker: Option<bool>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[doc = "Specifies that the request was served from the prefetch cache."]
    pub from_prefetch_cache: Option<bool>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[doc = "Specifies that the request was served from the prefetch cache."]
    pub from_early_hints: Option<bool>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[doc = "Information about how ServiceWorker Static Router API was used. If this\n field is set with `matchedSourceType` field, a matching rule is found.\n If this field is set without `matchedSource`, no matching rule is found.\n Otherwise, the API is not used."]
    pub service_worker_router_info: Option<ServiceWorkerRouterInfo>,
    #[serde(default)]
    #[doc = "Total number of bytes received for this request so far."]
    pub encoded_data_length: JsFloat,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[doc = "Timing information for the given request."]
    pub timing: Option<ResourceTiming>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[doc = "Response source of response from ServiceWorker."]
    pub service_worker_response_source: Option<ServiceWorkerResponseSource>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[doc = "The time at which the returned response was generated."]
    pub response_time: Option<TimeSinceEpoch>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[doc = "Cache Storage Cache Name."]
    pub cache_storage_cache_name: Option<String>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[doc = "Protocol used to fetch this request."]
    pub protocol: Option<String>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[doc = "The reason why Chrome uses a specific transport protocol for HTTP semantics."]
    pub alternate_protocol_usage: Option<AlternateProtocolUsage>,
    #[doc = "Security state of the request resource."]
    pub security_state: security::SecurityState,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[doc = "Security details for the request."]
    pub security_details: Option<SecurityDetails>,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "camelCase")]
#[doc = "WebSocket request data."]
pub struct WebSocketRequest {
    #[doc = "HTTP request headers."]
    pub headers: Headers,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "camelCase")]
#[doc = "WebSocket response data."]
pub struct WebSocketResponse {
    #[serde(default)]
    #[doc = "HTTP response status code."]
    pub status: JsUInt,
    #[serde(default)]
    #[doc = "HTTP response status text."]
    pub status_text: String,
    #[doc = "HTTP response headers."]
    pub headers: Headers,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[doc = "HTTP response headers text."]
    pub headers_text: Option<String>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[doc = "HTTP request headers."]
    pub request_headers: Option<Headers>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[doc = "HTTP request headers text."]
    pub request_headers_text: Option<String>,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "camelCase")]
#[doc = "WebSocket message data. This represents an entire WebSocket message, not just a fragmented frame as the name suggests."]
pub struct WebSocketFrame {
    #[serde(default)]
    #[doc = "WebSocket message opcode."]
    pub opcode: JsFloat,
    #[serde(default)]
    #[doc = "WebSocket message mask."]
    pub mask: bool,
    #[serde(default)]
    #[doc = "WebSocket message payload data.\n If the opcode is 1, this is a text message and payloadData is a UTF-8 string.\n If the opcode isn't 1, then payloadData is a base64 encoded string representing binary data."]
    pub payload_data: String,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "camelCase")]
#[doc = "Information about the cached resource."]
pub struct CachedResource {
    #[serde(default)]
    #[doc = "Resource URL. This is the url of the original network request."]
    pub url: String,
    #[doc = "Type of this resource."]
    pub r#type: ResourceType,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[doc = "Cached response data."]
    pub response: Option<Response>,
    #[serde(default)]
    #[doc = "Cached response body size."]
    pub body_size: JsFloat,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "camelCase")]
#[doc = "Information about the request initiator."]
pub struct Initiator {
    #[doc = "Type of this initiator."]
    pub r#type: InitiatorType,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[doc = "Initiator JavaScript stack trace, set for Script only.\n Requires the Debugger domain to be enabled."]
    pub stack: Option<runtime::StackTrace>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[doc = "Initiator URL, set for Parser type or for Script type (when script is importing module) or for SignedExchange type."]
    pub url: Option<String>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[doc = "Initiator line number, set for Parser type or for Script type (when script is importing\n module) (0-based)."]
    pub line_number: Option<JsFloat>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[doc = "Initiator column number, set for Parser type or for Script type (when script is importing\n module) (0-based)."]
    pub column_number: Option<JsFloat>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[doc = "Set if another request triggered this request (e.g. preflight)."]
    pub request_id: Option<RequestId>,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "camelCase")]
#[doc = "cookiePartitionKey object\n The representation of the components of the key that are created by the cookiePartitionKey class contained in net/cookies/cookie_partition_key.h."]
pub struct CookiePartitionKey {
    #[serde(default)]
    #[doc = "The site of the top-level URL the browser was visiting at the start\n of the request to the endpoint that set the cookie."]
    pub top_level_site: String,
    #[serde(default)]
    #[doc = "Indicates if the cookie has any ancestors that are cross-site to the topLevelSite."]
    pub has_cross_site_ancestor: bool,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "camelCase")]
#[doc = "Cookie object"]
pub struct Cookie {
    #[serde(default)]
    #[doc = "Cookie name."]
    pub name: String,
    #[serde(default)]
    #[doc = "Cookie value."]
    pub value: String,
    #[serde(default)]
    #[doc = "Cookie domain."]
    pub domain: String,
    #[serde(default)]
    #[doc = "Cookie path."]
    pub path: String,
    #[serde(default)]
    #[doc = "Cookie expiration date as the number of seconds since the UNIX epoch.\n The value is set to -1 if the expiry date is not set.\n The value can be null for values that cannot be represented in\n JSON (±Inf)."]
    pub expires: JsFloat,
    #[serde(default)]
    #[doc = "Cookie size."]
    pub size: JsUInt,
    #[serde(default)]
    #[doc = "True if cookie is http-only."]
    pub http_only: bool,
    #[serde(default)]
    #[doc = "True if cookie is secure."]
    pub secure: bool,
    #[serde(default)]
    #[doc = "True in case of session cookie."]
    pub session: bool,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[doc = "Cookie SameSite type."]
    pub same_site: Option<CookieSameSite>,
    #[doc = "Cookie Priority"]
    pub priority: CookiePriority,
    #[doc = "Cookie source scheme type."]
    pub source_scheme: CookieSourceScheme,
    #[serde(default)]
    #[doc = "Cookie source port. Valid values are {-1, [1, 65535]}, -1 indicates an unspecified port.\n An unspecified port value allows protocol clients to emulate legacy cookie scope for the port.\n This is a temporary ability and it will be removed in the future."]
    pub source_port: JsUInt,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[doc = "Cookie partition key."]
    pub partition_key: Option<CookiePartitionKey>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[doc = "True if cookie partition key is opaque."]
    pub partition_key_opaque: Option<bool>,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "camelCase")]
#[doc = "A cookie which was not stored from a response with the corresponding reason."]
pub struct BlockedSetCookieWithReason {
    #[doc = "The reason(s) this cookie was blocked."]
    pub blocked_reasons: Vec<SetCookieBlockedReason>,
    #[serde(default)]
    #[doc = "The string representing this individual cookie as it would appear in the header.\n This is not the entire \"cookie\" or \"set-cookie\" header which could have multiple cookies."]
    pub cookie_line: String,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[doc = "The cookie object which represents the cookie which was not stored. It is optional because\n sometimes complete cookie information is not available, such as in the case of parsing\n errors."]
    pub cookie: Option<Cookie>,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "camelCase")]
#[doc = "A cookie should have been blocked by 3PCD but is exempted and stored from a response with the\n corresponding reason. A cookie could only have at most one exemption reason."]
pub struct ExemptedSetCookieWithReason {
    #[doc = "The reason the cookie was exempted."]
    pub exemption_reason: CookieExemptionReason,
    #[serde(default)]
    #[doc = "The string representing this individual cookie as it would appear in the header."]
    pub cookie_line: String,
    #[doc = "The cookie object representing the cookie."]
    pub cookie: Cookie,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "camelCase")]
#[doc = "A cookie associated with the request which may or may not be sent with it.\n Includes the cookies itself and reasons for blocking or exemption."]
pub struct AssociatedCookie {
    #[doc = "The cookie object representing the cookie which was not sent."]
    pub cookie: Cookie,
    #[doc = "The reason(s) the cookie was blocked. If empty means the cookie is included."]
    pub blocked_reasons: Vec<CookieBlockedReason>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[doc = "The reason the cookie should have been blocked by 3PCD but is exempted. A cookie could\n only have at most one exemption reason."]
    pub exemption_reason: Option<CookieExemptionReason>,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "camelCase")]
#[doc = "Cookie parameter object"]
pub struct CookieParam {
    #[serde(default)]
    #[doc = "Cookie name."]
    pub name: String,
    #[serde(default)]
    #[doc = "Cookie value."]
    pub value: String,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[doc = "The request-URI to associate with the setting of the cookie. This value can affect the\n default domain, path, source port, and source scheme values of the created cookie."]
    pub url: Option<String>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[doc = "Cookie domain."]
    pub domain: Option<String>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[doc = "Cookie path."]
    pub path: Option<String>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[doc = "True if cookie is secure."]
    pub secure: Option<bool>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[doc = "True if cookie is http-only."]
    pub http_only: Option<bool>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[doc = "Cookie SameSite type."]
    pub same_site: Option<CookieSameSite>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[doc = "Cookie expiration date, session cookie if not set"]
    pub expires: Option<TimeSinceEpoch>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[doc = "Cookie Priority."]
    pub priority: Option<CookiePriority>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[doc = "Cookie source scheme type."]
    pub source_scheme: Option<CookieSourceScheme>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[doc = "Cookie source port. Valid values are {-1, [1, 65535]}, -1 indicates an unspecified port.\n An unspecified port value allows protocol clients to emulate legacy cookie scope for the port.\n This is a temporary ability and it will be removed in the future."]
    pub source_port: Option<JsUInt>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[doc = "Cookie partition key. If not set, the cookie will be set as not partitioned."]
    pub partition_key: Option<CookiePartitionKey>,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "camelCase")]
#[doc = "Authorization challenge for HTTP status code 401 or 407."]
pub struct AuthChallenge {
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[doc = "Source of the authentication challenge."]
    pub source: Option<AuthChallengeSource>,
    #[serde(default)]
    #[doc = "Origin of the challenger."]
    pub origin: String,
    #[serde(default)]
    #[doc = "The authentication scheme used, such as basic or digest"]
    pub scheme: String,
    #[serde(default)]
    #[doc = "The realm of the challenge. May be empty."]
    pub realm: String,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "camelCase")]
#[doc = "Response to an AuthChallenge."]
pub struct AuthChallengeResponse {
    #[doc = "The decision on what to do in response to the authorization challenge.  Default means\n deferring to the default behavior of the net stack, which will likely either the Cancel\n authentication or display a popup dialog box."]
    pub response: AuthChallengeResponseResponse,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[doc = "The username to provide, possibly empty. Should only be set if response is\n ProvideCredentials."]
    pub username: Option<String>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[doc = "The password to provide, possibly empty. Should only be set if response is\n ProvideCredentials."]
    pub password: Option<String>,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "camelCase")]
#[doc = "Request pattern for interception."]
pub struct RequestPattern {
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[doc = "Wildcards (`'*'` -> zero or more, `'?'` -> exactly one) are allowed. Escape character is\n backslash. Omitting is equivalent to `\"*\"`."]
    pub url_pattern: Option<String>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[doc = "If set, only requests for matching resource types will be intercepted."]
    pub resource_type: Option<ResourceType>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[doc = "Stage at which to begin intercepting requests. Default is Request."]
    pub interception_stage: Option<InterceptionStage>,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "camelCase")]
#[doc = "Information about a signed exchange signature.\n https://wicg.github.io/webpackage/draft-yasskin-httpbis-origin-signed-exchanges-impl.html#rfc.section.3.1"]
pub struct SignedExchangeSignature {
    #[serde(default)]
    #[doc = "Signed exchange signature label."]
    pub label: String,
    #[serde(default)]
    #[doc = "The hex string of signed exchange signature."]
    pub signature: String,
    #[serde(default)]
    #[doc = "Signed exchange signature integrity."]
    pub integrity: String,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[doc = "Signed exchange signature cert Url."]
    pub cert_url: Option<String>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[doc = "The hex string of signed exchange signature cert sha256."]
    pub cert_sha_256: Option<String>,
    #[serde(default)]
    #[doc = "Signed exchange signature validity Url."]
    pub validity_url: String,
    #[serde(default)]
    #[doc = "Signed exchange signature date."]
    pub date: JsUInt,
    #[serde(default)]
    #[doc = "Signed exchange signature expires."]
    pub expires: JsUInt,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[doc = "The encoded certificates."]
    pub certificates: Option<Vec<String>>,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "camelCase")]
#[doc = "Information about a signed exchange header.\n https://wicg.github.io/webpackage/draft-yasskin-httpbis-origin-signed-exchanges-impl.html#cbor-representation"]
pub struct SignedExchangeHeader {
    #[serde(default)]
    #[doc = "Signed exchange request URL."]
    pub request_url: String,
    #[serde(default)]
    #[doc = "Signed exchange response code."]
    pub response_code: JsUInt,
    #[doc = "Signed exchange response headers."]
    pub response_headers: Headers,
    #[doc = "Signed exchange response signature."]
    pub signatures: Vec<SignedExchangeSignature>,
    #[serde(default)]
    #[doc = "Signed exchange header integrity hash in the form of `sha256-<base64-hash-value>`."]
    pub header_integrity: String,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "camelCase")]
#[doc = "Information about a signed exchange response."]
pub struct SignedExchangeError {
    #[serde(default)]
    #[doc = "Error message."]
    pub message: String,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[doc = "The index of the signature which caused the error."]
    pub signature_index: Option<JsUInt>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[doc = "The field which caused the error."]
    pub error_field: Option<SignedExchangeErrorField>,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "camelCase")]
#[doc = "Information about a signed exchange response."]
pub struct SignedExchangeInfo {
    #[doc = "The outer response of signed HTTP exchange which was received from network."]
    pub outer_response: Response,
    #[serde(default)]
    #[doc = "Whether network response for the signed exchange was accompanied by\n extra headers."]
    pub has_extra_info: bool,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[doc = "Information about the signed exchange header."]
    pub header: Option<SignedExchangeHeader>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[doc = "Security details for the signed exchange header."]
    pub security_details: Option<SecurityDetails>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[doc = "Errors occurred while handling the signed exchange."]
    pub errors: Option<Vec<SignedExchangeError>>,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "camelCase")]
pub struct NetworkConditions {
    #[serde(default)]
    #[doc = "Only matching requests will be affected by these conditions. Patterns use the URLPattern constructor string\n syntax (https://urlpattern.spec.whatwg.org/) and must be absolute. If the pattern is empty, all requests are\n matched (including p2p connections)."]
    pub url_pattern: String,
    #[serde(default)]
    #[doc = "Minimum latency from request sent to response headers received (ms)."]
    pub latency: JsFloat,
    #[serde(default)]
    #[doc = "Maximal aggregated download throughput (bytes/sec). -1 disables download throttling."]
    pub download_throughput: JsFloat,
    #[serde(default)]
    #[doc = "Maximal aggregated upload throughput (bytes/sec).  -1 disables upload throttling."]
    pub upload_throughput: JsFloat,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[doc = "Connection type if known."]
    pub connection_type: Option<ConnectionType>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[doc = "WebRTC packet loss (percent, 0-100). 0 disables packet loss emulation, 100 drops all the packets."]
    pub packet_loss: Option<JsFloat>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[doc = "WebRTC packet queue length (packet). 0 removes any queue length limitations."]
    pub packet_queue_length: Option<JsUInt>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[doc = "WebRTC packetReordering feature."]
    pub packet_reordering: Option<bool>,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "camelCase")]
pub struct BlockPattern {
    #[serde(default)]
    #[doc = "URL pattern to match. Patterns use the URLPattern constructor string syntax\n (https://urlpattern.spec.whatwg.org/) and must be absolute. Example: `*://*:*/*.css`."]
    pub url_pattern: String,
    #[serde(default)]
    #[doc = "Whether or not to block the pattern. If false, a matching request will not be blocked even if it matches a later\n `BlockPattern`."]
    pub block: bool,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "camelCase")]
pub struct DirectTcpSocketOptions {
    #[serde(default)]
    #[doc = "TCP_NODELAY option"]
    pub no_delay: bool,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[doc = "Expected to be unsigned integer."]
    pub keep_alive_delay: Option<JsFloat>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[doc = "Expected to be unsigned integer."]
    pub send_buffer_size: Option<JsFloat>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[doc = "Expected to be unsigned integer."]
    pub receive_buffer_size: Option<JsFloat>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dns_query_type: Option<DirectSocketDnsQueryType>,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "camelCase")]
pub struct DirectUdpSocketOptions {
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub remote_addr: Option<String>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[doc = "Unsigned int 16."]
    pub remote_port: Option<JsUInt>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub local_addr: Option<String>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[doc = "Unsigned int 16."]
    pub local_port: Option<JsUInt>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dns_query_type: Option<DirectSocketDnsQueryType>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[doc = "Expected to be unsigned integer."]
    pub send_buffer_size: Option<JsFloat>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[doc = "Expected to be unsigned integer."]
    pub receive_buffer_size: Option<JsFloat>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub multicast_loopback: Option<bool>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[doc = "Unsigned int 8."]
    pub multicast_time_to_live: Option<JsUInt>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub multicast_allow_address_sharing: Option<bool>,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "camelCase")]
pub struct DirectUdpMessage {
    pub data: Vec<u8>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[doc = "Null for connected mode."]
    pub remote_addr: Option<String>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[doc = "Null for connected mode.\n Expected to be unsigned integer."]
    pub remote_port: Option<JsUInt>,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "camelCase")]
pub struct ConnectTiming {
    #[serde(default)]
    #[doc = "Timing's requestTime is a baseline in seconds, while the other numbers are ticks in\n milliseconds relatively to this requestTime. Matches ResourceTiming's requestTime for\n the same request (but not for redirected requests)."]
    pub request_time: JsFloat,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "camelCase")]
pub struct ClientSecurityState {
    #[serde(default)]
    pub initiator_is_secure_context: bool,
    #[serde(rename = "initiatorIPAddressSpace")]
    pub initiator_ip_address_space: IpAddressSpace,
    pub local_network_access_request_policy: LocalNetworkAccessRequestPolicy,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "camelCase")]
pub struct CrossOriginOpenerPolicyStatus {
    pub value: CrossOriginOpenerPolicyValue,
    pub report_only_value: CrossOriginOpenerPolicyValue,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub reporting_endpoint: Option<String>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub report_only_reporting_endpoint: Option<String>,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "camelCase")]
pub struct CrossOriginEmbedderPolicyStatus {
    pub value: CrossOriginEmbedderPolicyValue,
    pub report_only_value: CrossOriginEmbedderPolicyValue,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub reporting_endpoint: Option<String>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub report_only_reporting_endpoint: Option<String>,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "camelCase")]
pub struct ContentSecurityPolicyStatus {
    #[serde(default)]
    pub effective_directives: String,
    #[serde(default)]
    pub is_enforced: bool,
    pub source: ContentSecurityPolicySource,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "camelCase")]
pub struct SecurityIsolationStatus {
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub coop: Option<CrossOriginOpenerPolicyStatus>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub coep: Option<CrossOriginEmbedderPolicyStatus>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub csp: Option<Vec<ContentSecurityPolicyStatus>>,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "camelCase")]
#[doc = "An object representing a report generated by the Reporting API."]
pub struct ReportingApiReport {
    pub id: ReportId,
    #[serde(default)]
    #[doc = "The URL of the document that triggered the report."]
    pub initiator_url: String,
    #[serde(default)]
    #[doc = "The name of the endpoint group that should be used to deliver the report."]
    pub destination: String,
    #[serde(default)]
    #[doc = "The type of the report (specifies the set of data that is contained in the report body)."]
    pub r#type: String,
    #[doc = "When the report was generated."]
    pub timestamp: network::TimeSinceEpoch,
    #[serde(default)]
    #[doc = "How many uploads deep the related request was."]
    pub depth: JsUInt,
    #[serde(default)]
    #[doc = "The number of delivery attempts made so far, not including an active attempt."]
    pub completed_attempts: JsUInt,
    #[serde(default)]
    pub body: Json,
    pub status: ReportStatus,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "camelCase")]
pub struct ReportingApiEndpoint {
    #[serde(default)]
    #[doc = "The URL of the endpoint to which reports may be delivered."]
    pub url: String,
    #[serde(default)]
    #[doc = "Name of the endpoint group."]
    pub group_name: String,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "camelCase")]
#[doc = "Unique identifier for a device bound session."]
pub struct DeviceBoundSessionKey {
    #[serde(default)]
    #[doc = "The site the session is set up for."]
    pub site: String,
    #[serde(default)]
    #[doc = "The id of the session."]
    pub id: String,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "camelCase")]
#[doc = "How a device bound session was used during a request."]
pub struct DeviceBoundSessionWithUsage {
    #[doc = "The key for the session."]
    pub session_key: DeviceBoundSessionKey,
    #[doc = "How the session was used (or not used)."]
    pub usage: DeviceBoundSessionWithUsageUsage,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "camelCase")]
#[doc = "A device bound session's cookie craving."]
pub struct DeviceBoundSessionCookieCraving {
    #[serde(default)]
    #[doc = "The name of the craving."]
    pub name: String,
    #[serde(default)]
    #[doc = "The domain of the craving."]
    pub domain: String,
    #[serde(default)]
    #[doc = "The path of the craving."]
    pub path: String,
    #[serde(default)]
    #[doc = "The `Secure` attribute of the craving attributes."]
    pub secure: bool,
    #[serde(default)]
    #[doc = "The `HttpOnly` attribute of the craving attributes."]
    pub http_only: bool,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[doc = "The `SameSite` attribute of the craving attributes."]
    pub same_site: Option<CookieSameSite>,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "camelCase")]
#[doc = "A device bound session's inclusion URL rule."]
pub struct DeviceBoundSessionUrlRule {
    #[doc = "See comments on `net::device_bound_sessions::SessionInclusionRules::UrlRule::rule_type`."]
    pub rule_type: DeviceBoundSessionUrlRuleRuleType,
    #[serde(default)]
    #[doc = "See comments on `net::device_bound_sessions::SessionInclusionRules::UrlRule::host_pattern`."]
    pub host_pattern: String,
    #[serde(default)]
    #[doc = "See comments on `net::device_bound_sessions::SessionInclusionRules::UrlRule::path_prefix`."]
    pub path_prefix: String,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "camelCase")]
#[doc = "A device bound session's inclusion rules."]
pub struct DeviceBoundSessionInclusionRules {
    #[serde(default)]
    #[doc = "See comments on `net::device_bound_sessions::SessionInclusionRules::origin_`."]
    pub origin: String,
    #[serde(default)]
    #[doc = "Whether the whole site is included. See comments on\n `net::device_bound_sessions::SessionInclusionRules::include_site_` for more\n details; this boolean is true if that value is populated."]
    pub include_site: bool,
    #[doc = "See comments on `net::device_bound_sessions::SessionInclusionRules::url_rules_`."]
    pub url_rules: Vec<DeviceBoundSessionUrlRule>,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "camelCase")]
#[doc = "A device bound session."]
pub struct DeviceBoundSession {
    #[doc = "The site and session ID of the session."]
    pub key: DeviceBoundSessionKey,
    #[serde(default)]
    #[doc = "See comments on `net::device_bound_sessions::Session::refresh_url_`."]
    pub refresh_url: String,
    #[doc = "See comments on `net::device_bound_sessions::Session::inclusion_rules_`."]
    pub inclusion_rules: DeviceBoundSessionInclusionRules,
    #[doc = "See comments on `net::device_bound_sessions::Session::cookie_cravings_`."]
    pub cookie_cravings: Vec<DeviceBoundSessionCookieCraving>,
    #[doc = "See comments on `net::device_bound_sessions::Session::expiry_date_`."]
    pub expiry_date: network::TimeSinceEpoch,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[doc = "See comments on `net::device_bound_sessions::Session::cached_challenge__`."]
    pub cached_challenge: Option<String>,
    #[serde(default)]
    #[doc = "See comments on `net::device_bound_sessions::Session::allowed_refresh_initiators_`."]
    pub allowed_refresh_initiators: Vec<String>,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "camelCase")]
#[doc = "Session event details specific to creation."]
pub struct CreationEventDetails {
    #[doc = "The result of the fetch attempt."]
    pub fetch_result: DeviceBoundSessionFetchResult,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[doc = "The session if there was a newly created session. This is populated for\n all successful creation events."]
    pub new_session: Option<DeviceBoundSession>,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "camelCase")]
#[doc = "Session event details specific to refresh."]
pub struct RefreshEventDetails {
    #[doc = "The result of a refresh."]
    pub refresh_result: RefreshEventDetailsRefreshResult,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[doc = "If there was a fetch attempt, the result of that."]
    pub fetch_result: Option<DeviceBoundSessionFetchResult>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[doc = "The session display if there was a newly created session. This is populated\n for any refresh event that modifies the session config."]
    pub new_session: Option<DeviceBoundSession>,
    #[serde(default)]
    #[doc = "See comments on `net::device_bound_sessions::RefreshEventResult::was_fully_proactive_refresh`."]
    pub was_fully_proactive_refresh: bool,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "camelCase")]
#[doc = "Session event details specific to termination."]
pub struct TerminationEventDetails {
    #[doc = "The reason for a session being deleted."]
    pub deletion_reason: TerminationEventDetailsDeletionReason,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "camelCase")]
#[doc = "Session event details specific to challenges."]
pub struct ChallengeEventDetails {
    #[doc = "The result of a challenge."]
    pub challenge_result: ChallengeEventDetailsChallengeResult,
    #[serde(default)]
    #[doc = "The challenge set."]
    pub challenge: String,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "camelCase")]
#[doc = "An object providing the result of a network resource load."]
pub struct LoadNetworkResourcePageResult {
    #[serde(default)]
    pub success: bool,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[doc = "Optional values used for error reporting."]
    pub net_error: Option<JsFloat>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub net_error_name: Option<String>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub http_status_code: Option<JsFloat>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[doc = "If successful, one of the following two fields holds the result."]
    pub stream: Option<io::StreamHandle>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[doc = "Response headers."]
    pub headers: Option<network::Headers>,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "camelCase")]
#[doc = "An options object that may be extended later to better support CORS,\n CORB and streaming."]
pub struct LoadNetworkResourceOptions {
    #[serde(default)]
    pub disable_cache: bool,
    #[serde(default)]
    pub include_credentials: bool,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "camelCase")]
#[doc = "Sets a list of content encodings that will be accepted. Empty list means no encoding is accepted."]
pub struct SetAcceptedEncodings {
    #[doc = "List of accepted content encodings."]
    pub encodings: Vec<ContentEncoding>,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct ClearAcceptedEncodingsOverride(pub Option<Json>);
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct CanClearBrowserCache(pub Option<Json>);
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct CanClearBrowserCookies(pub Option<Json>);
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct CanEmulateNetworkConditions(pub Option<Json>);
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct ClearBrowserCache(pub Option<Json>);
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct ClearBrowserCookies(pub Option<Json>);
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "camelCase")]
#[doc = "Response to Network.requestIntercepted which either modifies the request to continue with any\n modifications, or blocks it, or completes it with the provided response bytes. If a network\n fetch occurs as a result which encounters a redirect an additional Network.requestIntercepted\n event will be sent with the same InterceptionId.\n Deprecated, use Fetch.continueRequest, Fetch.fulfillRequest and Fetch.failRequest instead."]
#[deprecated]
pub struct ContinueInterceptedRequest {
    pub interception_id: InterceptionId,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[doc = "If set this causes the request to fail with the given reason. Passing `Aborted` for requests\n marked with `isNavigationRequest` also cancels the navigation. Must not be set in response\n to an authChallenge."]
    pub error_reason: Option<ErrorReason>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[doc = "If set the requests completes using with the provided base64 encoded raw response, including\n HTTP status line and headers etc... Must not be set in response to an authChallenge."]
    pub raw_response: Option<Vec<u8>>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[doc = "If set the request url will be modified in a way that's not observable by page. Must not be\n set in response to an authChallenge."]
    pub url: Option<String>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[doc = "If set this allows the request method to be overridden. Must not be set in response to an\n authChallenge."]
    pub method: Option<String>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[doc = "If set this allows postData to be set. Must not be set in response to an authChallenge."]
    pub post_data: Option<String>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[doc = "If set this allows the request headers to be changed. Must not be set in response to an\n authChallenge."]
    pub headers: Option<Headers>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[doc = "Response to a requestIntercepted with an authChallenge. Must not be set otherwise."]
    pub auth_challenge_response: Option<AuthChallengeResponse>,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "camelCase")]
#[doc = "Deletes browser cookies with matching name and url or domain/path/partitionKey pair."]
pub struct DeleteCookies {
    #[serde(default)]
    #[doc = "Name of the cookies to remove."]
    pub name: String,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[doc = "If specified, deletes all the cookies with the given name where domain and path match\n provided URL."]
    pub url: Option<String>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[doc = "If specified, deletes only cookies with the exact domain."]
    pub domain: Option<String>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[doc = "If specified, deletes only cookies with the exact path."]
    pub path: Option<String>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[doc = "If specified, deletes only cookies with the the given name and partitionKey where\n all partition key attributes match the cookie partition key attribute."]
    pub partition_key: Option<CookiePartitionKey>,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct Disable(pub Option<Json>);
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "camelCase")]
#[doc = "Activates emulation of network conditions. This command is deprecated in favor of the emulateNetworkConditionsByRule\n and overrideNetworkState commands, which can be used together to the same effect."]
#[deprecated]
pub struct EmulateNetworkConditions {
    #[serde(default)]
    #[doc = "True to emulate internet disconnection."]
    pub offline: bool,
    #[serde(default)]
    #[doc = "Minimum latency from request sent to response headers received (ms)."]
    pub latency: JsFloat,
    #[serde(default)]
    #[doc = "Maximal aggregated download throughput (bytes/sec). -1 disables download throttling."]
    pub download_throughput: JsFloat,
    #[serde(default)]
    #[doc = "Maximal aggregated upload throughput (bytes/sec).  -1 disables upload throttling."]
    pub upload_throughput: JsFloat,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[doc = "Connection type if known."]
    pub connection_type: Option<ConnectionType>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[doc = "WebRTC packet loss (percent, 0-100). 0 disables packet loss emulation, 100 drops all the packets."]
    pub packet_loss: Option<JsFloat>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[doc = "WebRTC packet queue length (packet). 0 removes any queue length limitations."]
    pub packet_queue_length: Option<JsUInt>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[doc = "WebRTC packetReordering feature."]
    pub packet_reordering: Option<bool>,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "camelCase")]
#[doc = "Activates emulation of network conditions for individual requests using URL match patterns. Unlike the deprecated\n Network.emulateNetworkConditions this method does not affect `navigator` state. Use Network.overrideNetworkState to\n explicitly modify `navigator` behavior."]
pub struct EmulateNetworkConditionsByRule {
    #[serde(default)]
    #[doc = "True to emulate internet disconnection."]
    pub offline: bool,
    #[doc = "Configure conditions for matching requests. If multiple entries match a request, the first entry wins.  Global\n conditions can be configured by leaving the urlPattern for the conditions empty. These global conditions are\n also applied for throttling of p2p connections."]
    pub matched_network_conditions: Vec<NetworkConditions>,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "camelCase")]
#[doc = "Override the state of navigator.onLine and navigator.connection."]
pub struct OverrideNetworkState {
    #[serde(default)]
    #[doc = "True to emulate internet disconnection."]
    pub offline: bool,
    #[serde(default)]
    #[doc = "Minimum latency from request sent to response headers received (ms)."]
    pub latency: JsFloat,
    #[serde(default)]
    #[doc = "Maximal aggregated download throughput (bytes/sec). -1 disables download throttling."]
    pub download_throughput: JsFloat,
    #[serde(default)]
    #[doc = "Maximal aggregated upload throughput (bytes/sec).  -1 disables upload throttling."]
    pub upload_throughput: JsFloat,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[doc = "Connection type if known."]
    pub connection_type: Option<ConnectionType>,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "camelCase")]
#[doc = "Enables network tracking, network events will now be delivered to the client."]
pub struct Enable {
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[doc = "Buffer size in bytes to use when preserving network payloads (XHRs, etc)."]
    pub max_total_buffer_size: Option<JsUInt>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[doc = "Per-resource buffer size in bytes to use when preserving network payloads (XHRs, etc)."]
    pub max_resource_buffer_size: Option<JsUInt>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[doc = "Longest post body size (in bytes) that would be included in requestWillBeSent notification"]
    pub max_post_data_size: Option<JsUInt>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[doc = "Whether DirectSocket chunk send/receive events should be reported."]
    pub report_direct_socket_traffic: Option<bool>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[doc = "Enable storing response bodies outside of renderer, so that these survive\n a cross-process navigation. Requires maxTotalBufferSize to be set.\n Currently defaults to false. This field is being deprecated in favor of the dedicated\n configureDurableMessages command, due to the possibility of deadlocks when awaiting\n Network.enable before issuing Runtime.runIfWaitingForDebugger."]
    pub enable_durable_messages: Option<bool>,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "camelCase")]
#[doc = "Configures storing response bodies outside of renderer, so that these survive\n a cross-process navigation.\n If maxTotalBufferSize is not set, durable messages are disabled."]
pub struct ConfigureDurableMessages {
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[doc = "Buffer size in bytes to use when preserving network payloads (XHRs, etc)."]
    pub max_total_buffer_size: Option<JsUInt>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[doc = "Per-resource buffer size in bytes to use when preserving network payloads (XHRs, etc)."]
    pub max_resource_buffer_size: Option<JsUInt>,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct GetAllCookies(pub Option<Json>);
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "camelCase")]
#[doc = "Returns the DER-encoded certificate."]
pub struct GetCertificate {
    #[serde(default)]
    #[doc = "Origin to get certificate for."]
    pub origin: String,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "camelCase")]
#[doc = "Returns all browser cookies for the current URL. Depending on the backend support, will return\n detailed cookie information in the `cookies` field."]
pub struct GetCookies {
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[doc = "The list of URLs for which applicable cookies will be fetched.\n If not specified, it's assumed to be set to the list containing\n the URLs of the page and all of its subframes."]
    pub urls: Option<Vec<String>>,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "camelCase")]
#[doc = "Returns content served for the given request."]
pub struct GetResponseBody {
    #[doc = "Identifier of the network request to get content for."]
    pub request_id: RequestId,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "camelCase")]
#[doc = "Returns post data sent with the request. Returns an error when no data was sent with the request."]
pub struct GetRequestPostData {
    #[doc = "Identifier of the network request to get content for."]
    pub request_id: RequestId,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "camelCase")]
#[doc = "Returns content served for the given currently intercepted request."]
pub struct GetResponseBodyForInterception {
    #[doc = "Identifier for the intercepted request to get body for."]
    pub interception_id: InterceptionId,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "camelCase")]
#[doc = "Returns a handle to the stream representing the response body. Note that after this command,\n the intercepted request can't be continued as is -- you either need to cancel it or to provide\n the response body. The stream only supports sequential read, IO.read will fail if the position\n is specified."]
pub struct TakeResponseBodyForInterceptionAsStream {
    pub interception_id: InterceptionId,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "camelCase")]
#[doc = "This method sends a new XMLHttpRequest which is identical to the original one. The following\n parameters should be identical: method, url, async, request body, extra headers, withCredentials\n attribute, user, password."]
pub struct ReplayXHR {
    #[doc = "Identifier of XHR to replay."]
    pub request_id: RequestId,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "camelCase")]
#[doc = "Searches for given string in response content."]
pub struct SearchInResponseBody {
    #[doc = "Identifier of the network response to search."]
    pub request_id: RequestId,
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
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "camelCase")]
#[doc = "Blocks URLs from loading."]
pub struct SetBlockedURLs {
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[doc = "Patterns to match in the order in which they are given. These patterns\n also take precedence over any wildcard patterns defined in `urls`."]
    pub url_patterns: Option<Vec<BlockPattern>>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[doc = "URL patterns to block. Wildcards ('*') are allowed."]
    #[deprecated]
    pub urls: Option<Vec<String>>,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "camelCase")]
#[doc = "Toggles ignoring of service worker for each request."]
pub struct SetBypassServiceWorker {
    #[serde(default)]
    #[doc = "Bypass service worker and load from network."]
    pub bypass: bool,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "camelCase")]
#[doc = "Toggles ignoring cache for each request. If `true`, cache will not be used."]
pub struct SetCacheDisabled {
    #[serde(default)]
    #[doc = "Cache disabled state."]
    pub cache_disabled: bool,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "camelCase")]
#[doc = "Sets a cookie with the given cookie data; may overwrite equivalent cookies if they exist."]
pub struct SetCookie {
    #[serde(default)]
    #[doc = "Cookie name."]
    pub name: String,
    #[serde(default)]
    #[doc = "Cookie value."]
    pub value: String,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[doc = "The request-URI to associate with the setting of the cookie. This value can affect the\n default domain, path, source port, and source scheme values of the created cookie."]
    pub url: Option<String>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[doc = "Cookie domain."]
    pub domain: Option<String>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[doc = "Cookie path."]
    pub path: Option<String>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[doc = "True if cookie is secure."]
    pub secure: Option<bool>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[doc = "True if cookie is http-only."]
    pub http_only: Option<bool>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[doc = "Cookie SameSite type."]
    pub same_site: Option<CookieSameSite>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[doc = "Cookie expiration date, session cookie if not set"]
    pub expires: Option<TimeSinceEpoch>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[doc = "Cookie Priority type."]
    pub priority: Option<CookiePriority>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[doc = "Cookie source scheme type."]
    pub source_scheme: Option<CookieSourceScheme>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[doc = "Cookie source port. Valid values are {-1, [1, 65535]}, -1 indicates an unspecified port.\n An unspecified port value allows protocol clients to emulate legacy cookie scope for the port.\n This is a temporary ability and it will be removed in the future."]
    pub source_port: Option<JsUInt>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[doc = "Cookie partition key. If not set, the cookie will be set as not partitioned."]
    pub partition_key: Option<CookiePartitionKey>,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "camelCase")]
#[doc = "Sets given cookies."]
pub struct SetCookies {
    #[doc = "Cookies to be set."]
    pub cookies: Vec<CookieParam>,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "camelCase")]
#[doc = "Specifies whether to always send extra HTTP headers with the requests from this page."]
pub struct SetExtraHTTPHeaders {
    #[doc = "Map with extra HTTP headers."]
    pub headers: Headers,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "camelCase")]
#[doc = "Specifies whether to attach a page script stack id in requests"]
pub struct SetAttachDebugStack {
    #[serde(default)]
    #[doc = "Whether to attach a page script stack for debugging purpose."]
    pub enabled: bool,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "camelCase")]
#[doc = "Sets the requests to intercept that match the provided patterns and optionally resource types.\n Deprecated, please use Fetch.enable instead."]
#[deprecated]
pub struct SetRequestInterception {
    #[doc = "Requests matching any of these patterns will be forwarded and wait for the corresponding\n continueInterceptedRequest call."]
    pub patterns: Vec<RequestPattern>,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "camelCase")]
#[doc = "Allows overriding user agent with the given string."]
pub struct SetUserAgentOverride {
    #[serde(default)]
    #[doc = "User agent to use."]
    pub user_agent: String,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[doc = "Browser language to emulate."]
    pub accept_language: Option<String>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[doc = "The platform navigator.platform should return."]
    pub platform: Option<String>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[doc = "To be sent in Sec-CH-UA-* headers and returned in navigator.userAgentData"]
    pub user_agent_metadata: Option<emulation::UserAgentMetadata>,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "camelCase")]
#[doc = "Enables streaming of the response for the given requestId.\n If enabled, the dataReceived event contains the data that was received during streaming."]
pub struct StreamResourceContent {
    #[doc = "Identifier of the request to stream."]
    pub request_id: RequestId,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "camelCase")]
#[doc = "Returns information about the COEP/COOP isolation status."]
pub struct GetSecurityIsolationStatus {
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[doc = "If no frameId is provided, the status of the target is provided."]
    pub frame_id: Option<page::FrameId>,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "camelCase")]
#[doc = "Enables tracking for the Reporting API, events generated by the Reporting API will now be delivered to the client.\n Enabling triggers 'reportingApiReportAdded' for all existing reports."]
pub struct EnableReportingApi {
    #[serde(default)]
    #[doc = "Whether to enable or disable events for the Reporting API"]
    pub enable: bool,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "camelCase")]
#[doc = "Sets up tracking device bound sessions and fetching of initial set of sessions."]
pub struct EnableDeviceBoundSessions {
    #[serde(default)]
    #[doc = "Whether to enable or disable events."]
    pub enable: bool,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "camelCase")]
#[doc = "Fetches the schemeful site for a specific origin."]
pub struct FetchSchemefulSite {
    #[serde(default)]
    #[doc = "The URL origin."]
    pub origin: String,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "camelCase")]
#[doc = "Fetches the resource and returns the content."]
pub struct LoadNetworkResource {
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[doc = "Frame id to get the resource for. Mandatory for frame targets, and\n should be omitted for worker targets."]
    pub frame_id: Option<page::FrameId>,
    #[serde(default)]
    #[doc = "URL of the resource to get content for."]
    pub url: String,
    #[doc = "Options for the request."]
    pub options: LoadNetworkResourceOptions,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "camelCase")]
#[doc = "Sets Controls for third-party cookie access\n Page reload is required before the new cookie behavior will be observed"]
pub struct SetCookieControls {
    #[serde(default)]
    #[doc = "Whether 3pc restriction is enabled."]
    pub enable_third_party_cookie_restriction: bool,
    #[serde(default)]
    #[doc = "Whether 3pc grace period exception should be enabled; false by default."]
    pub disable_third_party_cookie_metadata: bool,
    #[serde(default)]
    #[doc = "Whether 3pc heuristics exceptions should be enabled; false by default."]
    pub disable_third_party_cookie_heuristics: bool,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[doc = "Sets a list of content encodings that will be accepted. Empty list means no encoding is accepted."]
pub struct SetAcceptedEncodingsReturnObject(pub Option<Json>);
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[doc = "Clears accepted encodings set by setAcceptedEncodings"]
pub struct ClearAcceptedEncodingsOverrideReturnObject(pub Option<Json>);
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[serde(rename_all = "camelCase")]
#[doc = "Tells whether clearing browser cache is supported."]
#[deprecated]
pub struct CanClearBrowserCacheReturnObject {
    #[serde(default)]
    #[doc = "True if browser cache can be cleared."]
    pub result: bool,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[serde(rename_all = "camelCase")]
#[doc = "Tells whether clearing browser cookies is supported."]
#[deprecated]
pub struct CanClearBrowserCookiesReturnObject {
    #[serde(default)]
    #[doc = "True if browser cookies can be cleared."]
    pub result: bool,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[serde(rename_all = "camelCase")]
#[doc = "Tells whether emulation of network conditions is supported."]
#[deprecated]
pub struct CanEmulateNetworkConditionsReturnObject {
    #[serde(default)]
    #[doc = "True if emulation of network conditions is supported."]
    pub result: bool,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[doc = "Clears browser cache."]
pub struct ClearBrowserCacheReturnObject(pub Option<Json>);
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[doc = "Clears browser cookies."]
pub struct ClearBrowserCookiesReturnObject(pub Option<Json>);
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[doc = "Response to Network.requestIntercepted which either modifies the request to continue with any\n modifications, or blocks it, or completes it with the provided response bytes. If a network\n fetch occurs as a result which encounters a redirect an additional Network.requestIntercepted\n event will be sent with the same InterceptionId.\n Deprecated, use Fetch.continueRequest, Fetch.fulfillRequest and Fetch.failRequest instead."]
#[deprecated]
pub struct ContinueInterceptedRequestReturnObject(pub Option<Json>);
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[doc = "Deletes browser cookies with matching name and url or domain/path/partitionKey pair."]
pub struct DeleteCookiesReturnObject(pub Option<Json>);
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[doc = "Disables network tracking, prevents network events from being sent to the client."]
pub struct DisableReturnObject(pub Option<Json>);
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[doc = "Activates emulation of network conditions. This command is deprecated in favor of the emulateNetworkConditionsByRule\n and overrideNetworkState commands, which can be used together to the same effect."]
#[deprecated]
pub struct EmulateNetworkConditionsReturnObject(pub Option<Json>);
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[serde(rename_all = "camelCase")]
#[doc = "Activates emulation of network conditions for individual requests using URL match patterns. Unlike the deprecated\n Network.emulateNetworkConditions this method does not affect `navigator` state. Use Network.overrideNetworkState to\n explicitly modify `navigator` behavior."]
pub struct EmulateNetworkConditionsByRuleReturnObject {
    #[doc = "An id for each entry in matchedNetworkConditions. The id will be included in the requestWillBeSentExtraInfo for\n requests affected by a rule."]
    pub rule_ids: Vec<String>,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[doc = "Override the state of navigator.onLine and navigator.connection."]
pub struct OverrideNetworkStateReturnObject(pub Option<Json>);
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[doc = "Enables network tracking, network events will now be delivered to the client."]
pub struct EnableReturnObject(pub Option<Json>);
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[doc = "Configures storing response bodies outside of renderer, so that these survive\n a cross-process navigation.\n If maxTotalBufferSize is not set, durable messages are disabled."]
pub struct ConfigureDurableMessagesReturnObject(pub Option<Json>);
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[serde(rename_all = "camelCase")]
#[doc = "Returns all browser cookies. Depending on the backend support, will return detailed cookie\n information in the `cookies` field.\n Deprecated. Use Storage.getCookies instead."]
#[deprecated]
pub struct GetAllCookiesReturnObject {
    #[doc = "Array of cookie objects."]
    pub cookies: Vec<Cookie>,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[serde(rename_all = "camelCase")]
#[doc = "Returns the DER-encoded certificate."]
pub struct GetCertificateReturnObject {
    pub table_names: Vec<String>,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[serde(rename_all = "camelCase")]
#[doc = "Returns all browser cookies for the current URL. Depending on the backend support, will return\n detailed cookie information in the `cookies` field."]
pub struct GetCookiesReturnObject {
    #[doc = "Array of cookie objects."]
    pub cookies: Vec<Cookie>,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[serde(rename_all = "camelCase")]
#[doc = "Returns content served for the given request."]
pub struct GetResponseBodyReturnObject {
    #[serde(default)]
    #[doc = "Response body."]
    pub body: String,
    #[serde(default)]
    #[doc = "True, if content was sent as base64."]
    pub base_64_encoded: bool,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[serde(rename_all = "camelCase")]
#[doc = "Returns post data sent with the request. Returns an error when no data was sent with the request."]
pub struct GetRequestPostDataReturnObject {
    #[serde(default)]
    #[doc = "Request body string, omitting files from multipart requests"]
    pub post_data: String,
    #[serde(default)]
    #[doc = "True, if content was sent as base64."]
    pub base_64_encoded: bool,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[serde(rename_all = "camelCase")]
#[doc = "Returns content served for the given currently intercepted request."]
pub struct GetResponseBodyForInterceptionReturnObject {
    #[serde(default)]
    #[doc = "Response body."]
    pub body: String,
    #[serde(default)]
    #[doc = "True, if content was sent as base64."]
    pub base_64_encoded: bool,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[serde(rename_all = "camelCase")]
#[doc = "Returns a handle to the stream representing the response body. Note that after this command,\n the intercepted request can't be continued as is -- you either need to cancel it or to provide\n the response body. The stream only supports sequential read, IO.read will fail if the position\n is specified."]
pub struct TakeResponseBodyForInterceptionAsStreamReturnObject {
    pub stream: io::StreamHandle,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[doc = "This method sends a new XMLHttpRequest which is identical to the original one. The following\n parameters should be identical: method, url, async, request body, extra headers, withCredentials\n attribute, user, password."]
pub struct ReplayXHRReturnObject(pub Option<Json>);
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[serde(rename_all = "camelCase")]
#[doc = "Searches for given string in response content."]
pub struct SearchInResponseBodyReturnObject {
    #[doc = "List of search matches."]
    pub result: debugger::SearchMatch,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[doc = "Blocks URLs from loading."]
pub struct SetBlockedURLsReturnObject(pub Option<Json>);
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[doc = "Toggles ignoring of service worker for each request."]
pub struct SetBypassServiceWorkerReturnObject(pub Option<Json>);
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[doc = "Toggles ignoring cache for each request. If `true`, cache will not be used."]
pub struct SetCacheDisabledReturnObject(pub Option<Json>);
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[serde(rename_all = "camelCase")]
#[doc = "Sets a cookie with the given cookie data; may overwrite equivalent cookies if they exist."]
pub struct SetCookieReturnObject {
    #[serde(default)]
    #[doc = "Always set to true. If an error occurs, the response indicates protocol error."]
    #[deprecated]
    pub success: bool,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[doc = "Sets given cookies."]
pub struct SetCookiesReturnObject(pub Option<Json>);
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[doc = "Specifies whether to always send extra HTTP headers with the requests from this page."]
pub struct SetExtraHTTPHeadersReturnObject(pub Option<Json>);
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[doc = "Specifies whether to attach a page script stack id in requests"]
pub struct SetAttachDebugStackReturnObject(pub Option<Json>);
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[doc = "Sets the requests to intercept that match the provided patterns and optionally resource types.\n Deprecated, please use Fetch.enable instead."]
#[deprecated]
pub struct SetRequestInterceptionReturnObject(pub Option<Json>);
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[doc = "Allows overriding user agent with the given string."]
pub struct SetUserAgentOverrideReturnObject(pub Option<Json>);
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[serde(rename_all = "camelCase")]
#[doc = "Enables streaming of the response for the given requestId.\n If enabled, the dataReceived event contains the data that was received during streaming."]
pub struct StreamResourceContentReturnObject {
    #[doc = "Data that has been buffered until streaming is enabled."]
    pub buffered_data: Vec<u8>,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[serde(rename_all = "camelCase")]
#[doc = "Returns information about the COEP/COOP isolation status."]
pub struct GetSecurityIsolationStatusReturnObject {
    pub status: SecurityIsolationStatus,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[doc = "Enables tracking for the Reporting API, events generated by the Reporting API will now be delivered to the client.\n Enabling triggers 'reportingApiReportAdded' for all existing reports."]
pub struct EnableReportingApiReturnObject(pub Option<Json>);
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[doc = "Sets up tracking device bound sessions and fetching of initial set of sessions."]
pub struct EnableDeviceBoundSessionsReturnObject(pub Option<Json>);
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[serde(rename_all = "camelCase")]
#[doc = "Fetches the schemeful site for a specific origin."]
pub struct FetchSchemefulSiteReturnObject {
    #[serde(default)]
    #[doc = "The corresponding schemeful site."]
    pub schemeful_site: String,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[serde(rename_all = "camelCase")]
#[doc = "Fetches the resource and returns the content."]
pub struct LoadNetworkResourceReturnObject {
    pub resource: LoadNetworkResourcePageResult,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[doc = "Sets Controls for third-party cookie access\n Page reload is required before the new cookie behavior will be observed"]
pub struct SetCookieControlsReturnObject(pub Option<Json>);
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
impl Method for EmulateNetworkConditionsByRule {
    const NAME: &'static str = "Network.emulateNetworkConditionsByRule";
    type ReturnObject = EmulateNetworkConditionsByRuleReturnObject;
}
impl Method for OverrideNetworkState {
    const NAME: &'static str = "Network.overrideNetworkState";
    type ReturnObject = OverrideNetworkStateReturnObject;
}
impl Method for Enable {
    const NAME: &'static str = "Network.enable";
    type ReturnObject = EnableReturnObject;
}
impl Method for ConfigureDurableMessages {
    const NAME: &'static str = "Network.configureDurableMessages";
    type ReturnObject = ConfigureDurableMessagesReturnObject;
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
impl Method for EnableDeviceBoundSessions {
    const NAME: &'static str = "Network.enableDeviceBoundSessions";
    type ReturnObject = EnableDeviceBoundSessionsReturnObject;
}
impl Method for FetchSchemefulSite {
    const NAME: &'static str = "Network.fetchSchemefulSite";
    type ReturnObject = FetchSchemefulSiteReturnObject;
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
    use derive_builder::Builder;
    #[allow(unused_imports)]
    use serde::{Deserialize, Serialize};
    #[allow(unused_imports)]
    use serde_json::Value as Json;
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
    pub struct DataReceivedEvent {
        pub params: DataReceivedEventParams,
    }
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
    pub struct DataReceivedEventParams {
        #[doc = "Request identifier."]
        pub request_id: super::RequestId,
        #[doc = "Timestamp."]
        pub timestamp: super::MonotonicTime,
        #[serde(default)]
        #[doc = "Data chunk length."]
        pub data_length: JsUInt,
        #[serde(default)]
        #[doc = "Actual bytes received (might be less than dataLength for compressed encodings)."]
        pub encoded_data_length: JsUInt,
        #[builder(default)]
        #[serde(skip_serializing_if = "Option::is_none")]
        #[serde(default)]
        #[doc = "Data that was received."]
        pub data: Option<u8>,
    }
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
    pub struct EventSourceMessageReceivedEvent {
        pub params: EventSourceMessageReceivedEventParams,
    }
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
    pub struct EventSourceMessageReceivedEventParams {
        #[doc = "Request identifier."]
        pub request_id: super::RequestId,
        #[doc = "Timestamp."]
        pub timestamp: super::MonotonicTime,
        #[serde(default)]
        #[doc = "Message type."]
        pub event_name: String,
        #[serde(default)]
        #[doc = "Message identifier."]
        pub event_id: String,
        #[serde(default)]
        #[doc = "Message content."]
        pub data: String,
    }
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
    pub struct LoadingFailedEvent {
        pub params: LoadingFailedEventParams,
    }
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
    pub struct LoadingFailedEventParams {
        #[doc = "Request identifier."]
        pub request_id: super::RequestId,
        #[doc = "Timestamp."]
        pub timestamp: super::MonotonicTime,
        #[doc = "Resource type."]
        pub r#type: super::ResourceType,
        #[serde(default)]
        #[doc = "Error message. List of network errors: https://cs.chromium.org/chromium/src/net/base/net_error_list.h"]
        pub error_text: String,
        #[builder(default)]
        #[serde(skip_serializing_if = "Option::is_none")]
        #[serde(default)]
        #[doc = "True if loading was canceled."]
        pub canceled: Option<bool>,
        #[builder(default)]
        #[serde(skip_serializing_if = "Option::is_none")]
        #[doc = "The reason why loading was blocked, if any."]
        pub blocked_reason: Option<super::BlockedReason>,
        #[builder(default)]
        #[serde(skip_serializing_if = "Option::is_none")]
        #[doc = "The reason why loading was blocked by CORS, if any."]
        pub cors_error_status: Option<super::CorsErrorStatus>,
    }
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
    pub struct LoadingFinishedEvent {
        pub params: LoadingFinishedEventParams,
    }
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
    pub struct LoadingFinishedEventParams {
        #[doc = "Request identifier."]
        pub request_id: super::RequestId,
        #[doc = "Timestamp."]
        pub timestamp: super::MonotonicTime,
        #[serde(default)]
        #[doc = "Total number of bytes received for this request."]
        pub encoded_data_length: JsFloat,
    }
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
    pub struct RequestInterceptedEvent {
        pub params: RequestInterceptedEventParams,
    }
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
    pub struct RequestInterceptedEventParams {
        #[doc = "Each request the page makes will have a unique id, however if any redirects are encountered\n while processing that fetch, they will be reported with the same id as the original fetch.\n Likewise if HTTP authentication is needed then the same fetch id will be used."]
        pub interception_id: super::InterceptionId,
        pub request: super::Request,
        #[doc = "The id of the frame that initiated the request."]
        pub frame_id: super::super::page::FrameId,
        #[doc = "How the requested resource will be used."]
        pub resource_type: super::ResourceType,
        #[serde(default)]
        #[doc = "Whether this is a navigation request, which can abort the navigation completely."]
        pub is_navigation_request: bool,
        #[builder(default)]
        #[serde(skip_serializing_if = "Option::is_none")]
        #[serde(default)]
        #[doc = "Set if the request is a navigation that will result in a download.\n Only present after response is received from the server (i.e. HeadersReceived stage)."]
        pub is_download: Option<bool>,
        #[builder(default)]
        #[serde(skip_serializing_if = "Option::is_none")]
        #[serde(default)]
        #[doc = "Redirect location, only sent if a redirect was intercepted."]
        pub redirect_url: Option<String>,
        #[builder(default)]
        #[serde(skip_serializing_if = "Option::is_none")]
        #[doc = "Details of the Authorization Challenge encountered. If this is set then\n continueInterceptedRequest must contain an authChallengeResponse."]
        pub auth_challenge: Option<super::AuthChallenge>,
        #[builder(default)]
        #[serde(skip_serializing_if = "Option::is_none")]
        #[doc = "Response error if intercepted at response stage or if redirect occurred while intercepting\n request."]
        pub response_error_reason: Option<super::ErrorReason>,
        #[builder(default)]
        #[serde(skip_serializing_if = "Option::is_none")]
        #[serde(default)]
        #[doc = "Response code if intercepted at response stage or if redirect occurred while intercepting\n request or auth retry occurred."]
        pub response_status_code: Option<JsUInt>,
        #[builder(default)]
        #[serde(skip_serializing_if = "Option::is_none")]
        #[doc = "Response headers if intercepted at the response stage or if redirect occurred while\n intercepting request or auth retry occurred."]
        pub response_headers: Option<super::Headers>,
        #[builder(default)]
        #[serde(skip_serializing_if = "Option::is_none")]
        #[doc = "If the intercepted request had a corresponding requestWillBeSent event fired for it, then\n this requestId will be the same as the requestId present in the requestWillBeSent event."]
        pub request_id: Option<super::RequestId>,
    }
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
    pub struct RequestServedFromCacheEvent {
        pub params: RequestServedFromCacheEventParams,
    }
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
    pub struct RequestServedFromCacheEventParams {
        #[doc = "Request identifier."]
        pub request_id: super::RequestId,
    }
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
    pub struct RequestWillBeSentEvent {
        pub params: RequestWillBeSentEventParams,
    }
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
    pub struct RequestWillBeSentEventParams {
        #[doc = "Request identifier."]
        pub request_id: super::RequestId,
        #[doc = "Loader identifier. Empty string if the request is fetched from worker."]
        pub loader_id: super::LoaderId,
        #[serde(default)]
        #[doc = "URL of the document this request is loaded for."]
        #[serde(rename = "documentURL")]
        pub document_url: String,
        #[doc = "Request data."]
        pub request: super::Request,
        #[doc = "Timestamp."]
        pub timestamp: super::MonotonicTime,
        #[doc = "Timestamp."]
        pub wall_time: super::TimeSinceEpoch,
        #[doc = "Request initiator."]
        pub initiator: super::Initiator,
        #[serde(default)]
        #[doc = "In the case that redirectResponse is populated, this flag indicates whether\n requestWillBeSentExtraInfo and responseReceivedExtraInfo events will be or were emitted\n for the request which was just redirected."]
        pub redirect_has_extra_info: bool,
        #[builder(default)]
        #[serde(skip_serializing_if = "Option::is_none")]
        #[doc = "Redirect response data."]
        pub redirect_response: Option<super::Response>,
        #[builder(default)]
        #[serde(skip_serializing_if = "Option::is_none")]
        #[doc = "Type of this resource."]
        pub r#type: Option<super::ResourceType>,
        #[builder(default)]
        #[serde(skip_serializing_if = "Option::is_none")]
        #[doc = "Frame identifier."]
        pub frame_id: Option<super::super::page::FrameId>,
        #[builder(default)]
        #[serde(skip_serializing_if = "Option::is_none")]
        #[serde(default)]
        #[doc = "Whether the request is initiated by a user gesture. Defaults to false."]
        pub has_user_gesture: Option<bool>,
        #[builder(default)]
        #[serde(skip_serializing_if = "Option::is_none")]
        #[doc = "The render blocking behavior of the request."]
        pub render_blocking_behavior: Option<super::RenderBlockingBehavior>,
    }
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
    pub struct ResourceChangedPriorityEvent {
        pub params: ResourceChangedPriorityEventParams,
    }
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
    pub struct ResourceChangedPriorityEventParams {
        #[doc = "Request identifier."]
        pub request_id: super::RequestId,
        #[doc = "New priority"]
        pub new_priority: super::ResourcePriority,
        #[doc = "Timestamp."]
        pub timestamp: super::MonotonicTime,
    }
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
    pub struct SignedExchangeReceivedEvent {
        pub params: SignedExchangeReceivedEventParams,
    }
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
    pub struct SignedExchangeReceivedEventParams {
        #[doc = "Request identifier."]
        pub request_id: super::RequestId,
        #[doc = "Information about the signed exchange response."]
        pub info: super::SignedExchangeInfo,
    }
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
    pub struct ResponseReceivedEvent {
        pub params: ResponseReceivedEventParams,
    }
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
    pub struct ResponseReceivedEventParams {
        #[doc = "Request identifier."]
        pub request_id: super::RequestId,
        #[doc = "Loader identifier. Empty string if the request is fetched from worker."]
        pub loader_id: super::LoaderId,
        #[doc = "Timestamp."]
        pub timestamp: super::MonotonicTime,
        #[doc = "Resource type."]
        pub r#type: super::ResourceType,
        #[doc = "Response data."]
        pub response: super::Response,
        #[serde(default)]
        #[doc = "Indicates whether requestWillBeSentExtraInfo and responseReceivedExtraInfo events will be\n or were emitted for this request."]
        pub has_extra_info: bool,
        #[builder(default)]
        #[serde(skip_serializing_if = "Option::is_none")]
        #[doc = "Frame identifier."]
        pub frame_id: Option<super::super::page::FrameId>,
    }
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
    pub struct WebSocketClosedEvent {
        pub params: WebSocketClosedEventParams,
    }
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
    pub struct WebSocketClosedEventParams {
        #[doc = "Request identifier."]
        pub request_id: super::RequestId,
        #[doc = "Timestamp."]
        pub timestamp: super::MonotonicTime,
    }
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
    pub struct WebSocketCreatedEvent {
        pub params: WebSocketCreatedEventParams,
    }
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
    pub struct WebSocketCreatedEventParams {
        #[doc = "Request identifier."]
        pub request_id: super::RequestId,
        #[serde(default)]
        #[doc = "WebSocket request URL."]
        pub url: String,
        #[builder(default)]
        #[serde(skip_serializing_if = "Option::is_none")]
        #[doc = "Request initiator."]
        pub initiator: Option<super::Initiator>,
    }
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
    pub struct WebSocketFrameErrorEvent {
        pub params: WebSocketFrameErrorEventParams,
    }
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
    pub struct WebSocketFrameErrorEventParams {
        #[doc = "Request identifier."]
        pub request_id: super::RequestId,
        #[doc = "Timestamp."]
        pub timestamp: super::MonotonicTime,
        #[serde(default)]
        #[doc = "WebSocket error message."]
        pub error_message: String,
    }
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
    pub struct WebSocketFrameReceivedEvent {
        pub params: WebSocketFrameReceivedEventParams,
    }
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
    pub struct WebSocketFrameReceivedEventParams {
        #[doc = "Request identifier."]
        pub request_id: super::RequestId,
        #[doc = "Timestamp."]
        pub timestamp: super::MonotonicTime,
        #[doc = "WebSocket response data."]
        pub response: super::WebSocketFrame,
    }
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
    pub struct WebSocketFrameSentEvent {
        pub params: WebSocketFrameSentEventParams,
    }
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
    pub struct WebSocketFrameSentEventParams {
        #[doc = "Request identifier."]
        pub request_id: super::RequestId,
        #[doc = "Timestamp."]
        pub timestamp: super::MonotonicTime,
        #[doc = "WebSocket response data."]
        pub response: super::WebSocketFrame,
    }
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
    pub struct WebSocketHandshakeResponseReceivedEvent {
        pub params: WebSocketHandshakeResponseReceivedEventParams,
    }
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
    pub struct WebSocketHandshakeResponseReceivedEventParams {
        #[doc = "Request identifier."]
        pub request_id: super::RequestId,
        #[doc = "Timestamp."]
        pub timestamp: super::MonotonicTime,
        #[doc = "WebSocket response data."]
        pub response: super::WebSocketResponse,
    }
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
    pub struct WebSocketWillSendHandshakeRequestEvent {
        pub params: WebSocketWillSendHandshakeRequestEventParams,
    }
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
    pub struct WebSocketWillSendHandshakeRequestEventParams {
        #[doc = "Request identifier."]
        pub request_id: super::RequestId,
        #[doc = "Timestamp."]
        pub timestamp: super::MonotonicTime,
        #[doc = "UTC Timestamp."]
        pub wall_time: super::TimeSinceEpoch,
        #[doc = "WebSocket request data."]
        pub request: super::WebSocketRequest,
    }
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
    pub struct WebTransportCreatedEvent {
        pub params: WebTransportCreatedEventParams,
    }
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
    pub struct WebTransportCreatedEventParams {
        #[doc = "WebTransport identifier."]
        pub transport_id: super::RequestId,
        #[serde(default)]
        #[doc = "WebTransport request URL."]
        pub url: String,
        #[doc = "Timestamp."]
        pub timestamp: super::MonotonicTime,
        #[builder(default)]
        #[serde(skip_serializing_if = "Option::is_none")]
        #[doc = "Request initiator."]
        pub initiator: Option<super::Initiator>,
    }
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
    pub struct WebTransportConnectionEstablishedEvent {
        pub params: WebTransportConnectionEstablishedEventParams,
    }
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
    pub struct WebTransportConnectionEstablishedEventParams {
        #[doc = "WebTransport identifier."]
        pub transport_id: super::RequestId,
        #[doc = "Timestamp."]
        pub timestamp: super::MonotonicTime,
    }
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
    pub struct WebTransportClosedEvent {
        pub params: WebTransportClosedEventParams,
    }
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
    pub struct WebTransportClosedEventParams {
        #[doc = "WebTransport identifier."]
        pub transport_id: super::RequestId,
        #[doc = "Timestamp."]
        pub timestamp: super::MonotonicTime,
    }
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
    pub struct DirectTCPSocketCreatedEvent {
        pub params: DirectTCPSocketCreatedEventParams,
    }
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
    pub struct DirectTCPSocketCreatedEventParams {
        pub identifier: super::RequestId,
        #[serde(default)]
        pub remote_addr: String,
        #[serde(default)]
        #[doc = "Unsigned int 16."]
        pub remote_port: JsUInt,
        pub options: super::DirectTcpSocketOptions,
        pub timestamp: super::MonotonicTime,
        #[builder(default)]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub initiator: Option<super::Initiator>,
    }
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
    pub struct DirectTCPSocketOpenedEvent {
        pub params: DirectTCPSocketOpenedEventParams,
    }
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
    pub struct DirectTCPSocketOpenedEventParams {
        pub identifier: super::RequestId,
        #[serde(default)]
        pub remote_addr: String,
        #[serde(default)]
        #[doc = "Expected to be unsigned integer."]
        pub remote_port: JsUInt,
        pub timestamp: super::MonotonicTime,
        #[builder(default)]
        #[serde(skip_serializing_if = "Option::is_none")]
        #[serde(default)]
        pub local_addr: Option<String>,
        #[builder(default)]
        #[serde(skip_serializing_if = "Option::is_none")]
        #[serde(default)]
        #[doc = "Expected to be unsigned integer."]
        pub local_port: Option<JsUInt>,
    }
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
    pub struct DirectTCPSocketAbortedEvent {
        pub params: DirectTCPSocketAbortedEventParams,
    }
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
    pub struct DirectTCPSocketAbortedEventParams {
        pub identifier: super::RequestId,
        #[serde(default)]
        pub error_message: String,
        pub timestamp: super::MonotonicTime,
    }
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
    pub struct DirectTCPSocketClosedEvent {
        pub params: DirectTCPSocketClosedEventParams,
    }
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
    pub struct DirectTCPSocketClosedEventParams {
        pub identifier: super::RequestId,
        pub timestamp: super::MonotonicTime,
    }
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
    pub struct DirectTCPSocketChunkSentEvent {
        pub params: DirectTCPSocketChunkSentEventParams,
    }
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
    pub struct DirectTCPSocketChunkSentEventParams {
        pub identifier: super::RequestId,
        #[serde(default)]
        pub data: u8,
        pub timestamp: super::MonotonicTime,
    }
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
    pub struct DirectTCPSocketChunkReceivedEvent {
        pub params: DirectTCPSocketChunkReceivedEventParams,
    }
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
    pub struct DirectTCPSocketChunkReceivedEventParams {
        pub identifier: super::RequestId,
        #[serde(default)]
        pub data: u8,
        pub timestamp: super::MonotonicTime,
    }
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
    pub struct DirectUDPSocketJoinedMulticastGroupEvent {
        pub params: DirectUDPSocketJoinedMulticastGroupEventParams,
    }
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
    pub struct DirectUDPSocketJoinedMulticastGroupEventParams {
        pub identifier: super::RequestId,
        #[serde(default)]
        #[serde(rename = "IPAddress")]
        pub ip_address: String,
    }
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
    pub struct DirectUDPSocketLeftMulticastGroupEvent {
        pub params: DirectUDPSocketLeftMulticastGroupEventParams,
    }
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
    pub struct DirectUDPSocketLeftMulticastGroupEventParams {
        pub identifier: super::RequestId,
        #[serde(default)]
        #[serde(rename = "IPAddress")]
        pub ip_address: String,
    }
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
    pub struct DirectUDPSocketCreatedEvent {
        pub params: DirectUDPSocketCreatedEventParams,
    }
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
    pub struct DirectUDPSocketCreatedEventParams {
        pub identifier: super::RequestId,
        pub options: super::DirectUdpSocketOptions,
        pub timestamp: super::MonotonicTime,
        #[builder(default)]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub initiator: Option<super::Initiator>,
    }
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
    pub struct DirectUDPSocketOpenedEvent {
        pub params: DirectUDPSocketOpenedEventParams,
    }
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
    pub struct DirectUDPSocketOpenedEventParams {
        pub identifier: super::RequestId,
        #[serde(default)]
        pub local_addr: String,
        #[serde(default)]
        #[doc = "Expected to be unsigned integer."]
        pub local_port: JsUInt,
        pub timestamp: super::MonotonicTime,
        #[builder(default)]
        #[serde(skip_serializing_if = "Option::is_none")]
        #[serde(default)]
        pub remote_addr: Option<String>,
        #[builder(default)]
        #[serde(skip_serializing_if = "Option::is_none")]
        #[serde(default)]
        #[doc = "Expected to be unsigned integer."]
        pub remote_port: Option<JsUInt>,
    }
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
    pub struct DirectUDPSocketAbortedEvent {
        pub params: DirectUDPSocketAbortedEventParams,
    }
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
    pub struct DirectUDPSocketAbortedEventParams {
        pub identifier: super::RequestId,
        #[serde(default)]
        pub error_message: String,
        pub timestamp: super::MonotonicTime,
    }
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
    pub struct DirectUDPSocketClosedEvent {
        pub params: DirectUDPSocketClosedEventParams,
    }
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
    pub struct DirectUDPSocketClosedEventParams {
        pub identifier: super::RequestId,
        pub timestamp: super::MonotonicTime,
    }
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
    pub struct DirectUDPSocketChunkSentEvent {
        pub params: DirectUDPSocketChunkSentEventParams,
    }
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
    pub struct DirectUDPSocketChunkSentEventParams {
        pub identifier: super::RequestId,
        pub message: super::DirectUdpMessage,
        pub timestamp: super::MonotonicTime,
    }
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
    pub struct DirectUDPSocketChunkReceivedEvent {
        pub params: DirectUDPSocketChunkReceivedEventParams,
    }
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
    pub struct DirectUDPSocketChunkReceivedEventParams {
        pub identifier: super::RequestId,
        pub message: super::DirectUdpMessage,
        pub timestamp: super::MonotonicTime,
    }
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
    pub struct RequestWillBeSentExtraInfoEvent {
        pub params: RequestWillBeSentExtraInfoEventParams,
    }
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
    pub struct RequestWillBeSentExtraInfoEventParams {
        #[doc = "Request identifier. Used to match this information to an existing requestWillBeSent event."]
        pub request_id: super::RequestId,
        #[doc = "A list of cookies potentially associated to the requested URL. This includes both cookies sent with\n the request and the ones not sent; the latter are distinguished by having blockedReasons field set."]
        pub associated_cookies: Vec<super::AssociatedCookie>,
        #[doc = "Raw request headers as they will be sent over the wire."]
        pub headers: super::Headers,
        #[doc = "Connection timing information for the request."]
        pub connect_timing: super::ConnectTiming,
        #[builder(default)]
        #[serde(skip_serializing_if = "Option::is_none")]
        #[doc = "How the request site's device bound sessions were used during this request."]
        pub device_bound_session_usages: Option<Vec<super::DeviceBoundSessionWithUsage>>,
        #[builder(default)]
        #[serde(skip_serializing_if = "Option::is_none")]
        #[doc = "The client security state set for the request."]
        pub client_security_state: Option<super::ClientSecurityState>,
        #[builder(default)]
        #[serde(skip_serializing_if = "Option::is_none")]
        #[serde(default)]
        #[doc = "Whether the site has partitioned cookies stored in a partition different than the current one."]
        pub site_has_cookie_in_other_partition: Option<bool>,
        #[builder(default)]
        #[serde(skip_serializing_if = "Option::is_none")]
        #[serde(default)]
        #[doc = "The network conditions id if this request was affected by network conditions configured via\n emulateNetworkConditionsByRule."]
        pub applied_network_conditions_id: Option<String>,
    }
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
    pub struct ResponseReceivedExtraInfoEvent {
        pub params: ResponseReceivedExtraInfoEventParams,
    }
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
    pub struct ResponseReceivedExtraInfoEventParams {
        #[doc = "Request identifier. Used to match this information to another responseReceived event."]
        pub request_id: super::RequestId,
        #[doc = "A list of cookies which were not stored from the response along with the corresponding\n reasons for blocking. The cookies here may not be valid due to syntax errors, which\n are represented by the invalid cookie line string instead of a proper cookie."]
        pub blocked_cookies: Vec<super::BlockedSetCookieWithReason>,
        #[doc = "Raw response headers as they were received over the wire.\n Duplicate headers in the response are represented as a single key with their values\n concatentated using `\\n` as the separator.\n See also `headersText` that contains verbatim text for HTTP/1.*."]
        pub headers: super::Headers,
        #[doc = "The IP address space of the resource. The address space can only be determined once the transport\n established the connection, so we can't send it in `requestWillBeSentExtraInfo`."]
        #[serde(rename = "resourceIPAddressSpace")]
        pub resource_ip_address_space: super::IpAddressSpace,
        #[serde(default)]
        #[doc = "The status code of the response. This is useful in cases the request failed and no responseReceived\n event is triggered, which is the case for, e.g., CORS errors. This is also the correct status code\n for cached requests, where the status in responseReceived is a 200 and this will be 304."]
        pub status_code: JsUInt,
        #[builder(default)]
        #[serde(skip_serializing_if = "Option::is_none")]
        #[serde(default)]
        #[doc = "Raw response header text as it was received over the wire. The raw text may not always be\n available, such as in the case of HTTP/2 or QUIC."]
        pub headers_text: Option<String>,
        #[builder(default)]
        #[serde(skip_serializing_if = "Option::is_none")]
        #[doc = "The cookie partition key that will be used to store partitioned cookies set in this response.\n Only sent when partitioned cookies are enabled."]
        pub cookie_partition_key: Option<super::CookiePartitionKey>,
        #[builder(default)]
        #[serde(skip_serializing_if = "Option::is_none")]
        #[serde(default)]
        #[doc = "True if partitioned cookies are enabled, but the partition key is not serializable to string."]
        pub cookie_partition_key_opaque: Option<bool>,
        #[builder(default)]
        #[serde(skip_serializing_if = "Option::is_none")]
        #[doc = "A list of cookies which should have been blocked by 3PCD but are exempted and stored from\n the response with the corresponding reason."]
        pub exempted_cookies: Option<Vec<super::ExemptedSetCookieWithReason>>,
    }
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
    pub struct ResponseReceivedEarlyHintsEvent {
        pub params: ResponseReceivedEarlyHintsEventParams,
    }
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
    pub struct ResponseReceivedEarlyHintsEventParams {
        #[doc = "Request identifier. Used to match this information to another responseReceived event."]
        pub request_id: super::RequestId,
        #[doc = "Raw response headers as they were received over the wire.\n Duplicate headers in the response are represented as a single key with their values\n concatentated using `\\n` as the separator.\n See also `headersText` that contains verbatim text for HTTP/1.*."]
        pub headers: super::Headers,
    }
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
    pub struct TrustTokenOperationDoneEvent {
        pub params: TrustTokenOperationDoneEventParams,
    }
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
    pub struct TrustTokenOperationDoneEventParams {
        #[doc = "Detailed success or error status of the operation.\n 'AlreadyExists' also signifies a successful operation, as the result\n of the operation already exists und thus, the operation was abort\n preemptively (e.g. a cache hit)."]
        pub status: super::TrustTokenOperationDoneStatusOption,
        pub r#type: super::TrustTokenOperationType,
        pub request_id: super::RequestId,
        #[builder(default)]
        #[serde(skip_serializing_if = "Option::is_none")]
        #[serde(default)]
        #[doc = "Top level origin. The context in which the operation was attempted."]
        pub top_level_origin: Option<String>,
        #[builder(default)]
        #[serde(skip_serializing_if = "Option::is_none")]
        #[serde(default)]
        #[doc = "Origin of the issuer in case of a \"Issuance\" or \"Redemption\" operation."]
        pub issuer_origin: Option<String>,
        #[builder(default)]
        #[serde(skip_serializing_if = "Option::is_none")]
        #[serde(default)]
        #[doc = "The number of obtained Trust Tokens on a successful \"Issuance\" operation."]
        pub issued_token_count: Option<JsUInt>,
    }
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
    pub struct PolicyUpdatedEvent(pub Option<Json>);
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
    pub struct ReportingApiReportAddedEvent {
        pub params: ReportingApiReportAddedEventParams,
    }
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
    pub struct ReportingApiReportAddedEventParams {
        pub report: super::ReportingApiReport,
    }
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
    pub struct ReportingApiReportUpdatedEvent {
        pub params: ReportingApiReportUpdatedEventParams,
    }
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
    pub struct ReportingApiReportUpdatedEventParams {
        pub report: super::ReportingApiReport,
    }
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
    pub struct ReportingApiEndpointsChangedForOriginEvent {
        pub params: ReportingApiEndpointsChangedForOriginEventParams,
    }
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
    pub struct ReportingApiEndpointsChangedForOriginEventParams {
        #[serde(default)]
        #[doc = "Origin of the document(s) which configured the endpoints."]
        pub origin: String,
        pub endpoints: Vec<super::ReportingApiEndpoint>,
    }
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
    pub struct DeviceBoundSessionsAddedEvent {
        pub params: DeviceBoundSessionsAddedEventParams,
    }
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
    pub struct DeviceBoundSessionsAddedEventParams {
        #[doc = "The device bound sessions."]
        pub sessions: Vec<super::DeviceBoundSession>,
    }
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
    pub struct DeviceBoundSessionEventOccurredEvent {
        pub params: DeviceBoundSessionEventOccurredEventParams,
    }
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
    pub struct DeviceBoundSessionEventOccurredEventParams {
        #[doc = "A unique identifier for this session event."]
        pub event_id: super::DeviceBoundSessionEventId,
        #[serde(default)]
        #[doc = "The site this session event is associated with."]
        pub site: String,
        #[serde(default)]
        #[doc = "Whether this event was considered successful."]
        pub succeeded: bool,
        #[builder(default)]
        #[serde(skip_serializing_if = "Option::is_none")]
        #[serde(default)]
        #[doc = "The session ID this event is associated with. May not be populated for\n failed events."]
        pub session_id: Option<String>,
        #[builder(default)]
        #[serde(skip_serializing_if = "Option::is_none")]
        #[doc = "The below are the different session event type details. Exactly one is populated."]
        pub creation_event_details: Option<super::CreationEventDetails>,
        #[builder(default)]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub refresh_event_details: Option<super::RefreshEventDetails>,
        #[builder(default)]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub termination_event_details: Option<super::TerminationEventDetails>,
        #[builder(default)]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub challenge_event_details: Option<super::ChallengeEventDetails>,
    }
}
