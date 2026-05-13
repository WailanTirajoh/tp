mod core;
mod features;
mod schema;

use core::Database;
use features::{health, users};
use std::sync::Arc;
use tauri::Manager;

// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_dialog::init())
        .setup(|app| {
            // Initialize database
            let app_dir = app
                .path()
                .app_data_dir()
                .expect("Failed to get app data dir");
            std::fs::create_dir_all(&app_dir).expect("Failed to create app data dir");
            let db_path = app_dir.join("database.sqlite");

            let database = Arc::new(Database::new(db_path).expect("Failed to initialize database"));
            app.manage(database);

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            greet,
            users::get_users,
            users::get_user,
            users::create_user,
            users::update_user,
            users::delete_user,
            health::health_check
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
