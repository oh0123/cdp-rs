use super::{
    discovery::{find_available_port, find_running_browser_port},
    launcher::{BrowserLaunchOptions, BrowserType, LaunchedBrowser},
    ws_endpoints::resolve_browser_ws_url,
};
use crate::emulation::EmulationConfig;
use crate::error::Result;
use crate::page::Page;
use crate::session::Session;
use crate::transport::{cdp_protocol::*, websocket_connection::*};

use cdp_protocol::browser::{
    GrantPermissions, GrantPermissionsReturnObject, PermissionDescriptor, PermissionSetting,
    PermissionType, ResetPermissions, ResetPermissionsReturnObject, SetDownloadBehavior,
    SetDownloadBehaviorBehaviorOption, SetDownloadBehaviorReturnObject, SetPermission,
    SetPermissionReturnObject,
};
use cdp_protocol::target::{
    AttachToTargetReturnObject, CreateBrowserContext, CreateBrowserContextReturnObject,
    CreateTarget, CreateTargetReturnObject, DisposeBrowserContext,
};
use futures_util::StreamExt;
use serde::Deserialize;
use serde_json::Value;
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::sync::{Arc, Weak};
use std::time::Duration;
use tokio::sync::{Mutex, mpsc};
use tokio_tungstenite::connect_async;
use url::Url;

/// Options applied when creating a [`BrowserContext`].
#[derive(Clone, Debug, Default)]
pub struct BrowserContextOptions {
    pub dispose_on_detach: Option<bool>,
    pub proxy_server: Option<String>,
    pub proxy_bypass_list: Option<String>,
    pub origins_with_universal_network_access: Option<Vec<String>>,
    pub download: Option<DownloadOptions>,
    pub permission_grants: Vec<PermissionGrant>,
    pub permission_overrides: Vec<PermissionOverride>,
    pub emulation: Option<EmulationConfig>,
}

impl BrowserContextOptions {
    pub fn with_emulation(mut self, emulation: EmulationConfig) -> Self {
        self.emulation = Some(emulation);
        self
    }
}

/// Configuration for download behavior within a context.
#[derive(Clone, Debug)]
pub struct DownloadOptions {
    pub behavior: DownloadBehavior,
    pub download_path: Option<PathBuf>,
    pub events_enabled: Option<bool>,
}

impl DownloadOptions {
    pub fn new(behavior: DownloadBehavior) -> Self {
        Self {
            behavior,
            download_path: None,
            events_enabled: None,
        }
    }

    pub fn with_path<P: Into<PathBuf>>(mut self, path: P) -> Self {
        self.download_path = Some(path.into());
        self
    }

    pub fn with_events_enabled(mut self, enabled: bool) -> Self {
        self.events_enabled = Some(enabled);
        self
    }
}

/// Supported Chrome download behaviors.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum DownloadBehavior {
    Deny,
    #[default]
    Allow,
    AllowAndName,
    Default,
}

impl DownloadBehavior {
    fn as_cdp_behavior(self) -> SetDownloadBehaviorBehaviorOption {
        match self {
            DownloadBehavior::Deny => SetDownloadBehaviorBehaviorOption::Deny,
            DownloadBehavior::Allow => SetDownloadBehaviorBehaviorOption::Allow,
            DownloadBehavior::AllowAndName => SetDownloadBehaviorBehaviorOption::AllowAndName,
            DownloadBehavior::Default => SetDownloadBehaviorBehaviorOption::Default,
        }
    }
}

/// A batch of permissions that should be granted when a context is created.
#[derive(Clone, Debug, Default)]
pub struct PermissionGrant {
    pub origin: Option<String>,
    pub permissions: Vec<PermissionType>,
}

impl PermissionGrant {
    pub fn new<I>(permissions: I) -> Self
    where
        I: IntoIterator<Item = PermissionType>,
    {
        Self {
            origin: None,
            permissions: permissions.into_iter().collect(),
        }
    }

    pub fn with_origin<T: Into<String>>(mut self, origin: T) -> Self {
        self.origin = Some(origin.into());
        self
    }
}

/// Fine-grained permission override using a [`PermissionDescriptor`].
#[derive(Clone, Debug)]
pub struct PermissionOverride {
    pub descriptor: PermissionDescriptor,
    pub setting: PermissionSetting,
    pub origin: Option<String>,
    pub embedded_origin: Option<String>,
}

