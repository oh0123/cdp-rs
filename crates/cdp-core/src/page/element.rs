use super::page_core::Page;
use crate::error::{CdpError, Result};
use crate::input::mouse::{DoubleClickOptions, MouseClickOptions, MousePosition, PressHoldOptions};
use cdp_protocol::input::MouseButton;
use cdp_protocol::{dom, page as page_cdp, runtime};
use futures_util::StreamExt;
use serde_json::Value;
use std::{
    future::Future,
    path::{Path, PathBuf},
    sync::Arc,
    time::Duration,
};
use tokio::{fs, fs::File, io::AsyncWriteExt, time::timeout};

/// Defines the bounding box strategy used when capturing element screenshots.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ScreenshotBoxType {
    /// Content box (excludes padding).
    Content,
    /// Padding box (includes padding but excludes border).
    Padding,
    /// Border box (includes border; ideal for rounded elements).
    #[default]
    Border,
    /// Margin box (includes the element margin).
    Margin,
}

const FILE_CHOOSER_WAIT_TIMEOUT: Duration = Duration::from_secs(10);

#[derive(Clone)] // So we can easily pass it around
pub struct ElementHandle {
    /// A stable identifier for the DOM node in the browser's backend.
    /// This ID is persistent and doesn't change even if the DOM is modified.
    pub(crate) backend_node_id: u32,

    /// A temporary identifier for the DOM node.
    /// This ID is more accurate for certain operations but may become invalid after DOM changes.
    /// If None, commands will fall back to using backend_node_id.
    pub(crate) node_id: Option<u32>,

    // A shared reference to the page this element belongs to.
    // This is how we send commands related to this element.
    pub(crate) page: Arc<Page>,
}

impl ElementHandle {
    async fn set_file_chooser_intercept(page: &Arc<Page>, enabled: bool) -> Result<()> {
        page.session
            .send_command::<_, page_cdp::SetInterceptFileChooserDialogReturnObject>(
                page_cdp::SetInterceptFileChooserDialog {
                    enabled,
                    cancel: None,
                },
                None,
            )
            .await?;
        Ok(())
    }

    /// Computes the center point used for mouse interactions, ensuring the
    /// element is scrolled into view first.
    async fn interaction_point(&self) -> Result<(f64, f64)> {
        let scroll_params = dom::ScrollIntoViewIfNeeded {
            node_id: self.node_id,
            backend_node_id: if self.node_id.is_none() {
                Some(self.backend_node_id)
            } else {
                None
            },
            object_id: None,
            rect: None,
        };
        self.page
            .session
            .send_command::<_, dom::ScrollIntoViewIfNeededReturnObject>(scroll_params, None)
            .await?;

        // Use `getBoundingClientRect` to determine the on-screen centroid.
        let resolve_params = dom::ResolveNode {
            backend_node_id: Some(self.backend_node_id),
            node_id: None,
            object_group: Some("element-helper".to_string()),
            execution_context_id: None,
        };

        let resolve_result: dom::ResolveNodeReturnObject =
            self.page.session.send_command(resolve_params, None).await?;

        let object_id = resolve_result
            .object
            .object_id
            .ok_or_else(|| CdpError::element("Object ID is unavailable for element".to_string()))?;

        let params = runtime::CallFunctionOn {
            function_declaration:
                "function() { const rect = this.getBoundingClientRect(); return { x: rect.left + rect.width / 2, y: rect.top + rect.height / 2 }; }"
                    .to_string(),
            object_id: Some(object_id),
            arguments: None,
            silent: None,
            return_by_value: Some(true),
            generate_preview: None,
            user_gesture: None,
            await_promise: None,
            execution_context_id: None,
            object_group: None,
            throw_on_side_effect: None,
            unique_context_id: None,
            serialization_options: None,
        };

        let result: runtime::CallFunctionOnReturnObject =
            self.page.session.send_command(params, None).await?;

        if let Some(details) = result.exception_details.as_ref() {
            return Err(CdpError::element(format!(
                "Failed to compute interaction point: {:?}",
                details
            )));
        }

        let value = result
            .result
            .value
            .ok_or_else(|| CdpError::element("No point returned for element".to_string()))?;

        let center_x = value
            .get("x")
            .and_then(|v| v.as_f64())
            .ok_or_else(|| CdpError::element("Invalid X coordinate for element".to_string()))?;
        let center_y = value
            .get("y")
            .and_then(|v| v.as_f64())
            .ok_or_else(|| CdpError::element("Invalid Y coordinate for element".to_string()))?;

        Ok((center_x, center_y))
    }

    /// Clicks the element (left button).
    pub async fn click(&self) -> Result<()> {
        self.left_click().await
    }

    /// Left-clicks the element.
    pub async fn left_click(&self) -> Result<()> {
        let (x, y) = self.interaction_point().await?;
        self.page.mouse().left_click(x, y).await
    }

    /// Right-clicks the element.
    pub async fn right_click(&self) -> Result<()> {
        let (x, y) = self.interaction_point().await?;
        let options = MouseClickOptions {
            button: MouseButton::Right,
            ..Default::default()
        };
        self.page.mouse().click(x, y, options).await
    }

    /// Moves the mouse to the element center and returns the resulting pointer
    /// position.
    pub async fn hover(&self) -> Result<MousePosition> {
        let (x, y) = self.interaction_point().await?;
        self.page.mouse().move_to(x, y).await
    }

    /// Semantic alias for [`Self::hover`].
    pub async fn move_mouse_to(&self) -> Result<MousePosition> {
        self.hover().await
    }

    /// Double-clicks the element.
    pub async fn double_click(&self) -> Result<()> {
        let (x, y) = self.interaction_point().await?;
        self.page
            .mouse()
            .double_click(x, y, DoubleClickOptions::default())
            .await
    }

    /// Presses and holds the element for the provided duration using the left
    /// mouse button.
    pub async fn press_and_hold(&self, duration: Duration) -> Result<()> {
        let (x, y) = self.interaction_point().await?;
        self.page
            .mouse()
            .press_and_hold(x, y, MouseButton::Left, duration)
            .await
    }

    /// Presses and holds the element until the provided condition returns
    /// `true` or the configured timeout elapses.
    pub async fn press_and_hold_until<F, Fut>(
        &self,
        options: PressHoldOptions,
        condition: F,
    ) -> Result<bool>
    where
        F: FnMut() -> Fut + Send,
        Fut: Future<Output = Result<bool>> + Send,
    {
        let (x, y) = self.interaction_point().await?;
        self.page
            .mouse()
            .press_and_hold_until(x, y, options, condition)
            .await
    }

