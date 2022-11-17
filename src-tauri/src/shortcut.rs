use tauri::{App, GlobalShortcutManager, Wry};

use crate::screenshot::screenshot;

pub fn init(app: &App<Wry>) {
    let mut shot_cut = app.global_shortcut_manager();
    shot_cut
        .register("ctrl+z", screenshot)
        .expect("register fail")
}
