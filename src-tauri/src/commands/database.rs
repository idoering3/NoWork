use rusqlite::{Connection, Result};
use std::path::PathBuf;
use tauri::{AppHandle, Manager};

pub fn get_db_path(app: &AppHandle) -> PathBuf {
    let base_dir = app
        .path()
        .app_data_dir()
        .expect("Base directory not found!");
    std::fs::create_dir_all(&base_dir).expect("Failed to create data dir");
    base_dir.join("tasks.db")
}

pub fn init_db(app: &AppHandle) -> Result<()> {
    let conn = Connection::open(get_db_path(app))?;

    conn.execute(
        "CREATE TABLE IF NOT EXISTS tasks (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            name TEXT NOT NULL,
            due_date TEXT,
            created_at TEXT DEFAULT (strftime('%Y-%m-%dT%H:%M:%fZ', 'now')),
            completed BOOLEAN DEFAULT 0,
            completed_at TEXT
        )",
        [],
    )?;

    conn.execute(
        "CREATE TABLE IF NOT EXISTS tags (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            name TEXT NOT NULL UNIQUE,
            color TEXT DEFAULT 'default'
        )",
        [],
    )?;

    // ignore error if column exists
    let _ = conn.execute(
        "ALTER TABLE tags ADD COLUMN color TEXT DEFAULT 'default'",
        [],
    );

    conn.execute(
        "CREATE TABLE IF NOT EXISTS task_tags (
            task_id INTEGER NOT NULL,
            tag_id INTEGER NOT NULL,
            PRIMARY KEY (task_id, tag_id),
            FOREIGN KEY (task_id) REFERENCES tasks(id) ON DELETE CASCADE,
            FOREIGN KEY (tag_id) REFERENCES tags(id) ON DELETE CASCADE
        )",
        [],
    )?;

    Ok(())
}

pub fn open_conn(app: &AppHandle) -> Result<Connection> {
    Connection::open(get_db_path(app))
}

#[tauri::command]
pub fn reset_database(app: AppHandle) -> Result<(), String> {
    let db_path = get_db_path(&app);
    if db_path.exists() {
        std::fs::remove_file(&db_path).map_err(|e| e.to_string())?;
    }
    init_db(&app).map_err(|e| e.to_string())?;
    Ok(())
}
