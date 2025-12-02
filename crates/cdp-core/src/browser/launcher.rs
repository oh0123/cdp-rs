use crate::{
    browser::discovery::{get_executable_version, is_executable, which},
    error::{CdpError, Result},
};

#[cfg(target_os = "macos")]
use crate::browser::discovery::find_app_bundle_for_exec;
use std::{
    fs, io,
    path::{Path, PathBuf},
    process::{Child, Command, Stdio},
};
use tempfile::TempDir;
use tracing::info;

#[cfg(windows)]
use std::ffi::OsStr;

#[derive(Debug)]
pub struct BrowserTypeInfo {
    pub browser: BrowserType,
    pub path: PathBuf,
    pub version: Option<String>,
}

#[derive(Debug)]
pub struct LaunchedBrowser {
    pub browser: BrowserType,
    pub exec_path: PathBuf,
    pub user_data_dir: PathBuf,
    pub debug_port: u16,
    pub child: Child,
    _temp_dir: Option<TempDir>,
}

#[derive(Debug, Clone, Default)]
pub struct BrowserLaunchOptions {
    pub disable_image_loading: bool,
    pub mute_audio: bool,
    pub incognito: bool,
    pub user_data_dir: Option<PathBuf>,
    pub profile_directory: Option<String>,
    pub extension_paths: Vec<PathBuf>,
    pub extension_keep_list: Vec<String>,
    pub remove_default_flags: Vec<String>,
    pub additional_args: Vec<String>,
    flag_overrides: Vec<FlagOverride>,
    pub enable_features: Vec<String>,
    pub disable_features: Vec<String>,
    pub force_field_trials: Vec<String>,
}

#[derive(Debug, Clone)]
struct FlagOverride {
    name: String,
    value: Option<String>,
}

impl BrowserLaunchOptions {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add_extension<P: Into<PathBuf>>(&mut self, path: P) {
        let path = path.into();
        if !self
            .extension_paths
            .iter()
            .any(|existing| existing == &path)
        {
            self.extension_paths.push(path);
        }
    }

    pub fn remove_extension<P: AsRef<Path>>(&mut self, path: P) {
        let target = path.as_ref();
        self.extension_paths
            .retain(|existing| existing.as_path() != target);
    }

    pub fn clear_extensions(&mut self) {
        self.extension_paths.clear();
    }

    pub fn disable_extensions_except<I, S>(&mut self, ids: I)
    where
        I: IntoIterator<Item = S>,
        S: Into<String>,
    {
        self.extension_keep_list.clear();
        for id in ids {
            let value = id.into();
            if !value.is_empty() && !self.extension_keep_list.contains(&value) {
                self.extension_keep_list.push(value);
            }
        }
    }

    pub fn add_arg<S: Into<String>>(&mut self, arg: S) {
        let arg = arg.into();
        if !self.additional_args.contains(&arg) {
            self.additional_args.push(arg);
        }
    }

    pub fn remove_default_flag<S: Into<String>>(&mut self, flag: S) {
        let raw = flag.into();
        let canonical = canonical_switch_name(&raw);
        if !self
            .remove_default_flags
            .iter()
            .any(|existing| existing == &canonical)
        {
            self.remove_default_flags.push(canonical);
        }
    }

    pub fn set_switch_flag<S: Into<String>>(&mut self, switch: S) {
        let raw = switch.into();
        let canonical = canonical_switch_name(&raw);
        self.upsert_switch(canonical, None);
    }

    pub fn set_switch_value<S, V>(&mut self, switch: S, value: V)
    where
        S: Into<String>,
        V: Into<String>,
    {
        let raw = switch.into();
        let canonical = canonical_switch_name(&raw);
        self.upsert_switch(canonical, Some(value.into()));
    }

    pub fn clear_switch<S: Into<String>>(&mut self, switch: S) {
        let raw = switch.into();
        let canonical = canonical_switch_name(&raw);
        self.flag_overrides.retain(|flag| flag.name != canonical);
    }

