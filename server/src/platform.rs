//! Cross-platform paths, executable lookup, and browser file URLs.
//!
//! The rest of the daemon must not assume `HOME`, `/tmp`, or a Unix `PATH`.
//! Everything that differs between Linux, macOS, and Windows lives here.

use std::path::{Path, PathBuf};

pub fn home_dir() -> Option<PathBuf> {
    std::env::var_os("HOME")
        .or_else(|| std::env::var_os("USERPROFILE"))
        .map(PathBuf::from)
        .filter(|path| !path.as_os_str().is_empty())
}

pub fn config_dir() -> PathBuf {
    if cfg!(windows) {
        std::env::var_os("APPDATA")
            .map(PathBuf::from)
            .or_else(|| home_dir().map(|home| home.join("AppData").join("Roaming")))
            .unwrap_or_else(|| PathBuf::from("."))
    } else if cfg!(target_os = "macos") {
        home_dir()
            .map(|home| home.join("Library").join("Application Support"))
            .unwrap_or_else(|| PathBuf::from("."))
    } else {
        std::env::var_os("XDG_CONFIG_HOME")
            .map(PathBuf::from)
            .filter(|path| !path.as_os_str().is_empty())
            .or_else(|| home_dir().map(|home| home.join(".config")))
            .unwrap_or_else(|| PathBuf::from(".config"))
    }
}

pub fn data_dir() -> PathBuf {
    if cfg!(windows) {
        std::env::var_os("LOCALAPPDATA")
            .map(PathBuf::from)
            .or_else(|| home_dir().map(|home| home.join("AppData").join("Local")))
            .unwrap_or_else(|| config_dir())
    } else if cfg!(target_os = "macos") {
        home_dir()
            .map(|home| home.join("Library").join("Application Support"))
            .unwrap_or_else(|| PathBuf::from("."))
    } else {
        std::env::var_os("XDG_DATA_HOME")
            .map(PathBuf::from)
            .filter(|path| !path.as_os_str().is_empty())
            .or_else(|| home_dir().map(|home| home.join(".local").join("share")))
            .unwrap_or_else(|| PathBuf::from("."))
    }
}

pub fn cache_dir() -> PathBuf {
    if cfg!(windows) {
        data_dir()
    } else if cfg!(target_os = "macos") {
        home_dir()
            .map(|home| home.join("Library").join("Caches"))
            .unwrap_or_else(std::env::temp_dir)
    } else {
        std::env::var_os("XDG_CACHE_HOME")
            .map(PathBuf::from)
            .filter(|path| !path.as_os_str().is_empty())
            .or_else(|| home_dir().map(|home| home.join(".cache")))
            .unwrap_or_else(std::env::temp_dir)
    }
}

pub fn app_config_dir() -> PathBuf {
    config_dir().join("zen-design")
}

pub fn app_config_file() -> PathBuf {
    let path = app_config_dir().join("config.toml");
    prefer_legacy(&path, &["zen-design/config.toml"])
}

pub fn app_history_file() -> PathBuf {
    let path = app_config_dir().join("history.json");
    prefer_legacy(&path, &["zen-design/history.json"])
}

/// Older builds always used `~/.config`; keep reading their files on macOS
/// and Windows so an upgrade does not look like a fresh install.
fn prefer_legacy(path: &Path, suffixes: &[&str]) -> PathBuf {
    if path.is_file() {
        return path.to_path_buf();
    }
    if let Some(home) = home_dir() {
        for suffix in suffixes {
            let legacy = home.join(".config").join(suffix);
            if legacy.is_file() {
                return legacy;
            }
        }
    }
    path.to_path_buf()
}

pub fn default_workspace() -> PathBuf {
    data_dir().join("zen-design").join("workspace")
}

/// Find a real executable, adding the Windows extensions that `Command::new`
/// does not add for `.cmd`/`.bat` scripts.
pub fn find_in_path(name: &str) -> Option<PathBuf> {
    let trimmed = name.trim();
    if trimmed.is_empty() {
        return None;
    }

    let direct = PathBuf::from(trimmed);
    if direct.components().count() > 1 || direct.is_absolute() {
        if direct.is_file() {
            return Some(direct);
        }
        if cfg!(windows) && direct.extension().is_none() {
            for ext in ["exe", "cmd", "bat", "com"] {
                let candidate = direct.with_extension(ext);
                if candidate.is_file() {
                    return Some(candidate);
                }
            }
        }
        return None;
    }

    let mut names = vec![trimmed.to_string()];
    if cfg!(windows) && direct.extension().is_none() {
        for ext in ["exe", "cmd", "bat", "com"] {
            names.push(format!("{trimmed}.{ext}"));
        }
    }

    let paths = std::env::var_os("PATH")?;
    for dir in std::env::split_paths(&paths) {
        for candidate in &names {
            let path = dir.join(candidate);
            if path.is_file() {
                return Some(path);
            }
        }
    }
    None
}

