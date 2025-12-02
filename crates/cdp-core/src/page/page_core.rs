use crate::DomainType;
use crate::accessibility::AccessibilityController;
use crate::browser::{
    discovery::{find_running_browser_port, find_running_browser_port_by_process_name},
    launcher::BrowserType,
    ws_endpoints::resolve_active_page_ws_url,
};
use crate::domain_manager::DomainManager;
use crate::emulation::EmulationController;
use crate::error::{CdpError, Result};
use crate::input::{keyboard::Keyboard, mouse::Mouse};
use crate::network::network_intercept::{
    NetworkEventCallback, NetworkMonitor, ResponseMonitorManager,
};
use crate::page::{element::ElementHandle, frame::Frame};
use crate::session::Session;
use crate::tracing::{TracingController, TracingSessionState};
use crate::transport::{cdp_protocol::*, websocket_connection::*};
use cdp_protocol::page::Viewport;
use cdp_protocol::{
    page::{
        self as page_cdp, FrameTree, GetFrameTree, GetFrameTreeReturnObject, Navigate,
        NavigateReturnObject,
    },
    runtime::{Evaluate, EvaluateReturnObject},
};
use futures_util::Stream;
use futures_util::StreamExt;
use std::collections::HashMap;
use std::path::PathBuf;
use std::pin::Pin;
use std::sync::Arc;
use tokio::fs::File;
use tokio::io::AsyncWriteExt;
use tokio::sync::Mutex;
use tokio::sync::broadcast;
use tokio::sync::mpsc;
use tokio_stream::wrappers::BroadcastStream;
use tokio_tungstenite::connect_async;

/// Frame lifecycle event type
#[derive(Clone, Debug)]
pub enum FrameLifecycleEvent {
    /// Frame attached
    Attached {
        frame_id: String,
        parent_frame_id: Option<String>,
    },
    /// Frame detached
    Detached { frame_id: String },
    /// Frame navigated
    Navigated { frame_id: String, url: String },
}

/// Frame lifecycle callback
pub type FrameLifecycleCallback = Arc<dyn Fn(FrameLifecycleEvent) + Send + Sync>;

/// DOM mutation event type
#[derive(Clone, Debug)]
pub enum DomMutationEvent {
    /// Child node inserted
    ChildNodeInserted {
        parent_node_id: u32,
        previous_node_id: u32,
        node: serde_json::Value,
    },
    /// Child node removed
    ChildNodeRemoved { parent_node_id: u32, node_id: u32 },
    /// Attribute modified
    AttributeModified {
        node_id: u32,
        name: String,
        value: String,
    },
    /// Attribute removed
    AttributeRemoved { node_id: u32, name: String },
    /// Character data modified
    CharacterDataModified {
        node_id: u32,
        character_data: String,
    },
}

/// Navigation wait options
#[derive(Debug, Clone, Default)]
pub struct WaitForNavigationOptions {
    /// Timeout in milliseconds, default 30000
    pub timeout_ms: Option<u64>,
    /// Wait condition, default WaitUntil::Load
    pub wait_until: Option<WaitUntil>,
}

/// Selector wait options
#[derive(Debug, Clone, Default)]
pub struct WaitForSelectorOptions {
    /// Timeout in milliseconds, default 30000
    pub timeout_ms: Option<u64>,
    /// Whether to wait for element to be visible, default false
    pub visible: Option<bool>,
    /// Whether to wait for element to be hidden, default false
    pub hidden: Option<bool>,
}

/// Page navigation wait condition
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum WaitUntil {
    /// Wait for load event (document.readyState === 'complete')
    /// This is the strictest condition, waiting for all resources (images, stylesheets, etc.) to load
    Load,

    /// Wait for DOMContentLoaded event (document.readyState === 'interactive' or 'complete')
    /// DOM tree is built, but external resources may still be loading
    DOMContentLoaded,

    /// Wait for network idle (no network connections for at least 500ms)
    /// Suitable for SPAs waiting for async data loading
    NetworkIdle0,

    /// Wait for network almost idle (no more than 2 network connections for at least 500ms)
    /// More relaxed than NetworkIdle0, suitable for most scenarios
    #[default]
    NetworkIdle2,
}

/// DOM mutation callback
pub type DomMutationCallback = Arc<dyn Fn(DomMutationEvent) + Send + Sync>;

// --- PAGE (Unified Interface) ---
pub struct Page {
    pub(crate) session: Arc<Session>,
    /// Frame ID -> Execution Context ID mapping
    pub contexts: Arc<Mutex<HashMap<String, u32>>>,
    /// Domain manager (unified management of CDP Domain enabling/disabling)
    pub domain_manager: Arc<DomainManager>,
    /// Network monitor
    pub network_monitor: Arc<NetworkMonitor>,
    /// Response monitor manager
    pub response_monitor_manager: Arc<ResponseMonitorManager>,
    /// Main Frame (lazy loaded)
    main_frame_cache: Arc<Mutex<Option<Frame>>>,
    /// Cache of all Frames (Frame ID -> Frame)
    frames_cache: Arc<Mutex<HashMap<String, Frame>>>,
    /// Frame parent-child relationship mapping (Frame ID -> Parent Frame ID)
    frame_parent_map: Arc<Mutex<HashMap<String, String>>>,
    /// Frame children mapping (Frame ID -> Child Frame IDs)
    frame_children_map: Arc<Mutex<HashMap<String, Vec<String>>>>,
    /// Frame lifecycle callback list
    lifecycle_callbacks: Arc<Mutex<Vec<FrameLifecycleCallback>>>,
    /// DOM mutation callback list
    dom_mutation_callbacks: Arc<Mutex<Vec<DomMutationCallback>>>,
    /// Tracing state cache
    tracing_state: Arc<Mutex<TracingSessionState>>,
    /// File chooser mutex lock, ensuring upload flow serialization
    pub(crate) file_chooser_lock: Arc<Mutex<()>>,
}

impl Page {
    /// Mode 1 entry: Create via Browser
    pub(crate) fn new_from_browser(session: Arc<Session>) -> Self {
        let domain_manager = Arc::new(DomainManager::new(Arc::clone(&session)));
        Self {
            session,
            contexts: Arc::new(Mutex::new(HashMap::new())),
            domain_manager,
            main_frame_cache: Arc::new(Mutex::new(None)),
            frames_cache: Arc::new(Mutex::new(HashMap::new())),
            frame_parent_map: Arc::new(Mutex::new(HashMap::new())),
            frame_children_map: Arc::new(Mutex::new(HashMap::new())),
            lifecycle_callbacks: Arc::new(Mutex::new(Vec::new())),
            dom_mutation_callbacks: Arc::new(Mutex::new(Vec::new())),
            network_monitor: Arc::new(NetworkMonitor::default()),
            response_monitor_manager: Arc::new(ResponseMonitorManager::default()),
            tracing_state: Arc::new(Mutex::new(TracingSessionState::default())),
            file_chooser_lock: Arc::new(Mutex::new(())),
        }
    }

    /// Mode 2 entry: Connect directly
    pub async fn connect_to_active_page(
        port: Option<u16>,
        pattern: Option<&str>,
    ) -> Result<Arc<Self>> {
        let port = Self::resolve_remote_debug_port(port, None)?;
        Self::connect_to_active_page_inner(port, pattern).await
    }

    /// Mode 2 entry: Connect directly (supports custom browser process)
    pub async fn connect_to_active_page_with_browser(
        port: Option<u16>,
        pattern: Option<&str>,
        browser_process: Option<&str>,
    ) -> Result<Arc<Self>> {
        let port = Self::resolve_remote_debug_port(port, browser_process)?;
        Self::connect_to_active_page_inner(port, pattern).await
    }

    fn resolve_remote_debug_port(port: Option<u16>, browser_process: Option<&str>) -> Result<u16> {
        if let Some(explicit) = port {
            return Ok(explicit);
        }

        if let Some(process_name) = browser_process {
            return find_running_browser_port_by_process_name(process_name);
        }

        for bt in [
            BrowserType::Chrome,
            BrowserType::Edge,
            BrowserType::Chromium,
        ] {
            if let Ok(found) = find_running_browser_port(bt) {
                return Ok(found);
            }
        }

        Err(CdpError::tool(
            "Found no running browser with remote debugging enabled, please specify browser type or port explicitly",
        ))
    }

