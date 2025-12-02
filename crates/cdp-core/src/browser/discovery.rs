use crate::{
    browser::launcher::BrowserType,
    error::{CdpError, Result},
};
use rand::Rng;
use std::{
    env, io,
    path::{Path, PathBuf},
    process::Command,
};
use tokio::net::TcpListener;

#[cfg(windows)]
use std::ffi::OsStr;

/// Minimal which implementation: search PATH for an executable with this name.
pub fn which(name: &str) -> Option<PathBuf> {
    if name.contains(std::path::MAIN_SEPARATOR) {
        let path = PathBuf::from(name);
        if path.exists() && is_executable(&path) {
            return Some(path);
        }
        return None;
    }

    if let Some(paths) = env::var_os("PATH") {
        for dir in env::split_paths(&paths) {
            let candidate = dir.join(name);
            if candidate.exists() && is_executable(&candidate) {
                return Some(candidate);
            }
            if cfg!(windows) {
                let exe_candidate = dir.join(format!("{}.exe", name));
                if exe_candidate.exists() && is_executable(&exe_candidate) {
                    return Some(exe_candidate);
                }
            }
        }
    }

    None
}

#[cfg(target_os = "macos")]
pub(crate) fn find_app_bundle_for_exec(exec: &Path) -> Option<PathBuf> {
    let mut current = exec;
    while let Some(parent) = current.parent() {
        if parent
            .extension()
            .and_then(|ext| ext.to_str())
            .map(|ext| ext.eq_ignore_ascii_case("app"))
            .unwrap_or(false)
        {
            return Some(parent.to_path_buf());
        }
        current = parent;
    }
    None
}

pub fn get_executable_version(path: &Path) -> io::Result<String> {
    // WINDOWS
    #[cfg(windows)]
    {
        // Use PowerShell to read the file version info without executing the binary.
        // (Get-Item 'C:\Program Files\Google\Chrome\Application\chrome.exe').VersionInfo.FileVersion
        use std::process::Stdio;
        let p = path.display().to_string().replace('\'', "''"); // escape single quotes in path
        let ps_cmd = format!("(Get-Item '{}').VersionInfo.FileVersion", p);
        let output = Command::new("powershell")
            .args(&["-NoProfile", "-Command", &ps_cmd])
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .output()?;

        if !output.status.success() {
            let err = String::from_utf8_lossy(&output.stderr).to_string();
            return Err(io::Error::new(
                io::ErrorKind::Other,
                format!("powershell failed: {}", err),
            ));
        }

        let out = String::from_utf8_lossy(&output.stdout).trim().to_string();
        if out.is_empty() {
            Err(io::Error::new(
                io::ErrorKind::Other,
                "empty version string from PowerShell",
            ))
        } else {
            Ok(out)
        }
    }

    // macOS
    #[cfg(target_os = "macos")]
    {
        // If the path is inside a .app bundle, walk up to <Foo.app> and use mdls to read kMDItemVersion (doesn't launch app).
        if let Some(app_dir) = find_app_bundle_for_exec(path) {
            let app_path = app_dir.to_string_lossy().to_string();
            let output = Command::new("mdls")
                .args(["-name", "kMDItemVersion", &app_path])
                .output()?;
            if output.status.success() {
                let stdout = String::from_utf8_lossy(&output.stdout).to_string();
                // mdls output example: kMDItemVersion = "115.0.5790.170"
                if let Some(pos) = stdout.find('=') {
                    let val = stdout[pos + 1..]
                        .trim()
                        .trim_matches('"')
                        .trim()
                        .to_string();
                    if !val.is_empty() {
                        return Ok(val);
                    }
                }
            }
        }

        // fallback: try --product-version or --version
        let output = Command::new(path)
            .arg("--product-version")
            .output()
            .or_else(|_| Command::new(path).arg("--version").output())?;
        let mut s = String::from_utf8_lossy(&output.stdout).trim().to_string();
        if s.is_empty() {
            s = String::from_utf8_lossy(&output.stderr).trim().to_string();
        }
        if s.is_empty() {
            Err(io::Error::other("empty version output"))
        } else {
            Ok(s)
        }
    }

    // LINUX / other UNIX
    #[cfg(all(unix, not(target_os = "macos")))]
    {
        // Try --product-version then --version
        let output = Command::new(path)
            .arg("--product-version")
            .output()
            .or_else(|_| Command::new(path).arg("--version").output())?;
        let mut s = String::from_utf8_lossy(&output.stdout).trim().to_string();
        if s.is_empty() {
            s = String::from_utf8_lossy(&output.stderr).trim().to_string();
        }
        if s.is_empty() {
            Err(io::Error::new(io::ErrorKind::Other, "empty version output"))
        } else {
            Ok(s)
        }
    }
}