    fn upsert_switch(&mut self, name: String, value: Option<String>) {
        if let Some(existing) = self
            .flag_overrides
            .iter_mut()
            .find(|flag| flag.name == name)
        {
            existing.value = value;
        } else {
            self.flag_overrides.push(FlagOverride { name, value });
        }
    }

    pub fn enable_feature<S: Into<String>>(&mut self, feature: S) {
        let feature = feature.into();
        if !feature.is_empty() && !self.enable_features.contains(&feature) {
            self.enable_features.push(feature);
        }
    }

    pub fn disable_feature<S: Into<String>>(&mut self, feature: S) {
        let feature = feature.into();
        if !feature.is_empty() && !self.disable_features.contains(&feature) {
            self.disable_features.push(feature);
        }
    }

    pub fn force_field_trial<S: Into<String>>(&mut self, trial: S) {
        let trial = trial.into();
        if !trial.is_empty() && !self.force_field_trials.contains(&trial) {
            self.force_field_trials.push(trial);
        }
    }

    pub fn has_override<S: AsRef<str>>(&self, switch: S) -> bool {
        let canonical = canonical_switch_name(switch.as_ref());
        self.flag_overrides
            .iter()
            .any(|flag| flag.name == canonical)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BrowserType {
    Chrome,
    Chromium,
    Edge,
}

#[cfg(target_os = "macos")]
const CHROME_EXPECTED_EXEC_PATHS: &[&str] =
    &["/Applications/Google Chrome.app/Contents/MacOS/Google Chrome"];
#[cfg(target_os = "windows")]
const CHROME_EXPECTED_EXEC_PATHS: &[&str] = &[
    r"C:\\Program Files\\Google\\Chrome\\Application\\chrome.exe",
    r"C:\\Program Files (x86)\\Google\\Chrome\\Application\\chrome.exe",
];
#[cfg(all(unix, not(target_os = "macos")))]
const CHROME_EXPECTED_EXEC_PATHS: &[&str] = &["/usr/bin/google-chrome", "/usr/bin/chrome"];
#[cfg(not(any(
    target_os = "macos",
    target_os = "windows",
    all(unix, not(target_os = "macos"))
)))]
const CHROME_EXPECTED_EXEC_PATHS: &[&str] = &[];

#[cfg(target_os = "macos")]
const CHROMIUM_EXPECTED_EXEC_PATHS: &[&str] =
    &["/Applications/Chromium.app/Contents/MacOS/Chromium"];
#[cfg(target_os = "windows")]
const CHROMIUM_EXPECTED_EXEC_PATHS: &[&str] = &[
    r"C:\\Program Files\\Chromium\\Application\\chrome.exe",
    r"C:\\Program Files (x86)\\Chromium\\Application\\chrome.exe",
];
#[cfg(all(unix, not(target_os = "macos")))]
const CHROMIUM_EXPECTED_EXEC_PATHS: &[&str] = &["/usr/bin/chromium", "/usr/bin/chromium-browser"];
#[cfg(not(any(
    target_os = "macos",
    target_os = "windows",
    all(unix, not(target_os = "macos"))
)))]
const CHROMIUM_EXPECTED_EXEC_PATHS: &[&str] = &[];

#[cfg(target_os = "macos")]
const EDGE_EXPECTED_EXEC_PATHS: &[&str] =
    &["/Applications/Microsoft Edge.app/Contents/MacOS/Microsoft Edge"];
#[cfg(target_os = "windows")]
const EDGE_EXPECTED_EXEC_PATHS: &[&str] = &[
    r"C:\\Program Files (x86)\\Microsoft\\Edge\\Application\\msedge.exe",
    r"C:\\Program Files\\Microsoft\\Edge\\Application\\msedge.exe",
];
#[cfg(all(unix, not(target_os = "macos")))]
const EDGE_EXPECTED_EXEC_PATHS: &[&str] =
    &["/usr/bin/microsoft-edge", "/usr/bin/microsoft-edge-stable"];