    async fn connect_to_active_page_inner(port: u16, pattern: Option<&str>) -> Result<Arc<Self>> {
        let page_ws_url = resolve_active_page_ws_url(port, pattern).await?;
        let (ws_stream, _) = connect_async(page_ws_url.as_str()).await?;
        let (writer, reader) = ws_stream.split();
        // Only one Session, so only one event channel
        let (event_sender, event_receiver) = mpsc::channel(crate::DEFAULT_CHANNEL_CAPACITY);
        let connection = Arc::new(ConnectionInternals::new(writer));

        let session = Session::new(None, connection.clone(), event_receiver);
        let session = Arc::new(session);
        let domain_manager = Arc::new(DomainManager::new(Arc::clone(&session)));

        let page = Arc::new(Page {
            session,
            contexts: Arc::new(Mutex::new(HashMap::new())),
            domain_manager,
            main_frame_cache: Arc::new(Mutex::new(None)),
            frames_cache: Arc::new(Mutex::new(HashMap::new())),
            frame_parent_map: Arc::new(Mutex::new(HashMap::new())),
            frame_children_map: Arc::new(Mutex::new(HashMap::new())),
            lifecycle_callbacks: Arc::new(Mutex::new(Vec::new())),
            dom_mutation_callbacks: Arc::new(Mutex::new(Vec::new())),
            network_monitor: Arc::new(NetworkMonitor::default()),
            response_monitor_manager: Arc::new(ResponseMonitorManager::default()),
            tracing_state: Arc::new(Mutex::new(TracingSessionState::default())),
            file_chooser_lock: Arc::new(Mutex::new(())),
        });

        // Create weak reference to pass to dispatcher
        let page_weak = Arc::downgrade(&page);

        // Start the event dispatcher specific to this connection (with Page reference)
        tokio::spawn(direct_message_dispatcher(
            reader,
            connection.clone(),
            event_sender,
            page_weak,
        ));

        // Use DomainManager to enable all required Domains
        page.domain_manager.enable_required_domains().await?;

        Ok(page)
    }

    /// Get mouse controller
    pub fn mouse(&self) -> Mouse {
        Mouse::new(Arc::clone(&self.session), Arc::clone(&self.domain_manager))
    }

    /// Get keyboard controller
    pub fn keyboard(&self) -> Keyboard {
        Keyboard::new(Arc::clone(&self.session), Arc::clone(&self.domain_manager))
    }

    pub fn accessibility(self: &Arc<Self>) -> AccessibilityController {
        AccessibilityController::new(Arc::clone(self))
    }

    pub fn emulation(self: &Arc<Self>) -> EmulationController {
        EmulationController::new(Arc::clone(self))
    }

    pub fn tracing(self: &Arc<Self>) -> TracingController {
        TracingController::new(Arc::clone(self), Arc::clone(&self.tracing_state))
    }

    pub async fn navigate(&self, url: &str) -> Result<NavigateReturnObject> {
        let navigate = Navigate {
            url: url.to_string(),
            referrer: None,
            transition_type: None,
            frame_id: None,
            referrer_policy: None,
        };
        self.session
            .send_command::<_, NavigateReturnObject>(navigate, None)
            .await
    }

    fn events(&self) -> broadcast::Receiver<CdpEvent> {
        self.session.event_bus.subscribe()
    }

    pub fn on<E>(&self) -> Pin<Box<dyn Stream<Item = E> + Send + 'static>>
    where
        E: TryFrom<CdpEvent> + Send + 'static,
        <E as TryFrom<CdpEvent>>::Error: Send,
    {
        let stream = BroadcastStream::new(self.events())
            .filter_map(|result| async { result.ok() })
            .filter_map(|event| async { E::try_from(event).ok() });
        Box::pin(stream)
    }

    pub(crate) async fn wait_for<E>(&self) -> Result<E>
    where
        E: TryFrom<CdpEvent> + Send + 'static,
        <E as TryFrom<CdpEvent>>::Error: Send,
    {
        self.on::<E>().next().await.ok_or_else(|| {
            CdpError::page("Event stream closed before event was received.".to_string())
        })
    }

    pub async fn wait_for_loaded(&self) -> Result<page_cdp::events::LoadEventFiredEvent> {
        self.wait_for::<page_cdp::events::LoadEventFiredEvent>()
            .await
    }

    pub async fn get_title(&self) -> Result<String> {
        let method = Evaluate {
            expression: "document.title".to_string(),
            object_group: None,
            include_command_line_api: None,
            silent: None,
            context_id: None,
            return_by_value: None,
            generate_preview: None,
            user_gesture: None,
            await_promise: None,
            throw_on_side_effect: None,
            timeout: None,
            disable_breaks: None,
            repl_mode: None,
            allow_unsafe_eval_blocked_by_csp: None,
            unique_context_id: None,
            serialization_options: None,
        };
        let eval_result = self
            .session
            .send_command::<_, EvaluateReturnObject>(method, None)
            .await?;

        // Extract title string from complex return structure
        let title = eval_result
            .result
            .value
            .and_then(|v| v.as_str().map(ToString::to_string))
            .ok_or_else(|| {
                CdpError::page("Could not extract title string from evaluation result".to_string())
            })?;

        Ok(title)
    }

    /// Get main Frame (cached)
    pub async fn main_frame(self: &Arc<Self>) -> Result<Frame> {
        {
            let cache = self.main_frame_cache.lock().await;
            if let Some(frame) = cache.as_ref() {
                return Ok(frame.clone());
            }
        }
        let method = GetFrameTree(None);
        let obj = self
            .session
            .send_command::<_, GetFrameTreeReturnObject>(method, None)
            .await?;
        let main_frame_id = obj.frame_tree.frame.id.clone();
        let main_frame = Frame::new(main_frame_id.clone(), Arc::clone(self));
        {
            let mut cache = self.main_frame_cache.lock().await;
            *cache = Some(main_frame.clone());
        }
        {
            let mut frames = self.frames_cache.lock().await;
            frames.insert(main_frame_id, main_frame.clone());
        }

        Ok(main_frame)
    }

    /// Queries the first element matching the CSS selector.
    ///
    /// This method automatically searches across all frames (including iframes):
    /// 1. First searches in the main frame
    /// 2. If not found, searches in all child frames
    ///
    /// # Parameters
    /// * `selector` - CSS selector string
    ///
    /// # Returns
    /// First matching element handle, or None if no match
    ///
    /// # Examples
    /// ```no_run
    /// # use cdp_core::Page;
    /// # use std::sync::Arc;
    /// # async fn example(page: Arc<Page>) -> anyhow::Result<()> {
    /// // Automatically search for first matching button in all frames
    /// if let Some(button) = page.query_selector(".submit-btn").await? {
    ///     button.click().await?;
    /// }
    /// # Ok(())
    /// # }
    /// ```
    pub async fn query_selector(self: &Arc<Self>, selector: &str) -> Result<Option<ElementHandle>> {
        // Search in main frame first
        let main_frame = self.main_frame().await?;
        match main_frame.query_selector(selector).await {
            Ok(Some(element)) => return Ok(Some(element)),
            Ok(None) => {} // Not found, continue searching other frames
            Err(e) => {
                // If execution context does not exist, continue trying other frames
                eprintln!("Warning: Failed to query in main frame: {}", e);
            }
        }

        // If not found in main frame or error occurred, search in all child frames
        let frames = self.all_frames().await?;
        for frame in frames.iter().skip(1) {
            // Skip the first one (main frame)
            match frame.query_selector(selector).await {
                Ok(Some(element)) => return Ok(Some(element)),
                Ok(None) => continue, // Not found, continue to next frame
                Err(e) => {
                    // Frame query failed (possibly detached or context missing), continue trying next
                    eprintln!("Warning: Failed to query in frame {}: {}", frame.id(), e);
                    continue;
                }
            }
        }

        Ok(None)
    }

