use super::page_core::Page;
use crate::error::{CdpError, Result};
use crate::page::element;
use cdp_protocol::runtime;
use serde_json::Value;
use std::sync::Arc;
use tokio::time::{Duration, sleep};

/// Retry configuration shared by frame helpers.
#[derive(Clone, Debug)]
pub struct RetryConfig {
    /// Maximum number of retries.
    pub max_retries: u32,
    /// Initial delay in milliseconds before retrying.
    pub initial_delay_ms: u64,
    /// Backoff multiplier applied between attempts (exponential backoff).
    pub backoff_multiplier: f64,
    /// Maximum delay in milliseconds between attempts.
    pub max_delay_ms: u64,
}

impl Default for RetryConfig {
    fn default() -> Self {
        Self {
            max_retries: 3,
            initial_delay_ms: 100,
            backoff_multiplier: 2.0,
            max_delay_ms: 5000,
        }
    }
}

impl RetryConfig {
    /// Creates a conservative retry profile (more attempts, longer delays).
    pub fn conservative() -> Self {
        Self {
            max_retries: 5,
            initial_delay_ms: 200,
            backoff_multiplier: 2.0,
            max_delay_ms: 10000,
        }
    }

    /// Creates an aggressive retry profile (fewer attempts, short delays).
    pub fn aggressive() -> Self {
        Self {
            max_retries: 2,
            initial_delay_ms: 50,
            backoff_multiplier: 1.5,
            max_delay_ms: 2000,
        }
    }

    /// Disables retries entirely.
    pub fn no_retry() -> Self {
        Self {
            max_retries: 0,
            initial_delay_ms: 0,
            backoff_multiplier: 1.0,
            max_delay_ms: 0,
        }
    }
}

#[derive(Clone)]
pub struct Frame {
    pub id: String, // frameId

    // Hold a reference to the owning page instead of duplicating state.
    pub page: Arc<Page>,
}

impl Frame {
    // Construct a frame wrapper.
    pub fn new(id: String, page: Arc<Page>) -> Self {
        Self { id, page }
    }

    /// Returns the frame identifier exposed by CDP.
    pub fn id(&self) -> &str {
        &self.id
    }

    /// Resolves the execution context associated with the frame.
    pub async fn execution_context_id(&self) -> Result<u32> {
        self.page
            .contexts
            .lock()
            .await
            .get(&self.id)
            .cloned()
            .ok_or_else(|| {
                CdpError::frame(format!("Execution context not found for frame {}", self.id))
            })
    }

    /// Calls JavaScript within the frame's execution context.
    pub async fn call_function_on(
        &self,
        function_declaration: &str,
        args: Vec<Value>,
    ) -> Result<Value> {
        let context_id = self.execution_context_id().await?;
        let params = runtime::CallFunctionOn {
            function_declaration: function_declaration.to_string(),
            object_id: None,
            arguments: Some(
                args.into_iter()
                    .map(|v| runtime::CallArgument {
                        value: Some(v),
                        unserializable_value: None,
                        object_id: None,
                    })
                    .collect(),
            ),
            silent: None,
            return_by_value: Some(true),
            generate_preview: None,
            user_gesture: None,
            await_promise: Some(true),
            execution_context_id: Some(context_id),
            object_group: None,
            throw_on_side_effect: None,
            unique_context_id: None,
            serialization_options: None,
        };
        let result: runtime::CallFunctionOnReturnObject =
            self.page.session.send_command(params, None).await?;
        if let Some(details) = result.exception_details.as_ref() {
            return Err(CdpError::frame(format!(
                "JavaScript execution failed: {:?}",
                details
            )));
        }
        // Return the JSON payload produced by the browser.
        Ok(result.result.value.unwrap_or(Value::Null))
    }

