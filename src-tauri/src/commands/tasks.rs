use crate::commands::{
    database, types::{NewTag, Tag, Task, TaskPriority},
};
use chrono::{DateTime, Datelike, Duration, Local, TimeZone, Utc};
use rusqlite::{Connection, params};
use tauri::AppHandle;

struct TaskRow {
    id: i32,
    name: String,
    due_date: Option<String>,
    created_at: String,
    completed: i32,
    completed_at: Option<String>,
    priority: Option<i32>,
}

// helper to construct a task given a sqlite row
fn task_from_row(
    row: TaskRow,
    conn: &Connection) 
-> Result<Task, String> {
    let priority_enum = row.priority.and_then(|n| TaskPriority::from_i32(n));

    Ok(Task {
        id: row.id,
        name: row.name,
        due_date: parse_opt_date(row.due_date)?,
        created_at: parse_opt_date(Some(row.created_at))?.unwrap(),
        completed: row.completed != 0,
        completed_at: parse_opt_date(row.completed_at)?,
        priority: priority_enum,
        tags: fetch_tags(conn, row.id)?,
    })
}

// Helper to fetch tags for a task
fn fetch_tags(conn: &rusqlite::Connection, task_id: i32) -> Result<Option<Vec<Tag>>, String> {
    let mut stmt = conn
        .prepare(
            "SELECT tags.id, tags.name, tags.color FROM tags
             JOIN task_tags ON tags.id = task_tags.tag_id
             WHERE task_tags.task_id = ?1",
        )
        .map_err(|e| e.to_string())?;

    let tag_iter = stmt
        .query_map([task_id], |row| {
            Ok(Tag {
                id: row.get(0)?,
                name: row.get(1)?,
                color: row.get(2)?,
            })
        })
        .map_err(|e| e.to_string())?;

    let mut tags = Vec::new();
    for t in tag_iter {
        tags.push(t.map_err(|e| e.to_string())?);
    }

    Ok(if tags.is_empty() { None } else { Some(tags) })
}

// Helper to parse optional RFC3339 date string
fn parse_opt_date(s: Option<String>) -> Result<Option<DateTime<Utc>>, String> {
    match s {
        Some(s) => Ok(Some(
            DateTime::parse_from_rfc3339(&s)
                .map_err(|e| e.to_string())?
                .with_timezone(&Utc),
        )),
        None => Ok(None),
    }
}

// Add a task
#[tauri::command]
pub fn add_database_task(
    app: AppHandle,
    name: String,
    due_date: Option<DateTime<Utc>>,
    priority: Option<TaskPriority>,
    tags: Option<Vec<NewTag>>,
) -> Result<(), String> {
    let mut conn = database::open_conn(&app).map_err(|e| e.to_string())?;
    let tx = conn.transaction().map_err(|e| e.to_string())?;

    let due_date_str = due_date.map(|dt| dt.to_rfc3339());
    let priority_num = priority.map(|p| p.as_i32());
    tx.execute(
        "INSERT INTO tasks (name, due_date, priority) VALUES (?1, ?2, ?3)",
        params![name, due_date_str, priority_num],
    )
    .map_err(|e| e.to_string())?;

    let task_id = tx.last_insert_rowid();

    if let Some(tags_vec) = tags {
        for tag in tags_vec {
            tx.execute(
                "INSERT INTO tags (name, color) VALUES (?1, ?2)
                 ON CONFLICT(name) DO UPDATE SET color = excluded.color",
                params![tag.name, tag.color],
            )
            .map_err(|e| e.to_string())?;

            let tag_id: i64 = tx
                .query_row(
                    "SELECT id FROM tags WHERE name = ?1",
                    params![tag.name],
                    |row| row.get(0),
                )
                .map_err(|e| e.to_string())?;

            tx.execute(
                "INSERT OR IGNORE INTO task_tags (task_id, tag_id) VALUES (?1, ?2)",
                params![task_id, tag_id],
            )
            .map_err(|e| e.to_string())?;
        }
    }

    tx.commit().map_err(|e| e.to_string())?;
    Ok(())
}

