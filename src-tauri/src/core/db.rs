use diesel::prelude::*;
use diesel::sqlite::SqliteConnection;
use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};
use std::path::PathBuf;
use std::sync::Mutex;

pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!("migrations");

pub struct Database {
    pub conn: Mutex<SqliteConnection>,
}

impl Database {
    pub fn new(db_path: PathBuf) -> Result<Self, Box<dyn std::error::Error + Send + Sync>> {
        let database_url = db_path.to_str().ok_or("Invalid database path")?;
        let mut conn = SqliteConnection::establish(database_url)?;

        // Enable WAL mode for better concurrency and foreign keys
        diesel::sql_query("PRAGMA journal_mode=WAL").execute(&mut conn)?;
        diesel::sql_query("PRAGMA foreign_keys=ON").execute(&mut conn)?;

        // Run migrations
        conn.run_pending_migrations(MIGRATIONS)
            .map_err(|e| format!("Migration error: {}", e))?;

        Ok(Database {
            conn: Mutex::new(conn),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    #[test]
    fn test_database_initialization() {
        let dir = tempdir().unwrap();
        let db_path = dir.path().join("test.db");

        let db = Database::new(db_path.clone()).unwrap();
        let mut conn = db.conn.lock().unwrap();

        // Check that users table exists
        let table_exists = diesel::dsl::sql::<diesel::sql_types::BigInt>(
            "SELECT COUNT(*) FROM sqlite_master WHERE type='table' AND name='users'",
        )
        .get_result::<i64>(&mut *conn)
        .unwrap();

        assert_eq!(table_exists, 1);
    }
}