    /// Gets the text content of the element.
    pub async fn text_content(&self) -> Result<String> {
        // Step 1: resolve the backend node into an object reference.
        let resolve_params = dom::ResolveNode {
            backend_node_id: Some(self.backend_node_id),
            node_id: None,
            object_group: Some("element-helper".to_string()),
            execution_context_id: None,
        };

        let resolve_result: dom::ResolveNodeReturnObject =
            self.page.session.send_command(resolve_params, None).await?;

        let object_id = resolve_result
            .object
            .object_id
            .ok_or_else(|| CdpError::element("Object ID is unavailable for element".to_string()))?;

        // Step 2: call into the runtime object to read textContent.
        // Note: when objectId is provided executionContextId must stay `None`.
        let params = runtime::CallFunctionOn {
            function_declaration: "function() { return this.textContent; }".to_string(),
            object_id: Some(object_id),
            arguments: None,
            silent: None,
            return_by_value: Some(true),
            generate_preview: None,
            user_gesture: None,
            await_promise: None,
            execution_context_id: None, // Must remain None when using objectId
            object_group: None,
            throw_on_side_effect: None,
            unique_context_id: None,
            serialization_options: None,
        };

        let result: runtime::CallFunctionOnReturnObject =
            self.page.session.send_command(params, None).await?;

        // Step 3: propagate runtime exceptions explicitly.
        if let Some(details) = result.exception_details.as_ref() {
            return Err(CdpError::element(format!(
                "Failed to get textContent: {:?}",
                details
            )));
        }

        // Step 4: normalize the runtime value into a String.
        match result.result.value {
            Some(Value::String(s)) => Ok(s),
            Some(Value::Null) => Ok("".to_string()),
            Some(v) => Ok(v.to_string()),
            None => Ok("".to_string()),
        }
    }

    /// Inserts the entire text payload into the element in a single CDP call.
    ///
    /// The element is focused before invoking `Input.insertText`, which means
    /// the input behaves like a fast paste action instead of character-by-character typing.
    ///
    /// # Parameters
    /// * `text` - The text to insert.
    ///
    /// # Examples
    /// ```no_run
    /// # use cdp_core::Page;
    /// # use std::sync::Arc;
    /// # async fn example(page: Arc<Page>) -> anyhow::Result<()> {
    /// if let Some(input) = page.query_selector("input[type='text']").await? {
    ///     input.type_text("Hello, World!").await?;
    /// }
    /// # Ok(())
    /// # }
    /// ```
    pub async fn type_text(&self, text: &str) -> Result<()> {
        // Focus the element before issuing the CDP command.
        self.click().await?;

        // Delegate to the page-level helper.
        self.page.type_text(text).await
    }

    /// Opens the file chooser dialog and populates it with the provided paths.
    pub async fn upload_files<P>(&self, files: impl IntoIterator<Item = P>) -> Result<()>
    where
        P: AsRef<Path>,
    {
        let mut resolved: Vec<PathBuf> = Vec::new();
        for path in files.into_iter() {
            let original = PathBuf::from(path.as_ref());
            let canonical = fs::canonicalize(&original).await.map_err(|err| {
                CdpError::element(format!(
                    "Failed to resolve file path '{}': {err}",
                    original.display()
                ))
            })?;
            let metadata = fs::metadata(&canonical).await.map_err(|err| {
                CdpError::element(format!(
                    "Failed to inspect file '{}': {err}",
                    canonical.display()
                ))
            })?;
            if !metadata.is_file() {
                return Err(CdpError::element(format!(
                    "Path '{}' is not a regular file",
                    canonical.display()
                )));
            }
            resolved.push(canonical);
        }

        if resolved.is_empty() {
            return Err(CdpError::element(
                "upload_files requires at least one file path".to_string(),
            ));
        }

        let _chooser_guard = self.page.file_chooser_lock.lock().await;

        Self::set_file_chooser_intercept(&self.page, true).await?;

        let upload_result: Result<()> = async {
            let mut events = self.page.on::<page_cdp::events::FileChooserOpenedEvent>();

            self.click().await?;

            let chooser_event = timeout(FILE_CHOOSER_WAIT_TIMEOUT, events.next())
                .await
                .map_err(|_| {
                    CdpError::element(
                        "Timed out waiting for file chooser dialog to open".to_string(),
                    )
                })?
                .ok_or_else(|| {
                    CdpError::element(
                        "File chooser event stream ended before a dialog was received".to_string(),
                    )
                })?;

            if matches!(
                chooser_event.params.mode,
                page_cdp::FileChooserOpenedModeOption::SelectSingle
            ) && resolved.len() > 1
            {
                return Err(CdpError::element(
                    "File chooser allows selecting only one file but multiple paths were provided"
                        .to_string(),
                ));
            }

            let backend_node_id = chooser_event.params.backend_node_id.ok_or_else(|| {
                CdpError::element(
                    "File chooser event missing backendNodeId; cannot upload files".to_string(),
                )
            })?;

            let payload: Vec<String> = resolved
                .iter()
                .map(|path| path.to_string_lossy().into_owned())
                .collect();

            self.page
                .session
                .send_command::<_, dom::SetFileInputFilesReturnObject>(
                    dom::SetFileInputFiles {
                        files: payload,
                        node_id: None,
                        backend_node_id: Some(backend_node_id),
                        object_id: None,
                    },
                    None,
                )
                .await?;

            Ok(())
        }
        .await;

        let disable_result = Self::set_file_chooser_intercept(&self.page, false).await;

        match (upload_result, disable_result) {
            (Ok(()), Ok(())) => Ok(()),
            (Err(err), Ok(())) => Err(err),
            (Ok(()), Err(err)) => Err(err),
            (Err(err), Err(disable_err)) => {
                tracing::warn!(
                    "Failed to disable file chooser interception after error: {:?}",
                    disable_err
                );
                Err(err)
            }
        }
    }