    /// Queries all elements matching the CSS selector.
    ///
    /// This method automatically searches across all frames (including iframes):
    /// 1. First searches in the main frame
    /// 2. Then searches in all child frames and collects all matches
    ///
    /// # Parameters
    /// * `selector` - CSS selector string
    ///
    /// # Returns
    /// List of all matching element handles, or empty list if no match
    ///
    /// # Examples
    /// ```no_run
    /// # use cdp_core::Page;
    /// # use std::sync::Arc;
    /// # async fn example(page: Arc<Page>) -> anyhow::Result<()> {
    /// // Automatically search for all matching links in all frames
    /// let links = page.query_selector_all("a").await?;
    /// println!("Found {} links across all frames", links.len());
    ///
    /// for (i, link) in links.iter().enumerate() {
    ///     let href = link.get_attribute("href").await?;
    ///     println!("Link {}: {:?}", i + 1, href);
    /// }
    /// # Ok(())
    /// # }
    /// ```
    pub async fn query_selector_all(
        self: &Arc<Self>,
        selector: &str,
    ) -> Result<Vec<ElementHandle>> {
        // Get all frames
        let frames = self.all_frames().await?;
        let mut all_elements = Vec::new();

        // Query and collect results in each frame
        for frame in frames {
            match frame.query_selector_all(selector).await {
                Ok(elements) => {
                    all_elements.extend(elements);
                }
                Err(e) => {
                    // Frame query failed (possibly detached or context missing), continue trying next
                    eprintln!("Warning: Failed to query in frame {}: {}", frame.id(), e);
                    continue;
                }
            }
        }

        Ok(all_elements)
    }

    /// Returns a flat list of all frames attached to this page.
    pub async fn all_frames(self: &Arc<Self>) -> Result<Vec<Frame>> {
        let method = GetFrameTree(None);
        let result = self
            .session
            .send_command::<_, GetFrameTreeReturnObject>(method, None)
            .await?;
        let mut all_frames = Vec::new();
        let mut new_cache = HashMap::new();
        let mut new_parent_map = HashMap::new();
        let mut new_children_map = HashMap::new();

        Self::collect_frames_recursive(
            &result.frame_tree,
            None, // Main frame has no parent
            Arc::clone(self),
            &mut all_frames,
            &mut new_cache,
            &mut new_parent_map,
            &mut new_children_map,
        );

        // Update all caches
        {
            let mut frames_cache = self.frames_cache.lock().await;
            *frames_cache = new_cache;
        }

        {
            let mut parent_map = self.frame_parent_map.lock().await;
            *parent_map = new_parent_map;
        }

        {
            let mut children_map = self.frame_children_map.lock().await;
            *children_map = new_children_map;
        }

        {
            let mut main_cache = self.main_frame_cache.lock().await;
            if main_cache.is_none() && !all_frames.is_empty() {
                *main_cache = Some(all_frames[0].clone());
            }
        }

        Ok(all_frames)
    }

    /// Recursive helper to collect Frames (with hierarchy)
    fn collect_frames_recursive(
        frame_tree: &FrameTree,
        parent_id: Option<String>,
        page_arc: Arc<Page>,
        frames_list: &mut Vec<Frame>,
        frames_map: &mut HashMap<String, Frame>,
        parent_map: &mut HashMap<String, String>,
        children_map: &mut HashMap<String, Vec<String>>,
    ) {
        let frame_id = frame_tree.frame.id.clone();

        // Create Frame object for current node
        let current_frame = Frame::new(frame_id.clone(), Arc::clone(&page_arc));

        // Add to list and Map
        frames_list.push(current_frame.clone());
        frames_map.insert(frame_id.clone(), current_frame);

        // Record parent-child relationship
        if let Some(ref parent) = parent_id {
            parent_map.insert(frame_id.clone(), parent.clone());
            children_map
                .entry(parent.clone())
                .or_default()
                .push(frame_id.clone());
        }

        // If there are child Frames, recurse
        if let Some(child_frames) = &frame_tree.child_frames {
            for child_tree in child_frames {
                Self::collect_frames_recursive(
                    child_tree,
                    Some(frame_id.clone()),
                    Arc::clone(&page_arc),
                    frames_list,
                    frames_map,
                    parent_map,
                    children_map,
                );
            }
        }
    }

    /// Get Frame by ID (from cache)
    pub async fn get_frame(&self, frame_id: &str) -> Option<Frame> {
        self.frames_cache.lock().await.get(frame_id).cloned()
    }

    /// Clear Frame cache (called after navigation)
    pub async fn clear_frame_cache(&self) {
        self.main_frame_cache.lock().await.take();
        self.frames_cache.lock().await.clear();
    }

    /// Register Execution Context (called by event handler)
    pub async fn register_execution_context(&self, frame_id: String, context_id: u32) {
        self.contexts.lock().await.insert(frame_id, context_id);
    }

    /// Remove specified Execution Context (called by event handler)
    pub async fn remove_execution_context(&self, context_id: u32) {
        let mut contexts = self.contexts.lock().await;
        // Find and remove entry with this context_id
        contexts.retain(|_frame_id, &mut ctx_id| ctx_id != context_id);
    }

    /// Clear all Execution Contexts (called by event handler)
    pub async fn clear_execution_contexts(&self) {
        self.contexts.lock().await.clear();
    }

    // ========== Frame Hierarchy Query Methods ==========

    /// Get parent Frame of specified Frame
    pub async fn get_parent_frame(&self, frame_id: &str) -> Option<Frame> {
        let parent_map = self.frame_parent_map.lock().await;
        let parent_id = parent_map.get(frame_id)?;
        self.get_frame(parent_id).await
    }

    /// Get all child Frames of specified Frame
    pub async fn get_child_frames(&self, frame_id: &str) -> Vec<Frame> {
        let children_map = self.frame_children_map.lock().await;
        let child_ids = children_map.get(frame_id);

        match child_ids {
            Some(ids) => {
                let mut children = Vec::new();
                for id in ids {
                    if let Some(frame) = self.get_frame(id).await {
                        children.push(frame);
                    }
                }
                children
            }
            None => Vec::new(),
        }
    }

    /// Get all ancestor Frames of specified Frame (from near to far)
    pub async fn get_ancestor_frames(&self, frame_id: &str) -> Vec<Frame> {
        let mut ancestors = Vec::new();
        let mut current_id = frame_id.to_string();

        loop {
            let parent = self.get_parent_frame(&current_id).await;
            match parent {
                Some(frame) => {
                    current_id = frame.id.clone();
                    ancestors.push(frame);
                }
                None => break,
            }
        }

        ancestors
    }

    /// Get all descendant Frames of specified Frame (DFS)
    pub async fn get_descendant_frames(&self, frame_id: &str) -> Vec<Frame> {
        let mut descendants = Vec::new();
        self.collect_descendants_recursive(frame_id, &mut descendants)
            .await;
        descendants
    }

