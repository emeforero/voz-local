use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;
use std::sync::Mutex;
use tauri::{AppHandle, Manager, Runtime};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppSettings {
    pub shortcut: String,
    pub push_to_talk: bool,
    pub selected_language: String,
    pub selected_model: String,
    pub autostart: bool,
    pub onboarding_done: bool,
    pub widget_position: String, // "center" | "left" | "right"
    /// Passed to Whisper as initial_prompt to bias recognition toward these terms.
    /// Comma or newline separated, e.g. "GitHub, Claude Code, Node.js, TypeScript"
    #[serde(default)]
    pub custom_words: String,
    /// Jaro-Winkler similarity threshold for post-transcription word correction (0.0–1.0).
    /// 0.85 catches obvious typos without false positives.
    #[serde(default = "default_word_correction_threshold")]
    pub word_correction_threshold: f32,
}

fn default_word_correction_threshold() -> f32 { 0.85 }

impl Default for AppSettings {
    fn default() -> Self {
        Self {
            shortcut: "Alt+Space".to_string(),
            push_to_talk: true,
            selected_language: "auto".to_string(),
            selected_model: "large-v3-turbo".to_string(),
            autostart: false,
            onboarding_done: false,
            widget_position: "center".to_string(),
            custom_words: String::new(),
            word_correction_threshold: default_word_correction_threshold(),
        }
    }
}

// In-process cache so shortcut handlers don't hit disk on every key event.
static CACHE: Mutex<Option<AppSettings>> = Mutex::new(None);

fn settings_path<R: Runtime>(app: &AppHandle<R>) -> PathBuf {
    app.path()
        .app_data_dir()
        .expect("no app data dir")
        .join("settings.json")
}

pub fn init<R: Runtime>(app: &AppHandle<R>) -> tauri::Result<()> {
    let path = settings_path(app);
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).ok();
    }
    if !path.exists() {
        let json = serde_json::to_string_pretty(&AppSettings::default()).unwrap();
        fs::write(&path, json).ok();
    }
    Ok(())
}

pub fn load<R: Runtime>(app: &AppHandle<R>) -> AppSettings {
    if let Some(cached) = CACHE.lock().unwrap().as_ref() {
        return cached.clone();
    }
    let path = settings_path(app);
    let settings: AppSettings = fs::read_to_string(&path)
        .ok()
        .and_then(|s| serde_json::from_str(&s).ok())
        .unwrap_or_default();
    *CACHE.lock().unwrap() = Some(settings.clone());
    settings
}

pub fn save<R: Runtime>(app: &AppHandle<R>, settings: &AppSettings) -> tauri::Result<()> {
    let path = settings_path(app);
    let json = serde_json::to_string_pretty(settings).unwrap();
    fs::write(path, json).map_err(|e| tauri::Error::Anyhow(e.into()))?;
    *CACHE.lock().unwrap() = Some(settings.clone());
    Ok(())
}