#[cfg(not(any(
    target_os = "macos",
    target_os = "windows",
    all(unix, not(target_os = "macos"))
)))]
const EDGE_EXPECTED_EXEC_PATHS: &[&str] = &[];

const DEFAULT_FLAGS: &[&str] = &[
    "--no-first-run",
    "--no-default-browser-check",
    "--disable-default-apps",
    "--disable-extensions",
    "--disable-component-extensions-with-background-pages",
    "--disable-background-networking",
    "--disable-sync",
    "--disable-translate",
    "--metrics-recording-only",
    "--safebrowsing-disable-auto-update",
];

impl BrowserType {
    pub fn as_str(&self) -> &'static str {
        match self {
            BrowserType::Chrome => "chrome",
            BrowserType::Chromium => "chromium",
            BrowserType::Edge => "edge",
        }
    }

    pub fn installed_variants() -> Vec<BrowserTypeInfo> {
        const ORDER: [BrowserType; 3] = [
            BrowserType::Chrome,
            BrowserType::Chromium,
            BrowserType::Edge,
        ];

        ORDER
            .iter()
            .filter_map(|browser| {
                browser
                    .find_browser_executable()
                    .map(|path| BrowserTypeInfo {
                        browser: *browser,
                        version: get_executable_version(&path).ok(),
                        path,
                    })
            })
            .collect()
    }

    /// Launch a browser with a fresh user-data-dir and remote debugging enabled.
    pub fn launch(&self, debug_port: u16) -> Result<LaunchedBrowser> {
        self.launch_with_options(debug_port, BrowserLaunchOptions::default())
    }

    /// Launch a browser with custom launch options applied.
    pub fn launch_with_options(
        &self,
        debug_port: u16,
        options: BrowserLaunchOptions,
    ) -> Result<LaunchedBrowser> {
        let exec_path = self.find_browser_executable().ok_or_else(|| {
            CdpError::tool(format!(
                "Browser executable for '{}' not found",
                self.as_str()
            ))
        })?;

        for extension_path in &options.extension_paths {
            let metadata = fs::metadata(extension_path).map_err(|err| {
                CdpError::tool(format!(
                    "Failed to access extension '{}': {err}",
                    extension_path.display()
                ))
            })?;

            if !metadata.is_dir() {
                return Err(CdpError::tool(format!(
                    "Extension path '{}' is not a directory",
                    extension_path.display()
                )));
            }
        }

        let (user_data_dir, temp_dir_guard) = match options.user_data_dir.clone() {
            Some(custom_dir) => {
                fs::create_dir_all(&custom_dir).map_err(|err| {
                    CdpError::tool(format!(
                        "Failed to create user-data-dir '{}': {err}",
                        custom_dir.display()
                    ))
                })?;
                (custom_dir, None)
            }
            None => {
                let tempdir = tempfile::Builder::new()
                    .prefix(&format!("{}-remote-", self.as_str()))
                    .tempdir()
                    .map_err(|err| {
                        CdpError::tool(format!("Failed to create temporary user-data-dir: {err}"))
                    })?;
                (tempdir.path().to_path_buf(), Some(tempdir))
            }
        };

        let args = build_launch_args(debug_port, &user_data_dir, &options);

        info!("launching {:?} with args {:?}", exec_path, args);

        let child = spawn_browser(&exec_path, &args).map_err(|err| {
            CdpError::tool(format!("Failed to launch {}: {err}", exec_path.display()))
        })?;

        std::thread::sleep(std::time::Duration::from_secs(3));
        Ok(LaunchedBrowser {
            browser: *self,
            exec_path,
            user_data_dir,
            debug_port,
            child,
            _temp_dir: temp_dir_guard,
        })
    }

    pub fn find_browser_executable(&self) -> Option<PathBuf> {
        for candidate in self.candidates_for_browser() {
            let path = Path::new(candidate);
            if path.is_absolute() {
                if path.exists() && is_executable(path) {
                    return Some(path.to_path_buf());
                }
            } else if let Some(resolved) = which(candidate) {
                return Some(resolved);
            }
        }

        for name in self.generic_names() {
            if let Some(path) = which(name) {
                return Some(path);
            }
        }

        None
    }

    pub fn expected_exec_paths(&self) -> &'static [&'static str] {
        match self {
            BrowserType::Chrome => CHROME_EXPECTED_EXEC_PATHS,
            BrowserType::Chromium => CHROMIUM_EXPECTED_EXEC_PATHS,
            BrowserType::Edge => EDGE_EXPECTED_EXEC_PATHS,
        }
    }

    fn candidates_for_browser(&self) -> Vec<&'static str> {
        #[allow(unused_mut)]
        let mut candidates = self.expected_exec_paths().to_vec();
        match self {
            BrowserType::Chrome => {
                #[cfg(all(unix, not(target_os = "macos")))]
                {
                    candidates.push("google-chrome");
                }
            }
            BrowserType::Chromium => {
                #[cfg(all(unix, not(target_os = "macos")))]
                {
                    candidates.push("chromium");
                }
            }
            BrowserType::Edge => {
                #[cfg(all(unix, not(target_os = "macos")))]
                {
                    candidates.push("microsoft-edge");
                }
            }
        }
        candidates
    }

    fn generic_names(&self) -> &'static [&'static str] {
        match self {
            BrowserType::Chrome => &["google-chrome", "google-chrome-stable", "chrome"],
            BrowserType::Chromium => &["chromium", "chromium-browser"],
            BrowserType::Edge => &["microsoft-edge", "microsoft-edge-stable", "msedge"],
        }
    }
}