    /// Clears the element's value or selection state (input, textarea, select,
    /// form, and contentEditable are supported).
    ///
    /// This implementation mimics real user interactions where possible:
    /// - Text inputs and textareas reset `value = ""` and emit `input` / `change`
    /// - Checkboxes and radio buttons are unchecked
    /// - Select elements drop the active selection
    /// - ContentEditable nodes erase their HTML contents
    /// - Form elements recursively clear all editable descendants
    ///
    /// Returns an error when an `<input type="file">` is encountered to match
    /// browser-level restrictions.
    pub async fn clear(&self) -> Result<()> {
        let script = r#"
            function () {
                const summary = {
                    cleared: 0,
                    skippedFileInput: false,
                    unsupported: false,
                    target: (this.tagName || '').toLowerCase() || 'element'
                };

                const dispatch = (node, name) => {
                    const opts = { bubbles: true, cancelable: name === 'input', composed: true };
                    let event;
                    if (name === 'input' && typeof InputEvent === 'function') {
                        event = new InputEvent('input', opts);
                    } else {
                        event = new Event(name, opts);
                    }
                    node.dispatchEvent(event);
                };

                const clearEditable = (node) => {
                    if (!node) {
                        return false;
                    }

                    const tag = (node.tagName || '').toLowerCase();

                    if (typeof node.focus === 'function') {
                        try { node.focus({ preventScroll: true }); } catch (_) { /* ignore */ }
                    }

                    if (node.isContentEditable) {
                        if (node.innerHTML !== '') {
                            summary.cleared += 1;
                        }
                        node.innerHTML = '';
                        dispatch(node, 'input');
                        dispatch(node, 'change');
                        return true;
                    }

                    if (tag === 'input') {
                        const type = (node.type || '').toLowerCase();

                        if (type === 'file') {
                            summary.skippedFileInput = true;
                            return false;
                        }

                        if (type === 'checkbox' || type === 'radio') {
                            if (node.checked) {
                                node.checked = false;
                                summary.cleared += 1;
                                dispatch(node, 'input');
                                dispatch(node, 'change');
                            }
                            return true;
                        }

                        if (node.value !== '') {
                            summary.cleared += 1;
                        }
                        node.value = '';
                        dispatch(node, 'input');
                        dispatch(node, 'change');
                        return true;
                    }

                    if (tag === 'textarea') {
                        if (node.value !== '') {
                            summary.cleared += 1;
                        }
                        node.value = '';
                        dispatch(node, 'input');
                        dispatch(node, 'change');
                        return true;
                    }

                    if (tag === 'select') {
                        if (node.selectedIndex !== -1) {
                            summary.cleared += 1;
                        }
                        node.selectedIndex = -1;
                        dispatch(node, 'change');
                        return true;
                    }

                    if (tag === 'form') {
                        Array.from(node.elements || []).forEach((child) => {
                            clearEditable(child);
                        });
                        return true;
                    }

                    return false;
                };

                if (!clearEditable(this)) {
                    if (!this.isContentEditable) {
                        summary.unsupported = true;
                    }
                }

                return summary;
            }
        "#;

        let result = self.call_js_function(script).await?;

        let skipped_file = result
            .get("skippedFileInput")
            .and_then(Value::as_bool)
            .unwrap_or(false);

        if skipped_file {
            return Err(CdpError::element(
                "clear() cannot operate on <input type='file'> elements due to browser security restrictions".to_string(),
            ));
        }

        let unsupported = result
            .get("unsupported")
            .and_then(Value::as_bool)
            .unwrap_or(false);

        if unsupported {
            let target = result
                .get("target")
                .and_then(Value::as_str)
                .unwrap_or("element");
            return Err(CdpError::element(format!(
                "clear() is not supported for <{}> elements",
                target
            )));
        }

        Ok(())
    }

    /// Types the provided text character by character while applying a random
    /// delay between each keystroke.
    ///
    /// The element is first focused and then receives keyDown/keyUp events for
    /// every character. The delay for each character is randomly chosen within
    /// `[min_delay_ms, max_delay_ms]`.
    ///
    /// # Parameters
    /// * `text` - Text to type into the element.
    /// * `min_delay_ms` - Minimum delay (milliseconds) between characters.
    /// * `max_delay_ms` - Maximum delay (milliseconds) between characters.
    ///
    /// # Examples
    /// ```no_run
    /// # use cdp_core::Page;
    /// # use std::sync::Arc;
    /// # async fn example(page: Arc<Page>) -> anyhow::Result<()> {
    /// if let Some(input) = page.query_selector("input[type='text']").await? {
    ///     input.type_text_with_delay("Hello, World!", 50, 150).await?;
    /// }
    /// # Ok(())
    /// # }
    /// ```
    pub async fn type_text_with_delay(
        &self,
        text: &str,
        min_delay_ms: u64,
        max_delay_ms: u64,
    ) -> Result<()> {
        // Focus the element before typing.
        self.click().await?;

        // Delegate to the page helper that performs the actual typing.
        self.page
            .type_text_with_delay(text, min_delay_ms, max_delay_ms)
            .await
    }

    /// Gets the value of an attribute on the element, if present.
    ///
    /// # Parameters
    /// * `attribute_name` - The attribute to read (for example `id`, `class`, `data-value`).
    ///
    /// # Returns
    /// * `Some(String)` when the attribute is defined.
    /// * `None` when the attribute is missing or resolves to `null`.
    ///
    /// # Examples
    /// ```no_run
    /// # use cdp_core::Page;
    /// # use std::sync::Arc;
    /// # async fn example(page: Arc<Page>) -> anyhow::Result<()> {
    /// if let Some(element) = page.query_selector("div.example").await? {
    ///     if let Some(id) = element.get_attribute("id").await? {
    ///         println!("Element ID: {}", id);
    ///     }
    ///     if let Some(data_value) = element.get_attribute("data-value").await? {
    ///         println!("Data value: {}", data_value);
    ///     }
    /// }
    /// # Ok(())
    /// # }
    /// ```
    pub async fn get_attribute(&self, attribute_name: &str) -> Result<Option<String>> {
        // Step 1: resolve the backend node into an object reference.
        let resolve_params = dom::ResolveNode {
            backend_node_id: Some(self.backend_node_id),
            node_id: None,
            object_group: Some("element-helper".to_string()),
            execution_context_id: None,
        };

        let resolve_result: dom::ResolveNodeReturnObject =
            self.page.session.send_command(resolve_params, None).await?;

        let object_id = resolve_result
            .object
            .object_id
            .ok_or_else(|| CdpError::element("Object ID is unavailable for element".to_string()))?;

        // Step 2: call into the runtime to retrieve the attribute value.
        let function_declaration = format!(
            "function() {{ return this.getAttribute('{}'); }}",
            attribute_name.replace("'", "\\'")
        );

        let params = runtime::CallFunctionOn {
            function_declaration,
            object_id: Some(object_id),
            arguments: None,
            silent: None,
            return_by_value: Some(true),
            generate_preview: None,
            user_gesture: None,
            await_promise: None,
            execution_context_id: None,
            object_group: None,
            throw_on_side_effect: None,
            unique_context_id: None,
            serialization_options: None,
        };

        let result: runtime::CallFunctionOnReturnObject =
            self.page.session.send_command(params, None).await?;

        // Step 3: surface any runtime exceptions.
        if let Some(details) = result.exception_details.as_ref() {
            return Err(CdpError::element(format!(
                "Failed to get attribute '{}': {:?}",
                attribute_name, details
            )));
        }

        // Step 4: convert runtime values into an idiomatic Option<String>.
        match result.result.value {
            Some(Value::String(s)) => Ok(Some(s)),
            Some(Value::Null) => Ok(None),
            None => Ok(None),
            Some(v) => Ok(Some(v.to_string())),
        }
    }

