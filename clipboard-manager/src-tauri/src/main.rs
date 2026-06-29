// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod clipboard_monitor;
mod database;
mod search;
mod commands;

use tauri::Manager;

fn main() {
    tauri::Builder::default()
        .plugin(tauri_plugin_clipboard_manager::init())
        .plugin(tauri_plugin_global_shortcut::Builder::new().build())
        .setup(|app| {
            // 获取数据库路径
            let db_path = commands::get_db_path().unwrap_or_else(|_| {
                let data_dir = dirs_next::data_dir()
                    .unwrap_or_else(|| std::path::PathBuf::from("."))
                    .join("clipboard-manager");
                let _ = std::fs::create_dir_all(&data_dir);
                data_dir.join("clipboard.db").to_string_lossy().to_string()
            });

            // 初始化数据库
            let db = database::Database::new_with_path(&db_path)?;
            app.manage(db);

            // 启动剪切板监控
            let handle = app.handle().clone();
            std::thread::spawn(move || {
                clipboard_monitor::start_monitoring(handle);
            });

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            commands::get_clipboard_history,
            commands::search_clipboard,
            commands::delete_clipboard_item,
            commands::toggle_favorite,
            commands::get_settings,
            commands::update_settings,
            commands::clear_history,
            commands::copy_to_clipboard,
            commands::get_db_path,
            commands::select_db_path,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
