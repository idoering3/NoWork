use serde::{Serialize, Deserialize};
use chrono::{DateTime, Utc};
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
            due_date TEXT,
            created_at TEXT DEFAULT (datetime('now')),
            completed BOOLEAN DEFAULT 0,
            completed_at TEXT,
            tags TEXT
        )",
        [],
    )?;
    Ok(())
}

#[tauri::command]
pub fn add_database_task(app: tauri::AppHandle, name: String, due_date: Option<DateTime<Utc>>, tags: Option<Vec<String>>) -> Result<(), String> {
    let conn = Connection::open(get_db_path(app)).map_err(|e| e.to_string())?;
    
    let due_date_str = due_date.map(|dt| dt.to_rfc3339());

    let tags_str = tags.map(|vec| serde_json::to_string(&vec).unwrap());

    conn.execute(
        "INSERT INTO tasks (name, due_date, tags) VALUES (?1, ?2, ?3)",
        rusqlite::params![name, due_date_str, tags_str],
    ).map_err(|e| e.to_string())?;
    Ok(())
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Task {
    id: i32,
    name: String,
    due_date: Option<DateTime<Utc>>,
    created_at: DateTime<Utc>,
    completed: bool,
    completed_at: Option<DateTime<Utc>>,
    tags: Option<Vec<String>>,
}

#[tauri::command]
pub fn get_all_tasks(app: tauri::AppHandle) -> Result<Vec<Task>, String> {
    let conn = Connection::open(get_db_path(app)).map_err(|e| e.to_string())?;

    let mut stmt = conn.prepare(
        "SELECT id, name, due_date, created_at, completed, completed_at, tags FROM tasks"
    ).map_err(|e| e.to_string())?;

    let tasks_iter = stmt.query_map([], |row| {
        let due_date_str: Option<String> = row.get(2)?;
        let created_at_str: String = row.get(3)?;
        let completed_int: i32 = row.get(4)?;
        let completed_at_str: Option<String> = row.get(5)?;
        let tags_str: Option<String> = row.get(6)?;

        let due_date = match due_date_str {
            Some(s) => Some(
                DateTime::parse_from_rfc3339(&s)
                    .map_err(|e| rusqlite::Error::FromSqlConversionFailure(0, rusqlite::types::Type::Text, Box::new(e)))?
                    .with_timezone(&Utc)
            ),
            None => None,
        };

        let created_at = DateTime::parse_from_rfc3339(&created_at_str)
            .map_err(|e| rusqlite::Error::FromSqlConversionFailure(0, rusqlite::types::Type::Text, Box::new(e)))?
            .with_timezone(&Utc);

        let completed_at = match completed_at_str {
            Some(s) => Some(
                DateTime::parse_from_rfc3339(&s)
                    .map_err(|e| rusqlite::Error::FromSqlConversionFailure(0, rusqlite::types::Type::Text, Box::new(e)))?
                    .with_timezone(&Utc)
            ),
            None => None,
        };

        let completed = completed_int != 0;

        let tags = match tags_str {
            Some(s) => Some(serde_json::from_str(&s).unwrap_or_default()),
            None => None,
        };

        Ok(Task {
            id: row.get(0)?,
            name: row.get(1)?,
            due_date,
            created_at,
            completed,
            completed_at,
            tags,
        })
    }).map_err(|e| e.to_string())?;

    let mut tasks = Vec::new();
    for task_result in tasks_iter {
        tasks.push(task_result.map_err(|e| e.to_string())?);
    }

    Ok(tasks)
}

#[tauri::command]
pub fn complete_task(app: tauri::AppHandle, task_id: i32) -> Result<(), String> {
    let conn = Connection::open(get_db_path(app)).map_err(|e| e.to_string())?;
    conn.execute(
        "UPDATE tasks SET completed = 1, completed_at = datetime('now') WHERE id = ?1",
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
