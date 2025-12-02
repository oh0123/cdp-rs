use super::cdp_protocol::*;
use super::event_parser;
use crate::error::{CdpError, Result};
use crate::network::network_intercept;
use crate::page::Page;
use futures_util::{
    SinkExt,
    stream::{SplitSink, SplitStream, StreamExt},
};
use serde::Deserialize;
use serde_json::Value;
use std::{
    collections::HashMap,
    sync::{
        Arc,
        atomic::{AtomicU32, Ordering},
    },
};
use tokio::{
    net::TcpStream,
    sync::{Mutex, mpsc, oneshot},
    time::{Duration, timeout},
};
use tokio_tungstenite::{MaybeTlsStream, WebSocketStream, tungstenite::Message};

pub(crate) type WsWriter = SplitSink<WebSocketStream<MaybeTlsStream<TcpStream>>, Message>;
pub(crate) type WsReader = SplitStream<WebSocketStream<MaybeTlsStream<TcpStream>>>;

/// Shared state used by both dispatcher strategies.
pub(crate) struct ConnectionInternals {
    pub(crate) ws_writer: Mutex<WsWriter>,
    pub(crate) command_id_counter: Arc<AtomicU32>,
    pub(crate) pending_commands: Mutex<HashMap<u32, oneshot::Sender<CdpResponse>>>,
    pub(crate) command_timeout: Duration,
}

impl ConnectionInternals {
    /// Creates a new [`ConnectionInternals`] instance.
    pub(crate) fn new(ws_writer: WsWriter) -> Self {
        Self {
            ws_writer: Mutex::new(ws_writer),
            command_id_counter: Arc::new(AtomicU32::new(1)),
            pending_commands: Mutex::new(HashMap::new()),
            command_timeout: Duration::from_secs(30),
        }
    }

    /// Sends a CDP command and waits for the response, including timeout handling.
    ///
    /// # Parameters
    /// * `method` - The generated CDP command to execute.
    /// * `session_id` - Optional session to scope the command to.
    /// * `timeout_duration` - Custom timeout (defaults to the connection-level timeout).
    ///
    /// # Examples
    /// ```ignore
    /// let result: cdp_protocol::page::GetFrameTreeReturnObject =
    ///     internals.send(cdp_protocol::page::GetFrameTree {}, None, None).await?;
    /// ```
    pub(crate) async fn send<
        M: serde::Serialize + std::fmt::Debug + cdp_protocol::types::Method,
        R: for<'de> Deserialize<'de>,
    >(
        &self,
        method: M,
        session_id: Option<String>,
        timeout_duration: Option<Duration>,
    ) -> Result<R> {
        let id = self.command_id_counter.fetch_add(1, Ordering::SeqCst);
        let call = method.to_method_call(id);
        let mut v = serde_json::to_value(&call).map_err(|err| {
            CdpError::connection(format!("Failed to serialize command payload: {err}"))
        })?;
        if let Some(session_id) = session_id
            && let Some(obj) = v.as_object_mut() {
                obj.insert("sessionId".to_string(), Value::String(session_id));
            }

        let (response_sender, response_receiver) = oneshot::channel();
        self.pending_commands
            .lock()
            .await
            .insert(id, response_sender);

        let json_command = serde_json::to_string(&v).map_err(|err| {
            CdpError::connection(format!("Failed to encode command payload as JSON: {err}"))
        })?;
        let command_label = json_command.clone();

        self.ws_writer
            .lock()
            .await
            .send(Message::Text(json_command.into()))
            .await?;

        let timeout_duration = timeout_duration.unwrap_or(self.command_timeout);
        let response = timeout(timeout_duration, response_receiver)
            .await
            .map_err(|_| {
                CdpError::connection(format!(
                    "Command timed out after {:?}: {}",
                    timeout_duration, command_label
                ))
            })??;

        match response.result {
            Some(result) => Ok(serde_json::from_value(result).map_err(|err| {
                CdpError::connection(format!(
                    "Failed to deserialize response for command {}: {err}",
                    command_label
                ))
            })?),
            None => Err(CdpError::connection(format!(
                "Command failed: {} => {:?}",
                command_label, response.error
            ))),
        }
    }
}

/// Dispatch loop used when multiple sessions share a single connection.
///
/// Incoming text frames are decoded into either command responses or
/// structured events. Responses are routed back to the pending command
/// channel, while events are fanned out to the registered session inboxes.
///
/// # Examples
/// ```ignore
/// tokio::spawn(central_message_dispatcher(reader, internals, active_pages, inboxes));
/// ```
pub(crate) async fn central_message_dispatcher(
    mut reader: WsReader,
    internals: Arc<ConnectionInternals>,
    active_pages: Arc<Mutex<HashMap<String, Arc<Page>>>>,
    session_event_senders: Arc<Mutex<HashMap<String, mpsc::Sender<CdpEvent>>>>,
) {
    tracing::debug!("Central event dispatcher started.");
    while let Some(Ok(Message::Text(text))) = reader.next().await {
        if let Some((cdp_event, session_id)) = handle_incoming_text(&text, &internals).await
            && let Some(session_id) = session_id {
                if let Some(page) = active_pages.lock().await.get(&session_id) {
                    update_page_meta(page, cdp_event.clone()).await;
                }
                if let Some(event_sender) = session_event_senders.lock().await.get(&session_id)
                    && event_sender.send(cdp_event).await.is_err() {
                        break;
                    }
            }
    }
    tracing::debug!("Central event dispatcher stopped.");
}

