use super::page_core::Page;
use crate::error::Result;
use crate::network::cookies::{Cookie, CookieManager, SetCookieParams};
use crate::storage::manager::{StorageManager, StorageType};
use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;

/// Complete page session state including cookies and storage
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PageSession {
    /// Current page URL
    pub url: String,
    /// All cookies
    pub cookies: Vec<Cookie>,
    /// localStorage items
    pub local_storage: HashMap<String, String>,
    /// sessionStorage items
    pub session_storage: HashMap<String, String>,
    /// Timestamp when session was captured
    pub timestamp: u64,
}

impl PageSession {
    /// Create a new empty session
    pub fn new(url: String) -> Self {
        Self {
            url,
            cookies: Vec::new(),
            local_storage: HashMap::new(),
            session_storage: HashMap::new(),
            timestamp: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap_or_default()
                .as_secs(),
        }
    }

    /// Serialize session to JSON string
    pub fn to_json(&self) -> Result<String> {
        Ok(serde_json::to_string_pretty(self)?)
    }

    /// Deserialize session from JSON string
    pub fn from_json(json: &str) -> Result<Self> {
        Ok(serde_json::from_str(json)?)
    }

    /// Save session to file
    pub fn save_to_file(&self, path: &std::path::Path) -> Result<()> {
        let json = self.to_json()?;
        std::fs::write(path, json)?;
        Ok(())
    }

    /// Load session from file
    pub fn load_from_file(path: &std::path::Path) -> Result<Self> {
        let json = std::fs::read_to_string(path)?;
        Self::from_json(&json)
    }
}

/// Session management trait for Page
#[async_trait]
pub trait PageSessionManager {
    /// Export complete page session (cookies + storage)
    async fn export_session(self: &Arc<Self>) -> Result<PageSession>;

    /// Import complete page session (cookies + storage)
    /// Note: This will navigate to the session URL first
    async fn import_session(self: &Arc<Self>, session: &PageSession) -> Result<()>;

    /// Export session and save to file
    async fn export_session_to_file(
        self: &Arc<Self>,
        path: &std::path::Path,
    ) -> Result<PageSession>;

    /// Load session from file and import
    async fn import_session_from_file(self: &Arc<Self>, path: &std::path::Path) -> Result<()>;
}

#[async_trait]
impl PageSessionManager for Page {
    async fn export_session(self: &Arc<Self>) -> Result<PageSession> {
        // Get current URL from main frame
        let main_frame = self.main_frame().await?;
        let url = main_frame.url().await?;

        // Get all cookies
        let cookies = self.get_cookies(None).await?;

        // Get localStorage items
        let local_storage_items = self.get_storage_items(StorageType::Local).await?;
        let local_storage: HashMap<String, String> = local_storage_items
            .into_iter()
            .map(|item| (item.key, item.value))
            .collect();

        // Get sessionStorage items
        let session_storage_items = self.get_storage_items(StorageType::Session).await?;
        let session_storage: HashMap<String, String> = session_storage_items
            .into_iter()
            .map(|item| (item.key, item.value))
            .collect();

        Ok(PageSession {
            url,
            cookies,
            local_storage,
            session_storage,
            timestamp: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap_or_default()
                .as_secs(),
        })
    }

    async fn import_session(self: &Arc<Self>, session: &PageSession) -> Result<()> {
        // Navigate to the session URL first
        self.navigate(&session.url).await?;

        // Clear existing state
        self.clear_browser_cookies().await?;
        self.clear_storage(StorageType::Local).await?;
        self.clear_storage(StorageType::Session).await?;

        // Import cookies - convert Cookie to SetCookieParams
        for cookie in &session.cookies {
            let set_cookie_params = SetCookieParams {
                name: cookie.name.clone(),
                value: cookie.value.clone(),
                url: Some(session.url.clone()),
                domain: Some(cookie.domain.clone()),
                path: Some(cookie.path.clone()),
                secure: Some(cookie.secure),
                http_only: Some(cookie.http_only),
                same_site: cookie.same_site.clone(),
                expires: Some(cookie.expires),
                priority: Some(cookie.priority.clone()),
            };
            self.set_cookie(set_cookie_params).await?;
        }

        // Import localStorage
        for (key, value) in &session.local_storage {
            self.set_storage_item(StorageType::Local, key, value)
                .await?;
        }

        // Import sessionStorage
        for (key, value) in &session.session_storage {
            self.set_storage_item(StorageType::Session, key, value)
                .await?;
        }

        Ok(())
    }

    async fn export_session_to_file(
        self: &Arc<Self>,
        path: &std::path::Path,
    ) -> Result<PageSession> {
        let session = self.export_session().await?;
        session.save_to_file(path)?;
        Ok(session)
    }

    async fn import_session_from_file(self: &Arc<Self>, path: &std::path::Path) -> Result<()> {
        let session = PageSession::load_from_file(path)?;
        self.import_session(&session).await?;
        Ok(())
    }
}

/// Convenient methods for session snapshots
#[async_trait]
pub trait PageSessionSnapshot {
    /// Create a quick snapshot of current session
    async fn snapshot(self: &Arc<Self>) -> Result<PageSession>;

    /// Restore from a snapshot
    async fn restore(self: &Arc<Self>, snapshot: &PageSession) -> Result<()>;

    /// Clone current session to another page
    async fn clone_session_to(self: &Arc<Self>, target: &Arc<Page>) -> Result<()>;
}

#[async_trait]
impl PageSessionSnapshot for Page {
    async fn snapshot(self: &Arc<Self>) -> Result<PageSession> {
        self.export_session().await
    }

    async fn restore(self: &Arc<Self>, snapshot: &PageSession) -> Result<()> {
        self.import_session(snapshot).await
    }

    async fn clone_session_to(self: &Arc<Self>, target: &Arc<Page>) -> Result<()> {
        let session = self.export_session().await?;
        target.import_session(&session).await?;
        Ok(())
    }
}