    /// Evaluates JavaScript within the frame's execution context.
    pub async fn evaluate(&self, script: &str) -> Result<Value> {
        let context_id = self.execution_context_id().await?;
        let params = runtime::Evaluate {
            expression: script.to_string(),
            object_group: None,
            include_command_line_api: None,
            silent: None,
            context_id: Some(context_id),
            return_by_value: Some(true),
            generate_preview: None,
            user_gesture: None,
            await_promise: Some(true),
            throw_on_side_effect: None,
            timeout: None,
            disable_breaks: None,
            repl_mode: None,
            allow_unsafe_eval_blocked_by_csp: None,
            unique_context_id: None,
            serialization_options: None,
        };
        let result: runtime::EvaluateReturnObject =
            self.page.session.send_command(params, None).await?;
        if let Some(details) = result.exception_details.as_ref() {
            return Err(CdpError::frame(format!(
                "JavaScript execution failed: {:?}",
                details
            )));
        }
        Ok(result.result.value.unwrap_or(Value::Null))
    }

    /// Returns the window name for the frame, if any.
    pub async fn name(&self) -> Result<Option<String>> {
        let script = "window.name";
        let result = self.evaluate(script).await?;
        Ok(result.as_str().map(|s| s.to_string()))
    }

    /// Returns the current URL navigating inside the frame.
    pub async fn url(&self) -> Result<String> {
        let script = "window.location.href";
        let result = self.evaluate(script).await?;
        result
            .as_str()
            .map(|s| s.to_string())
            .ok_or_else(|| CdpError::frame(format!("Failed to resolve URL for frame {}", self.id)))
    }

    /// Returns `true` if the frame no longer has a valid execution context.
    pub async fn is_detached(&self) -> bool {
        // Missing context indicates the frame was detached.
        self.page.contexts.lock().await.get(&self.id).is_none()
    }

    // ===== DOM query helpers =================================================

    /// Queries the first element in the frame using either CSS or XPath.
    ///
    /// CSS selectors look like `div.class` or `#id`. XPath selectors start with
    /// `xpath:` or `/`, for example `//div[@class='test']`.
    pub async fn query_selector(&self, selector: &str) -> Result<Option<element::ElementHandle>> {
        // Treat XPath selectors specially.
        if Self::is_xpath(selector) {
            return self.query_selector_xpath(selector).await;
        }

        // Otherwise fall back to CSS selectors.
        use cdp_protocol::dom::{
            DescribeNode, DescribeNodeReturnObject, GetDocument, GetDocumentReturnObject,
            QuerySelector, QuerySelectorReturnObject,
        };

        // 1. Retrieve the root node of the frame document.
        let doc_result: GetDocumentReturnObject = self
            .page
            .session
            .send_command(
                GetDocument {
                    depth: Some(0), // Only fetch the root node.
                    pierce: Some(false),
                },
                None,
            )
            .await?;

        let root_node_id = doc_result.root.node_id;

        // 2. Use DOM.querySelector to find the first match.
        let query_result = self
            .page
            .session
            .send_command::<_, QuerySelectorReturnObject>(
                QuerySelector {
                    node_id: root_node_id,
                    selector: selector.to_string(),
                },
                None,
            )
            .await?;

        if query_result.node_id == 0 {
            return Ok(None);
        }

        let query_node_id = query_result.node_id;

        // 3. Hydrate the node into an element handle.
        let describe_result = self
            .page
            .session
            .send_command::<_, DescribeNodeReturnObject>(
                DescribeNode {
                    node_id: Some(query_node_id),
                    backend_node_id: None,
                    object_id: None,
                    depth: None,
                    pierce: None,
                },
                None,
            )
            .await?;

        Ok(Some(element::ElementHandle {
            backend_node_id: describe_result.node.backend_node_id,
            node_id: Some(query_node_id),
            page: Arc::clone(&self.page),
        }))
    }

