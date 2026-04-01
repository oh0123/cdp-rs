// Auto-generated from Chrome at version 146.0.7680.165 domain: Audits
use super::dom;
use super::network;
use super::page;
use super::runtime;
#[allow(unused_imports)]
use super::types::*;
#[allow(unused_imports)]
use derive_builder::Builder;
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
pub enum ConnectionAllowlistError {
    #[serde(rename = "InvalidHeader")]
    InvalidHeader,
    #[serde(rename = "MoreThanOneList")]
    MoreThanOneList,
    #[serde(rename = "ItemNotInnerList")]
    ItemNotInnerList,
    #[serde(rename = "InvalidAllowlistItemType")]
    InvalidAllowlistItemType,
    #[serde(rename = "ReportingEndpointNotToken")]
    ReportingEndpointNotToken,
    #[serde(rename = "InvalidUrlPattern")]
    InvalidUrlPattern,
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
    #[serde(rename = "FormAriaLabelledByToNonExistingIdError")]
    FormAriaLabelledByToNonExistingIdError,
    #[serde(rename = "FormInputAssignedAutocompleteValueToIdOrNameAttributeError")]
    FormInputAssignedAutocompleteValueToIdOrNameAttributeError,
    #[serde(rename = "FormLabelHasNeitherForNorNestedInputError")]
    FormLabelHasNeitherForNorNestedInputError,
    #[serde(rename = "FormLabelForMatchesNonExistingIdError")]
    FormLabelForMatchesNonExistingIdError,
    #[serde(rename = "FormInputHasWrongButWellIntendedAutocompleteValueError")]
    FormInputHasWrongButWellIntendedAutocompleteValueError,
    #[serde(rename = "ResponseWasBlockedByORB")]
    ResponseWasBlockedByOrb,
    #[serde(rename = "NavigationEntryMarkedSkippable")]
    NavigationEntryMarkedSkippable,
    #[serde(rename = "AutofillAndManualTextPolicyControlledFeaturesInfo")]
    AutofillAndManualTextPolicyControlledFeaturesInfo,
    #[serde(rename = "AutofillPolicyControlledFeatureInfo")]
    AutofillPolicyControlledFeatureInfo,
    #[serde(rename = "ManualTextPolicyControlledFeatureInfo")]
    ManualTextPolicyControlledFeatureInfo,
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
pub enum PermissionElementIssueType {
    #[serde(rename = "InvalidType")]
    InvalidType,
    #[serde(rename = "FencedFrameDisallowed")]
    FencedFrameDisallowed,
    #[serde(rename = "CspFrameAncestorsMissing")]
    CspFrameAncestorsMissing,
    #[serde(rename = "PermissionsPolicyBlocked")]
    PermissionsPolicyBlocked,
    #[serde(rename = "PaddingRightUnsupported")]
    PaddingRightUnsupported,
    #[serde(rename = "PaddingBottomUnsupported")]
    PaddingBottomUnsupported,
    #[serde(rename = "InsetBoxShadowUnsupported")]
    InsetBoxShadowUnsupported,
    #[serde(rename = "RequestInProgress")]
    RequestInProgress,
    #[serde(rename = "UntrustedEvent")]
    UntrustedEvent,
    #[serde(rename = "RegistrationFailed")]
    RegistrationFailed,
    #[serde(rename = "TypeNotSupported")]
    TypeNotSupported,
    #[serde(rename = "InvalidTypeActivation")]
    InvalidTypeActivation,
    #[serde(rename = "SecurityChecksFailed")]
    SecurityChecksFailed,
    #[serde(rename = "ActivationDisabled")]
    ActivationDisabled,
    #[serde(rename = "GeolocationDeprecated")]
    GeolocationDeprecated,
    #[serde(rename = "InvalidDisplayStyle")]
    InvalidDisplayStyle,
    #[serde(rename = "NonOpaqueColor")]
    NonOpaqueColor,
    #[serde(rename = "LowContrast")]
    LowContrast,
    #[serde(rename = "FontSizeTooSmall")]
    FontSizeTooSmall,
    #[serde(rename = "FontSizeTooLarge")]
    FontSizeTooLarge,
    #[serde(rename = "InvalidSizeValue")]
    InvalidSizeValue,
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
    #[serde(rename = "ConnectionAllowlistIssue")]
    ConnectionAllowlistIssue,
    #[serde(rename = "UserReidentificationIssue")]
    UserReidentificationIssue,
    #[serde(rename = "PermissionElementIssue")]
    PermissionElementIssue,
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
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "camelCase")]
#[doc = "Information about a cookie that is affected by an inspector issue."]
pub struct AffectedCookie {
    #[serde(default)]
    #[doc = "The following three properties uniquely identify a cookie"]
    pub name: String,
    #[serde(default)]
    pub path: String,
    #[serde(default)]
    pub domain: String,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "camelCase")]
