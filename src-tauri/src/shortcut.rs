use tauri::{AppHandle, Manager, Runtime};
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

fn register_shortcut<R: Runtime>(app: &AppHandle<R>, shortcut_str: &str) -> tauri::Result<()> {
    let app_press = app.clone();
    let app_release = app.clone();

    app.global_shortcut()
        .on_shortcut(shortcut_str, move |_app, _shortcut, event| {
            let settings = crate::settings::load(&app_press);

            match event.state {
                ShortcutState::Pressed => {
                    if settings.push_to_talk {
                        // Push-to-talk: start recording on press
                        show_widget(&app_press);
                        if let Err(e) = crate::commands::start_recording_internal(&app_press) {
                            eprintln!("[shortcut] start_recording error: {e}");
                        }
                    } else {
                        // Toggle mode
                        if crate::commands::is_recording() {
                            let app = app_press.clone();
                            tauri::async_runtime::spawn(async move {
                                crate::commands::stop_and_transcribe_internal(app).await;
                            });
                        } else {
                            show_widget(&app_press);
                            if let Err(e) = crate::commands::start_recording_internal(&app_press) {
                                eprintln!("[shortcut] start_recording error: {e}");
                            }
                        }
                    }
                }
                ShortcutState::Released => {
                    if settings.push_to_talk && crate::commands::is_recording() {
                        let app = app_release.clone();
                        tauri::async_runtime::spawn(async move {
                            crate::commands::stop_and_transcribe_internal(app).await;
                        });
                    }
                }
            }
        })
        .map_err(|e| tauri::Error::Anyhow(anyhow::anyhow!("{}", e)))
}

pub fn register<R: Runtime>(app: &AppHandle<R>) -> tauri::Result<()> {
    let shortcut = crate::settings::load(app).shortcut;
    register_shortcut(app, &shortcut)
}

pub fn re_register<R: Runtime>(app: &AppHandle<R>, new_shortcut: &str) -> tauri::Result<()> {
    app.global_shortcut()
        .unregister_all()
        .map_err(|e| tauri::Error::Anyhow(anyhow::anyhow!("{}", e)))?;
    register_shortcut(app, new_shortcut)
}

fn show_widget<R: Runtime>(app: &AppHandle<R>) {
    if let Some(widget) = app.get_webview_window("widget") {
        // Position center-bottom above dock
        let settings = crate::settings::load(app);
        if let Ok(Some(monitor)) = widget.primary_monitor() {
            let size: tauri::PhysicalSize<u32> = *monitor.size();
            let scale: f64 = monitor.scale_factor();
            let sw = size.width as f64 / scale;
            let sh = size.height as f64 / scale;
            let ww = 300.0_f64;
            let wh = 52.0_f64;

            let x = match settings.widget_position.as_str() {
                "left"  => 40.0,
                "right" => sw - ww - 40.0,
                _       => (sw - ww) / 2.0, // center
            };
            let y = sh - wh - 100.0; // ~100px above dock

            widget
                .set_position(tauri::Position::Logical(tauri::LogicalPosition::new(x, y)))
                .ok();
        }
        widget.show().ok();
        // Do NOT steal focus — user is in another app
    }
}
