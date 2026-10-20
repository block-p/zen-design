//! Records the live preview to an MP4.
//!
//! The pipeline is intentionally tool-based: Xvfb provides a virtual display,
//! Chrome paints the artifact full-screen on it, and ffmpeg grabs that display
//! with x11grab. Because the browser does the rendering, any CSS/JS animation
//! is captured exactly as it looks — no automation framework is added to the
//! daemon, and no artifact format is assumed beyond "it runs in a browser".

use std::path::{Path, PathBuf};
use std::process::Stdio;
use std::sync::atomic::{AtomicBool, Ordering};
use std::time::{Duration, SystemTime, UNIX_EPOCH};

use tokio::process::{Child, Command};
use tokio::sync::broadcast;
use tokio::time::{sleep, timeout, Instant};

use crate::providers::emit_event;

const RECORD_WIDTH: u32 = 1440;
const RECORD_HEIGHT: u32 = 900;
const RECORD_FPS: u32 = 60;

/// Chrome needs a moment to paint before capture begins. The preview is loaded
/// in two stages: a black holding page first, then the artifact itself once
/// ffmpeg is already recording (see `export_armed`). That keeps the recorded
/// video aligned with the artifact's first frame instead of its third second.
/// This fixed delay is only used when `xwininfo` is unavailable.
const WARMUP_MS: u64 = 2_400;

pub struct ExportOutcome {
    pub bytes: u64,
}

/// Records `{base_url}/preview/index.html` for `seconds` and writes
/// `preview.mp4` into `export_dir`.
pub async fn record_preview(
    base_url: &str,
    seconds: u64,
    export_dir: &Path,
    sender: &broadcast::Sender<String>,
    armed: &AtomicBool,
) -> Result<ExportOutcome, String> {
    let chrome = chrome_binary().ok_or_else(|| {
        "Google Chrome was not found. Set ZEN_CHROME_BIN to its path.".to_string()
    })?;
    let ffmpeg = binary_or_env("ZEN_FFMPEG_BIN", "ffmpeg")
        .ok_or_else(|| "ffmpeg was not found. Set ZEN_FFMPEG_BIN to its path.".to_string())?;
    let xvfb = binary_or_env("ZEN_XVFB_BIN", "Xvfb")
        .ok_or_else(|| "Xvfb was not found; install xvfb to record MP4 exports.".to_string())?;

    tokio::fs::create_dir_all(export_dir)
        .await
        .map_err(|error| format!("Could not create the export directory: {error}"))?;

    let output = export_dir.join("preview.mp4");
    let _ = tokio::fs::remove_file(&output).await;

    // A dedicated profile keeps this Chrome isolated from the user's browser,
    // and gives cleanup a unique string to match.
    let profile = std::env::temp_dir().join(format!("zen-design-chrome-{}", std::process::id()));
    let _ = tokio::fs::remove_dir_all(&profile).await;
    tokio::fs::create_dir_all(&profile)
        .await
        .map_err(|error| format!("Could not prepare the browser profile: {error}"))?;

    emit_event(sender, "export_status", "Starting a virtual display…");
    let (display, mut display_child) = start_display(&xvfb).await?;

    armed.store(false, Ordering::SeqCst);
    let url = format!("{base_url}/api/export/stage?t={}", unix_millis());
    emit_event(sender, "export_status", "Opening the artifact in Chrome…");
    let mut chrome_child = start_chrome(&chrome, &display, &profile, &url)?;

    // Wait until Chrome has actually mapped a window on the virtual display.
    // Without this check, a browser that opens elsewhere (for example on the
    // user's Wayland session) records a black screen and reports success.
    let capture = match wait_for_browser(&display).await {
        Ok(()) => {
            emit_event(
                sender,
                "export_status",
                format!("Recording {seconds}s at {RECORD_FPS} fps…"),
            );
            capture_display(&ffmpeg, &display, seconds, &output, armed).await
        }
        Err(error) => Err(error),
    };

    // Kill Chrome by profile as well: the launcher exits quickly but leaves
    // renderer children behind, and they keep the display busy.
    let _ = chrome_child.start_kill();
    let _ = chrome_child.wait().await;
    let _ = Command::new("pkill")
        .arg("-f")
        .arg(profile.as_os_str())
        .status()
        .await;
    let _ = display_child.start_kill();
    let _ = display_child.wait().await;
    let _ = tokio::fs::remove_dir_all(&profile).await;

    capture?;

    let metadata = tokio::fs::metadata(&output)
        .await
        .map_err(|error| format!("The recording produced no file: {error}"))?;
    if metadata.len() < 8_000 {
        return Err("The recording came out empty — the preview did not render.".to_string());
    }

    emit_event(
        sender,
        "export_status",
        format!("Encoding finished · {}", format_size(metadata.len())),
    );

    Ok(ExportOutcome {
        bytes: metadata.len(),
    })
}