pub(crate) async fn find_available_port() -> Result<u16> {
    let mut rng = rand::rng();
    for _ in 0..10 {
        let port = rng.random_range(20000..60000);
        if TcpListener::bind(("127.0.0.1", port)).await.is_ok() {
            return Ok(port);
        }
    }
    Err(CdpError::tool(
        "Failed to find available port after 10 attempts",
    ))
}

pub(crate) fn find_running_browser_port(browser_type: BrowserType) -> Result<u16> {
    let search_term = browser_type.as_str();
    let expected_exec_paths = browser_type.expected_exec_paths();
    if let Some(port) = try_find_remote_debug_port(search_term, Some(expected_exec_paths))? {
        Ok(port)
    } else {
        Err(CdpError::tool(format!(
            "No running {} browser with remote debugging enabled found",
            search_term
        )))
    }
}

pub(crate) fn find_running_browser_port_by_process_name(process_name: &str) -> Result<u16> {
    let trimmed = process_name.trim();
    if trimmed.is_empty() {
        return Err(CdpError::tool("Process name must not be empty"));
    }

    if let Some(port) = try_find_remote_debug_port(trimmed, None)? {
        return Ok(port);
    }

    #[cfg(target_os = "windows")]
    {
        let lower = trimmed.to_ascii_lowercase();
        if let Some(stripped) = lower.strip_suffix(".exe") {
            if let Some(port) = try_find_remote_debug_port(stripped, None)? {
                return Ok(port);
            }
        }
    }

    Err(CdpError::tool(format!(
        "No running process matching '{}' with remote debugging enabled found",
        process_name
    )))
}

fn try_find_remote_debug_port(
    process_pattern: &str,
    expected_exec_paths: Option<&[&str]>,
) -> Result<Option<u16>> {
    let pattern = process_pattern.trim();
    if pattern.is_empty() {
        return Ok(None);
    }

    try_find_remote_debug_port_impl(pattern, expected_exec_paths)
}

#[cfg(target_os = "macos")]
fn try_find_remote_debug_port_impl(
    pattern: &str,
    expected_exec_paths: Option<&[&str]>,
) -> Result<Option<u16>> {
    let output = Command::new("pgrep")
        .arg("-f")
        .arg(pattern)
        .output()
        .map_err(|e| CdpError::tool(format!("Failed to run pgrep: {}", e)))?;
    let pids = String::from_utf8_lossy(&output.stdout);
    for pid_str in pids.lines() {
        if let Ok(pid) = pid_str.trim().parse::<u32>() {
            let cmd_output = Command::new("ps")
                .args(["-p", &pid.to_string(), "-o", "command="])
                .output()
                .map_err(|e| {
                    CdpError::tool(format!("Failed to get command for pid {}: {}", pid, e))
                })?;
            let cmd = String::from_utf8_lossy(&cmd_output.stdout).to_string();
            if !matches_expected_exec(&cmd, expected_exec_paths) {
                continue;
            }
            if let Some(port_str) = extract_port_from_cmd(&cmd)
                && let Ok(port) = port_str.parse::<u16>()
            {
                return Ok(Some(port));
            }
        }
    }
    Ok(None)
}

