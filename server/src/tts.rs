//! Multi-Provider Neural Text-to-Speech (TTS) engine.
//!
//! Supported Engines:
//! 1. `edge` (Default, 0 keys needed): Microsoft Edge Neural voices (`fa-IR-DilaraNeural`, `fa-IR-FaridNeural`, `en-US-JennyNeural`, `en-US-GuyNeural`)
//! 2. `elevenlabs`: ElevenLabs Multilingual v2 (State of the art emotional speech synthesis)
//! 3. `openai`: OpenAI TTS-1 / TTS-1-HD (`alloy`, `echo`, `fable`, `onyx`, `nova`, `shimmer`)
//! 4. `gemini`: Google Gemini 2.0 Native Audio generation (`Puck`, `Charon`, `Kore`, `Fenrir`, `Aoede`)
//!
//! Provides ultra-high-quality, natural speech with automatic fallback to Edge Neural.
//! Thread-safe in-memory cache prevents re-synthesizing repeated slides.

use std::collections::HashMap;
use std::sync::OnceLock;
use base64::Engine;
use reqwest::Client;
use serde_json::json;
use tokio::process::Command;
use tokio::sync::Mutex;
use tracing::{info, warn};

static AUDIO_CACHE: OnceLock<Mutex<HashMap<String, Vec<u8>>>> = OnceLock::new();
static HTTP_CLIENT: OnceLock<Client> = OnceLock::new();

fn get_audio_cache() -> &'static Mutex<HashMap<String, Vec<u8>>> {
    AUDIO_CACHE.get_or_init(|| Mutex::new(HashMap::new()))
}

fn get_http_client() -> &'static Client {
    HTTP_CLIENT.get_or_init(|| {
        Client::builder()
            .timeout(std::time::Duration::from_secs(30))
            .build()
            .unwrap_or_default()
    })
}

/// Detects if text contains Persian (Arabic script) characters.
pub fn is_persian_text(text: &str) -> bool {
    text.chars().any(|c| ('\u{0600}'..='\u{06FF}').contains(&c))
}

/// Selects the best neural voice for the provided text.
pub fn default_voice_for_text(text: &str, provider: &str) -> &'static str {
    match provider {
        "openai" => {
            if is_persian_text(text) {
                "nova"
            } else {
                "alloy"
            }
        }
        "gemini" => "Puck",
        "elevenlabs" => "21m00Tcm4TlvDq8ikWAM", // Rachel / default multilingual
        _ => {
            if is_persian_text(text) {
                "fa-IR-DilaraNeural"
            } else {
                "en-US-JennyNeural"
            }
        }
    }
}

/// Synthesize using Microsoft Edge Neural TTS (Free, 0 keys needed, high quality Persian).
async fn synthesize_edge(text: &str, voice: &str, rate: &str) -> Result<Vec<u8>, String> {
    info!("Synthesizing TTS via edge-tts with voice '{}' ({} chars)…", voice, text.chars().count());

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

    let python_cmd = if cfg!(windows) { "python" } else { "python3" };

    let mut child = Command::new(python_cmd)
        .arg("-c")
        .arg(py_script)
        .arg(voice)
        .arg(rate)
        .stdin(std::process::Stdio::piped())
        .stdout(std::process::Stdio::piped())
        .stderr(std::process::Stdio::piped())
        .spawn()
        .map_err(|e| format!("Failed to spawn {python_cmd} for TTS: {e}"))?;

    if let Some(mut stdin) = child.stdin.take() {
        use tokio::io::AsyncWriteExt;
        let _ = stdin.write_all(text.as_bytes()).await;
        let _ = stdin.flush().await;
    }

    let output = child
        .wait_with_output()
        .await
        .map_err(|e| format!("edge-tts process error: {e}"))?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        warn!("edge-tts failed with status {}: {}", output.status, stderr);
        return Err(format!("Edge TTS synthesis failed: {stderr}"));
    }

    let audio_bytes = output.stdout;
    if audio_bytes.is_empty() {
        return Err("Edge TTS synthesis produced zero bytes".to_string());
    }

    Ok(audio_bytes)
}