    /// Recursively collect descendant Frames
    fn collect_descendants_recursive<'a>(
        &'a self,
        frame_id: &'a str,
        result: &'a mut Vec<Frame>,
    ) -> std::pin::Pin<Box<dyn std::future::Future<Output = ()> + 'a>> {
        Box::pin(async move {
            let children = self.get_child_frames(frame_id).await;
            for child in children {
                result.push(child.clone());
                self.collect_descendants_recursive(&child.id, result).await;
            }
        })
    }

    // ========== Frame Selector Methods ==========

    /// Find Frame by selector
    ///
    /// # Selector Syntax
    /// - `name:iframe-name` - Match by name
    /// - `url:https://example.com` - Match by URL prefix
    /// - `url~pattern` - Match by URL regex
    /// - `depth:2` - Match by depth (0 = Main Frame, 1 = Direct Child Frame)
    ///
    /// # Examples
    /// ```no_run
    /// # use cdp_core::Page;
    /// # use std::sync::Arc;
    /// # async fn example(page: Arc<Page>) -> anyhow::Result<()> {
    /// // Find Frame named "login-iframe"
    /// if let Some(frame) = page.query_frame("name:login-iframe").await? {
    ///     println!("Found iframe: {}", frame.id());
    /// }
    ///
    /// // Find Frame with URL containing "checkout"
    /// if let Some(frame) = page.query_frame("url~checkout").await? {
    ///     println!("Found checkout frame: {}", frame.id());
    /// }
    /// # Ok(())
    /// # }
    /// ```
    pub async fn query_frame(self: &Arc<Self>, selector: &str) -> Result<Option<Frame>> {
        let frames = self.all_frames().await?;

        for frame in frames {
            if self.matches_selector(&frame, selector).await? {
                return Ok(Some(frame));
            }
        }

        Ok(None)
    }

    /// Find all Frames matching selector
    pub async fn query_frames(self: &Arc<Self>, selector: &str) -> Result<Vec<Frame>> {
        let frames = self.all_frames().await?;
        let mut matched = Vec::new();

        for frame in frames {
            if self.matches_selector(&frame, selector).await? {
                matched.push(frame);
            }
        }

        Ok(matched)
    }

    /// Check if Frame matches selector
    async fn matches_selector(&self, frame: &Frame, selector: &str) -> Result<bool> {
        if let Some(name_pattern) = selector.strip_prefix("name:") {
            let frame_name = frame.name().await?;
            return Ok(frame_name.as_deref() == Some(name_pattern));
        }

        if let Some(url_prefix) = selector.strip_prefix("url:") {
            let frame_url = frame.url().await?;
            return Ok(frame_url.starts_with(url_prefix));
        }

        if let Some(url_pattern) = selector.strip_prefix("url~") {
            let frame_url = frame.url().await?;
            // Simple pattern matching (can be enhanced with regex crate)
            return Ok(frame_url.contains(url_pattern));
        }

        if let Some(depth_str) = selector.strip_prefix("depth:")
            && let Ok(target_depth) = depth_str.parse::<usize>()
        {
            let ancestors = self.get_ancestor_frames(&frame.id).await;
            return Ok(ancestors.len() == target_depth);
        }

        Ok(false)
    }

    // ========== Frame Lifecycle Callbacks ==========

    /// Register Frame lifecycle callback
    ///
    /// # Examples
    /// ```rust
    /// use cdp_core::{Page, FrameLifecycleEvent};
    /// use std::sync::Arc;
    ///
    /// # async fn example(page: Arc<Page>) {
    /// page.on_frame_lifecycle(Arc::new(|event| {
    ///     match event {
    ///         FrameLifecycleEvent::Attached { frame_id, parent_frame_id } => {
    ///             println!("Frame attached: {}", frame_id);
    ///         }
    ///         FrameLifecycleEvent::Detached { frame_id } => {
    ///             println!("Frame detached: {}", frame_id);
    ///         }
    ///         FrameLifecycleEvent::Navigated { frame_id, url } => {
    ///             println!("Frame navigated: {} to {}", frame_id, url);
    ///         }
    ///     }
    /// })).await;
    /// # }
    /// ```
    pub async fn on_frame_lifecycle(&self, callback: FrameLifecycleCallback) {
        self.lifecycle_callbacks.lock().await.push(callback);
    }

    /// Trigger all registered lifecycle callbacks (internal use)
    pub(crate) async fn trigger_lifecycle_event(&self, event: FrameLifecycleEvent) {
        let callbacks = self.lifecycle_callbacks.lock().await;
        for callback in callbacks.iter() {
            callback(event.clone());
        }
    }

    /// Handle Frame attached event (internal use)
    pub(crate) async fn handle_frame_attached(
        &self,
        frame_id: String,
        parent_frame_id: Option<String>,
    ) {
        // Update hierarchy
        if let Some(ref parent_id) = parent_frame_id {
            self.frame_parent_map
                .lock()
                .await
                .insert(frame_id.clone(), parent_id.clone());

            self.frame_children_map
                .lock()
                .await
                .entry(parent_id.clone())
                .or_insert_with(Vec::new)
                .push(frame_id.clone());
        }

        // Trigger callback
        self.trigger_lifecycle_event(FrameLifecycleEvent::Attached {
            frame_id,
            parent_frame_id,
        })
        .await;
    }

    /// Handle Frame detached event (internal use)
    pub(crate) async fn handle_frame_detached(&self, frame_id: String) {
        // Remove from cache
        self.frames_cache.lock().await.remove(&frame_id);

        // Remove hierarchy
        let parent_id = self.frame_parent_map.lock().await.remove(&frame_id);
        if let Some(parent) = parent_id {
            let mut children_map = self.frame_children_map.lock().await;
            if let Some(children) = children_map.get_mut(&parent) {
                children.retain(|id| id != &frame_id);
            }
        }
        self.frame_children_map.lock().await.remove(&frame_id);

        // Trigger callback
        self.trigger_lifecycle_event(FrameLifecycleEvent::Detached { frame_id })
            .await;
    }

    /// Handle Frame navigated event (internal use)
    pub(crate) async fn handle_frame_navigated(&self, frame_id: String, url: String) {
        // Clear cache for this Frame
        self.frames_cache.lock().await.remove(&frame_id);

        // Trigger callback
        self.trigger_lifecycle_event(FrameLifecycleEvent::Navigated { frame_id, url })
            .await;
    }

    // ========== DOM Mutation Monitoring ==========

    /// Register DOM mutation callback
    ///
    /// # Examples
    /// ```rust
    /// use cdp_core::{Page, DomMutationEvent};
    /// use std::sync::Arc;
    ///
    /// # async fn example(page: Arc<Page>) -> anyhow::Result<()> {
    /// // Enable DOM monitoring
    /// page.enable_dom_mutations().await?;
    ///
    /// // Register callback
    /// page.on_dom_mutation(Arc::new(|event| {
    ///     match event {
    ///         DomMutationEvent::ChildNodeInserted { parent_node_id, .. } => {
    ///             println!("Child node inserted into {}", parent_node_id);
    ///         }
    ///         DomMutationEvent::AttributeModified { node_id, name, value } => {
    ///             println!("Attribute {} = {} on node {}", name, value, node_id);
    ///         }
    ///         _ => {}
    ///     }
    /// })).await;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn on_dom_mutation(&self, callback: DomMutationCallback) {
        self.dom_mutation_callbacks.lock().await.push(callback);
    }

    /// Enable DOM mutation monitoring
    pub async fn enable_dom_mutations(&self) -> Result<()> {
        use cdp_protocol::dom;

        // Enable DOM domain
        let enable = dom::Enable {
            include_whitespace: None,
        };
        self.session
            .send_command::<_, dom::EnableReturnObject>(enable, None)
            .await?;

        println!("DOM mutation monitoring enabled");
        Ok(())
    }

    /// Trigger all registered DOM mutation callbacks (internal use)
    pub(crate) async fn trigger_dom_mutation_event(&self, event: DomMutationEvent) {
        let callbacks = self.dom_mutation_callbacks.lock().await;
        for callback in callbacks.iter() {
            callback(event.clone());
        }
    }

    /// Handle DOM child node inserted event (internal use)
    pub(crate) async fn handle_child_node_inserted(
        &self,
        parent_node_id: u32,
        previous_node_id: u32,
        node: serde_json::Value,
    ) {
        self.trigger_dom_mutation_event(DomMutationEvent::ChildNodeInserted {
            parent_node_id,
            previous_node_id,
            node,
        })
        .await;
    }

    /// Handle DOM child node removed event (internal use)
    pub(crate) async fn handle_child_node_removed(&self, parent_node_id: u32, node_id: u32) {
        self.trigger_dom_mutation_event(DomMutationEvent::ChildNodeRemoved {
            parent_node_id,
            node_id,
        })
        .await;
    }

    /// Handle DOM attribute modified event (internal use)
    pub(crate) async fn handle_attribute_modified(
        &self,
        node_id: u32,
        name: String,
        value: String,
    ) {
        self.trigger_dom_mutation_event(DomMutationEvent::AttributeModified {
            node_id,
            name,
            value,
        })
        .await;
    }

    /// Handle DOM attribute removed event (internal use)
    pub(crate) async fn handle_attribute_removed(&self, node_id: u32, name: String) {
        self.trigger_dom_mutation_event(DomMutationEvent::AttributeRemoved { node_id, name })
            .await;
    }

    /// Handle character data modified event (internal use)
    pub(crate) async fn handle_character_data_modified(
        &self,
        node_id: u32,
        character_data: String,
    ) {
        self.trigger_dom_mutation_event(DomMutationEvent::CharacterDataModified {
            node_id,
            character_data,
        })
        .await;
    }
}

