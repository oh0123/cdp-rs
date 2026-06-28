use crate::DomainType;
use crate::error::Result;
use crate::page::Page;
use async_trait::async_trait;
use cdp_protocol::fetch::{
    self as fetch_cdp, ContinueRequest, ContinueResponse, FailRequest, FulfillRequest,
    RequestPattern, RequestStage,
};
use cdp_protocol::network;
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};
use std::str::FromStr;
use std::sync::Arc;
use tokio::sync::Mutex;

/// HTTP methods supported by the interceptor helpers.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum HttpMethod {
    GET,
    POST,
    PUT,
    DELETE,
    PATCH,
    HEAD,
    OPTIONS,
    CONNECT,
    TRACE,
}

impl HttpMethod {
    pub fn as_str(&self) -> &'static str {
        match self {
            HttpMethod::GET => "GET",
            HttpMethod::POST => "POST",
            HttpMethod::PUT => "PUT",
            HttpMethod::DELETE => "DELETE",
            HttpMethod::PATCH => "PATCH",
            HttpMethod::HEAD => "HEAD",
            HttpMethod::OPTIONS => "OPTIONS",
            HttpMethod::CONNECT => "CONNECT",
            HttpMethod::TRACE => "TRACE",
        }
    }
}

impl FromStr for HttpMethod {
    type Err = ();

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        match s.to_uppercase().as_str() {
            "GET" => Ok(HttpMethod::GET),
            "POST" => Ok(HttpMethod::POST),
            "PUT" => Ok(HttpMethod::PUT),
            "DELETE" => Ok(HttpMethod::DELETE),
            "PATCH" => Ok(HttpMethod::PATCH),
            "HEAD" => Ok(HttpMethod::HEAD),
            "OPTIONS" => Ok(HttpMethod::OPTIONS),
            "CONNECT" => Ok(HttpMethod::CONNECT),
            "TRACE" => Ok(HttpMethod::TRACE),
            _ => Ok(HttpMethod::GET), // Default to GET for unknown methods
        }
    }
}

/// Metadata captured for an intercepted request.
#[derive(Debug, Clone)]
pub struct InterceptedRequest {
    /// The CDP-assigned request identifier.
    pub request_id: String,
    /// Request URL.
    pub url: String,
    /// HTTP method in use.
    pub method: HttpMethod,
    /// Request headers.
    pub headers: HashMap<String, String>,
    /// POST body if present.
    pub post_data: Option<String>,
    /// Resource type reported by CDP.
    pub resource_type: Option<String>,
}

/// Metadata captured for an intercepted response.
#[derive(Debug, Clone)]
pub struct InterceptedResponse {
    /// The CDP-assigned request identifier.
    pub request_id: String,
    /// HTTP status code.
    pub status_code: i64,
    /// Status text.
    pub status_text: String,
    /// Response headers.
    pub headers: HashMap<String, String>,
    /// Response body captured for the request.
    ///
    /// - When available the payload contains either raw text or a base64 encoded blob.
    /// - Requests with no body (304, 204, and similar) leave this as `None`.
    /// - Binary responses are stored using base64 encoding.
    pub base_64_encoded: bool,
    pub body: Option<String>,
}

/// Options used when mutating a request before it continues.
#[derive(Debug, Clone, Default)]
pub struct RequestModification {
    /// Updated URL.
    pub url: Option<String>,
    /// Updated HTTP method.
    pub method: Option<HttpMethod>,
    /// Updated headers.
    pub headers: Option<HashMap<String, String>>,
    /// Updated POST body.
    pub post_data: Option<String>,
}

impl RequestModification {
    pub fn with_url<T: Into<String>>(mut self, url: T) -> Self {
        self.url = Some(url.into());
        self
    }

    pub fn with_method(mut self, method: HttpMethod) -> Self {
        self.method = Some(method);
        self
    }

    pub fn with_headers(mut self, headers: HashMap<String, String>) -> Self {
        self.headers = Some(headers);
        self
    }

    pub fn with_header<K, V>(mut self, key: K, value: V) -> Self
    where
        K: Into<String>,
        V: Into<String>,
    {
        self.headers
            .get_or_insert_with(HashMap::new)
            .insert(key.into(), value.into());
        self
    }