    /// Gets the element's outer HTML (including its own tag).
    ///
    /// # Returns
    /// The full HTML string representing the element and its descendants.
    ///
    /// # Examples
    /// ```no_run
    /// # use cdp_core::Page;
    /// # use std::sync::Arc;
    /// # async fn example(page: Arc<Page>) -> anyhow::Result<()> {
    /// if let Some(element) = page.query_selector("div.example").await? {
    ///     let html = element.get_html().await?;
    ///     println!("Element HTML: {}", html);
    /// }
    /// # Ok(())
    /// # }
    /// ```
    pub async fn get_html(&self) -> Result<String> {
        // Step 1: resolve the backend node into an object reference.
        let resolve_params = dom::ResolveNode {
            backend_node_id: Some(self.backend_node_id),
            node_id: None,
            object_group: Some("element-helper".to_string()),
            execution_context_id: None,
        };

        let resolve_result: dom::ResolveNodeReturnObject =
            self.page.session.send_command(resolve_params, None).await?;

        let object_id = resolve_result
            .object
            .object_id
            .ok_or_else(|| CdpError::element("Object ID is unavailable for element".to_string()))?;

        // Step 2: invoke a runtime helper to read outerHTML.
        let params = runtime::CallFunctionOn {
            function_declaration: "function() { return this.outerHTML; }".to_string(),
            object_id: Some(object_id),
            arguments: None,
            silent: None,
            return_by_value: Some(true),
            generate_preview: None,
            user_gesture: None,
            await_promise: None,
            execution_context_id: None,
            object_group: None,
            throw_on_side_effect: None,
            unique_context_id: None,
            serialization_options: None,
        };

        let result: runtime::CallFunctionOnReturnObject =
            self.page.session.send_command(params, None).await?;

        // Step 3: surface any runtime exceptions.
        if let Some(details) = result.exception_details.as_ref() {
            return Err(CdpError::element(format!(
                "Failed to get HTML: {:?}",
                details
            )));
        }

        // Step 4: normalize the returned value into a String.
        match result.result.value {
            Some(Value::String(s)) => Ok(s),
            Some(Value::Null) => Ok("".to_string()),
            Some(v) => Ok(v.to_string()),
            None => Ok("".to_string()),
        }
    }

    /// Takes a screenshot of the element.
    ///
    /// # Parameters
    /// * `save_path` - Optional file path (including file name). When `None`, a
    ///   timestamped file such as `element_screenshot_<ts>.png` is created in
    ///   the current working directory.
    ///
    /// # Returns
    /// The path where the screenshot was saved.
    ///
    /// # Notes
    /// - Uses the default border box, which generally works well for rounded elements.
    /// - Automatically adapts to the device pixel ratio (DPR) for crisp images.
    /// - Use [`screenshot_with_options`](Self::screenshot_with_options) for
    ///   additional control.
    ///
    /// # Examples
    /// ```no_run
    /// # use cdp_core::Page;
    /// # use std::sync::Arc;
    /// # async fn example(page: Arc<Page>) -> anyhow::Result<()> {
    /// if let Some(element) = page.query_selector("div.example").await? {
    ///     // Save to the current directory (with DPR auto-adjust)
    ///     let path = element.screenshot(None).await?;
    ///     println!("Element screenshot saved to: {}", path);
    ///
    ///     // Save to a custom location
    ///     let path = element.screenshot(Some("screenshots/element.png".into())).await?;
    ///     println!("Element screenshot saved to: {}", path);
    /// }
    /// # Ok(())
    /// # }
    /// ```
    pub async fn screenshot(&self, save_path: Option<PathBuf>) -> Result<String> {
        self.screenshot_with_options(save_path, ScreenshotBoxType::default(), true)
            .await
    }

