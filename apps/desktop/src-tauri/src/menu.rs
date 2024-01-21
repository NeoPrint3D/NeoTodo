use tauri::{CustomMenuItem, Menu, MenuItem, Submenu};

pub fn get_menu() -> Menu {
    Menu::new()
        .add_submenu(Submenu::new(
            "File",
            Menu::new().add_native_item(MenuItem::Quit),
        ))
        .add_submenu(Submenu::new(
            "Help",
            Menu::new()
                .add_item(CustomMenuItem::new("About NeoTodo", "About"))
                .add_native_item(MenuItem::Hide),
        ))
}
