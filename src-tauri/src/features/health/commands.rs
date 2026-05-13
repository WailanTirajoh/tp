/// Health check command
#[tauri::command]
pub fn health_check() -> Result<String, String> {
    Ok("OK".to_string())
}
