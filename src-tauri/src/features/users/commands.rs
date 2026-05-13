use super::dto::{CreateUserInput, UpdateUserInput};
use super::entities::User;
use super::service::UserService;
use crate::core::Database;
use std::sync::Arc;
use tauri::State;

/// Get all users
#[tauri::command]
pub fn get_users(database: State<Arc<Database>>) -> Result<Vec<User>, String> {
    let mut conn = database.conn.lock().map_err(|e| e.to_string())?;
    UserService::get_all(&mut *conn).map_err(|e| e.into())
}

/// Get a single user by ID
#[tauri::command]
pub fn get_user(id: i32, database: State<Arc<Database>>) -> Result<User, String> {
    let mut conn = database.conn.lock().map_err(|e| e.to_string())?;
    UserService::get_by_id(&mut *conn, id).map_err(|e| e.into())
}

/// Create a new user
#[tauri::command]
pub fn create_user(input: CreateUserInput, database: State<Arc<Database>>) -> Result<User, String> {
    let mut conn = database.conn.lock().map_err(|e| e.to_string())?;
    UserService::create(&mut *conn, input).map_err(|e| e.into())
}

/// Update an existing user
#[tauri::command]
pub fn update_user(
    id: i32,
    input: UpdateUserInput,
    database: State<Arc<Database>>,
) -> Result<User, String> {
    let mut conn = database.conn.lock().map_err(|e| e.to_string())?;
    UserService::update(&mut *conn, id, input).map_err(|e| e.into())
}

/// Delete a user
#[tauri::command]
pub fn delete_user(id: i32, database: State<Arc<Database>>) -> Result<(), String> {
    let mut conn = database.conn.lock().map_err(|e| e.to_string())?;
    UserService::delete(&mut *conn, id).map_err(|e| e.into())
}
