mod commands;
mod db;
mod menu;

use db::Database;

use std::process::Command;
fn main() {
    let db = Database::new().expect("error initializing database");
    tauri::Builder::default()
        .manage(db)
        .invoke_handler(tauri::generate_handler![
            commands::get_todos,
            commands::add_todo,
            commands::update_todo,
            commands::delete_todo,
        ])
        .menu(menu::get_menu())
        .on_menu_event(|event| match event.menu_item_id() {
            "About NeoTodo" => {
                println!("About NeoTodo");
                Command::new("open")
                    .arg("https://github.com/NeoPrint3d/NeoTodo")
                    .spawn()
                    .expect("failed to open browser");
            }
            _ => {}
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
