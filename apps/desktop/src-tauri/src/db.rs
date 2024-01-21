use rusqlite::{named_params, Connection, Result};
use serde::Serialize;
use std::sync::Mutex;

use uuid::Uuid; // Import the uuid crate

#[derive(Debug, Serialize, Clone)]
pub struct Todo {
    id: String, // Change the type of id to Uuid
    created_at: String,
    title: String,
    description: Option<String>,
    due_date: Option<String>,
    completed: bool,
}

pub struct Database {
    conn: Mutex<Connection>,
}

impl Database {
    pub fn new() -> Result<Database> {
        let data_dir = tauri::api::path::data_dir().unwrap();
        let db_path = data_dir.join("./com.neotodo.dev/db.sqlite");

        let conn = Connection::open(db_path)?;
        conn.execute(
            "CREATE TABLE IF NOT EXISTS todo (
                id UUID PRIMARY KEY NOT NULL,
                created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
                due_date TIMESTAMP,
                title TEXT NOT NULL,
                description TEXT,
                completed BOOLEAN NOT NULL DEFAULT FALSE
            )",
            [],
        )?;
        Ok(Database {
            conn: Mutex::new(conn),
        })
    }

    pub fn get_todos(&self) -> Result<Vec<Todo>> {
        let conn = self.conn.lock().unwrap();
        let mut stmt = conn.prepare("SELECT * FROM todo")?;
        let rows = stmt.query_map([], |row| {
            Ok(Todo {
                id: row.get(0)?,
                created_at: row.get(1)?,
                due_date: row.get(2)?,
                title: row.get(3)?,
                description: row.get(4)?,
                completed: row.get(5)?,
            })
        })?;

        let mut todos = Vec::new();

        for todo in rows {
            todos.push(todo?);
        }

        Ok(todos)
    }

    pub fn add_todo(
        &self,
        title: &str,
        description: Option<&str>,
        due_date: Option<&str>,
    ) -> Result<()> {
        let conn = self.conn.lock().unwrap();
        let id = Uuid::new_v4(); // Generate a new UUID
        conn.execute(
            "INSERT INTO todo (id, title, description, due_date) VALUES (:id, :title, :description, :due_date)",
            named_params! {
                ":id": id.to_string(), // Convert UUID to string for database insertion
                ":title": title,
                ":description": description,
                ":due_date": due_date,
            },
        )?;
        Ok(())
    }
    pub fn update_todo(
        &self,
        id: &str,
        title: &str,
        description: Option<&str>,
        due_date: Option<&str>,
        completed: bool,
    ) -> Result<()> {
        let conn = self.conn.lock().unwrap();
        conn.execute(
            "UPDATE todo SET title = :title, description = :description, due_date = :due_date, completed = :completed WHERE id = :id",
            named_params! {
                ":id": id,
                ":title": title,
                ":description": description,
                ":due_date": due_date,
                ":completed": completed,
            },
        )?;
        Ok(())
    }

    pub fn delete_todo(&self, id: &str) -> Result<()> {
        let conn = self.conn.lock().unwrap();
        conn.execute(
            "DELETE FROM todo WHERE id = :id",
            named_params! {
                ":id": id,
            },
        )?;
        Ok(())
    }
}
