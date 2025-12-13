// Auto-generated from Chrome at version 143.0.7499.110 domain: Preload
use super::dom;
use super::network;
#[allow(unused_imports)]
use super::types::*;
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
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct RuleSet {
    #[serde(rename = "id")]
    pub id: RuleSetId,
    #[serde(rename = "loaderId")]
    pub loader_id: network::LoaderId,
    #[serde(default)]
    #[serde(rename = "sourceText")]
    pub source_text: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "backendNodeId")]
    pub backend_node_id: Option<dom::BackendNodeId>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(rename = "url")]
    pub url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "requestId")]
    pub request_id: Option<network::RequestId>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "errorType")]
    pub error_type: Option<RuleSetErrorType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(rename = "errorMessage")]
    pub error_message: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(rename = "tag")]
    pub tag: Option<String>,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct PreloadingAttemptKey {
    #[serde(rename = "loaderId")]
    pub loader_id: network::LoaderId,
    #[serde(rename = "action")]
    pub action: SpeculationAction,
    #[serde(default)]
    #[serde(rename = "url")]
    pub url: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "targetHint")]
    pub target_hint: Option<SpeculationTargetHint>,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct PreloadingAttemptSource {
    #[serde(rename = "key")]
    pub key: PreloadingAttemptKey,
    #[serde(rename = "ruleSetIds")]
    pub rule_set_ids: Vec<RuleSetId>,
    #[serde(rename = "nodeIds")]
    pub node_ids: Vec<dom::BackendNodeId>,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct PrerenderMismatchedHeaders {
    #[serde(default)]
    #[serde(rename = "headerName")]
    pub header_name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(rename = "initialValue")]
    pub initial_value: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(rename = "activationValue")]
    pub activation_value: Option<String>,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Enable(pub Option<serde_json::Value>);
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Disable(pub Option<serde_json::Value>);
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct EnableReturnObject {}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct DisableReturnObject {}
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
    use serde::{Deserialize, Serialize};
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
    pub struct RuleSetUpdatedEvent {
        pub params: RuleSetUpdatedEventParams,
    }
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
    pub struct RuleSetUpdatedEventParams {
        #[serde(rename = "ruleSet")]
        pub rule_set: super::RuleSet,
    }
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
    pub struct RuleSetRemovedEvent {
        pub params: RuleSetRemovedEventParams,
    }
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
    pub struct RuleSetRemovedEventParams {
        #[serde(rename = "id")]
        pub id: super::RuleSetId,
    }
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
    pub struct PreloadEnabledStateUpdatedEvent {
        pub params: PreloadEnabledStateUpdatedEventParams,
    }
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
    pub struct PreloadEnabledStateUpdatedEventParams {
        #[serde(default)]
        #[serde(rename = "disabledByPreference")]
        pub disabled_by_preference: bool,
        #[serde(default)]
        #[serde(rename = "disabledByDataSaver")]
        pub disabled_by_data_saver: bool,
        #[serde(default)]
        #[serde(rename = "disabledByBatterySaver")]
        pub disabled_by_battery_saver: bool,
        #[serde(default)]
        #[serde(rename = "disabledByHoldbackPrefetchSpeculationRules")]
        pub disabled_by_holdback_prefetch_speculation_rules: bool,
        #[serde(default)]
        #[serde(rename = "disabledByHoldbackPrerenderSpeculationRules")]
        pub disabled_by_holdback_prerender_speculation_rules: bool,
    }
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
    pub struct PrefetchStatusUpdatedEvent {
        pub params: PrefetchStatusUpdatedEventParams,
    }
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
    pub struct PrefetchStatusUpdatedEventParams {
        #[serde(rename = "key")]
        pub key: super::PreloadingAttemptKey,
        #[serde(rename = "pipelineId")]
        pub pipeline_id: super::PreloadPipelineId,
        #[serde(rename = "initiatingFrameId")]
        pub initiating_frame_id: super::super::page::FrameId,
        #[serde(default)]
        #[serde(rename = "prefetchUrl")]
        pub prefetch_url: String,
        #[serde(rename = "status")]
        pub status: super::PreloadingStatus,
        #[serde(rename = "prefetchStatus")]
        pub prefetch_status: super::PrefetchStatus,
        #[serde(rename = "requestId")]
        pub request_id: super::super::network::RequestId,
    }
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
    pub struct PrerenderStatusUpdatedEvent {
        pub params: PrerenderStatusUpdatedEventParams,
    }
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
    pub struct PrerenderStatusUpdatedEventParams {
        #[serde(rename = "key")]
        pub key: super::PreloadingAttemptKey,
        #[serde(rename = "pipelineId")]
        pub pipeline_id: super::PreloadPipelineId,
        #[serde(rename = "status")]
        pub status: super::PreloadingStatus,
        #[serde(skip_serializing_if = "Option::is_none")]
        #[serde(rename = "prerenderStatus")]
        pub prerender_status: Option<super::PrerenderFinalStatus>,
        #[serde(skip_serializing_if = "Option::is_none")]
        #[serde(default)]
        #[serde(rename = "disallowedMojoInterface")]
        pub disallowed_mojo_interface: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        #[serde(rename = "mismatchedHeaders")]
        pub mismatched_headers: Option<Vec<super::PrerenderMismatchedHeaders>>,
    }
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
    pub struct PreloadingAttemptSourcesUpdatedEvent {
        pub params: PreloadingAttemptSourcesUpdatedEventParams,
    }
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
    pub struct PreloadingAttemptSourcesUpdatedEventParams {
        #[serde(rename = "loaderId")]
        pub loader_id: super::super::network::LoaderId,
        #[serde(rename = "preloadingAttemptSources")]
        pub preloading_attempt_sources: Vec<super::PreloadingAttemptSource>,
    }
}
