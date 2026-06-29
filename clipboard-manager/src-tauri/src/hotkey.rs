use tauri::{AppHandle, GlobalShortcutManager, Manager};

pub fn register_global_shortcut(app: &AppHandle, shortcut: &str) -> Result<(), String> {
    let mut shortcut_manager = app.global_shortcut_manager();

    // 先取消之前的注册
    let _ = shortcut_manager.unregister_all();

    // 注册新的快捷键
    let app_handle = app.clone();
    shortcut_manager
        .register(shortcut, move || {
            toggle_window(&app_handle);
        })
        .map_err(|e| format!("Failed to register shortcut: {}", e))?;

    Ok(())
}

fn toggle_window(app: &AppHandle) {
    if let Some(window) = app.get_window("main") {
        if window.is_visible().unwrap_or(false) {
            let _ = window.hide();
        } else {
            let _ = window.show();
            let _ = window.set_focus();
        }
    }
}

pub fn unregister_all_shortcuts(app: &AppHandle) {
    let mut shortcut_manager = app.global_shortcut_manager();
    let _ = shortcut_manager.unregister_all();
}