    pub fn with_post_data<T: Into<String>>(mut self, post_data: T) -> Self {
        self.post_data = Some(post_data.into());
        self
    }
}

/// Payload used to mock an entire response without issuing a network request.
#[derive(Debug, Clone)]
pub struct ResponseMock {
    /// HTTP status code to report.
    pub status_code: i64,
    /// Headers returned to the client.
    pub headers: HashMap<String, String>,
    /// Body returned to the client when fulfilling a request.
    ///
    /// `Fetch.continueResponse` can modify status and headers, but it cannot replace the response
    /// body. Use [`NetworkInterceptor::fulfill_request`] when a mocked body is required.
    pub body: String,
}

impl Default for ResponseMock {
    fn default() -> Self {
        Self {
            status_code: 200,
            headers: HashMap::new(),
            body: String::new(),
        }
    }
}

impl ResponseMock {
    pub fn new<T: Into<String>>(body: T) -> Self {
        Self {
            body: body.into(),
            ..Default::default()
        }
    }

    pub fn with_status_code(mut self, status_code: i64) -> Self {
        self.status_code = status_code;
        self
    }

    pub fn with_headers(mut self, headers: HashMap<String, String>) -> Self {
        self.headers = headers;
        self
    }

    pub fn with_header<K, V>(mut self, key: K, value: V) -> Self
    where
        K: Into<String>,
        V: Into<String>,
    {
        self.headers.insert(key.into(), value.into());
        self
    }
}

fn header_entries(headers: HashMap<String, String>) -> Vec<fetch_cdp::HeaderEntry> {
    headers
        .into_iter()
        .map(|(name, value)| fetch_cdp::HeaderEntry { name, value })
        .collect()
}

fn build_fulfill_request(request_id: &str, response: ResponseMock) -> FulfillRequest {
    FulfillRequest {
        request_id: request_id.to_string(),
        response_code: response.status_code as u32,
        response_headers: Some(header_entries(response.headers)),
        binary_response_headers: None,
        body: Some(response.body.into_bytes()),
        response_phrase: None,
    }
}

fn build_continue_response(request_id: &str, response: ResponseMock) -> ContinueResponse {
    if !response.body.is_empty() {
        tracing::warn!(
            "Fetch.continueResponse cannot replace response bodies; use fulfill_request for mocked body content"
        );
    }

    ContinueResponse {
        request_id: request_id.to_string(),
        response_code: Some(response.status_code as u32),
        response_phrase: None,
        response_headers: Some(header_entries(response.headers)),
        binary_response_headers: None,
    }
}

/// Trait describing the interception primitives exposed by `Page`.
#[async_trait]
pub trait NetworkInterceptor {
    /// Enables request interception with the provided URL patterns.
    async fn enable_request_interception(&self, patterns: Vec<String>) -> Result<()>;

    /// Disables request interception.
    async fn disable_request_interception(&self) -> Result<()>;

    /// Continues a request without modification.
    async fn continue_request(&self, request_id: &str) -> Result<()>;

    /// Continues a request after applying the provided modifications.
    async fn continue_request_with_modification(
        &self,
        request_id: &str,
        modification: RequestModification,
    ) -> Result<()>;

    /// Aborts the request with the CDP error reason provided.
    async fn fail_request(&self, request_id: &str, error_reason: &str) -> Result<()>;

    /// Fulfills the request with a mocked response payload.
    async fn fulfill_request(&self, request_id: &str, response: ResponseMock) -> Result<()>;

    /// Continues the response without modification.
    async fn continue_response(&self, request_id: &str) -> Result<()>;

    /// Continues the response after applying status/header modifications.
    ///
    /// CDP's `Fetch.continueResponse` does not support replacing the response body; use
    /// [`NetworkInterceptor::fulfill_request`] when body content must be mocked.
    async fn continue_response_with_modification(
        &self,
        request_id: &str,
        response: ResponseMock,
    ) -> Result<()>;
}

