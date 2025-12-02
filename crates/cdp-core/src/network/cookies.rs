use crate::error::Result;
use crate::page::Page;
use async_trait::async_trait;
use serde::{Deserialize, Serialize};

// Re-exporting the generated Cookie type for easier access
pub use cdp_protocol::network::{Cookie, CookiePriority, CookieSameSite};

/// Parameters for setting a cookie.
/// This struct provides a high-level interface for `Network.setCookie`.
#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct SetCookieParams {
    pub name: String,
    pub value: String,
    pub url: Option<String>,
    pub domain: Option<String>,
    pub path: Option<String>,
    pub secure: Option<bool>,
    pub http_only: Option<bool>,
    pub same_site: Option<cdp_protocol::network::CookieSameSite>,
    pub expires: Option<f64>, // CDP uses a specific timestamp format
    pub priority: Option<cdp_protocol::network::CookiePriority>,
}

#[async_trait]
pub trait CookieManager {
    /// Gets cookies for the current page context.
    async fn get_cookies(&self, urls: Option<Vec<String>>) -> Result<Vec<Cookie>>;

    /// Sets a cookie.
    async fn set_cookie(&self, cookie: SetCookieParams) -> Result<bool>;

    /// Deletes cookies.
    async fn delete_cookies(
        &self,
        name: &str,
        url: Option<String>,
        domain: Option<String>,
        path: Option<String>,
    ) -> Result<()>;

    /// Clears all browser cookies for the current browser context.
    async fn clear_browser_cookies(&self) -> Result<()>;
}

#[async_trait]
impl CookieManager for Page {
    async fn get_cookies(&self, urls: Option<Vec<String>>) -> Result<Vec<Cookie>> {
        use cdp_protocol::network::{GetCookies, GetCookiesReturnObject};

        let params = GetCookies { urls };
        let result: GetCookiesReturnObject = self.session.send_command(params, None).await?;
        Ok(result.cookies)
    }

    async fn set_cookie(&self, cookie: SetCookieParams) -> Result<bool> {
        use cdp_protocol::network::{SetCookie, SetCookieReturnObject};

        let params = SetCookie {
            name: cookie.name,
            value: cookie.value,
            url: cookie.url,
            domain: cookie.domain,
            path: cookie.path,
            secure: cookie.secure,
            http_only: cookie.http_only,
            same_site: cookie.same_site,
            expires: cookie.expires,
            priority: cookie.priority,
            same_party: None,
            source_scheme: None,
            source_port: None,
            partition_key: None,
        };

        let result: SetCookieReturnObject = self.session.send_command(params, None).await?;
        Ok(result.success)
    }

    async fn delete_cookies(
        &self,
        name: &str,
        url: Option<String>,
        domain: Option<String>,
        path: Option<String>,
    ) -> Result<()> {
        use cdp_protocol::network::{DeleteCookies, DeleteCookiesReturnObject};

        let params = DeleteCookies {
            name: name.to_string(),
            url,
            domain,
            path,
            partition_key: None,
        };

        self.session
            .send_command::<_, DeleteCookiesReturnObject>(params, None)
            .await?;
        Ok(())
    }

    async fn clear_browser_cookies(&self) -> Result<()> {
        use cdp_protocol::network::{ClearBrowserCookies, ClearBrowserCookiesReturnObject};

        let params = ClearBrowserCookies(None);
        self.session
            .send_command::<_, ClearBrowserCookiesReturnObject>(params, None)
            .await?;
        Ok(())
    }
}