#[doc = "Information about a request that is affected by an inspector issue."]
pub struct AffectedRequest {
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[doc = "The unique request id."]
    pub request_id: Option<network::RequestId>,
    #[serde(default)]
    pub url: String,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "camelCase")]
#[doc = "Information about the frame affected by an inspector issue."]
pub struct AffectedFrame {
    pub frame_id: page::FrameId,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "camelCase")]
#[doc = "Information about the suggested solution to a cookie issue."]
pub struct CookieIssueInsight {
    pub r#type: InsightType,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[doc = "Link to table entry in third-party cookie migration readiness list."]
    pub table_entry_url: Option<String>,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "camelCase")]
#[doc = "This information is currently necessary, as the front-end has a difficult\n time finding a specific cookie. With this, we can convey specific error\n information without the cookie."]
pub struct CookieIssueDetails {
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[doc = "If AffectedCookie is not set then rawCookieLine contains the raw\n Set-Cookie header string. This hints at a problem where the\n cookie line is syntactically or semantically malformed in a way\n that no valid cookie could be created."]
    pub cookie: Option<AffectedCookie>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub raw_cookie_line: Option<String>,
    pub cookie_warning_reasons: Vec<CookieWarningReason>,
    pub cookie_exclusion_reasons: Vec<CookieExclusionReason>,
    #[doc = "Optionally identifies the site-for-cookies and the cookie url, which\n may be used by the front-end as additional context."]
    pub operation: CookieOperation,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub site_for_cookies: Option<String>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub cookie_url: Option<String>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request: Option<AffectedRequest>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[doc = "The recommended solution to the issue."]
    pub insight: Option<CookieIssueInsight>,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "camelCase")]
pub struct MixedContentIssueDetails {
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[doc = "The type of resource causing the mixed content issue (css, js, iframe,\n form,...). Marked as optional because it is mapped to from\n blink::mojom::RequestContextType, which will be replaced\n by network::mojom::RequestDestination"]
    pub resource_type: Option<MixedContentResourceType>,
    #[doc = "The way the mixed content issue is being resolved."]
    pub resolution_status: MixedContentResolutionStatus,
    #[serde(default)]
    #[doc = "The unsafe http url causing the mixed content issue."]
    #[serde(rename = "insecureURL")]
    pub insecure_url: String,
    #[serde(default)]
    #[doc = "The url responsible for the call to an unsafe url."]
    #[serde(rename = "mainResourceURL")]
    pub main_resource_url: String,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[doc = "The mixed content request.\n Does not always exist (e.g. for unsafe form submission urls)."]
    pub request: Option<AffectedRequest>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[doc = "Optional because not every mixed content issue is necessarily linked to a frame."]
    pub frame: Option<AffectedFrame>,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "camelCase")]
#[doc = "Details for a request that has been blocked with the BLOCKED_BY_RESPONSE\n code. Currently only used for COEP/COOP, but may be extended to include\n some CSP errors in the future."]
pub struct BlockedByResponseIssueDetails {
    pub request: AffectedRequest,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent_frame: Option<AffectedFrame>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub blocked_frame: Option<AffectedFrame>,
    pub reason: BlockedByResponseReason,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "camelCase")]