fn push_unique(args: &mut Vec<String>, arg: String) {
    if !args.iter().any(|existing| existing == &arg) {
        args.push(arg);
    }
}

fn format_switch(name: &str, value: Option<&String>) -> String {
    match value {
        Some(v) => format!("{name}={v}"),
        None => name.to_string(),
    }
}

fn canonical_switch_name(value: &str) -> String {
    let trimmed = value.trim();
    let without_value = match trimmed.split_once('=') {
        Some((name, _)) => name.trim(),
        None => trimmed,
    };
    let without_prefix = without_value.trim_start_matches('-');
    if without_prefix.is_empty() {
        "--".to_string()
    } else {
        format!("--{}", without_prefix)
    }
}

fn build_launch_args(
    debug_port: u16,
    user_data_dir: &Path,
    options: &BrowserLaunchOptions,
) -> Vec<String> {
    let mut args = Vec::new();

    push_unique(&mut args, format!("--remote-debugging-port={}", debug_port));
    push_unique(
        &mut args,
        format!("--user-data-dir={}", user_data_dir.to_string_lossy()),
    );

    let mut defaults: Vec<String> = DEFAULT_FLAGS
        .iter()
        .map(|flag| (*flag).to_string())
        .collect();
    defaults.retain(|flag| {
        let canonical = canonical_switch_name(flag);
        if options
            .remove_default_flags
            .iter()
            .any(|remove| remove == &canonical)
        {
            return false;
        }
        if options
            .flag_overrides
            .iter()
            .any(|override_flag| override_flag.name == canonical)
        {
            return false;
        }
        true
    });

    if !options.extension_paths.is_empty() || !options.extension_keep_list.is_empty() {
        let disable_extensions_key = canonical_switch_name("--disable-extensions");
        let disable_component_key =
            canonical_switch_name("--disable-component-extensions-with-background-pages");
        defaults.retain(|flag| {
            let canonical = canonical_switch_name(flag);
            canonical != disable_extensions_key && canonical != disable_component_key
        });
    }

    args.extend(defaults);

    if options.disable_image_loading {
        push_unique(
            &mut args,
            "--blink-settings=imagesEnabled=false".to_string(),
        );
    }
    if options.mute_audio {
        push_unique(&mut args, "--mute-audio".to_string());
    }
    if options.incognito {
        push_unique(&mut args, "--incognito".to_string());
    }
    if let Some(profile_directory) = options.profile_directory.as_ref()
        && !profile_directory.trim().is_empty()
        && !options.has_override("--profile-directory")
    {
        push_unique(
            &mut args,
            format!("--profile-directory={}", profile_directory.trim()),
        );
    }
    if !options.extension_paths.is_empty() && !options.has_override("--load-extension") {
        let joined = options
            .extension_paths
            .iter()
            .map(|path| path.to_string_lossy().to_string())
            .collect::<Vec<_>>()
            .join(",");
        if !joined.is_empty() {
            push_unique(&mut args, format!("--load-extension={joined}"));
        }
    }
    if !options.extension_keep_list.is_empty()
        && !options.has_override("--disable-extensions-except")
    {
        let joined = options.extension_keep_list.join(",");
        if !joined.is_empty() {
            push_unique(&mut args, format!("--disable-extensions-except={joined}"));
        }
    }
    if !options.enable_features.is_empty() && !options.has_override("--enable-features") {
        let joined = options.enable_features.join(",");
        if !joined.is_empty() {
            push_unique(&mut args, format!("--enable-features={joined}"));
        }
    }
    if !options.disable_features.is_empty() && !options.has_override("--disable-features") {
        let joined = options.disable_features.join(",");
        if !joined.is_empty() {
            push_unique(&mut args, format!("--disable-features={joined}"));
        }
    }
    if !options.force_field_trials.is_empty() && !options.has_override("--force-fieldtrials") {
        let joined = options.force_field_trials.join(",");
        if !joined.is_empty() {
            push_unique(&mut args, format!("--force-fieldtrials={joined}"));
        }
    }

    for override_switch in &options.flag_overrides {
        let formatted = format_switch(&override_switch.name, override_switch.value.as_ref());
        push_unique(&mut args, formatted);
    }

    for arg in &options.additional_args {
        push_unique(&mut args, arg.clone());
    }

    push_unique(&mut args, "about:blank".to_string());
    args
}

