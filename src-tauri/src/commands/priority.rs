use rusqlite::params;
use tauri::AppHandle;

use crate::commands::{database, tasks::get_task_by_id, types::{Task, TaskPriority}};

#[tauri::command]
pub fn update_task_priority_by_id(app: AppHandle, task_id: i32, new_priority: Option<TaskPriority>) -> Result<Task, String> {
    let conn = database::open_conn(&app).map_err(|e| e.to_string())?;
    
    let priority_num = new_priority.map(|p| p.as_i32());

    let rows = conn
    .execute(
        "UPDATE tasks SET priority = ?1 WHERE id = ?2",
        params![priority_num, task_id],
    ).map_err(|e| e.to_string())?;

    if rows == 0 {
        return Err("No task found with given id".to_string());
    }

    get_task_by_id(app, task_id)
}