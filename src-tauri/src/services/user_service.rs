use crate::errors::{AppError, AppResult};
use crate::models::{CreateUserInput, UpdateUserInput, User};
use crate::repositories::UserRepository;
use crate::schema::users;
use diesel::prelude::*;
use diesel::sqlite::SqliteConnection;

/// Service for user business logic
pub struct UserService;

impl UserService {
    /// Validate email format
    fn validate_email(email: &str) -> AppResult<()> {
        if !email.contains('@') || !email.contains('.') {
            return Err(AppError::Validation("Invalid email format".to_string()));
        }
        Ok(())
    }

    /// Validate user input
    fn validate_create_input(input: &CreateUserInput) -> AppResult<()> {
        if input.name.trim().is_empty() {
            return Err(AppError::Validation("Name cannot be empty".to_string()));
        }

        if input.name.len() > 100 {
            return Err(AppError::Validation(
                "Name cannot exceed 100 characters".to_string(),
            ));
        }

        Self::validate_email(&input.email)?;

        if let Some(age) = input.age {
            if age < 0 || age > 150 {
                return Err(AppError::Validation(
                    "Age must be between 0 and 150".to_string(),
                ));
            }
        }

        Ok(())
    }

    /// Get all users
    pub fn get_all(conn: &mut SqliteConnection) -> AppResult<Vec<User>> {
        UserRepository::get_all(conn)
    }

    /// Get user by ID
    pub fn get_by_id(conn: &mut SqliteConnection, id: i32) -> AppResult<User> {
        if id <= 0 {
            return Err(AppError::Validation("Invalid user ID".to_string()));
        }
        UserRepository::get_by_id(conn, id)
    }

    /// Create a new user
    pub fn create(conn: &mut SqliteConnection, input: CreateUserInput) -> AppResult<User> {
        // Validate input
        Self::validate_create_input(&input)?;

        // Check if email already exists
        if UserRepository::email_exists(conn, &input.email)? {
            return Err(AppError::AlreadyExists(format!(
                "User with email '{}' already exists",
                input.email
            )));
        }

        // Create user
        UserRepository::create(conn, &input)
    }

    /// Update an existing user
    pub fn update(conn: &mut SqliteConnection, id: i32, input: UpdateUserInput) -> AppResult<User> {
        if id <= 0 {
            return Err(AppError::Validation("Invalid user ID".to_string()));
        }

        // Validate name if provided
        if let Some(ref name) = input.name {
            if name.trim().is_empty() {
                return Err(AppError::Validation("Name cannot be empty".to_string()));
            }
            if name.len() > 100 {
                return Err(AppError::Validation(
                    "Name cannot exceed 100 characters".to_string(),
                ));
            }
        }

        // Validate email if provided
        if let Some(ref email) = input.email {
            Self::validate_email(email)?;

            // Check if new email already exists (and belongs to a different user)
            if UserRepository::email_exists(conn, email)? {
                // Check if it's a different user using Diesel
                let existing_user = users::table
                    .filter(users::email.eq(email))
                    .select(users::id)
                    .first::<i32>(conn)
                    .ok();

                if let Some(existing_id) = existing_user {
                    if existing_id != id {
                        return Err(AppError::AlreadyExists(format!(
                            "Email '{}' is already in use",
                            email
                        )));
                    }
                }
            }
        }

        // Validate age if provided
        if let Some(age) = input.age {
            if age < 0 || age > 150 {
                return Err(AppError::Validation(
                    "Age must be between 0 and 150".to_string(),
                ));
            }
        }

        UserRepository::update(conn, id, &input)
    }

    /// Delete a user
    pub fn delete(conn: &mut SqliteConnection, id: i32) -> AppResult<()> {
        if id <= 0 {
            return Err(AppError::Validation("Invalid user ID".to_string()));
        }
        UserRepository::delete(conn, id)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::db::Database;
    use tempfile::tempdir;

    fn setup_test_db() -> SqliteConnection {
        let dir = tempdir().unwrap();
        let db_path = dir.path().join("test.db");
        let db = Database::new(db_path).unwrap();
        db.conn.into_inner().unwrap()
    }

    #[test]
    fn test_create_user_validation() {
        let mut conn = setup_test_db();

        // Empty name
        let input = CreateUserInput {
            name: "".to_string(),
            email: "test@example.com".to_string(),
            age: Some(25),
        };
        assert!(matches!(
            UserService::create(&mut conn, input),
            Err(AppError::Validation(_))
        ));

        // Invalid email
        let input = CreateUserInput {
            name: "John".to_string(),
            email: "invalid-email".to_string(),
            age: Some(25),
        };
        assert!(matches!(
            UserService::create(&mut conn, input),
            Err(AppError::Validation(_))
        ));

        // Invalid age
        let input = CreateUserInput {
            name: "John".to_string(),
            email: "john@example.com".to_string(),
            age: Some(200),
        };
        assert!(matches!(
            UserService::create(&mut conn, input),
            Err(AppError::Validation(_))
        ));
    }

    #[test]
    fn test_create_user_success() {
        let mut conn = setup_test_db();

        let input = CreateUserInput {
            name: "Jane Doe".to_string(),
            email: "jane@example.com".to_string(),
            age: Some(30),
        };

        let user = UserService::create(&mut conn, input).unwrap();
        assert_eq!(user.name, "Jane Doe");
        assert_eq!(user.email, "jane@example.com");
    }

    #[test]
    fn test_duplicate_email() {
        let mut conn = setup_test_db();

        let input1 = CreateUserInput {
            name: "User1".to_string(),
            email: "same@example.com".to_string(),
            age: Some(25),
        };

        UserService::create(&mut conn, input1).unwrap();

        let input2 = CreateUserInput {
            name: "User2".to_string(),
            email: "same@example.com".to_string(),
            age: Some(30),
        };

        assert!(matches!(
            UserService::create(&mut conn, input2),
            Err(AppError::AlreadyExists(_))
        ));
    }

    #[test]
    fn test_update_user_validation() {
        let mut conn = setup_test_db();

        let create_input = CreateUserInput {
            name: "Alice".to_string(),
            email: "alice@example.com".to_string(),
            age: Some(25),
        };

        let user = UserService::create(&mut conn, create_input).unwrap();
        let id = user.id;

        // Empty name update
        let update = UpdateUserInput {
            name: Some("".to_string()),
            email: None,
            age: None,
        };
        assert!(matches!(
            UserService::update(&mut conn, id, update),
            Err(AppError::Validation(_))
        ));

        // Invalid email update
        let update = UpdateUserInput {
            name: None,
            email: Some("invalid".to_string()),
            age: None,
        };
        assert!(matches!(
            UserService::update(&mut conn, id, update),
            Err(AppError::Validation(_))
        ));
    }

    #[test]
    fn test_invalid_id_validation() {
        let mut conn = setup_test_db();

        assert!(matches!(
            UserService::get_by_id(&mut conn, -1),
            Err(AppError::Validation(_))
        ));

        assert!(matches!(
            UserService::delete(&mut conn, 0),
            Err(AppError::Validation(_))
        ));
    }
}