/// Dispatch loop used when a single [`Page`] owns the connection directly.
///
/// The page reference is upgraded on demand to keep frame metadata in sync
/// with incoming events.
///
/// # Examples
/// ```ignore
/// tokio::spawn(direct_message_dispatcher(reader, internals, page_events, Arc::downgrade(&page)));
/// ```
pub(crate) async fn direct_message_dispatcher(
    mut reader: WsReader,
    internals: Arc<ConnectionInternals>,
    event_sender: mpsc::Sender<CdpEvent>,
    page_weak: std::sync::Weak<Page>,
) {
    tracing::debug!("Direct event dispatcher started.");
    while let Some(Ok(Message::Text(text))) = reader.next().await {
        if let Some((cdp_event, _)) = handle_incoming_text(&text, &internals).await {
            if let Some(page) = page_weak.upgrade() {
                update_page_meta(&page, cdp_event.clone()).await;
            }
            if event_sender.send(cdp_event).await.is_err() {
                break;
            }
        }
    }
    tracing::debug!("Direct event dispatcher stopped.");
}

async fn handle_incoming_text(
    text: &str,
    internals: &Arc<ConnectionInternals>,
) -> Option<(CdpEvent, Option<String>)> {
    if let Ok(response) = serde_json::from_str::<CdpResponse>(text) {
        if let Some(sender) = internals.pending_commands.lock().await.remove(&response.id) {
            let _ = sender.send(response);
        }
        return None;
    }

    if let Ok(event_msg) = serde_json::from_str::<CdpEventMessage>(text) {
        match event_parser::parse_cdp_event(&event_msg) {
            Ok(event) => return Some((event, event_msg.session_id)),
            Err(err) => {
                tracing::error!(
                    "Failed to parse event, method: {}. Error: {:?}",
                    event_msg.method,
                    err
                );
            }
        }
    }
    None
}

