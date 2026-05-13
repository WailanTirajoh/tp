use thiserror::Error;

/// Custom error types for the POS application
#[derive(Error, Debug)]
pub enum AppError {
    #[error("Database error: {0}")]
    Database(#[from] diesel::result::Error),

    #[error("Validation error: {0}")]
    Validation(String),

    #[error("Not found: {0}")]
    NotFound(String),

    #[error("Already exists: {0}")]
    AlreadyExists(String),

    #[error("Internal error: {0}")]
    Internal(String),

    #[error("Serialization error: {0}")]
    Serialization(#[from] serde_json::Error),

    #[error("Mutex lock error")]
    MutexLock,
}

/// Result type alias for convenience
pub type AppResult<T> = Result<T, AppError>;

/// Convert AppError to String for Tauri commands
impl From<AppError> for String {
    fn from(error: AppError) -> Self {
        error.to_string()
    }
}

/// Convert mutex lock errors
impl<T> From<std::sync::PoisonError<T>> for AppError {
    fn from(_: std::sync::PoisonError<T>) -> Self {
        AppError::MutexLock
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error_display() {
        let err = AppError::Validation("Invalid email".to_string());
        assert_eq!(err.to_string(), "Validation error: Invalid email");
    }

    #[test]
    fn test_error_to_string_conversion() {
        let err = AppError::NotFound("User".to_string());
        let s: String = err.into();
        assert_eq!(s, "Not found: User");
    }

    #[test]
    fn test_database_error_conversion() {
        let db_err = diesel::result::Error::NotFound;
        let app_err: AppError = db_err.into();
        assert!(matches!(app_err, AppError::Database(_)));
    }
}
