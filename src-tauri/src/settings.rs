use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;
use tauri::{AppHandle, Manager, Runtime};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppSettings {
    pub shortcut: String,
    pub push_to_talk: bool,
    pub selected_language: String,
    pub selected_model: String,
    pub autostart: bool,
    pub audio_feedback: bool,
    pub onboarding_done: bool,
    pub widget_position: String, // "center" | "left" | "right"
}

impl Default for AppSettings {
    fn default() -> Self {
        Self {
            shortcut: "Alt+Space".to_string(),
            push_to_talk: true,
            selected_language: "auto".to_string(),
            selected_model: "large-v3-turbo".to_string(),
            autostart: false,
            audio_feedback: true,
            onboarding_done: false,
            widget_position: "center".to_string(),
        }
    }
}

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
    let path = settings_path(app);
    fs::read_to_string(&path)
        .ok()
        .and_then(|s| serde_json::from_str(&s).ok())
        .unwrap_or_default()
}

pub fn save<R: Runtime>(app: &AppHandle<R>, settings: &AppSettings) -> tauri::Result<()> {
    let path = settings_path(app);
    let json = serde_json::to_string_pretty(settings).unwrap();
    fs::write(path, json).map_err(|e| tauri::Error::Anyhow(e.into()))
}