/// Wait until a browser window is mapped on the virtual display, then let the
/// holding page paint. Returns an error instead of recording a black screen.
pub(crate) async fn wait_for_browser(display: &str) -> Result<(), String> {
    if !find_in_path("xwininfo") {
        // Geometry inspection is unavailable; fall back to a fixed warmup.
        sleep(Duration::from_millis(WARMUP_MS)).await;
        return Ok(());
    }

    let deadline = Instant::now() + Duration::from_secs(8);
    while Instant::now() < deadline {
        if let Ok(output) = Command::new("xwininfo")
            .env("DISPLAY", display)
            .args(["-root", "-tree"])
            .output()
            .await
        {
            if has_browser_window(&String::from_utf8_lossy(&output.stdout)) {
                // One beat for the holding page to paint before capture starts.
                sleep(Duration::from_millis(350)).await;
                return Ok(());
            }
        }
        sleep(Duration::from_millis(150)).await;
    }

    Err("Chrome did not open a window — the recording was cancelled.".to_string())
}

/// True when the X tree contains a mapped child window large enough to be the
/// kiosk browser (helper windows are 1×1 and the root has no geometry line).
fn has_browser_window(tree: &str) -> bool {
    tree.lines().any(|line| {
        let Some(size) = line.split_whitespace().find(|token| {
            token.ends_with("+0+0")
                && token.contains('x')
                && token.starts_with(|character: char| character.is_ascii_digit())
        }) else {
            return false;
        };
        let Some((width, height)) = size.split('+').next().and_then(|spec| spec.split_once('x'))
        else {
            return false;
        };
        width
            .parse::<u32>()
            .map(|width| width > 600)
            .unwrap_or(false)
            && height
                .parse::<u32>()
                .map(|height| height > 400)
                .unwrap_or(false)
    })
}

pub fn format_size(bytes: u64) -> String {
    if bytes >= 1_000_000 {
        format!("{:.1} MB", bytes as f64 / 1_000_000.0)
    } else {
        format!("{} KB", (bytes + 999) / 1_000)
    }
}

pub(crate) async fn start_display(xvfb: &str) -> Result<(String, Child), String> {
    for number in 90..110 {
        let socket = PathBuf::from(format!("/tmp/.X11-unix/X{number}"));
        if socket.exists() {
            continue;
        }

        let geometry = format!("{RECORD_WIDTH}x{RECORD_HEIGHT}x24");
        let child = Command::new(xvfb)
            .arg(format!(":{number}"))
            .args(["-screen", "0", geometry.as_str(), "-nolisten", "tcp", "-ac"])
            .stdin(Stdio::null())
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .kill_on_drop(true)
            .spawn()
            .map_err(|error| format!("Could not start Xvfb: {error}"))?;

        let mut child = child;
        for _ in 0..40 {
            if socket.exists() {
                return Ok((format!(":{number}"), child));
            }
            sleep(Duration::from_millis(75)).await;
        }

        let _ = child.start_kill();
    }

    Err("Could not start a virtual display for the recording.".to_string())
}

pub(crate) fn start_chrome(
    chrome: &str,
    display: &str,
    profile: &Path,
    url: &str,
) -> Result<Child, String> {
    Command::new(chrome)
        .env("DISPLAY", display)
        // The user's desktop may be Wayland; Chrome would otherwise open its
        // window there instead of on the virtual X display being recorded.
        .env_remove("WAYLAND_DISPLAY")
        .arg("--ozone-platform=x11")
        .arg("--disable-gpu")
        .arg("--kiosk")
        .arg(format!("--window-size={RECORD_WIDTH},{RECORD_HEIGHT}"))
        .arg("--window-position=0,0")
        .arg(format!("--user-data-dir={}", profile.display()))
        .args([
            "--no-first-run",
            "--no-default-browser-check",
            "--disable-extensions",
            "--disable-default-apps",
            "--disable-sync",
            "--disable-background-networking",
            "--disable-component-update",
            "--disable-dev-shm-usage",
            "--disable-features=Translate,MediaRouter,OptimizationHints",
            "--hide-scrollbars",
            "--mute-audio",
            "--autoplay-policy=no-user-gesture-required",
            "--force-device-scale-factor=1",
        ])
        .arg(url)
        .stdin(Stdio::null())
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .kill_on_drop(true)
        .spawn()
        .map_err(|error| format!("Could not start Chrome: {error}"))
}

async fn capture_display(
    ffmpeg: &str,
    display: &str,
    seconds: u64,
    output: &Path,
    armed: &AtomicBool,
) -> Result<(), String> {
    record_display(ffmpeg, display, seconds, output, Some(armed)).await
}