pub struct HeavyAdIssueDetails {
    #[doc = "The resolution status, either blocking the content or warning."]
    pub resolution: HeavyAdResolutionStatus,
    #[doc = "The reason the ad was blocked, total network or cpu or peak cpu."]
    pub reason: HeavyAdReason,
    #[doc = "The frame that was blocked."]
    pub frame: AffectedFrame,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "camelCase")]
pub struct SourceCodeLocation {
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub script_id: Option<runtime::ScriptId>,
    #[serde(default)]
    pub url: String,
    #[serde(default)]
    pub line_number: JsUInt,
    #[serde(default)]
    pub column_number: JsUInt,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "camelCase")]
pub struct ContentSecurityPolicyIssueDetails {
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[doc = "The url not included in allowed sources."]
    #[serde(rename = "blockedURL")]
    pub blocked_url: Option<String>,
    #[serde(default)]
    #[doc = "Specific directive that is violated, causing the CSP issue."]
    pub violated_directive: String,
    #[serde(default)]
    pub is_report_only: bool,
    pub content_security_policy_violation_type: ContentSecurityPolicyViolationType,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub frame_ancestor: Option<AffectedFrame>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_code_location: Option<SourceCodeLocation>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub violating_node_id: Option<dom::BackendNodeId>,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "camelCase")]
#[doc = "Details for a issue arising from an SAB being instantiated in, or\n transferred to a context that is not cross-origin isolated."]
pub struct SharedArrayBufferIssueDetails {
    pub source_code_location: SourceCodeLocation,
    #[serde(default)]
    pub is_warning: bool,
    pub r#type: SharedArrayBufferIssueType,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "camelCase")]
pub struct LowTextContrastIssueDetails {
    pub violating_node_id: dom::BackendNodeId,
    #[serde(default)]
    pub violating_node_selector: String,
    #[serde(default)]
    pub contrast_ratio: JsFloat,
    #[serde(default)]
    #[serde(rename = "thresholdAA")]
    pub threshold_aa: JsFloat,
    #[serde(default)]
    #[serde(rename = "thresholdAAA")]
    pub threshold_aaa: JsFloat,
    #[serde(default)]
    pub font_size: String,
    #[serde(default)]
    pub font_weight: String,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "camelCase")]
#[doc = "Details for a CORS related issue, e.g. a warning or error related to\n CORS RFC1918 enforcement."]
pub struct CorsIssueDetails {
    pub cors_error_status: network::CorsErrorStatus,
    #[serde(default)]
    pub is_warning: bool,
    pub request: AffectedRequest,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location: Option<SourceCodeLocation>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub initiator_origin: Option<String>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "resourceIPAddressSpace")]
    pub resource_ip_address_space: Option<network::IpAddressSpace>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_security_state: Option<network::ClientSecurityState>,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "camelCase")]
#[doc = "Details for issues around \"Attribution Reporting API\" usage.\n Explainer: https://github.com/WICG/attribution-reporting-api"]
pub struct AttributionReportingIssueDetails {
    pub violation_type: AttributionReportingIssueType,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request: Option<AffectedRequest>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub violating_node_id: Option<dom::BackendNodeId>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub invalid_parameter: Option<String>,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "camelCase")]
#[doc = "Details for issues about documents in Quirks Mode\n or Limited Quirks Mode that affects page layouting."]
pub struct QuirksModeIssueDetails {
    #[serde(default)]
    #[doc = "If false, it means the document's mode is \"quirks\"\n instead of \"limited-quirks\"."]
    pub is_limited_quirks_mode: bool,
    pub document_node_id: dom::BackendNodeId,
    #[serde(default)]
    pub url: String,
    pub frame_id: page::FrameId,
    pub loader_id: network::LoaderId,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "camelCase")]
#[deprecated]
pub struct NavigatorUserAgentIssueDetails {
    #[serde(default)]
    pub url: String,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location: Option<SourceCodeLocation>,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "camelCase")]
pub struct SharedDictionaryIssueDetails {
    pub shared_dictionary_error: SharedDictionaryError,
    pub request: AffectedRequest,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "camelCase")]
