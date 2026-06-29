use std::thread;
use std::time::Duration;
use tauri::{AppHandle, Manager};
use uuid::Uuid;
use chrono::Utc;

use crate::database::{ClipboardItem, Database};

pub fn start_monitoring(app: AppHandle) {
    let db = app.state::<Database>();
    let mut last_content = String::new();

    loop {
        thread::sleep(Duration::from_millis(300));

        // 读取剪切板内容
        let current_content = read_clipboard_content();

        if current_content.is_empty() || current_content == last_content {
            continue;
        }

        last_content = current_content.clone();

        // 生成 UUID v7
        let id = Uuid::now_v7().to_string();

        // 获取设备 ID
        let device_id = get_device_id();

        // 创建记录
        let item = ClipboardItem {
            id,
            content_type: "text".to_string(),
            content: current_content.clone(),
            preview: Some(truncate_text(&current_content, 100)),
            created_at: Utc::now().timestamp(),
            is_favorite: false,
            device_id,
            is_deleted: false,
        };

        // 插入数据库
        if let Err(e) = db.insert(&item) {
            eprintln!("Failed to insert clipboard item: {}", e);
            continue;
        }

        // 通知前端
        let _ = app.emit_all("clipboard-changed", &item);
    }
}

fn read_clipboard_content() -> String {
    #[cfg(target_os = "windows")]
    {
        read_windows_clipboard()
    }

    #[cfg(not(target_os = "windows"))]
    {
        // Linux/macOS 使用 arboard crate
        read_cross_platform_clipboard()
    }
}

#[cfg(target_os = "windows")]
fn read_windows_clipboard() -> String {
    use std::ptr::null_mut;

    unsafe {
        use winapi::um::winuser::{OpenClipboard, GetClipboardData, CloseClipboard, CF_UNICODETEXT};
        use winapi::um::winbase::GlobalLock;
        use winapi::um::winbase::GlobalUnlock;

        if OpenClipboard(null_mut()) == 0 {
            return String::new();
        }

        let handle = GetClipboardData(CF_UNICODETEXT);
        if handle.is_null() {
            CloseClipboard();
            return String::new();
        }

        let ptr = GlobalLock(handle) as *const u16;
        if ptr.is_null() {
            CloseClipboard();
            return String::new();
        }

        let mut len = 0;
        while *ptr.add(len) != 0 {
            len += 1;
        }

        let slice = std::slice::from_raw_parts(ptr, len);
        let content = String::from_utf16_lossy(slice);

        GlobalUnlock(handle);
        CloseClipboard();

        content
    }
}

#[cfg(not(target_os = "windows"))]
fn read_cross_platform_clipboard() -> String {
    // 在 Linux 上使用 arboard 或 xclip
    // 这里返回空字符串作为占位
    String::new()
}

fn truncate_text(text: &str, max_len: usize) -> String {
    if text.len() <= max_len {
        text.to_string()
    } else {
        format!("{}...", &text[..max_len])
    }
}

fn get_device_id() -> String {
    // 生成或读取设备唯一标识
    use std::fs;
    use std::path::Path;

    let config_dir = dirs_next::config_dir()
        .unwrap_or_else(|| Path::new(".").to_path_buf())
        .join("clipboard-manager");

    let id_file = config_dir.join("device_id");

    if let Ok(id) = fs::read_to_string(&id_file) {
        return id.trim().to_string();
    }

    // 生成新的设备 ID
    let id = Uuid::now_v7().to_string();
    let _ = fs::create_dir_all(&config_dir);
    let _ = fs::write(&id_file, &id);

    id
}

/// 写入内容到剪切板
pub fn write_to_clipboard(content: &str) -> Result<(), String> {
    #[cfg(target_os = "windows")]
    {
        write_windows_clipboard(content)
    }

    #[cfg(not(target_os = "windows"))]
    {
        Err("Clipboard write not supported on this platform".to_string())
    }
}

#[cfg(target_os = "windows")]
fn write_windows_clipboard(content: &str) -> Result<(), String> {
    use std::ptr::null_mut;

    unsafe {
        use winapi::um::winuser::{OpenClipboard, EmptyClipboard, SetClipboardData, CloseClipboard};
        use winapi::um::winbase::{GlobalAlloc, GlobalLock, GlobalUnlock, GMEM_MOVEABLE};

        if OpenClipboard(null_mut()) == 0 {
            return Err("Failed to open clipboard".to_string());
        }

        EmptyClipboard();

        let content_utf16: Vec<u16> = content.encode_utf16().chain(std::iter::once(0)).collect();
        let size = content_utf16.len() * 2;

        let h_mem = GlobalAlloc(GMEM_MOVEABLE, size);
        if h_mem.is_null() {
            CloseClipboard();
            return Err("Failed to allocate memory".to_string());
        }

        let ptr = GlobalLock(h_mem) as *mut u16;
        if ptr.is_null() {
            CloseClipboard();
            return Err("Failed to lock memory".to_string());
        }

        std::ptr::copy_nonoverlapping(content_utf16.as_ptr(), ptr, content_utf16.len());
        GlobalUnlock(h_mem);

        SetClipboardData(1, h_mem); // CF_UNICODETEXT = 1
        CloseClipboard();

        Ok(())
    }
}