/// Synthesize using ElevenLabs Multilingual v2 API.
async fn synthesize_elevenlabs(text: &str, voice: &str, api_key: &str) -> Result<Vec<u8>, String> {
    let voice_id = if voice.trim().is_empty() {
        "21m00Tcm4TlvDq8ikWAM"
    } else {
        voice
    };

    info!("Synthesizing TTS via ElevenLabs with voice '{}'…", voice_id);

    let url = format!("https://api.elevenlabs.io/v1/text-to-speech/{}", voice_id);
    let payload = json!({
        "text": text,
        "model_id": "eleven_multilingual_v2",
        "voice_settings": {
            "stability": 0.5,
            "similarity_boost": 0.75
        }
    });

    let res = get_http_client()
        .post(&url)
        .header("xi-api-key", api_key)
        .header("Content-Type", "application/json")
        .header("Accept", "audio/mpeg")
        .json(&payload)
        .send()
        .await
        .map_err(|e| format!("ElevenLabs request failed: {e}"))?;

    if !res.status().is_success() {
        let err_body = res.text().await.unwrap_or_default();
        return Err(format!("ElevenLabs returned error: {err_body}"));
    }

    let bytes = res.bytes().await.map_err(|e| format!("Failed to read ElevenLabs audio bytes: {e}"))?;
    Ok(bytes.to_vec())
}

/// Synthesize using OpenAI TTS-1 / TTS-1-HD.
async fn synthesize_openai(text: &str, voice: &str, api_key: &str) -> Result<Vec<u8>, String> {
    let clean_voice = if voice.trim().is_empty() {
        if is_persian_text(text) { "nova" } else { "alloy" }
    } else {
        voice
    };

    info!("Synthesizing TTS via OpenAI TTS with voice '{}'…", clean_voice);

    let url = "https://api.openai.com/v1/audio/speech";
    let payload = json!({
        "model": "tts-1-hd",
        "input": text,
        "voice": clean_voice
    });

    let res = get_http_client()
        .post(url)
        .header("Authorization", format!("Bearer {}", api_key))
        .header("Content-Type", "application/json")
        .json(&payload)
        .send()
        .await
        .map_err(|e| format!("OpenAI TTS request failed: {e}"))?;

    if !res.status().is_success() {
        let err_body = res.text().await.unwrap_or_default();
        return Err(format!("OpenAI TTS returned error: {err_body}"));
    }

    let bytes = res.bytes().await.map_err(|e| format!("Failed to read OpenAI audio bytes: {e}"))?;
    Ok(bytes.to_vec())
}

/// Synthesize using Google Gemini 2.0 Native Audio Output.
async fn synthesize_gemini(text: &str, voice: &str, api_key: &str) -> Result<Vec<u8>, String> {
    let clean_voice = if voice.trim().is_empty() { "Puck" } else { voice };

    info!("Synthesizing TTS via Gemini Audio with voice '{}'…", clean_voice);

    let url = format!(
        "https://generativelanguage.googleapis.com/v1beta/models/gemini-2.0-flash:generateContent?key={}",
        api_key
    );

    let prompt_msg = format!("Please speak the following text out loud with a clear, engaging tone:\n\n{text}");
    let payload = json!({
        "contents": [{
            "parts": [{ "text": prompt_msg }]
        }],
        "generationConfig": {
            "responseModalities": ["AUDIO"],
            "speechConfig": {
                "voiceConfig": {
                    "prebuiltVoiceConfig": {
                        "voiceName": clean_voice
                    }
                }
            }
        }
    });

    let res = get_http_client()
        .post(&url)
        .header("Content-Type", "application/json")
        .json(&payload)
        .send()
        .await
        .map_err(|e| format!("Gemini Audio request failed: {e}"))?;

    if !res.status().is_success() {
        let err_body = res.text().await.unwrap_or_default();
        return Err(format!("Gemini Audio returned error: {err_body}"));
    }

    let json_resp: serde_json::Value = res.json().await.map_err(|e| format!("Failed to parse Gemini json: {e}"))?;
    
    // Extract base64 inline audio data
    let base64_audio = json_resp
        .pointer("/candidates/0/content/parts/0/inlineData/data")
        .and_then(|v| v.as_str())
        .ok_or_else(|| "No inline audio data found in Gemini response".to_string())?;

    let audio_bytes = base64::engine::general_purpose::STANDARD
        .decode(base64_audio)
        .map_err(|e| format!("Failed to decode Gemini audio base64: {e}"))?;

    Ok(audio_bytes)
}