#[async_trait]
impl NetworkInterceptor for Arc<Page> {
    async fn enable_request_interception(&self, patterns: Vec<String>) -> Result<()> {
        // Convert raw URL patterns into the CDP request pattern format.
        let request_patterns = patterns
            .into_iter()
            .map(|url_pattern| RequestPattern {
                url_pattern: Some(url_pattern),
                resource_type: None,
                request_stage: Some(RequestStage::Request),
            })
            .collect();

        // Enable the Fetch domain with our pattern set.
        self.domain_manager
            .enable_fetch_domain_with_patterns(Some(request_patterns))
            .await?;

        Ok(())
    }

    async fn disable_request_interception(&self) -> Result<()> {
        // Disable the Fetch domain for the current session.
        self.domain_manager.disable_fetch_domain().await?;
        Ok(())
    }

    async fn continue_request(&self, request_id: &str) -> Result<()> {
        let cont = ContinueRequest {
            request_id: request_id.to_string(),
            url: None,
            method: None,
            post_data: None,
            headers: None,
            intercept_response: None,
        };

        self.session
            .send_command::<_, fetch_cdp::ContinueRequestReturnObject>(cont, None)
            .await?;

        Ok(())
    }

    async fn continue_request_with_modification(
        &self,
        request_id: &str,
        modification: RequestModification,
    ) -> Result<()> {
        let headers = modification.headers.map(|h| {
            h.into_iter()
                .map(|(k, v)| fetch_cdp::HeaderEntry { name: k, value: v })
                .collect()
        });

        let post_data = modification.post_data.map(|s| s.into_bytes());

        let cont = ContinueRequest {
            request_id: request_id.to_string(),
            url: modification.url,
            method: modification.method.map(|m| m.as_str().to_string()),
            post_data,
            headers,
            intercept_response: None,
        };

        self.session
            .send_command::<_, fetch_cdp::ContinueRequestReturnObject>(cont, None)
            .await?;

        Ok(())
    }

    async fn fail_request(&self, request_id: &str, error_reason: &str) -> Result<()> {
        // Convert the supplied string into the CDP `ErrorReason` enum.
        let error = match error_reason.to_uppercase().as_str() {
            "FAILED" => network::ErrorReason::Failed,
            "ABORTED" => network::ErrorReason::Aborted,
            "TIMEDOUT" => network::ErrorReason::TimedOut,
            "ACCESSDENIED" => network::ErrorReason::AccessDenied,
            "CONNECTIONCLOSED" => network::ErrorReason::ConnectionClosed,
            "CONNECTIONRESET" => network::ErrorReason::ConnectionReset,
            "CONNECTIONREFUSED" => network::ErrorReason::ConnectionRefused,
            "CONNECTIONABORTED" => network::ErrorReason::ConnectionAborted,
            "CONNECTIONFAILED" => network::ErrorReason::ConnectionFailed,
            "NAMENOTRESOLVED" => network::ErrorReason::NameNotResolved,
            "INTERNETDISCONNECTED" => network::ErrorReason::InternetDisconnected,
            "ADDRESSUNREACHABLE" => network::ErrorReason::AddressUnreachable,
            "BLOCKEDBYCLIENT" => network::ErrorReason::BlockedByClient,
            "BLOCKEDBYRESPONSE" => network::ErrorReason::BlockedByResponse,
            _ => network::ErrorReason::Failed,
        };

        let fail = FailRequest {
            request_id: request_id.to_string(),
            error_reason: error,
        };

        self.session
            .send_command::<_, fetch_cdp::FailRequestReturnObject>(fail, None)
            .await?;

        Ok(())
    }

    async fn fulfill_request(&self, request_id: &str, response: ResponseMock) -> Result<()> {
        let fulfill = build_fulfill_request(request_id, response);

        self.session
            .send_command::<_, fetch_cdp::FulfillRequestReturnObject>(fulfill, None)
            .await?;

        Ok(())
    }

    async fn continue_response(&self, request_id: &str) -> Result<()> {
        let cont = ContinueResponse {
            request_id: request_id.to_string(),
            response_code: None,
            response_phrase: None,
            response_headers: None,
            binary_response_headers: None,
        };

        self.session
            .send_command::<_, fetch_cdp::ContinueResponseReturnObject>(cont, None)
            .await?;

        Ok(())
    }

