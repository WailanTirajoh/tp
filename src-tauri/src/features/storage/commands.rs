use super::entities::{
    CollectionQueryResponse, DeleteDocumentRequest, Document,
    QueryCollectionRequest, SetDocumentRequest,
};
use super::manager::StorageManager;
use super::repository::StorageRepository;
use crate::core::Database;
use std::sync::Arc;
use tauri::State;

/// Get a document by key
#[tauri::command]
pub async fn storage_get(
    key: String,
    storage: State<'_, Arc<StorageManager>>,
) -> Result<Option<Document>, String> {
    Ok(storage.get(&key))
}

/// Set a document
#[tauri::command]
pub async fn storage_set(
    request: SetDocumentRequest,
    storage: State<'_, Arc<StorageManager>>,
    database: State<'_, Arc<Database>>,
    origin: String,
) -> Result<Document, String> {
    // Update in-memory store and emit event
    let document = storage
        .set(
            request.key,
            request.value,
            request.collection,
            request.expected_version,
            origin,
        )
        .map_err(|e| e.to_string())?;

    // Persist to SQLite asynchronously (in production, use debounced queue)
    let db = database.inner().clone();
    let doc = document.clone();
    tokio::spawn(async move {
        let _ = StorageRepository::persist(&db, &doc);
    });

    Ok(document)
}

/// Delete a document
#[tauri::command]
pub async fn storage_delete(
    request: DeleteDocumentRequest,
    storage: State<'_, Arc<StorageManager>>,
    database: State<'_, Arc<Database>>,
    origin: String,
) -> Result<(), String> {
    // Delete from in-memory store and emit event
    storage
        .delete(request.key.clone(), request.expected_version, origin)
        .map_err(|e| e.to_string())?;

    // Delete from SQLite asynchronously
    let db = database.inner().clone();
    let key = request.key.clone();
    tokio::spawn(async move {
        let _ = StorageRepository::delete(&db, &key);
    });

    Ok(())
}

/// Query documents by collection
#[tauri::command]
pub async fn storage_query_collection(
    request: QueryCollectionRequest,
    storage: State<'_, Arc<StorageManager>>,
) -> Result<CollectionQueryResponse, String> {
    let total = storage.count_collection(&request.collection);
    let documents = storage.get_collection(
        &request.collection,
        request.limit,
        request.offset,
    );

    let _limit = request.limit.unwrap_or(usize::MAX);
    let offset = request.offset.unwrap_or(0);
    let has_more = offset + documents.len() < total;

    Ok(CollectionQueryResponse {
        documents,
        total,
        has_more,
    })
}

/// Get all keys (for debugging)
#[tauri::command]
pub async fn storage_keys(storage: State<'_, Arc<StorageManager>>) -> Result<Vec<String>, String> {
    let documents = storage.get_all();
    Ok(documents.into_iter().map(|d| d.key).collect())
}

/// Clear all storage (for testing)
#[cfg(debug_assertions)]
#[tauri::command]
pub async fn storage_clear(
    storage: State<'_, Arc<StorageManager>>,
    database: State<'_, Arc<Database>>,
) -> Result<(), String> {
    storage.clear();
    
    let db = database.inner().clone();
    tokio::spawn(async move {
        use diesel::RunQueryDsl;
        if let Ok(mut conn) = db.conn.lock() {
            let _ = diesel::delete(crate::schema::documents::table).execute(&mut *conn);
        }
    });

    Ok(())
}
