mod core;
mod features;
mod schema;

use core::Database;
use features::storage;
use std::sync::Arc;
use tauri::Manager;

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

            // Initialize reactive storage manager
            let storage_manager = Arc::new(storage::StorageManager::new(app.handle().clone()));

            // Load existing documents from SQLite into memory
            if let Ok(documents) = storage::repository::StorageRepository::load_all(&database) {
                storage_manager.load_documents(documents);
                println!(
                    "Loaded {} documents into memory",
                    storage_manager.get_all().len()
                );
            }

            app.manage(database);
            app.manage(storage_manager);

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            storage::storage_get,
            storage::storage_set,
            storage::storage_delete,
            storage::storage_query_collection,
            storage::storage_keys,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
