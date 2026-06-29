use tauri::AppHandle;

pub fn register_global_shortcut(_app: &AppHandle, _shortcut: &str) -> Result<(), String> {
    // Tauri 2 使用 tauri-plugin-global-shortcut
    // 暂时返回成功
    Ok(())
}

pub fn unregister_all_shortcuts(_app: &AppHandle) {
    // Tauri 2 使用 tauri-plugin-global-shortcut
}
