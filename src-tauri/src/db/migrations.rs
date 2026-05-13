use rusqlite::{Connection, Result};

/// Migration struct to hold version and SQL
pub struct Migration {
    pub version: u32,
    pub name: String,
    pub sql: String,
}

/// List of all migrations
pub fn get_migrations() -> Vec<Migration> {
    vec![
        Migration {
            version: 1,
            name: "initial_schema".to_string(),
            sql: include_str!("../../migrations/V001__initial_schema.sql").to_string(),
        },
        Migration {
            version: 2,
            name: "schema_migrations".to_string(),
            sql: include_str!("../../migrations/V002__schema_migrations.sql").to_string(),
        },
        Migration {
            version: 3,
            name: "audit_logs".to_string(),
            sql: include_str!("../../migrations/V003__audit_logs.sql").to_string(),
        },
    ]
}

/// Initialize the schema_migrations table
fn init_migrations_table(conn: &Connection) -> Result<()> {
    conn.execute(
        "CREATE TABLE IF NOT EXISTS schema_migrations (
            version INTEGER PRIMARY KEY,
            name TEXT NOT NULL,
            applied_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP
        )",
        [],
    )?;
    Ok(())
}

/// Get the current schema version
fn get_current_version(conn: &Connection) -> Result<u32> {
    match conn.query_row(
        "SELECT MAX(version) FROM schema_migrations",
        [],
        |row| row.get(0),
    ) {
        Ok(Some(version)) => Ok(version),
        Ok(None) => Ok(0),
        Err(_) => Ok(0),
    }
}

/// Check if a migration has been applied
fn is_migration_applied(conn: &Connection, version: u32) -> Result<bool> {
    let count: i64 = conn.query_row(
        "SELECT COUNT(*) FROM schema_migrations WHERE version = ?1",
        [version],
        |row| row.get(0),
    )?;
    Ok(count > 0)
}

/// Record a migration as applied
fn record_migration(conn: &Connection, version: u32, name: &str) -> Result<()> {
    conn.execute(
        "INSERT INTO schema_migrations (version, name) VALUES (?1, ?2)",
        (version, name),
    )?;
    Ok(())
}

/// Run all pending migrations
pub fn run_migrations(conn: &Connection) -> Result<()> {
    // Initialize migrations table
    init_migrations_table(conn)?;

    // Get migrations
    let migrations = get_migrations();

    // Run each migration if not already applied
    for migration in migrations {
        if !is_migration_applied(conn, migration.version)? {
            println!(
                "Running migration V{:03}: {}",
                migration.version, migration.name
            );

            // Execute the migration SQL
            conn.execute_batch(&migration.sql)?;

            // Record the migration
            record_migration(conn, migration.version, &migration.name)?;

            println!("Migration V{:03} applied successfully", migration.version);
        } else {
            println!(
                "Migration V{:03}: {} already applied",
                migration.version, migration.name
            );
        }
    }

    let current_version = get_current_version(conn)?;
    println!("Database schema is up to date (version {})", current_version);

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use rusqlite::Connection;

    #[test]
    fn test_migrations() {
        let conn = Connection::open_in_memory().unwrap();

        // Run migrations
        run_migrations(&conn).unwrap();

        // Check that migrations table exists
        let count: i64 = conn
            .query_row(
                "SELECT COUNT(*) FROM sqlite_master WHERE type='table' AND name='schema_migrations'",
                [],
                |row| row.get(0),
            )
            .unwrap();
        assert_eq!(count, 1);

        // Check that users table exists
        let count: i64 = conn
            .query_row(
                "SELECT COUNT(*) FROM sqlite_master WHERE type='table' AND name='users'",
                [],
                |row| row.get(0),
            )
            .unwrap();
        assert_eq!(count, 1);

        // Check current version
        let version = get_current_version(&conn).unwrap();
        assert_eq!(version, 3);

        // Run migrations again (should be idempotent)
        run_migrations(&conn).unwrap();
        let version = get_current_version(&conn).unwrap();
        assert_eq!(version, 3);
    }

    #[test]
    fn test_migration_list() {
        let migrations = get_migrations();
        assert_eq!(migrations.len(), 3);
        assert_eq!(migrations[0].version, 1);
        assert_eq!(migrations[1].version, 2);
        assert_eq!(migrations[2].version, 3);
    }
}
