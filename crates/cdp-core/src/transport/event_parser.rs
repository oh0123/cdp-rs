use super::cdp_protocol::{CdpEvent, CdpEventMessage};
use crate::error::Result;
use cdp_protocol::{dom, network, page, runtime, types::Event};

/// Parses a raw CDP event message into a strongly typed [`CdpEvent`].
pub(crate) fn parse_cdp_event(event_msg: &CdpEventMessage) -> Result<CdpEvent> {
    match event_msg.method.as_str() {
        // Runtime events
        "Runtime.executionContextCreated" => parse_runtime_event(event_msg),
        "Runtime.executionContextDestroyed" => parse_runtime_event(event_msg),
        "Runtime.executionContextsCleared" => parse_runtime_event(event_msg),

        // Page events
        "Page.frameAttached" => parse_page_event(event_msg),
        "Page.frameDetached" => parse_page_event(event_msg),
        "Page.frameNavigated" => parse_page_event(event_msg),
        "Page.loadEventFired" => parse_page_event(event_msg),

        // DOM events
        "DOM.childNodeInserted" => parse_dom_event(event_msg),
        "DOM.childNodeRemoved" => parse_dom_event(event_msg),
        "DOM.attributeModified" => parse_dom_event(event_msg),
        "DOM.attributeRemoved" => parse_dom_event(event_msg),
        "DOM.characterDataModified" => parse_dom_event(event_msg),

        // Network events
        "Network.requestWillBeSent" => parse_network_event(event_msg),
        "Network.responseReceived" => parse_network_event(event_msg),
        "Network.loadingFinished" => parse_network_event(event_msg),
        "Network.loadingFailed" => parse_network_event(event_msg),
        "Network.requestServedFromCache" => parse_network_event(event_msg),
        "Network.dataReceived" => parse_network_event(event_msg),
        "Network.requestWillBeSentExtraInfo" => parse_network_event(event_msg),
        "Network.responseReceivedExtraInfo" => parse_network_event(event_msg),

        // Fallback
        _ => Ok(CdpEvent::UnhnownEvent(event_msg.clone())),
    }
}

fn parse_runtime_event(event_msg: &CdpEventMessage) -> Result<CdpEvent> {
    match event_msg.method.as_str() {
        "Runtime.executionContextCreated" => {
            let params: runtime::events::ExecutionContextCreatedEventParams =
                serde_json::from_value(event_msg.params.clone())?;
            let event = runtime::events::ExecutionContextCreatedEvent { params };
            tracing::debug!("Parsed Runtime.executionContextCreated event");
            Ok(CdpEvent::Event(Box::new(Event::RuntimeExecutionContextCreated(
                event,
            ))))
        }
        "Runtime.executionContextDestroyed" => {
            let params: runtime::events::ExecutionContextDestroyedEventParams =
                serde_json::from_value(event_msg.params.clone())?;
            let event = runtime::events::ExecutionContextDestroyedEvent { params };
            tracing::debug!("Parsed Runtime.executionContextDestroyed event");
            Ok(CdpEvent::Event(Box::new(Event::RuntimeExecutionContextDestroyed(
                event,
            ))))
        }
        "Runtime.executionContextsCleared" => {
            let params: Option<serde_json::Value> =
                serde_json::from_value(event_msg.params.clone())?;
            let event = runtime::events::ExecutionContextsClearedEvent(params);
            tracing::debug!("Parsed Runtime.executionContextsCleared event");
            Ok(CdpEvent::Event(Box::new(Event::RuntimeExecutionContextsCleared(
                event,
            ))))
        }
        _ => unreachable!("Runtime event mismatch"),
    }
}

fn parse_page_event(event_msg: &CdpEventMessage) -> Result<CdpEvent> {
    match event_msg.method.as_str() {
        "Page.frameAttached" => {
            let params: page::events::FrameAttachedEventParams =
                serde_json::from_value(event_msg.params.clone())?;
            let event = page::events::FrameAttachedEvent { params };
            tracing::debug!("Parsed Page.frameAttached event");
            Ok(CdpEvent::Event(Box::new(Event::PageFrameAttached(event))))
        }
        "Page.frameDetached" => {
            let params: page::events::FrameDetachedEventParams =
                serde_json::from_value(event_msg.params.clone())?;
            let event = page::events::FrameDetachedEvent { params };
            tracing::debug!("Parsed Page.frameDetached event");
            Ok(CdpEvent::Event(Box::new(Event::PageFrameDetached(event))))
        }
        "Page.frameNavigated" => {
            let params: page::events::FrameNavigatedEventParams =
                serde_json::from_value(event_msg.params.clone())?;
            let event = page::events::FrameNavigatedEvent { params };
            tracing::debug!("Parsed Page.frameNavigated event");
            Ok(CdpEvent::Event(Box::new(Event::PageFrameNavigated(event))))
        }
        "Page.loadEventFired" => {
            let params: page::events::LoadEventFiredEventParams =
                serde_json::from_value(event_msg.params.clone())?;
            let event = page::events::LoadEventFiredEvent { params };
            tracing::debug!("Parsed Page.loadEventFired event");
            Ok(CdpEvent::Event(Box::new(Event::PageLoadEventFired(event))))
        }
        _ => unreachable!("Page event mismatch"),
    }
}

