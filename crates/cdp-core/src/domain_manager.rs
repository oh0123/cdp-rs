//! CDP Domain Manager
//!
//! Centralized helper that enables and disables Chrome DevTools Protocol domains.
//!
//! # Design Goals
//!
//! 1. **Required Domains Managed Automatically:** Core domains such as Page, Runtime, DOM, and
//!    Network can be enabled up-front and stay active for the lifetime of the page (RAII-style).
//! 2. **Optional Domains On Demand:** Optional domains like Fetch, Performance, and CSS expose
//!    explicit enable/disable helpers so they are activated only when needed.
//! 3. **Guard Against Duplicate Operations:** Internal state tracks each domain so repeated
//!    enable/disable calls become no-ops.
//! 4. **Resource Efficiency:** Domains remain disabled until requested, keeping browser overhead
//!    low.
//!
//! # Examples
//!
//! ```no_run
//! # use cdp_core::Page;
//! # use std::sync::Arc;
//! # async fn example(page: Arc<Page>) -> anyhow::Result<()> {
//! // Required domains (Page, Runtime, DOM, Network) are enabled when the page is created.
//!
//! // Enable optional domains as needed.
//! page.domain_manager.enable_fetch_domain().await?;
//! page.domain_manager.enable_performance_domain().await?;
//!
//! // Disable optional domains to free resources once finished.
//! page.domain_manager.disable_fetch_domain().await?;
//! page.domain_manager.disable_performance_domain().await?;
//!
//! // When the page is dropped, all domains are cleaned up automatically.
//! # Ok(())
//! # }
//! ```

use crate::error::Result;
use cdp_protocol::{dom, fetch, network, page as page_cdp, performance, runtime as runtime_cdp};
use serde::Deserialize;
use std::sync::Arc;
use tokio::sync::Mutex;

use crate::session::Session;

/// Enumerates known Chrome domains this crate manages.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DomainType {
    /// Page domain - controls lifecycle events (required).
    Page,
    /// Runtime domain - exposes the JavaScript runtime (required).
    Runtime,
    /// DOM domain - DOM inspection and manipulation (required).
    Dom,
    /// Network domain - network monitoring (required).
    Network,
    /// Fetch domain - request interception (optional).
    Fetch,
    /// Performance domain - performance metrics (optional).
    Performance,
    /// CSS domain - stylesheet inspection (optional).
    Css,
    /// Debugger domain - script debugging (optional).
    Debugger,
    /// Profiler domain - CPU profiling (optional).
    Profiler,
    /// Input domain - keyboard control (optional).
    Keyboard,
    /// Input domain - mouse control (optional).
    Mouse,
}

impl DomainType {
    /// Returns `true` for domains that should stay enabled for the entire page lifecycle.
    pub fn is_required(&self) -> bool {
        matches!(
            self,
            DomainType::Page | DomainType::Runtime | DomainType::Dom | DomainType::Network
        )
    }

    /// Returns the DevTools Protocol domain name.
    pub fn name(&self) -> &'static str {
        match self {
            DomainType::Page => "Page",
            DomainType::Runtime => "Runtime",
            DomainType::Dom => "DOM",
            DomainType::Network => "Network",
            DomainType::Fetch => "Fetch",
            DomainType::Performance => "Performance",
            DomainType::Css => "CSS",
            DomainType::Debugger => "Debugger",
            DomainType::Profiler => "Profiler",
            DomainType::Keyboard => "Keyboard",
            DomainType::Mouse => "Mouse",
        }
    }
}

/// Tracks the current enablement status for a domain.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DomainState {
    /// Disabled and idle.
    Disabled,
    /// In the process of enabling (command dispatched, awaiting completion).
    Enabling,
    /// Fully enabled and available.
    Enabled,
    /// In the process of disabling.
    Disabling,
}

/// Configuration values applied while enabling individual domains.
#[derive(Debug, Clone)]
pub struct DomainConfig {
    /// Whether to emit `FileChooserOpened` events from the Page domain.
    pub page_enable_file_chooser: bool,
    /// DOM whitespace behavior.
    pub dom_include_whitespace: Option<dom::EnableIncludeWhitespaceOption>,
    /// Network buffer size limits.
    pub network_max_total_buffer_size: Option<u32>,
    pub network_max_resource_buffer_size: Option<u32>,
    pub network_max_post_data_size: Option<u32>,
    /// Whether Fetch should handle auth requests by default.
    pub fetch_handle_auth_requests: bool,
}

impl Default for DomainConfig {
    fn default() -> Self {
        Self {
            page_enable_file_chooser: true,
            dom_include_whitespace: Some(dom::EnableIncludeWhitespaceOption::All),
            network_max_total_buffer_size: None,
            network_max_resource_buffer_size: None,
            network_max_post_data_size: None,
            fetch_handle_auth_requests: false,
        }
    }
}