impl PermissionOverride {
    pub fn new(descriptor: PermissionDescriptor, setting: PermissionSetting) -> Self {
        Self {
            descriptor,
            setting,
            origin: None,
            embedded_origin: None,
        }
    }

    pub fn with_origin<T: Into<String>>(mut self, origin: T) -> Self {
        self.origin = Some(origin.into());
        self
    }
}

#[derive(Default)]
struct BrowserContextState {
    closed: bool,
    emulation_config: Option<EmulationConfig>,
    pages: Vec<Weak<Page>>,
}

/// Builder used to launch or connect to a Chromium-based browser instance.
///
/// # Examples
/// ```no_run
/// use cdp_core::Browser;
///
/// # async fn example() -> cdp_core::Result<()> {
/// let browser = Browser::launcher()
///     .port(9222)
///     .launch()
///     .await?;
/// # let _ = browser;
/// # Ok(())
/// # }
/// ```
pub struct Launcher {
    port: Option<u16>,
    connect_addr: Option<String>,
    browser_type: BrowserType,
    launch_options: BrowserLaunchOptions,
}

impl Default for Launcher {
    fn default() -> Self {
        Self {
            port: None,
            connect_addr: None,
            browser_type: BrowserType::Chrome,
            launch_options: BrowserLaunchOptions::default(),
        }
    }
}

impl Launcher {
    pub fn port(mut self, port: u16) -> Self {
        self.port = Some(port);
        self
    }

    pub fn connect_to_existing(mut self, addr: &str) -> Self {
        self.connect_addr = Some(addr.to_string());
        self
    }

    pub fn browser(mut self, browser: BrowserType) -> Self {
        self.browser_type = browser;
        self
    }

    pub fn launch_options(mut self, options: BrowserLaunchOptions) -> Self {
        self.launch_options = options;
        self
    }

    pub fn configure_options(mut self, configure: impl FnOnce(&mut BrowserLaunchOptions)) -> Self {
        configure(&mut self.launch_options);
        self
    }

    pub fn disable_images(mut self, disable: bool) -> Self {
        self.launch_options.disable_image_loading = disable;
        self
    }

    pub fn mute_audio(mut self, mute: bool) -> Self {
        self.launch_options.mute_audio = mute;
        self
    }

    pub fn incognito(mut self, incognito: bool) -> Self {
        self.launch_options.incognito = incognito;
        self
    }

    pub fn user_data_dir<P: Into<PathBuf>>(mut self, path: P) -> Self {
        self.launch_options.user_data_dir = Some(path.into());
        self
    }

    pub fn clear_user_data_dir(mut self) -> Self {
        self.launch_options.user_data_dir = None;
        self
    }

    pub fn profile_directory<S: Into<String>>(mut self, profile: S) -> Self {
        self.launch_options.profile_directory = Some(profile.into());
        self
    }

    pub fn clear_profile_directory(mut self) -> Self {
        self.launch_options.profile_directory = None;
        self
    }

    pub fn add_extension<P: Into<PathBuf>>(mut self, path: P) -> Self {
        self.launch_options.add_extension(path);
        self
    }

    pub fn remove_extension<P: AsRef<Path>>(mut self, path: P) -> Self {
        self.launch_options.remove_extension(path);
        self
    }

    pub fn clear_extensions(mut self) -> Self {
        self.launch_options.clear_extensions();
        self
    }

