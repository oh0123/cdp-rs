use crate::CdpCommandBuilder;
use crate::page::Page;
use cdp_protocol::network as network_cdp;
use serde_json::{Map, Value};
use std::collections::HashMap;

impl Page {
    /// Clears the browser's HTTP cache.
    pub fn clear_browser_cache(&self) -> CdpCommandBuilder<'_, network_cdp::ClearBrowserCache> {
        self.cdp(network_cdp::ClearBrowserCache(None))
    }

    /// Enables or disables network cache usage for this page.
    pub fn set_cache_disabled(
        &self,
        cache_disabled: bool,
    ) -> CdpCommandBuilder<'_, network_cdp::SetCacheDisabled> {
        self.cdp(network_cdp::SetCacheDisabled { cache_disabled })
    }

    /// Bypasses service workers for each request when enabled.
    pub fn set_bypass_service_worker(
        &self,
        bypass: bool,
    ) -> CdpCommandBuilder<'_, network_cdp::SetBypassServiceWorker> {
        self.cdp(network_cdp::SetBypassServiceWorker { bypass })
    }

    /// Sets extra HTTP headers sent with requests from this page.
    pub fn set_extra_http_headers(
        &self,
        headers: HashMap<String, String>,
    ) -> CdpCommandBuilder<'_, network_cdp::SetExtraHTTPHeaders> {
        let headers = headers
            .into_iter()
            .map(|(key, value)| (key, Value::String(value)))
            .collect::<Map<String, Value>>();
        self.cdp(network_cdp::SetExtraHTTPHeaders {
            headers: network_cdp::Headers(Some(Value::Object(headers))),
        })
    }

    /// Sets URL block patterns using the current CDP `BlockPattern` representation.
    #[allow(deprecated)]
    pub fn set_blocked_url_patterns(
        &self,
        patterns: Vec<network_cdp::BlockPattern>,
    ) -> CdpCommandBuilder<'_, network_cdp::SetBlockedURLs> {
        self.cdp(network_cdp::SetBlockedURLs {
            url_patterns: Some(patterns),
            urls: None,
        })
    }

    /// Blocks URL patterns. Patterns use the URLPattern constructor string syntax.
    pub fn block_urls<I, S>(
        &self,
        patterns: I,
    ) -> CdpCommandBuilder<'_, network_cdp::SetBlockedURLs>
    where
        I: IntoIterator<Item = S>,
        S: Into<String>,
    {
        let patterns = patterns
            .into_iter()
            .map(|pattern| network_cdp::BlockPattern {
                url_pattern: pattern.into(),
                block: true,
            })
            .collect();
        self.set_blocked_url_patterns(patterns)
    }

    /// Returns the response body for a network request.
    pub fn get_response_body(
        &self,
        request_id: impl Into<network_cdp::RequestId>,
    ) -> CdpCommandBuilder<'_, network_cdp::GetResponseBody> {
        self.cdp(network_cdp::GetResponseBody {
            request_id: request_id.into(),
        })
    }

    /// Returns POST data sent with a network request.
    pub fn get_request_post_data(
        &self,
        request_id: impl Into<network_cdp::RequestId>,
    ) -> CdpCommandBuilder<'_, network_cdp::GetRequestPostData> {
        self.cdp(network_cdp::GetRequestPostData {
            request_id: request_id.into(),
        })
    }
}
