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

    // tasks table
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

    // Tags table (unique names)
    conn.execute(
        "CREATE TABLE IF NOT EXISTS tags (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            name TEXT NOT NULL UNIQUE
        )",
        [],
    )?;

    // Many-to-many relationship
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


#[tauri::command]
pub fn add_database_task(
    app: tauri::AppHandle,
    name: String,
    due_date: Option<DateTime<Utc>>,
    tags: Option<Vec<String>>
) -> Result<(), String> {
    let mut conn = Connection::open(get_db_path(app)).map_err(|e| e.to_string())?;
    let tx = conn.transaction().map_err(|e| e.to_string())?;

    let due_date_str = due_date.map(|dt| dt.to_rfc3339());

    tx.execute(
        "INSERT INTO tasks (name, due_date) VALUES (?1, ?2)",
        rusqlite::params![name, due_date_str],
    ).map_err(|e| e.to_string())?;

    let task_id = tx.last_insert_rowid();

    if let Some(tags_vec) = tags {
        for tag_name in tags_vec {
            // Insert tag if it doesn't exist
            tx.execute(
                "INSERT OR IGNORE INTO tags (name) VALUES (?1)",
                rusqlite::params![tag_name],
            ).map_err(|e| e.to_string())?;

            // Get the tag ID
            let tag_id: i64 = tx.query_row(
                "SELECT id FROM tags WHERE name = ?1",
                rusqlite::params![tag_name],
                |row| row.get(0),
            ).map_err(|e| e.to_string())?;

            // Link task and tag
            tx.execute(
                "INSERT INTO task_tags (task_id, tag_id) VALUES (?1, ?2)",
                rusqlite::params![task_id, tag_id],
            ).map_err(|e| e.to_string())?;
        }
    }

    tx.commit().map_err(|e| e.to_string())?;
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
        "SELECT id, name, due_date, created_at, completed, completed_at FROM tasks"
    ).map_err(|e| e.to_string())?;

    let task_rows = stmt.query_map([], |row| {
        Ok((
            row.get::<_, i32>(0)?,  // id
            row.get::<_, String>(1)?, // name
            row.get::<_, Option<String>>(2)?, // due_date
            row.get::<_, String>(3)?, // created_at
            row.get::<_, i32>(4)?, // completed
            row.get::<_, Option<String>>(5)?, // completed_at
        ))
    }).map_err(|e| e.to_string())?;

    let mut tasks = Vec::new();

    for row in task_rows {
        let (id, name, due_date_str, created_at_str, completed_int, completed_at_str) = 
            row.map_err(|e| e.to_string())?;

        let due_date = match due_date_str {
            Some(s) => Some(DateTime::parse_from_rfc3339(&s)
                .map_err(|e| format!("Invalid due_date format: {}", e))?
                .with_timezone(&Utc)),
            None => None,
        };

        let created_at = if created_at_str.trim().is_empty() {
            // Handle empty created_at: either error or provide a fallback datetime
            return Err("Empty created_at field in DB".to_string());
        } else {
            DateTime::parse_from_rfc3339(&created_at_str)
                .map_err(|e| format!("Invalid created_at format: {}", e))?
                .with_timezone(&Utc)
        };

        let completed_at = match completed_at_str {
            Some(s) => Some(DateTime::parse_from_rfc3339(&s)
                .map_err(|e| format!("Invalid completed_at format: {}", e))?
                .with_timezone(&Utc)),
            None => None,
        };

        let completed = completed_int != 0;

        // Fetch tags for this task
        let mut tag_stmt = conn.prepare(
            "SELECT tags.name FROM tags
             JOIN task_tags ON tags.id = task_tags.tag_id
             WHERE task_tags.task_id = ?1"
        ).map_err(|e| e.to_string())?;

        let tag_iter = tag_stmt.query_map([id], |row| row.get::<_, String>(0))
            .map_err(|e| e.to_string())?;

        let mut tags = Vec::new();
        for t in tag_iter {
            tags.push(t.map_err(|e| e.to_string())?);
        }

        tasks.push(Task {
            id,
            name,
            due_date,
            created_at,
            completed,
            completed_at,
            tags: if tags.is_empty() { None } else { Some(tags) },
        });
    }

    Ok(tasks)
}

#[tauri::command]
pub fn get_incomplete_tasks(app: tauri::AppHandle) -> Result<Vec<Task>, String> {
    let conn = Connection::open(get_db_path(app)).map_err(|e| e.to_string())?;

    let mut stmt = conn.prepare(
        "SELECT id, name, due_date, created_at, completed, completed_at
         FROM tasks
         WHERE completed = 0"
    ).map_err(|e| e.to_string())?;

    let task_rows = stmt.query_map([], |row| {
        Ok((
            row.get::<_, i32>(0)?,  // id
            row.get::<_, String>(1)?, // name
            row.get::<_, Option<String>>(2)?, // due_date
            row.get::<_, String>(3)?, // created_at
            row.get::<_, i32>(4)?, // completed
            row.get::<_, Option<String>>(5)?, // completed_at
        ))
    }).map_err(|e| e.to_string())?;

    let mut tasks = Vec::new();

    for row in task_rows {
        let (id, name, due_date_str, created_at_str, completed_int, completed_at_str) = 
            row.map_err(|e| e.to_string())?;

        let due_date = match due_date_str {
            Some(s) => Some(DateTime::parse_from_rfc3339(&s)
                .map_err(|e| format!("Invalid due_date format: {}", e))?
                .with_timezone(&Utc)),
            None => None,
        };

        let created_at = if created_at_str.trim().is_empty() {
            return Err("Empty created_at field in DB".to_string());
        } else {
            DateTime::parse_from_rfc3339(&created_at_str)
                .map_err(|e| format!("Invalid created_at format: {}", e))?
                .with_timezone(&Utc)
        };

        let completed_at = match completed_at_str {
            Some(s) => Some(DateTime::parse_from_rfc3339(&s)
                .map_err(|e| format!("Invalid completed_at format: {}", e))?
                .with_timezone(&Utc)),
            None => None,
        };

        let completed = completed_int != 0;

        // Get tags for this task
        let mut tag_stmt = conn.prepare(
            "SELECT tags.name FROM tags
             JOIN task_tags ON tags.id = task_tags.tag_id
             WHERE task_tags.task_id = ?1"
        ).map_err(|e| e.to_string())?;

        let tag_iter = tag_stmt.query_map([id], |row| row.get::<_, String>(0))
            .map_err(|e| e.to_string())?;

        let mut tags = Vec::new();
        for t in tag_iter {
            tags.push(t.map_err(|e| e.to_string())?);
        }

        tasks.push(Task {
            id,
            name,
            due_date,
            created_at,
            completed,
            completed_at,
            tags: if tags.is_empty() { None } else { Some(tags) },
        });
    }

    Ok(tasks)
}

