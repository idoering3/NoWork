
use rusqlite::{Connection, Result};
use tauri::Manager;
use std::path::PathBuf;


pub fn get_db_path(app: tauri::AppHandle) -> PathBuf {
    let base_dir = app.path().app_data_dir().expect("Base directory not found!");
    std::fs::create_dir_all(&base_dir).expect("Failed to create data dir");
    base_dir.join("tasks.db")
}

pub fn init_db(app: tauri::AppHandle) -> Result<()> {
    let conn = Connection::open(get_db_path(app))?;
    conn.execute(
        "CREATE TABLE IF NOT EXISTS tasks (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            name TEXT NOT NULL,
            due_date DATE,
            created_at DATE DEFAULT CURRENT_DATE,
            completed BOOLEAN DEFAULT 0,
            completed_at DATE,
            tags TEXT
        )",
        [],
    )?;
    Ok(())
}

#[tauri::command]
pub fn add_database_task(app: tauri::AppHandle, name: String, due_date: Option<String>, tags: Option<String>) -> Result<(), String> {
    let conn = Connection::open(get_db_path(app)).map_err(|e| e.to_string())?;
    conn.execute(
        "INSERT INTO tasks (name, due_date, tags) VALUES (?1, ?2, ?3)",
        rusqlite::params![name, due_date, tags],
    ).map_err(|e| e.to_string())?;
    Ok(())
}