/// Shared state guarded by the domain manager mutex.
struct DomainManagerInner {
    /// Per-domain state map.
    states: std::collections::HashMap<DomainType, DomainState>,
    /// Static configuration.
    config: DomainConfig,
    /// Underlying session reference.
    session: Arc<Session>,
}

/// Coordinates enabling/disabling Chrome domains in a Page lifecycle.
pub struct DomainManager {
    inner: Arc<Mutex<DomainManagerInner>>,
}

impl DomainManager {
    /// Creates a new domain manager. Call [`enable_required_domains`](Self::enable_required_domains)
    /// to activate the core domains.
    pub(crate) fn new(session: Arc<Session>) -> Self {
        Self {
            inner: Arc::new(Mutex::new(DomainManagerInner {
                states: std::collections::HashMap::new(),
                config: DomainConfig::default(),
                session,
            })),
        }
    }

    /// Creates a new domain manager with a custom configuration.
    pub(crate) fn with_config(session: Arc<Session>, config: DomainConfig) -> Self {
        Self {
            inner: Arc::new(Mutex::new(DomainManagerInner {
                states: std::collections::HashMap::new(),
                config,
                session,
            })),
        }
    }

    /// Enables the required domains (Page, Runtime, DOM, Network).
    ///
    /// Call this immediately after constructing a `Page` to ensure core features work.
    pub async fn enable_required_domains(&self) -> Result<()> {
        tracing::debug!("Enabling required CDP domains...");

        self.enable_page_domain().await?;
        self.enable_runtime_domain().await?;
        self.enable_dom_domain().await?;
        self.enable_network_domain().await?;

        tracing::info!("All required CDP domains are enabled");
        Ok(())
    }

    /// Disables every domain that is currently enabled.
    ///
    /// This is automatically invoked when the manager is dropped, but can be called manually to
    /// free resources earlier.
    pub async fn disable_all_domains(&self) -> Result<()> {
        tracing::debug!("Disabling all enabled CDP domains...");

        let inner = self.inner.lock().await;
        let enabled_domains: Vec<DomainType> = inner
            .states
            .iter()
            .filter(|(_, state)| **state == DomainState::Enabled)
            .map(|(domain, _)| *domain)
            .collect();

        drop(inner); // Release the lock before issuing disable commands.

        for domain in enabled_domains {
            if let Err(e) = self.disable_domain_internal(domain).await {
                tracing::warn!("Failed to disable {} domain: {:?}", domain.name(), e);
            }
        }

        tracing::info!("All CDP domains disabled");
        Ok(())
    }

    /// Returns true if the specified domain is enabled or currently enabling.
    pub async fn is_enabled(&self, domain: DomainType) -> bool {
        let inner = self.inner.lock().await;
        matches!(
            inner.states.get(&domain),
            Some(DomainState::Enabled) | Some(DomainState::Enabling)
        )
    }

    /// Returns the current state tracked for the domain.
    pub async fn get_state(&self, domain: DomainType) -> DomainState {
        let inner = self.inner.lock().await;
        inner
            .states
            .get(&domain)
            .copied()
            .unwrap_or(DomainState::Disabled)
    }

    // ===== Required domain helpers =====

    /// Enables the Page domain.
    pub async fn enable_page_domain(&self) -> Result<()> {
        self.enable_domain_generic::<_, _, page_cdp::EnableReturnObject>(
            DomainType::Page,
            |config| page_cdp::Enable {
                enable_file_chooser_opened_event: Some(config.page_enable_file_chooser),
            },
        )
        .await
    }

    /// Enables the Runtime domain.
    pub async fn enable_runtime_domain(&self) -> Result<()> {
        self.enable_domain_generic::<_, _, runtime_cdp::EnableReturnObject>(
            DomainType::Runtime,
            |_| runtime_cdp::Enable(None),
        )
        .await
    }

    /// Enables the DOM domain.
    pub async fn enable_dom_domain(&self) -> Result<()> {
        self.enable_domain_generic::<_, _, dom::EnableReturnObject>(DomainType::Dom, |config| {
            dom::Enable {
                include_whitespace: config.dom_include_whitespace.clone(),
            }
        })
        .await
    }

    /// Enables the Network domain.
    pub async fn enable_network_domain(&self) -> Result<()> {
        self.enable_domain_generic::<_, _, network::EnableReturnObject>(
            DomainType::Network,
            |config| network::Enable {
                max_total_buffer_size: config.network_max_total_buffer_size,
                max_resource_buffer_size: config.network_max_resource_buffer_size,
                max_post_data_size: config.network_max_post_data_size,
                report_direct_socket_traffic: None,
            },
        )
        .await
    }

    // ===== Optional domain helpers =====

    /// Enables the Fetch domain so requests can be intercepted.
    pub async fn enable_fetch_domain(&self) -> Result<()> {
        self.enable_fetch_domain_with_patterns(None).await
    }

