use crate::commands::{database, types::{Tag, NewTag}};
use rusqlite::params;
use tauri::AppHandle;

// Add a tag
#[tauri::command]
pub fn add_tag(app: AppHandle, new_tag: NewTag) -> Result<Tag, String> {
    let conn = database::open_conn(&app).map_err(|e| e.to_string())?;

    conn.execute(
        "INSERT INTO tags (name, color) VALUES (?1, ?2)",
        (&new_tag.name, &new_tag.color),
    ).map_err(|e| e.to_string())?;

    let new_id = conn.last_insert_rowid();

    Ok(Tag {
        id: new_id,
        name: new_tag.name,
        color: new_tag.color,
    })
}

// Get all tags
#[tauri::command]
pub fn get_all_tags(app: AppHandle) -> Result<Vec<Tag>, String> {
    let conn = database::open_conn(&app).map_err(|e| e.to_string())?;

    let mut stmt = conn.prepare("SELECT id, name, color FROM tags ORDER BY name")
        .map_err(|e| e.to_string())?;

    let iter = stmt.query_map([], |row| Ok(Tag {
        id: row.get(0)?,
        name: row.get(1)?,
        color: row.get(2)?,
    })).map_err(|e| e.to_string())?;

    let mut tags = Vec::new();
    for tag in iter {
        tags.push(tag.map_err(|e| e.to_string())?);
    }

    Ok(tags)
}

// Remove tag
#[tauri::command]
pub fn remove_tag(app: AppHandle, tag_name: String) -> Result<(), String> {
    let conn = database::open_conn(&app).map_err(|e| e.to_string())?;

    let tag_id: i64 = conn
        .query_row("SELECT id FROM tags WHERE name = ?1", params![tag_name], |row| row.get(0))
        .map_err(|e| e.to_string())?;

    conn.execute("DELETE FROM task_tags WHERE tag_id = ?1", params![tag_id])
        .map_err(|e| e.to_string())?;
    conn.execute("DELETE FROM tags WHERE id = ?1", params![tag_id])
        .map_err(|e| e.to_string())?;

    Ok(())
}

// Add tag to task
#[tauri::command]
pub fn add_tag_to_task(app: AppHandle, task_id: i32, tag_id: i32) -> Result<(), String> {
    let conn = database::open_conn(&app).map_err(|e| e.to_string())?;
    conn.execute(
        "INSERT OR IGNORE INTO task_tags (task_id, tag_id) VALUES (?, ?)",
        params![task_id, tag_id],
    ).map_err(|e| e.to_string())?;
    Ok(())
}

// Remove tag from task
#[tauri::command]
pub fn remove_tag_from_task(app: AppHandle, task_id: i32, tag_id: i32) -> Result<(), String> {
    let conn = database::open_conn(&app).map_err(|e| e.to_string())?;
    conn.execute(
        "DELETE FROM task_tags WHERE task_id = ? AND tag_id = ?",
        params![task_id, tag_id],
    ).map_err(|e| e.to_string())?;
    Ok(())
}

// Update tag color
#[tauri::command]
pub fn update_tag_color(app: AppHandle, tag_id: i32, color: String) -> Result<(), String> {
    let conn = database::open_conn(&app).map_err(|e| e.to_string())?;
    conn.execute(
        "UPDATE tags SET color = ?1 WHERE id = ?2",
        params![color, tag_id],
    ).map_err(|e| e.to_string())?;
    Ok(())
}