    pub fn disable_extensions_except<I, S>(mut self, ids: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<String>,
    {
        self.launch_options.disable_extensions_except(ids);
        self
    }

    pub fn remove_default_flag<S: Into<String>>(mut self, flag: S) -> Self {
        self.launch_options.remove_default_flag(flag);
        self
    }

    pub fn arg<S: Into<String>>(mut self, arg: S) -> Self {
        self.launch_options.add_arg(arg);
        self
    }

    pub fn set_switch_flag<S: Into<String>>(mut self, switch: S) -> Self {
        self.launch_options.set_switch_flag(switch);
        self
    }

    pub fn set_switch_value<S, V>(mut self, switch: S, value: V) -> Self
    where
        S: Into<String>,
        V: Into<String>,
    {
        self.launch_options.set_switch_value(switch, value);
        self
    }

    pub fn clear_switch<S: Into<String>>(mut self, switch: S) -> Self {
        self.launch_options.clear_switch(switch);
        self
    }

    pub fn enable_feature<S: Into<String>>(mut self, feature: S) -> Self {
        self.launch_options.enable_feature(feature);
        self
    }

    pub fn disable_feature<S: Into<String>>(mut self, feature: S) -> Self {
        self.launch_options.disable_feature(feature);
        self
    }

    pub fn force_field_trial<S: Into<String>>(mut self, trial: S) -> Self {
        self.launch_options.force_field_trial(trial);
        self
    }

    pub async fn launch(self) -> Result<Arc<Browser>> {
        let Launcher {
            port,
            connect_addr,
            browser_type,
            launch_options,
        } = self;

        let (ws_url, process) = if let Some(addr) = connect_addr {
            // Mode 2: connect to an existing browser instance.
            let url = resolve_browser_ws_url(&addr).await?;
            (url, None)
        } else {
            // Try to find running browser first
            match find_running_browser_port(browser_type) {
                Ok(found_port) => {
                    let addr = format!("http://127.0.0.1:{}", found_port);
                    tracing::info!("Connecting to existing browser at {}", addr);
                    let url = resolve_browser_ws_url(&addr).await?;
                    (url, None)
                }
                Err(_) => {
                    // No running browser, launch new
                    let selected_port = if let Some(p) = port {
                        p
                    } else {
                        find_available_port().await?
                    };
                    let launched =
                        browser_type.launch_with_options(selected_port, launch_options)?;
                    let addr = format!("http://127.0.0.1:{}", launched.debug_port);
                    tracing::info!("Launched new browser at {}", addr);
                    let url = resolve_browser_ws_url(&addr).await?;
                    (url, Some(ChromeProcess(launched)))
                }
            }
        };

        Browser::connect(ws_url, process).await
    }
}

/// RAII guard that terminates the spawned browser process when dropped.
struct ChromeProcess(LaunchedBrowser);
impl Drop for ChromeProcess {
    fn drop(&mut self) {
        self.0.child.kill().ok();
    }
}

/// High-level interface for working with a Chrome DevTools Protocol browser.
///
/// Instances are typically produced via [`Browser::launcher`].
pub struct Browser {
    internals: Arc<ConnectionInternals>,
    session_event_senders: Arc<Mutex<HashMap<String, mpsc::Sender<CdpEvent>>>>,
    active_pages: Arc<Mutex<HashMap<String, Arc<Page>>>>, // K: sessionId, V: Arc<Page>
    browser_contexts: Arc<Mutex<HashMap<String, Weak<BrowserContext>>>>,
    _chrome_process: Option<ChromeProcess>,
}

impl Browser {
    /// Returns a [`Launcher`] configured with default settings.
    ///
    /// # Examples
    /// ```no_run
    /// use cdp_core::Browser;
    ///
    /// # async fn example() -> cdp_core::Result<()> {
    /// let browser = Browser::launcher().launch().await?;
    /// # let _ = browser;
    /// # Ok(())
    /// # }
    /// ```
    pub fn launcher() -> Launcher {
        Launcher::default()
    }

    async fn connect(ws_url: Url, process: Option<ChromeProcess>) -> Result<Arc<Browser>> {
        let (ws_stream, _) = connect_async(ws_url.as_str()).await?;
        let (writer, reader) = ws_stream.split();
        let connection = Arc::new(ConnectionInternals::new(writer));
        let active_pages = Arc::new(Mutex::new(HashMap::new()));
        let session_event_senders = Arc::new(Mutex::new(HashMap::new()));
        let browser_contexts = Arc::new(Mutex::new(HashMap::new()));
        tokio::spawn(central_message_dispatcher(
            reader,
            Arc::clone(&connection),
            Arc::clone(&active_pages),
            Arc::clone(&session_event_senders),
        ));
        let browser = Browser {
            internals: connection,
            session_event_senders,
            active_pages,
            browser_contexts,
            _chrome_process: process,
        };
        let browser_arc = Arc::new(browser);
        Ok(browser_arc)
    }