    /// Enables the Fetch domain using the provided request interception patterns.
    pub async fn enable_fetch_domain_with_patterns(
        &self,
        patterns: Option<Vec<fetch::RequestPattern>>,
    ) -> Result<()> {
        self.enable_domain_generic::<_, _, fetch::EnableReturnObject>(DomainType::Fetch, |config| {
            fetch::Enable {
                patterns,
                handle_auth_requests: Some(config.fetch_handle_auth_requests),
            }
        })
        .await
    }

    /// Disables the Fetch domain.
    pub async fn disable_fetch_domain(&self) -> Result<()> {
        self.disable_domain_internal(DomainType::Fetch).await
    }

    /// Enables the Performance domain.
    pub async fn enable_performance_domain(&self) -> Result<()> {
        self.enable_domain_generic::<_, _, performance::EnableReturnObject>(
            DomainType::Performance,
            |_| performance::Enable { time_domain: None },
        )
        .await
    }

    /// Disables the Performance domain.
    pub async fn disable_performance_domain(&self) -> Result<()> {
        self.disable_domain_internal(DomainType::Performance).await
    }

    // ===== Internal helpers =====

    /// Updates the cached domain state.
    async fn set_state(&self, domain: DomainType, state: DomainState) {
        let mut inner = self.inner.lock().await;
        inner.states.insert(domain, state);
    }

    async fn enable_domain_generic<F, M, R>(
        &self,
        domain: DomainType,
        command_factory: F,
    ) -> Result<()>
    where
        F: FnOnce(&DomainConfig) -> M,
        M: serde::Serialize + std::fmt::Debug + cdp_protocol::types::Method,
        R: for<'de> Deserialize<'de>,
    {
        if self.is_enabled(domain).await {
            return Ok(());
        }

        tracing::debug!("Enabling {} domain...", domain.name());
        self.set_state(domain, DomainState::Enabling).await;

        let inner = self.inner.lock().await;
        let command = command_factory(&inner.config);
        let result = inner.session.send_command::<M, R>(command, None).await;
        drop(inner);

        match result {
            Ok(_) => {
                self.set_state(domain, DomainState::Enabled).await;
                tracing::debug!("{} domain enabled", domain.name());
                Ok(())
            }
            Err(e) => {
                self.set_state(domain, DomainState::Disabled).await;
                Err(e)
            }
        }
    }

    /// Generic implementation for disabling a domain.
    async fn disable_domain_internal(&self, domain: DomainType) -> Result<()> {
        if !self.is_enabled(domain).await {
            return Ok(());
        }

        tracing::debug!("Disabling {} domain...", domain.name());
        self.set_state(domain, DomainState::Disabling).await;

        let inner = self.inner.lock().await;
        let result: Result<()> = match domain {
            DomainType::Fetch => {
                let disable = fetch::Disable(None);
                inner
                    .session
                    .send_command::<_, fetch::DisableReturnObject>(disable, None)
                    .await
                    .map(|_| ())
            }
            DomainType::Performance => {
                let disable = performance::Disable(None);
                inner
                    .session
                    .send_command::<_, performance::DisableReturnObject>(disable, None)
                    .await
                    .map(|_| ())
            }
            DomainType::Mouse | DomainType::Keyboard => Ok(()),
            DomainType::Page | DomainType::Runtime | DomainType::Dom | DomainType::Network => {
                // Required domains typically remain enabled; log instead of disabling.
                tracing::warn!(
                    "{} domain is required and should normally stay enabled",
                    domain.name()
                );
                Ok(())
            }
            DomainType::Css | DomainType::Debugger | DomainType::Profiler => {
                tracing::warn!("Disabling the {} domain is not supported", domain.name());
                Ok(())
            }
        };
        drop(inner);

        if result.is_ok() {
            self.set_state(domain, DomainState::Disabled).await;
            tracing::debug!("{} domain disabled", domain.name());
        } else {
            // If disabling failed, leave the domain marked as enabled.
            self.set_state(domain, DomainState::Enabled).await;
        }

        result
    }
}

impl Drop for DomainManager {
    fn drop(&mut self) {
        // Async work is not possible inside Drop; logging only.
        // Pages should call `disable_all_domains()` before dropping when cleanup is required.
        tracing::debug!("Dropping DomainManager");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_domain_type_is_required() {
        assert!(DomainType::Page.is_required());
        assert!(DomainType::Runtime.is_required());
        assert!(DomainType::Dom.is_required());
        assert!(DomainType::Network.is_required());
        assert!(!DomainType::Fetch.is_required());
        assert!(!DomainType::Performance.is_required());
    }

    #[test]
    fn test_domain_config_default() {
        let config = DomainConfig::default();
        assert!(config.page_enable_file_chooser);
        assert!(!config.fetch_handle_auth_requests);
    }
}
