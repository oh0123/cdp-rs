//! Core protocol helpers for translating raw Chrome DevTools
//! Protocol (CDP) messages into strongly typed structures.

use std::fmt;

use crate::error::CdpError;
use cdp_protocol::{
    dom as dom_cdp, fetch as fetch_cdp, inspector as inspector_cdp, log as log_cdp,
    network as network_cdp, page as page_cdp, runtime as runtime_cdp, target as target_cdp,
    tracing as tracing_cdp,
};
use serde::Deserialize;
use serde_json::Value;

/// Raw response payload returned by the DevTools protocol.
///
/// # Examples
/// ```ignore
/// let response: CdpResponse = serde_json::from_str(raw_json)?;
/// assert_eq!(response.id, 42);
/// ```
#[derive(Deserialize, Debug)]
pub struct CdpResponse {
    pub id: u32,
    pub result: Option<Value>,
    pub error: Option<Value>,
}

/// Raw event payload emitted by the DevTools protocol.
///
/// # Examples
/// ```ignore
/// let event: CdpEventMessage = serde_json::from_str(raw_json)?;
/// println!("received {}", event.method);
/// ```
#[derive(Deserialize, Debug, Clone)]
pub struct CdpEventMessage {
    pub method: String,
    #[serde(default)]
    pub params: Value,
    #[serde(rename = "sessionId")]
    pub session_id: Option<String>,
}

impl fmt::Display for CdpEventMessage {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "CdpEventMessage {{ method: {}, params: {:?}, session_id: {:?} }}",
            self.method, self.params, self.session_id
        )
    }
}

/// Strongly typed events that flow through the transport layer.
#[derive(Clone, Debug)]
pub enum CdpEvent {
    /// A parsed CDP event mapped to the strongly typed `cdp_protocol` model.
    Event(Box<cdp_protocol::types::Event>),
    /// Fallback wrapper that carries the unparsed event payload verbatim.
    UnhnownEvent(CdpEventMessage),
}

macro_rules! impl_try_from_cdp_event {
    ($inner_variant:path, $type:ty) => {
        impl TryFrom<CdpEvent> for $type {
            type Error = CdpError;

            fn try_from(value: CdpEvent) -> Result<Self, Self::Error> {
                if let CdpEvent::Event(event) = value
                    && let $inner_variant(inner_event) = *event
                {
                    return Ok(inner_event);
                }
                Err(CdpError::protocol(format!(
                    "Mismatched event type for {}",
                    stringify!($inner_variant)
                )))
            }
        }
    };
}

// Apply the conversion helper macro to frequently used event kinds.
impl_try_from_cdp_event!(
    cdp_protocol::types::Event::DOMAttributeModified,
    dom_cdp::events::AttributeModifiedEvent
);

impl_try_from_cdp_event!(
    cdp_protocol::types::Event::DOMAttributeRemoved,
    dom_cdp::events::AttributeRemovedEvent
);

impl_try_from_cdp_event!(
    cdp_protocol::types::Event::DOMCharacterDataModified,
    dom_cdp::events::CharacterDataModifiedEvent
);

impl_try_from_cdp_event!(
    cdp_protocol::types::Event::DOMChildNodeCountUpdated,
    dom_cdp::events::ChildNodeCountUpdatedEvent
);

impl_try_from_cdp_event!(
    cdp_protocol::types::Event::DOMChildNodeInserted,
    dom_cdp::events::ChildNodeInsertedEvent
);

impl_try_from_cdp_event!(
    cdp_protocol::types::Event::DOMChildNodeRemoved,
    dom_cdp::events::ChildNodeRemovedEvent
);

impl_try_from_cdp_event!(
    cdp_protocol::types::Event::DOMDocumentUpdated,
    dom_cdp::events::DocumentUpdatedEvent
);

impl_try_from_cdp_event!(
    cdp_protocol::types::Event::DOMSetChildNodes,
    dom_cdp::events::SetChildNodesEvent
);

impl_try_from_cdp_event!(
    cdp_protocol::types::Event::DOMShadowRootPopped,
    dom_cdp::events::ShadowRootPoppedEvent
);

impl_try_from_cdp_event!(
    cdp_protocol::types::Event::DOMShadowRootPushed,
    dom_cdp::events::ShadowRootPushedEvent
);

impl_try_from_cdp_event!(
    cdp_protocol::types::Event::FetchAuthRequired,
    fetch_cdp::events::AuthRequiredEvent
);

impl_try_from_cdp_event!(
    cdp_protocol::types::Event::FetchRequestPaused,
    fetch_cdp::events::RequestPausedEvent
);

impl_try_from_cdp_event!(
    cdp_protocol::types::Event::InspectorDetached,
    inspector_cdp::events::DetachedEvent
);

impl_try_from_cdp_event!(
    cdp_protocol::types::Event::InspectorTargetCrashed,
    inspector_cdp::events::TargetCrashedEvent
);

impl_try_from_cdp_event!(
    cdp_protocol::types::Event::InspectorTargetReloadedAfterCrash,
    inspector_cdp::events::TargetReloadedAfterCrashEvent
);

