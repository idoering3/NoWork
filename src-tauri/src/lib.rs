// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
mod commands;
use crate::commands::database::init_db;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
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
            commands::database::delete_task
            ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