    /// Takes a screenshot of the element with a custom box type.
    ///
    /// # Parameters
    /// * `save_path` - Optional file path (including file name).
    /// * `box_type` - Bounding box strategy that determines the capture region.
    /// * `auto_resolve_dpr` - Whether to adapt the screenshot to the device
    ///   pixel ratio (`true` is recommended for high-DPI displays).
    ///
    /// # Bounding Box Types
    ///
    /// - `Content`: Captures only the content area, excluding padding.
    ///   - Best for: screenshots that should omit interior padding.
    ///   - Less ideal when padding needs to remain visible.
    ///
    /// - `Padding`: Captures content and padding, excluding the border.
    ///   - Best for: including interior spacing while omitting borders.
    ///   - Less ideal when borders or rounded corners must be preserved.
    ///
    /// - `Border` (default): Captures content, padding, and border.
    ///   - Ideal for rounded corners (`border-radius`).
    ///   - Suitable for elements where the border defines the visual shape.
    ///   - Works well for most everyday use cases.
    ///
    /// - `Margin`: Captures content, padding, border, and margin.
    ///   - Best for: including surrounding whitespace in the capture.
    ///   - Less ideal when margins introduce too much empty space.
    ///
    /// # Returns
    /// The path where the screenshot was saved.
    ///
    /// # Examples
    /// ```no_run
    /// # use cdp_core::Page;
    /// # use cdp_core::page::element::ScreenshotBoxType;
    /// # use std::sync::Arc;
    /// # async fn example(page: Arc<Page>) -> anyhow::Result<()> {
    /// if let Some(element) = page.query_selector("button.rounded").await? {
    ///     // Rounded button with border box + DPR auto adjustment (recommended)
    ///     let path = element.screenshot_with_options(
    ///         Some("button.png".into()),
    ///         ScreenshotBoxType::Border,
    ///         true  // enable DPR auto adjustment
    ///     ).await?;
    ///
    ///     // Content-only capture with DPR adaptation disabled
    ///     let path = element.screenshot_with_options(
    ///         Some("content.png".into()),
    ///         ScreenshotBoxType::Content,
    ///         false  // fixed scale = 1.0
    ///     ).await?;
    ///
    ///     // Include the margin while keeping DPR auto adjustment
    ///     let path = element.screenshot_with_options(
    ///         Some("with-margin.png".into()),
    ///         ScreenshotBoxType::Margin,
    ///         true
    ///     ).await?;
    /// }
    /// # Ok(())
    /// # }
    /// ```
    pub async fn screenshot_with_options(
        &self,
        save_path: Option<PathBuf>,
        box_type: ScreenshotBoxType,
        auto_resolve_dpr: bool,
    ) -> Result<String> {
        use base64::Engine;
        use cdp_protocol::page as page_cdp;

        // Capture the target region using Page.captureScreenshot.
        // Determine the device pixel ratio so the output looks sharp on high-DPI screens.
        let device_scale = if auto_resolve_dpr {
            // Query DPR via the page evaluate helper.
            use cdp_protocol::runtime::{Evaluate, EvaluateReturnObject};

            let eval_result = self
                .page
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

            // Clamp the DPR to a sensible range so wildly high values do not explode file sizes.
            dpr.clamp(0.5, 3.0)
        } else {
            1.0
        };

        // Resolve the backend node to an object ID (CDP allows either nodeId or backendNodeId, never both).
        let resolve_params = dom::ResolveNode {
            backend_node_id: if self.node_id.is_none() {
                Some(self.backend_node_id)
            } else {
                None
            },
            node_id: self.node_id,
            object_group: Some("screenshot-helper".to_string()),
            execution_context_id: None,
        };

        let resolve_result: dom::ResolveNodeReturnObject =
            self.page.session.send_command(resolve_params, None).await?;

        let object_id = resolve_result
            .object
            .object_id
            .ok_or_else(|| CdpError::element("Object ID is unavailable for element".to_string()))?;

        // Scroll the element into view using the object ID for stability.
        let scroll_params = dom::ScrollIntoViewIfNeeded {
            node_id: None,
            backend_node_id: None,
            object_id: Some(object_id.clone()),
            rect: None,
        };
        self.page
            .session
            .send_command::<_, dom::ScrollIntoViewIfNeededReturnObject>(scroll_params, None)
            .await?;

        // Give the renderer a moment to settle after scrolling.
        tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;

        // Compute precise bounds via getBoundingClientRect(). This is more
        // accurate than GetBoxModel because it comes directly from the render
        // engine. Remember the values are viewport-relative, so scroll offsets
        // must be reintroduced to get absolute coordinates.
        let js_function = match box_type {
            ScreenshotBoxType::Content => {
                // Content box excludes padding.
                r#"function() {
                    const rect = this.getBoundingClientRect();
                    const style = window.getComputedStyle(this);
                    const paddingLeft = parseFloat(style.paddingLeft) || 0;
                    const paddingTop = parseFloat(style.paddingTop) || 0;
                    const paddingRight = parseFloat(style.paddingRight) || 0;
                    const paddingBottom = parseFloat(style.paddingBottom) || 0;
                    return {
                        x: rect.left + window.scrollX + paddingLeft,
                        y: rect.top + window.scrollY + paddingTop,
                        width: rect.width - paddingLeft - paddingRight,
                        height: rect.height - paddingTop - paddingBottom
                    };
                }"#
            }
            ScreenshotBoxType::Padding => {
                // Padding box captures content and padding (no border).
                r#"function() {
                    const rect = this.getBoundingClientRect();
                    const style = window.getComputedStyle(this);
                    const borderLeft = parseFloat(style.borderLeftWidth) || 0;
                    const borderTop = parseFloat(style.borderTopWidth) || 0;
                    const borderRight = parseFloat(style.borderRightWidth) || 0;
                    const borderBottom = parseFloat(style.borderBottomWidth) || 0;
                    return {
                        x: rect.left + window.scrollX + borderLeft,
                        y: rect.top + window.scrollY + borderTop,
                        width: rect.width - borderLeft - borderRight,
                        height: rect.height - borderTop - borderBottom
                    };
                }"#
            }
            ScreenshotBoxType::Border => {
                // Border box captures content, padding, and border (recommended).
                r#"function() {
                    const rect = this.getBoundingClientRect();
                    return {
                        x: rect.left + window.scrollX,
                        y: rect.top + window.scrollY,
                        width: rect.width,
                        height: rect.height
                    };
                }"#
            }
            ScreenshotBoxType::Margin => {
                // Margin box captures everything including the margin.
                r#"function() {
                    const rect = this.getBoundingClientRect();
                    const style = window.getComputedStyle(this);
                    const marginLeft = parseFloat(style.marginLeft) || 0;
                    const marginTop = parseFloat(style.marginTop) || 0;
                    const marginRight = parseFloat(style.marginRight) || 0;
                    const marginBottom = parseFloat(style.marginBottom) || 0;
                    return {
                        x: rect.left + window.scrollX - marginLeft,
                        y: rect.top + window.scrollY - marginTop,
                        width: rect.width + marginLeft + marginRight,
                        height: rect.height + marginTop + marginBottom
                    };
                }"#
            }
        };

        let call_params = runtime::CallFunctionOn {
            function_declaration: js_function.to_string(),
            object_id: Some(object_id),
            arguments: None,
            silent: None,
            return_by_value: Some(true),
            generate_preview: None,
            user_gesture: None,
            await_promise: None,
            execution_context_id: None,
            object_group: None,
            throw_on_side_effect: None,
            unique_context_id: None,
            serialization_options: None,
        };

        let call_result: runtime::CallFunctionOnReturnObject =
            self.page.session.send_command(call_params, None).await?;

        if let Some(details) = call_result.exception_details.as_ref() {
            return Err(CdpError::element(format!(
                "Failed to get element bounds: {:?}",
                details
            )));
        }

        // Parse the bounding rectangle returned from the JavaScript helper.
        let bounds = call_result.result.value.ok_or_else(|| {
            CdpError::element("No bounds returned from JavaScript for element".to_string())
        })?;

        let x = bounds["x"].as_f64().ok_or_else(|| {
            CdpError::element("Invalid x coordinate returned for element".to_string())
        })?;
        let y = bounds["y"].as_f64().ok_or_else(|| {
            CdpError::element("Invalid y coordinate returned for element".to_string())
        })?;
        let width = bounds["width"]
            .as_f64()
            .ok_or_else(|| CdpError::element("Invalid width returned for element".to_string()))?;
        let height = bounds["height"]
            .as_f64()
            .ok_or_else(|| CdpError::element("Invalid height returned for element".to_string()))?;

        // Align to device pixels so the screenshot edges stay crisp on high-DPI screens.
        // Strategy:
        // - Floor the top-left corner to avoid accidentally clipping.
        // - Ceil the bottom-right corner to capture the full element.
        // - Recompute width/height from those adjusted edges.
        let aligned_x = (x * device_scale).floor() / device_scale;
        let aligned_y = (y * device_scale).floor() / device_scale;

        // Compute the bottom-right corner using a ceiling operation.
        let right_edge = ((x + width) * device_scale).ceil() / device_scale;
        let bottom_edge = ((y + height) * device_scale).ceil() / device_scale;

        // Determine the final width and height from the aligned edges.
        let aligned_width = right_edge - aligned_x;
        let aligned_height = bottom_edge - aligned_y;

        // Ensure at least one logical pixel remains after alignment.
        let final_width = aligned_width.max(1.0 / device_scale);
        let final_height = aligned_height.max(1.0 / device_scale);

        let clip = Some(page_cdp::Viewport {
            x: aligned_x,
            y: aligned_y,
            width: final_width,
            height: final_height,
            scale: device_scale,
        });

        // Execute the screenshot command with the computed clip.
        let screenshot_params = page_cdp::CaptureScreenshot {
            format: Some(page_cdp::CaptureScreenshotFormatOption::Png),
            quality: None,
            clip,
            from_surface: Some(true),
            capture_beyond_viewport: Some(true),
            optimize_for_speed: None,
        };

        let result: page_cdp::CaptureScreenshotReturnObject = self
            .page
            .session
            .send_command(screenshot_params, None)
            .await?;

        // Derive the output path.
        let out_path_buf: std::path::PathBuf = match save_path {
            Some(pv) => {
                if pv.parent().is_none() || pv.parent().unwrap().as_os_str().is_empty() {
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
                dir.join(format!("element-screenshot-{}.png", nanos))
            }
        };

        // Decode the base64 payload and persist it to disk.
        let bytes = base64::engine::general_purpose::STANDARD
            .decode(&result.data)
            .map_err(|err| CdpError::element(format!("Failed to decode screenshot data: {err}")))?;
        let mut f = File::create(&out_path_buf).await?;
        f.write_all(&bytes).await?;
        f.flush().await?;

        let out_path = out_path_buf.to_string_lossy();
        Ok(out_path.into_owned())
    }

    // ========= Helper utilities =========

    /// Executes a JavaScript function against the element.
    async fn call_js_function(&self, function_declaration: &str) -> Result<serde_json::Value> {
        use cdp_protocol::runtime::{CallFunctionOn, CallFunctionOnReturnObject};

        // Resolve an object_id for the element.
        let obj_id = if let Some(node_id) = self.node_id {
            // Use DOM.resolveNode to obtain an object_id when node_id is present.
            use cdp_protocol::dom::{ResolveNode, ResolveNodeReturnObject};

            let resolve_result: ResolveNodeReturnObject = self
                .page
                .session
                .send_command(
                    ResolveNode {
                        node_id: Some(node_id),
                        backend_node_id: None,
                        object_group: Some("element-utils".to_string()),
                        execution_context_id: None,
                    },
                    None,
                )
                .await?;

            resolve_result.object.object_id.ok_or_else(|| {
                CdpError::element("No object ID available for element".to_string())
            })?
        } else {
            return Err(CdpError::element(
                "No node ID available for element".to_string(),
            ));
        };

        // Invoke the function within the runtime.
        let params = CallFunctionOn {
            function_declaration: function_declaration.to_string(),
            object_id: Some(obj_id.clone()),
            arguments: Some(vec![]),
            silent: Some(true),
            return_by_value: Some(true),
            generate_preview: None,
            user_gesture: None,
            await_promise: None,
            execution_context_id: None,
            object_group: Some("element-utils".to_string()),
            throw_on_side_effect: None,
            unique_context_id: None,
            serialization_options: None,
        };

        let result: CallFunctionOnReturnObject =
            self.page.session.send_command(params, None).await?;

        if let Some(value) = result.result.value {
            Ok(value)
        } else {
            Ok(serde_json::Value::Null)
        }
    }

    // ========= Visibility and state checks =========

    /// Determines whether the element is visible.
    ///
    /// The element is considered visible when:
    /// - It is present in the DOM tree.
    /// - It does not use `display: none` or `visibility: hidden` (and `opacity` is not `0`).
    /// - Its rendered bounding box is non-zero.
    ///
    /// # Returns
    /// `true` if the element is visible, otherwise `false`.
    ///
    /// # Examples
    /// ```no_run
    /// # use cdp_core::Page;
    /// # use std::sync::Arc;
    /// # async fn example(page: Arc<Page>) -> anyhow::Result<()> {
    /// let element = page.query_selector("#my-element").await?.unwrap();
    /// if element.is_visible().await? {
    ///     println!("Element is visible");
    /// }
    /// # Ok(())
    /// # }
    /// ```
    pub async fn is_visible(&self) -> Result<bool> {
        // Evaluate a JavaScript helper to assess visibility heuristics.
        let script = r#"
        function() {
            if (!this) return false;
            
            // Use offsetParent as the quick visibility hint; body/html are special-cased.
            if (this === document.body || this === document.documentElement) {
                return true;
            }
            if (!this.offsetParent && this.tagName !== 'BODY') {
                return false;
            }
            
            // Inspect computed styles for common hidden states.
            const style = window.getComputedStyle(this);
            if (style.display === 'none' || style.visibility === 'hidden' || style.opacity === '0') {
                return false;
            }
            
            // Reject zero-sized rectangles.
            const rect = this.getBoundingClientRect();
            if (rect.width === 0 || rect.height === 0) {
                return false;
            }
            
            return true;
        }
        "#;

        let result = self.call_js_function(script).await?;
        Ok(result.as_bool().unwrap_or(false))
    }

    /// Checks whether the element is enabled (not `disabled`).
    ///
    /// # Returns
    /// `true` when the element is enabled, otherwise `false`.
    pub async fn is_enabled(&self) -> Result<bool> {
        let script = r#"
        function() {
            if (!this) return false;
            return !this.disabled;
        }
        "#;

        let result = self.call_js_function(script).await?;
        Ok(result.as_bool().unwrap_or(true))
    }

    /// Determines whether the element can be clicked.
    ///
    /// The element is considered clickable when it is visible, enabled, and
    /// not occluded by another element at its center point.
    ///
    /// # Returns
    /// `true` if the element is clickable, otherwise `false`.
    pub async fn is_clickable(&self) -> Result<bool> {
        // Check visibility and enabled state first.
        if !self.is_visible().await? {
            return Ok(false);
        }

        if !self.is_enabled().await? {
            return Ok(false);
        }

        // Verify that no other element occludes the center point.
        let script = r#"
        function() {
            if (!this) return false;
            
            const rect = this.getBoundingClientRect();
            const centerX = rect.left + rect.width / 2;
            const centerY = rect.top + rect.height / 2;
            
            const topElement = document.elementFromPoint(centerX, centerY);
            if (!topElement) return false;
            
            // Ensure the target point belongs to the element or one of its descendants.
            return this.contains(topElement);
        }
        "#;

        let result = self.call_js_function(script).await?;
        Ok(result.as_bool().unwrap_or(false))
    }

    // ========= Waiting helpers =========

    /// Waits for the element to become visible.
    ///
    /// # Parameters
    /// * `timeout_ms` - Timeout in milliseconds (defaults to `30000`).
    ///
    /// # Examples
    /// ```no_run
    /// # use cdp_core::Page;
    /// # use std::sync::Arc;
    /// # async fn example(page: Arc<Page>) -> anyhow::Result<()> {
    /// let element = page.query_selector("#dynamic-content").await?.unwrap();
    /// element.wait_for_visible(Some(5000)).await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn wait_for_visible(&self, timeout_ms: Option<u64>) -> Result<()> {
        let timeout = timeout_ms.unwrap_or(30000);
        let start = std::time::Instant::now();
        let poll_interval = std::time::Duration::from_millis(100);

        loop {
            if start.elapsed().as_millis() > timeout as u128 {
                return Err(CdpError::element(format!(
                    "Timeout waiting for element to be visible ({}ms)",
                    timeout
                )));
            }

            if self.is_visible().await? {
                return Ok(());
            }

            tokio::time::sleep(poll_interval).await;
        }
    }

    /// Waits for the element to become hidden.
    ///
    /// # Parameters
    /// * `timeout_ms` - Timeout in milliseconds (defaults to `30000`).
    pub async fn wait_for_hidden(&self, timeout_ms: Option<u64>) -> Result<()> {
        let timeout = timeout_ms.unwrap_or(30000);
        let start = std::time::Instant::now();
        let poll_interval = std::time::Duration::from_millis(100);

        loop {
            if start.elapsed().as_millis() > timeout as u128 {
                return Err(CdpError::element(format!(
                    "Timeout waiting for element to be hidden ({}ms)",
                    timeout
                )));
            }

            if !self.is_visible().await? {
                return Ok(());
            }

            tokio::time::sleep(poll_interval).await;
        }
    }

    /// Waits for the element to become clickable.
    ///
    /// # Parameters
    /// * `timeout_ms` - Timeout in milliseconds (defaults to `30000`).
    ///
    /// # Examples
    /// ```no_run
    /// # use cdp_core::Page;
    /// # use std::sync::Arc;
    /// # async fn example(page: Arc<Page>) -> anyhow::Result<()> {
    /// let button = page.query_selector("#submit-btn").await?.unwrap();
    /// button.wait_for_clickable(Some(5000)).await?;
    /// button.click().await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn wait_for_clickable(&self, timeout_ms: Option<u64>) -> Result<()> {
        let timeout = timeout_ms.unwrap_or(30000);
        let start = std::time::Instant::now();
        let poll_interval = std::time::Duration::from_millis(100);

        loop {
            if start.elapsed().as_millis() > timeout as u128 {
                return Err(CdpError::element(format!(
                    "Timeout waiting for element to be clickable ({}ms)",
                    timeout
                )));
            }

            if self.is_clickable().await? {
                return Ok(());
            }

            tokio::time::sleep(poll_interval).await;
        }
    }

    /// Waits for the element to become enabled (not `disabled`).
    ///
    /// # Parameters
    /// * `timeout_ms` - Timeout in milliseconds (defaults to `30000`).
    pub async fn wait_for_enabled(&self, timeout_ms: Option<u64>) -> Result<()> {
        let timeout = timeout_ms.unwrap_or(30000);
        let start = std::time::Instant::now();
        let poll_interval = std::time::Duration::from_millis(100);

        loop {
            if start.elapsed().as_millis() > timeout as u128 {
                return Err(CdpError::element(format!(
                    "Timeout waiting for element to be enabled ({}ms)",
                    timeout
                )));
            }

            if self.is_enabled().await? {
                return Ok(());
            }

            tokio::time::sleep(poll_interval).await;
        }
    }

    // ========= Shadow DOM helpers =========

    /// Retrieves the element's shadow root (supports both open and closed modes).
    ///
    /// # Returns
    /// - `Ok(Some(ShadowRoot))` when the element exposes a shadow root.
    /// - `Ok(None)` when the element has no shadow root.
    /// - `Err(_)` if the operation failed.
    ///
    /// # Examples
    /// ```no_run
    /// # use cdp_core::Page;
    /// # use std::sync::Arc;
    /// # async fn example(page: Arc<Page>) -> anyhow::Result<()> {
    /// let host = page.query_selector("#shadow-host").await?.unwrap();
    ///
    /// if let Some(shadow_root) = host.shadow_root().await? {
    ///     // Query inside the shadow DOM
    ///     let inner = shadow_root.query_selector(".inner-element").await?;
    /// }
    /// # Ok(())
    /// # }
    /// ```
    pub async fn shadow_root(&self) -> Result<Option<ShadowRoot>> {
        use cdp_protocol::dom::{DescribeNode, DescribeNodeReturnObject};

        // Use DescribeNode to retrieve node details, including the shadow root.
        let describe_params = DescribeNode {
            node_id: self.node_id,
            backend_node_id: if self.node_id.is_none() {
                Some(self.backend_node_id)
            } else {
                None
            },
            object_id: None,
            depth: Some(1), // Include one level of children to expose the shadow root.
            pierce: Some(true), // Allow traversal through shadow boundaries.
        };

        let describe_result: DescribeNodeReturnObject = self
            .page
            .session
            .send_command(describe_params, None)
            .await?;

        // Determine whether a shadow root is available.
        if let Some(shadow_roots) = describe_result.node.shadow_roots
            && !shadow_roots.is_empty()
        {
            let shadow_root_node = &shadow_roots[0];
            return Ok(Some(ShadowRoot {
                node_id: shadow_root_node.node_id,
                backend_node_id: shadow_root_node.backend_node_id,
                shadow_root_type: shadow_root_node.shadow_root_type.clone(),
                page: Arc::clone(&self.page),
            }));
        }

        Ok(None)
    }

    /// Queries for a single element inside the shadow DOM (supports open and closed modes).
    ///
    /// This is a convenience wrapper around `element.shadow_root().await?.query_selector(selector)`.
    ///
    /// # Parameters
    /// * `selector` - CSS selector.
    ///
    /// # Returns
    /// - `Ok(Some(ElementHandle))` when a matching element is found.
    /// - `Ok(None)` when the element has no shadow root or no match is located.
    /// - `Err(_)` if querying fails.
    ///
    /// # Examples
    /// ```no_run
    /// # use cdp_core::Page;
    /// # use std::sync::Arc;
    /// # async fn example(page: Arc<Page>) -> anyhow::Result<()> {
    /// let host = page.query_selector("#shadow-host").await?.unwrap();
    ///
    /// if let Some(inner) = host.query_selector_shadow(".inner-element").await? {
    ///     let text = inner.text_content().await?;
    ///     println!("Shadow DOM content: {}", text);
    /// }
    /// # Ok(())
    /// # }
    /// ```
    pub async fn query_selector_shadow(&self, selector: &str) -> Result<Option<ElementHandle>> {
        if let Some(shadow_root) = self.shadow_root().await? {
            shadow_root.query_selector(selector).await
        } else {
            Ok(None)
        }
    }

    /// Queries for all matching elements within the shadow DOM (supports open and closed modes).
    ///
    /// # Parameters
    /// * `selector` - CSS selector used for the lookup.
    ///
    /// # Returns
    /// A vector of matching elements, or an empty vector when no shadow root is present.
    ///
    /// # Examples
    /// ```no_run
    /// # use cdp_core::Page;
    /// # use std::sync::Arc;
    /// # async fn example(page: Arc<Page>) -> anyhow::Result<()> {
    /// let host = page.query_selector("#shadow-host").await?.unwrap();
    ///
    /// let items = host.query_selector_all_shadow(".list-item").await?;
    /// println!("Found {} items in Shadow DOM", items.len());
    /// # Ok(())
    /// # }
    /// ```
    pub async fn query_selector_all_shadow(&self, selector: &str) -> Result<Vec<ElementHandle>> {
        if let Some(shadow_root) = self.shadow_root().await? {
            shadow_root.query_selector_all(selector).await
        } else {
            Ok(Vec::new())
        }
    }
}

