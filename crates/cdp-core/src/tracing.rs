use std::{path::PathBuf, sync::Arc};

use base64::Engine as _;
use base64::engine::general_purpose::STANDARD as BASE64_STANDARD;
use cdp_protocol::{io, tracing as tracing_cdp, tracing::StartTransferModeOption};
use tokio::{fs, sync::Mutex};

use crate::{
    error::{CdpError, Result},
    page::Page,
};

const DEFAULT_TRACE_CATEGORIES: &[&str] = &[
    "-*",
    "devtools.timeline",
    "v8.execute",
    "disabled-by-default-devtools.timeline",
    "disabled-by-default-devtools.timeline.frame",
    "disabled-by-default-devtools.timeline.stack",
    "disabled-by-default-v8.cpu_profiler",
];

/// Tracing lifecycle status.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub(crate) enum TracingStatus {
    #[default]
    Idle,
    Active,
    Stopping,
}

/// Page-scoped cache that records tracing state transitions.
#[derive(Debug, Default)]
pub(crate) struct TracingSessionState {
    pub status: TracingStatus,
}

/// Configuration used when starting a tracing session.
#[derive(Debug, Clone)]
pub struct TracingStartOptions {
    /// Categories to trace; defaults to a curated set that captures page activity.
    pub categories: Option<Vec<String>>,
    /// Whether to capture screenshots in addition to trace events.
    pub screenshots: bool,
    /// Trace record mode (see `TraceConfig.record_mode`).
    pub record_mode: tracing_cdp::TraceConfigRecordMode,
    /// Optional buffer usage reporting interval in milliseconds.
    pub buffer_usage_reporting_interval_ms: Option<f64>,
    /// Requested stream format for trace data.
    pub stream_format: tracing_cdp::StreamFormat,
    /// Requested compression format for the streamed trace.
    pub stream_compression: tracing_cdp::StreamCompression,
    /// Optional tracing backend to use.
    pub tracing_backend: Option<tracing_cdp::TracingBackend>,
}

impl Default for TracingStartOptions {
    fn default() -> Self {
        Self {
            categories: None,
            screenshots: false,
            record_mode: tracing_cdp::TraceConfigRecordMode::RecordAsMuchAsPossible,
            buffer_usage_reporting_interval_ms: None,
            stream_format: tracing_cdp::StreamFormat::Json,
            stream_compression: tracing_cdp::StreamCompression::None,
            tracing_backend: None,
        }
    }
}

/// Result returned after stopping tracing.
#[derive(Debug, Clone)]
pub struct TracingStopResult {
    /// Raw trace data gathered during the session.
    pub data: Vec<u8>,
    /// Indicates whether Chrome reported data loss.
    pub data_loss_occurred: bool,
    /// Reported stream format.
    pub format: Option<tracing_cdp::StreamFormat>,
    /// Reported compression algorithm.
    pub compression: Option<tracing_cdp::StreamCompression>,
    /// Path to the saved file if trace data was persisted to disk.
    pub saved_path: Option<PathBuf>,
}

/// High-level controller for Chrome tracing on a single `Page`.
///
/// # Examples
/// ```no_run
/// # use cdp_core::Page;
/// # use cdp_core::TracingStartOptions;
/// # use std::sync::Arc;
/// # async fn example(page: Arc<Page>) -> anyhow::Result<()> {
/// let tracing = page.tracing();
/// tracing.start(Some(TracingStartOptions::default())).await?;
/// // ... perform actions to record ...
/// let result = tracing.stop(None).await?;
/// assert!(!result.data.is_empty());
/// # Ok(())
/// # }
/// ```
pub struct TracingController {
    page: Arc<Page>,
    state: Arc<Mutex<TracingSessionState>>,
}

impl TracingController {
    pub(crate) fn new(page: Arc<Page>, state: Arc<Mutex<TracingSessionState>>) -> Self {
        Self { page, state }
    }

    /// Returns `true` if tracing has been started and not yet fully stopped.
    pub async fn is_recording(&self) -> bool {
        !matches!(self.state.lock().await.status, TracingStatus::Idle)
    }

    /// Queries the browser for supported tracing categories.
    pub async fn categories(&self) -> Result<Vec<String>> {
        let result: tracing_cdp::GetCategoriesReturnObject = self
            .page
            .session
            .send_command(tracing_cdp::GetCategories(None), None)
            .await?;
        Ok(result.categories)
    }

    /// Starts tracing with the provided options (or defaults when `None`).
    pub async fn start(&self, options: Option<TracingStartOptions>) -> Result<()> {
        let options = options.unwrap_or_default();

        {
            let mut state = self.state.lock().await;
            match state.status {
                TracingStatus::Idle => state.status = TracingStatus::Active,
                TracingStatus::Active => {
                    return Err(CdpError::tool(
                        "Tracing is already active; call stop() before starting again",
                    ));
                }
                TracingStatus::Stopping => {
                    return Err(CdpError::tool(
                        "Tracing stop is still in progress; try again after it completes",
                    ));
                }
            }
        }

        let start_params = build_start_command(&options);
        let result = self
            .page
            .session
            .send_command::<_, tracing_cdp::StartReturnObject>(start_params, None)
            .await;

        if let Err(err) = result {
            let mut state = self.state.lock().await;
            state.status = TracingStatus::Idle;
            return Err(err);
        }

        Ok(())
    }

