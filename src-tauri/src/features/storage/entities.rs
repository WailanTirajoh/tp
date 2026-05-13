use serde::{Deserialize, Serialize};
use serde_json::Value;

/// A document stored in the reactive storage system
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Document {
    /// Unique key for this document
    pub key: String,

    /// Optional collection name
    pub collection: Option<String>,

    /// JSON value of the document
    pub value: Value,

    /// Monotonic version number (increments on every change)
    pub version: u64,

    /// ISO8601 timestamp of creation
    pub created_at: String,

    /// ISO8601 timestamp of last update
    pub updated_at: String,
}

/// Event emitted when storage changes
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StorageChangeEvent {
    /// Document key that changed
    pub key: String,

    /// Optional collection name
    pub collection: Option<String>,

    /// New value of the document
    pub value: Value,

    /// New version number
    pub version: u64,

    /// ISO8601 timestamp
    pub timestamp: String,

    /// Origin window/source identifier
    pub origin: String,

    /// Type of change (set, delete)
    pub change_type: ChangeType,
}

/// Type of storage change
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ChangeType {
    Set,
    Delete,
}

/// Request to set a document
#[derive(Debug, Deserialize)]
pub struct SetDocumentRequest {
    pub key: String,
    pub value: Value,
    pub collection: Option<String>,
    pub expected_version: Option<u64>, // For optimistic concurrency
}

/// Request to get a document
#[derive(Debug, Deserialize)]
pub struct GetDocumentRequest {
    pub key: String,
}

/// Request to delete a document
#[derive(Debug, Deserialize)]
pub struct DeleteDocumentRequest {
    pub key: String,
    pub expected_version: Option<u64>,
}

/// Request to query documents by collection
#[derive(Debug, Deserialize)]
pub struct QueryCollectionRequest {
    pub collection: String,
    pub limit: Option<usize>,
    pub offset: Option<usize>,
}

/// Response for a collection query
#[derive(Debug, Serialize)]
pub struct CollectionQueryResponse {
    pub documents: Vec<Document>,
    pub total: usize,
    pub has_more: bool,
}