pub struct SriMessageSignatureIssueDetails {
    pub error: SriMessageSignatureError,
    #[serde(default)]
    pub signature_base: String,
    #[serde(default)]
    pub integrity_assertions: Vec<String>,
    pub request: AffectedRequest,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "camelCase")]
pub struct UnencodedDigestIssueDetails {
    pub error: UnencodedDigestError,
    pub request: AffectedRequest,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "camelCase")]
pub struct ConnectionAllowlistIssueDetails {
    pub error: ConnectionAllowlistError,
    pub request: AffectedRequest,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "camelCase")]
#[doc = "Depending on the concrete errorType, different properties are set."]
pub struct GenericIssueDetails {
    #[doc = "Issues with the same errorType are aggregated in the frontend."]
    pub error_type: GenericIssueErrorType,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub frame_id: Option<page::FrameId>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub violating_node_id: Option<dom::BackendNodeId>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub violating_node_attribute: Option<String>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request: Option<AffectedRequest>,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "camelCase")]
#[doc = "This issue tracks information needed to print a deprecation message.\n https://source.chromium.org/chromium/chromium/src/+/main:third_party/blink/renderer/core/frame/third_party/blink/renderer/core/frame/deprecation/README.md"]
pub struct DeprecationIssueDetails {
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub affected_frame: Option<AffectedFrame>,
    pub source_code_location: SourceCodeLocation,
    #[serde(default)]
    #[doc = "One of the deprecation names from third_party/blink/renderer/core/frame/deprecation/deprecation.json5"]
    pub r#type: String,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "camelCase")]
#[doc = "This issue warns about sites in the redirect chain of a finished navigation\n that may be flagged as trackers and have their state cleared if they don't\n receive a user interaction. Note that in this context 'site' means eTLD+1.\n For example, if the URL `https://example.test:80/bounce` was in the\n redirect chain, the site reported would be `example.test`."]
pub struct BounceTrackingIssueDetails {
    #[serde(default)]
    pub tracking_sites: Vec<String>,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "camelCase")]
#[doc = "This issue warns about third-party sites that are accessing cookies on the\n current page, and have been permitted due to having a global metadata grant.\n Note that in this context 'site' means eTLD+1. For example, if the URL\n `https://example.test:80/web_page` was accessing cookies, the site reported\n would be `example.test`."]
pub struct CookieDeprecationMetadataIssueDetails {
    #[serde(default)]
    pub allowed_sites: Vec<String>,
    #[serde(default)]
    pub opt_out_percentage: JsFloat,
    #[serde(default)]
    pub is_opt_out_top_level: bool,
    pub operation: CookieOperation,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "camelCase")]
pub struct FederatedAuthRequestIssueDetails {
    pub federated_auth_request_issue_reason: FederatedAuthRequestIssueReason,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "camelCase")]
pub struct FederatedAuthUserInfoRequestIssueDetails {
    pub federated_auth_user_info_request_issue_reason: FederatedAuthUserInfoRequestIssueReason,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "camelCase")]
#[doc = "This issue tracks client hints related issues. It's used to deprecate old\n features, encourage the use of new ones, and provide general guidance."]
pub struct ClientHintIssueDetails {
    pub source_code_location: SourceCodeLocation,
    pub client_hint_issue_reason: ClientHintIssueReason,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "camelCase")]
pub struct FailedRequestInfo {
    #[serde(default)]
    #[doc = "The URL that failed to load."]
    pub url: String,
    #[serde(default)]
    #[doc = "The failure message for the failed request."]
    pub failure_message: String,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<network::RequestId>,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "camelCase")]
pub struct PartitioningBlobUrlIssueDetails {
    #[serde(default)]
    #[doc = "The BlobURL that failed to load."]
    pub url: String,
    #[doc = "Additional information about the Partitioning Blob URL issue."]
    #[serde(rename = "partitioningBlobURLInfo")]
    pub partitioning_blob_url_info: PartitioningBlobUrlInfo,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "camelCase")]
