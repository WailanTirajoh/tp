use super::entities::Document;
use super::persistence::{NewDocument, PersistedDocument, UpdateDocument};
use crate::core::{AppError, AppResult, Database};
use crate::schema::documents;
use diesel::prelude::*;
use std::sync::Arc;

/// Repository for persisting documents to SQLite
pub struct StorageRepository;

impl StorageRepository {
    /// Load all documents from database (on startup)
    pub fn load_all(database: &Arc<Database>) -> AppResult<Vec<Document>> {
        let mut conn = database.conn.lock().map_err(|e| AppError::Internal(e.to_string()))?;
        
        let persisted: Vec<PersistedDocument> = documents::table
            .order(documents::updated_at.desc())
            .select(PersistedDocument::as_select())
            .load(&mut *conn)?;

        let mut result = Vec::new();
        for doc in persisted {
            let value: serde_json::Value = serde_json::from_str(&doc.value)?;
            result.push(Document {
                key: doc.key,
                collection: doc.collection,
                value,
                version: doc.version as u64,
                created_at: doc.created_at,
                updated_at: doc.updated_at,
            });
        }

        Ok(result)
    }

    /// Persist a single document
    pub fn persist(database: &Arc<Database>, document: &Document) -> AppResult<()> {
        let mut conn = database.conn.lock().map_err(|e| AppError::Internal(e.to_string()))?;
        
        let value_json = serde_json::to_string(&document.value)?;
        
        // Try to update existing document
        let updated = diesel::update(documents::table.find(&document.key))
            .set(UpdateDocument {
                value: value_json.clone(),
                version: document.version as i64,
                updated_at: document.updated_at.clone(),
            })
            .execute(&mut *conn)?;

        // If no rows updated, insert new document
        if updated == 0 {
            let new_doc = NewDocument {
                key: document.key.clone(),
                collection: document.collection.clone(),
                value: value_json,
                version: document.version as i64,
                created_at: document.created_at.clone(),
                updated_at: document.updated_at.clone(),
            };

            diesel::insert_into(documents::table)
                .values(&new_doc)
                .execute(&mut *conn)?;
        }

        Ok(())
    }

    /// Persist multiple documents in a batch
    pub fn persist_batch(database: &Arc<Database>, documents: &[Document]) -> AppResult<()> {
        let mut conn = database.conn.lock().map_err(|e| AppError::Internal(e.to_string()))?;
        
        conn.transaction::<_, AppError, _>(|conn| {
            for document in documents {
                let value_json = serde_json::to_string(&document.value)?;
                
                let updated = diesel::update(documents::table.find(&document.key))
                    .set(UpdateDocument {
                        value: value_json.clone(),
                        version: document.version as i64,
                        updated_at: document.updated_at.clone(),
                    })
                    .execute(conn)?;

                if updated == 0 {
                    let new_doc = NewDocument {
                        key: document.key.clone(),
                        collection: document.collection.clone(),
                        value: value_json,
                        version: document.version as i64,
                        created_at: document.created_at.clone(),
                        updated_at: document.updated_at.clone(),
                    };

                    diesel::insert_into(documents::table)
                        .values(&new_doc)
                        .execute(conn)?;
                }
            }
            Ok(())
        })
    }

    /// Delete a document from database
    pub fn delete(database: &Arc<Database>, key: &str) -> AppResult<()> {
        let mut conn = database.conn.lock().map_err(|e| AppError::Internal(e.to_string()))?;
        
        diesel::delete(documents::table.find(key))
            .execute(&mut *conn)?;

        Ok(())
    }

    /// Get a single document from database
    pub fn get(database: &Arc<Database>, key: &str) -> AppResult<Option<Document>> {
        let mut conn = database.conn.lock().map_err(|e| AppError::Internal(e.to_string()))?;
        
        let result: Option<PersistedDocument> = documents::table
            .find(key)
            .select(PersistedDocument::as_select())
            .first(&mut *conn)
            .optional()?;

        if let Some(doc) = result {
            let value: serde_json::Value = serde_json::from_str(&doc.value)?;
            Ok(Some(Document {
                key: doc.key,
                collection: doc.collection,
                value,
                version: doc.version as u64,
                created_at: doc.created_at,
                updated_at: doc.updated_at,
            }))
        } else {
            Ok(None)
        }
    }
}
