//! Core protocol helpers for translating raw Chrome DevTools
//! Protocol (CDP) messages into strongly typed structures.

use std::fmt;

use crate::error::CdpError;
use cdp_protocol::{page as page_cdp, tracing as tracing_cdp};
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
    cdp_protocol::types::Event::PageLoadEventFired,
    page_cdp::events::LoadEventFiredEvent
);

impl_try_from_cdp_event!(
    cdp_protocol::types::Event::PageFileChooserOpened,
    page_cdp::events::FileChooserOpenedEvent
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
