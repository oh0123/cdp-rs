// Auto-generated from Chrome at version 143.0.7499.110 domain: Audits
use super::dom;
use super::network;
use super::page;
use super::runtime;
#[allow(unused_imports)]
use super::types::*;
#[allow(unused_imports)]
use serde::{Deserialize, Serialize};
#[allow(unused_imports)]
use serde_json::Value as Json;
pub type IssueId = String;
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub enum CookieExclusionReason {
    #[serde(rename = "ExcludeSameSiteUnspecifiedTreatedAsLax")]
    ExcludeSameSiteUnspecifiedTreatedAsLax,
    #[serde(rename = "ExcludeSameSiteNoneInsecure")]
    ExcludeSameSiteNoneInsecure,
    #[serde(rename = "ExcludeSameSiteLax")]
    ExcludeSameSiteLax,
    #[serde(rename = "ExcludeSameSiteStrict")]
    ExcludeSameSiteStrict,
    #[serde(rename = "ExcludeInvalidSameParty")]
    ExcludeInvalidSameParty,
    #[serde(rename = "ExcludeSamePartyCrossPartyContext")]
    ExcludeSamePartyCrossPartyContext,
    #[serde(rename = "ExcludeDomainNonASCII")]
    ExcludeDomainNonAscii,
    #[serde(rename = "ExcludeThirdPartyCookieBlockedInFirstPartySet")]
    ExcludeThirdPartyCookieBlockedInFirstPartySet,
    #[serde(rename = "ExcludeThirdPartyPhaseout")]
    ExcludeThirdPartyPhaseout,
    #[serde(rename = "ExcludePortMismatch")]
    ExcludePortMismatch,
    #[serde(rename = "ExcludeSchemeMismatch")]
    ExcludeSchemeMismatch,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub enum CookieWarningReason {
    #[serde(rename = "WarnSameSiteUnspecifiedCrossSiteContext")]
    WarnSameSiteUnspecifiedCrossSiteContext,
    #[serde(rename = "WarnSameSiteNoneInsecure")]
    WarnSameSiteNoneInsecure,
    #[serde(rename = "WarnSameSiteUnspecifiedLaxAllowUnsafe")]
    WarnSameSiteUnspecifiedLaxAllowUnsafe,
    #[serde(rename = "WarnSameSiteStrictLaxDowngradeStrict")]
    WarnSameSiteStrictLaxDowngradeStrict,
    #[serde(rename = "WarnSameSiteStrictCrossDowngradeStrict")]
    WarnSameSiteStrictCrossDowngradeStrict,
    #[serde(rename = "WarnSameSiteStrictCrossDowngradeLax")]
    WarnSameSiteStrictCrossDowngradeLax,
    #[serde(rename = "WarnSameSiteLaxCrossDowngradeStrict")]
    WarnSameSiteLaxCrossDowngradeStrict,
    #[serde(rename = "WarnSameSiteLaxCrossDowngradeLax")]
    WarnSameSiteLaxCrossDowngradeLax,
    #[serde(rename = "WarnAttributeValueExceedsMaxSize")]
    WarnAttributeValueExceedsMaxSize,
    #[serde(rename = "WarnDomainNonASCII")]
    WarnDomainNonAscii,
    #[serde(rename = "WarnThirdPartyPhaseout")]
    WarnThirdPartyPhaseout,
    #[serde(rename = "WarnCrossSiteRedirectDowngradeChangesInclusion")]
    WarnCrossSiteRedirectDowngradeChangesInclusion,
    #[serde(rename = "WarnDeprecationTrialMetadata")]
    WarnDeprecationTrialMetadata,
    #[serde(rename = "WarnThirdPartyCookieHeuristic")]
    WarnThirdPartyCookieHeuristic,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub enum CookieOperation {
    #[serde(rename = "SetCookie")]
    SetCookie,
    #[serde(rename = "ReadCookie")]
    ReadCookie,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub enum InsightType {
    #[serde(rename = "GitHubResource")]
    GitHubResource,
    #[serde(rename = "GracePeriod")]
    GracePeriod,
    #[serde(rename = "Heuristics")]
    Heuristics,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub enum MixedContentResolutionStatus {
    #[serde(rename = "MixedContentBlocked")]
    MixedContentBlocked,
    #[serde(rename = "MixedContentAutomaticallyUpgraded")]
    MixedContentAutomaticallyUpgraded,
    #[serde(rename = "MixedContentWarning")]
    MixedContentWarning,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub enum MixedContentResourceType {
    #[serde(rename = "AttributionSrc")]
    AttributionSrc,
    #[serde(rename = "Audio")]
    Audio,
    #[serde(rename = "Beacon")]
    Beacon,
    #[serde(rename = "CSPReport")]
    CspReport,
    #[serde(rename = "Download")]
    Download,
    #[serde(rename = "EventSource")]
    EventSource,
    #[serde(rename = "Favicon")]
    Favicon,
    #[serde(rename = "Font")]
    Font,
    #[serde(rename = "Form")]
    Form,
    #[serde(rename = "Frame")]
    Frame,
    #[serde(rename = "Image")]
    Image,
    #[serde(rename = "Import")]
    Import,
    #[serde(rename = "JSON")]
    Json,
    #[serde(rename = "Manifest")]
    Manifest,
    #[serde(rename = "Ping")]
    Ping,
    #[serde(rename = "PluginData")]
    PluginData,
    #[serde(rename = "PluginResource")]
    PluginResource,
    #[serde(rename = "Prefetch")]
    Prefetch,
    #[serde(rename = "Resource")]
    Resource,
    #[serde(rename = "Script")]
    Script,
    #[serde(rename = "ServiceWorker")]
    ServiceWorker,
    #[serde(rename = "SharedWorker")]
    SharedWorker,
    #[serde(rename = "SpeculationRules")]
    SpeculationRules,
    #[serde(rename = "Stylesheet")]
    Stylesheet,
    #[serde(rename = "Track")]
    Track,
    #[serde(rename = "Video")]
    Video,
    #[serde(rename = "Worker")]
    Worker,
    #[serde(rename = "XMLHttpRequest")]
    XmlHttpRequest,
    #[serde(rename = "XSLT")]
    Xslt,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub enum BlockedByResponseReason {
    #[serde(rename = "CoepFrameResourceNeedsCoepHeader")]
    CoepFrameResourceNeedsCoepHeader,
    #[serde(rename = "CoopSandboxedIFrameCannotNavigateToCoopPage")]
    CoopSandboxedIFrameCannotNavigateToCoopPage,
    #[serde(rename = "CorpNotSameOrigin")]
    CorpNotSameOrigin,
    #[serde(rename = "CorpNotSameOriginAfterDefaultedToSameOriginByCoep")]
    CorpNotSameOriginAfterDefaultedToSameOriginByCoep,
    #[serde(rename = "CorpNotSameOriginAfterDefaultedToSameOriginByDip")]
    CorpNotSameOriginAfterDefaultedToSameOriginByDip,
    #[serde(rename = "CorpNotSameOriginAfterDefaultedToSameOriginByCoepAndDip")]
    CorpNotSameOriginAfterDefaultedToSameOriginByCoepAndDip,
    #[serde(rename = "CorpNotSameSite")]
    CorpNotSameSite,
    #[serde(rename = "SRIMessageSignatureMismatch")]
    SriMessageSignatureMismatch,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub enum HeavyAdResolutionStatus {
    #[serde(rename = "HeavyAdBlocked")]
    HeavyAdBlocked,
    #[serde(rename = "HeavyAdWarning")]
    HeavyAdWarning,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub enum HeavyAdReason {
    #[serde(rename = "NetworkTotalLimit")]
    NetworkTotalLimit,
    #[serde(rename = "CpuTotalLimit")]
    CpuTotalLimit,
    #[serde(rename = "CpuPeakLimit")]
    CpuPeakLimit,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub enum ContentSecurityPolicyViolationType {
    #[serde(rename = "kInlineViolation")]
    KInlineViolation,
    #[serde(rename = "kEvalViolation")]
    KEvalViolation,
    #[serde(rename = "kURLViolation")]
    KUrlViolation,
    #[serde(rename = "kSRIViolation")]
    KSriViolation,
    #[serde(rename = "kTrustedTypesSinkViolation")]
    KTrustedTypesSinkViolation,
    #[serde(rename = "kTrustedTypesPolicyViolation")]
    KTrustedTypesPolicyViolation,
    #[serde(rename = "kWasmEvalViolation")]
    KWasmEvalViolation,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub enum SharedArrayBufferIssueType {
    #[serde(rename = "TransferIssue")]
    TransferIssue,
    #[serde(rename = "CreationIssue")]
    CreationIssue,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub enum AttributionReportingIssueType {
    #[serde(rename = "PermissionPolicyDisabled")]
    PermissionPolicyDisabled,
    #[serde(rename = "UntrustworthyReportingOrigin")]
    UntrustworthyReportingOrigin,
    #[serde(rename = "InsecureContext")]
    InsecureContext,
    #[serde(rename = "InvalidHeader")]
    InvalidHeader,
    #[serde(rename = "InvalidRegisterTriggerHeader")]
    InvalidRegisterTriggerHeader,
    #[serde(rename = "SourceAndTriggerHeaders")]
    SourceAndTriggerHeaders,
    #[serde(rename = "SourceIgnored")]
    SourceIgnored,
    #[serde(rename = "TriggerIgnored")]
    TriggerIgnored,
    #[serde(rename = "OsSourceIgnored")]
    OsSourceIgnored,
    #[serde(rename = "OsTriggerIgnored")]
    OsTriggerIgnored,
    #[serde(rename = "InvalidRegisterOsSourceHeader")]
    InvalidRegisterOsSourceHeader,
    #[serde(rename = "InvalidRegisterOsTriggerHeader")]
    InvalidRegisterOsTriggerHeader,
    #[serde(rename = "WebAndOsHeaders")]
    WebAndOsHeaders,
    #[serde(rename = "NoWebOrOsSupport")]
    NoWebOrOsSupport,
    #[serde(rename = "NavigationRegistrationWithoutTransientUserActivation")]
    NavigationRegistrationWithoutTransientUserActivation,
    #[serde(rename = "InvalidInfoHeader")]
    InvalidInfoHeader,
    #[serde(rename = "NoRegisterSourceHeader")]
    NoRegisterSourceHeader,
    #[serde(rename = "NoRegisterTriggerHeader")]
    NoRegisterTriggerHeader,
    #[serde(rename = "NoRegisterOsSourceHeader")]
    NoRegisterOsSourceHeader,
    #[serde(rename = "NoRegisterOsTriggerHeader")]
    NoRegisterOsTriggerHeader,
    #[serde(rename = "NavigationRegistrationUniqueScopeAlreadySet")]
    NavigationRegistrationUniqueScopeAlreadySet,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub enum SharedDictionaryError {
    #[serde(rename = "UseErrorCrossOriginNoCorsRequest")]
    UseErrorCrossOriginNoCorsRequest,
    #[serde(rename = "UseErrorDictionaryLoadFailure")]
    UseErrorDictionaryLoadFailure,
    #[serde(rename = "UseErrorMatchingDictionaryNotUsed")]
    UseErrorMatchingDictionaryNotUsed,
    #[serde(rename = "UseErrorUnexpectedContentDictionaryHeader")]
    UseErrorUnexpectedContentDictionaryHeader,
    #[serde(rename = "WriteErrorCossOriginNoCorsRequest")]
    WriteErrorCossOriginNoCorsRequest,
    #[serde(rename = "WriteErrorDisallowedBySettings")]
    WriteErrorDisallowedBySettings,
    #[serde(rename = "WriteErrorExpiredResponse")]
    WriteErrorExpiredResponse,
    #[serde(rename = "WriteErrorFeatureDisabled")]
    WriteErrorFeatureDisabled,
    #[serde(rename = "WriteErrorInsufficientResources")]
    WriteErrorInsufficientResources,
    #[serde(rename = "WriteErrorInvalidMatchField")]
    WriteErrorInvalidMatchField,
    #[serde(rename = "WriteErrorInvalidStructuredHeader")]
    WriteErrorInvalidStructuredHeader,
    #[serde(rename = "WriteErrorInvalidTTLField")]
    WriteErrorInvalidTtlField,
    #[serde(rename = "WriteErrorNavigationRequest")]
    WriteErrorNavigationRequest,
    #[serde(rename = "WriteErrorNoMatchField")]
    WriteErrorNoMatchField,
    #[serde(rename = "WriteErrorNonIntegerTTLField")]
    WriteErrorNonIntegerTtlField,
    #[serde(rename = "WriteErrorNonListMatchDestField")]
    WriteErrorNonListMatchDestField,
    #[serde(rename = "WriteErrorNonSecureContext")]
    WriteErrorNonSecureContext,
    #[serde(rename = "WriteErrorNonStringIdField")]
    WriteErrorNonStringIdField,
    #[serde(rename = "WriteErrorNonStringInMatchDestList")]
    WriteErrorNonStringInMatchDestList,
    #[serde(rename = "WriteErrorNonStringMatchField")]
    WriteErrorNonStringMatchField,
    #[serde(rename = "WriteErrorNonTokenTypeField")]
    WriteErrorNonTokenTypeField,
    #[serde(rename = "WriteErrorRequestAborted")]
    WriteErrorRequestAborted,
    #[serde(rename = "WriteErrorShuttingDown")]
    WriteErrorShuttingDown,
    #[serde(rename = "WriteErrorTooLongIdField")]
    WriteErrorTooLongIdField,
    #[serde(rename = "WriteErrorUnsupportedType")]
    WriteErrorUnsupportedType,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub enum SriMessageSignatureError {
    #[serde(rename = "MissingSignatureHeader")]
    MissingSignatureHeader,
    #[serde(rename = "MissingSignatureInputHeader")]
    MissingSignatureInputHeader,
    #[serde(rename = "InvalidSignatureHeader")]
    InvalidSignatureHeader,
    #[serde(rename = "InvalidSignatureInputHeader")]
    InvalidSignatureInputHeader,
    #[serde(rename = "SignatureHeaderValueIsNotByteSequence")]
    SignatureHeaderValueIsNotByteSequence,
    #[serde(rename = "SignatureHeaderValueIsParameterized")]
    SignatureHeaderValueIsParameterized,
    #[serde(rename = "SignatureHeaderValueIsIncorrectLength")]
    SignatureHeaderValueIsIncorrectLength,
    #[serde(rename = "SignatureInputHeaderMissingLabel")]
    SignatureInputHeaderMissingLabel,
    #[serde(rename = "SignatureInputHeaderValueNotInnerList")]
    SignatureInputHeaderValueNotInnerList,
    #[serde(rename = "SignatureInputHeaderValueMissingComponents")]
    SignatureInputHeaderValueMissingComponents,
    #[serde(rename = "SignatureInputHeaderInvalidComponentType")]
    SignatureInputHeaderInvalidComponentType,
    #[serde(rename = "SignatureInputHeaderInvalidComponentName")]
    SignatureInputHeaderInvalidComponentName,
    #[serde(rename = "SignatureInputHeaderInvalidHeaderComponentParameter")]
    SignatureInputHeaderInvalidHeaderComponentParameter,
    #[serde(rename = "SignatureInputHeaderInvalidDerivedComponentParameter")]
    SignatureInputHeaderInvalidDerivedComponentParameter,
    #[serde(rename = "SignatureInputHeaderKeyIdLength")]
    SignatureInputHeaderKeyIdLength,
    #[serde(rename = "SignatureInputHeaderInvalidParameter")]
    SignatureInputHeaderInvalidParameter,
    #[serde(rename = "SignatureInputHeaderMissingRequiredParameters")]
    SignatureInputHeaderMissingRequiredParameters,
    #[serde(rename = "ValidationFailedSignatureExpired")]
    ValidationFailedSignatureExpired,
    #[serde(rename = "ValidationFailedInvalidLength")]
    ValidationFailedInvalidLength,
    #[serde(rename = "ValidationFailedSignatureMismatch")]
    ValidationFailedSignatureMismatch,
    #[serde(rename = "ValidationFailedIntegrityMismatch")]
    ValidationFailedIntegrityMismatch,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub enum UnencodedDigestError {
    #[serde(rename = "MalformedDictionary")]
    MalformedDictionary,
    #[serde(rename = "UnknownAlgorithm")]
    UnknownAlgorithm,
    #[serde(rename = "IncorrectDigestType")]
    IncorrectDigestType,
    #[serde(rename = "IncorrectDigestLength")]
    IncorrectDigestLength,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub enum GenericIssueErrorType {
    #[serde(rename = "FormLabelForNameError")]
    FormLabelForNameError,
    #[serde(rename = "FormDuplicateIdForInputError")]
    FormDuplicateIdForInputError,
    #[serde(rename = "FormInputWithNoLabelError")]
    FormInputWithNoLabelError,
    #[serde(rename = "FormAutocompleteAttributeEmptyError")]
    FormAutocompleteAttributeEmptyError,
    #[serde(rename = "FormEmptyIdAndNameAttributesForInputError")]
    FormEmptyIdAndNameAttributesForInputError,
    #[serde(rename = "FormAriaLabelledByToNonExistingId")]
    FormAriaLabelledByToNonExistingId,
    #[serde(rename = "FormInputAssignedAutocompleteValueToIdOrNameAttributeError")]
    FormInputAssignedAutocompleteValueToIdOrNameAttributeError,
    #[serde(rename = "FormLabelHasNeitherForNorNestedInput")]
    FormLabelHasNeitherForNorNestedInput,
    #[serde(rename = "FormLabelForMatchesNonExistingIdError")]
    FormLabelForMatchesNonExistingIdError,
    #[serde(rename = "FormInputHasWrongButWellIntendedAutocompleteValueError")]
    FormInputHasWrongButWellIntendedAutocompleteValueError,
    #[serde(rename = "ResponseWasBlockedByORB")]
    ResponseWasBlockedByOrb,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub enum ClientHintIssueReason {
    #[serde(rename = "MetaTagAllowListInvalidOrigin")]
    MetaTagAllowListInvalidOrigin,
    #[serde(rename = "MetaTagModifiedHTML")]
    MetaTagModifiedHtml,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub enum FederatedAuthRequestIssueReason {
    #[serde(rename = "ShouldEmbargo")]
    ShouldEmbargo,
    #[serde(rename = "TooManyRequests")]
    TooManyRequests,
    #[serde(rename = "WellKnownHttpNotFound")]
    WellKnownHttpNotFound,
    #[serde(rename = "WellKnownNoResponse")]
    WellKnownNoResponse,
    #[serde(rename = "WellKnownInvalidResponse")]
    WellKnownInvalidResponse,
    #[serde(rename = "WellKnownListEmpty")]
    WellKnownListEmpty,
    #[serde(rename = "WellKnownInvalidContentType")]
    WellKnownInvalidContentType,
    #[serde(rename = "ConfigNotInWellKnown")]
    ConfigNotInWellKnown,
    #[serde(rename = "WellKnownTooBig")]
    WellKnownTooBig,
    #[serde(rename = "ConfigHttpNotFound")]
    ConfigHttpNotFound,
    #[serde(rename = "ConfigNoResponse")]
    ConfigNoResponse,
    #[serde(rename = "ConfigInvalidResponse")]
    ConfigInvalidResponse,
    #[serde(rename = "ConfigInvalidContentType")]
    ConfigInvalidContentType,
    #[serde(rename = "ClientMetadataHttpNotFound")]
    ClientMetadataHttpNotFound,
    #[serde(rename = "ClientMetadataNoResponse")]
    ClientMetadataNoResponse,
    #[serde(rename = "ClientMetadataInvalidResponse")]
    ClientMetadataInvalidResponse,
    #[serde(rename = "ClientMetadataInvalidContentType")]
    ClientMetadataInvalidContentType,
    #[serde(rename = "IdpNotPotentiallyTrustworthy")]
    IdpNotPotentiallyTrustworthy,
    #[serde(rename = "DisabledInSettings")]
    DisabledInSettings,
    #[serde(rename = "DisabledInFlags")]
    DisabledInFlags,
    #[serde(rename = "ErrorFetchingSignin")]
    ErrorFetchingSignin,
    #[serde(rename = "InvalidSigninResponse")]
    InvalidSigninResponse,
    #[serde(rename = "AccountsHttpNotFound")]
    AccountsHttpNotFound,
    #[serde(rename = "AccountsNoResponse")]
    AccountsNoResponse,
    #[serde(rename = "AccountsInvalidResponse")]
    AccountsInvalidResponse,
    #[serde(rename = "AccountsListEmpty")]
    AccountsListEmpty,
    #[serde(rename = "AccountsInvalidContentType")]
    AccountsInvalidContentType,
    #[serde(rename = "IdTokenHttpNotFound")]
    IdTokenHttpNotFound,
    #[serde(rename = "IdTokenNoResponse")]
    IdTokenNoResponse,
    #[serde(rename = "IdTokenInvalidResponse")]
    IdTokenInvalidResponse,
    #[serde(rename = "IdTokenIdpErrorResponse")]
    IdTokenIdpErrorResponse,
    #[serde(rename = "IdTokenCrossSiteIdpErrorResponse")]
    IdTokenCrossSiteIdpErrorResponse,
    #[serde(rename = "IdTokenInvalidRequest")]
    IdTokenInvalidRequest,
    #[serde(rename = "IdTokenInvalidContentType")]
    IdTokenInvalidContentType,
    #[serde(rename = "ErrorIdToken")]
    ErrorIdToken,
    #[serde(rename = "Canceled")]
    Canceled,
    #[serde(rename = "RpPageNotVisible")]
    RpPageNotVisible,
    #[serde(rename = "SilentMediationFailure")]
    SilentMediationFailure,
    #[serde(rename = "ThirdPartyCookiesBlocked")]
    ThirdPartyCookiesBlocked,
    #[serde(rename = "NotSignedInWithIdp")]
    NotSignedInWithIdp,
    #[serde(rename = "MissingTransientUserActivation")]
    MissingTransientUserActivation,
    #[serde(rename = "ReplacedByActiveMode")]
    ReplacedByActiveMode,
    #[serde(rename = "InvalidFieldsSpecified")]
    InvalidFieldsSpecified,
    #[serde(rename = "RelyingPartyOriginIsOpaque")]
    RelyingPartyOriginIsOpaque,
    #[serde(rename = "TypeNotMatching")]
    TypeNotMatching,
    #[serde(rename = "UiDismissedNoEmbargo")]
    UiDismissedNoEmbargo,
    #[serde(rename = "CorsError")]
    CorsError,
    #[serde(rename = "SuppressedBySegmentationPlatform")]
    SuppressedBySegmentationPlatform,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub enum FederatedAuthUserInfoRequestIssueReason {
    #[serde(rename = "NotSameOrigin")]
    NotSameOrigin,
    #[serde(rename = "NotIframe")]
    NotIframe,
    #[serde(rename = "NotPotentiallyTrustworthy")]
    NotPotentiallyTrustworthy,
    #[serde(rename = "NoApiPermission")]
    NoApiPermission,
    #[serde(rename = "NotSignedInWithIdp")]
    NotSignedInWithIdp,
    #[serde(rename = "NoAccountSharingPermission")]
    NoAccountSharingPermission,
    #[serde(rename = "InvalidConfigOrWellKnown")]
    InvalidConfigOrWellKnown,
    #[serde(rename = "InvalidAccountsResponse")]
    InvalidAccountsResponse,
    #[serde(rename = "NoReturningUserFromFetchedAccounts")]
    NoReturningUserFromFetchedAccounts,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub enum PartitioningBlobUrlInfo {
    #[serde(rename = "BlockedCrossPartitionFetching")]
    BlockedCrossPartitionFetching,
    #[serde(rename = "EnforceNoopenerForNavigation")]
    EnforceNoopenerForNavigation,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub enum ElementAccessibilityIssueReason {
    #[serde(rename = "DisallowedSelectChild")]
    DisallowedSelectChild,
    #[serde(rename = "DisallowedOptGroupChild")]
    DisallowedOptGroupChild,
    #[serde(rename = "NonPhrasingContentOptionChild")]
    NonPhrasingContentOptionChild,
    #[serde(rename = "InteractiveContentOptionChild")]
    InteractiveContentOptionChild,
    #[serde(rename = "InteractiveContentLegendChild")]
    InteractiveContentLegendChild,
    #[serde(rename = "InteractiveContentSummaryDescendant")]
    InteractiveContentSummaryDescendant,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub enum StyleSheetLoadingIssueReason {
    #[serde(rename = "LateImportRule")]
    LateImportRule,
    #[serde(rename = "RequestFailed")]
    RequestFailed,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub enum PropertyRuleIssueReason {
    #[serde(rename = "InvalidSyntax")]
    InvalidSyntax,
    #[serde(rename = "InvalidInitialValue")]
    InvalidInitialValue,
    #[serde(rename = "InvalidInherits")]
    InvalidInherits,
    #[serde(rename = "InvalidName")]
    InvalidName,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub enum UserReidentificationIssueType {
    #[serde(rename = "BlockedFrameNavigation")]
    BlockedFrameNavigation,
    #[serde(rename = "BlockedSubresource")]
    BlockedSubresource,
    #[serde(rename = "NoisedCanvasReadback")]
    NoisedCanvasReadback,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub enum InspectorIssueCode {
    #[serde(rename = "CookieIssue")]
    CookieIssue,
    #[serde(rename = "MixedContentIssue")]
    MixedContentIssue,
    #[serde(rename = "BlockedByResponseIssue")]
    BlockedByResponseIssue,
    #[serde(rename = "HeavyAdIssue")]
    HeavyAdIssue,
    #[serde(rename = "ContentSecurityPolicyIssue")]
    ContentSecurityPolicyIssue,
    #[serde(rename = "SharedArrayBufferIssue")]
    SharedArrayBufferIssue,
    #[serde(rename = "LowTextContrastIssue")]
    LowTextContrastIssue,
    #[serde(rename = "CorsIssue")]
    CorsIssue,
    #[serde(rename = "AttributionReportingIssue")]
    AttributionReportingIssue,
    #[serde(rename = "QuirksModeIssue")]
    QuirksModeIssue,
    #[serde(rename = "PartitioningBlobURLIssue")]
    PartitioningBlobUrlIssue,
    #[serde(rename = "NavigatorUserAgentIssue")]
    NavigatorUserAgentIssue,
    #[serde(rename = "GenericIssue")]
    GenericIssue,
    #[serde(rename = "DeprecationIssue")]
    DeprecationIssue,
    #[serde(rename = "ClientHintIssue")]
    ClientHintIssue,
    #[serde(rename = "FederatedAuthRequestIssue")]
    FederatedAuthRequestIssue,
    #[serde(rename = "BounceTrackingIssue")]
    BounceTrackingIssue,
    #[serde(rename = "CookieDeprecationMetadataIssue")]
    CookieDeprecationMetadataIssue,
    #[serde(rename = "StylesheetLoadingIssue")]
    StylesheetLoadingIssue,
    #[serde(rename = "FederatedAuthUserInfoRequestIssue")]
    FederatedAuthUserInfoRequestIssue,
    #[serde(rename = "PropertyRuleIssue")]
    PropertyRuleIssue,
    #[serde(rename = "SharedDictionaryIssue")]
    SharedDictionaryIssue,
    #[serde(rename = "ElementAccessibilityIssue")]
    ElementAccessibilityIssue,
    #[serde(rename = "SRIMessageSignatureIssue")]
    SriMessageSignatureIssue,
    #[serde(rename = "UnencodedDigestIssue")]
    UnencodedDigestIssue,
    #[serde(rename = "UserReidentificationIssue")]
    UserReidentificationIssue,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub enum GetEncodedResponseEncodingOption {
    #[serde(rename = "webp")]
    Webp,
    #[serde(rename = "jpeg")]
    Jpeg,
    #[serde(rename = "png")]
    Png,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct AffectedCookie {
    #[serde(default)]
    #[serde(rename = "name")]
    pub name: String,
    #[serde(default)]
    #[serde(rename = "path")]
    pub path: String,
    #[serde(default)]
    #[serde(rename = "domain")]
    pub domain: String,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct AffectedRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "requestId")]
    pub request_id: Option<network::RequestId>,
    #[serde(default)]
    #[serde(rename = "url")]
    pub url: String,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct AffectedFrame {
    #[serde(rename = "frameId")]
    pub frame_id: page::FrameId,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct CookieIssueInsight {
    #[serde(rename = "type")]
    pub r#type: InsightType,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(rename = "tableEntryUrl")]
    pub table_entry_url: Option<String>,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct CookieIssueDetails {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "cookie")]
    pub cookie: Option<AffectedCookie>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(rename = "rawCookieLine")]
    pub raw_cookie_line: Option<String>,
    #[serde(rename = "cookieWarningReasons")]
    pub cookie_warning_reasons: Vec<CookieWarningReason>,
    #[serde(rename = "cookieExclusionReasons")]
    pub cookie_exclusion_reasons: Vec<CookieExclusionReason>,
    #[serde(rename = "operation")]
    pub operation: CookieOperation,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(rename = "siteForCookies")]
    pub site_for_cookies: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(rename = "cookieUrl")]
    pub cookie_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "request")]
    pub request: Option<AffectedRequest>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "insight")]
    pub insight: Option<CookieIssueInsight>,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct MixedContentIssueDetails {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "resourceType")]
    pub resource_type: Option<MixedContentResourceType>,
    #[serde(rename = "resolutionStatus")]
    pub resolution_status: MixedContentResolutionStatus,
    #[serde(default)]
    #[serde(rename = "insecureURL")]
    pub insecure_url: String,
    #[serde(default)]
    #[serde(rename = "mainResourceURL")]
    pub main_resource_url: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "request")]
    pub request: Option<AffectedRequest>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "frame")]
    pub frame: Option<AffectedFrame>,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct BlockedByResponseIssueDetails {
    #[serde(rename = "request")]
    pub request: AffectedRequest,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "parentFrame")]
    pub parent_frame: Option<AffectedFrame>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "blockedFrame")]
    pub blocked_frame: Option<AffectedFrame>,
    #[serde(rename = "reason")]
    pub reason: BlockedByResponseReason,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct HeavyAdIssueDetails {
    #[serde(rename = "resolution")]
    pub resolution: HeavyAdResolutionStatus,
    #[serde(rename = "reason")]
    pub reason: HeavyAdReason,
    #[serde(rename = "frame")]
    pub frame: AffectedFrame,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct SourceCodeLocation {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "scriptId")]
    pub script_id: Option<runtime::ScriptId>,
    #[serde(default)]
    #[serde(rename = "url")]
    pub url: String,
    #[serde(default)]
    #[serde(rename = "lineNumber")]
    pub line_number: JsUInt,
    #[serde(default)]
    #[serde(rename = "columnNumber")]
    pub column_number: JsUInt,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct ContentSecurityPolicyIssueDetails {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(rename = "blockedURL")]
    pub blocked_url: Option<String>,
    #[serde(default)]
    #[serde(rename = "violatedDirective")]
    pub violated_directive: String,
    #[serde(default)]
    #[serde(rename = "isReportOnly")]
    pub is_report_only: bool,
    #[serde(rename = "contentSecurityPolicyViolationType")]
    pub content_security_policy_violation_type: ContentSecurityPolicyViolationType,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "frameAncestor")]
    pub frame_ancestor: Option<AffectedFrame>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "sourceCodeLocation")]
    pub source_code_location: Option<SourceCodeLocation>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "violatingNodeId")]
    pub violating_node_id: Option<dom::BackendNodeId>,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct SharedArrayBufferIssueDetails {
    #[serde(rename = "sourceCodeLocation")]
    pub source_code_location: SourceCodeLocation,
    #[serde(default)]
    #[serde(rename = "isWarning")]
    pub is_warning: bool,
    #[serde(rename = "type")]
    pub r#type: SharedArrayBufferIssueType,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct LowTextContrastIssueDetails {
    #[serde(rename = "violatingNodeId")]
    pub violating_node_id: dom::BackendNodeId,
    #[serde(default)]
    #[serde(rename = "violatingNodeSelector")]
    pub violating_node_selector: String,
    #[serde(default)]
    #[serde(rename = "contrastRatio")]
    pub contrast_ratio: JsFloat,
    #[serde(default)]
    #[serde(rename = "thresholdAA")]
    pub threshold_aa: JsFloat,
    #[serde(default)]
    #[serde(rename = "thresholdAAA")]
    pub threshold_aaa: JsFloat,
    #[serde(default)]
    #[serde(rename = "fontSize")]
    pub font_size: String,
    #[serde(default)]
    #[serde(rename = "fontWeight")]
    pub font_weight: String,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct CorsIssueDetails {
    #[serde(rename = "corsErrorStatus")]
    pub cors_error_status: network::CorsErrorStatus,
    #[serde(default)]
    #[serde(rename = "isWarning")]
    pub is_warning: bool,
    #[serde(rename = "request")]
    pub request: AffectedRequest,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "location")]
    pub location: Option<SourceCodeLocation>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(rename = "initiatorOrigin")]
    pub initiator_origin: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "resourceIPAddressSpace")]
    pub resource_ip_address_space: Option<network::IpAddressSpace>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "clientSecurityState")]
    pub client_security_state: Option<network::ClientSecurityState>,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct AttributionReportingIssueDetails {
    #[serde(rename = "violationType")]
    pub violation_type: AttributionReportingIssueType,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "request")]
    pub request: Option<AffectedRequest>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "violatingNodeId")]
    pub violating_node_id: Option<dom::BackendNodeId>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(rename = "invalidParameter")]
    pub invalid_parameter: Option<String>,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct QuirksModeIssueDetails {
    #[serde(default)]
    #[serde(rename = "isLimitedQuirksMode")]
    pub is_limited_quirks_mode: bool,
    #[serde(rename = "documentNodeId")]
    pub document_node_id: dom::BackendNodeId,
    #[serde(default)]
    #[serde(rename = "url")]
    pub url: String,
    #[serde(rename = "frameId")]
    pub frame_id: page::FrameId,
    #[serde(rename = "loaderId")]
    pub loader_id: network::LoaderId,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct NavigatorUserAgentIssueDetails {
    #[serde(default)]
    #[serde(rename = "url")]
    pub url: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "location")]
    pub location: Option<SourceCodeLocation>,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct SharedDictionaryIssueDetails {
    #[serde(rename = "sharedDictionaryError")]
    pub shared_dictionary_error: SharedDictionaryError,
    #[serde(rename = "request")]
    pub request: AffectedRequest,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct SriMessageSignatureIssueDetails {
    #[serde(rename = "error")]
    pub error: SriMessageSignatureError,
    #[serde(default)]
    #[serde(rename = "signatureBase")]
    pub signature_base: String,
    #[serde(default)]
    #[serde(rename = "integrityAssertions")]
    pub integrity_assertions: Vec<String>,
    #[serde(rename = "request")]
    pub request: AffectedRequest,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct UnencodedDigestIssueDetails {
    #[serde(rename = "error")]
    pub error: UnencodedDigestError,
    #[serde(rename = "request")]
    pub request: AffectedRequest,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct GenericIssueDetails {
    #[serde(rename = "errorType")]
    pub error_type: GenericIssueErrorType,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "frameId")]
    pub frame_id: Option<page::FrameId>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "violatingNodeId")]
    pub violating_node_id: Option<dom::BackendNodeId>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(rename = "violatingNodeAttribute")]
    pub violating_node_attribute: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "request")]
    pub request: Option<AffectedRequest>,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct DeprecationIssueDetails {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "affectedFrame")]
    pub affected_frame: Option<AffectedFrame>,
    #[serde(rename = "sourceCodeLocation")]
    pub source_code_location: SourceCodeLocation,
    #[serde(default)]
    #[serde(rename = "type")]
    pub r#type: String,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct BounceTrackingIssueDetails {
    #[serde(default)]
    #[serde(rename = "trackingSites")]
    pub tracking_sites: Vec<String>,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct CookieDeprecationMetadataIssueDetails {
    #[serde(default)]
    #[serde(rename = "allowedSites")]
    pub allowed_sites: Vec<String>,
    #[serde(default)]
    #[serde(rename = "optOutPercentage")]
    pub opt_out_percentage: JsFloat,
    #[serde(default)]
    #[serde(rename = "isOptOutTopLevel")]
    pub is_opt_out_top_level: bool,
    #[serde(rename = "operation")]
    pub operation: CookieOperation,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct FederatedAuthRequestIssueDetails {
    #[serde(rename = "federatedAuthRequestIssueReason")]
    pub federated_auth_request_issue_reason: FederatedAuthRequestIssueReason,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct FederatedAuthUserInfoRequestIssueDetails {
    #[serde(rename = "federatedAuthUserInfoRequestIssueReason")]
    pub federated_auth_user_info_request_issue_reason: FederatedAuthUserInfoRequestIssueReason,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct ClientHintIssueDetails {
    #[serde(rename = "sourceCodeLocation")]
    pub source_code_location: SourceCodeLocation,
    #[serde(rename = "clientHintIssueReason")]
    pub client_hint_issue_reason: ClientHintIssueReason,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct FailedRequestInfo {
    #[serde(default)]
    #[serde(rename = "url")]
    pub url: String,
    #[serde(default)]
    #[serde(rename = "failureMessage")]
    pub failure_message: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "requestId")]
    pub request_id: Option<network::RequestId>,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct PartitioningBlobUrlIssueDetails {
    #[serde(default)]
    #[serde(rename = "url")]
    pub url: String,
    #[serde(rename = "partitioningBlobURLInfo")]
    pub partitioning_blob_url_info: PartitioningBlobUrlInfo,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct ElementAccessibilityIssueDetails {
    #[serde(rename = "nodeId")]
    pub node_id: dom::BackendNodeId,
    #[serde(rename = "elementAccessibilityIssueReason")]
    pub element_accessibility_issue_reason: ElementAccessibilityIssueReason,
    #[serde(default)]
    #[serde(rename = "hasDisallowedAttributes")]
    pub has_disallowed_attributes: bool,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct StylesheetLoadingIssueDetails {
    #[serde(rename = "sourceCodeLocation")]
    pub source_code_location: SourceCodeLocation,
    #[serde(rename = "styleSheetLoadingIssueReason")]
    pub style_sheet_loading_issue_reason: StyleSheetLoadingIssueReason,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "failedRequestInfo")]
    pub failed_request_info: Option<FailedRequestInfo>,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct PropertyRuleIssueDetails {
    #[serde(rename = "sourceCodeLocation")]
    pub source_code_location: SourceCodeLocation,
    #[serde(rename = "propertyRuleIssueReason")]
    pub property_rule_issue_reason: PropertyRuleIssueReason,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(rename = "propertyValue")]
    pub property_value: Option<String>,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct UserReidentificationIssueDetails {
    #[serde(rename = "type")]
    pub r#type: UserReidentificationIssueType,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "request")]
    pub request: Option<AffectedRequest>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "sourceCodeLocation")]
    pub source_code_location: Option<SourceCodeLocation>,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct InspectorIssueDetails {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "cookieIssueDetails")]
    pub cookie_issue_details: Option<CookieIssueDetails>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "mixedContentIssueDetails")]
    pub mixed_content_issue_details: Option<MixedContentIssueDetails>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "blockedByResponseIssueDetails")]
    pub blocked_by_response_issue_details: Option<BlockedByResponseIssueDetails>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "heavyAdIssueDetails")]
    pub heavy_ad_issue_details: Option<HeavyAdIssueDetails>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "contentSecurityPolicyIssueDetails")]
    pub content_security_policy_issue_details: Option<ContentSecurityPolicyIssueDetails>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "sharedArrayBufferIssueDetails")]
    pub shared_array_buffer_issue_details: Option<SharedArrayBufferIssueDetails>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "lowTextContrastIssueDetails")]
    pub low_text_contrast_issue_details: Option<LowTextContrastIssueDetails>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "corsIssueDetails")]
    pub cors_issue_details: Option<CorsIssueDetails>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "attributionReportingIssueDetails")]
    pub attribution_reporting_issue_details: Option<AttributionReportingIssueDetails>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "quirksModeIssueDetails")]
    pub quirks_mode_issue_details: Option<QuirksModeIssueDetails>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "partitioningBlobURLIssueDetails")]
    pub partitioning_blob_url_issue_details: Option<PartitioningBlobUrlIssueDetails>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "navigatorUserAgentIssueDetails")]
    pub navigator_user_agent_issue_details: Option<NavigatorUserAgentIssueDetails>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "genericIssueDetails")]
    pub generic_issue_details: Option<GenericIssueDetails>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "deprecationIssueDetails")]
    pub deprecation_issue_details: Option<DeprecationIssueDetails>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "clientHintIssueDetails")]
    pub client_hint_issue_details: Option<ClientHintIssueDetails>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "federatedAuthRequestIssueDetails")]
    pub federated_auth_request_issue_details: Option<FederatedAuthRequestIssueDetails>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "bounceTrackingIssueDetails")]
    pub bounce_tracking_issue_details: Option<BounceTrackingIssueDetails>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "cookieDeprecationMetadataIssueDetails")]
    pub cookie_deprecation_metadata_issue_details: Option<CookieDeprecationMetadataIssueDetails>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "stylesheetLoadingIssueDetails")]
    pub stylesheet_loading_issue_details: Option<StylesheetLoadingIssueDetails>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "propertyRuleIssueDetails")]
    pub property_rule_issue_details: Option<PropertyRuleIssueDetails>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "federatedAuthUserInfoRequestIssueDetails")]
    pub federated_auth_user_info_request_issue_details:
        Option<FederatedAuthUserInfoRequestIssueDetails>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "sharedDictionaryIssueDetails")]
    pub shared_dictionary_issue_details: Option<SharedDictionaryIssueDetails>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "elementAccessibilityIssueDetails")]
    pub element_accessibility_issue_details: Option<ElementAccessibilityIssueDetails>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "sriMessageSignatureIssueDetails")]
    pub sri_message_signature_issue_details: Option<SriMessageSignatureIssueDetails>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "unencodedDigestIssueDetails")]
    pub unencoded_digest_issue_details: Option<UnencodedDigestIssueDetails>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "userReidentificationIssueDetails")]
    pub user_reidentification_issue_details: Option<UserReidentificationIssueDetails>,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct InspectorIssue {
    #[serde(rename = "code")]
    pub code: InspectorIssueCode,
    #[serde(rename = "details")]
    pub details: InspectorIssueDetails,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "issueId")]
    pub issue_id: Option<IssueId>,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct GetEncodedResponse {
    #[serde(rename = "requestId")]
    pub request_id: network::RequestId,
    #[serde(rename = "encoding")]
    pub encoding: GetEncodedResponseEncodingOption,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(rename = "quality")]
    pub quality: Option<JsFloat>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(rename = "sizeOnly")]
    pub size_only: Option<bool>,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Disable(pub Option<serde_json::Value>);
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Enable(pub Option<serde_json::Value>);
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct CheckContrast {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(rename = "reportAAA")]
    pub report_aaa: Option<bool>,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct CheckFormsIssues(pub Option<serde_json::Value>);
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct GetEncodedResponseReturnObject {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "body")]
    pub body: Option<Vec<u8>>,
    #[serde(default)]
    #[serde(rename = "originalSize")]
    pub original_size: JsUInt,
    #[serde(default)]
    #[serde(rename = "encodedSize")]
    pub encoded_size: JsUInt,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct DisableReturnObject {}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct EnableReturnObject {}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct CheckContrastReturnObject {}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct CheckFormsIssuesReturnObject {
    #[serde(rename = "formIssues")]
    pub form_issues: Vec<GenericIssueDetails>,
}
impl Method for GetEncodedResponse {
    const NAME: &'static str = "Audits.getEncodedResponse";
    type ReturnObject = GetEncodedResponseReturnObject;
}
impl Method for Disable {
    const NAME: &'static str = "Audits.disable";
    type ReturnObject = DisableReturnObject;
}
impl Method for Enable {
    const NAME: &'static str = "Audits.enable";
    type ReturnObject = EnableReturnObject;
}
impl Method for CheckContrast {
    const NAME: &'static str = "Audits.checkContrast";
    type ReturnObject = CheckContrastReturnObject;
}
impl Method for CheckFormsIssues {
    const NAME: &'static str = "Audits.checkFormsIssues";
    type ReturnObject = CheckFormsIssuesReturnObject;
}
pub mod events {
    #[allow(unused_imports)]
    use super::super::types::*;
    #[allow(unused_imports)]
    use serde::{Deserialize, Serialize};
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
    pub struct IssueAddedEvent {
        pub params: IssueAddedEventParams,
    }
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
    pub struct IssueAddedEventParams {
        #[serde(rename = "issue")]
        pub issue: super::InspectorIssue,
    }
}