pub fn python_binary() -> Option<PathBuf> {
    for name in ["python3", "python"] {
        if let Some(path) = find_in_path(name) {
            // The Microsoft Store alias is a 0-byte shim that opens the Store.
            if path
                .to_string_lossy()
                .to_ascii_lowercase()
                .contains("windowsapps")
            {
                continue;
            }
            return Some(path);
        }
    }
    None
}

/// Chrome/Chromium paths that are not on `PATH` on a normal desktop install.
pub fn chrome_binary() -> Option<String> {
    if let Ok(value) = std::env::var("ZEN_CHROME_BIN") {
        let value = value.trim();
        if !value.is_empty() {
            return Some(value.to_string());
        }
    }

    let mut candidates: Vec<PathBuf> = Vec::new();
    if cfg!(windows) {
        if let Some(program_files) = std::env::var_os("ProgramFiles") {
            candidates.push(
                PathBuf::from(program_files)
                    .join("Google")
                    .join("Chrome")
                    .join("Application")
                    .join("chrome.exe"),
            );
        }
        if let Some(program_files) = std::env::var_os("ProgramFiles(x86)") {
            candidates.push(
                PathBuf::from(program_files)
                    .join("Google")
                    .join("Chrome")
                    .join("Application")
                    .join("chrome.exe"),
            );
        }
        if let Some(local) = std::env::var_os("LOCALAPPDATA") {
            let local = PathBuf::from(local);
            candidates.push(
                local
                    .join("Google")
                    .join("Chrome")
                    .join("Application")
                    .join("chrome.exe"),
            );
            candidates.push(
                local
                    .join("Microsoft")
                    .join("Edge")
                    .join("Application")
                    .join("msedge.exe"),
            );
        }
    } else if cfg!(target_os = "macos") {
        for root in ["/Applications", "/System/Applications"] {
            candidates.push(
                PathBuf::from(root)
                    .join("Google Chrome.app")
                    .join("Contents")
                    .join("MacOS")
                    .join("Google Chrome"),
            );
            candidates.push(
                PathBuf::from(root)
                    .join("Chromium.app")
                    .join("Contents")
                    .join("MacOS")
                    .join("Chromium"),
            );
            candidates.push(
                PathBuf::from(root)
                    .join("Brave Browser.app")
                    .join("Contents")
                    .join("MacOS")
                    .join("Brave Browser"),
            );
        }
        if let Some(home) = home_dir() {
            candidates.push(
                home.join("Applications")
                    .join("Google Chrome.app")
                    .join("Contents")
                    .join("MacOS")
                    .join("Google Chrome"),
            );
        }
    }

    for candidate in candidates {
        if candidate.is_file() {
            return Some(candidate.to_string_lossy().to_string());
        }
    }

    [
        "google-chrome",
        "google-chrome-stable",
        "chromium",
        "chromium-browser",
        "brave-browser",
        "chrome",
        "msedge",
    ]
    .into_iter()
    .find_map(|name| find_in_path(name).map(|path| path.to_string_lossy().to_string()))
}

/// A browser-safe `file://` URL. `Url::from_file_path` handles spaces,
/// non-ASCII names, and Windows drive letters; Windows canonicalization can
/// prefix `\\?\`, which is stripped first.
pub fn file_url(path: &Path) -> Result<String, String> {
    let normalized = normalize_path(path);
    url::Url::from_file_path(&normalized)
        .map(|url| url.to_string())
        .map_err(|_| format!("Could not turn {} into a file URL", path.display()))
}

fn normalize_path(path: &Path) -> PathBuf {
    #[cfg(windows)]
    {
        let text = path.to_string_lossy();
        if let Some(rest) = text.strip_prefix(r"\\?\UNC\") {
            return PathBuf::from(format!(r"\\{rest}"));
        }
        if let Some(rest) = text.strip_prefix(r"\\?\") {
            return PathBuf::from(rest);
        }
    }
    path.to_path_buf()
}

/// Put the child in its own process group so a tree kill can reach Chrome's
/// renderer children as well as the launcher.
#[allow(dead_code)]
#[allow(dead_code)]
/// Configure a `std::process::Command` so on Windows it runs silently without
/// popping up a console/cmd window.
#[allow(dead_code)]
pub fn configure_command(command: &mut std::process::Command) {
    #[cfg(windows)]
    {
        use std::os::windows::process::CommandExt;
        const CREATE_NO_WINDOW: u32 = 0x0800_0000;
        command.creation_flags(CREATE_NO_WINDOW);
    }
    let _ = command;
}

#[allow(dead_code)]
/// Configure a `tokio::process::Command` so on Windows it runs silently without
/// popping up a console/cmd window.
pub fn configure_tokio_command(command: &mut tokio::process::Command) {
    #[cfg(windows)]
    {
        use std::os::windows::process::CommandExt;
        const CREATE_NO_WINDOW: u32 = 0x0800_0000;
        command.creation_flags(CREATE_NO_WINDOW);
    }
    let _ = command;
}