    async fn continue_response_with_modification(
        &self,
        request_id: &str,
        response: ResponseMock,
    ) -> Result<()> {
        let cont = build_continue_response(request_id, response);

        self.session
            .send_command::<_, fetch_cdp::ContinueResponseReturnObject>(cont, None)
            .await?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn continue_response_does_not_encode_body_as_binary_headers() {
        let command = build_continue_response(
            "request-1",
            ResponseMock::new("replacement body")
                .with_status_code(201)
                .with_header("content-type", "text/plain"),
        );

        assert_eq!(command.request_id, "request-1");
        assert_eq!(command.response_code, Some(201));
        assert!(command.binary_response_headers.is_none());
        assert_eq!(
            command.response_headers,
            Some(vec![fetch_cdp::HeaderEntry {
                name: "content-type".to_string(),
                value: "text/plain".to_string(),
            }])
        );
    }

    #[test]
    fn fulfill_request_keeps_body_on_body_field() {
        let command = build_fulfill_request("request-2", ResponseMock::new("mocked body"));

        assert_eq!(command.request_id, "request-2");
        assert_eq!(command.response_code, 200);
        assert_eq!(command.body, Some(b"mocked body".to_vec()));
        assert!(command.binary_response_headers.is_none());
    }
}

/// Convenience helpers that wrap the interceptor trait.
#[async_trait]
pub trait RequestInterceptorExt {
    /// Intercepts every request.
    async fn intercept_all_requests(&self) -> Result<()>;

    /// Intercepts requests that match the provided pattern.
    async fn intercept_requests_matching(&self, pattern: &str) -> Result<()>;

    /// Blocks common image formats.
    async fn block_images(&self) -> Result<()>;

    /// Blocks stylesheet resources.
    async fn block_stylesheets(&self) -> Result<()>;
}

#[async_trait]
impl RequestInterceptorExt for Arc<Page> {
    async fn intercept_all_requests(&self) -> Result<()> {
        self.enable_request_interception(vec!["*".to_string()])
            .await
    }

    async fn intercept_requests_matching(&self, pattern: &str) -> Result<()> {
        self.enable_request_interception(vec![pattern.to_string()])
            .await
    }

    async fn block_images(&self) -> Result<()> {
        self.enable_request_interception(vec![
            "*.png".to_string(),
            "*.jpg".to_string(),
            "*.jpeg".to_string(),
            "*.gif".to_string(),
            "*.webp".to_string(),
        ])
        .await
    }

    async fn block_stylesheets(&self) -> Result<()> {
        self.enable_request_interception(vec!["*.css".to_string()])
            .await
    }
}

// ========= Network monitoring =========

/// Network event envelope emitted by the monitor.
#[derive(Clone, Debug)]
pub enum NetworkEvent {
    /// Request about to be sent.
    RequestWillBeSent {
        request_id: String,
        url: String,
        method: String,
        headers: serde_json::Value,
    },
    /// Request finished loading.
    LoadingFinished { request_id: String },
    /// Request failed to load.
    LoadingFailed {
        request_id: String,
        error_text: String,
    },
    /// Response received.
    ResponseReceived {
        request_id: String,
        status: i64,
        headers: serde_json::Value,
    },
    /// Response served from cache.
    RequestServedFromCache { request_id: String },
}

/// Network event callback signature.
pub type NetworkEventCallback = Arc<dyn Fn(NetworkEvent) + Send + Sync>;

/// Tracks network activity and propagates events to user callbacks.
pub struct NetworkMonitor {
    /// Registered callbacks.
    pub callbacks: Arc<Mutex<Vec<NetworkEventCallback>>>,
    /// Number of inflight requests.
    inflight_count: Arc<std::sync::atomic::AtomicUsize>,
    /// Known request identifiers to prevent duplicate accounting.
    active_requests: Arc<Mutex<HashSet<String>>>,
}

impl NetworkMonitor {
    fn new() -> Self {
        Self {
            inflight_count: Arc::new(std::sync::atomic::AtomicUsize::new(0)),
            callbacks: Arc::new(Mutex::new(Vec::new())),
            active_requests: Arc::new(Mutex::new(HashSet::new())),
        }
    }

    /// Returns the number of active requests.
    pub fn get_inflight_count(&self) -> usize {
        self.inflight_count
            .load(std::sync::atomic::Ordering::SeqCst)
    }

