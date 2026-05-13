use super::Database;
use std::sync::Arc;

/// Application state shared across Tauri handlers
#[allow(dead_code)]
pub struct AppState {
    pub database: Arc<Database>,
}

#[allow(dead_code)]
impl AppState {
    pub fn new(database: Arc<Database>) -> Self {
        Self { database }
    }
}
