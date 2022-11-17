#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use tauri::SystemTray;

mod menu;
mod screenshot;
mod shortcut;
mod tray;

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

fn main() {
    let context = tauri::generate_context!();
    tauri::Builder::default()
        .setup(|app| {
            shortcut::init(app);
            Ok(())
        })
        .menu(menu::init(&context))
        .on_menu_event(menu::handler)
        .system_tray(SystemTray::new().with_menu(tray::init()))
        .on_system_tray_event(tray::handler)
        .invoke_handler(tauri::generate_handler![greet])
        .run(context)
        .expect("error while running tauri application");
}