    /// Creates a new, isolated browser context using default options.
    /// This is equivalent to opening a new incognito window.
    ///
    /// # Examples
    /// ```no_run
    /// use cdp_core::Browser;
    ///
    /// # async fn example() -> cdp_core::Result<()> {
    /// let browser = Browser::launcher().launch().await?;
    /// let context = browser.new_context().await?;
    /// # let _ = context;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn new_context(self: &Arc<Self>) -> Result<Arc<BrowserContext>> {
        self.new_context_with_options(BrowserContextOptions::default())
            .await
    }

    /// Creates a new browser context with the provided options.
    ///
    /// # Examples
    /// ```no_run
    /// use cdp_core::{Browser, BrowserContextOptions};
    ///
    /// # async fn example() -> cdp_core::Result<()> {
    /// let browser = Browser::launcher().launch().await?;
    /// let options = BrowserContextOptions {
    ///     dispose_on_detach: Some(true),
    ///     ..Default::default()
    /// };
    /// let context = browser.new_context_with_options(options).await?;
    /// # let _ = context;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn new_context_with_options(
        self: &Arc<Self>,
        options: BrowserContextOptions,
    ) -> Result<Arc<BrowserContext>> {
        let method = CreateBrowserContext {
            dispose_on_detach: options.dispose_on_detach,
            proxy_server: options.proxy_server.clone(),
            proxy_bypass_list: options.proxy_bypass_list.clone(),
            origins_with_universal_network_access: options
                .origins_with_universal_network_access
                .clone(),
        };
        let obj = self
            .send_command::<_, CreateBrowserContextReturnObject>(method, None)
            .await?;
        let context_id = obj.browser_context_id;
        println!("Created new BrowserContext with ID: {}", context_id);

        let context = Arc::new(BrowserContext::new(context_id.clone(), Arc::clone(self)));
        self.register_context(&context).await;

        if let Err(err) = context.apply_options(&options).await {
            self.unregister_context(&context_id).await;
            return Err(err);
        }

        Ok(context)
    }

    /// Returns all live browser contexts tracked by this browser.
    pub async fn contexts(&self) -> Vec<Arc<BrowserContext>> {
        let mut contexts = Vec::new();
        let mut guard = self.browser_contexts.lock().await;
        guard.retain(|_, weak| match weak.upgrade() {
            Some(context) => {
                contexts.push(context);
                true
            }
            None => false,
        });
        contexts
    }

    /// Attempts to fetch an existing browser context by id.
    pub async fn get_context(&self, id: &str) -> Option<Arc<BrowserContext>> {
        let mut guard = self.browser_contexts.lock().await;
        if let Some(weak) = guard.get(id)
            && let Some(context) = weak.upgrade()
        {
            return Some(context);
        }
        guard.remove(id);
        None
    }

    async fn register_context(&self, context: &Arc<BrowserContext>) {
        let id = context.id().to_string();
        self.browser_contexts
            .lock()
            .await
            .insert(id, Arc::downgrade(context));
    }

    async fn unregister_context(&self, id: &str) {
        self.browser_contexts.lock().await.remove(id);
    }

    /// Opens a new page in the default browser context.
    ///
    /// # Examples
    /// ```no_run
    /// use cdp_core::Browser;
    ///
    /// # async fn example() -> cdp_core::Result<()> {
    /// let browser = Browser::launcher().launch().await?;
    /// let page = browser.new_page().await?;
    /// # let _ = page;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn new_page(self: &Arc<Self>) -> Result<Arc<Page>> {
        // We call a new method on BrowserContext, passing `None` for the context id
        self.new_page_in_context(None).await
    }

    /// The underlying method that actually creates pages
    pub(crate) async fn new_page_in_context(&self, context_id: Option<&str>) -> Result<Arc<Page>> {
        let (event_sender, event_receiver) = mpsc::channel(crate::DEFAULT_CHANNEL_CAPACITY);
        let method = CreateTarget {
            url: "about:blank".to_string(),
            left: None,
            top: None,
            width: None,
            height: None,
            window_state: None,
            browser_context_id: context_id.map(|s| s.to_string()),
            enable_begin_frame_control: None,
            new_window: None,
            background: None,
            for_tab: None,
            hidden: None,
        };
        let obj = self
            .send_command::<_, CreateTargetReturnObject>(method, None)
            .await?;
        let method = cdp_protocol::target::AttachToTarget {
            target_id: obj.target_id,
            flatten: Some(true),
        };
        let obj = self
            .send_command::<_, AttachToTargetReturnObject>(method, None)
            .await?;
        self.session_event_senders
            .lock()
            .await
            .insert(obj.session_id.clone(), event_sender);

        let session = Session::new(
            Some(obj.session_id.clone()),
            Arc::clone(&self.internals),
            event_receiver,
        );
        let page = Arc::new(Page::new_from_browser(Arc::new(session)));

        // Enable all required domains through the DomainManager helper.
        page.domain_manager.enable_required_domains().await?;

        self.active_pages
            .lock()
            .await
            .insert(obj.session_id.clone(), Arc::clone(&page));
        Ok(page)
    }

    pub(crate) async fn send_command<
        M: serde::Serialize + std::fmt::Debug + cdp_protocol::types::Method,
        R: for<'de> Deserialize<'de>,
    >(
        &self,
        method: M,
        timeout: Option<Duration>,
    ) -> Result<R> {
        self.internals.send(method, None, timeout).await
    }
}