#[cfg(target_os = "macos")]
fn spawn_browser(exec_path: &Path, args: &[String]) -> Result<Child> {
    if let Some(app_bundle) = find_app_bundle_for_exec(exec_path) {
        let mut cmd = Command::new("open");
        cmd.arg("-a").arg(app_bundle).arg("--args");
        cmd.args(args)
            .stdin(Stdio::null())
            .stdout(Stdio::null())
            .stderr(Stdio::null());
        return cmd.spawn().map_err(|err| {
            CdpError::tool(format!(
                "Failed to launch {} via open: {err}",
                exec_path.display()
            ))
        });
    }

    spawn_direct(exec_path, args).map_err(|err| {
        CdpError::tool(format!(
            "Failed to launch {} directly: {err}",
            exec_path.display()
        ))
    })
}

#[cfg(target_os = "windows")]
fn spawn_browser(exec_path: &Path, args: &[String]) -> Result<Child> {
    match spawn_direct(exec_path, args) {
        Ok(child) => Ok(child),
        Err(primary_err) => {
            let mut cmd = Command::new("cmd");
            cmd.arg("/C")
                .arg("start")
                .arg("")
                .arg(exec_path.as_os_str());
            cmd.args(args)
                .stdin(Stdio::null())
                .stdout(Stdio::null())
                .stderr(Stdio::null());
            cmd.spawn().map_err(|fallback_err| {
                CdpError::tool(format!(
                    "Failed to launch {} directly ({primary_err}) and via cmd start ({fallback_err})",
                    exec_path.display()
                ))
            })
        }
    }
}

#[cfg(all(unix, not(target_os = "macos")))]
fn spawn_browser(exec_path: &Path, args: &[String]) -> Result<Child> {
    spawn_direct(exec_path, args)
        .map_err(|err| CdpError::tool(format!("Failed to launch {}: {err}", exec_path.display())))
}

fn spawn_direct(exec_path: &Path, args: &[String]) -> io::Result<Child> {
    let mut cmd = Command::new(exec_path);
    cmd.args(args)
        .stdin(Stdio::null())
        .stdout(Stdio::null())
        .stderr(Stdio::null());
    cmd.spawn()
}
