# Noken POS

A modern Point of Sale application built with Tauri + Nuxt, featuring a reactive document storage system.

## 🏗️ Architecture Philosophy

**Backend = Persistence Transport Only**  
The Rust backend is intentionally kept agnostic and minimal. It only handles:
- ✅ Document storage (key-value)
- ✅ SQLite persistence
- ✅ Multi-window event synchronization

**Frontend = Business Logic**  
All business logic lives in the Vue/Nuxt frontend:
- 🎯 User management
- 🎯 Transaction processing
- 🎯 Validation rules
- 🎯 Application state
- 🎯 UI workflows

## 🚀 Quick Start

### Prerequisites
- Node.js 18+ and pnpm
- Rust 1.70+
- Tauri CLI

### Installation

```bash
# Install dependencies
pnpm install

# Install Tauri CLI
pnpm add -D @tauri-apps/cli

# Run development server
pnpm tauri dev
```

### Build

```bash
# Build for production
pnpm tauri build
```

## 📚 Tech Stack

### Frontend
- **Nuxt 3** - Vue framework with SSR
- **Vue 3** - Composition API
- **TypeScript** - Type safety
- **Tailwind CSS** - Utility-first CSS
- **VueUse** - Vue composition utilities
- **Pinia** - State management

### Backend
- **Tauri 2** - Native desktop framework
- **Rust** - Systems programming language
- **Diesel 2** - Type-safe ORM
- **SQLite** - Embedded database (persistence only)

## 💾 Reactive Storage System

### Concept

SQLite is **not** the source of truth. It's purely a persistence layer.

```
┌─────────────────────────────────────────────┐
│           Vue Frontend (Source of Truth)     │
│  ┌────────────────────────────────────────┐ │
│  │  useDocumentStorage('user', defaultValue)│ │
│  │  ↓                                      │ │
│  │  Reactive Ref<T>                       │ │
│  └────────────────────────────────────────┘ │
│              ↕ (Tauri IPC)                   │
├─────────────────────────────────────────────┤
│        Rust Backend (Transport Only)         │
│  ┌────────────────────────────────────────┐ │
│  │  StorageManager (In-Memory HashMap)    │ │
│  │  ↓ (Async, Debounced)                  │ │
│  │  SQLite (Persistence Snapshot)         │ │
│  └────────────────────────────────────────┘ │
└─────────────────────────────────────────────┘
```

### Usage

```ts
import { useDocumentStorage } from '@/composables/useDocumentStorage'

// Use it like VueUse's useLocalStorage
const user = useDocumentStorage('user', { 
  name: 'Guest',
  email: 'guest@example.com'
})

// Changes automatically sync to Rust + SQLite
user.value.name = 'Wailan'

// Multi-window sync is automatic
// Changes in one window appear in all windows instantly
```

### API

```ts
// Get/set reactive document
useDocumentStorage<T>(key: string, initialValue?: T, options?: StorageOptions)

// Remove document
removeDocument(key: string): Promise<void>

// Get all keys (debugging)
getDocumentKeys(): Promise<string[]>

// Get registry keys (local)
getRegistryKeys(): string[]
```

### Options

```ts
interface StorageOptions {
  collection?: string      // Group documents (e.g., 'users', 'products')
  debounce?: number        // Delay before persisting (default: 300ms)
  shallow?: boolean        // Shallow reactivity (default: false)
  manual?: boolean         // Disable auto-listening (default: false)
  onError?: (error: Error) => void  // Error handler
}
```

## 🗄️ Database Structure

### Tables

#### `documents`
The only table in the database. Stores all application data as JSON documents.

```sql
CREATE TABLE documents (
    key TEXT PRIMARY KEY,           -- Unique document key
    collection TEXT,                -- Optional grouping (e.g., 'users')
    value TEXT NOT NULL,            -- JSON document
    version INTEGER NOT NULL,       -- Optimistic concurrency control
    created_at TEXT NOT NULL,       -- ISO 8601 timestamp
    updated_at TEXT NOT NULL        -- ISO 8601 timestamp
);
```

### Indexes
- `idx_documents_collection` - Fast collection queries
- `idx_documents_updated` - Recent changes tracking
- `idx_documents_collection_updated` - Combined collection + time queries

### Features
- ✅ WAL mode enabled (better concurrency)
- ✅ Foreign keys enabled
- ✅ Automatic timestamps
- ✅ Version-based conflict detection

## 📖 Documentation

- [REACTIVE-ARCHITECTURE.md](REACTIVE-ARCHITECTURE.md) - Architecture overview
- [REACTIVE-STORAGE-GUIDE.md](REACTIVE-STORAGE-GUIDE.md) - Complete implementation guide
- [QUICK-START.md](QUICK-START.md) - Quick reference
- [GET-DATABASE-PATH.md](GET-DATABASE-PATH.md) - Database validation guide

## 🔧 Development

### Project Structure

```
noken-pos/
├── app/                        # Nuxt frontend
│   ├── composables/
│   │   └── useDocumentStorage.ts  # Main document storage composable
│   ├── types/
│   │   └── storage.ts         # TypeScript types
│   ├── utils/
│   │   └── storageRegistry.ts # Registry manager
│   └── pages/
│       └── index.vue          # Home page
│
├── src-tauri/                 # Rust backend
│   ├── migrations/
│   │   └── 2026-05-14-000001_create_documents/  # Only migration
│   └── src/
│       ├── core/
│       │   ├── db.rs         # SQLite connection
│       │   └── errors.rs     # Error types
│       └── features/
│           └── storage/      # Storage feature
│               ├── commands.rs      # Tauri commands
│               ├── entities.rs      # Data types
│               ├── manager.rs       # In-memory store
│               ├── persistence.rs   # Diesel models
│               └── repository.rs    # Database operations
│
├── nuxt.config.ts
├── package.json
└── README.md
```

### Tauri Commands

```rust
storage_get              // Get document by key
storage_set              // Set/update document
storage_delete           // Delete document
storage_query_collection // Query documents by collection
storage_keys             // Get all keys (debugging)
```

### Frontend Composables

```ts
useDocumentStorage()  // Main reactive document storage
removeDocument()      // Remove document
getDocumentKeys()     // Get all keys
getRegistryKeys()     // Get local registry keys
```

## 🧪 Testing

### Check SQLite Database

```bash
# Get database path
open ~/Library/Application\ Support/com.wailan.noken-pos/

# Connect with DBeaver or any SQLite client
# Path: ~/Library/Application Support/com.wailan.noken-pos/database.sqlite
```

### Example Queries

```sql
-- View all documents
SELECT * FROM documents ORDER BY updated_at DESC;

-- View by collection
SELECT * FROM documents WHERE collection = 'users';

-- Document count
SELECT collection, COUNT(*) as count 
FROM documents 
GROUP BY collection;
```

## 📝 License

MIT

## 🤝 Contributing

Contributions are welcome! Please read the documentation first to understand the architecture philosophy.
