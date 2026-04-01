// Auto-generated from Chrome at version 146.0.7680.165 domain: Preload
use super::dom;
use super::network;
#[allow(unused_imports)]
use super::types::*;
#[allow(unused_imports)]
use derive_builder::Builder;
#[allow(unused_imports)]
use serde::{Deserialize, Serialize};
#[allow(unused_imports)]
use serde_json::Value as Json;
pub type RuleSetId = String;
pub type PreloadPipelineId = String;
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub enum RuleSetErrorType {
    #[serde(rename = "SourceIsNotJsonObject")]
    SourceIsNotJsonObject,
    #[serde(rename = "InvalidRulesSkipped")]
    InvalidRulesSkipped,
    #[serde(rename = "InvalidRulesetLevelTag")]
    InvalidRulesetLevelTag,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub enum SpeculationAction {
    #[serde(rename = "Prefetch")]
    Prefetch,
    #[serde(rename = "Prerender")]
    Prerender,
    #[serde(rename = "PrerenderUntilScript")]
    PrerenderUntilScript,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub enum SpeculationTargetHint {
    #[serde(rename = "Blank")]
    Blank,
    #[serde(rename = "Self")]
    CdpSelf,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub enum PrerenderFinalStatus {
    #[serde(rename = "Activated")]
    Activated,
    #[serde(rename = "Destroyed")]
    Destroyed,
    #[serde(rename = "LowEndDevice")]
    LowEndDevice,
    #[serde(rename = "InvalidSchemeRedirect")]
    InvalidSchemeRedirect,
    #[serde(rename = "InvalidSchemeNavigation")]
    InvalidSchemeNavigation,
    #[serde(rename = "NavigationRequestBlockedByCsp")]
    NavigationRequestBlockedByCsp,
    #[serde(rename = "MojoBinderPolicy")]
    MojoBinderPolicy,
    #[serde(rename = "RendererProcessCrashed")]
    RendererProcessCrashed,
    #[serde(rename = "RendererProcessKilled")]
    RendererProcessKilled,
    #[serde(rename = "Download")]
    Download,
    #[serde(rename = "TriggerDestroyed")]
    TriggerDestroyed,
    #[serde(rename = "NavigationNotCommitted")]
    NavigationNotCommitted,
    #[serde(rename = "NavigationBadHttpStatus")]
    NavigationBadHttpStatus,
    #[serde(rename = "ClientCertRequested")]
    ClientCertRequested,
    #[serde(rename = "NavigationRequestNetworkError")]
    NavigationRequestNetworkError,
    #[serde(rename = "CancelAllHostsForTesting")]
    CancelAllHostsForTesting,
    #[serde(rename = "DidFailLoad")]
    DidFailLoad,
    #[serde(rename = "Stop")]
    Stop,
    #[serde(rename = "SslCertificateError")]
    SslCertificateError,
    #[serde(rename = "LoginAuthRequested")]
    LoginAuthRequested,
    #[serde(rename = "UaChangeRequiresReload")]
    UaChangeRequiresReload,
    #[serde(rename = "BlockedByClient")]
    BlockedByClient,
    #[serde(rename = "AudioOutputDeviceRequested")]
    AudioOutputDeviceRequested,
    #[serde(rename = "MixedContent")]
    MixedContent,
    #[serde(rename = "TriggerBackgrounded")]
    TriggerBackgrounded,
    #[serde(rename = "MemoryLimitExceeded")]
    MemoryLimitExceeded,
    #[serde(rename = "DataSaverEnabled")]
    DataSaverEnabled,
    #[serde(rename = "TriggerUrlHasEffectiveUrl")]
    TriggerUrlHasEffectiveUrl,
    #[serde(rename = "ActivatedBeforeStarted")]
    ActivatedBeforeStarted,
    #[serde(rename = "InactivePageRestriction")]
    InactivePageRestriction,
    #[serde(rename = "StartFailed")]
    StartFailed,
    #[serde(rename = "TimeoutBackgrounded")]
    TimeoutBackgrounded,
    #[serde(rename = "CrossSiteRedirectInInitialNavigation")]
    CrossSiteRedirectInInitialNavigation,
    #[serde(rename = "CrossSiteNavigationInInitialNavigation")]
    CrossSiteNavigationInInitialNavigation,
    #[serde(rename = "SameSiteCrossOriginRedirectNotOptInInInitialNavigation")]
    SameSiteCrossOriginRedirectNotOptInInInitialNavigation,
    #[serde(rename = "SameSiteCrossOriginNavigationNotOptInInInitialNavigation")]
    SameSiteCrossOriginNavigationNotOptInInInitialNavigation,
    #[serde(rename = "ActivationNavigationParameterMismatch")]
    ActivationNavigationParameterMismatch,
    #[serde(rename = "ActivatedInBackground")]
    ActivatedInBackground,
    #[serde(rename = "EmbedderHostDisallowed")]
    EmbedderHostDisallowed,
    #[serde(rename = "ActivationNavigationDestroyedBeforeSuccess")]
    ActivationNavigationDestroyedBeforeSuccess,
    #[serde(rename = "TabClosedByUserGesture")]
    TabClosedByUserGesture,
    #[serde(rename = "TabClosedWithoutUserGesture")]
    TabClosedWithoutUserGesture,
    #[serde(rename = "PrimaryMainFrameRendererProcessCrashed")]
    PrimaryMainFrameRendererProcessCrashed,
    #[serde(rename = "PrimaryMainFrameRendererProcessKilled")]
    PrimaryMainFrameRendererProcessKilled,
    #[serde(rename = "ActivationFramePolicyNotCompatible")]
    ActivationFramePolicyNotCompatible,
    #[serde(rename = "PreloadingDisabled")]
    PreloadingDisabled,
    #[serde(rename = "BatterySaverEnabled")]
    BatterySaverEnabled,
    #[serde(rename = "ActivatedDuringMainFrameNavigation")]
    ActivatedDuringMainFrameNavigation,
    #[serde(rename = "PreloadingUnsupportedByWebContents")]
    PreloadingUnsupportedByWebContents,
    #[serde(rename = "CrossSiteRedirectInMainFrameNavigation")]
    CrossSiteRedirectInMainFrameNavigation,
    #[serde(rename = "CrossSiteNavigationInMainFrameNavigation")]
    CrossSiteNavigationInMainFrameNavigation,
    #[serde(rename = "SameSiteCrossOriginRedirectNotOptInInMainFrameNavigation")]
    SameSiteCrossOriginRedirectNotOptInInMainFrameNavigation,
    #[serde(rename = "SameSiteCrossOriginNavigationNotOptInInMainFrameNavigation")]
    SameSiteCrossOriginNavigationNotOptInInMainFrameNavigation,
    #[serde(rename = "MemoryPressureOnTrigger")]
    MemoryPressureOnTrigger,
    #[serde(rename = "MemoryPressureAfterTriggered")]
    MemoryPressureAfterTriggered,
    #[serde(rename = "PrerenderingDisabledByDevTools")]
    PrerenderingDisabledByDevTools,
    #[serde(rename = "SpeculationRuleRemoved")]
    SpeculationRuleRemoved,
    #[serde(rename = "ActivatedWithAuxiliaryBrowsingContexts")]
    ActivatedWithAuxiliaryBrowsingContexts,
    #[serde(rename = "MaxNumOfRunningEagerPrerendersExceeded")]
    MaxNumOfRunningEagerPrerendersExceeded,
    #[serde(rename = "MaxNumOfRunningNonEagerPrerendersExceeded")]
    MaxNumOfRunningNonEagerPrerendersExceeded,
    #[serde(rename = "MaxNumOfRunningEmbedderPrerendersExceeded")]
    MaxNumOfRunningEmbedderPrerendersExceeded,
    #[serde(rename = "PrerenderingUrlHasEffectiveUrl")]
    PrerenderingUrlHasEffectiveUrl,
    #[serde(rename = "RedirectedPrerenderingUrlHasEffectiveUrl")]
    RedirectedPrerenderingUrlHasEffectiveUrl,
    #[serde(rename = "ActivationUrlHasEffectiveUrl")]
    ActivationUrlHasEffectiveUrl,
    #[serde(rename = "JavaScriptInterfaceAdded")]
    JavaScriptInterfaceAdded,
    #[serde(rename = "JavaScriptInterfaceRemoved")]
    JavaScriptInterfaceRemoved,
    #[serde(rename = "AllPrerenderingCanceled")]
    AllPrerenderingCanceled,
    #[serde(rename = "WindowClosed")]
    WindowClosed,
    #[serde(rename = "SlowNetwork")]
    SlowNetwork,
    #[serde(rename = "OtherPrerenderedPageActivated")]
    OtherPrerenderedPageActivated,
    #[serde(rename = "V8OptimizerDisabled")]
    V8OptimizerDisabled,
    #[serde(rename = "PrerenderFailedDuringPrefetch")]
    PrerenderFailedDuringPrefetch,
    #[serde(rename = "BrowsingDataRemoved")]
    BrowsingDataRemoved,
    #[serde(rename = "PrerenderHostReused")]
    PrerenderHostReused,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub enum PreloadingStatus {
    #[serde(rename = "Pending")]
    Pending,
    #[serde(rename = "Running")]
    Running,
    #[serde(rename = "Ready")]
    Ready,
    #[serde(rename = "Success")]
    Success,
    #[serde(rename = "Failure")]
    Failure,
    #[serde(rename = "NotSupported")]
    NotSupported,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub enum PrefetchStatus {
    #[serde(rename = "PrefetchAllowed")]
    PrefetchAllowed,
    #[serde(rename = "PrefetchFailedIneligibleRedirect")]
    PrefetchFailedIneligibleRedirect,
    #[serde(rename = "PrefetchFailedInvalidRedirect")]
    PrefetchFailedInvalidRedirect,
    #[serde(rename = "PrefetchFailedMIMENotSupported")]
    PrefetchFailedMimeNotSupported,
    #[serde(rename = "PrefetchFailedNetError")]
    PrefetchFailedNetError,
    #[serde(rename = "PrefetchFailedNon2XX")]
    PrefetchFailedNon2Xx,
    #[serde(rename = "PrefetchEvictedAfterBrowsingDataRemoved")]
    PrefetchEvictedAfterBrowsingDataRemoved,
    #[serde(rename = "PrefetchEvictedAfterCandidateRemoved")]
    PrefetchEvictedAfterCandidateRemoved,
    #[serde(rename = "PrefetchEvictedForNewerPrefetch")]
    PrefetchEvictedForNewerPrefetch,
    #[serde(rename = "PrefetchHeldback")]
    PrefetchHeldback,
    #[serde(rename = "PrefetchIneligibleRetryAfter")]
    PrefetchIneligibleRetryAfter,
    #[serde(rename = "PrefetchIsPrivacyDecoy")]
    PrefetchIsPrivacyDecoy,
    #[serde(rename = "PrefetchIsStale")]
    PrefetchIsStale,
    #[serde(rename = "PrefetchNotEligibleBrowserContextOffTheRecord")]
    PrefetchNotEligibleBrowserContextOffTheRecord,
    #[serde(rename = "PrefetchNotEligibleDataSaverEnabled")]
    PrefetchNotEligibleDataSaverEnabled,
    #[serde(rename = "PrefetchNotEligibleExistingProxy")]
    PrefetchNotEligibleExistingProxy,
    #[serde(rename = "PrefetchNotEligibleHostIsNonUnique")]
    PrefetchNotEligibleHostIsNonUnique,
    #[serde(rename = "PrefetchNotEligibleNonDefaultStoragePartition")]
    PrefetchNotEligibleNonDefaultStoragePartition,
    #[serde(rename = "PrefetchNotEligibleSameSiteCrossOriginPrefetchRequiredProxy")]
    PrefetchNotEligibleSameSiteCrossOriginPrefetchRequiredProxy,
    #[serde(rename = "PrefetchNotEligibleSchemeIsNotHttps")]
    PrefetchNotEligibleSchemeIsNotHttps,
    #[serde(rename = "PrefetchNotEligibleUserHasCookies")]
    PrefetchNotEligibleUserHasCookies,
    #[serde(rename = "PrefetchNotEligibleUserHasServiceWorker")]
    PrefetchNotEligibleUserHasServiceWorker,
    #[serde(rename = "PrefetchNotEligibleUserHasServiceWorkerNoFetchHandler")]
    PrefetchNotEligibleUserHasServiceWorkerNoFetchHandler,
    #[serde(rename = "PrefetchNotEligibleRedirectFromServiceWorker")]
    PrefetchNotEligibleRedirectFromServiceWorker,
    #[serde(rename = "PrefetchNotEligibleRedirectToServiceWorker")]
    PrefetchNotEligibleRedirectToServiceWorker,
    #[serde(rename = "PrefetchNotEligibleBatterySaverEnabled")]
    PrefetchNotEligibleBatterySaverEnabled,
    #[serde(rename = "PrefetchNotEligiblePreloadingDisabled")]
    PrefetchNotEligiblePreloadingDisabled,
    #[serde(rename = "PrefetchNotFinishedInTime")]
    PrefetchNotFinishedInTime,
    #[serde(rename = "PrefetchNotStarted")]
    PrefetchNotStarted,
    #[serde(rename = "PrefetchNotUsedCookiesChanged")]
    PrefetchNotUsedCookiesChanged,
    #[serde(rename = "PrefetchProxyNotAvailable")]
    PrefetchProxyNotAvailable,
    #[serde(rename = "PrefetchResponseUsed")]
    PrefetchResponseUsed,
    #[serde(rename = "PrefetchSuccessfulButNotUsed")]
    PrefetchSuccessfulButNotUsed,
    #[serde(rename = "PrefetchNotUsedProbeFailed")]
    PrefetchNotUsedProbeFailed,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "camelCase")]
#[doc = "Corresponds to SpeculationRuleSet"]
pub struct RuleSet {
    pub id: RuleSetId,
    #[doc = "Identifies a document which the rule set is associated with."]
    pub loader_id: network::LoaderId,
    #[serde(default)]
    #[doc = "Source text of JSON representing the rule set. If it comes from\n `<script>` tag, it is the textContent of the node. Note that it is\n a JSON for valid case.\n \n See also:\n - https://wicg.github.io/nav-speculation/speculation-rules.html\n - https://github.com/WICG/nav-speculation/blob/main/triggers.md"]
    pub source_text: String,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[doc = "A speculation rule set is either added through an inline\n `<script>` tag or through an external resource via the\n 'Speculation-Rules' HTTP header. For the first case, we include\n the BackendNodeId of the relevant `<script>` tag. For the second\n case, we include the external URL where the rule set was loaded\n from, and also RequestId if Network domain is enabled.\n \n See also:\n - https://wicg.github.io/nav-speculation/speculation-rules.html#speculation-rules-script\n - https://wicg.github.io/nav-speculation/speculation-rules.html#speculation-rules-header"]
    pub backend_node_id: Option<dom::BackendNodeId>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub url: Option<String>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<network::RequestId>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[doc = "Error information\n `errorMessage` is null iff `errorType` is null."]
    pub error_type: Option<RuleSetErrorType>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[doc = "TODO(https://crbug.com/1425354): Replace this property with structured error."]
    #[deprecated]
    pub error_message: Option<String>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[doc = "For more details, see:\n https://github.com/WICG/nav-speculation/blob/main/speculation-rules-tags.md"]
    pub tag: Option<String>,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "camelCase")]
#[doc = "A key that identifies a preloading attempt.\n \n The url used is the url specified by the trigger (i.e. the initial URL), and\n not the final url that is navigated to. For example, prerendering allows\n same-origin main frame navigations during the attempt, but the attempt is\n still keyed with the initial URL."]
pub struct PreloadingAttemptKey {
    pub loader_id: network::LoaderId,
    pub action: SpeculationAction,
    #[serde(default)]
    pub url: String,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_hint: Option<SpeculationTargetHint>,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "camelCase")]
#[doc = "Lists sources for a preloading attempt, specifically the ids of rule sets\n that had a speculation rule that triggered the attempt, and the\n BackendNodeIds of <a href> or <area href> elements that triggered the\n attempt (in the case of attempts triggered by a document rule). It is\n possible for multiple rule sets and links to trigger a single attempt."]
pub struct PreloadingAttemptSource {
    pub key: PreloadingAttemptKey,
    pub rule_set_ids: Vec<RuleSetId>,
    pub node_ids: Vec<dom::BackendNodeId>,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "camelCase")]
#[doc = "Information of headers to be displayed when the header mismatch occurred."]
pub struct PrerenderMismatchedHeaders {
    #[serde(default)]
    pub header_name: String,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub initial_value: Option<String>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub activation_value: Option<String>,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct Enable(pub Option<Json>);
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct Disable(pub Option<Json>);
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct EnableReturnObject(pub Option<Json>);
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct DisableReturnObject(pub Option<Json>);
impl Method for Enable {
    const NAME: &'static str = "Preload.enable";
    type ReturnObject = EnableReturnObject;
}
impl Method for Disable {
    const NAME: &'static str = "Preload.disable";
    type ReturnObject = DisableReturnObject;
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
    pub struct RuleSetUpdatedEvent {
        pub params: RuleSetUpdatedEventParams,
    }
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
    pub struct RuleSetUpdatedEventParams {
        pub rule_set: super::RuleSet,
    }
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
    pub struct RuleSetRemovedEvent {
        pub params: RuleSetRemovedEventParams,
    }
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
    pub struct RuleSetRemovedEventParams {
        pub id: super::RuleSetId,
    }
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
    pub struct PreloadEnabledStateUpdatedEvent {
        pub params: PreloadEnabledStateUpdatedEventParams,
    }
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
    pub struct PreloadEnabledStateUpdatedEventParams {
        #[serde(default)]
        pub disabled_by_preference: bool,
        #[serde(default)]
        pub disabled_by_data_saver: bool,
        #[serde(default)]
        pub disabled_by_battery_saver: bool,
        #[serde(default)]
        pub disabled_by_holdback_prefetch_speculation_rules: bool,
        #[serde(default)]
        pub disabled_by_holdback_prerender_speculation_rules: bool,
    }
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
    pub struct PrefetchStatusUpdatedEvent {
        pub params: PrefetchStatusUpdatedEventParams,
    }
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
    pub struct PrefetchStatusUpdatedEventParams {
        pub key: super::PreloadingAttemptKey,
        pub pipeline_id: super::PreloadPipelineId,
        #[doc = "The frame id of the frame initiating prefetch."]
        pub initiating_frame_id: super::super::page::FrameId,
        #[serde(default)]
        pub prefetch_url: String,
        pub status: super::PreloadingStatus,
        pub prefetch_status: super::PrefetchStatus,
        pub request_id: super::super::network::RequestId,
    }
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
    pub struct PrerenderStatusUpdatedEvent {
        pub params: PrerenderStatusUpdatedEventParams,
    }
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
    pub struct PrerenderStatusUpdatedEventParams {
        pub key: super::PreloadingAttemptKey,
        pub pipeline_id: super::PreloadPipelineId,
        pub status: super::PreloadingStatus,
        #[builder(default)]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub prerender_status: Option<super::PrerenderFinalStatus>,
        #[builder(default)]
        #[serde(skip_serializing_if = "Option::is_none")]
        #[serde(default)]
        #[doc = "This is used to give users more information about the name of Mojo interface\n that is incompatible with prerender and has caused the cancellation of the attempt."]
        pub disallowed_mojo_interface: Option<String>,
        #[builder(default)]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub mismatched_headers: Option<Vec<super::PrerenderMismatchedHeaders>>,
    }
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
    pub struct PreloadingAttemptSourcesUpdatedEvent {
        pub params: PreloadingAttemptSourcesUpdatedEventParams,
    }
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
    pub struct PreloadingAttemptSourcesUpdatedEventParams {
        pub loader_id: super::super::network::LoaderId,
        pub preloading_attempt_sources: Vec<super::PreloadingAttemptSource>,
    }
}