    /// Queries all matching elements in the frame using CSS or XPath.
    pub async fn query_selector_all(&self, selector: &str) -> Result<Vec<element::ElementHandle>> {
        // Treat XPath selectors specially.
        if Self::is_xpath(selector) {
            return self.query_selector_all_xpath(selector).await;
        }

        // Otherwise fall back to CSS selectors.
        use cdp_protocol::dom::{
            DescribeNode, DescribeNodeReturnObject, GetDocument, GetDocumentReturnObject,
            QuerySelectorAll, QuerySelectorAllReturnObject,
        };

        // 1. Retrieve the root node of the frame document.
        let doc_result: GetDocumentReturnObject = self
            .page
            .session
            .send_command(
                GetDocument {
                    depth: Some(0), // Only fetch the root node.
                    pierce: Some(false),
                },
                None,
            )
            .await?;

        let root_node_id = doc_result.root.node_id;

        // 2. Collect all matching node identifiers.
        let query_result = self
            .page
            .session
            .send_command::<_, QuerySelectorAllReturnObject>(
                QuerySelectorAll {
                    node_id: root_node_id,
                    selector: selector.to_string(),
                },
                None,
            )
            .await?;

        let mut elements = Vec::new();
        for node_id in query_result.node_ids {
            if node_id == 0 {
                continue;
            }

            let describe_result = self
                .page
                .session
                .send_command::<_, DescribeNodeReturnObject>(
                    DescribeNode {
                        node_id: Some(node_id),
                        backend_node_id: None,
                        object_id: None,
                        depth: None,
                        pierce: None,
                    },
                    None,
                )
                .await?;

            elements.push(element::ElementHandle {
                backend_node_id: describe_result.node.backend_node_id,
                node_id: Some(node_id),
                page: Arc::clone(&self.page),
            });
        }

        Ok(elements)
    }

    // ===== Retry-enabled helpers ============================================

    /// Retries [`Self::call_function_on`] using the provided configuration.
    pub async fn call_function_on_with_retry(
        &self,
        function_declaration: &str,
        args: Vec<Value>,
        config: RetryConfig,
    ) -> Result<Value> {
        let mut attempt = 0;
        let mut delay_ms = config.initial_delay_ms;

        loop {
            match self
                .call_function_on(function_declaration, args.clone())
                .await
            {
                Ok(result) => return Ok(result),
                Err(err) => {
                    if attempt >= config.max_retries {
                        return Err(CdpError::frame(format!(
                            "call_function_on failed after {} retries: {}",
                            config.max_retries, err
                        )));
                    }

                    // If the frame is gone, bail out immediately.
                    if self.is_detached().await {
                        return Err(CdpError::frame(format!(
                            "Frame '{}' is detached; cannot retry",
                            self.id
                        )));
                    }

                    // Wait for the next attempt using the configured backoff.
                    sleep(Duration::from_millis(delay_ms)).await;
                    delay_ms = ((delay_ms as f64) * config.backoff_multiplier) as u64;
                    delay_ms = delay_ms.min(config.max_delay_ms);
                    attempt += 1;
                }
            }
        }
    }

    /// Retries [`Self::evaluate`] using the provided configuration.
    pub async fn evaluate_with_retry(&self, script: &str, config: RetryConfig) -> Result<Value> {
        let mut attempt = 0;
        let mut delay_ms = config.initial_delay_ms;

        loop {
            match self.evaluate(script).await {
                Ok(result) => return Ok(result),
                Err(err) => {
                    if attempt >= config.max_retries {
                        return Err(CdpError::frame(format!(
                            "evaluate failed after {} retries: {}",
                            config.max_retries, err
                        )));
                    }

                    // If the frame is gone, bail out immediately.
                    if self.is_detached().await {
                        return Err(CdpError::frame(format!(
                            "Frame '{}' is detached; cannot retry",
                            self.id
                        )));
                    }

                    // Wait for the next attempt using the configured backoff.
                    sleep(Duration::from_millis(delay_ms)).await;
                    delay_ms = ((delay_ms as f64) * config.backoff_multiplier) as u64;
                    delay_ms = delay_ms.min(config.max_delay_ms);
                    attempt += 1;
                }
            }
        }
    }

    // ===== XPath helpers =====================================================

    /// Returns true if the selector is interpreted as XPath (starts with
    /// `xpath:`, `/`, or `(`).
    fn is_xpath(selector: &str) -> bool {
        selector.starts_with("xpath:") || selector.starts_with("/") || selector.starts_with("(")
    }

    /// Strips the optional `xpath:` prefix from the selector.
    fn normalize_xpath(selector: &str) -> &str {
        selector.strip_prefix("xpath:").unwrap_or(selector)
    }