fn parse_dom_event(event_msg: &CdpEventMessage) -> Result<CdpEvent> {
    match event_msg.method.as_str() {
        "DOM.childNodeInserted" => {
            let params: dom::events::ChildNodeInsertedEventParams =
                serde_json::from_value(event_msg.params.clone())?;
            let event = dom::events::ChildNodeInsertedEvent { params };
            tracing::debug!("Parsed DOM.childNodeInserted event");
            Ok(CdpEvent::Event(Box::new(Event::DOMChildNodeInserted(event))))
        }
        "DOM.childNodeRemoved" => {
            let params: dom::events::ChildNodeRemovedEventParams =
                serde_json::from_value(event_msg.params.clone())?;
            let event = dom::events::ChildNodeRemovedEvent { params };
            tracing::debug!("Parsed DOM.childNodeRemoved event");
            Ok(CdpEvent::Event(Box::new(Event::DOMChildNodeRemoved(event))))
        }
        "DOM.attributeModified" => {
            let params: dom::events::AttributeModifiedEventParams =
                serde_json::from_value(event_msg.params.clone())?;
            let event = dom::events::AttributeModifiedEvent { params };
            tracing::debug!("Parsed DOM.attributeModified event");
            Ok(CdpEvent::Event(Box::new(Event::DOMAttributeModified(event))))
        }
        "DOM.attributeRemoved" => {
            let params: dom::events::AttributeRemovedEventParams =
                serde_json::from_value(event_msg.params.clone())?;
            let event = dom::events::AttributeRemovedEvent { params };
            tracing::debug!("Parsed DOM.attributeRemoved event");
            Ok(CdpEvent::Event(Box::new(Event::DOMAttributeRemoved(event))))
        }
        "DOM.characterDataModified" => {
            let params: dom::events::CharacterDataModifiedEventParams =
                serde_json::from_value(event_msg.params.clone())?;
            let event = dom::events::CharacterDataModifiedEvent { params };
            tracing::debug!("Parsed DOM.characterDataModified event");
            Ok(CdpEvent::Event(Box::new(Event::DOMCharacterDataModified(event))))
        }
        _ => unreachable!("DOM event mismatch"),
    }
}

fn parse_network_event(event_msg: &CdpEventMessage) -> Result<CdpEvent> {
    match event_msg.method.as_str() {
        "Network.requestWillBeSent" => {
            let params: network::events::RequestWillBeSentEventParams =
                serde_json::from_value(event_msg.params.clone())?;
            let event = network::events::RequestWillBeSentEvent { params };
            tracing::debug!("Parsed Network.requestWillBeSent event");
            Ok(CdpEvent::Event(Box::new(Event::NetworkRequestWillBeSent(event))))
        }
        "Network.responseReceived" => {
            let params: network::events::ResponseReceivedEventParams =
                serde_json::from_value(event_msg.params.clone())?;
            let event = network::events::ResponseReceivedEvent { params };
            tracing::debug!("Parsed Network.responseReceived event");
            Ok(CdpEvent::Event(Box::new(Event::NetworkResponseReceived(event))))
        }
        "Network.loadingFinished" => {
            let params: network::events::LoadingFinishedEventParams =
                serde_json::from_value(event_msg.params.clone())?;
            let event = network::events::LoadingFinishedEvent { params };
            tracing::debug!("Parsed Network.loadingFinished event");
            Ok(CdpEvent::Event(Box::new(Event::NetworkLoadingFinished(event))))
        }
        "Network.loadingFailed" => {
            let params: network::events::LoadingFailedEventParams =
                serde_json::from_value(event_msg.params.clone())?;
            let event = network::events::LoadingFailedEvent { params };
            tracing::debug!("Parsed Network.loadingFailed event");
            Ok(CdpEvent::Event(Box::new(Event::NetworkLoadingFailed(event))))
        }
        "Network.requestServedFromCache" => {
            let params: network::events::RequestServedFromCacheEventParams =
                serde_json::from_value(event_msg.params.clone())?;
            let event = network::events::RequestServedFromCacheEvent { params };
            tracing::debug!("Parsed Network.requestServedFromCache event");
            Ok(CdpEvent::Event(Box::new(Event::NetworkRequestServedFromCache(event))))
        }
        "Network.dataReceived" => {
            let params: network::events::DataReceivedEventParams =
                serde_json::from_value(event_msg.params.clone())?;
            let event = network::events::DataReceivedEvent { params };
            tracing::debug!("Parsed Network.dataReceived event");
            Ok(CdpEvent::Event(Box::new(Event::NetworkDataReceived(event))))
        }
        "Network.requestWillBeSentExtraInfo" => {
            let params: network::events::RequestWillBeSentExtraInfoEventParams =
                serde_json::from_value(event_msg.params.clone())?;
            let event = network::events::RequestWillBeSentExtraInfoEvent { params };
            tracing::debug!("Parsed Network.requestWillBeSentExtraInfo event");
            Ok(CdpEvent::Event(Box::new(Event::NetworkRequestWillBeSentExtraInfo(
                event,
            ))))
        }
        "Network.responseReceivedExtraInfo" => {
            let params: network::events::ResponseReceivedExtraInfoEventParams =
                serde_json::from_value(event_msg.params.clone())?;
            let event = network::events::ResponseReceivedExtraInfoEvent { params };
            tracing::debug!("Parsed Network.responseReceivedExtraInfo event");
            Ok(CdpEvent::Event(Box::new(Event::NetworkResponseReceivedExtraInfo(
                event,
            ))))
        }
        _ => unreachable!("Network event mismatch"),
    }
}