    /// Marks a request as started, incrementing the counter only once.
    pub async fn request_started(&self, request_id: &str) {
        let mut active = self.active_requests.lock().await;
        if active.insert(request_id.to_string()) {
            self.inflight_count
                .fetch_add(1, std::sync::atomic::Ordering::SeqCst);
        } else {
            tracing::trace!("request_started called for tracked request {request_id}");
        }
    }

    /// Marks a request as finished and decrements the counter if tracked.
    pub async fn request_finished(&self, request_id: &str) {
        let mut active = self.active_requests.lock().await;
        if active.remove(request_id) {
            if self
                .inflight_count
                .fetch_update(
                    std::sync::atomic::Ordering::SeqCst,
                    std::sync::atomic::Ordering::SeqCst,
                    |current| current.checked_sub(1),
                )
                .is_err()
            {
                // Reset to zero if a mismatch occurs to avoid underflow.
                self.inflight_count
                    .store(0, std::sync::atomic::Ordering::SeqCst);
                tracing::warn!(
                    "request_finished detected underflow for {request_id}, resetting inflight count"
                );
            }
        } else {
            tracing::trace!("request_finished called for unknown request {request_id}");
        }
    }

    /// Resets the inflight counter.
    pub async fn reset_inflight(&self) {
        self.inflight_count
            .store(0, std::sync::atomic::Ordering::SeqCst);
        self.active_requests.lock().await.clear();
    }

    /// Registers a network event callback.
    pub async fn add_callback(&self, callback: NetworkEventCallback) {
        self.callbacks.lock().await.push(callback);
    }

    /// Emits an event to all registered callbacks.
    pub async fn trigger_event(&self, event: NetworkEvent) {
        let callbacks = self.callbacks.lock().await;
        for callback in callbacks.iter() {
            callback(event.clone());
        }
    }
}

impl Default for NetworkMonitor {
    fn default() -> Self {
        Self::new()
    }
}

// ========= Response monitoring =========

/// Callback used to decide whether a URL should be inspected.
pub type ResponseFilterCallback = Arc<dyn Fn(&str) -> bool + Send + Sync>;

/// Callback invoked with the captured response metadata.
pub type ResponseHandlerCallback = Arc<dyn Fn(&InterceptedResponse) + Send + Sync>;

pub(crate) struct PendingResponse {
    pub(crate) url: String,
    pub(crate) response: InterceptedResponse,
}

/// Manages response filters and handlers.
pub struct ResponseMonitorManager {
    /// Registered monitor pairs.
    monitors: Mutex<Vec<(ResponseFilterCallback, ResponseHandlerCallback)>>,
    /// Tracks whether monitoring is enabled.
    enabled: std::sync::atomic::AtomicBool,
    /// Pending responses waiting for body (requestId -> Response)
    pending_responses: Mutex<HashMap<String, PendingResponse>>,
}

impl ResponseMonitorManager {
    fn new() -> Self {
        Self {
            monitors: Mutex::new(Vec::new()),
            enabled: std::sync::atomic::AtomicBool::new(false),
            pending_responses: Mutex::new(HashMap::new()),
        }
    }

    /// Returns whether response monitoring is active.
    pub fn is_enabled(&self) -> bool {
        self.enabled.load(std::sync::atomic::Ordering::SeqCst)
    }

    /// Adds a monitor pair and enables monitoring.
    pub async fn add_monitor(
        &self,
        filter: ResponseFilterCallback,
        handler: ResponseHandlerCallback,
    ) {
        let mut monitors = self.monitors.lock().await;
        monitors.push((filter, handler));
        // Automatically enable monitoring when a handler exists.
        self.enabled
            .store(true, std::sync::atomic::Ordering::SeqCst);
    }

    /// Clears all monitors and disables monitoring.
    pub async fn clear_monitors(&self) {
        let mut monitors = self.monitors.lock().await;
        monitors.clear();
        // Disable monitoring once the list is empty.
        self.enabled
            .store(false, std::sync::atomic::Ordering::SeqCst);
    }

    /// Dispatches the response to all registered handlers.
    pub async fn handle_response(&self, response: &InterceptedResponse) {
        // Skip work unless monitoring is active.
        if !self.is_enabled() {
            return;
        }
        let monitors = self.monitors.lock().await;
        for (_, handler) in monitors.iter() {
            handler(response);
        }
    }

