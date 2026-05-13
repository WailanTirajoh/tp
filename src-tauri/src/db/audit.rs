use diesel::prelude::*;
use diesel::sqlite::SqliteConnection;
use serde_json::Value;
use crate::schema::audit_logs;

/// Audit action types
#[derive(Debug, Clone, Copy)]
pub enum AuditAction {
    Create,
    Update,
    Delete,
}

impl AuditAction {
    pub fn as_str(&self) -> &'static str {
        match self {
            AuditAction::Create => "CREATE",
            AuditAction::Update => "UPDATE",
            AuditAction::Delete => "DELETE",
        }
    }
}

/// Insertable audit log entry
#[derive(Insertable)]
#[diesel(table_name = audit_logs)]
struct NewAuditLog<'a> {
    table_name: &'a str,
    record_id: i32,
    action: &'a str,
    old_values: Option<String>,
    new_values: Option<String>,
}

/// Queryable audit log entry
#[derive(Debug, Queryable, Selectable)]
#[diesel(table_name = audit_logs)]
pub struct AuditLogEntry {
    pub id: i32,
    pub table_name: String,
    pub record_id: i32,
    pub action: String,
    pub old_values: Option<String>,
    pub new_values: Option<String>,
    pub user_id: Option<i32>,
    pub created_at: String,
}

/// Log an audit entry
pub fn log_audit(
    conn: &mut SqliteConnection,
    table_name: &str,
    record_id: i32,
    action: AuditAction,
    old_values: Option<Value>,
    new_values: Option<Value>,
) -> Result<(), diesel::result::Error> {
    let old_json = old_values.map(|v| v.to_string());
    let new_json = new_values.map(|v| v.to_string());

    let new_log = NewAuditLog {
        table_name,
        record_id,
        action: action.as_str(),
        old_values: old_json,
        new_values: new_json,
    };

    diesel::insert_into(audit_logs::table)
        .values(&new_log)
        .execute(conn)?;

    Ok(())
}

/// Get audit history for a specific record
pub fn get_audit_history(
    conn: &mut SqliteConnection,
    table_name: &str,
    record_id: i32,
) -> Result<Vec<AuditLogEntry>, diesel::result::Error> {
    audit_logs::table
        .filter(audit_logs::table_name.eq(table_name))
        .filter(audit_logs::record_id.eq(record_id))
        .order(audit_logs::id.desc())
        .select(AuditLogEntry::as_select())
        .load(conn)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::db::Database;
    use diesel::prelude::*;
    use serde_json::json;
    use tempfile::tempdir;

    fn setup_test_db() -> SqliteConnection {
        let dir = tempdir().unwrap();
        let db_path = dir.path().join("test.db");
        let db = Database::new(db_path).unwrap();
        db.conn.into_inner().unwrap()
    }

    #[test]
    fn test_log_audit_create() {
        let mut conn = setup_test_db();

        let new_values = json!({
            "name": "John Doe",
            "email": "john@example.com",
            "age": 30
        });

        log_audit(
            &mut conn,
            "users",
            1,
            AuditAction::Create,
            None,
            Some(new_values),
        )
        .unwrap();

        let history = get_audit_history(&mut conn, "users", 1).unwrap();
        assert_eq!(history.len(), 1);
        assert_eq!(history[0].action, "CREATE");
        assert!(history[0].new_values.is_some());
        assert!(history[0].old_values.is_none());
    }

    #[test]
    fn test_log_audit_update() {
        let mut conn = setup_test_db();

        let old_values = json!({"name": "John", "age": 30});
        let new_values = json!({"name": "John Doe", "age": 31});

        log_audit(
            &mut conn,
            "users",
            1,
            AuditAction::Update,
            Some(old_values),
            Some(new_values),
        )
        .unwrap();

        let history = get_audit_history(&mut conn, "users", 1).unwrap();
        assert_eq!(history.len(), 1);
        assert_eq!(history[0].action, "UPDATE");
        assert!(history[0].old_values.is_some());
        assert!(history[0].new_values.is_some());
    }

    #[test]
    fn test_log_audit_delete() {
        let mut conn = setup_test_db();

        let old_values = json!({"name": "John Doe", "email": "john@example.com"});

        log_audit(
            &mut conn,
            "users",
            1,
            AuditAction::Delete,
            Some(old_values),
            None,
        )
        .unwrap();

        let history = get_audit_history(&mut conn, "users", 1).unwrap();
        assert_eq!(history.len(), 1);
        assert_eq!(history[0].action, "DELETE");
        assert!(history[0].old_values.is_some());
        assert!(history[0].new_values.is_none());
    }

    #[test]
    fn test_audit_history_multiple_entries() {
        let mut conn = setup_test_db();

        // Create
        log_audit(
            &mut conn,
            "users",
            1,
            AuditAction::Create,
            None,
            Some(json!({"name": "John"})),
        )
        .unwrap();

        // Update
        log_audit(
            &mut conn,
            "users",
            1,
            AuditAction::Update,
            Some(json!({"name": "John"})),
            Some(json!({"name": "John Doe"})),
        )
        .unwrap();

        // Delete
        log_audit(
            &mut conn,
            "users",
            1,
            AuditAction::Delete,
            Some(json!({"name": "John Doe"})),
            None,
        )
        .unwrap();

        let history = get_audit_history(&mut conn, "users", 1).unwrap();
        assert_eq!(history.len(), 3);
        // Should be in DESC order (newest first)
        assert_eq!(history[0].action, "DELETE");
        assert_eq!(history[1].action, "UPDATE");
        assert_eq!(history[2].action, "CREATE");
    }
}