// ========== Frame Snapshot Functionality ==========

/// Frame snapshot data
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct FrameSnapshot {
    /// Frame ID
    pub frame_id: String,
    /// Frame URL
    pub url: String,
    /// Frame Name
    pub name: Option<String>,
    /// Parent Frame ID
    pub parent_frame_id: Option<String>,
    /// Child Frame IDs
    pub child_frame_ids: Vec<String>,
    /// Snapshot timestamp
    pub timestamp: u64,
    /// HTML content (optional)
    pub html_content: Option<String>,
    /// DOM tree depth
    pub dom_depth: Option<usize>,
}

impl Page {
    /// Create Frame snapshot
    ///
    /// # Arguments
    /// * `frame_id` - Frame ID
    /// * `include_html` - Whether to include HTML content
    ///
    /// # Examples
    /// ```rust
    /// # use cdp_core::Page;
    /// # use std::sync::Arc;
    /// # async fn example(page: Arc<Page>) -> anyhow::Result<()> {
    /// let main_frame = page.main_frame().await?;
    ///
    /// // Create snapshot (without HTML)
    /// let snapshot = page.create_frame_snapshot(&main_frame.id, false).await?;
    /// println!("Snapshot: {:?}", snapshot);
    ///
    /// // Create snapshot (with HTML)
    /// let snapshot_with_html = page.create_frame_snapshot(&main_frame.id, true).await?;
    /// println!("HTML length: {}", snapshot_with_html.html_content.as_ref().map(|s| s.len()).unwrap_or(0));
    /// # Ok(())
    /// # }
    /// ```
    pub async fn create_frame_snapshot(
        &self,
        frame_id: &str,
        include_html: bool,
    ) -> Result<FrameSnapshot> {
        let frame = self
            .get_frame(frame_id)
            .await
            .ok_or_else(|| CdpError::page(format!("Frame not found: {}", frame_id)))?;

        let url = frame.url().await.unwrap_or_else(|_| "".to_string());
        let name = frame.name().await.unwrap_or(None);
        let parent_frame_id = self.frame_parent_map.lock().await.get(frame_id).cloned();
        let child_frame_ids = self
            .frame_children_map
            .lock()
            .await
            .get(frame_id)
            .cloned()
            .unwrap_or_default();

        let html_content = if include_html {
            match frame.evaluate("document.documentElement.outerHTML").await {
                Ok(html_value) => html_value.as_str().map(|s| s.to_string()),
                Err(_) => None,
            }
        } else {
            None
        };

        let dom_depth = if include_html {
            match frame
                .evaluate(
                    r#"
                    (function() {
                        function getDepth(element) {
                            if (!element || !element.children) return 0;
                            let maxDepth = 0;
                            for (let child of element.children) {
                                maxDepth = Math.max(maxDepth, getDepth(child));
                            }
                            return maxDepth + 1;
                        }
                        return getDepth(document.documentElement);
                    })()
                "#,
                )
                .await
            {
                Ok(depth_value) => depth_value.as_u64().map(|d| d as usize),
                Err(_) => None,
            }
        } else {
            None
        };

        let timestamp = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs();

        Ok(FrameSnapshot {
            frame_id: frame_id.to_string(),
            url,
            name,
            parent_frame_id,
            child_frame_ids,
            timestamp,
            html_content,
            dom_depth,
        })
    }

