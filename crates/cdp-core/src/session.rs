use crate::error::Result;
use serde::Deserialize;
use std::{sync::Arc, time::Duration};
use tokio::sync::{broadcast, mpsc};

use crate::transport::{cdp_protocol::CdpEvent, websocket_connection::ConnectionInternals};

// The target enum distinguishes between attached sessions and flattened (direct) connections.
enum SessionTarget {
    Attached { session_id: String },
    Direct,
}

/// Represents a logical CDP session backed by a WebSocket connection.
pub(crate) struct Session {
    target: SessionTarget,
    pub session_id: Option<String>, // `None` for direct connections.
    pub internals: Arc<ConnectionInternals>,
    pub event_bus: broadcast::Sender<CdpEvent>,
}

/// Background task that fans out incoming CDP events to broadcast subscribers.
async fn session_event_broadcaster_task(
    mut receiver: mpsc::Receiver<CdpEvent>,
    sender: broadcast::Sender<CdpEvent>,
) {
    while let Some(event) = receiver.recv().await {
        let _ = sender.send(event);
    }
}

impl Session {
    /// Creates a new session and spawns a broadcaster for domain events.
    pub(crate) fn new(
        session_id: Option<String>,
        connection: Arc<ConnectionInternals>,
        event_receiver: mpsc::Receiver<CdpEvent>,
    ) -> Self {
        let (broadcast_sender, _) = broadcast::channel(crate::DEFAULT_CHANNEL_CAPACITY);
        tokio::spawn(session_event_broadcaster_task(
            event_receiver,
            broadcast_sender.clone(),
        ));
        let target = match session_id.clone() {
            Some(id) => SessionTarget::Attached { session_id: id },
            None => SessionTarget::Direct,
        };
        Self {
            target,
            session_id,
            internals: connection,
            event_bus: broadcast_sender,
        }
    }

    /// Sends a CDP command, attaching the session id when the session is attached.
    pub(crate) async fn send_command<
        M: serde::Serialize + std::fmt::Debug + cdp_protocol::types::Method,
        R: for<'de> Deserialize<'de>,
    >(
        &self,
        method: M,
        timeout: Option<Duration>,
    ) -> Result<R> {
        let session_id = match &self.target {
            SessionTarget::Attached { session_id } => Some(session_id.clone()),
            SessionTarget::Direct => None, // Direct connections don't use the sessionId field
        };
        self.internals.send(method, session_id, timeout).await
    }
}
