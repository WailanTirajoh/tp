use super::entities::{ChangeType, Document, StorageChangeEvent};
use crate::core::{AppError, AppResult};
use parking_lot::RwLock;
use serde_json::Value;
use std::collections::HashMap;
use std::sync::Arc;
use tauri::{AppHandle, Emitter};

/// In-memory reactive storage manager
pub struct StorageManager {
    /// In-memory document store
    store: Arc<RwLock<HashMap<String, Document>>>,
    
    /// App handle for event emission
    app_handle: AppHandle,
}

impl StorageManager {
    /// Create a new storage manager
    pub fn new(app_handle: AppHandle) -> Self {
        Self {
            store: Arc::new(RwLock::new(HashMap::new())),
            app_handle,
        }
    }

    /// Get a document by key
    pub fn get(&self, key: &str) -> Option<Document> {
        let store = self.store.read();
        store.get(key).cloned()
    }

    /// Set a document
    pub fn set(
        &self,
        key: String,
        value: Value,
        collection: Option<String>,
        expected_version: Option<u64>,
        origin: String,
    ) -> AppResult<Document> {
        let now = chrono::Utc::now().to_rfc3339();
        
        let mut store = self.store.write();
        
        // Check optimistic concurrency if version is provided
        if let Some(expected) = expected_version {
            if let Some(existing) = store.get(&key) {
                if existing.version != expected {
                    return Err(AppError::Validation(format!(
                        "Version mismatch: expected {}, got {}",
                        expected, existing.version
                    )));
                }
            }
        }

        // Update or create document
        let document = if let Some(mut existing) = store.get(&key).cloned() {
            existing.value = value.clone();
            existing.version += 1;
            existing.updated_at = now.clone();
            if collection.is_some() {
                existing.collection = collection.clone();
            }
            existing
        } else {
            Document {
                key: key.clone(),
                collection: collection.clone(),
                value: value.clone(),
                version: 1,
                created_at: now.clone(),
                updated_at: now.clone(),
            }
        };

        store.insert(key.clone(), document.clone());
        drop(store); // Release lock before emitting event

        // Emit change event to all windows
        let event = StorageChangeEvent {
            key: key.clone(),
            collection: collection.clone(),
            value: value.clone(),
            version: document.version,
            timestamp: now,
            origin,
            change_type: ChangeType::Set,
        };

        let _ = self.app_handle.emit("storage:changed", &event);

        Ok(document)
    }

    /// Delete a document
    pub fn delete(
        &self,
        key: String,
        expected_version: Option<u64>,
        origin: String,
    ) -> AppResult<()> {
        let now = chrono::Utc::now().to_rfc3339();
        
        let mut store = self.store.write();
        
        // Check if document exists
        let existing = store.get(&key).ok_or_else(|| {
            AppError::NotFound(format!("Document with key '{}' not found", key))
        })?;

        // Check optimistic concurrency if version is provided
        if let Some(expected) = expected_version {
            if existing.version != expected {
                return Err(AppError::Validation(format!(
                    "Version mismatch: expected {}, got {}",
                    expected, existing.version
                )));
            }
        }

        let collection = existing.collection.clone();
        store.remove(&key);
        drop(store);

        // Emit delete event
        let event = StorageChangeEvent {
            key: key.clone(),
            collection,
            value: Value::Null,
            version: 0,
            timestamp: now,
            origin,
            change_type: ChangeType::Delete,
        };

        let _ = self.app_handle.emit("storage:changed", &event);

        Ok(())
    }

    /// Get all documents in a collection
    pub fn get_collection(&self, collection: &str, limit: Option<usize>, offset: Option<usize>) -> Vec<Document> {
        let store = self.store.read();
        
        let mut documents: Vec<Document> = store
            .values()
            .filter(|doc| doc.collection.as_deref() == Some(collection))
            .cloned()
            .collect();

        // Sort by updated_at descending
        documents.sort_by(|a, b| b.updated_at.cmp(&a.updated_at));

        // Apply pagination
        let offset = offset.unwrap_or(0);
        let documents = documents.into_iter().skip(offset);
        
        if let Some(limit) = limit {
            documents.take(limit).collect()
        } else {
            documents.collect()
        }
    }

    /// Get total count of documents in a collection
    pub fn count_collection(&self, collection: &str) -> usize {
        let store = self.store.read();
        store
            .values()
            .filter(|doc| doc.collection.as_deref() == Some(collection))
            .count()
    }

    /// Get all documents (for persistence)
    pub fn get_all(&self) -> Vec<Document> {
        let store = self.store.read();
        store.values().cloned().collect()
    }

    /// Load documents from persistence layer (called at startup)
    pub fn load_documents(&self, documents: Vec<Document>) {
        let mut store = self.store.write();
        for doc in documents {
            store.insert(doc.key.clone(), doc);
        }
    }

    /// Clear all documents (for testing)
    pub fn clear(&self) {
        let mut store = self.store.write();
        store.clear();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    // Note: Tests requiring AppHandle are integration tests
    // Unit tests here would need mocking
}