impl_try_from_cdp_event!(
    cdp_protocol::types::Event::LogEntryAdded,
    log_cdp::events::EntryAddedEvent
);

impl_try_from_cdp_event!(
    cdp_protocol::types::Event::NetworkDataReceived,
    network_cdp::events::DataReceivedEvent
);

impl_try_from_cdp_event!(
    cdp_protocol::types::Event::NetworkEventSourceMessageReceived,
    network_cdp::events::EventSourceMessageReceivedEvent
);

impl_try_from_cdp_event!(
    cdp_protocol::types::Event::NetworkLoadingFailed,
    network_cdp::events::LoadingFailedEvent
);

impl_try_from_cdp_event!(
    cdp_protocol::types::Event::NetworkLoadingFinished,
    network_cdp::events::LoadingFinishedEvent
);

impl_try_from_cdp_event!(
    cdp_protocol::types::Event::NetworkRequestServedFromCache,
    network_cdp::events::RequestServedFromCacheEvent
);

impl_try_from_cdp_event!(
    cdp_protocol::types::Event::NetworkRequestWillBeSent,
    network_cdp::events::RequestWillBeSentEvent
);

impl_try_from_cdp_event!(
    cdp_protocol::types::Event::NetworkRequestWillBeSentExtraInfo,
    network_cdp::events::RequestWillBeSentExtraInfoEvent
);

impl_try_from_cdp_event!(
    cdp_protocol::types::Event::NetworkResponseReceived,
    network_cdp::events::ResponseReceivedEvent
);

impl_try_from_cdp_event!(
    cdp_protocol::types::Event::NetworkResponseReceivedEarlyHints,
    network_cdp::events::ResponseReceivedEarlyHintsEvent
);

impl_try_from_cdp_event!(
    cdp_protocol::types::Event::NetworkResponseReceivedExtraInfo,
    network_cdp::events::ResponseReceivedExtraInfoEvent
);

impl_try_from_cdp_event!(
    cdp_protocol::types::Event::NetworkWebSocketClosed,
    network_cdp::events::WebSocketClosedEvent
);

impl_try_from_cdp_event!(
    cdp_protocol::types::Event::NetworkWebSocketCreated,
    network_cdp::events::WebSocketCreatedEvent
);

impl_try_from_cdp_event!(
    cdp_protocol::types::Event::NetworkWebSocketFrameError,
    network_cdp::events::WebSocketFrameErrorEvent
);

impl_try_from_cdp_event!(
    cdp_protocol::types::Event::NetworkWebSocketFrameReceived,
    network_cdp::events::WebSocketFrameReceivedEvent
);

impl_try_from_cdp_event!(
    cdp_protocol::types::Event::NetworkWebSocketFrameSent,
    network_cdp::events::WebSocketFrameSentEvent
);

impl_try_from_cdp_event!(
    cdp_protocol::types::Event::NetworkWebSocketHandshakeResponseReceived,
    network_cdp::events::WebSocketHandshakeResponseReceivedEvent
);

impl_try_from_cdp_event!(
    cdp_protocol::types::Event::NetworkWebSocketWillSendHandshakeRequest,
    network_cdp::events::WebSocketWillSendHandshakeRequestEvent
);

impl_try_from_cdp_event!(
    cdp_protocol::types::Event::PageBackForwardCacheNotUsed,
    page_cdp::events::BackForwardCacheNotUsedEvent
);

impl_try_from_cdp_event!(
    cdp_protocol::types::Event::PageDomContentEventFired,
    page_cdp::events::DomContentEventFiredEvent
);

impl_try_from_cdp_event!(
    cdp_protocol::types::Event::PageDownloadProgress,
    page_cdp::events::DownloadProgressEvent
);

impl_try_from_cdp_event!(
    cdp_protocol::types::Event::PageDownloadWillBegin,
    page_cdp::events::DownloadWillBeginEvent
);

impl_try_from_cdp_event!(
    cdp_protocol::types::Event::PageFileChooserOpened,
    page_cdp::events::FileChooserOpenedEvent
);

impl_try_from_cdp_event!(
    cdp_protocol::types::Event::PageFrameAttached,
    page_cdp::events::FrameAttachedEvent
);

impl_try_from_cdp_event!(
    cdp_protocol::types::Event::PageFrameClearedScheduledNavigation,
    page_cdp::events::FrameClearedScheduledNavigationEvent
);

impl_try_from_cdp_event!(
    cdp_protocol::types::Event::PageFrameDetached,
    page_cdp::events::FrameDetachedEvent
);

impl_try_from_cdp_event!(
    cdp_protocol::types::Event::PageFrameNavigated,
    page_cdp::events::FrameNavigatedEvent
);

impl_try_from_cdp_event!(
    cdp_protocol::types::Event::PageFrameRequestedNavigation,
    page_cdp::events::FrameRequestedNavigationEvent
);

impl_try_from_cdp_event!(
    cdp_protocol::types::Event::PageFrameScheduledNavigation,
    page_cdp::events::FrameScheduledNavigationEvent
);