#[doc = "This issue warns about errors in the select or summary element content model."]
pub struct ElementAccessibilityIssueDetails {
    pub node_id: dom::BackendNodeId,
    pub element_accessibility_issue_reason: ElementAccessibilityIssueReason,
    #[serde(default)]
    pub has_disallowed_attributes: bool,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "camelCase")]
#[doc = "This issue warns when a referenced stylesheet couldn't be loaded."]
pub struct StylesheetLoadingIssueDetails {
    #[doc = "Source code position that referenced the failing stylesheet."]
    pub source_code_location: SourceCodeLocation,
    #[doc = "Reason why the stylesheet couldn't be loaded."]
    pub style_sheet_loading_issue_reason: StyleSheetLoadingIssueReason,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[doc = "Contains additional info when the failure was due to a request."]
    pub failed_request_info: Option<FailedRequestInfo>,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "camelCase")]
#[doc = "This issue warns about errors in property rules that lead to property\n registrations being ignored."]
pub struct PropertyRuleIssueDetails {
    #[doc = "Source code position of the property rule."]
    pub source_code_location: SourceCodeLocation,
    #[doc = "Reason why the property rule was discarded."]
    pub property_rule_issue_reason: PropertyRuleIssueReason,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[doc = "The value of the property rule property that failed to parse"]
    pub property_value: Option<String>,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "camelCase")]
#[doc = "This issue warns about uses of APIs that may be considered misuse to\n re-identify users."]
pub struct UserReidentificationIssueDetails {
    pub r#type: UserReidentificationIssueType,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[doc = "Applies to BlockedFrameNavigation and BlockedSubresource issue types."]
    pub request: Option<AffectedRequest>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[doc = "Applies to NoisedCanvasReadback issue type."]
    pub source_code_location: Option<SourceCodeLocation>,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "camelCase")]
#[doc = "This issue warns about improper usage of the <permission> element."]
pub struct PermissionElementIssueDetails {
    pub issue_type: PermissionElementIssueType,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[doc = "The value of the type attribute."]
    pub r#type: Option<String>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[doc = "The node ID of the <permission> element."]
    pub node_id: Option<dom::BackendNodeId>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[doc = "True if the issue is a warning, false if it is an error."]
    pub is_warning: Option<bool>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[doc = "Fields for message construction:\n Used for messages that reference a specific permission name"]
    pub permission_name: Option<String>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[doc = "Used for messages about occlusion"]
    pub occluder_node_info: Option<String>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[doc = "Used for messages about occluder's parent"]
    pub occluder_parent_node_info: Option<String>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[doc = "Used for messages about activation disabled reason"]
    pub disable_reason: Option<String>,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "camelCase")]
#[doc = "This struct holds a list of optional fields with additional information\n specific to the kind of issue. When adding a new issue code, please also\n add a new optional field to this type."]
pub struct InspectorIssueDetails {
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cookie_issue_details: Option<CookieIssueDetails>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mixed_content_issue_details: Option<MixedContentIssueDetails>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub blocked_by_response_issue_details: Option<BlockedByResponseIssueDetails>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub heavy_ad_issue_details: Option<HeavyAdIssueDetails>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content_security_policy_issue_details: Option<ContentSecurityPolicyIssueDetails>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shared_array_buffer_issue_details: Option<SharedArrayBufferIssueDetails>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub low_text_contrast_issue_details: Option<LowTextContrastIssueDetails>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cors_issue_details: Option<CorsIssueDetails>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attribution_reporting_issue_details: Option<AttributionReportingIssueDetails>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quirks_mode_issue_details: Option<QuirksModeIssueDetails>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "partitioningBlobURLIssueDetails")]
    pub partitioning_blob_url_issue_details: Option<PartitioningBlobUrlIssueDetails>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[deprecated]
    pub navigator_user_agent_issue_details: Option<NavigatorUserAgentIssueDetails>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub generic_issue_details: Option<GenericIssueDetails>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deprecation_issue_details: Option<DeprecationIssueDetails>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_hint_issue_details: Option<ClientHintIssueDetails>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub federated_auth_request_issue_details: Option<FederatedAuthRequestIssueDetails>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bounce_tracking_issue_details: Option<BounceTrackingIssueDetails>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cookie_deprecation_metadata_issue_details: Option<CookieDeprecationMetadataIssueDetails>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stylesheet_loading_issue_details: Option<StylesheetLoadingIssueDetails>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub property_rule_issue_details: Option<PropertyRuleIssueDetails>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub federated_auth_user_info_request_issue_details:
        Option<FederatedAuthUserInfoRequestIssueDetails>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shared_dictionary_issue_details: Option<SharedDictionaryIssueDetails>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub element_accessibility_issue_details: Option<ElementAccessibilityIssueDetails>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sri_message_signature_issue_details: Option<SriMessageSignatureIssueDetails>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unencoded_digest_issue_details: Option<UnencodedDigestIssueDetails>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connection_allowlist_issue_details: Option<ConnectionAllowlistIssueDetails>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_reidentification_issue_details: Option<UserReidentificationIssueDetails>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub permission_element_issue_details: Option<PermissionElementIssueDetails>,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "camelCase")]
