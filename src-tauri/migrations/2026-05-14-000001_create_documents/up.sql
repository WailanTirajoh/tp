-- Create documents table for reactive storage
CREATE TABLE documents (
    key TEXT PRIMARY KEY NOT NULL,
    collection TEXT,
    value TEXT NOT NULL,
    version INTEGER NOT NULL DEFAULT 1,
    created_at TEXT NOT NULL DEFAULT (datetime('now')),
    updated_at TEXT NOT NULL DEFAULT (datetime('now'))
);

-- Index for collection queries
CREATE INDEX idx_documents_collection ON documents(collection)
WHERE
    collection IS NOT NULL;

-- Index for timestamp-based queries
CREATE INDEX idx_documents_updated ON documents(updated_at);

-- Index for collection + timestamp queries
CREATE INDEX idx_documents_collection_updated ON documents(collection, updated_at)
WHERE
    collection IS NOT NULL;