mod db;
mod models;

use db::Database;
use models::{CreateUserInput, UpdateUserInput, User};
use std::sync::Arc;
use tauri::{Manager, State};

// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
fn get_users(database: State<Arc<Database>>) -> Result<Vec<User>, String> {
    let conn = database.conn.lock().map_err(|e| e.to_string())?;

    let mut stmt = conn
        .prepare("SELECT id, name, email, age, created_at, updated_at FROM users ORDER BY id DESC")
        .map_err(|e| e.to_string())?;

    let users = stmt
        .query_map([], |row| {
            Ok(User {
                id: row.get(0)?,
                name: row.get(1)?,
                email: row.get(2)?,
                age: row.get(3)?,
                created_at: row.get(4)?,
                updated_at: row.get(5)?,
            })
        })
        .map_err(|e| e.to_string())?
        .collect::<Result<Vec<_>, _>>()
        .map_err(|e| e.to_string())?;

    Ok(users)
}

#[tauri::command]
fn get_user(id: i64, database: State<Arc<Database>>) -> Result<User, String> {
    let conn = database.conn.lock().map_err(|e| e.to_string())?;

    let user = conn
        .query_row(
            "SELECT id, name, email, age, created_at, updated_at FROM users WHERE id = ?1",
            [id],
            |row| {
                Ok(User {
                    id: row.get(0)?,
                    name: row.get(1)?,
                    email: row.get(2)?,
                    age: row.get(3)?,
                    created_at: row.get(4)?,
                    updated_at: row.get(5)?,
                })
            },
        )
        .map_err(|e| e.to_string())?;

    Ok(user)
}

#[tauri::command]
fn create_user(input: CreateUserInput, database: State<Arc<Database>>) -> Result<User, String> {
    let conn = database.conn.lock().map_err(|e| e.to_string())?;

    conn.execute(
        "INSERT INTO users (name, email, age) VALUES (?1, ?2, ?3)",
        (&input.name, &input.email, &input.age),
    )
    .map_err(|e| e.to_string())?;

    let id = conn.last_insert_rowid();

    let user = conn
        .query_row(
            "SELECT id, name, email, age, created_at, updated_at FROM users WHERE id = ?1",
            [id],
            |row| {
                Ok(User {
                    id: row.get(0)?,
                    name: row.get(1)?,
                    email: row.get(2)?,
                    age: row.get(3)?,
                    created_at: row.get(4)?,
                    updated_at: row.get(5)?,
                })
            },
        )
        .map_err(|e| e.to_string())?;

    Ok(user)
}

#[tauri::command]
fn update_user(
    id: i64,
    input: UpdateUserInput,
    database: State<Arc<Database>>,
) -> Result<User, String> {
    let conn = database.conn.lock().map_err(|e| e.to_string())?;

    // Build dynamic update query
    let mut updates = Vec::new();
    let mut params: Vec<Box<dyn rusqlite::ToSql>> = Vec::new();

    if let Some(name) = &input.name {
        updates.push("name = ?");
        params.push(Box::new(name.clone()));
    }
    if let Some(email) = &input.email {
        updates.push("email = ?");
        params.push(Box::new(email.clone()));
    }
    if let Some(age) = &input.age {
        updates.push("age = ?");
        params.push(Box::new(*age));
    }

    if updates.is_empty() {
        return Err("No fields to update".to_string());
    }

    updates.push("updated_at = CURRENT_TIMESTAMP");
    params.push(Box::new(id));

    let query = format!("UPDATE users SET {} WHERE id = ?", updates.join(", "));

    let params_refs: Vec<&dyn rusqlite::ToSql> = params.iter().map(|p| p.as_ref()).collect();

    conn.execute(&query, params_refs.as_slice())
        .map_err(|e| e.to_string())?;

    let user = conn
        .query_row(
            "SELECT id, name, email, age, created_at, updated_at FROM users WHERE id = ?1",
            [id],
            |row| {
                Ok(User {
                    id: row.get(0)?,
                    name: row.get(1)?,
                    email: row.get(2)?,
                    age: row.get(3)?,
                    created_at: row.get(4)?,
                    updated_at: row.get(5)?,
                })
            },
        )
        .map_err(|e| e.to_string())?;

    Ok(user)
}

#[tauri::command]
fn delete_user(id: i64, database: State<Arc<Database>>) -> Result<(), String> {
    let conn = database.conn.lock().map_err(|e| e.to_string())?;

    let rows_affected = conn
        .execute("DELETE FROM users WHERE id = ?1", [id])
        .map_err(|e| e.to_string())?;

    if rows_affected == 0 {
        return Err("User not found".to_string());
    }

    Ok(())
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
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
            get_users,
            get_user,
            create_user,
            update_user,
            delete_user
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