#[cfg(unix)]
pub fn detach_process_group(command: &mut tokio::process::Command) {
    use std::os::unix::process::CommandExt;
    command.as_std_mut().process_group(0);
}

#[cfg(not(unix))]
pub fn detach_process_group(_command: &mut tokio::process::Command) {}

/// Terminate a process and its descendants. On Unix the caller must have used
/// `detach_process_group` at spawn time so the children share a group id.
pub async fn kill_process_tree(child: &mut tokio::process::Child) {
    let Some(pid) = child.id() else {
        let _ = child.start_kill();
        return;
    };

    #[cfg(windows)]
    {
        let mut cmd = tokio::process::Command::new("taskkill");
        configure_tokio_command(&mut cmd);
        let _ = cmd
            .args(["/PID", &pid.to_string(), "/T", "/F"])
            .stdout(std::process::Stdio::null())
            .stderr(std::process::Stdio::null())
            .status()
            .await;
    }

    #[cfg(unix)]
    {
        // Negative pid targets the whole process group created at spawn time.
        unsafe {
            libc::kill(-(pid as i32), libc::SIGTERM);
        }
        tokio::time::sleep(std::time::Duration::from_millis(120)).await;
        unsafe {
            libc::kill(-(pid as i32), libc::SIGKILL);
        }
    }

    let _ = child.start_kill();
    let _ = child.wait().await;
}

/// Blocking tree kill for `Drop` implementations that cannot await.
pub fn kill_process_tree_blocking(child: &mut tokio::process::Child) {
    let Some(pid) = child.id() else {
        let _ = child.start_kill();
        return;
    };

    #[cfg(windows)]
    {
        let mut cmd = std::process::Command::new("taskkill");
        configure_command(&mut cmd);
        let _ = cmd
            .args(["/PID", &pid.to_string(), "/T", "/F"])
            .stdout(std::process::Stdio::null())
            .stderr(std::process::Stdio::null())
            .status();
    }

    #[cfg(unix)]
    {
        unsafe {
            libc::kill(-(pid as i32), libc::SIGTERM);
        }
        std::thread::sleep(std::time::Duration::from_millis(80));
        unsafe {
            libc::kill(-(pid as i32), libc::SIGKILL);
        }
    }

    let _ = child.start_kill();
}

/// Chrome launchers exit immediately, leaving renderer processes behind.
/// Match them by the unique profile path on Unix; Windows uses the direct
/// process-tree kill first, then a profile-scoped fallback for renderers whose
/// launcher has already exited.
pub fn kill_chrome_profile(profile: &Path) {
    #[cfg(unix)]
    {
        if find_in_path("pkill").is_some() {
            let _ = std::process::Command::new("pkill")
                .arg("-f")
                .arg(profile)
                .stdout(std::process::Stdio::null())
                .stderr(std::process::Stdio::null())
                .status();
        }
    }
    #[cfg(windows)]
    {
        // Do not use `taskkill /IM chrome.exe`: that would close the user's
        // own browser. Scope the cleanup to processes started with our unique
        // recording profile path.
        let escaped = profile.to_string_lossy().replace('\'', "''");
        let script = format!(
            "Get-CimInstance Win32_Process | Where-Object {{ $_.CommandLine -and $_.CommandLine.Contains('{escaped}') }} | ForEach-Object {{ Stop-Process -Id $_.ProcessId -Force -ErrorAction SilentlyContinue }}"
        );
        let mut cmd = std::process::Command::new("powershell");
        configure_command(&mut cmd);
        let _ = cmd
            .args(["-NoProfile", "-NonInteractive", "-Command", &script])
            .stdout(std::process::Stdio::null())
            .stderr(std::process::Stdio::null())
            .status();
    }
    #[cfg(not(any(unix, windows)))]
    {
        let _ = profile;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn config_and_data_dirs_are_absolute_when_home_exists() {
        if home_dir().is_some() {
            assert!(config_dir().is_absolute());
            assert!(data_dir().is_absolute());
            assert!(cache_dir().is_absolute());
            assert!(default_workspace().is_absolute());
        }
    }

    #[test]
    fn file_urls_encode_spaces_and_unicode() {
        let path = std::env::temp_dir().join("zen design").join("سلام.html");
        let url = file_url(&path).expect("file URL");
        assert!(url.starts_with("file://"));
        assert!(!url.contains(' '), "spaces must be percent-encoded: {url}");
    }

    #[test]
    fn finds_an_executable_that_exists() {
        #[cfg(windows)]
        let name = "cmd";
        #[cfg(not(windows))]
        let name = "sh";
        assert!(find_in_path(name).is_some());
    }
}
