//! In-app update checker and self-updater.
//!
//! Queries the GitHub Releases API for `block-p/zen-design` to check if a newer
//! release is available, and provides one-click update support for both git-based
//! and standalone binary installations.

use std::fs;
use std::path::Path;
use serde::{Deserialize, Serialize};
use tracing::{info, warn};

pub const CURRENT_VERSION: &str = env!("CARGO_PKG_VERSION");
pub const GITHUB_REPO: &str = "block-p/zen-design";

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct VersionInfo {
    pub current_version: String,
    pub os: String,
    pub arch: String,
    pub is_git_install: bool,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct UpdateCheckResponse {
    pub current_version: String,
    pub latest_version: String,
    pub has_update: bool,
    pub release_name: String,
    pub release_notes: String,
    pub release_url: String,
    pub download_url: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct UpdateResult {
    pub status: String,
    pub message: String,
    pub new_version: Option<String>,
}

/// Parses semver-like version strings ("2.4.0" or "v2.4.1") into (major, minor, patch)
pub fn parse_semver(v: &str) -> (u64, u64, u64) {
    let clean = v.trim_start_matches('v').trim();
    let parts: Vec<&str> = clean.split('.').collect();
    let major = parts.first().and_then(|s| s.parse().ok()).unwrap_or(0);
    let minor = parts.get(1).and_then(|s| s.parse().ok()).unwrap_or(0);
    let patch = parts.get(2).and_then(|s| s.parse().ok()).unwrap_or(0);
    (major, minor, patch)
}

pub fn is_newer_version(latest: &str, current: &str) -> bool {
    parse_semver(latest) > parse_semver(current)
}

pub fn get_version_info() -> VersionInfo {
    let os = std::env::consts::OS.to_string();
    let arch = std::env::consts::ARCH.to_string();
    let is_git_install = check_is_git_install();
    VersionInfo {
        current_version: CURRENT_VERSION.to_string(),
        os,
        arch,
        is_git_install,
    }
}

fn check_is_git_install() -> bool {
    if let Ok(exe) = std::env::current_exe() {
        let mut curr = exe.parent();
        while let Some(parent) = curr {
            if parent.join(".git").is_dir() {
                return true;
            }
            curr = parent.parent();
        }
    }
    Path::new(".git").is_dir() || Path::new("../.git").is_dir()
}

pub async fn check_for_updates(client: &reqwest::Client) -> Result<UpdateCheckResponse, String> {
    let url = format!("https://api.github.com/repos/{GITHUB_REPO}/releases/latest");
    let resp = client
        .get(&url)
        .header("User-Agent", "ZenDesign-App")
        .header("Accept", "application/vnd.github.v3+json")
        .send()
        .await
        .map_err(|e| format!("Failed to reach GitHub: {e}"))?;

    if !resp.status().is_success() {
        return Err(format!("GitHub API returned status {}", resp.status()));
    }

    let val: serde_json::Value = resp
        .json()
        .await
        .map_err(|e| format!("Failed to parse GitHub response: {e}"))?;

    let tag_name = val.get("tag_name").and_then(|v| v.as_str()).unwrap_or("");
    let latest_version = tag_name.trim_start_matches('v').to_string();
    let release_name = val
        .get("name")
        .and_then(|v| v.as_str())
        .unwrap_or(tag_name)
        .to_string();
    let release_notes = val
        .get("body")
        .and_then(|v| v.as_str())
        .unwrap_or("")
        .to_string();
    let release_url = val
        .get("html_url")
        .and_then(|v| v.as_str())
        .unwrap_or("")
        .to_string();

    let has_update = is_newer_version(&latest_version, CURRENT_VERSION);

    // Target asset name based on OS & architecture
    let target_asset_name = if cfg!(target_os = "linux") {
        "zen-design-linux-x86_64"
    } else if cfg!(target_os = "windows") {
        "zen-design-windows-x86_64.exe"
    } else if cfg!(target_os = "macos") {
        if cfg!(target_arch = "aarch64") {
            "zen-design-macos-arm64"
        } else {
            "zen-design-macos-x86_64"
        }
    } else {
        ""
    };

    let mut download_url = None;
    if let Some(assets) = val.get("assets").and_then(|v| v.as_array()) {
        for asset in assets {
            if let Some(name) = asset.get("name").and_then(|v| v.as_str()) {
                if name == target_asset_name {
                    download_url = asset
                        .get("browser_download_url")
                        .and_then(|v| v.as_str())
                        .map(|s| s.to_string());
                    break;
                }
            }
        }
        // Fallback search
        if download_url.is_none() {
            let os_keyword = if cfg!(target_os = "linux") {
                "linux"
            } else if cfg!(target_os = "windows") {
                "windows"
            } else {
                "macos"
            };
            for asset in assets {
                if let Some(name) = asset.get("name").and_then(|v| v.as_str()) {
                    if name.contains(os_keyword)
                        && !name.ends_with(".tar.gz")
                        && !name.ends_with(".zip")
                        && !name.ends_with(".dmg")
                    {
                        download_url = asset
                            .get("browser_download_url")
                            .and_then(|v| v.as_str())
                            .map(|s| s.to_string());
                        break;
                    }
                }
            }
        }
    }

    Ok(UpdateCheckResponse {
        current_version: CURRENT_VERSION.to_string(),
        latest_version,
        has_update,
        release_name,
        release_notes,
        release_url,
        download_url,
    })
}

pub async fn perform_update(
    client: &reqwest::Client,
    download_url_override: Option<&str>,
) -> Result<UpdateResult, String> {
    info!("Starting in-app self-update...");

    // Check if git repository installation
    if check_is_git_install() {
        info!("Detected git clone installation; attempting git pull...");
        let output = std::process::Command::new("git")
            .args(["pull", "--rebase"])
            .output();

        if let Ok(out) = output {
            if out.status.success() {
                let msg = String::from_utf8_lossy(&out.stdout).trim().to_string();
                info!("Git pull succeeded: {msg}");
                return Ok(UpdateResult {
                    status: "ok".to_string(),
                    message: format!("Git repository updated successfully: {msg}. Please restart the app."),
                    new_version: None,
                });
            } else {
                let err_msg = String::from_utf8_lossy(&out.stderr).trim().to_string();
                warn!("Git pull failed ({err_msg}); falling back to release binary download...");
            }
        }
    }

    // Binary download path
    let download_url = if let Some(url) = download_url_override {
        url.to_string()
    } else {
        let check = check_for_updates(client).await?;
        check
            .download_url
            .ok_or_else(|| "No compatible release asset found for your platform.".to_string())?
    };

    info!("Downloading binary from {download_url}...");
    let resp = client
        .get(&download_url)
        .header("User-Agent", "ZenDesign-App")
        .send()
        .await
        .map_err(|e| format!("Failed to download update: {e}"))?;

    if !resp.status().is_success() {
        return Err(format!("Download failed with HTTP {}", resp.status()));
    }

    let bytes = resp
        .bytes()
        .await
        .map_err(|e| format!("Failed to read downloaded file: {e}"))?;

    if bytes.is_empty() {
        return Err("Downloaded update binary was empty.".to_string());
    }

    let current_exe = std::env::current_exe()
        .map_err(|e| format!("Failed to determine current executable path: {e}"))?;

    let exe_dir = current_exe
        .parent()
        .ok_or_else(|| "Failed to get executable directory".to_string())?;

    let temp_exe = exe_dir.join(format!(
        ".zen-design-server.update.{}",
        std::process::id()
    ));

    fs::write(&temp_exe, &bytes)
        .map_err(|e| format!("Failed to write temporary binary: {e}"))?;

    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        let _ = fs::set_permissions(&temp_exe, fs::Permissions::from_mode(0o755));
    }

    // Replace current binary atomically
    let backup_exe = exe_dir.join(".zen-design-server.old");
    let _ = fs::remove_file(&backup_exe);
    if let Err(e) = fs::rename(&current_exe, &backup_exe) {
        warn!("Failed to rename current binary to backup ({e}); trying direct overwrite");
    }

    if let Err(e) = fs::rename(&temp_exe, &current_exe) {
        // Rollback if possible
        let _ = fs::rename(&backup_exe, &current_exe);
        let _ = fs::remove_file(&temp_exe);
        return Err(format!("Failed to replace binary with updated version: {e}"));
    }

    let _ = fs::remove_file(&backup_exe);
    info!("Successfully replaced binary at {:?}", current_exe);

    Ok(UpdateResult {
        status: "ok".to_string(),
        message: "Update installed successfully! Restart the app to run the new version.".to_string(),
        new_version: None,
    })
}
