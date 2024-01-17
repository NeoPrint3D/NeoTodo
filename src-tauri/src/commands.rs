use tauri::State;

use crate::db::{Database, Todo};

#[tauri::command]
pub fn get_todos(db: State<Database>) -> Result<Vec<Todo>, String> {
    let todos: Result<Vec<Todo>, String> = db.get_todos().map_err(|e| e.to_string());
    Ok(todos?)
}

#[tauri::command]
pub fn add_todo(
    db: State<Database>,
    title: String,
    description: Option<String>,
    due_date: Option<String>,
) -> Result<(), String> {
    println!("add_todo {:?}", due_date);
    db.add_todo(&title, description.as_deref(), due_date.as_deref())
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub fn update_todo(
    db: State<Database>,
    id: String,
    title: String,
    description: Option<String>,
    due_date: Option<String>,
    completed: bool,
) -> Result<(), String> {
    db.update_todo(
        &id,
        &title,
        description.as_deref(),
        due_date.as_deref(),
        completed,
    )
    .map_err(|e| e.to_string())
}

#[tauri::command]
pub fn delete_todo(db: State<Database>, id: String) -> Result<(), String> {
    db.delete_todo(&id).map_err(|e| e.to_string())
}