/// Represents an isolated browser context (similar to an incognito profile).
pub struct BrowserContext {
    id: String,            // The browserContextId from CDP
    browser: Arc<Browser>, // Reference back to the parent browser
    state: Mutex<BrowserContextState>,
}

impl BrowserContext {
    fn new(id: String, browser: Arc<Browser>) -> Self {
        Self {
            id,
            browser,
            state: Mutex::new(BrowserContextState::default()),
        }
    }

    pub fn id(&self) -> &str {
        &self.id
    }

    async fn apply_options(&self, options: &BrowserContextOptions) -> Result<()> {
        for grant in &options.permission_grants {
            self.apply_permission_grant(grant).await?;
        }

        for permission in &options.permission_overrides {
            self.set_permission_override(permission).await?;
        }

        if let Some(download) = &options.download {
            self.set_download_behavior(download).await?;
        }

        if let Some(emulation) = &options.emulation {
            self.set_emulation_config(emulation.clone()).await?;
        }

        Ok(())
    }

    /// Creates a new page within this specific browser context.
    ///
    /// # Examples
    /// ```no_run
    /// use cdp_core::Browser;
    ///
    /// # async fn example() -> cdp_core::Result<()> {
    /// let browser = Browser::launcher().launch().await?;
    /// let context = browser.new_context().await?;
    /// let page = context.new_page().await?;
    /// # let _ = page;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn new_page(&self) -> Result<Arc<Page>> {
        let page = self.browser.new_page_in_context(Some(&self.id)).await?;
        self.register_page(&page).await;
        self.apply_emulation_to_page(&page).await?;
        Ok(page)
    }

    /// Grants the provided permissions for the optional origin.
    pub async fn grant_permissions(
        &self,
        origin: Option<&str>,
        permissions: &[PermissionType],
    ) -> Result<()> {
        if permissions.is_empty() {
            return Ok(());
        }

        let method = GrantPermissions {
            permissions: permissions.to_vec(),
            origin: origin.map(|value| value.to_string()),
            browser_context_id: Some(self.id.clone()),
        };
        let _: GrantPermissionsReturnObject = self.browser.send_command(method, None).await?;
        Ok(())
    }

    /// Applies a permission grant described by [`PermissionGrant`].
    pub async fn apply_permission_grant(&self, grant: &PermissionGrant) -> Result<()> {
        self.grant_permissions(grant.origin.as_deref(), &grant.permissions)
            .await
    }

    /// Sets a fine-grained permission override.
    pub async fn set_permission_override(&self, permission: &PermissionOverride) -> Result<()> {
        let method = SetPermission {
            permission: permission.descriptor.clone(),
            setting: permission.setting.clone(),
            origin: permission.origin.clone(),
            embedded_origin: permission.embedded_origin.clone(),
            browser_context_id: Some(self.id.clone()),
        };
        let _: SetPermissionReturnObject = self.browser.send_command(method, None).await?;
        Ok(())
    }

    /// Resets all permission overrides that have been applied to this context.
    pub async fn reset_permissions(&self) -> Result<()> {
        let method = ResetPermissions {
            browser_context_id: Some(self.id.clone()),
        };
        let _: ResetPermissionsReturnObject = self.browser.send_command(method, None).await?;
        Ok(())
    }

    /// Configures how downloads should be handled within this context.
    pub async fn set_download_behavior(&self, options: &DownloadOptions) -> Result<()> {
        let download_path = options
            .download_path
            .as_ref()
            .map(|path| path.to_string_lossy().into_owned());
        let method = SetDownloadBehavior {
            behavior: options.behavior.as_cdp_behavior(),
            browser_context_id: Some(self.id.clone()),
            download_path,
            events_enabled: options.events_enabled,
        };
        let _: SetDownloadBehaviorReturnObject = self.browser.send_command(method, None).await?;
        Ok(())
    }

    /// Resets download handling back to Chrome defaults.
    pub async fn clear_download_behavior(&self) -> Result<()> {
        let method = SetDownloadBehavior {
            behavior: SetDownloadBehaviorBehaviorOption::Default,
            browser_context_id: Some(self.id.clone()),
            download_path: None,
            events_enabled: None,
        };
        let _: SetDownloadBehaviorReturnObject = self.browser.send_command(method, None).await?;
        Ok(())
    }

    /// Applies the provided emulation configuration to all existing pages and stores it for future pages.
    pub async fn set_emulation_config(&self, config: EmulationConfig) -> Result<()> {
        let pages = {
            let mut state = self.state.lock().await;
            state.emulation_config = Some(config.clone());
            let mut pages = Vec::new();
            state.pages.retain(|weak| match weak.upgrade() {
                Some(page) => {
                    pages.push(page);
                    true
                }
                None => false,
            });
            pages
        };

        for page in pages {
            page.emulation().apply_config(&config).await?;
        }

        Ok(())
    }

    /// Clears the stored emulation configuration and best-effort resets overrides on existing pages.
    pub async fn clear_emulation_config(&self) -> Result<()> {
        let (pages, previous) = {
            let mut state = self.state.lock().await;
            let previous = state.emulation_config.take();
            let mut pages = Vec::new();
            state.pages.retain(|weak| match weak.upgrade() {
                Some(page) => {
                    pages.push(page);
                    true
                }
                None => false,
            });
            (pages, previous)
        };

        if let Some(config) = previous {
            for page in pages {
                let controller = page.emulation();
                if config.geolocation.is_some() {
                    controller.clear_geolocation().await?;
                }
                if config.timezone_id.is_some() {
                    controller.reset_timezone().await?;
                }
                if config.locale.is_some() {
                    controller.set_locale(None).await?;
                }
                if config.media.is_some() {
                    controller.clear_media().await?;
                }
                // User agent overrides remain active unless explicitly replaced.
            }
        }

        Ok(())
    }

    /// Closes this browser context and all pages within it.
    pub async fn close(&self) -> Result<()> {
        {
            let mut state = self.state.lock().await;
            if state.closed {
                return Ok(());
            }
            state.closed = true;
        }

        let method = DisposeBrowserContext {
            browser_context_id: self.id.clone(),
        };
        let result: Result<Value> = self.browser.send_command(method, None).await;

        if let Err(err) = result {
            let mut state = self.state.lock().await;
            state.closed = false;
            return Err(err);
        }

        {
            let mut state = self.state.lock().await;
            state.pages.clear();
            state.emulation_config = None;
        }

        self.browser.unregister_context(&self.id).await;
        Ok(())
    }

    async fn register_page(&self, page: &Arc<Page>) {
        let mut state = self.state.lock().await;
        state.pages.retain(|weak| weak.upgrade().is_some());
        let weak = Arc::downgrade(page);
        if !state
            .pages
            .iter()
            .any(|existing| Weak::ptr_eq(existing, &weak))
        {
            state.pages.push(weak);
        }
    }

    async fn apply_emulation_to_page(&self, page: &Arc<Page>) -> Result<()> {
        let config = {
            let state = self.state.lock().await;
            state.emulation_config.clone()
        };

        if let Some(config) = config {
            page.emulation().apply_config(&config).await?;
        }

        Ok(())
    }
}