    /// XPath-powered version of [`Self::query_selector`].
    async fn query_selector_xpath(&self, xpath: &str) -> Result<Option<element::ElementHandle>> {
        use cdp_protocol::dom::{
            DescribeNode, DescribeNodeReturnObject, DiscardSearchResults, GetSearchResults,
            GetSearchResultsReturnObject, PerformSearch, PerformSearchReturnObject,
        };

        let xpath = Self::normalize_xpath(xpath);

        // 1. Execute the global search.
        let search_result: PerformSearchReturnObject = self
            .page
            .session
            .send_command(
                PerformSearch {
                    query: xpath.to_string(),
                    include_user_agent_shadow_dom: Some(true),
                },
                None,
            )
            .await?;

        let search_id = search_result.search_id;
        let result_count = search_result.result_count;

        if result_count == 0 {
            // Discard the search results to keep the browser clean.
            let _ = self
                .page
                .session
                .send_command::<_, ()>(
                    DiscardSearchResults {
                        search_id: search_id.clone(),
                    },
                    None,
                )
                .await;
            return Ok(None);
        }

        // 2. Fetch the first result only.
        let get_results: GetSearchResultsReturnObject = self
            .page
            .session
            .send_command(
                GetSearchResults {
                    search_id: search_id.clone(),
                    from_index: 0,
                    to_index: 1,
                },
                None,
            )
            .await?;

        // 3. Discard the search results to avoid leaking handles.
        let _ = self
            .page
            .session
            .send_command::<_, ()>(DiscardSearchResults { search_id }, None)
            .await;

        if get_results.node_ids.is_empty() {
            return Ok(None);
        }

        let node_id = get_results.node_ids[0];

        if node_id == 0 {
            eprintln!("Warning: XPath search returned an invalid node_id (0)");
            return Ok(None);
        }

        // 4. Describe the node and promote it to an element handle.
        let describe_result = self
            .page
            .session
            .send_command::<_, DescribeNodeReturnObject>(
                DescribeNode {
                    node_id: Some(node_id),
                    backend_node_id: None,
                    object_id: None,
                    depth: None,
                    pierce: None,
                },
                None,
            )
            .await?;

        Ok(Some(element::ElementHandle {
            backend_node_id: describe_result.node.backend_node_id,
            node_id: Some(node_id),
            page: Arc::clone(&self.page),
        }))
    }

    /// XPath-powered version of [`Self::query_selector_all`].
    async fn query_selector_all_xpath(&self, xpath: &str) -> Result<Vec<element::ElementHandle>> {
        use cdp_protocol::dom::{
            DescribeNode, DescribeNodeReturnObject, DiscardSearchResults, GetSearchResults,
            GetSearchResultsReturnObject, PerformSearch, PerformSearchReturnObject,
        };

        let xpath = Self::normalize_xpath(xpath);

        // 1. Execute the global search.
        let search_result: PerformSearchReturnObject = self
            .page
            .session
            .send_command(
                PerformSearch {
                    query: xpath.to_string(),
                    include_user_agent_shadow_dom: Some(true),
                },
                None,
            )
            .await?;

        let search_id = search_result.search_id;
        let result_count = search_result.result_count;

        if result_count == 0 {
            // Discard the search results to keep the browser clean.
            let _ = self
                .page
                .session
                .send_command::<_, ()>(
                    DiscardSearchResults {
                        search_id: search_id.clone(),
                    },
                    None,
                )
                .await;
            return Ok(Vec::new());
        }

        // 2. Fetch every node that matched the query.
        let get_results: GetSearchResultsReturnObject = self
            .page
            .session
            .send_command(
                GetSearchResults {
                    search_id: search_id.clone(),
                    from_index: 0,
                    to_index: result_count,
                },
                None,
            )
            .await?;

        // 3. Discard the search results to avoid leaking handles.
        let _ = self
            .page
            .session
            .send_command::<_, ()>(DiscardSearchResults { search_id }, None)
            .await;

        // 4. Create element handles for the retrieved node identifiers.
        let mut elements = Vec::new();
        for node_id in get_results.node_ids {
            if node_id == 0 {
                continue;
            }

            let describe_result = self
                .page
                .session
                .send_command::<_, DescribeNodeReturnObject>(
                    DescribeNode {
                        node_id: Some(node_id),
                        backend_node_id: None,
                        object_id: None,
                        depth: None,
                        pierce: None,
                    },
                    None,
                )
                .await?;

            elements.push(element::ElementHandle {
                backend_node_id: describe_result.node.backend_node_id,
                node_id: Some(node_id),
                page: Arc::clone(&self.page),
            });
        }

        Ok(elements)
    }
}