    /// Create snapshots for all Frames
    ///
    /// # Examples
    /// ```rust
    /// # use cdp_core::Page;
    /// # use std::sync::Arc;
    /// # async fn example(page: Arc<Page>) -> anyhow::Result<()> {
    /// let snapshots = page.create_all_frames_snapshot(false).await?;
    /// println!("Created {} snapshots", snapshots.len());
    ///
    /// // Save as JSON
    /// let json = serde_json::to_string_pretty(&snapshots)?;
    /// std::fs::write("frame_snapshots.json", json)?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn create_all_frames_snapshot(
        self: &Arc<Self>,
        include_html: bool,
    ) -> Result<Vec<FrameSnapshot>> {
        let frames = self.all_frames().await?;
        let mut snapshots = Vec::new();

        for frame in frames {
            match self.create_frame_snapshot(&frame.id, include_html).await {
                Ok(snapshot) => snapshots.push(snapshot),
                Err(e) => {
                    eprintln!("Failed to create snapshot for frame {}: {}", frame.id, e);
                }
            }
        }

        Ok(snapshots)
    }

    /// Compare differences between two snapshots
    ///
    /// # Examples
    /// ```rust
    /// # use cdp_core::{Page, FrameSnapshot};
    /// # use std::sync::Arc;
    /// # async fn example(page: Arc<Page>) -> anyhow::Result<()> {
    /// let snapshot1 = page.create_all_frames_snapshot(false).await?;
    ///
    /// // Wait for some changes...
    /// tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
    ///
    /// let snapshot2 = page.create_all_frames_snapshot(false).await?;
    ///
    /// for (snap1, snap2) in snapshot1.iter().zip(snapshot2.iter()) {
    ///     let diff = Page::compare_snapshots(snap1, snap2);
    ///     println!("Frame {}: {} changes", snap1.frame_id, diff.len());
    /// }
    /// # Ok(())
    /// # }
    /// ```
    pub fn compare_snapshots(snapshot1: &FrameSnapshot, snapshot2: &FrameSnapshot) -> Vec<String> {
        let mut differences = Vec::new();

        if snapshot1.frame_id != snapshot2.frame_id {
            differences.push(format!(
                "Frame ID changed: {} -> {}",
                snapshot1.frame_id, snapshot2.frame_id
            ));
        }

        if snapshot1.url != snapshot2.url {
            differences.push(format!(
                "URL changed: {} -> {}",
                snapshot1.url, snapshot2.url
            ));
        }

        if snapshot1.name != snapshot2.name {
            differences.push(format!(
                "Name changed: {:?} -> {:?}",
                snapshot1.name, snapshot2.name
            ));
        }

        if snapshot1.parent_frame_id != snapshot2.parent_frame_id {
            differences.push(format!(
                "Parent Frame ID changed: {:?} -> {:?}",
                snapshot1.parent_frame_id, snapshot2.parent_frame_id
            ));
        }

        if snapshot1.child_frame_ids != snapshot2.child_frame_ids {
            differences.push(format!(
                "Child Frames changed: {} -> {}",
                snapshot1.child_frame_ids.len(),
                snapshot2.child_frame_ids.len()
            ));
        }

        if let (Some(html1), Some(html2)) = (&snapshot1.html_content, &snapshot2.html_content)
            && html1 != html2
        {
            differences.push(format!(
                "HTML content changed: {} bytes -> {} bytes",
                html1.len(),
                html2.len()
            ));
        }

        if snapshot1.dom_depth != snapshot2.dom_depth {
            differences.push(format!(
                "DOM depth changed: {:?} -> {:?}",
                snapshot1.dom_depth, snapshot2.dom_depth
            ));
        }

        differences
    }

    /// Type text into current focused element (insert all text at once)
    ///
    /// This method uses CDP's `Input.insertText` command to insert all text at once.
    /// Suitable for fast input, but does not simulate real user character-by-character input.
    ///
    /// # Parameters
    /// * `text` - Text to type
    ///
    /// # Examples
    /// ```no_run
    /// # use cdp_core::Page;
    /// # use std::sync::Arc;
    /// # async fn example(page: Arc<Page>) -> anyhow::Result<()> {
    /// // Focus on input box first
    /// if let Some(input) = page.query_selector("input[type='text']").await? {
    ///     input.click().await?;
    /// }
    ///
    /// // Fast type text
    /// page.type_text("Hello, World!").await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn type_text(&self, text: &str) -> Result<()> {
        self.keyboard().insert_text(text).await
    }

    /// Type text into current focused element character by character, with random delay between each character
    ///
    /// This method simulates real user input behavior, each character triggers keyDown, keyUp events.
    /// Delay time is randomly generated within `[min_delay_ms, max_delay_ms]`.
    ///
    /// # Parameters
    /// * `text` - Text to type
    /// * `min_delay_ms` - Minimum delay (ms)
    /// * `max_delay_ms` - Maximum delay (ms)
    ///
    /// # Examples
    /// ```no_run
    /// # use cdp_core::Page;
    /// # use std::sync::Arc;
    /// # async fn example(page: Arc<Page>) -> anyhow::Result<()> {
    /// // Focus on input box first
    /// if let Some(input) = page.query_selector("input[type='text']").await? {
    ///     input.click().await?;
    /// }
    ///
    /// // Type with random delay
    /// page.type_text_with_delay("Hello, World!", 50, 150).await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn type_text_with_delay(
        &self,
        text: &str,
        min_delay_ms: u64,
        max_delay_ms: u64,
    ) -> Result<()> {
        self.keyboard()
            .type_text_with_delay(text, min_delay_ms, max_delay_ms)
            .await
    }

    /// Get device pixel ratio (DPR) of current page
    ///
    /// # Returns
    /// Device pixel ratio, usually 1.0, 1.5, 2.0, 3.0 etc.
    async fn get_device_pixel_ratio(&self) -> Result<f64> {
        let eval_result = self
            .session
            .send_command::<_, EvaluateReturnObject>(
                Evaluate {
                    expression: "window.devicePixelRatio".to_string(),
                    object_group: None,
                    include_command_line_api: None,
                    silent: None,
                    context_id: None,
                    return_by_value: Some(true),
                    generate_preview: None,
                    user_gesture: None,
                    await_promise: None,
                    throw_on_side_effect: None,
                    timeout: None,
                    disable_breaks: None,
                    repl_mode: None,
                    allow_unsafe_eval_blocked_by_csp: None,
                    unique_context_id: None,
                    serialization_options: None,
                },
                None,
            )
            .await?;

        let dpr = eval_result
            .result
            .value
            .and_then(|v| v.as_f64())
            .unwrap_or(1.0);

        Ok(dpr)
    }

    /// Takes a screenshot of the page.
    ///
    /// Takes a screenshot of the page.
    ///
    /// # Parameters
    /// * `full_page` - Whether to capture full page (including scroll area). If false, only capture current viewport
    /// * `save_path` - Optional save path (including filename). If None, save to current directory with name `screenshot_timestamp.png`
    ///
    /// # Returns
    /// Saved file path
    ///
    /// # Examples
    /// ```no_run
    /// # use cdp_core::Page;
    /// # use std::sync::Arc;
    /// # async fn example(page: Arc<Page>) -> anyhow::Result<()> {
    /// // Capture current viewport and save automatically
    /// let path = page.screenshot(false, None).await?;
    /// println!("Screenshot saved to: {}", path);
    ///
    /// // Capture full page and save to specified path
    /// let path = page.screenshot(true, Some("screenshots/fullpage.png".into())).await?;
    /// println!("Full page screenshot saved to: {}", path);
    /// # Ok(())
    /// # }
    /// ```
    pub async fn screenshot(&self, full_page: bool, save_path: Option<PathBuf>) -> Result<String> {
        self.screenshot_with_options(full_page, save_path, true)
            .await
    }

    /// Takes a screenshot of the page with custom options.
    ///
    /// Takes a screenshot of the page with custom options.
    ///
    /// # Parameters
    /// * `full_page` - Whether to capture full page (including scroll area). If false, only capture current viewport
    /// * `save_path` - Optional save path (including filename). If None, save to current directory with name `screenshot_timestamp.png`
    /// * `auto_resolve_dpr` - Whether to automatically adapt to device pixel ratio. If true, automatically detect and use actual DPR to avoid screenshot being too large or distorted
    ///
    /// # Returns
    /// Saved file path
    ///
    /// # Examples
    /// ```no_run
    /// # use cdp_core::Page;
    /// # use std::sync::Arc;
    /// # async fn example(page: Arc<Page>) -> anyhow::Result<()> {
    /// // Capture full page, do not adapt DPR (use fixed 1.0, compatible with old behavior)
    /// let path = page.screenshot_with_options(true, None, false).await?;
    ///
    /// // Capture full page, adapt DPR (recommended, avoid screenshot being too large)
    /// let path = page.screenshot_with_options(true, None, true).await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn screenshot_with_options(
        &self,
        full_page: bool,
        save_path: Option<PathBuf>,
        auto_resolve_dpr: bool,
    ) -> Result<String> {
        use base64::Engine;
        use cdp_protocol::page as page_cdp;

        // 1. Get device pixel ratio
        let device_scale = if auto_resolve_dpr {
            let dpr = self.get_device_pixel_ratio().await?;
            // Ensure DPR is within reasonable range (0.5 to 3.0)
            dpr.clamp(0.5, 3.0)
        } else {
            1.0
        };

        let mut clip: Option<Viewport> = None;
        let layout_metrics_params = page_cdp::GetLayoutMetrics(None);
        let layout_metrics: page_cdp::GetLayoutMetricsReturnObject = self
            .session
            .send_command(layout_metrics_params, None)
            .await?;
        let visual_viewport_scale = layout_metrics.visual_viewport.scale;
        let device_scale_factor = device_scale.max(visual_viewport_scale);
        tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;

        if let Some(mut c) = clip.as_ref().cloned() {
            let dpr = c.scale.max(1.0);
            let to_device = |v: f64| (v * dpr).round();
            let x_dev = to_device(c.x);
            let y_dev = to_device(c.y);
            let x_px = (x_dev.floor() - 1.0).max(0.0);
            let y_px = (y_dev.floor() - 1.0).max(0.0);
            let right_px = to_device(c.x + c.width).ceil();
            let bottom_px = to_device(c.y + c.height).ceil();
            let w_px = (right_px - x_px).max(1.0);
            let h_px = (bottom_px - y_px).max(1.0);
            c.x = x_px / dpr;
            c.y = y_px / dpr;
            c.width = w_px / dpr;
            c.height = h_px / dpr;
            c.scale = device_scale_factor;
            clip = Some(c);
        }

        // 2. Capture screenshot
        let screenshot_params = page_cdp::CaptureScreenshot {
            format: Some(page_cdp::CaptureScreenshotFormatOption::Png),
            quality: None,
            clip,
            from_surface: Some(true),
            capture_beyond_viewport: Some(full_page),
            optimize_for_speed: None,
        };

        let result: page_cdp::CaptureScreenshotReturnObject =
            self.session.send_command(screenshot_params, None).await?;

        // 4. Generate save path
        let out_path_buf: std::path::PathBuf = match save_path {
            Some(pv) => {
                if pv.parent().is_none_or(|p| p.as_os_str().is_empty()) {
                    std::env::current_dir()?.join(pv)
                } else {
                    if let Some(parent) = pv.parent() {
                        std::fs::create_dir_all(parent)?;
                    }
                    pv.to_path_buf()
                }
            }
            None => {
                let out_dir = std::env::var("OUT_PATH").unwrap_or_else(|_| ".".to_string());
                let dir = std::path::PathBuf::from(out_dir);
                std::fs::create_dir_all(&dir)?;
                let nanos = std::time::SystemTime::now()
                    .duration_since(std::time::UNIX_EPOCH)
                    .unwrap_or_default()
                    .as_nanos();
                dir.join(format!("screenshot-{}.png", nanos))
            }
        };

        // 3. Decode and save file
        let bytes = base64::engine::general_purpose::STANDARD
            .decode(&result.data)
            .map_err(|err| {
                CdpError::page(format!("Failed to decode screenshot response: {}", err))
            })?;
        let mut f = File::create(&out_path_buf).await?;
        f.write_all(&bytes).await?;
        f.flush().await?;

        let out_path = out_path_buf.to_string_lossy();
        Ok(out_path.into_owned())
    }

    // ========== Wait Functionality ==========

    /// Wait for element matching selector to appear
    ///
    /// # Parameters
    /// * `selector` - CSS selector or XPath expression
    /// * `options` - Wait options (timeout, visibility, etc.)
    ///
    /// # Returns
    /// Matching element handle
    ///
    /// # Examples
    /// ```no_run
    /// # use cdp_core::{Page, WaitForSelectorOptions};
    /// # use std::sync::Arc;
    /// # async fn example(page: Arc<Page>) -> anyhow::Result<()> {
    /// // Wait for element to appear (default 30s timeout)
    /// let button = page.wait_for_selector("#submit-btn", None).await?;
    ///
    /// // Custom timeout and visibility requirements
    /// let element = page.wait_for_selector(
    ///     ".dynamic-content",
    ///     Some(WaitForSelectorOptions {
    ///         timeout_ms: Some(5000),
    ///         visible: Some(true),
    ///         hidden: Some(false),
    ///     })
    /// ).await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn wait_for_selector(
        self: &Arc<Self>,
        selector: &str,
        options: Option<WaitForSelectorOptions>,
    ) -> Result<ElementHandle> {
        let opts = options.unwrap_or_default();
        let timeout_ms = opts.timeout_ms.unwrap_or(30000);
        let visible = opts.visible.unwrap_or(false);
        let hidden = opts.hidden.unwrap_or(false);

        let start = std::time::Instant::now();
        let poll_interval = std::time::Duration::from_millis(100);

        loop {
            // Check timeout
            if start.elapsed().as_millis() > timeout_ms as u128 {
                return Err(CdpError::page(format!(
                    "Timeout waiting for selector '{}' ({}ms)",
                    selector, timeout_ms
                )));
            }

            // Try to query element
            if let Some(element) = self.query_selector(selector).await? {
                // If visibility check is needed
                if visible
                    && let Ok(is_visible) = element.is_visible().await
                    && !is_visible
                {
                    tokio::time::sleep(poll_interval).await;
                    continue;
                }

                // If hidden check is needed
                if hidden
                    && let Ok(is_visible) = element.is_visible().await
                    && is_visible
                {
                    tokio::time::sleep(poll_interval).await;
                    continue;
                }

                return Ok(element);
            }

            tokio::time::sleep(poll_interval).await;
        }
    }

    /// Wait for element matching selector to disappear or be hidden
    ///
    /// # Parameters
    /// * `selector` - CSS selector or XPath expression
    /// * `timeout_ms` - Timeout in milliseconds, default 30000
    ///
    /// # Examples
    /// ```no_run
    /// # use cdp_core::Page;
    /// # use std::sync::Arc;
    /// # async fn example(page: Arc<Page>) -> anyhow::Result<()> {
    /// // Wait for loading spinner to disappear
    /// page.wait_for_selector_hidden(".loading-spinner", None).await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn wait_for_selector_hidden(
        self: &Arc<Self>,
        selector: &str,
        timeout_ms: Option<u64>,
    ) -> Result<()> {
        let timeout = timeout_ms.unwrap_or(30000);
        let start = std::time::Instant::now();
        let poll_interval = std::time::Duration::from_millis(100);

        loop {
            if start.elapsed().as_millis() > timeout as u128 {
                return Err(CdpError::page(format!(
                    "Timeout waiting for selector '{}' to be hidden ({}ms)",
                    selector, timeout
                )));
            }

            match self.query_selector(selector).await? {
                None => return Ok(()), // Element does not exist, considered hidden
                Some(element) => {
                    // Check if element is visible
                    if let Ok(is_visible) = element.is_visible().await
                        && !is_visible
                    {
                        return Ok(());
                    }
                }
            }

            tokio::time::sleep(poll_interval).await;
        }
    }

    /// Wait for custom function to return true
    ///
    /// # Parameters
    /// * `function` - JavaScript function string, should return boolean
    /// * `timeout_ms` - Timeout in milliseconds, default 30000
    /// * `poll_interval_ms` - Polling interval (ms), default 100
    ///
    /// # Examples
    /// ```no_run
    /// # use cdp_core::Page;
    /// # use std::sync::Arc;
    /// # async fn example(page: Arc<Page>) -> anyhow::Result<()> {
    /// // Wait for page title to change
    /// page.wait_for_function(
    ///     "() => document.title === 'Loaded'",
    ///     Some(5000),
    ///     None
    /// ).await?;
    ///
    /// // Wait for global variable
    /// page.wait_for_function(
    ///     "() => window.myApp && window.myApp.ready",
    ///     None,
    ///     None
    /// ).await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn wait_for_function(
        self: &Arc<Self>,
        function: &str,
        timeout_ms: Option<u64>,
        poll_interval_ms: Option<u64>,
    ) -> Result<()> {
        let timeout = timeout_ms.unwrap_or(30000);
        let poll_interval = std::time::Duration::from_millis(poll_interval_ms.unwrap_or(100));
        let start = std::time::Instant::now();

        // Construct wrapper script: supports passing function or expression
        let script = format!(
            r#"
                (() => {{
                    try {{
                        const candidate = ({});
                        const value = typeof candidate === 'function' ? candidate() : candidate;
                        if (value && typeof value.then === 'function') {{
                            return value.then(Boolean).catch(() => false);
                        }}
                        return Boolean(value);
                    }} catch (error) {{
                        return false;
                    }}
                }})()
            "#,
            function
        );

        loop {
            if start.elapsed().as_millis() > timeout as u128 {
                return Err(CdpError::page(format!(
                    "Timeout waiting for function ({}ms)",
                    timeout
                )));
            }

            // Execute function
            let main_frame = match self.main_frame().await {
                Ok(frame) => frame,
                Err(_) => {
                    tokio::time::sleep(poll_interval).await;
                    continue;
                }
            };

            match main_frame.evaluate(&script).await {
                Ok(result) => {
                    if let Some(true) = result.as_bool() {
                        return Ok(());
                    }
                }
                Err(_) => {
                    // Execution context may not be ready, retry later
                }
            }

            tokio::time::sleep(poll_interval).await;
        }
    }

    /// Wait for page navigation to complete
    ///
    /// Supports multiple wait conditions:
    /// - **Load**: Wait for `load` event (all resources loaded)
    /// - **DOMContentLoaded**: Wait for DOM tree construction to complete
    /// - **NetworkIdle0**: Wait for network to be fully idle (no requests for 500ms)
    /// - **NetworkIdle2**: Wait for network to be almost idle (<= 2 requests for 500ms)
    ///
    /// # Parameters
    /// * `options` - Navigation wait options (timeout, wait condition, etc.)
    ///
    /// # Examples
    /// ```no_run
    /// # use cdp_core::{Page, WaitForNavigationOptions, WaitUntil};
    /// # use std::sync::Arc;
    /// # async fn example(page: Arc<Page>) -> anyhow::Result<()> {
    /// // Wait for load event (default)
    /// page.wait_for_navigation(None).await?;
    ///
    /// // Wait for DOMContentLoaded (faster)
    /// page.wait_for_navigation(Some(WaitForNavigationOptions {
    ///     timeout_ms: Some(5000),
    ///     wait_until: Some(WaitUntil::DOMContentLoaded),
    /// })).await?;
    ///
    /// // Wait for network idle (suitable for SPA)
    /// page.wait_for_navigation(Some(WaitForNavigationOptions {
    ///     timeout_ms: Some(10000),
    ///     wait_until: Some(WaitUntil::NetworkIdle2),
    /// })).await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn wait_for_navigation(
        self: &Arc<Self>,
        options: Option<WaitForNavigationOptions>,
    ) -> Result<()> {
        let opts = options.unwrap_or_default();
        let timeout = opts.timeout_ms.unwrap_or(60000);
        let wait_until = opts.wait_until.unwrap_or(WaitUntil::NetworkIdle2);

        match wait_until {
            WaitUntil::Load => {
                // Wait for load event
                self.wait_for_function(
                    "() => document.readyState === 'complete'",
                    Some(timeout),
                    None,
                )
                .await
            }
            WaitUntil::DOMContentLoaded => {
                // Wait for DOMContentLoaded event
                self.wait_for_function(
                    "() => document.readyState === 'interactive'",
                    Some(timeout),
                    None,
                )
                .await
            }
            WaitUntil::NetworkIdle0 => {
                // Wait for network fully idle
                self.wait_for_network_idle(timeout, 0).await
            }
            WaitUntil::NetworkIdle2 => {
                // Wait for network almost idle
                self.wait_for_network_idle(timeout, 2).await
            }
        }
    }

    /// Register network event callback
    ///
    /// # Examples
    /// ```ignore
    /// # use cdp_core::{Page, NetworkEvent};
    /// # use std::sync::Arc;
    /// # async fn example(page: Arc<Page>) -> anyhow::Result<()> {
    /// page.enable_network_monitoring().await?;
    ///
    /// page.on_network(Arc::new(|event| {
    ///     match event {
    ///         NetworkEvent::RequestWillBeSent { url, method, .. } => {
    ///             println!("[{}] {}", method, url);
    ///         }
    ///         NetworkEvent::ResponseReceived { request_id, status, .. } => {
    ///             println!("[{}] Status: {}", request_id, status);
    ///         }
    ///         NetworkEvent::LoadingFailed { request_id, error_text } => {
    ///             eprintln!("[{}] Failed: {}", request_id, error_text);
    ///         }
    ///         _ => {}
    ///     }
    /// })).await;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn on_network(&self, callback: NetworkEventCallback) {
        self.network_monitor.add_callback(callback).await;
    }

    /// Get count of currently active network requests
    ///
    /// # Examples
    /// ```ignore
    /// # use cdp_core::Page;
    /// # use std::sync::Arc;
    /// # async fn example(page: Arc<Page>) -> anyhow::Result<()> {
    /// page.enable_network_monitoring().await?;
    ///
    /// let count = page.get_inflight_requests_count();
    /// println!("Active requests: {}", count);
    /// # Ok(())
    /// # }
    /// ```
    pub fn get_inflight_requests_count(&self) -> usize {
        self.network_monitor.get_inflight_count()
    }

    /// Wait for network idle
    ///
    /// # Parameters
    /// * `timeout_ms` - Timeout (ms)
    /// * `max_inflight` - Maximum allowed concurrent requests (0 = fully idle, 2 = almost idle)
    ///
    /// # Implementation Details
    /// This method listens for network request events, considering network idle when concurrent requests <= max_inflight for at least 500ms
    async fn wait_for_network_idle(
        self: &Arc<Self>,
        timeout_ms: u64,
        max_inflight: usize,
    ) -> Result<()> {
        // Ensure network monitoring is enabled
        if !self.domain_manager.is_enabled(DomainType::Network).await {
            self.domain_manager.enable_network_domain().await?;
        }

        // Reset count (avoid influence from previous requests)
        self.network_monitor.reset_inflight().await;

        // Wait for network idle
        let start = std::time::Instant::now();
        let mut idle_start: Option<std::time::Instant> = None;
        let idle_duration = std::time::Duration::from_millis(500); // 500ms idle time

        loop {
            if start.elapsed().as_millis() > timeout_ms as u128 {
                return Ok(());
            }

            let current_inflight = self.network_monitor.get_inflight_count();
            tracing::debug!("Current active requests: {}", current_inflight);

            if current_inflight <= max_inflight {
                // Network is idle, ensure timer exists
                let start_time = idle_start.get_or_insert_with(std::time::Instant::now);

                // Check if idle for enough time
                if start_time.elapsed() >= idle_duration {
                    return Ok(());
                }
            } else {
                // Network not idle, reset timer
                idle_start = None;
            }

            tokio::time::sleep(std::time::Duration::from_millis(100)).await;
        }
    }

    /// Wait for specified time (ms)
    ///
    /// # Examples
    /// ```no_run
    /// # use cdp_core::Page;
    /// # use std::sync::Arc;
    /// # async fn example(page: Arc<Page>) -> anyhow::Result<()> {
    /// // Wait for 2 seconds
    /// page.wait_for_timeout(2000).await;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn wait_for_timeout(&self, ms: u64) {
        tokio::time::sleep(std::time::Duration::from_millis(ms)).await;
    }

    /// Clean up Page resources (explicit async cleanup)
    ///
    /// This method will:
    /// 1. Clear all network monitors and interceptors
    /// 2. Disable all enabled CDP Domains
    /// 3. Clear Frame cache and callbacks
    ///
    /// # Best Practices
    ///
    /// Although Page automatically cleans up on Drop, it is recommended to call this method explicitly when Page is no longer used:
    ///
    /// ```no_run
    /// # use cdp_core::Browser;
    /// # async fn example() -> anyhow::Result<()> {
    /// let browser = Browser::launcher().launch().await?;
    /// let page = browser.new_page().await?;
    ///
    /// // Use page...
    ///
    /// // Explicitly cleanup when no longer used
    /// page.cleanup().await?;
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// # Note
    ///
    /// - After calling this method, Page will be in an unusable state
    /// - If Page is shared via `Arc`, ensure no other places are still using it
    /// - Cleanup is also performed automatically on Drop, but explicit call gives better control over timing and error handling
    pub async fn cleanup(&self) -> Result<()> {
        tracing::info!("Starting Page resource cleanup...");

        // 1. Clear network monitors
        self.response_monitor_manager.clear_monitors().await;
        tracing::debug!("Network monitors cleared");

        // 2. Clear callbacks
        self.lifecycle_callbacks.lock().await.clear();
        self.dom_mutation_callbacks.lock().await.clear();
        tracing::debug!("Lifecycle and DOM mutation callbacks cleared");

        // 3. Clear Frame cache
        self.main_frame_cache.lock().await.take();
        self.frames_cache.lock().await.clear();
        self.frame_parent_map.lock().await.clear();
        self.frame_children_map.lock().await.clear();
        tracing::debug!("Frame cache cleared");

        // 4. Disable all CDP Domains
        self.domain_manager.disable_all_domains().await?;
        tracing::debug!("All CDP Domains disabled");

        tracing::info!("Page resource cleanup completed");
        Ok(())
    }
}

