use tauri::{
    api::dialog::message, utils::assets::EmbeddedAssets, Context, CustomMenuItem, Menu, Submenu,
    WindowMenuEvent,
};

pub fn init(context: &Context<EmbeddedAssets>) -> Menu {
    // 获取应用名称
    // let name = &context.package_info().name;
    let menu = tauri::Menu::new();
    menu.add_submenu(Submenu::new(
        "菜单",
        Menu::new().add_item(CustomMenuItem::new("menu".to_string(), "菜单一")),
    ))
}

pub fn handler(event: WindowMenuEvent) {
    let win = Some(event.window());
    match event.menu_item_id() {
        "menu" => {
            message(win, "Eidt File", "TODO");
        }
        _ => {}
    }
}
