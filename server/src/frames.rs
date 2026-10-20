//! Renders a candidate artifact and extracts stills for the deep review pass.
//!
//! The daemon already knows how to put Chrome on a virtual display for MP4
//! export; this reuses that machinery without adding a browser automation
//! stack. Chrome opens a tiny black holding page, ffmpeg starts recording the
//! X display, and the holding page then navigates to the candidate artifact.
//! That keeps the film's first frame near the start of the recording.

use std::path::{Path, PathBuf};
use std::process::Stdio;
use std::time::{SystemTime, UNIX_EPOCH};

use tokio::process::{Child, Command};
use tracing::warn;

use crate::export;

/// The holding page waits this long before showing the artifact. Recording
/// starts once the browser window exists, so this leaves a short black lead-in
/// and then captures the film from its first beat.
const STAGE_DELAY_MS: u64 = 2_000;
const RECORD_SECONDS: u64 = 13;
const FRAME_TIMES: [f64; 4] = [2.9, 5.5, 8.5, 11.4];
const FRAME_LABELS: [&str; 4] = ["~0.9s", "~3.5s", "~6.5s", "~9.4s"];

pub struct CapturedFrames {
    pub pngs: Vec<Vec<u8>>,
    pub labels: Vec<String>,
    pub stats: Vec<FrameStats>,
}

/// Luma statistics of a frame, measured on a 144×90 grayscale reduction.
/// The mean catches an all-white/all-black field; the standard deviation
/// catches a frame whose type has collapsed into the background.
#[derive(Clone, Copy, Debug)]
pub struct FrameStats {
    pub mean: f64,
    pub stddev: f64,
}

impl CapturedFrames {
    /// The flattest of the four stills. If this is tiny, at least one beat is
    /// effectively blank even when the other beats look fine.
    pub fn min_contrast(&self) -> f64 {
        self.stats
            .iter()
            .map(|stats| stats.stddev)
            .fold(f64::INFINITY, f64::min)
    }

    pub fn report(&self) -> String {
        let parts: Vec<String> = self
            .stats
            .iter()
            .zip(self.labels.iter())
            .map(|(stats, label)| {
                format!(
                    "{label}: mean {:.0}, contrast {:.1}",
                    stats.mean, stats.stddev
                )
            })
            .collect();
        format!(
            "Frame luma (0–255): {}. Contrast below 8 means a nearly blank frame; \
white type on a white field will read near zero.",
            parts.join("; ")
        )
    }
}

/// Captures four stills of `html`. The work directory and browser processes
/// are cleaned up even if the run is cancelled.
pub async fn capture(html: &str, pass: usize) -> Result<CapturedFrames, String> {
    let chrome = export::chrome_binary().ok_or_else(|| {
        "Google Chrome was not found. Set ZEN_CHROME_BIN to its path.".to_string()
    })?;
    let ffmpeg = export::binary_or_env("ZEN_FFMPEG_BIN", "ffmpeg")
        .ok_or_else(|| "ffmpeg was not found. Set ZEN_FFMPEG_BIN to its path.".to_string())?;
    let xvfb = export::binary_or_env("ZEN_XVFB_BIN", "Xvfb")
        .ok_or_else(|| "Xvfb was not found; install xvfb to review rendered frames.".to_string())?;

    let stamp = unix_millis();
    let work = std::env::temp_dir().join(format!(
        "zen-design-frames-{}-{pass}-{stamp}",
        std::process::id()
    ));
    tokio::fs::create_dir_all(&work)
        .await
        .map_err(|error| format!("Could not create the frame workspace: {error}"))?;

    let mut guard = CaptureGuard {
        chrome: None,
        display: None,
        work: work.clone(),
    };

    let candidate = work.join("candidate.html");
    tokio::fs::write(&candidate, html)
        .await
        .map_err(|error| format!("Could not write the review candidate: {error}"))?;

    let candidate_url = format!("file://{}", candidate.display());
    let stage = work.join("stage.html");
    tokio::fs::write(&stage, stage_html(&candidate_url))
        .await
        .map_err(|error| format!("Could not write the review stage: {error}"))?;

    let profile = work.join("profile");
    tokio::fs::create_dir_all(&profile)
        .await
        .map_err(|error| format!("Could not prepare the review browser: {error}"))?;

    let (display, display_child) = export::start_display(&xvfb).await?;
    guard.display = Some(display_child);

    let stage_url = format!("file://{}", stage.display());
    let chrome_child = export::start_chrome(&chrome, &display, &profile, &stage_url)?;
    guard.chrome = Some(chrome_child);

    export::wait_for_browser(&display).await?;

    let video = work.join("review.mp4");
    export::record_display(&ffmpeg, &display, RECORD_SECONDS, &video, None).await?;

    let mut pngs = Vec::with_capacity(FRAME_TIMES.len());
    let mut stats = Vec::with_capacity(FRAME_TIMES.len());
    for (index, seconds) in FRAME_TIMES.iter().enumerate() {
        let png = work.join(format!("frame-{index}.png"));
        extract_frame(&ffmpeg, &video, *seconds, &png).await?;
        let bytes = tokio::fs::read(&png)
            .await
            .map_err(|error| format!("Could not read frame {index}: {error}"))?;
        if bytes.len() < 1_000 {
            return Err(format!("Frame {index} came out empty."));
        }
        stats.push(analyze_frame(&ffmpeg, &png).await?);
        pngs.push(bytes);
    }

    Ok(CapturedFrames {
        pngs,
        labels: FRAME_LABELS.iter().map(|label| label.to_string()).collect(),
        stats,
    })
}

