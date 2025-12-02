// Auto-generated from Chrome at version 140.0.7339.186
#[allow(unused)]
use serde::{Deserialize, Serialize};
use std::fmt::Debug;
pub type JsFloat = f64;
pub type JsUInt = u32;
pub type WindowId = JsUInt;
pub type CallId = JsUInt;
#[derive(Serialize, Debug)]
pub struct MethodCall<T>
where
    T: Debug,
{
    #[serde(rename = "method")]
    method_name: &'static str,
    pub id: CallId,
    params: T,
}
impl<T> MethodCall<T>
where
    T: Debug,
{
    pub fn get_params(&self) -> &T {
        &self.params
    }
}
pub trait Method: Debug {
    const NAME: &'static str;
    type ReturnObject: serde::de::DeserializeOwned + std::fmt::Debug;
    fn to_method_call(self, call_id: CallId) -> MethodCall<Self>
    where
        Self: std::marker::Sized,
    {
        MethodCall {
            id: call_id,
            params: self,
            method_name: Self::NAME,
        }
    }
}
#[derive(Deserialize, Debug, Clone, PartialEq)]
#[serde(tag = "method")]
#[allow(clippy::large_enum_variant)]
pub enum Event {
    #[serde(rename = "Accessibility.loadComplete")]
    AccessibilityLoadComplete(super::accessibility::events::LoadCompleteEvent),
    #[serde(rename = "Accessibility.nodesUpdated")]
    AccessibilityNodesUpdated(super::accessibility::events::NodesUpdatedEvent),
    #[serde(rename = "Animation.animationCanceled")]
    AnimationCanceled(super::animation::events::AnimationCanceledEvent),
    #[serde(rename = "Animation.animationCreated")]
    AnimationCreated(super::animation::events::AnimationCreatedEvent),
    #[serde(rename = "Animation.animationStarted")]
    AnimationStarted(super::animation::events::AnimationStartedEvent),
    #[serde(rename = "Animation.animationUpdated")]
    AnimationUpdated(super::animation::events::AnimationUpdatedEvent),
    #[serde(rename = "Audits.issueAdded")]
    AuditsIssueAdded(super::audits::events::IssueAddedEvent),
    #[serde(rename = "Autofill.addressFormFilled")]
    AutofillAddressFormFilled(super::autofill::events::AddressFormFilledEvent),
    #[serde(rename = "BackgroundService.recordingStateChanged")]
    BackgroundServiceRecordingStateChanged(
        super::background_service::events::RecordingStateChangedEvent,
    ),
    #[serde(rename = "BackgroundService.backgroundServiceEventReceived")]
    BackgroundServiceEventReceived(
        super::background_service::events::BackgroundServiceEventReceivedEvent,
    ),
    #[serde(rename = "Browser.downloadWillBegin")]
    BrowserDownloadWillBegin(super::browser::events::DownloadWillBeginEvent),
    #[serde(rename = "Browser.downloadProgress")]
    BrowserDownloadProgress(super::browser::events::DownloadProgressEvent),
    #[serde(rename = "CSS.fontsUpdated")]
    CSSFontsUpdated(super::css::events::FontsUpdatedEvent),
    #[serde(rename = "CSS.mediaQueryResultChanged")]
    CSSMediaQueryResultChanged(super::css::events::MediaQueryResultChangedEvent),
    #[serde(rename = "CSS.styleSheetAdded")]
    CSSStyleSheetAdded(super::css::events::StyleSheetAddedEvent),
    #[serde(rename = "CSS.styleSheetChanged")]
    CSSStyleSheetChanged(super::css::events::StyleSheetChangedEvent),
    #[serde(rename = "CSS.styleSheetRemoved")]
    CSSStyleSheetRemoved(super::css::events::StyleSheetRemovedEvent),
    #[serde(rename = "CSS.computedStyleUpdated")]
    CSSComputedStyleUpdated(super::css::events::ComputedStyleUpdatedEvent),
    #[serde(rename = "Cast.sinksUpdated")]
    CastSinksUpdated(super::cast::events::SinksUpdatedEvent),
    #[serde(rename = "Cast.issueUpdated")]
    CastIssueUpdated(super::cast::events::IssueUpdatedEvent),
    #[serde(rename = "DOM.attributeModified")]
    DOMAttributeModified(super::dom::events::AttributeModifiedEvent),
    #[serde(rename = "DOM.attributeRemoved")]
    DOMAttributeRemoved(super::dom::events::AttributeRemovedEvent),
    #[serde(rename = "DOM.characterDataModified")]
    DOMCharacterDataModified(super::dom::events::CharacterDataModifiedEvent),
    #[serde(rename = "DOM.childNodeCountUpdated")]
    DOMChildNodeCountUpdated(super::dom::events::ChildNodeCountUpdatedEvent),
    #[serde(rename = "DOM.childNodeInserted")]
    DOMChildNodeInserted(super::dom::events::ChildNodeInsertedEvent),
    #[serde(rename = "DOM.childNodeRemoved")]
    DOMChildNodeRemoved(super::dom::events::ChildNodeRemovedEvent),
    #[serde(rename = "DOM.distributedNodesUpdated")]
    DOMDistributedNodesUpdated(super::dom::events::DistributedNodesUpdatedEvent),
    #[serde(rename = "DOM.documentUpdated")]
    DOMDocumentUpdated(super::dom::events::DocumentUpdatedEvent),
    #[serde(rename = "DOM.inlineStyleInvalidated")]
    DOMInlineStyleInvalidated(super::dom::events::InlineStyleInvalidatedEvent),
    #[serde(rename = "DOM.pseudoElementAdded")]
    DOMPseudoElementAdded(super::dom::events::PseudoElementAddedEvent),
    #[serde(rename = "DOM.topLayerElementsUpdated")]
    DOMTopLayerElementsUpdated(super::dom::events::TopLayerElementsUpdatedEvent),
    #[serde(rename = "DOM.scrollableFlagUpdated")]
    DOMScrollableFlagUpdated(super::dom::events::ScrollableFlagUpdatedEvent),
    #[serde(rename = "DOM.pseudoElementRemoved")]
    DOMPseudoElementRemoved(super::dom::events::PseudoElementRemovedEvent),
    #[serde(rename = "DOM.setChildNodes")]
    DOMSetChildNodes(super::dom::events::SetChildNodesEvent),
    #[serde(rename = "DOM.shadowRootPopped")]
    DOMShadowRootPopped(super::dom::events::ShadowRootPoppedEvent),
    #[serde(rename = "DOM.shadowRootPushed")]
    DOMShadowRootPushed(super::dom::events::ShadowRootPushedEvent),
    #[serde(rename = "DOMStorage.domStorageItemAdded")]
    DOMStorageDomStorageItemAdded(super::dom_storage::events::DomStorageItemAddedEvent),
    #[serde(rename = "DOMStorage.domStorageItemRemoved")]
    DOMStorageDomStorageItemRemoved(super::dom_storage::events::DomStorageItemRemovedEvent),
    #[serde(rename = "DOMStorage.domStorageItemUpdated")]
    DOMStorageDomStorageItemUpdated(super::dom_storage::events::DomStorageItemUpdatedEvent),
    #[serde(rename = "DOMStorage.domStorageItemsCleared")]
    DOMStorageDomStorageItemsCleared(super::dom_storage::events::DomStorageItemsClearedEvent),
    #[serde(rename = "Emulation.virtualTimeBudgetExpired")]
    EmulationVirtualTimeBudgetExpired(super::emulation::events::VirtualTimeBudgetExpiredEvent),
    #[serde(rename = "Input.dragIntercepted")]
    InputDragIntercepted(super::input::events::DragInterceptedEvent),
    #[serde(rename = "Inspector.detached")]
    InspectorDetached(super::inspector::events::DetachedEvent),
    #[serde(rename = "Inspector.targetCrashed")]
    InspectorTargetCrashed(super::inspector::events::TargetCrashedEvent),
    #[serde(rename = "Inspector.targetReloadedAfterCrash")]
    InspectorTargetReloadedAfterCrash(super::inspector::events::TargetReloadedAfterCrashEvent),
    #[serde(rename = "LayerTree.layerPainted")]
    LayerTreeLayerPainted(super::layer_tree::events::LayerPaintedEvent),
    #[serde(rename = "LayerTree.layerTreeDidChange")]
    LayerTreeDidChange(super::layer_tree::events::LayerTreeDidChangeEvent),
    #[serde(rename = "Log.entryAdded")]
    LogEntryAdded(super::log::events::EntryAddedEvent),
    #[serde(rename = "Network.dataReceived")]
    NetworkDataReceived(super::network::events::DataReceivedEvent),
    #[serde(rename = "Network.eventSourceMessageReceived")]
    NetworkEventSourceMessageReceived(super::network::events::EventSourceMessageReceivedEvent),
    #[serde(rename = "Network.loadingFailed")]
    NetworkLoadingFailed(super::network::events::LoadingFailedEvent),
    #[serde(rename = "Network.loadingFinished")]
    NetworkLoadingFinished(super::network::events::LoadingFinishedEvent),
    #[serde(rename = "Network.requestIntercepted")]
    NetworkRequestIntercepted(super::network::events::RequestInterceptedEvent),
    #[serde(rename = "Network.requestServedFromCache")]
    NetworkRequestServedFromCache(super::network::events::RequestServedFromCacheEvent),
    #[serde(rename = "Network.requestWillBeSent")]
    NetworkRequestWillBeSent(super::network::events::RequestWillBeSentEvent),
    #[serde(rename = "Network.resourceChangedPriority")]
    NetworkResourceChangedPriority(super::network::events::ResourceChangedPriorityEvent),
    #[serde(rename = "Network.signedExchangeReceived")]
    NetworkSignedExchangeReceived(super::network::events::SignedExchangeReceivedEvent),
    #[serde(rename = "Network.responseReceived")]
    NetworkResponseReceived(super::network::events::ResponseReceivedEvent),
    #[serde(rename = "Network.webSocketClosed")]
    NetworkWebSocketClosed(super::network::events::WebSocketClosedEvent),
    #[serde(rename = "Network.webSocketCreated")]
    NetworkWebSocketCreated(super::network::events::WebSocketCreatedEvent),
    #[serde(rename = "Network.webSocketFrameError")]
    NetworkWebSocketFrameError(super::network::events::WebSocketFrameErrorEvent),
    #[serde(rename = "Network.webSocketFrameReceived")]
    NetworkWebSocketFrameReceived(super::network::events::WebSocketFrameReceivedEvent),
    #[serde(rename = "Network.webSocketFrameSent")]
    NetworkWebSocketFrameSent(super::network::events::WebSocketFrameSentEvent),
    #[serde(rename = "Network.webSocketHandshakeResponseReceived")]
    NetworkWebSocketHandshakeResponseReceived(
        super::network::events::WebSocketHandshakeResponseReceivedEvent,
    ),
    #[serde(rename = "Network.webSocketWillSendHandshakeRequest")]
    NetworkWebSocketWillSendHandshakeRequest(
        super::network::events::WebSocketWillSendHandshakeRequestEvent,
    ),
    #[serde(rename = "Network.webTransportCreated")]
    NetworkWebTransportCreated(super::network::events::WebTransportCreatedEvent),
    #[serde(rename = "Network.webTransportConnectionEstablished")]
    NetworkWebTransportConnectionEstablished(
        super::network::events::WebTransportConnectionEstablishedEvent,
    ),
    #[serde(rename = "Network.webTransportClosed")]
    NetworkWebTransportClosed(super::network::events::WebTransportClosedEvent),
    #[serde(rename = "Network.directTCPSocketCreated")]
    NetworkDirectTCPSocketCreated(super::network::events::DirectTCPSocketCreatedEvent),
    #[serde(rename = "Network.directTCPSocketOpened")]
    NetworkDirectTCPSocketOpened(super::network::events::DirectTCPSocketOpenedEvent),
    #[serde(rename = "Network.directTCPSocketAborted")]
    NetworkDirectTCPSocketAborted(super::network::events::DirectTCPSocketAbortedEvent),
    #[serde(rename = "Network.directTCPSocketClosed")]
    NetworkDirectTCPSocketClosed(super::network::events::DirectTCPSocketClosedEvent),
    #[serde(rename = "Network.directTCPSocketChunkSent")]
    NetworkDirectTCPSocketChunkSent(super::network::events::DirectTCPSocketChunkSentEvent),
    #[serde(rename = "Network.directTCPSocketChunkReceived")]
    NetworkDirectTCPSocketChunkReceived(super::network::events::DirectTCPSocketChunkReceivedEvent),
    #[serde(rename = "Network.directUDPSocketCreated")]
    NetworkDirectUDPSocketCreated(super::network::events::DirectUDPSocketCreatedEvent),
    #[serde(rename = "Network.directUDPSocketOpened")]
    NetworkDirectUDPSocketOpened(super::network::events::DirectUDPSocketOpenedEvent),
    #[serde(rename = "Network.directUDPSocketAborted")]
    NetworkDirectUDPSocketAborted(super::network::events::DirectUDPSocketAbortedEvent),
    #[serde(rename = "Network.directUDPSocketClosed")]
    NetworkDirectUDPSocketClosed(super::network::events::DirectUDPSocketClosedEvent),
    #[serde(rename = "Network.directUDPSocketChunkSent")]
    NetworkDirectUDPSocketChunkSent(super::network::events::DirectUDPSocketChunkSentEvent),
    #[serde(rename = "Network.directUDPSocketChunkReceived")]
    NetworkDirectUDPSocketChunkReceived(super::network::events::DirectUDPSocketChunkReceivedEvent),
    #[serde(rename = "Network.requestWillBeSentExtraInfo")]
    NetworkRequestWillBeSentExtraInfo(super::network::events::RequestWillBeSentExtraInfoEvent),
    #[serde(rename = "Network.responseReceivedExtraInfo")]
    NetworkResponseReceivedExtraInfo(super::network::events::ResponseReceivedExtraInfoEvent),
    #[serde(rename = "Network.responseReceivedEarlyHints")]
    NetworkResponseReceivedEarlyHints(super::network::events::ResponseReceivedEarlyHintsEvent),
    #[serde(rename = "Network.trustTokenOperationDone")]
    NetworkTrustTokenOperationDone(super::network::events::TrustTokenOperationDoneEvent),
    #[serde(rename = "Network.policyUpdated")]
    NetworkPolicyUpdated(super::network::events::PolicyUpdatedEvent),
    #[serde(rename = "Network.subresourceWebBundleMetadataReceived")]
    NetworkSubresourceWebBundleMetadataReceived(
        super::network::events::SubresourceWebBundleMetadataReceivedEvent,
    ),
    #[serde(rename = "Network.subresourceWebBundleMetadataError")]
    NetworkSubresourceWebBundleMetadataError(
        super::network::events::SubresourceWebBundleMetadataErrorEvent,
    ),
    #[serde(rename = "Network.subresourceWebBundleInnerResponseParsed")]
    NetworkSubresourceWebBundleInnerResponseParsed(
        super::network::events::SubresourceWebBundleInnerResponseParsedEvent,
    ),
    #[serde(rename = "Network.subresourceWebBundleInnerResponseError")]
    NetworkSubresourceWebBundleInnerResponseError(
        super::network::events::SubresourceWebBundleInnerResponseErrorEvent,
    ),
    #[serde(rename = "Network.reportingApiReportAdded")]
    NetworkReportingApiReportAdded(super::network::events::ReportingApiReportAddedEvent),
    #[serde(rename = "Network.reportingApiReportUpdated")]
    NetworkReportingApiReportUpdated(super::network::events::ReportingApiReportUpdatedEvent),
    #[serde(rename = "Network.reportingApiEndpointsChangedForOrigin")]
    NetworkReportingApiEndpointsChangedForOrigin(
        super::network::events::ReportingApiEndpointsChangedForOriginEvent,
    ),
    #[serde(rename = "Overlay.inspectNodeRequested")]
    OverlayInspectNodeRequested(super::overlay::events::InspectNodeRequestedEvent),
    #[serde(rename = "Overlay.nodeHighlightRequested")]
    OverlayNodeHighlightRequested(super::overlay::events::NodeHighlightRequestedEvent),
    #[serde(rename = "Overlay.screenshotRequested")]
    OverlayScreenshotRequested(super::overlay::events::ScreenshotRequestedEvent),
    #[serde(rename = "Overlay.inspectModeCanceled")]
    OverlayInspectModeCanceled(super::overlay::events::InspectModeCanceledEvent),
    #[serde(rename = "Page.domContentEventFired")]
    PageDomContentEventFired(super::page::events::DomContentEventFiredEvent),
    #[serde(rename = "Page.fileChooserOpened")]
    PageFileChooserOpened(super::page::events::FileChooserOpenedEvent),
    #[serde(rename = "Page.frameAttached")]
    PageFrameAttached(super::page::events::FrameAttachedEvent),
    #[serde(rename = "Page.frameClearedScheduledNavigation")]
    PageFrameClearedScheduledNavigation(super::page::events::FrameClearedScheduledNavigationEvent),
    #[serde(rename = "Page.frameDetached")]
    PageFrameDetached(super::page::events::FrameDetachedEvent),
    #[serde(rename = "Page.frameSubtreeWillBeDetached")]
    PageFrameSubtreeWillBeDetached(super::page::events::FrameSubtreeWillBeDetachedEvent),
    #[serde(rename = "Page.frameNavigated")]
    PageFrameNavigated(super::page::events::FrameNavigatedEvent),
    #[serde(rename = "Page.documentOpened")]
    PageDocumentOpened(super::page::events::DocumentOpenedEvent),
    #[serde(rename = "Page.frameResized")]
    PageFrameResized(super::page::events::FrameResizedEvent),
    #[serde(rename = "Page.frameStartedNavigating")]
    PageFrameStartedNavigating(super::page::events::FrameStartedNavigatingEvent),
    #[serde(rename = "Page.frameRequestedNavigation")]
    PageFrameRequestedNavigation(super::page::events::FrameRequestedNavigationEvent),
    #[serde(rename = "Page.frameScheduledNavigation")]
    PageFrameScheduledNavigation(super::page::events::FrameScheduledNavigationEvent),
    #[serde(rename = "Page.frameStartedLoading")]
    PageFrameStartedLoading(super::page::events::FrameStartedLoadingEvent),
    #[serde(rename = "Page.frameStoppedLoading")]
    PageFrameStoppedLoading(super::page::events::FrameStoppedLoadingEvent),
    #[serde(rename = "Page.downloadWillBegin")]
    PageDownloadWillBegin(super::page::events::DownloadWillBeginEvent),
    #[serde(rename = "Page.downloadProgress")]
    PageDownloadProgress(super::page::events::DownloadProgressEvent),
    #[serde(rename = "Page.interstitialHidden")]
    PageInterstitialHidden(super::page::events::InterstitialHiddenEvent),
    #[serde(rename = "Page.interstitialShown")]
    PageInterstitialShown(super::page::events::InterstitialShownEvent),
    #[serde(rename = "Page.javascriptDialogClosed")]
    PageJavascriptDialogClosed(super::page::events::JavascriptDialogClosedEvent),
    #[serde(rename = "Page.javascriptDialogOpening")]
    PageJavascriptDialogOpening(super::page::events::JavascriptDialogOpeningEvent),
    #[serde(rename = "Page.lifecycleEvent")]
    PageLifecycleEvent(super::page::events::LifecycleEventEvent),
    #[serde(rename = "Page.backForwardCacheNotUsed")]
    PageBackForwardCacheNotUsed(super::page::events::BackForwardCacheNotUsedEvent),
    #[serde(rename = "Page.loadEventFired")]
    PageLoadEventFired(super::page::events::LoadEventFiredEvent),
    #[serde(rename = "Page.navigatedWithinDocument")]
    PageNavigatedWithinDocument(super::page::events::NavigatedWithinDocumentEvent),
    #[serde(rename = "Page.screencastFrame")]
    PageScreencastFrame(super::page::events::ScreencastFrameEvent),
    #[serde(rename = "Page.screencastVisibilityChanged")]
    PageScreencastVisibilityChanged(super::page::events::ScreencastVisibilityChangedEvent),
    #[serde(rename = "Page.windowOpen")]
    PageWindowOpen(super::page::events::WindowOpenEvent),
    #[serde(rename = "Page.compilationCacheProduced")]
    PageCompilationCacheProduced(super::page::events::CompilationCacheProducedEvent),
    #[serde(rename = "Performance.metrics")]
    PerformanceMetrics(super::performance::events::MetricsEvent),
    #[serde(rename = "PerformanceTimeline.timelineEventAdded")]
    PerformanceTimelineTimelineEventAdded(
        super::performance_timeline::events::TimelineEventAddedEvent,
    ),
    #[serde(rename = "Security.certificateError")]
    SecurityCertificateError(super::security::events::CertificateErrorEvent),
    #[serde(rename = "Security.visibleSecurityStateChanged")]
    VisibleSecurityStateChanged(super::security::events::VisibleSecurityStateChangedEvent),
    #[serde(rename = "Security.securityStateChanged")]
    SecurityStateChanged(super::security::events::SecurityStateChangedEvent),
    #[serde(rename = "ServiceWorker.workerErrorReported")]
    ServiceWorkerWorkerErrorReported(super::service_worker::events::WorkerErrorReportedEvent),
    #[serde(rename = "ServiceWorker.workerRegistrationUpdated")]
    ServiceWorkerWorkerRegistrationUpdated(
        super::service_worker::events::WorkerRegistrationUpdatedEvent,
    ),
    #[serde(rename = "ServiceWorker.workerVersionUpdated")]
    ServiceWorkerWorkerVersionUpdated(super::service_worker::events::WorkerVersionUpdatedEvent),
    #[serde(rename = "Storage.cacheStorageContentUpdated")]
    CacheStorageContentUpdated(super::storage::events::CacheStorageContentUpdatedEvent),
    #[serde(rename = "Storage.cacheStorageListUpdated")]
    CacheStorageListUpdated(super::storage::events::CacheStorageListUpdatedEvent),
    #[serde(rename = "Storage.indexedDBContentUpdated")]
    StorageIndexedDBContentUpdated(super::storage::events::IndexedDBContentUpdatedEvent),
    #[serde(rename = "Storage.indexedDBListUpdated")]
    StorageIndexedDBListUpdated(super::storage::events::IndexedDBListUpdatedEvent),
    #[serde(rename = "Storage.interestGroupAccessed")]
    StorageInterestGroupAccessed(super::storage::events::InterestGroupAccessedEvent),
    #[serde(rename = "Storage.interestGroupAuctionEventOccurred")]
    StorageInterestGroupAuctionEventOccurred(
        super::storage::events::InterestGroupAuctionEventOccurredEvent,
    ),
    #[serde(rename = "Storage.interestGroupAuctionNetworkRequestCreated")]
    StorageInterestGroupAuctionNetworkRequestCreated(
        super::storage::events::InterestGroupAuctionNetworkRequestCreatedEvent,
    ),
    #[serde(rename = "Storage.sharedStorageAccessed")]
    SharedStorageAccessed(super::storage::events::SharedStorageAccessedEvent),
    #[serde(rename = "Storage.sharedStorageWorkletOperationExecutionFinished")]
    SharedStorageWorkletOperationExecutionFinished(
        super::storage::events::SharedStorageWorkletOperationExecutionFinishedEvent,
    ),
    #[serde(rename = "Storage.storageBucketCreatedOrUpdated")]
    StorageBucketCreatedOrUpdated(super::storage::events::StorageBucketCreatedOrUpdatedEvent),
    #[serde(rename = "Storage.storageBucketDeleted")]
    StorageBucketDeleted(super::storage::events::StorageBucketDeletedEvent),
    #[serde(rename = "Storage.attributionReportingSourceRegistered")]
    StorageAttributionReportingSourceRegistered(
        super::storage::events::AttributionReportingSourceRegisteredEvent,
    ),
    #[serde(rename = "Storage.attributionReportingTriggerRegistered")]
    StorageAttributionReportingTriggerRegistered(
        super::storage::events::AttributionReportingTriggerRegisteredEvent,
    ),
    #[serde(rename = "Storage.attributionReportingReportSent")]
    StorageAttributionReportingReportSent(
        super::storage::events::AttributionReportingReportSentEvent,
    ),
    #[serde(rename = "Storage.attributionReportingVerboseDebugReportSent")]
    StorageAttributionReportingVerboseDebugReportSent(
        super::storage::events::AttributionReportingVerboseDebugReportSentEvent,
    ),
    #[serde(rename = "Target.attachedToTarget")]
    AttachedToTarget(super::target::events::AttachedToTargetEvent),
    #[serde(rename = "Target.detachedFromTarget")]
    DetachedFromTarget(super::target::events::DetachedFromTargetEvent),
    #[serde(rename = "Target.receivedMessageFromTarget")]
    ReceivedMessageFromTarget(super::target::events::ReceivedMessageFromTargetEvent),
    #[serde(rename = "Target.targetCreated")]
    TargetCreated(super::target::events::TargetCreatedEvent),
    #[serde(rename = "Target.targetDestroyed")]
    TargetDestroyed(super::target::events::TargetDestroyedEvent),
    #[serde(rename = "Target.targetCrashed")]
    TargetCrashed(super::target::events::TargetCrashedEvent),
    #[serde(rename = "Target.targetInfoChanged")]
    TargetInfoChanged(super::target::events::TargetInfoChangedEvent),
    #[serde(rename = "Tethering.accepted")]
    TetheringAccepted(super::tethering::events::AcceptedEvent),
    #[serde(rename = "Tracing.bufferUsage")]
    TracingBufferUsage(super::tracing::events::BufferUsageEvent),
    #[serde(rename = "Tracing.dataCollected")]
    TracingDataCollected(super::tracing::events::DataCollectedEvent),
    #[serde(rename = "Tracing.tracingComplete")]
    TracingComplete(super::tracing::events::TracingCompleteEvent),
    #[serde(rename = "Fetch.requestPaused")]
    FetchRequestPaused(super::fetch::events::RequestPausedEvent),
    #[serde(rename = "Fetch.authRequired")]
    FetchAuthRequired(super::fetch::events::AuthRequiredEvent),
    #[serde(rename = "WebAudio.contextCreated")]
    WebAudioContextCreated(super::web_audio::events::ContextCreatedEvent),
    #[serde(rename = "WebAudio.contextWillBeDestroyed")]
    WebAudioContextWillBeDestroyed(super::web_audio::events::ContextWillBeDestroyedEvent),
    #[serde(rename = "WebAudio.contextChanged")]
    WebAudioContextChanged(super::web_audio::events::ContextChangedEvent),
    #[serde(rename = "WebAudio.audioListenerCreated")]
    WebAudioAudioListenerCreated(super::web_audio::events::AudioListenerCreatedEvent),
    #[serde(rename = "WebAudio.audioListenerWillBeDestroyed")]
    WebAudioAudioListenerWillBeDestroyed(
        super::web_audio::events::AudioListenerWillBeDestroyedEvent,
    ),
    #[serde(rename = "WebAudio.audioNodeCreated")]
    WebAudioAudioNodeCreated(super::web_audio::events::AudioNodeCreatedEvent),
    #[serde(rename = "WebAudio.audioNodeWillBeDestroyed")]
    WebAudioAudioNodeWillBeDestroyed(super::web_audio::events::AudioNodeWillBeDestroyedEvent),
    #[serde(rename = "WebAudio.audioParamCreated")]
    WebAudioAudioParamCreated(super::web_audio::events::AudioParamCreatedEvent),
    #[serde(rename = "WebAudio.audioParamWillBeDestroyed")]
    WebAudioAudioParamWillBeDestroyed(super::web_audio::events::AudioParamWillBeDestroyedEvent),
    #[serde(rename = "WebAudio.nodesConnected")]
    WebAudioNodesConnected(super::web_audio::events::NodesConnectedEvent),
    #[serde(rename = "WebAudio.nodesDisconnected")]
    WebAudioNodesDisconnected(super::web_audio::events::NodesDisconnectedEvent),
    #[serde(rename = "WebAudio.nodeParamConnected")]
    WebAudioNodeParamConnected(super::web_audio::events::NodeParamConnectedEvent),
    #[serde(rename = "WebAudio.nodeParamDisconnected")]
    WebAudioNodeParamDisconnected(super::web_audio::events::NodeParamDisconnectedEvent),
    #[serde(rename = "WebAuthn.credentialAdded")]
    WebAuthnCredentialAdded(super::web_authn::events::CredentialAddedEvent),
    #[serde(rename = "WebAuthn.credentialDeleted")]
    WebAuthnCredentialDeleted(super::web_authn::events::CredentialDeletedEvent),
    #[serde(rename = "WebAuthn.credentialUpdated")]
    WebAuthnCredentialUpdated(super::web_authn::events::CredentialUpdatedEvent),
    #[serde(rename = "WebAuthn.credentialAsserted")]
    WebAuthnCredentialAsserted(super::web_authn::events::CredentialAssertedEvent),
    #[serde(rename = "Media.playerPropertiesChanged")]
    MediaPlayerPropertiesChanged(super::media::events::PlayerPropertiesChangedEvent),
    #[serde(rename = "Media.playerEventsAdded")]
    MediaPlayerEventsAdded(super::media::events::PlayerEventsAddedEvent),
    #[serde(rename = "Media.playerMessagesLogged")]
    MediaPlayerMessagesLogged(super::media::events::PlayerMessagesLoggedEvent),
    #[serde(rename = "Media.playerErrorsRaised")]
    MediaPlayerErrorsRaised(super::media::events::PlayerErrorsRaisedEvent),
    #[serde(rename = "Media.playersCreated")]
    MediaPlayersCreated(super::media::events::PlayersCreatedEvent),
    #[serde(rename = "DeviceAccess.deviceRequestPrompted")]
    DeviceAccessDeviceRequestPrompted(super::device_access::events::DeviceRequestPromptedEvent),
    #[serde(rename = "Preload.ruleSetUpdated")]
    PreloadRuleSetUpdated(super::preload::events::RuleSetUpdatedEvent),
    #[serde(rename = "Preload.ruleSetRemoved")]
    PreloadRuleSetRemoved(super::preload::events::RuleSetRemovedEvent),
    #[serde(rename = "Preload.preloadEnabledStateUpdated")]
    PreloadEnabledStateUpdated(super::preload::events::PreloadEnabledStateUpdatedEvent),
    #[serde(rename = "Preload.prefetchStatusUpdated")]
    PreloadPrefetchStatusUpdated(super::preload::events::PrefetchStatusUpdatedEvent),
    #[serde(rename = "Preload.prerenderStatusUpdated")]
    PreloadPrerenderStatusUpdated(super::preload::events::PrerenderStatusUpdatedEvent),
    #[serde(rename = "Preload.preloadingAttemptSourcesUpdated")]
    PreloadingAttemptSourcesUpdated(super::preload::events::PreloadingAttemptSourcesUpdatedEvent),
    #[serde(rename = "FedCm.dialogShown")]
    FedCmDialogShown(super::fed_cm::events::DialogShownEvent),
    #[serde(rename = "FedCm.dialogClosed")]
    FedCmDialogClosed(super::fed_cm::events::DialogClosedEvent),
    #[serde(rename = "BluetoothEmulation.gattOperationReceived")]
    BluetoothEmulationGattOperationReceived(
        super::bluetooth_emulation::events::GattOperationReceivedEvent,
    ),
    #[serde(rename = "BluetoothEmulation.characteristicOperationReceived")]
    BluetoothEmulationCharacteristicOperationReceived(
        super::bluetooth_emulation::events::CharacteristicOperationReceivedEvent,
    ),
    #[serde(rename = "BluetoothEmulation.descriptorOperationReceived")]
    BluetoothEmulationDescriptorOperationReceived(
        super::bluetooth_emulation::events::DescriptorOperationReceivedEvent,
    ),
    #[serde(rename = "Console.messageAdded")]
    ConsoleMessageAdded(super::console::events::MessageAddedEvent),
    #[serde(rename = "Debugger.breakpointResolved")]
    DebuggerBreakpointResolved(super::debugger::events::BreakpointResolvedEvent),
    #[serde(rename = "Debugger.paused")]
    DebuggerPaused(super::debugger::events::PausedEvent),
    #[serde(rename = "Debugger.resumed")]
    DebuggerResumed(super::debugger::events::ResumedEvent),
    #[serde(rename = "Debugger.scriptFailedToParse")]
    DebuggerScriptFailedToParse(super::debugger::events::ScriptFailedToParseEvent),
    #[serde(rename = "Debugger.scriptParsed")]
    DebuggerScriptParsed(super::debugger::events::ScriptParsedEvent),
    #[serde(rename = "HeapProfiler.addHeapSnapshotChunk")]
    HeapProfilerAddHeapSnapshotChunk(super::heap_profiler::events::AddHeapSnapshotChunkEvent),
    #[serde(rename = "HeapProfiler.heapStatsUpdate")]
    HeapProfilerHeapStatsUpdate(super::heap_profiler::events::HeapStatsUpdateEvent),
    #[serde(rename = "HeapProfiler.lastSeenObjectId")]
    HeapProfilerLastSeenObjectId(super::heap_profiler::events::LastSeenObjectIdEvent),
    #[serde(rename = "HeapProfiler.reportHeapSnapshotProgress")]
    HeapProfilerReportHeapSnapshotProgress(
        super::heap_profiler::events::ReportHeapSnapshotProgressEvent,
    ),
    #[serde(rename = "HeapProfiler.resetProfiles")]
    HeapProfilerResetProfiles(super::heap_profiler::events::ResetProfilesEvent),
    #[serde(rename = "Profiler.consoleProfileFinished")]
    ProfilerConsoleProfileFinished(super::profiler::events::ConsoleProfileFinishedEvent),
    #[serde(rename = "Profiler.consoleProfileStarted")]
    ProfilerConsoleProfileStarted(super::profiler::events::ConsoleProfileStartedEvent),
    #[serde(rename = "Profiler.preciseCoverageDeltaUpdate")]
    ProfilerPreciseCoverageDeltaUpdate(super::profiler::events::PreciseCoverageDeltaUpdateEvent),
    #[serde(rename = "Runtime.bindingCalled")]
    RuntimeBindingCalled(super::runtime::events::BindingCalledEvent),
    #[serde(rename = "Runtime.consoleAPICalled")]
    RuntimeConsoleAPICalled(super::runtime::events::ConsoleAPICalledEvent),
    #[serde(rename = "Runtime.exceptionRevoked")]
    RuntimeExceptionRevoked(super::runtime::events::ExceptionRevokedEvent),
    #[serde(rename = "Runtime.exceptionThrown")]
    RuntimeExceptionThrown(super::runtime::events::ExceptionThrownEvent),
    #[serde(rename = "Runtime.executionContextCreated")]
    RuntimeExecutionContextCreated(super::runtime::events::ExecutionContextCreatedEvent),
    #[serde(rename = "Runtime.executionContextDestroyed")]
    RuntimeExecutionContextDestroyed(super::runtime::events::ExecutionContextDestroyedEvent),
    #[serde(rename = "Runtime.executionContextsCleared")]
    RuntimeExecutionContextsCleared(super::runtime::events::ExecutionContextsClearedEvent),
    #[serde(rename = "Runtime.inspectRequested")]
    RuntimeInspectRequested(super::runtime::events::InspectRequestedEvent),
}