impl_try_from_cdp_event!(
    cdp_protocol::types::Event::PageFrameStartedLoading,
    page_cdp::events::FrameStartedLoadingEvent
);

impl_try_from_cdp_event!(
    cdp_protocol::types::Event::PageFrameStartedNavigating,
    page_cdp::events::FrameStartedNavigatingEvent
);

impl_try_from_cdp_event!(
    cdp_protocol::types::Event::PageFrameStoppedLoading,
    page_cdp::events::FrameStoppedLoadingEvent
);

impl_try_from_cdp_event!(
    cdp_protocol::types::Event::PageFrameSubtreeWillBeDetached,
    page_cdp::events::FrameSubtreeWillBeDetachedEvent
);

impl_try_from_cdp_event!(
    cdp_protocol::types::Event::PageJavascriptDialogClosed,
    page_cdp::events::JavascriptDialogClosedEvent
);

impl_try_from_cdp_event!(
    cdp_protocol::types::Event::PageJavascriptDialogOpening,
    page_cdp::events::JavascriptDialogOpeningEvent
);

impl_try_from_cdp_event!(
    cdp_protocol::types::Event::PageLifecycleEvent,
    page_cdp::events::LifecycleEventEvent
);

impl_try_from_cdp_event!(
    cdp_protocol::types::Event::PageLoadEventFired,
    page_cdp::events::LoadEventFiredEvent
);

impl_try_from_cdp_event!(
    cdp_protocol::types::Event::PageNavigatedWithinDocument,
    page_cdp::events::NavigatedWithinDocumentEvent
);

impl_try_from_cdp_event!(
    cdp_protocol::types::Event::PageScreencastFrame,
    page_cdp::events::ScreencastFrameEvent
);

impl_try_from_cdp_event!(
    cdp_protocol::types::Event::PageScreencastVisibilityChanged,
    page_cdp::events::ScreencastVisibilityChangedEvent
);

impl_try_from_cdp_event!(
    cdp_protocol::types::Event::PageWindowOpen,
    page_cdp::events::WindowOpenEvent
);

impl_try_from_cdp_event!(
    cdp_protocol::types::Event::RuntimeBindingCalled,
    runtime_cdp::events::BindingCalledEvent
);

impl_try_from_cdp_event!(
    cdp_protocol::types::Event::RuntimeConsoleAPICalled,
    runtime_cdp::events::ConsoleAPICalledEvent
);

impl_try_from_cdp_event!(
    cdp_protocol::types::Event::RuntimeExceptionRevoked,
    runtime_cdp::events::ExceptionRevokedEvent
);

impl_try_from_cdp_event!(
    cdp_protocol::types::Event::RuntimeExceptionThrown,
    runtime_cdp::events::ExceptionThrownEvent
);

impl_try_from_cdp_event!(
    cdp_protocol::types::Event::RuntimeExecutionContextCreated,
    runtime_cdp::events::ExecutionContextCreatedEvent
);

impl_try_from_cdp_event!(
    cdp_protocol::types::Event::RuntimeExecutionContextDestroyed,
    runtime_cdp::events::ExecutionContextDestroyedEvent
);

impl_try_from_cdp_event!(
    cdp_protocol::types::Event::RuntimeExecutionContextsCleared,
    runtime_cdp::events::ExecutionContextsClearedEvent
);

impl_try_from_cdp_event!(
    cdp_protocol::types::Event::RuntimeInspectRequested,
    runtime_cdp::events::InspectRequestedEvent
);

impl_try_from_cdp_event!(
    cdp_protocol::types::Event::AttachedToTarget,
    target_cdp::events::AttachedToTargetEvent
);

impl_try_from_cdp_event!(
    cdp_protocol::types::Event::DetachedFromTarget,
    target_cdp::events::DetachedFromTargetEvent
);

impl_try_from_cdp_event!(
    cdp_protocol::types::Event::ReceivedMessageFromTarget,
    target_cdp::events::ReceivedMessageFromTargetEvent
);

impl_try_from_cdp_event!(
    cdp_protocol::types::Event::TargetCrashed,
    target_cdp::events::TargetCrashedEvent
);

impl_try_from_cdp_event!(
    cdp_protocol::types::Event::TargetCreated,
    target_cdp::events::TargetCreatedEvent
);

impl_try_from_cdp_event!(
    cdp_protocol::types::Event::TargetDestroyed,
    target_cdp::events::TargetDestroyedEvent
);

impl_try_from_cdp_event!(
    cdp_protocol::types::Event::TargetInfoChanged,
    target_cdp::events::TargetInfoChangedEvent
);

impl_try_from_cdp_event!(
    cdp_protocol::types::Event::TracingComplete,
    tracing_cdp::events::TracingCompleteEvent
);

impl_try_from_cdp_event!(
    cdp_protocol::types::Event::TracingBufferUsage,
    tracing_cdp::events::BufferUsageEvent
);

impl_try_from_cdp_event!(
    cdp_protocol::types::Event::TracingDataCollected,
    tracing_cdp::events::DataCollectedEvent
);
