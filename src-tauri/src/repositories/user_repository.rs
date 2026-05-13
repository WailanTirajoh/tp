use crate::db::audit::{log_audit, AuditAction};
use crate::errors::{AppError, AppResult};
use crate::models::{CreateUserInput, NewUser, UpdateUser, UpdateUserInput, User};
use crate::schema::users;
use diesel::prelude::*;
use diesel::sqlite::SqliteConnection;
use serde_json::json;

/// Repository for user data access operations
pub struct UserRepository;

impl UserRepository {
    /// Get all users ordered by ID descending
    pub fn get_all(conn: &mut SqliteConnection) -> AppResult<Vec<User>> {
        users::table
            .order(users::id.desc())
            .select(User::as_select())
            .load(conn)
            .map_err(|e| AppError::from(e))
    }

    /// Get a user by ID
    pub fn get_by_id(conn: &mut SqliteConnection, id: i32) -> AppResult<User> {
        users::table
            .find(id)
            .select(User::as_select())
            .first(conn)
            .map_err(|e| match e {
                diesel::result::Error::NotFound => {
                    AppError::NotFound(format!("User with id {} not found", id))
                }
                _ => AppError::Database(e),
            })
    }

    /// Check if email already exists
    pub fn email_exists(conn: &mut SqliteConnection, email: &str) -> AppResult<bool> {
        let count = users::table
            .filter(users::email.eq(email))
            .count()
            .get_result::<i64>(conn)?;

        Ok(count > 0)
    }

    /// Create a new user
    pub fn create(conn: &mut SqliteConnection, input: &CreateUserInput) -> AppResult<User> {
        let new_user: NewUser = input.into();

        diesel::insert_into(users::table)
            .values(&new_user)
            .execute(conn)?;

        // Get the last inserted id using Diesel's sql_query
        let last_id: i32 =
            diesel::dsl::sql::<diesel::sql_types::Integer>("SELECT last_insert_rowid()")
                .get_result(conn)?;

        let user = Self::get_by_id(conn, last_id)?;

        // Log audit entry
        let new_values = json!({
            "id": user.id,
            "name": user.name,
            "email": user.email,
            "age": user.age
        });
        log_audit(
            conn,
            "users",
            user.id,
            AuditAction::Create,
            None,
            Some(new_values),
        )?;

        Ok(user)
    }

    /// Update an existing user
    pub fn update(
        conn: &mut SqliteConnection,
        id: i32,
        input: &UpdateUserInput,
    ) -> AppResult<User> {
        // Get old values for audit
        let old_user = Self::get_by_id(conn, id)?;
        let old_values = json!({
            "name": old_user.name,
            "email": old_user.email,
            "age": old_user.age
        });

        // Check if any fields to update
        if input.name.is_none() && input.email.is_none() && input.age.is_none() {
            return Err(AppError::Validation("No fields to update".to_string()));
        }

        let update_user: UpdateUser = input.into();

        // Update with Diesel - also update the updated_at timestamp
        diesel::update(users::table.find(id))
            .set((
                &update_user,
                users::updated_at.eq(diesel::dsl::sql("CURRENT_TIMESTAMP")),
            ))
            .execute(conn)
            .map_err(|e| match e {
                diesel::result::Error::NotFound => {
                    AppError::NotFound(format!("User with id {} not found", id))
                }
                _ => AppError::Database(e),
            })?;

        let updated_user = Self::get_by_id(conn, id)?;

        // Log audit entry
        let new_values = json!({
            "name": updated_user.name,
            "email": updated_user.email,
            "age": updated_user.age
        });
        log_audit(
            conn,
            "users",
            id,
            AuditAction::Update,
            Some(old_values),
            Some(new_values),
        )?;

        Ok(updated_user)
    }

    /// Delete a user by ID
    pub fn delete(conn: &mut SqliteConnection, id: i32) -> AppResult<()> {
        // Get old values for audit before deleting
        let old_user = Self::get_by_id(conn, id)?;
        let old_values = json!({
            "id": old_user.id,
            "name": old_user.name,
            "email": old_user.email,
            "age": old_user.age
        });

        let rows_affected = diesel::delete(users::table.find(id)).execute(conn)?;

        if rows_affected == 0 {
            return Err(AppError::NotFound(format!("User with id {} not found", id)));
        }

        // Log audit entry
        log_audit(
            conn,
            "users",
            id,
            AuditAction::Delete,
            Some(old_values),
            None,
        )?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::db::Database;
    use diesel::prelude::*;
    use tempfile::tempdir;

    fn setup_test_db() -> SqliteConnection {
        let dir = tempdir().unwrap();
        let db_path = dir.path().join("test.db");
        let db = Database::new(db_path).unwrap();
        db.conn.into_inner().unwrap()
    }

    #[test]
    fn test_create_and_get_user() {
        let mut conn = setup_test_db();
        let input = CreateUserInput {
            name: "John Doe".to_string(),
            email: "john@example.com".to_string(),
            age: Some(30),
        };

        let user = UserRepository::create(&mut conn, &input).unwrap();
        assert_eq!(user.name, "John Doe");
        assert_eq!(user.email, "john@example.com");

        let fetched = UserRepository::get_by_id(&mut conn, user.id).unwrap();
        assert_eq!(fetched.name, user.name);
    }

    #[test]
    fn test_email_exists() {
        let mut conn = setup_test_db();
        let input = CreateUserInput {
            name: "Jane".to_string(),
            email: "jane@example.com".to_string(),
            age: None,
        };

        UserRepository::create(&mut conn, &input).unwrap();
        assert!(UserRepository::email_exists(&mut conn, "jane@example.com").unwrap());
        assert!(!UserRepository::email_exists(&mut conn, "other@example.com").unwrap());
    }

    #[test]
    fn test_update_user() {
        let mut conn = setup_test_db();
        let input = CreateUserInput {
            name: "Alice".to_string(),
            email: "alice@example.com".to_string(),
            age: Some(25),
        };

        let user = UserRepository::create(&mut conn, &input).unwrap();
        let update = UpdateUserInput {
            name: Some("Alice Updated".to_string()),
            email: None,
            age: None,
        };

        let updated = UserRepository::update(&mut conn, user.id, &update).unwrap();
        assert_eq!(updated.name, "Alice Updated");
        assert_eq!(updated.email, "alice@example.com");
    }

    #[test]
    fn test_delete_user() {
        let mut conn = setup_test_db();
        let input = CreateUserInput {
            name: "Bob".to_string(),
            email: "bob@example.com".to_string(),
            age: None,
        };

        let user = UserRepository::create(&mut conn, &input).unwrap();
        let id = user.id;

        UserRepository::delete(&mut conn, id).unwrap();

        let result = UserRepository::get_by_id(&mut conn, id);
        assert!(matches!(result, Err(AppError::NotFound(_))));
    }

    #[test]
    fn test_get_all_users() {
        let mut conn = setup_test_db();

        let input1 = CreateUserInput {
            name: "User1".to_string(),
            email: "user1@example.com".to_string(),
            age: Some(20),
        };
        let input2 = CreateUserInput {
            name: "User2".to_string(),
            email: "user2@example.com".to_string(),
            age: Some(30),
        };

        UserRepository::create(&mut conn, &input1).unwrap();
        UserRepository::create(&mut conn, &input2).unwrap();

        let users = UserRepository::get_all(&mut conn).unwrap();
        assert_eq!(users.len(), 2);
        assert_eq!(users[0].name, "User2"); // Ordered by ID DESC
        assert_eq!(users[1].name, "User1");
    }
}