#[doc = "An inspector issue reported from the back-end."]
pub struct InspectorIssue {
    pub code: InspectorIssueCode,
    pub details: InspectorIssueDetails,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[doc = "A unique id for this issue. May be omitted if no other entity (e.g.\n exception, CDP message, etc.) is referencing this issue."]
    pub issue_id: Option<IssueId>,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "camelCase")]
#[doc = "Returns the response body and size if it were re-encoded with the specified settings. Only\n applies to images."]
pub struct GetEncodedResponse {
    #[doc = "Identifier of the network request to get content for."]
    pub request_id: network::RequestId,
    #[doc = "The encoding to use."]
    pub encoding: GetEncodedResponseEncodingOption,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[doc = "The quality of the encoding (0-1). (defaults to 1)"]
    pub quality: Option<JsFloat>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[doc = "Whether to only return the size information (defaults to false)."]
    pub size_only: Option<bool>,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct Disable(pub Option<Json>);
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct Enable(pub Option<Json>);
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "camelCase")]
#[doc = "Runs the contrast check for the target page. Found issues are reported\n using Audits.issueAdded event."]
pub struct CheckContrast {
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[doc = "Whether to report WCAG AAA level issues. Default is false."]
    #[serde(rename = "reportAAA")]
    pub report_aaa: Option<bool>,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct CheckFormsIssues(pub Option<Json>);
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[serde(rename_all = "camelCase")]
#[doc = "Returns the response body and size if it were re-encoded with the specified settings. Only\n applies to images."]
pub struct GetEncodedResponseReturnObject {
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[doc = "The encoded body as a base64 string. Omitted if sizeOnly is true."]
    pub body: Option<Vec<u8>>,
    #[serde(default)]
    #[doc = "Size before re-encoding."]
    pub original_size: JsUInt,
    #[serde(default)]
    #[doc = "Size after re-encoding."]
    pub encoded_size: JsUInt,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[doc = "Disables issues domain, prevents further issues from being reported to the client."]
pub struct DisableReturnObject(pub Option<Json>);
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[doc = "Enables issues domain, sends the issues collected so far to the client by means of the\n `issueAdded` event."]
pub struct EnableReturnObject(pub Option<Json>);
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[doc = "Runs the contrast check for the target page. Found issues are reported\n using Audits.issueAdded event."]
pub struct CheckContrastReturnObject(pub Option<Json>);
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[serde(rename_all = "camelCase")]
#[doc = "Runs the form issues check for the target page. Found issues are reported\n using Audits.issueAdded event."]
pub struct CheckFormsIssuesReturnObject {
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
    use derive_builder::Builder;
    #[allow(unused_imports)]
    use serde::{Deserialize, Serialize};
    #[allow(unused_imports)]
    use serde_json::Value as Json;
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
    pub struct IssueAddedEvent {
        pub params: IssueAddedEventParams,
    }
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
    pub struct IssueAddedEventParams {
        pub issue: super::InspectorIssue,
    }
}