/// Keeps [`Page`] bookkeeping in sync with incoming CDP events.
async fn update_page_meta(page: &Arc<Page>, cdp_event: CdpEvent) {
    if let CdpEvent::Event(boxed_event) = cdp_event { match *boxed_event {
        cdp_protocol::types::Event::RuntimeExecutionContextCreated(event) => {
            if let Some(aux_data) = event.params.context.aux_data
                && let Some(frame_id) = aux_data.get("frameId").and_then(|v| v.as_str()) {
                    page.register_execution_context(
                        frame_id.to_string(),
                        event.params.context.id,
                    )
                    .await;
                }
        }
        cdp_protocol::types::Event::RuntimeExecutionContextDestroyed(event) => {
            page.remove_execution_context(event.params.execution_context_id)
                .await;
        }
        cdp_protocol::types::Event::RuntimeExecutionContextsCleared(_) => {
            page.clear_execution_contexts().await;
        }
        cdp_protocol::types::Event::PageFrameAttached(event) => {
            page.handle_frame_attached(
                event.params.frame_id,
                Some(event.params.parent_frame_id),
            )
            .await;
        }
        cdp_protocol::types::Event::PageFrameDetached(event) => {
            page.handle_frame_detached(event.params.frame_id).await;
        }
        cdp_protocol::types::Event::PageFrameNavigated(event) => {
            page.handle_frame_navigated(event.params.frame.id, event.params.frame.url)
                .await;
        }
        cdp_protocol::types::Event::DOMChildNodeInserted(event) => {
            if let Ok(node_value) = serde_json::to_value(&event.params.node) {
                page.handle_child_node_inserted(
                    event.params.parent_node_id,
                    event.params.previous_node_id,
                    node_value,
                )
                .await;
            } else {
                tracing::error!("Failed to serialize node for DOM.childNodeInserted");
            }
        }
        cdp_protocol::types::Event::DOMChildNodeRemoved(event) => {
            page.handle_child_node_removed(event.params.parent_node_id, event.params.node_id)
                .await;
        }
        cdp_protocol::types::Event::DOMAttributeModified(event) => {
            page.handle_attribute_modified(
                event.params.node_id,
                event.params.name,
                event.params.value,
            )
            .await;
        }
        cdp_protocol::types::Event::DOMAttributeRemoved(event) => {
            page.handle_attribute_removed(event.params.node_id, event.params.name)
                .await;
        }
        cdp_protocol::types::Event::DOMCharacterDataModified(event) => {
            page.handle_character_data_modified(
                event.params.node_id,
                event.params.character_data,
            )
            .await;
        }
        cdp_protocol::types::Event::NetworkResponseReceived(event) => {
            let page_clone = Arc::clone(page);
            // Avoid blocking the dispatcher while fetching the response body.
            tokio::spawn(async move {
                if let Err(err) = handle_network_response_received(page_clone, event).await {
                    tracing::debug!("Failed to handle network response event: {:?}", err);
                }
            });
        }
        cdp_protocol::types::Event::NetworkRequestWillBeSent(event) => {
            // Notify the network monitor about request lifecycle events.
            use network_intercept::NetworkEvent;

            page.network_monitor
                .request_started(&event.params.request_id)
                .await;
            page.network_monitor
                .trigger_event(NetworkEvent::RequestWillBeSent {
                    request_id: event.params.request_id.clone(),
                    url: event.params.request.url.clone(),
                    method: event.params.request.method.clone(),
                    headers: serde_json::to_value(&event.params.request.headers)
                        .unwrap_or_default(),
                })
                .await;
        }
        cdp_protocol::types::Event::NetworkLoadingFinished(event) => {
            // Notify the network monitor about the completed request.
            use network_intercept::NetworkEvent;

            page.network_monitor
                .request_finished(&event.params.request_id)
                .await;
            page.network_monitor
                .trigger_event(NetworkEvent::LoadingFinished {
                    request_id: event.params.request_id.clone(),
                })
                .await;

            // Fetch response body if monitored
            let page_clone = Arc::clone(page);
            let request_id = event.params.request_id.clone();
            tokio::spawn(async move {
                if let Some(mut response) = page_clone
                    .response_monitor_manager
                    .retrieve_pending_response(&request_id)
                    .await
                {
                    let get_body_cmd = cdp_protocol::network::GetResponseBody {
                        request_id: request_id.clone(),
                    };

                    match page_clone
                        .session
                        .send_command::<_, cdp_protocol::network::GetResponseBodyReturnObject>(
                            get_body_cmd,
                            None,
                        )
                        .await
                    {
                        Ok(body_result) => {
                            response.body = Some(body_result.body);
                            response.base_64_encoded = body_result.base_64_encoded;
                        }
                        Err(e) => {
                            tracing::debug!(
                                "Failed to get response body for {}: {}",
                                request_id,
                                e
                            );
                        }
                    };

                    page_clone
                        .response_monitor_manager
                        .handle_response(&response)
                        .await;
                }
            });
        }
        cdp_protocol::types::Event::NetworkLoadingFailed(event) => {
            // Notify the network monitor about failures.
            use network_intercept::NetworkEvent;

            page.network_monitor
                .request_finished(&event.params.request_id)
                .await;
            page.network_monitor
                .trigger_event(NetworkEvent::LoadingFailed {
                    request_id: event.params.request_id.clone(),
                    error_text: event.params.error_text.clone(),
                })
                .await;

            // Remove pending response if any
            page.response_monitor_manager
                .retrieve_pending_response(&event.params.request_id)
                .await;
        }
        cdp_protocol::types::Event::NetworkRequestServedFromCache(event) => {
            // Notify the network monitor about cached responses.
            use network_intercept::NetworkEvent;

            page.network_monitor
                .trigger_event(NetworkEvent::RequestServedFromCache {
                    request_id: event.params.request_id.clone(),
                })
                .await;
        }
        _ => {}
    } }
}

/// Fetches response metadata and body snippets for the network monitor.
async fn handle_network_response_received(
    page: Arc<Page>,
    event: cdp_protocol::network::events::ResponseReceivedEvent,
) -> Result<()> {
    use network_intercept::{InterceptedResponse, NetworkEvent};

    let params = event.params;
    let request_id = params.request_id.clone();
    let status = params.response.status as i64;
    let status_text = params.response.status_text.clone();

    page.network_monitor
        .trigger_event(NetworkEvent::ResponseReceived {
            request_id: request_id.clone(),
            status,
            headers: serde_json::to_value(&params.response.headers).unwrap_or_default(),
        })
        .await;

    if !page.response_monitor_manager.is_enabled() {
        return Ok(());
    }

    let url = params.response.url.clone();
    if !page.response_monitor_manager.filter_url(&url).await {
        tracing::trace!("Skipping response body for {} (no monitors matched)", url);
        return Ok(());
    }

    let headers = if let Some(ref header_value) = params.response.headers.0 {
        if let Ok(map) =
            serde_json::from_value::<HashMap<String, serde_json::Value>>(header_value.clone())
        {
            map.into_iter()
                .map(|(k, v)| (k, v.to_string().trim_matches('"').to_string()))
                .collect()
        } else {
            HashMap::new()
        }
    } else {
        HashMap::new()
    };

    tracing::debug!("Response received for {}: {}", request_id, url);

    let response = InterceptedResponse {
        request_id: request_id.clone(),
        status_code: status,
        status_text,
        headers,
        base_64_encoded: false,
        body: None,
    };

    page.response_monitor_manager
        .store_pending_response(response)
        .await;

    Ok(())
}