// Fetch all tasks
#[tauri::command]
pub fn get_all_tasks(app: AppHandle) -> Result<Vec<Task>, String> {
    let conn = database::open_conn(&app).map_err(|e| e.to_string())?;
    let mut stmt = conn
        .prepare("SELECT id, name, due_date, created_at, completed, completed_at, priority FROM tasks")
        .map_err(|e| e.to_string())?;

    let rows = stmt
        .query_map([], |row| {
            Ok(TaskRow {
                id: row.get(0)?,
                name: row.get(1)?,
                due_date: row.get(2)?,
                created_at: row.get(3)?,
                completed: row.get(4)?,
                completed_at: row.get(5)?,
                priority: row.get(6)?
            })
        })
        .map_err(|e| e.to_string())?;

    let mut tasks = Vec::new();
    for row in rows {
        let task_row = row.map_err(|e| e.to_string())?;
        let task = task_from_row(task_row, &conn)?;

        tasks.push(task);
    }
    Ok(tasks)
}

// Fetch incomplete tasks
#[tauri::command]
pub fn get_incomplete_tasks(app: AppHandle) -> Result<Vec<Task>, String> {
    let conn = database::open_conn(&app).map_err(|e| e.to_string())?;
    let mut stmt = conn
        .prepare("SELECT id, name, due_date, created_at, completed, completed_at, priority FROM tasks WHERE completed = 0")
        .map_err(|e| e.to_string())?;

    let rows = stmt
        .query_map([], |row| {
            Ok(TaskRow {
                id: row.get(0)?,
                name: row.get(1)?,
                due_date: row.get(2)?,
                created_at: row.get(3)?,
                completed: row.get(4)?,
                completed_at: row.get(5)?,
                priority: row.get(6)?
            })
        })
        .map_err(|e| e.to_string())?;

    let mut tasks = Vec::new();
    for row in rows {
        let task_row = row.map_err(|e| e.to_string())?;
        let task = task_from_row(task_row, &conn)?;

        tasks.push(task);
    }
    Ok(tasks)
}

// Complete a task
#[tauri::command]
pub fn complete_task(app: AppHandle, task_id: i32) -> Result<(), String> {
    let conn = database::open_conn(&app).map_err(|e| e.to_string())?;
    conn.execute(
        "UPDATE tasks SET completed = 1, completed_at = strftime('%Y-%m-%dT%H:%M:%fZ', 'now') WHERE id = ?1",
        params![task_id],
    ).map_err(|e| e.to_string())?;
    Ok(())
}

// Delete a task
#[tauri::command]
pub fn delete_task(app: AppHandle, task_id: i32) -> Result<(), String> {
    let conn = database::open_conn(&app).map_err(|e| e.to_string())?;
    conn.execute("DELETE FROM tasks WHERE id = ?1", params![task_id])
        .map_err(|e| e.to_string())?;
    Ok(())
}

// Tasks due today
#[tauri::command]
pub fn get_tasks_due_today(app: AppHandle) -> Result<Vec<Task>, String> {
    let conn = database::open_conn(&app).map_err(|e| e.to_string())?;

    let today = Local::now().date_naive();
    let start = today.and_hms_opt(0, 0, 0).unwrap();
    let end = today.and_hms_opt(23, 59, 59).unwrap();

    let start_utc = Local
        .from_local_datetime(&start)
        .unwrap()
        .with_timezone(&Utc);
    let end_utc = Local.from_local_datetime(&end).unwrap().with_timezone(&Utc);

    let mut stmt = conn
        .prepare(
            "SELECT id, name, due_date, created_at, completed, completed_at, priority
         FROM tasks
         WHERE due_date IS NOT NULL AND due_date >= ?1 AND due_date <= ?2",
        )
        .map_err(|e| e.to_string())?;

    let rows = stmt
        .query_map([], |row| {
            Ok(TaskRow {
                id: row.get(0)?,
                name: row.get(1)?,
                due_date: row.get(2)?,
                created_at: row.get(3)?,
                completed: row.get(4)?,
                completed_at: row.get(5)?,
                priority: row.get(6)?
            })
        })
        .map_err(|e| e.to_string())?;

    let mut tasks = Vec::new();
    for row in rows {
        let task_row = row.map_err(|e| e.to_string())?;
        let task = task_from_row(task_row, &conn)?;

        tasks.push(task);
    }
    Ok(tasks)
}