fn stage_html(candidate_url: &str) -> String {
    let url = serde_json::to_string(candidate_url).unwrap_or_else(|_| "\"\"".to_string());
    format!(
        r#"<!doctype html>
<html lang="en">
<head><meta charset="utf-8"><title>ZenDesign review stage</title>
<style>html,body{{margin:0;height:100%;background:#000;overflow:hidden}}</style></head>
<body><script>
setTimeout(function () {{ location.replace({url}); }}, {STAGE_DELAY_MS});
</script></body></html>"#
    )
}

async fn extract_frame(
    ffmpeg: &str,
    video: &Path,
    seconds: f64,
    output: &Path,
) -> Result<(), String> {
    let seek = format!("{seconds:.2}");
    let result = Command::new(ffmpeg)
        .args([
            "-hide_banner",
            "-loglevel",
            "error",
            "-y",
            "-ss",
            &seek,
            "-i",
        ])
        .arg(video)
        .args(["-frames:v", "1", "-update", "1"])
        .arg(output)
        .stdin(Stdio::null())
        .stdout(Stdio::null())
        .stderr(Stdio::piped())
        .output()
        .await
        .map_err(|error| format!("Could not run ffmpeg for frame capture: {error}"))?;
    if result.status.success() {
        return Ok(());
    }
    let stderr = String::from_utf8_lossy(&result.stderr);
    Err(format!(
        "Could not extract a review frame: {}",
        stderr.trim().chars().take(300).collect::<String>()
    ))
}

/// Reduce the still to 144×90 grayscale and measure mean/stddev. Reducing
/// first stops one hairline from dominating the contrast reading, while still
/// catching a large title that has faded into the background.
async fn analyze_frame(ffmpeg: &str, png: &Path) -> Result<FrameStats, String> {
    let output = Command::new(ffmpeg)
        .args(["-v", "error", "-i"])
        .arg(png)
        .args(["-vf", "scale=144:90,format=gray", "-f", "rawvideo", "-"])
        .stdin(Stdio::null())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .output()
        .await
        .map_err(|error| format!("Could not analyze a review frame: {error}"))?;
    if !output.status.success() {
        return Err(format!(
            "Frame analysis failed: {}",
            String::from_utf8_lossy(&output.stderr).trim()
        ));
    }
    if output.stdout.len() < 1_000 {
        return Err("Frame analysis produced too few pixels.".to_string());
    }

    let pixels = &output.stdout;
    let mean = pixels.iter().map(|&pixel| pixel as f64).sum::<f64>() / pixels.len() as f64;
    let variance = pixels
        .iter()
        .map(|&pixel| {
            let delta = pixel as f64 - mean;
            delta * delta
        })
        .sum::<f64>()
        / pixels.len() as f64;

    Ok(FrameStats {
        mean,
        stddev: variance.sqrt(),
    })
}

struct CaptureGuard {
    chrome: Option<Child>,
    display: Option<Child>,
    work: PathBuf,
}

impl Drop for CaptureGuard {
    fn drop(&mut self) {
        if let Some(chrome) = self.chrome.as_mut() {
            let _ = chrome.start_kill();
        }
        if let Some(display) = self.display.as_mut() {
            let _ = display.start_kill();
        }
        let profile = self.work.join("profile");
        if profile.exists() {
            // The launcher exits quickly but leaves renderer children behind;
            // match them by the unique profile path before removing it.
            let _ = std::process::Command::new("pkill")
                .arg("-f")
                .arg(&profile)
                .status();
        }
        if let Err(error) = std::fs::remove_dir_all(&self.work) {
            warn!("Could not remove review workspace {:?}: {error}", self.work);
        }
    }
}

fn unix_millis() -> u128 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|duration| duration.as_millis())
        .unwrap_or_default()
}