/// Handle for a shadow root.
///
/// Represents the root of a shadow DOM tree and allows querying within it.
/// Supports both open and closed shadow roots.
#[derive(Clone)]
pub struct ShadowRoot {
    pub(crate) node_id: u32,
    pub(crate) backend_node_id: u32,
    pub(crate) shadow_root_type: Option<dom::ShadowRootType>,
    pub(crate) page: Arc<Page>,
}

impl ShadowRoot {
    /// Returns the shadow root type (open or closed) if known.
    pub fn shadow_root_type(&self) -> Option<&dom::ShadowRootType> {
        self.shadow_root_type.as_ref()
    }

    /// Checks whether the selector is an XPath expression.
    fn is_xpath(selector: &str) -> bool {
        selector.starts_with("xpath:") || selector.starts_with("/") || selector.starts_with("(")
    }

    /// Strips the `xpath:` prefix when present.
    fn normalize_xpath(selector: &str) -> &str {
        selector.strip_prefix("xpath:").unwrap_or(selector)
    }

    /// Queries the shadow DOM for the first matching element.
    ///
    /// # Parameters
    /// * `selector` - CSS selector or XPath expression.
    ///   - CSS selectors such as `"div.class"` or `"#id"` are recommended.
    ///   - XPath expressions start with `"xpath:"` or `/`, for example `"//div[@class='test']"`.
    ///
    /// # XPath Caveats
    /// XPath support within the shadow DOM is limited. Because CDP's
    /// `DOM.performSearch` operates globally, results may include nodes outside
    /// the shadow root. Prefer CSS selectors when possible.
    ///
    /// # Returns
    /// The first matching element handle, or `None` when nothing matches.
    ///
    /// # Examples
    /// ```no_run
    /// # use cdp_core::Page;
    /// # use std::sync::Arc;
    /// # async fn example(page: Arc<Page>) -> anyhow::Result<()> {
    /// let host = page.query_selector("#shadow-host").await?.unwrap();
    /// let shadow_root = host.shadow_root().await?.unwrap();
    ///
    /// // Preferred CSS selector path
    /// if let Some(element) = shadow_root.query_selector(".target").await? {
    ///     element.click().await?;
    /// }
    ///
    /// // XPath fallback (limited support)
    /// if let Some(element) = shadow_root.query_selector("//button[text()='Click']").await? {
    ///     element.click().await?;
    /// }
    /// # Ok(())
    /// # }
    /// ```
    pub async fn query_selector(&self, selector: &str) -> Result<Option<ElementHandle>> {
        // For XPath selectors emit a warning; DOM.performSearch is global and unreliable within the shadow tree.
        if Self::is_xpath(selector) {
            eprintln!("Warning: XPath support inside shadow DOM is limited; prefer CSS selectors");
            eprintln!(
                "If XPath is required, use query_selector_shadow on the host element instead"
            );
            // Continue anyway so callers can attempt the lookup.
        }

        use cdp_protocol::dom::{
            DescribeNode, DescribeNodeReturnObject, QuerySelector, QuerySelectorReturnObject,
        };

        let query_result = self
            .page
            .session
            .send_command::<_, QuerySelectorReturnObject>(
                QuerySelector {
                    node_id: self.node_id,
                    selector: selector.to_string(),
                },
                None,
            )
            .await?;

        if query_result.node_id == 0 {
            return Ok(None);
        }

        let describe_result = self
            .page
            .session
            .send_command::<_, DescribeNodeReturnObject>(
                DescribeNode {
                    node_id: Some(query_result.node_id),
                    backend_node_id: None,
                    object_id: None,
                    depth: None,
                    pierce: None,
                },
                None,
            )
            .await?;

        Ok(Some(ElementHandle {
            backend_node_id: describe_result.node.backend_node_id,
            node_id: Some(query_result.node_id),
            page: Arc::clone(&self.page),
        }))
    }

