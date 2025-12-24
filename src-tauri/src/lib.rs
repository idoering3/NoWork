// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
mod commands;
use crate::commands::database::init_db;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_fs::init())
        .plugin(tauri_plugin_window_state::Builder::new().build())
        .plugin(tauri_plugin_notification::init())
        .plugin(tauri_plugin_store::Builder::new().build())
        .plugin(tauri_plugin_opener::init())
        .setup(|app| {
            let handle = app.handle();
            init_db(handle.clone())?;
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            commands::greet::greet,
            commands::database::add_database_task,
            commands::database::get_all_tasks,
            commands::database::complete_task,
            commands::database::delete_task,
            commands::database::get_all_tags,
            commands::database::add_tag,
            commands::database::remove_tag,
            commands::database::add_tag_to_task,
            commands::database::remove_tag_from_task,
            commands::database::reset_database,
            commands::database::get_incomplete_tasks,
            commands::database::get_completed_task_count,
            commands::database::get_tasks_due_today,
            commands::database::update_tag_color,
            commands::chief::random_gif
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