    pub async fn filter_url(&self, url: &str) -> bool {
        if !self.is_enabled() {
            return false;
        }

        let monitors = self.monitors.lock().await;
        monitors.iter().any(|(filter, _)| filter(url))
    }

    pub async fn store_pending_response(&self, response: InterceptedResponse) {
        self.store_pending_response_for_url(String::new(), response)
            .await;
    }

    pub(crate) async fn store_pending_response_for_url(
        &self,
        url: String,
        response: InterceptedResponse,
    ) {
        self.pending_responses.lock().await.insert(
            response.request_id.clone(),
            PendingResponse { url, response },
        );
    }

    pub async fn retrieve_pending_response(&self, request_id: &str) -> Option<InterceptedResponse> {
        self.retrieve_pending_response_with_url(request_id)
            .await
            .map(|pending| pending.response)
    }

    pub(crate) async fn retrieve_pending_response_with_url(
        &self,
        request_id: &str,
    ) -> Option<PendingResponse> {
        self.pending_responses.lock().await.remove(request_id)
    }
}

impl Default for ResponseMonitorManager {
    fn default() -> Self {
        Self::new()
    }
}

/// Response monitoring convenience methods
impl Page {
    /// Registers a non-blocking handler for all responses that pass the filter.
    ///
    /// # Parameters
    /// * `filter` - Returns `true` when a response should be forwarded to the handler.
    /// * `handler` - Receives the captured response metadata.
    ///
    /// # Examples
    /// ```no_run
    /// # use cdp_core::Page;
    /// # use std::sync::Arc;
    /// # async fn example(page: Arc<Page>) -> anyhow::Result<()> {
    /// page.monitor_responses(
    ///     |url| url.contains("/api/"),
    ///     |response| {
    ///         println!("API Response: {} - {}", response.status_code, response.status_text);
    ///         if let Some(body) = &response.body {
    ///             println!("Body: {}", body);
    ///         }
    ///     },
    /// ).await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn monitor_responses<F, H>(self: &Arc<Self>, filter: F, handler: H) -> Result<()>
    where
        F: Fn(&str) -> bool + Send + Sync + 'static,
        H: Fn(&InterceptedResponse) + Send + Sync + 'static,
    {
        // Ensure the Network domain is active before monitoring.
        if !self.domain_manager.is_enabled(DomainType::Network).await {
            self.domain_manager.enable_network_domain().await?;
        }

        // Register the monitor pair.
        self.response_monitor_manager
            .add_monitor(Arc::new(filter), Arc::new(handler))
            .await;

        Ok(())
    }

    /// Registers a handler for responses whose URLs contain the given pattern.
    ///
    /// # Parameters
    /// * `url_pattern` - Substring that must be present in the URL.
    /// * `handler` - Invoked with the captured response metadata.
    ///
    /// # Examples
    /// ```no_run
    /// # use cdp_core::Page;
    /// # use std::sync::Arc;
    /// # async fn example(page: Arc<Page>) -> anyhow::Result<()> {
    /// page.monitor_responses_matching(
    ///     "data.json",
    ///     |response| {
    ///         println!("Data Response: {}", response.status_code);
    ///     },
    /// ).await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn monitor_responses_matching<H>(
        self: &Arc<Self>,
        url_pattern: &str,
        handler: H,
    ) -> Result<()>
    where
        H: Fn(&InterceptedResponse) + Send + Sync + 'static,
    {
        let pattern = url_pattern.to_string();
        self.monitor_responses(move |url| url.contains(&pattern), handler)
            .await
    }

    /// Removes all registered response monitors.
    ///
    /// # Examples
    /// ```no_run
    /// # use cdp_core::Page;
    /// # use std::sync::Arc;
    /// # async fn example(page: Arc<Page>) -> anyhow::Result<()> {
    /// page.stop_response_monitoring().await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn stop_response_monitoring(self: &Arc<Self>) -> Result<()> {
        // Remove all handlers; keep the Network domain enabled because other features rely on it.
        self.response_monitor_manager.clear_monitors().await;
        Ok(())
    }
}
