use crate::page::Page;
use crate::{CdpCommandBuilder, Result};
use async_trait::async_trait;
use cdp_protocol::network as network_cdp;
use serde_json::{Map, Value};
use std::collections::HashMap;

/// Network domain controls exposed on [`Page`].
///
/// These methods provide high-level page-scoped network controls. Use [`Page::cdp`] for native
/// CDP command parameters and per-command timeout control.
#[async_trait]
pub trait NetworkControl {
    /// Clears the browser's HTTP cache.
    async fn clear_browser_cache(&self) -> Result<()>;

    /// Enables or disables network cache usage for this page.
    async fn set_cache_disabled(&self, cache_disabled: bool) -> Result<()>;

    /// Bypasses service workers for each request when enabled.
    async fn set_bypass_service_worker(&self, bypass: bool) -> Result<()>;

    /// Sets extra HTTP headers sent with requests from this page.
    async fn set_extra_http_headers(&self, headers: HashMap<String, String>) -> Result<()>;

    /// Sets URL block patterns using the current CDP `BlockPattern` representation.
    ///
    /// This is a native CDP wrapper because it exposes protocol-level `BlockPattern` entries.
    fn set_blocked_url_patterns(
        &self,
        patterns: Vec<network_cdp::BlockPattern>,
    ) -> CdpCommandBuilder<'_, network_cdp::SetBlockedURLs>;

    /// Blocks URL patterns using CDP's wildcard URL syntax.
    ///
    /// This accepts ergonomic patterns such as `"*.png"` and sends them through the legacy
    /// `urls` field. Use [`NetworkControl::set_blocked_url_patterns`] for absolute `URLPattern`
    /// entries that need ordered allow/block behavior.
    async fn block_urls<I, S>(&self, patterns: I) -> Result<()>
    where
        I: IntoIterator<Item = S> + Send,
        I::IntoIter: Send,
        S: Into<String> + Send;

    /// Returns the response body for a network request.
    async fn get_response_body(
        &self,
        request_id: impl Into<network_cdp::RequestId> + Send,
    ) -> Result<network_cdp::GetResponseBodyReturnObject>;

    /// Returns POST data sent with a network request.
    async fn get_request_post_data(
        &self,
        request_id: impl Into<network_cdp::RequestId> + Send,
    ) -> Result<network_cdp::GetRequestPostDataReturnObject>;
}

#[async_trait]
impl NetworkControl for Page {
    async fn clear_browser_cache(&self) -> Result<()> {
        let _: network_cdp::ClearBrowserCacheReturnObject = self
            .session
            .send_command(network_cdp::ClearBrowserCache(None), None)
            .await?;
        Ok(())
    }

    async fn set_cache_disabled(&self, cache_disabled: bool) -> Result<()> {
        let _: network_cdp::SetCacheDisabledReturnObject = self
            .session
            .send_command(network_cdp::SetCacheDisabled { cache_disabled }, None)
            .await?;
        Ok(())
    }

    async fn set_bypass_service_worker(&self, bypass: bool) -> Result<()> {
        let _: network_cdp::SetBypassServiceWorkerReturnObject = self
            .session
            .send_command(network_cdp::SetBypassServiceWorker { bypass }, None)
            .await?;
        Ok(())
    }

    async fn set_extra_http_headers(&self, headers: HashMap<String, String>) -> Result<()> {
        let headers = headers
            .into_iter()
            .map(|(key, value)| (key, Value::String(value)))
            .collect::<Map<String, Value>>();
        let _: network_cdp::SetExtraHTTPHeadersReturnObject = self
            .session
            .send_command(
                network_cdp::SetExtraHTTPHeaders {
                    headers: network_cdp::Headers(Some(Value::Object(headers))),
                },
                None,
            )
            .await?;
        Ok(())
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

    async fn block_urls<I, S>(&self, patterns: I) -> Result<()>
    where
        I: IntoIterator<Item = S> + Send,
        I::IntoIter: Send,
        S: Into<String> + Send,
    {
        #[allow(deprecated)]
        let method = network_cdp::SetBlockedURLs {
            url_patterns: None,
            urls: Some(patterns.into_iter().map(Into::into).collect()),
        };
        let _: network_cdp::SetBlockedURLsReturnObject =
            self.session.send_command(method, None).await?;
        Ok(())
    }

    async fn get_response_body(
        &self,
        request_id: impl Into<network_cdp::RequestId> + Send,
    ) -> Result<network_cdp::GetResponseBodyReturnObject> {
        self.session
            .send_command(
                network_cdp::GetResponseBody {
                    request_id: request_id.into(),
                },
                None,
            )
            .await
    }

    async fn get_request_post_data(
        &self,
        request_id: impl Into<network_cdp::RequestId> + Send,
    ) -> Result<network_cdp::GetRequestPostDataReturnObject> {
        self.session
            .send_command(
                network_cdp::GetRequestPostData {
                    request_id: request_id.into(),
                },
                None,
            )
            .await
    }
}
