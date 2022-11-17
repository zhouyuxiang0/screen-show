use tauri::{
    api::dialog::message, AppHandle, CustomMenuItem, Manager, SystemTrayEvent, SystemTrayMenu,
};

pub fn init() -> SystemTrayMenu {
    SystemTrayMenu::new()
        .add_item(CustomMenuItem::new("quit".to_string(), "Quit"))
        .add_item(CustomMenuItem::new("close".to_string(), "Close"))
}

pub fn handler(app: &AppHandle, event: SystemTrayEvent) {
    let window = app.get_window("main").unwrap();
    let parent_window = Some(&window);
    match event {
        SystemTrayEvent::LeftClick {
            tray_id,
            position,
            size,
            ..
        } => {
            println!("left click");
        }
        SystemTrayEvent::RightClick {
            tray_id,
            position,
            size,
            ..
        } => {
            println!("right click");
        }
        SystemTrayEvent::DoubleClick {
            tray_id,
            position,
            size,
            ..
        } => {
            println!("double click");
        }
        SystemTrayEvent::MenuItemClick { tray_id, id, .. } => match id.as_str() {
            "close" => {
                message(parent_window, "close", "close...");
            }
            _ => {}
        },
        _ => {}
    }
}