    /// Stops tracing and optionally persists the trace to disk.
    pub async fn stop(&self, save_path: Option<PathBuf>) -> Result<TracingStopResult> {
        {
            let mut state = self.state.lock().await;
            match state.status {
                TracingStatus::Active => state.status = TracingStatus::Stopping,
                TracingStatus::Idle => {
                    return Err(CdpError::tool(
                        "Tracing has not been started; call start() before stop()",
                    ));
                }
                TracingStatus::Stopping => {
                    return Err(CdpError::tool(
                        "Tracing stop is already in progress; do not call stop() again",
                    ));
                }
            }
        }

        let stop_result = self.finish_stop(save_path).await;

        let mut state = self.state.lock().await;
        state.status = TracingStatus::Idle;

        stop_result
    }

    async fn finish_stop(&self, save_path: Option<PathBuf>) -> Result<TracingStopResult> {
        let _: tracing_cdp::EndReturnObject = self
            .page
            .session
            .send_command(tracing_cdp::End(None), None)
            .await?;

        let event = self
            .page
            .wait_for::<tracing_cdp::events::TracingCompleteEvent>()
            .await?;
        let params = event.params;

        let stream_handle = params.stream.ok_or_else(|| {
            CdpError::tool(
                "TracingComplete event did not include a stream handle; trace data unavailable",
            )
        })?;

        let mut data = self.drain_stream(stream_handle.clone()).await?;
        self.close_stream(stream_handle).await?;

        let saved_path = if let Some(path) = save_path {
            if let Some(parent) = path.parent()
                && !parent.as_os_str().is_empty()
            {
                fs::create_dir_all(parent).await?;
            }
            fs::write(&path, &data).await?;
            Some(path)
        } else {
            None
        };

        if params.data_loss_occurred {
            tracing::warn!("TracingComplete reported data loss; trace output may be incomplete");
        }

        Ok(TracingStopResult {
            data: std::mem::take(&mut data),
            data_loss_occurred: params.data_loss_occurred,
            format: params.trace_format,
            compression: params.stream_compression,
            saved_path,
        })
    }

    async fn drain_stream(&self, handle: String) -> Result<Vec<u8>> {
        let mut buffer = Vec::new();
        loop {
            let chunk: io::ReadReturnObject = self
                .page
                .session
                .send_command(
                    io::Read {
                        handle: handle.clone(),
                        offset: None,
                        size: None,
                    },
                    None,
                )
                .await?;

            if chunk.base_64_encoded.unwrap_or(false) {
                let mut decoded = BASE64_STANDARD
                    .decode(chunk.data.as_bytes())
                    .map_err(|err| {
                        CdpError::protocol(format!(
                            "Failed to decode base64-encoded tracing chunk: {err}"
                        ))
                    })?;
                buffer.append(&mut decoded);
            } else {
                buffer.extend_from_slice(chunk.data.as_bytes());
            }

            if chunk.eof {
                break;
            }
        }
        Ok(buffer)
    }

    async fn close_stream(&self, handle: String) -> Result<()> {
        let _: io::CloseReturnObject = self
            .page
            .session
            .send_command(io::Close { handle }, None)
            .await?;
        Ok(())
    }
}

fn build_start_command(options: &TracingStartOptions) -> tracing_cdp::Start {
    let mut categories: Vec<String> = options.categories.clone().unwrap_or_else(|| {
        DEFAULT_TRACE_CATEGORIES
            .iter()
            .map(|c| c.to_string())
            .collect()
    });

    if options.screenshots
        && !categories
            .iter()
            .any(|cat| cat == "disabled-by-default-devtools.screenshot")
    {
        categories.push("disabled-by-default-devtools.screenshot".to_string());
    }

    tracing_cdp::Start {
        categories: None,
        options: Some("record-as-much-as-possible".to_string()),
        buffer_usage_reporting_interval: options.buffer_usage_reporting_interval_ms,
        transfer_mode: Some(StartTransferModeOption::ReturnAsStream),
        stream_format: Some(options.stream_format.clone()),
        stream_compression: Some(options.stream_compression.clone()),
        trace_config: Some(tracing_cdp::TraceConfig {
            record_mode: Some(options.record_mode.clone()),
            trace_buffer_size_in_kb: None,
            enable_sampling: Some(true),
            enable_systrace: None,
            enable_argument_filter: None,
            included_categories: if categories.is_empty() {
                None
            } else {
                Some(categories)
            },
            excluded_categories: None,
            synthetic_delays: None,
            memory_dump_config: None,
        }),
        perfetto_config: None,
        tracing_backend: options.tracing_backend.clone(),
    }
}