/// RAII automatic cleanup implementation
///
/// Automatically cleans up all resources when Page is dropped
impl Drop for Page {
    fn drop(&mut self) {
        tracing::debug!("Page Drop triggered, starting automatic cleanup...");

        // Since Drop is synchronous and our cleanup is asynchronous,
        // we need to execute cleanup in the current tokio runtime
        if let Ok(handle) = tokio::runtime::Handle::try_current() {
            // Spawn a task in tokio runtime to execute cleanup
            let domain_manager = Arc::clone(&self.domain_manager);
            let response_monitor_manager = Arc::clone(&self.response_monitor_manager);
            let lifecycle_callbacks = Arc::clone(&self.lifecycle_callbacks);
            let dom_mutation_callbacks = Arc::clone(&self.dom_mutation_callbacks);
            let main_frame_cache = Arc::clone(&self.main_frame_cache);
            let frames_cache = Arc::clone(&self.frames_cache);
            let frame_parent_map = Arc::clone(&self.frame_parent_map);
            let frame_children_map = Arc::clone(&self.frame_children_map);

            handle.spawn(async move {
                // Clear network monitors
                response_monitor_manager.clear_monitors().await;

                // Clear callbacks
                lifecycle_callbacks.lock().await.clear();
                dom_mutation_callbacks.lock().await.clear();

                // Clear Frame cache
                main_frame_cache.lock().await.take();
                frames_cache.lock().await.clear();
                frame_parent_map.lock().await.clear();
                frame_children_map.lock().await.clear();

                // Disable all Domains
                if let Err(e) = domain_manager.disable_all_domains().await {
                    tracing::warn!("Failed to disable Domains during Page Drop: {:?}", e);
                }

                tracing::debug!("Page automatic cleanup completed");
            });
        } else {
            // If not in tokio runtime, just print warning
            tracing::warn!("Cannot perform async cleanup during Page Drop (not in tokio runtime)");
            tracing::warn!("Recommended to call page.cleanup().await explicitly");
        }
    }
}