/// Records the virtual display without the two-stage export handshake. The
/// deep-review loop uses this to grab a candidate artifact, then extracts a
/// handful of stills for the model to critique.
pub(crate) async fn record_display(
    ffmpeg: &str,
    display: &str,
    seconds: u64,
    output: &Path,
    armed: Option<&AtomicBool>,
) -> Result<(), String> {
    let framerate = RECORD_FPS.to_string();
    let size = format!("{RECORD_WIDTH}x{RECORD_HEIGHT}");
    let duration = seconds.to_string();

    let child = Command::new(ffmpeg)
        .env("DISPLAY", display)
        .args(["-hide_banner", "-loglevel", "error", "-y"])
        .args([
            "-f",
            "x11grab",
            "-draw_mouse",
            "0",
            "-framerate",
            framerate.as_str(),
            "-video_size",
            size.as_str(),
            "-i",
            display,
        ])
        .args([
            "-t",
            duration.as_str(),
            "-an",
            "-c:v",
            "libx264",
            "-preset",
            "medium",
            "-crf",
            "20",
            "-pix_fmt",
            "yuv420p",
            "-movflags",
            "+faststart",
        ])
        .arg(output)
        .stdin(Stdio::null())
        .stdout(Stdio::null())
        .stderr(Stdio::piped())
        .kill_on_drop(true)
        .spawn()
        .map_err(|error| format!("Could not run ffmpeg: {error}"))?;

    // Wait 350ms so ffmpeg's x11grab loop is fully capturing frames before
    // releasing the holding page. This eliminates dropped initial animation frames.
    sleep(Duration::from_millis(350)).await;

    if let Some(armed) = armed {
        armed.store(true, Ordering::SeqCst);
    }

    let waited = timeout(Duration::from_secs(seconds + 30), child.wait_with_output()).await;

    match waited {
        Ok(Ok(result)) => {
            if result.status.success() {
                Ok(())
            } else {
                let stderr = String::from_utf8_lossy(&result.stderr);
                let tail: String = stderr
                    .chars()
                    .rev()
                    .take(500)
                    .collect::<String>()
                    .chars()
                    .rev()
                    .collect();
                Err(format!("ffmpeg failed: {}", tail.trim()))
            }
        }
        Ok(Err(error)) => Err(format!("ffmpeg error: {error}")),
        Err(_) => Err("The recording timed out.".to_string()),
    }
}

pub(crate) fn binary_or_env(env_name: &str, fallback: &str) -> Option<String> {
    if let Ok(value) = std::env::var(env_name) {
        let value = value.trim();
        if !value.is_empty() {
            return Some(value.to_string());
        }
    }
    find_in_path(fallback).then(|| fallback.to_string())
}

pub(crate) fn chrome_binary() -> Option<String> {
    if let Ok(value) = std::env::var("ZEN_CHROME_BIN") {
        let value = value.trim();
        if !value.is_empty() {
            return Some(value.to_string());
        }
    }

    [
        "google-chrome",
        "google-chrome-stable",
        "chromium",
        "chromium-browser",
        "brave-browser",
    ]
    .into_iter()
    .find(|candidate| find_in_path(candidate))
    .map(str::to_string)
}

fn find_in_path(binary: &str) -> bool {
    std::env::var_os("PATH")
        .map(|paths| std::env::split_paths(&paths).any(|dir| dir.join(binary).is_file()))
        .unwrap_or(false)
}

fn unix_millis() -> u128 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|duration| duration.as_millis())
        .unwrap_or_default()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn finds_a_kiosk_sized_browser_window() {
        let tree = r#"
xwininfo: Window id: 0x46f (the root window) (has no name)

  Root window id: 0x46f (the root window) (has no name)
  Parent window id: 0x0 (none)
     2 children:
     0x200002 "ZenDesign recording stage - Google Chrome": ("google-chrome" "Google-chrome")  1439x899+0+0  +0+0
     0x200004 (has no name): ()  1x1+0+0  +0+0
"#;
        assert!(has_browser_window(tree));
    }

    #[test]
    fn ignores_an_empty_or_helper_only_x_tree() {
        let empty = "xwininfo: Window id: 0x46f (the root window) (has no name)\n\n  Root window id: 0x46f (the root window) (has no name)\n  Parent window id: 0x0 (none)\n     0 children:\n";
        assert!(!has_browser_window(empty));
        assert!(!has_browser_window(
            "     1 child:\n     0x200004 (has no name): ()  1x1+0+0  +0+0\n"
        ));
        assert!(!has_browser_window(""));
    }

    #[test]
    fn formats_sizes_for_the_activity_log() {
        assert_eq!(format_size(63_950), "64 KB");
        assert_eq!(format_size(1_260_000), "1.3 MB");
        assert_eq!(format_size(999), "1 KB");
    }
}
