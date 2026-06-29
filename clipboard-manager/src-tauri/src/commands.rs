use tauri::State;
use serde::{Deserialize, Serialize};

use crate::database::{ClipboardItem, Database};
use crate::search::SearchEngine;

#[derive(Debug, Serialize, Deserialize)]
pub struct SearchParams {
    pub query: Option<String>,
    pub content_type: Option<String>,
    pub limit: Option<i64>,
    pub offset: Option<i64>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Settings {
    pub max_records: i64,
    pub shortcut: String,
    pub auto_start: bool,
    pub db_path: String,
}

#[tauri::command]
pub fn get_clipboard_history(
    db: State<'_, Database>,
    limit: Option<i64>,
    offset: Option<i64>,
) -> Result<Vec<ClipboardItem>, String> {
    let limit = limit.unwrap_or(50);
    let offset = offset.unwrap_or(0);

    db.get_history(limit, offset)
        .map_err(|e| format!("Failed to get history: {}", e))
}

#[tauri::command]
pub fn search_clipboard(
    db: State<'_, Database>,
    params: SearchParams,
) -> Result<Vec<ClipboardItem>, String> {
    let search_engine = SearchEngine::new(&db);
    let limit = params.limit.unwrap_or(50);
    let offset = params.offset.unwrap_or(0);

    search_engine
        .search(
            params.query.as_deref().unwrap_or(""),
            params.content_type.as_deref(),
            limit,
            offset,
        )
        .map_err(|e| format!("Search failed: {}", e))
}

#[tauri::command]
pub fn delete_clipboard_item(
    db: State<'_, Database>,
    id: String,
) -> Result<(), String> {
    db.delete(&id)
        .map_err(|e| format!("Failed to delete item: {}", e))
}

#[tauri::command]
pub fn toggle_favorite(
    db: State<'_, Database>,
    id: String,
) -> Result<bool, String> {
    db.toggle_favorite(&id)
        .map_err(|e| format!("Failed to toggle favorite: {}", e))
}

#[tauri::command]
pub fn get_settings() -> Result<Settings, String> {
    let config = load_config();
    Ok(config)
}

#[tauri::command]
pub fn update_settings(settings: Settings) -> Result<(), String> {
    save_config(&settings)
}

#[tauri::command]
pub fn clear_history(db: State<'_, Database>) -> Result<(), String> {
    db.clear_history()
        .map_err(|e| format!("Failed to clear history: {}", e))
}

#[tauri::command]
pub async fn copy_to_clipboard(content: String) -> Result<(), String> {
    // 使用 tauri-plugin-clipboard-manager
    // 这里简化处理，实际应该调用插件
    Ok(())
}

#[tauri::command]
pub fn get_db_path() -> Result<String, String> {
    let config = load_config();
    Ok(config.db_path)
}

#[tauri::command]
pub async fn select_db_path(app: tauri::AppHandle) -> Result<String, String> {
    use tauri::api::dialog::blocking::FileDialogBuilder;

    let result = FileDialogBuilder::new()
        .set_title("选择数据库保存位置")
        .set_file_name("clipboard.db")
        .add_filter("SQLite Database", &["db"])
        .save_file();

    match result {
        Some(path) => {
            let path_str = path.to_string_lossy().to_string();
            let mut config = load_config();
            config.db_path = path_str.clone();
            save_config(&config)?;
            Ok(path_str)
        }
        None => Err("未选择路径".to_string()),
    }
}

fn get_config_path() -> std::path::PathBuf {
    let config_dir = dirs_next::config_dir()
        .unwrap_or_else(|| std::path::PathBuf::from("."))
        .join("clipboard-manager");

    let _ = std::fs::create_dir_all(&config_dir);
    config_dir.join("config.json")
}

fn load_config() -> Settings {
    let config_path = get_config_path();

    if let Ok(content) = std::fs::read_to_string(&config_path) {
        if let Ok(config) = serde_json::from_str(&content) {
            return config;
        }
    }

    // 默认配置
    Settings {
        max_records: 10000,
        shortcut: "Ctrl+W".to_string(),
        auto_start: false,
        db_path: get_default_db_path(),
    }
}

fn save_config(config: &Settings) -> Result<(), String> {
    let config_path = get_config_path();
    let content = serde_json::to_string_pretty(config)
        .map_err(|e| format!("Failed to serialize config: {}", e))?;

    std::fs::write(&config_path, content)
        .map_err(|e| format!("Failed to write config: {}", e))?;

    Ok(())
}

fn get_default_db_path() -> String {
    let data_dir = dirs_next::data_dir()
        .unwrap_or_else(|| std::path::PathBuf::from("."))
        .join("clipboard-manager");

    let _ = std::fs::create_dir_all(&data_dir);
    data_dir.join("clipboard.db").to_string_lossy().to_string()
}