/// Generates MP3 audio bytes for the given text with multi-engine routing and caching.
pub async fn synthesize(
    provider_override: Option<&str>,
    text: &str,
    voice_override: Option<&str>,
    rate_override: Option<&str>,
    api_key_override: Option<&str>,
) -> Result<Vec<u8>, String> {
    let clean_text = text.trim();
    if clean_text.is_empty() {
        return Err("Cannot synthesize empty text".to_string());
    }

    let provider = provider_override
        .filter(|p| !p.trim().is_empty())
        .unwrap_or("edge");

    let voice = voice_override
        .filter(|v| !v.trim().is_empty())
        .unwrap_or_else(|| default_voice_for_text(clean_text, provider));

    let rate = rate_override
        .filter(|r| !r.trim().is_empty())
        .unwrap_or("+0%");

    // Cache key: provider + voice + rate + text
    let cache_key = format!("{}:{}:{}:{}", provider, voice, rate, clean_text);
    {
        let cache = get_audio_cache().lock().await;
        if let Some(cached_bytes) = cache.get(&cache_key) {
            info!("TTS Cache hit for '{}:{}' ({} bytes)", provider, voice, cached_bytes.len());
            return Ok(cached_bytes.clone());
        }
    }

    // Attempt generation with selected provider, with automatic fallback to Edge Neural
    let result = match provider {
        "elevenlabs" => {
            if let Some(key) = api_key_override.filter(|k| !k.trim().is_empty()) {
                match synthesize_elevenlabs(clean_text, voice, key).await {
                    Ok(b) => Ok(b),
                    Err(e) => {
                        warn!("ElevenLabs failed ({e}), falling back to Edge Neural…");
                        let fallback_voice = default_voice_for_text(clean_text, "edge");
                        synthesize_edge(clean_text, fallback_voice, rate).await
                    }
                }
            } else {
                warn!("No ElevenLabs API key provided, falling back to Edge Neural…");
                let fallback_voice = default_voice_for_text(clean_text, "edge");
                synthesize_edge(clean_text, fallback_voice, rate).await
            }
        }
        "openai" => {
            if let Some(key) = api_key_override.filter(|k| !k.trim().is_empty()) {
                match synthesize_openai(clean_text, voice, key).await {
                    Ok(b) => Ok(b),
                    Err(e) => {
                        warn!("OpenAI TTS failed ({e}), falling back to Edge Neural…");
                        let fallback_voice = default_voice_for_text(clean_text, "edge");
                        synthesize_edge(clean_text, fallback_voice, rate).await
                    }
                }
            } else {
                warn!("No OpenAI API key provided, falling back to Edge Neural…");
                let fallback_voice = default_voice_for_text(clean_text, "edge");
                synthesize_edge(clean_text, fallback_voice, rate).await
            }
        }
        "gemini" => {
            if let Some(key) = api_key_override.filter(|k| !k.trim().is_empty()) {
                match synthesize_gemini(clean_text, voice, key).await {
                    Ok(b) => Ok(b),
                    Err(e) => {
                        warn!("Gemini Audio failed ({e}), falling back to Edge Neural…");
                        let fallback_voice = default_voice_for_text(clean_text, "edge");
                        synthesize_edge(clean_text, fallback_voice, rate).await
                    }
                }
            } else {
                warn!("No Gemini API key provided, falling back to Edge Neural…");
                let fallback_voice = default_voice_for_text(clean_text, "edge");
                synthesize_edge(clean_text, fallback_voice, rate).await
            }
        }
        _ => synthesize_edge(clean_text, voice, rate).await,
    };

    let audio_bytes = result?;

    info!("TTS synthesis complete: {} bytes generated (provider: {})", audio_bytes.len(), provider);

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
        assert_eq!(default_voice_for_text("توضیحات اسلاید", "edge"), "fa-IR-DilaraNeural");
        assert_eq!(default_voice_for_text("Executive Overview", "edge"), "en-US-JennyNeural");
        assert_eq!(default_voice_for_text("سلام", "openai"), "nova");
        assert_eq!(default_voice_for_text("Hello", "openai"), "alloy");
    }
}