#[tauri::command]
pub fn get_tasks_due_this_week(app: AppHandle) -> Result<Vec<Task>, String> {
    let conn = database::open_conn(&app).map_err(|e| e.to_string())?;

    let today = Local::now().date_naive();
    let weekday = today.weekday().number_from_sunday() as i64;

    let start_date = today - Duration::days(weekday - 1);
    let end_date = start_date + Duration::days(6);

    let start = start_date.and_hms_opt(0, 0, 0).unwrap();
    let end = end_date.and_hms_opt(23, 59, 59).unwrap();

    let start_utc = Local
        .from_local_datetime(&start)
        .unwrap()
        .with_timezone(&Utc);
    let end_utc = Local.from_local_datetime(&end).unwrap().with_timezone(&Utc);

    let mut stmt = conn
        .prepare(
            "SELECT id, name, due_date, created_at, completed, completed_at, priority
         FROM tasks
         WHERE due_date IS NOT NULL AND due_date >= ?1 AND due_date <= ?2",
        )
        .map_err(|e| e.to_string())?;

    let rows = stmt
        .query_map([], |row| {
            Ok(TaskRow {
                id: row.get(0)?,
                name: row.get(1)?,
                due_date: row.get(2)?,
                created_at: row.get(3)?,
                completed: row.get(4)?,
                completed_at: row.get(5)?,
                priority: row.get(6)?
            })
        })
        .map_err(|e| e.to_string())?;

    let mut tasks = Vec::new();
    for row in rows {
        let task_row = row.map_err(|e| e.to_string())?;
        let task = task_from_row(task_row, &conn)?;

        tasks.push(task);
    }
    Ok(tasks)
}

// Completed task count
#[tauri::command]
pub fn get_completed_task_count(app: AppHandle) -> Result<i64, String> {
    let conn = database::open_conn(&app).map_err(|e| e.to_string())?;
    let count: i64 = conn
        .query_row(
            "SELECT COUNT(*) FROM tasks WHERE completed = 1",
            [],
            |row| row.get(0),
        )
        .map_err(|e| e.to_string())?;
    Ok(count)
}

#[tauri::command]
pub fn get_task_by_id(app: AppHandle, task_id: i32) -> Result<Task, String> {
    let conn = database::open_conn(&app).map_err(|e| e.to_string())?;

    let mut stmt = conn
        .prepare(
            "SELECT id, name, due_date, created_at, completed, completed_at, priority
             FROM tasks
             WHERE id = ?1",
        )
        .map_err(|e| e.to_string())?;

    let row = stmt
        .query_row(params![task_id], |row| {
            Ok(TaskRow {
                id: row.get(0)?,
                name: row.get(1)?,
                due_date: row.get(2)?,
                created_at: row.get(3)?,
                completed: row.get(4)?,
                completed_at: row.get(5)?,
                priority: row.get(6)?,
            })
        })
        .map_err(|e| e.to_string())?;

    let task = task_from_row(row, &conn)?;

    Ok(task)
}

#[tauri::command]
pub fn update_task_name_by_id(
    app: AppHandle,
    task_id: i32,
    new_name: String,
) -> Result<Task, String> {
    let conn = database::open_conn(&app).map_err(|e| e.to_string())?;

    let rows = conn
        .execute(
            "UPDATE tasks SET name = ?1 WHERE id = ?2",
            params![new_name, task_id],
        )
        .map_err(|e| e.to_string())?;

    if rows == 0 {
        return Err("No task found with given id".to_string());
    }

    // Reuse your existing logic instead of duplicating it
    get_task_by_id(app, task_id)
}

#[tauri::command]
pub fn update_task_due_date_by_id(
    app: AppHandle,
    task_id: i32,
    new_due_date: Option<DateTime<Utc>>,
) -> Result<Task, String> {
    let conn = database::open_conn(&app).map_err(|e| e.to_string())?;

    // Emulate your existing date handling:
    // Option<DateTime<Utc>> -> Option<String> (RFC3339)
    let due_date_str = new_due_date.map(|dt| dt.to_rfc3339());

    let rows = conn
        .execute(
            "UPDATE tasks SET due_date = ?1 WHERE id = ?2",
            params![due_date_str, task_id],
        )
        .map_err(|e| e.to_string())?;

    if rows == 0 {
        return Err("No task found with given id".to_string());
    }

    // Reuse the canonical fetch logic
    get_task_by_id(app, task_id)
}