    /// Queries the shadow DOM for all matching elements.
    ///
    /// # Parameters
    /// * `selector` - CSS selector or XPath expression.
    ///   - CSS selectors such as `"div.class"` or `"#id"` work reliably.
    ///   - XPath expressions start with `"xpath:"` or `/`, for example `"//div[@class='test']"`.
    ///
    /// # XPath Caveats
    /// XPath support within the shadow DOM is limited. Because CDP's
    /// `DOM.performSearch` is global, XPath queries may surface nodes outside
    /// of the shadow root. Prefer CSS selectors when possible.
    ///
    /// # Returns
    /// A vector of matching element handles.
    ///
    /// # Examples
    /// ```no_run
    /// # use cdp_core::Page;
    /// # use std::sync::Arc;
    /// # async fn example(page: Arc<Page>) -> anyhow::Result<()> {
    /// let host = page.query_selector("#shadow-host").await?.unwrap();
    /// let shadow_root = host.shadow_root().await?.unwrap();
    ///
    /// // Preferred CSS selector path
    /// let items = shadow_root.query_selector_all(".item").await?;
    /// for (i, item) in items.iter().enumerate() {
    ///     println!("Item {}: {}", i + 1, item.text_content().await?);
    /// }
    ///
    /// // XPath fallback (limited support)
    /// let buttons = shadow_root.query_selector_all("//button").await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn query_selector_all(&self, selector: &str) -> Result<Vec<ElementHandle>> {
        // Emit a warning for XPath selectors.
        if Self::is_xpath(selector) {
            eprintln!("Warning: XPath support inside shadow DOM is limited; prefer CSS selectors");
        }

