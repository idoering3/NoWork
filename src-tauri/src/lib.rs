// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
mod commands;
mod calendar;
use crate::{calendar::cal_credentials::init_keyring, commands::database::init_db};

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    init_keyring();

    tauri::Builder::default()
        .plugin(tauri_plugin_single_instance::init(|_app, _args, _cwd| {
        // Write your code here...
        }))
        .plugin(tauri_plugin_fs::init())
        .plugin(tauri_plugin_window_state::Builder::new().build())
        .plugin(tauri_plugin_notification::init())
        .plugin(tauri_plugin_store::Builder::new().build())
        .plugin(tauri_plugin_opener::init())
        .setup(|app| {
            let handle = app.handle();
            init_db(&handle)?;
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            commands::greet::greet,
            commands::tasks::add_database_task,
            commands::tasks::get_all_tasks,
            commands::tasks::complete_task,
            commands::tasks::delete_task,
            commands::tags::get_all_tags,
            commands::tags::add_tag,
            commands::tags::remove_tag,
            commands::tags::add_tag_to_task,
            commands::tags::remove_tag_from_task,
            commands::database::reset_database,
            commands::tasks::get_incomplete_tasks,
            commands::tasks::get_completed_task_count,
            commands::tasks::get_tasks_due_today,
            commands::tasks::get_task_by_id,
            commands::tasks::update_task_name_by_id,
            commands::tasks::update_task_due_date_by_id,
            commands::tags::update_tag_color,
            calendar::authtest::test_auth,
            calendar::cal_credentials::save_credentials,
            calendar::cal_credentials::load_credentials,
            calendar::cal_credentials::delete_credentials

        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
