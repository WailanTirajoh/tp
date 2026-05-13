use crate::schema::documents;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

/// Persisted document in SQLite
#[derive(Debug, Clone, Queryable, Selectable, Serialize, Deserialize)]
#[diesel(table_name = documents)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct PersistedDocument {
    pub key: String,
    pub collection: Option<String>,
    pub value: String, // JSON string
    pub version: i64,
    pub created_at: String,
    pub updated_at: String,
}

/// New document for insertion
#[derive(Debug, Insertable)]
#[diesel(table_name = documents)]
pub struct NewDocument {
    pub key: String,
    pub collection: Option<String>,
    pub value: String,
    pub version: i64,
    pub created_at: String,
    pub updated_at: String,
}

/// Document update
#[derive(Debug, AsChangeset)]
#[diesel(table_name = documents)]
pub struct UpdateDocument {
    pub value: String,
    pub version: i64,
    pub updated_at: String,
}