        use cdp_protocol::dom::{
            DescribeNode, DescribeNodeReturnObject, QuerySelectorAll, QuerySelectorAllReturnObject,
        };

        let query_result = self
            .page
            .session
            .send_command::<_, QuerySelectorAllReturnObject>(
                QuerySelectorAll {
                    node_id: self.node_id,
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

            elements.push(ElementHandle {
                backend_node_id: describe_result.node.backend_node_id,
                node_id: Some(node_id),
                page: Arc::clone(&self.page),
            });
        }

        Ok(elements)
    }

    /// Retrieves the HTML markup contained within the shadow root.
    ///
    /// # Returns
    /// The HTML string for the shadow root.
    ///
    /// # Examples
    /// ```no_run
    /// # use cdp_core::Page;
    /// # use std::sync::Arc;
    /// # async fn example(page: Arc<Page>) -> anyhow::Result<()> {
    /// let host = page.query_selector("#shadow-host").await?.unwrap();
    /// let shadow_root = host.shadow_root().await?.unwrap();
    ///
    /// let html = shadow_root.get_html().await?;
    /// println!("Shadow DOM HTML: {}", html);
    /// # Ok(())
    /// # }
    /// ```
    pub async fn get_html(&self) -> Result<String> {
        use cdp_protocol::dom::{GetOuterHTML, GetOuterHTMLReturnObject};

        let result: GetOuterHTMLReturnObject = self
            .page
            .session
            .send_command(
                GetOuterHTML {
                    node_id: Some(self.node_id),
                    backend_node_id: None,
                    object_id: None,
                    include_shadow_dom: None,
                },
                None,
            )
            .await?;

        Ok(result.outer_html)
    }
}
