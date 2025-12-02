use regex::Regex;
use serde::{Deserialize, Serialize};
use url::Url;

use crate::error::{CdpError, Result};

#[derive(Debug, Serialize, Deserialize)]
pub struct BrowserVersion {
    #[serde(rename = "Browser")]
    pub browser: String,
    #[serde(rename = "Protocol-Version")]
    pub protocol_version: String,
    #[serde(rename = "User-Agent")]
    pub user_agent: String,
    #[serde(rename = "V8-Version")]
    pub v8_version: String,
    #[serde(rename = "WebKit-Version")]
    pub webkit_version: String,
    #[serde(rename = "webSocketDebuggerUrl")]
    pub web_socket_debugger_url: String,
}

pub(crate) async fn resolve_browser_ws_url(addr: &str) -> Result<Url> {
    let base = addr.trim_end_matches('/');
    let version_url = format!("{}/json/version", base);

    let timeout_duration = std::time::Duration::from_secs(60);

    match tokio::time::timeout(timeout_duration, retry_with_backoff(&version_url)).await {
        Ok(result) => result,
        Err(_) => Err(CdpError::ws(format!(
            "Timeout: failed to connect to browser at {} after {} seconds",
            version_url,
            timeout_duration.as_secs()
        ))),
    }
}

async fn retry_with_backoff(version_url: &str) -> Result<Url> {
    const RETRY_DELAYS_MS: &[u64] = &[100, 200, 500, 1000, 2000, 3000];

    let mut attempt = 0;

    loop {
        match fetch_browser_version(version_url).await {
            Ok(url) => {
                if attempt > 0 {
                    tracing::debug!(
                        "Successfully connected to browser at {} after {} attempt(s)",
                        version_url,
                        attempt + 1
                    );
                }
                return Ok(url);
            }
            Err(e) => {
                if attempt == 0 {
                    tracing::debug!("Browser not ready yet at {}, will retry", version_url);
                }

                let delay_ms = RETRY_DELAYS_MS
                    .get(attempt)
                    .copied()
                    .unwrap_or_else(|| *RETRY_DELAYS_MS.last().unwrap());

                tokio::time::sleep(std::time::Duration::from_millis(delay_ms)).await;
                attempt += 1;

                // If we've tried many times and still failing, check if it's a persistent error
                if attempt >= 20 {
                    return Err(e);
                }
            }
        }
    }
}

async fn fetch_browser_version(version_url: &str) -> Result<Url> {
    let resp = reqwest::get(version_url)
        .await?
        .json::<BrowserVersion>()
        .await?;

    Url::parse(&resp.web_socket_debugger_url).map_err(|err| {
        CdpError::ws(format!(
            "Failed to parse webSocketDebuggerUrl as Url: {err}"
        ))
    })
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct DevToolsTarget {
    #[serde(default)]
    pub description: Option<String>,
    #[serde(default)]
    pub devtools_frontend_url: Option<String>,
    pub id: String,
    #[serde(default)]
    pub parent_id: Option<String>,
    #[serde(default)]
    pub title: Option<String>,
    #[serde(rename = "type")]
    #[serde(default)]
    pub r#type: String,
    #[serde(default)]
    pub url: Option<String>,
    #[serde(default)]
    pub web_socket_debugger_url: String,
}

impl DevToolsTarget {
    pub fn is_page(&self) -> bool {
        self.r#type == "page"
    }
}

pub(crate) async fn resolve_active_page_ws_url(port: u16, pattern: Option<&str>) -> Result<Url> {
    let target = if let Some(pattern) = pattern {
        let matches = find_page_targets_by_regex(port, pattern).await?;
        let candidate = matches.first().cloned();
        candidate
            .ok_or_else(|| CdpError::ws(format!("No page target matched pattern '{pattern}'.")))?
    } else {
        let targets = list_page_targets(port).await?;
        let candidate = targets.first().cloned();
        candidate.ok_or_else(|| {
            CdpError::ws("No page target found. Is a browser running with a tab open?".to_string())
        })?
    };

    let ws_url = target.web_socket_debugger_url.as_str();
    Url::parse(ws_url).map_err(|err| {
        CdpError::ws(format!(
            "Failed to parse webSocketDebuggerUrl for active page: {err}"
        ))
    })
}

async fn list_page_targets(port: u16) -> Result<Vec<DevToolsTarget>> {
    let endpoint = format!("http://127.0.0.1:{}/json/list", port);
    let targets = reqwest::get(&endpoint)
        .await?
        .json::<Vec<DevToolsTarget>>()
        .await?;
    Ok(targets
        .into_iter()
        .filter(|target| target.is_page())
        .collect())
}

/// Finds all `page` targets whose title or URL matches the provided regular expression.
pub(crate) async fn find_page_targets_by_regex(
    port: u16,
    pattern: &str,
) -> Result<Vec<DevToolsTarget>> {
    let regex = Regex::new(pattern)
        .map_err(|err| CdpError::ws(format!("Invalid regular expression '{pattern}': {err}")))?;
    let targets = list_page_targets(port).await?;
    Ok(targets
        .into_iter()
        .filter(|target| {
            let title = target.title.as_deref().unwrap_or("");
            let url = target.url.as_deref().unwrap_or("");
            regex.is_match(title) || regex.is_match(url)
        })
        .collect())
}
