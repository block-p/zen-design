//! Neural Text-to-Speech (TTS) engine.
//!
//! Synthesizes speech using Microsoft Edge Neural TTS voices:
//! * Persian (فارسی): `fa-IR-DilaraNeural` (default), `fa-IR-FaridNeural`
//! * English: `en-US-JennyNeural` (default), `en-US-GuyNeural`
//!
//! Provides ultra-high-quality, natural speech with 0 API keys and zero cost.
//! Thread-safe in-memory LRU-like cache prevents re-synthesizing repeated slides.

use std::collections::HashMap;
use std::sync::OnceLock;
use tokio::process::Command;
use tokio::sync::Mutex;
use tracing::{info, warn};

static AUDIO_CACHE: OnceLock<Mutex<HashMap<String, Vec<u8>>>> = OnceLock::new();

fn get_audio_cache() -> &'static Mutex<HashMap<String, Vec<u8>>> {
    AUDIO_CACHE.get_or_init(|| Mutex::new(HashMap::new()))
}

/// Detects if text contains Persian (Arabic script) characters.
pub fn is_persian_text(text: &str) -> bool {
    text.chars().any(|c| ('\u{0600}'..='\u{06FF}').contains(&c))
}

/// Selects the best neural voice for the provided text.
pub fn default_voice_for_text(text: &str) -> &'static str {
    if is_persian_text(text) {
        "fa-IR-DilaraNeural"
    } else {
        "en-US-JennyNeural"
    }
}

/// Generates MP3 audio bytes for the given text.
pub async fn synthesize(
    text: &str,
    voice_override: Option<&str>,
    rate_override: Option<&str>,
) -> Result<Vec<u8>, String> {
    let clean_text = text.trim();
    if clean_text.is_empty() {
        return Err("Cannot synthesize empty text".to_string());
    }

    let voice = voice_override
        .filter(|v| !v.trim().is_empty())
        .unwrap_or_else(|| default_voice_for_text(clean_text));

    let rate = rate_override
        .filter(|r| !r.trim().is_empty())
        .unwrap_or("+0%");

    // Cache key: voice + rate + text
    let cache_key = format!("{}:{}:{}", voice, rate, clean_text);
    {
        let cache = get_audio_cache().lock().await;
        if let Some(cached_bytes) = cache.get(&cache_key) {
            info!("TTS Cache hit for key '{}' ({} bytes)", voice, cached_bytes.len());
            return Ok(cached_bytes.clone());
        }
    }

    info!("Synthesizing TTS via edge-tts with voice '{}' ({} chars)…", voice, clean_text.chars().count());

    // Execute edge_tts Python script via tokio::process
    let py_script = r#"
import sys, asyncio, aiohttp, edge_tts

async def run():
    text = sys.stdin.read().strip()
    voice = sys.argv[1]
    rate = sys.argv[2]
    connector = aiohttp.TCPConnector(resolver=aiohttp.ThreadedResolver())
    communicate = edge_tts.Communicate(text, voice, rate=rate, connector=connector)
    async for chunk in communicate.stream():
        if chunk["type"] == "audio":
            sys.stdout.buffer.write(chunk["data"])

if __name__ == '__main__':
    asyncio.run(run())
"#;

    let mut child = Command::new("python3")
        .arg("-c")
        .arg(py_script)
        .arg(voice)
        .arg(rate)
        .stdin(std::process::Stdio::piped())
        .stdout(std::process::Stdio::piped())
        .stderr(std::process::Stdio::piped())
        .spawn()
        .map_err(|e| format!("Failed to spawn python3 for TTS: {e}"))?;

    if let Some(mut stdin) = child.stdin.take() {
        use tokio::io::AsyncWriteExt;
        let _ = stdin.write_all(clean_text.as_bytes()).await;
        let _ = stdin.flush().await;
    }

    let output = child
        .wait_with_output()
        .await
        .map_err(|e| format!("edge-tts process error: {e}"))?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        warn!("edge-tts failed with status {}: {}", output.status, stderr);
        return Err(format!("TTS synthesis failed: {stderr}"));
    }

    let audio_bytes = output.stdout;
    if audio_bytes.is_empty() {
        return Err("TTS synthesis produced zero bytes".to_string());
    }

    info!("TTS synthesis complete: {} bytes generated", audio_bytes.len());

    // Store in cache (limit to 100 entries to prevent memory leak)
    {
        let mut cache = get_audio_cache().lock().await;
        if cache.len() > 100 {
            cache.clear();
        }
        cache.insert(cache_key, audio_bytes.clone());
    }

    Ok(audio_bytes)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn detects_persian_text() {
        assert!(is_persian_text("سلام دنیا"));
        assert!(is_persian_text("معماری فنی سیستم ZenDesign"));
        assert!(!is_persian_text("Hello World 123"));
    }

    #[test]
    fn chooses_appropriate_voice() {
        assert_eq!(default_voice_for_text("توضیحات اسلاید"), "fa-IR-DilaraNeural");
        assert_eq!(default_voice_for_text("Executive Overview"), "en-US-JennyNeural");
    }
}
