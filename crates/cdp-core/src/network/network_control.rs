use crate::CdpCommandBuilder;
use crate::page::Page;
use cdp_protocol::network as network_cdp;
use serde_json::{Map, Value};
use std::collections::HashMap;

/// Network domain controls exposed on [`Page`].
///
/// These methods return low-level CDP command builders so callers can keep the common wrapper
/// defaults while still adjusting generated protocol parameters and per-command timeouts.
pub trait NetworkControl {
    /// Clears the browser's HTTP cache.
    fn clear_browser_cache(&self) -> CdpCommandBuilder<'_, network_cdp::ClearBrowserCache>;

    /// Enables or disables network cache usage for this page.
    fn set_cache_disabled(
        &self,
        cache_disabled: bool,
    ) -> CdpCommandBuilder<'_, network_cdp::SetCacheDisabled>;

    /// Bypasses service workers for each request when enabled.
    fn set_bypass_service_worker(
        &self,
        bypass: bool,
    ) -> CdpCommandBuilder<'_, network_cdp::SetBypassServiceWorker>;

    /// Sets extra HTTP headers sent with requests from this page.
    fn set_extra_http_headers(
        &self,
        headers: HashMap<String, String>,
    ) -> CdpCommandBuilder<'_, network_cdp::SetExtraHTTPHeaders>;

    /// Sets URL block patterns using the current CDP `BlockPattern` representation.
    fn set_blocked_url_patterns(
        &self,
        patterns: Vec<network_cdp::BlockPattern>,
    ) -> CdpCommandBuilder<'_, network_cdp::SetBlockedURLs>;

    /// Blocks URL patterns using CDP's wildcard URL syntax.
    ///
    /// This accepts ergonomic patterns such as `"*.png"` and sends them through the legacy
    /// `urls` field. Use [`NetworkControl::set_blocked_url_patterns`] for absolute `URLPattern`
    /// entries that need ordered allow/block behavior.
    fn block_urls<I, S>(&self, patterns: I) -> CdpCommandBuilder<'_, network_cdp::SetBlockedURLs>
    where
        I: IntoIterator<Item = S>,
        S: Into<String>;

    /// Returns the response body for a network request.
    fn get_response_body(
        &self,
        request_id: impl Into<network_cdp::RequestId>,
    ) -> CdpCommandBuilder<'_, network_cdp::GetResponseBody>;

    /// Returns POST data sent with a network request.
    fn get_request_post_data(
        &self,
        request_id: impl Into<network_cdp::RequestId>,
    ) -> CdpCommandBuilder<'_, network_cdp::GetRequestPostData>;
}

impl NetworkControl for Page {
    fn clear_browser_cache(&self) -> CdpCommandBuilder<'_, network_cdp::ClearBrowserCache> {
        self.cdp(network_cdp::ClearBrowserCache(None))
    }

    fn set_cache_disabled(
        &self,
        cache_disabled: bool,
    ) -> CdpCommandBuilder<'_, network_cdp::SetCacheDisabled> {
        self.cdp(network_cdp::SetCacheDisabled { cache_disabled })
    }

    fn set_bypass_service_worker(
        &self,
        bypass: bool,
    ) -> CdpCommandBuilder<'_, network_cdp::SetBypassServiceWorker> {
        self.cdp(network_cdp::SetBypassServiceWorker { bypass })
    }

    fn set_extra_http_headers(
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

    #[allow(deprecated)]
    fn set_blocked_url_patterns(
        &self,
        patterns: Vec<network_cdp::BlockPattern>,
    ) -> CdpCommandBuilder<'_, network_cdp::SetBlockedURLs> {
        self.cdp(network_cdp::SetBlockedURLs {
            url_patterns: Some(patterns),
            urls: None,
        })
    }

    fn block_urls<I, S>(&self, patterns: I) -> CdpCommandBuilder<'_, network_cdp::SetBlockedURLs>
    where
        I: IntoIterator<Item = S>,
        S: Into<String>,
    {
        #[allow(deprecated)]
        self.cdp(network_cdp::SetBlockedURLs {
            url_patterns: None,
            urls: Some(patterns.into_iter().map(Into::into).collect()),
        })
    }

    fn get_response_body(
        &self,
        request_id: impl Into<network_cdp::RequestId>,
    ) -> CdpCommandBuilder<'_, network_cdp::GetResponseBody> {
        self.cdp(network_cdp::GetResponseBody {
            request_id: request_id.into(),
        })
    }

    fn get_request_post_data(
        &self,
        request_id: impl Into<network_cdp::RequestId>,
    ) -> CdpCommandBuilder<'_, network_cdp::GetRequestPostData> {
        self.cdp(network_cdp::GetRequestPostData {
            request_id: request_id.into(),
        })
    }
}
