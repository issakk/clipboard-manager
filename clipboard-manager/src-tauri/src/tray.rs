use tauri::{AppHandle, Manager};

#[cfg(feature = "system-tray")]
use tauri::{
    CustomMenuItem, SystemTray, SystemTrayEvent, SystemTrayMenu,
    SystemTrayMenuItem,
};

use crate::database::Database;

#[cfg(feature = "system-tray")]
pub fn create_system_tray() -> SystemTray {
    let show = CustomMenuItem::new("show".to_string(), "显示主窗口");
    let quit = CustomMenuItem::new("quit".to_string(), "退出");
    let settings = CustomMenuItem::new("settings".to_string(), "设置");

    let tray_menu = SystemTrayMenu::new()
        .add_item(show)
        .add_native_item(SystemTrayMenuItem::Separator)
        .add_item(settings)
        .add_native_item(SystemTrayMenuItem::Separator)
        .add_item(quit);

    SystemTray::new().with_menu(tray_menu)
}

#[cfg(feature = "system-tray")]
pub fn handle_system_tray_event(app: &AppHandle, event: SystemTrayEvent) {
    match event {
        SystemTrayEvent::LeftClick { .. } => {
            // 左键点击显示主窗口
            let window = app.get_window("main").unwrap();
            window.show().unwrap();
            window.set_focus().unwrap();
        }
        SystemTrayEvent::MenuItemClick { id, .. } => match id.as_str() {
            "show" => {
                let window = app.get_window("main").unwrap();
                window.show().unwrap();
                window.set_focus().unwrap();
            }
            "settings" => {
                // 显示设置窗口
                let window = app.get_window("main").unwrap();
                window.show().unwrap();
                window.set_focus().unwrap();
                // 可以发送事件到前端切换到设置页面
                let _ = app.emit_all("show-settings", ());
            }
            "quit" => {
                std::process::exit(0);
            }
            _ => {}
        },
        _ => {}
    }
}

#[cfg(feature = "system-tray")]
pub fn update_recent_items_menu(app: &AppHandle) {
    let db = app.state::<Database>();

    // 获取最近 5 条记录
    if let Ok(items) = db.get_recent(5) {
        let mut menu = SystemTrayMenu::new();

        let show = CustomMenuItem::new("show".to_string(), "显示主窗口");
        menu = menu.add_item(show);

        if !items.is_empty() {
            menu = menu.add_native_item(SystemTrayMenuItem::Separator);

            for (i, item) in items.iter().enumerate() {
                let preview = item.preview.clone().unwrap_or_else(|| "无预览".to_string());
                let label = format!("recent_{}", i);
                let menu_item = CustomMenuItem::new(label, preview);
                menu = menu.add_item(menu_item);
            }
        }

        menu = menu.add_native_item(SystemTrayMenuItem::Separator);

        let settings = CustomMenuItem::new("settings".to_string(), "设置");
        let quit = CustomMenuItem::new("quit".to_string(), "退出");

        menu = menu.add_item(settings);
        menu = menu.add_native_item(SystemTrayMenuItem::Separator);
        menu = menu.add_item(quit);

        // 更新托盘菜单
        // 注意：Tauri 1 中需要通过 SystemTrayHandle 来更新菜单
        // 这里暂时不实现，后续可以通过事件来更新
    }
}