#[cfg(target_os = "linux")]
fn try_find_remote_debug_port_impl(
    pattern: &str,
    expected_exec_paths: Option<&[&str]>,
) -> Result<Option<u16>> {
    let output = Command::new("pgrep")
        .arg("-f")
        .arg(pattern)
        .output()
        .map_err(|e| CdpError::tool(format!("Failed to run pgrep: {}", e)))?;
    let pids = String::from_utf8_lossy(&output.stdout);
    for pid_str in pids.lines() {
        if let Ok(pid) = pid_str.trim().parse::<u32>() {
            let cmd_path = format!("/proc/{}/cmdline", pid);
            if let Ok(cmd_bytes) = std::fs::read(&cmd_path) {
                let cmd = String::from_utf8_lossy(&cmd_bytes).replace('\0', " ");
                if !matches_expected_exec(&cmd, expected_exec_paths) {
                    continue;
                }
                if let Some(port_str) = extract_port_from_cmd(&cmd) {
                    if let Ok(port) = port_str.parse::<u16>() {
                        return Ok(Some(port));
                    }
                }
            }
        }
    }
    Ok(None)
}

#[cfg(target_os = "windows")]
fn try_find_remote_debug_port_impl(
    pattern: &str,
    expected_exec_paths: Option<&[&str]>,
) -> Result<Option<u16>> {
    let sanitized = pattern.replace('\'', "''");
    let condition = format!("$_.Name -like '*{}*'", sanitized);
    let ps_cmd = format!(
        "Get-Process | Where-Object {{ {} }} | Select-Object -ExpandProperty Id",
        condition
    );
    let output = Command::new("powershell")
        .args(&["-Command", &ps_cmd])
        .output()
        .map_err(|e| CdpError::tool(format!("Failed to run PowerShell: {}", e)))?;
    let pids = String::from_utf8_lossy(&output.stdout);
    for pid_str in pids.lines() {
        let pid_str = pid_str.trim();
        if pid_str.is_empty() {
            continue;
        }
        if let Ok(pid) = pid_str.parse::<u32>() {
            let ps_cmd2 = format!(
                "Get-WmiObject Win32_Process -Filter \"ProcessId = {}\" | Select-Object -ExpandProperty CommandLine",
                pid
            );
            let cmd_output = Command::new("powershell")
                .args(&["-Command", &ps_cmd2])
                .output()
                .map_err(|e| {
                    CdpError::tool(format!("Failed to get command for pid {}: {}", pid, e))
                })?;
            let stdout = String::from_utf8_lossy(&cmd_output.stdout);
            let cmd = if stdout.trim().is_empty() {
                String::from_utf8_lossy(&cmd_output.stderr).to_string()
            } else {
                stdout.to_string()
            };
            if !matches_expected_exec(&cmd, expected_exec_paths) {
                continue;
            }
            if let Some(port_str) = extract_port_from_cmd(&cmd) {
                if let Ok(port) = port_str.parse::<u16>() {
                    return Ok(Some(port));
                }
            }
        }
    }
    Ok(None)
}

fn extract_port_from_cmd(cmd: &str) -> Option<String> {
    for part in cmd.split_whitespace() {
        if part.starts_with("--remote-debugging-port=") {
            return Some(part.split('=').nth(1)?.to_string());
        }
    }
    None
}

fn matches_expected_exec(cmd: &str, expected_exec_paths: Option<&[&str]>) -> bool {
    match expected_exec_paths {
        None => true,
        Some([]) => true,
        Some(paths) => {
            let cmd_norm = cmd.to_lowercase();
            paths.iter().any(|path| {
                let trimmed = path.trim();
                if trimmed.is_empty() {
                    return false;
                }
                let path_norm = trimmed.to_lowercase();
                cmd_norm.contains(&path_norm)
            })
        }
    }
}

#[cfg(unix)]
pub(crate) fn is_executable(path: &Path) -> bool {
    use std::os::unix::fs::PermissionsExt;
    path.exists()
        && path
            .metadata()
            .map(|m| m.permissions().mode() & 0o111 != 0)
            .unwrap_or(false)
}

#[cfg(windows)]
pub(crate) fn is_executable(path: &Path) -> bool {
    path.exists()
        && path
            .extension()
            .and_then(OsStr::to_str)
            .map(|ext| ext.eq_ignore_ascii_case("exe"))
            .unwrap_or(false)
}

#[cfg(not(any(unix, windows)))]
pub(crate) fn is_executable(path: &Path) -> bool {
    path.exists()
}