#[tauri::command]
pub fn get_completed_task_count(app: tauri::AppHandle) -> Result<i64, String> {
    let conn = Connection::open(get_db_path(app)).map_err(|e| e.to_string())?;
    let count: i64 = conn.query_row(
        "SELECT COUNT(*) FROM tasks WHERE completed = 1",
        [],
        |row| row.get(0),
    ).map_err(|e| e.to_string())?;
    Ok(count)
}


#[tauri::command]
pub fn reset_database(app: tauri::AppHandle) -> Result<(), String> {
    let db_path = get_db_path(app.clone());
    if db_path.exists() {
        std::fs::remove_file(&db_path).map_err(|e| e.to_string())?;
    }
    init_db(app).map_err(|e| e.to_string())?;
    Ok(())
}



#[tauri::command]
pub fn complete_task(app: tauri::AppHandle, task_id: i32) -> Result<(), String> {
    let conn = Connection::open(get_db_path(app)).map_err(|e| e.to_string())?;
    conn.execute(
        "UPDATE tasks SET completed = 1, completed_at = strftime('%Y-%m-%dT%H:%M:%fZ', 'now') WHERE id = ?1",
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

#[derive(Serialize, Deserialize)]
pub struct Tag {
    id: i32,
    name: String,
}

#[tauri::command]
pub fn get_all_tags(app: tauri::AppHandle) -> Result<Vec<Tag>, String> {
    let conn = Connection::open(get_db_path(app)).map_err(|e| e.to_string())?;

    let mut stmt = conn.prepare("SELECT id, name FROM tags ORDER BY name")
        .map_err(|e| e.to_string())?;

    let tags_iter = stmt.query_map([], |row| {
        Ok(Tag {
            id: row.get(0)?,
            name: row.get(1)?,
        })
    }).map_err(|e| e.to_string())?;

    let mut tags = Vec::new();
    for tag in tags_iter {
        tags.push(tag.map_err(|e| e.to_string())?);
    }

    Ok(tags)
}

#[tauri::command]
pub fn add_tag(app: tauri::AppHandle, tag_name: String) -> Result<Tag, String> {
    let conn = Connection::open(get_db_path(app)).map_err(|e| e.to_string())?;

    // Insert tag if new
    conn.execute(
        "INSERT INTO tags (name) VALUES (?) ON CONFLICT(name) DO NOTHING",
        &[&tag_name],
    ).map_err(|e| e.to_string())?;

    // Get the tag ID
    let mut stmt = conn.prepare("SELECT id, name FROM tags WHERE name = ?")
        .map_err(|e| e.to_string())?;

    let tag = stmt.query_row([&tag_name], |row| {
        Ok(Tag {
            id: row.get(0)?,
            name: row.get(1)?,
        })
    }).map_err(|e| e.to_string())?;

    Ok(tag)
}

#[tauri::command]
pub fn remove_tag(app: tauri::AppHandle, tag_name: String) -> Result<(), String> {
    let conn = Connection::open(get_db_path(app)).map_err(|e| e.to_string())?;

    // Query the tag ID by name
    let tag_id: i64 = conn.query_row(
        "SELECT id FROM tags WHERE name = ?1",
        &[&tag_name],
        |row| row.get(0),
    ).map_err(|e| e.to_string())?;

    // Delete all links from task_tags with this tag_id
    conn.execute(
        "DELETE FROM task_tags WHERE tag_id = ?1",
        &[&tag_id],
    ).map_err(|e| e.to_string())?;

    // Delete the tag itself
    conn.execute(
        "DELETE FROM tags WHERE id = ?1",
        &[&tag_id],
    ).map_err(|e| e.to_string())?;

    Ok(())
}

#[tauri::command]
pub fn add_tag_to_task(app: tauri::AppHandle, task_id: i32, tag_id: i32) -> Result<(), String> {
    let conn = Connection::open(get_db_path(app)).map_err(|e| e.to_string())?;

    // Insert link, ignore if it already exists to avoid duplicates
    conn.execute(
        "INSERT INTO task_tags (task_id, tag_id) VALUES (?, ?) ON CONFLICT DO NOTHING",
        &[&task_id, &tag_id],
    ).map_err(|e| e.to_string())?;

    Ok(())
}


#[tauri::command]
pub fn remove_tag_from_task(app: tauri::AppHandle, task_id: i32, tag_id: i32) -> Result<(), String> {
    let conn = Connection::open(get_db_path(app)).map_err(|e| e.to_string())?;

    conn.execute(
        "DELETE FROM task_tags WHERE task_id = ? AND tag_id = ?",
        &[&task_id, &tag_id],
    ).map_err(|e| e.to_string())?;

    Ok(())
}