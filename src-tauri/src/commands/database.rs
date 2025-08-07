
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

#[derive(serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Task {
    id: i32,
    name: String,
    due_date: Option<String>,
    created_at: String,
    completed: bool,
    completed_at: Option<String>,
    tags: Option<String>,
}

#[tauri::command]
pub fn get_all_tasks(app: tauri::AppHandle) -> Result<Vec<Task>, String> {
    let conn = Connection::open(get_db_path(app)).map_err(|e| e.to_string())?;
    let mut stmt = conn.prepare("SELECT id, name, due_date, created_at, completed, completed_at, tags FROM tasks")
        .map_err(|e| e.to_string())?;

    let tasks_iter = stmt.query_map([], |row| {
        Ok(Task {
            id: row.get(0)?,
            name: row.get(1)?,
            due_date: row.get(2)?,
            created_at: row.get(3)?,
            completed: row.get(4)?,
            completed_at: row.get(5)?,
            tags: row.get(6)?,
        })
    }).map_err(|e| e.to_string())?;

    let mut tasks = Vec::new();
    for task in tasks_iter {
        tasks.push(task.map_err(|e| e.to_string())?);
    }

    Ok(tasks)
}

#[tauri::command]
pub fn complete_task(app: tauri::AppHandle, task_id: i32) -> Result<(), String> {
    let conn = Connection::open(get_db_path(app)).map_err(|e| e.to_string())?;
    conn.execute(
        "UPDATE tasks SET completed = 1, completed_at = CURRENT_DATE WHERE id = ?1",
        rusqlite::params![task_id],
    ).map_err(|e| e.to_string())?;
    Ok(())
}

#[tauri::command]
pub fn delete_task(app: tauri::AppHandle, task_id: i32) -> Result<(), String> {
    let conn = Connection::open(get_db_path(app)).map_err(|e| e.to_string())?;
    conn.execute(
        "DELETE FROM tasks WHERE id = ?1",
        rusqlite::params![task_id],
    ).map_err(|e| e.to_string())?;
    Ok(())
}
