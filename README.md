# Nuxt + Tauri Reactive Storage Boilerplate

<div align="center">

![Tauri](https://img.shields.io/badge/Tauri-2.0-FFC131?style=flat-square&logo=tauri)
![Nuxt](https://img.shields.io/badge/Nuxt-3.0-00DC82?style=flat-square&logo=nuxt.js)
![Vue](https://img.shields.io/badge/Vue-3.0-4FC08D?style=flat-square&logo=vue.js)
![TypeScript](https://img.shields.io/badge/TypeScript-5.0-3178C6?style=flat-square&logo=typescript)
![Rust](https://img.shields.io/badge/Rust-1.70+-000000?style=flat-square&logo=rust)
![License](https://img.shields.io/badge/License-MIT-green?style=flat-square)

**A production-ready boilerplate for building cross-platform desktop applications**

[Features](#-key-features) • [Quick Start](#-quick-start) • [Documentation](#-documentation) • [Examples](#-common-patterns) • [FAQ](#-faq)

</div>

---

## 🎯 What is This?

A **starter template** that combines the best of web and native development:

```
🌐 Nuxt 3 (Frontend)          🦀 Tauri 2 (Native)
  ├─ Vue 3 + Composition API    ├─ Rust backend
  ├─ TypeScript                 ├─ SQLite storage
  ├─ VueUse utilities           ├─ Native APIs
  ├─ Tailwind CSS               └─ Cross-platform
  └─ Reactive storage                ↓
        ↓                         Windows
    🔄 IPC Bridge                 macOS
        ↓                         Linux
    📦 Reactive Storage System
```

This boilerplate provides:

- ✅ **Fully configured** Nuxt 3 + Tauri 2 setup (no configuration needed)
- ✅ **Reactive local-first storage** system (VueUse-style API + SQLite)
- ✅ **TypeScript** throughout (frontend & type-safe Rust)
- ✅ **Production-ready architecture** with clear separation of concerns
- ✅ **Multi-window synchronization** out of the box
- ✅ **Feature-based project structure** that scales
- ✅ **Hot reload** development experience
- ✅ **Build scripts** for Windows, macOS, and Linux

## 🚀 Use Cases

This boilerplate is perfect for building:

| Scenario | Why This Boilerplate Works |
|----------|---------------------------|
| **POS Systems** | Local-first storage, offline-capable, fast reactive UI |
| **Admin Dashboards** | Electron alternative with native performance |
| **Desktop Tools** | Settings persistence, multi-window support, native OS integration |
| **CRM Applications** | Document-based storage for contacts, activities, notes |
| **Inventory Management** | Reactive product catalog, real-time updates across windows |
| **Note-Taking Apps** | Document storage, instant sync, local-first architecture |
| **Internal Business Tools** | Quick development with full-stack TypeScript patterns |
| **Offline-First Apps** | SQLite persistence, no backend required |

## � Why Choose This Boilerplate?

### vs. Electron Apps

| Aspect | This Boilerplate (Tauri) | Electron |
|--------|--------------------------|----------|
| **App Size** | 3-10 MB | 50-150 MB |
| **Memory** | 50-100 MB | 200-500 MB |
| **Startup** | < 1 second | 2-5 seconds |
| **Updates** | Native webview updates with OS | Bundle entire Chromium |
| **Security** | Allowlist API access | Full Node.js access |

### vs. Web Apps

| Aspect | This Boilerplate | Web App |
|--------|------------------|---------|
| **Offline** | ✅ Works offline (SQLite) | ❌ Requires server |
| **Performance** | ✅ Native (Rust + OS webview) | ⚠️ Network dependent |
| **File System** | ✅ Full access via Tauri API | ❌ Limited |
| **Distribution** | ✅ Single executable | ❌ Deploy + maintain server |
| **Cost** | ✅ One-time build | 💰 Hosting + maintenance |

### vs. Starting from Scratch

| Aspect | This Boilerplate | From Scratch |
|--------|------------------|--------------|
| **Setup Time** | ⚡ 5 minutes | 🐌 2-3 days |
| **Storage System** | ✅ Included (reactive + SQLite) | ❌ Build yourself |
| **Multi-window Sync** | ✅ Working out of the box | ❌ Complex to implement |
| **TypeScript Config** | ✅ Pre-configured | ⚙️ Hours of setup |
| **Best Practices** | ✅ Established patterns | 🤔 Trial and error |

## �🏗️ Architecture Philosophy

**Backend = Persistence Transport Only**  
The Rust backend is intentionally kept **agnostic and minimal**. It only handles:
- ✅ Document storage (key-value)
- ✅ SQLite persistence
- ✅ Multi-window event synchronization

**Frontend = Business Logic**  
All business logic lives in the Vue/Nuxt frontend:
- 🎯 Application features (users, products, orders, etc.)
- 🎯 Validation rules
- 🎯 Workflows and state machines
- 🎯 UI interactions
- 🎯 API integrations

**Why This Approach?**
- 🚀 Faster iteration (no Rust recompiles for business logic changes)
- 🧪 Easier testing (Vue component tests vs Rust integration tests)
- 🔄 Flexible (swap storage backend without touching business logic)
- 📦 Portable (business logic can move to web/mobile)

## 🚀 Quick Start

## 🚀 Quick Start

### 1. Clone & Setup

```bash
# Clone this repository
git clone <your-repo-url>
cd noken-pos

# Install dependencies
pnpm install

# Run development server
pnpm tauri dev
```

### 2. Try the Demo

The boilerplate includes a **live demo** at `/demo` that showcases:

| Demo | What It Shows |
|------|---------------|
| **Simple Reactive Object** | Auto-save text inputs with 300ms debounce |
| **Registry Sharing** | Two components sharing same document key (changes sync instantly) |
| **Manual Save Mode** | Explicit save button (useful for forms) |
| **Custom Debounce** | Counter with 1-second debounce |
| **Array Mutations** | Todo list with add/toggle/delete operations |
| **Storage Inspector** | View all registry keys and database keys |

**Try multi-window sync:**
1. Open `/demo` in the app
2. Open another window (File → New Window or Cmd+N)
3. Navigate to `/demo` in both windows
4. Change values in one window → see them update in the other! ✨

### 3. Start Building

```bash
# Create a new page
touch app/pages/your-feature.vue

# Use reactive storage in your component
```

```vue
<script setup>
import { useDocumentStorage } from '@/composables/useDocumentStorage'

const myData = useDocumentStorage('my-key', { 
  count: 0,
  items: []
})
</script>

<template>
  <div>
    <h1>Count: {{ myData.count }}</h1>
    <button @click="myData.count++">Increment</button>
  </div>
</template>
```

### Build for Production

```bash
# Build desktop application
pnpm tauri build

# Output in src-tauri/target/release/bundle/
```

## 🎨 Customizing Your App

Ready to make it yours? See the [Customization Guide](docs/customization.md) for:

- Renaming your application
- Updating app metadata and icons
- Adding your business logic
- Customizing the UI with Tailwind
- Configuring routes and navigation
- Adding authentication
- Integrating external APIs

**Quick start:** Edit `src-tauri/tauri.conf.json` and `package.json` to rename your app, then build your features!

## ✨ Key Features

### 🔥 Reactive Document Storage
- **VueUse-style API** - Familiar API like `useLocalStorage` but backed by SQLite
- **Auto-persistence** - Changes save automatically with configurable debounce
- **Type-safe** - Full TypeScript support with generics
- **Multi-window sync** - Changes broadcast across windows via Tauri events
- **Optimistic concurrency** - Version-based conflict detection

### 🏗️ Production-Ready Architecture
- **Feature-based structure** - Organized by features, not layers
- **Separation of concerns** - Business logic in frontend, persistence in backend
- **Modular design** - Easy to extend with new features
- **Clear patterns** - Established composables and utilities

### ⚡ Developer Experience
- **Hot reload** - Vite HMR for instant feedback
- **TypeScript everywhere** - Full type safety
- **VueUse integration** - Leverage 200+ utility functions
- **Tailwind CSS** - Rapid UI development
- **Pinia ready** - State management when needed

### 🚀 Performance
- **Native performance** - Rust backend with no Electron overhead
- **Debounced writes** - Prevents excessive SQLite operations
- **In-memory cache** - Fast reads from Rust HashMap
- **Async persistence** - Non-blocking storage operations

### 🔧 Flexible
- **No backend required** - Fully local-first architecture
- **Offline-capable** - Works without internet
- **Cross-platform** - Windows, macOS, Linux support
- **Easy to extend** - Add REST APIs, WebSocket, or other integrations

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

### Guides
- **[Customization Guide](docs/customization.md)** - Rename, customize, and build your app
- **[Patterns Guide](docs/patterns.md)** - Common patterns and best practices
- **[Extending Backend](docs/extending-backend.md)** - Add Rust features and APIs
- **[Development Guide](docs/development.md)** - Debugging, testing, and profiling
- **[Distribution Guide](docs/distribution.md)** - Building, signing, and distributing
- **[FAQ](docs/faq.md)** - Frequently asked questions

### Architecture
- **[REACTIVE-ARCHITECTURE.md](REACTIVE-ARCHITECTURE.md)** - Architecture deep dive
- **[REACTIVE-STORAGE-GUIDE.md](REACTIVE-STORAGE-GUIDE.md)** - Storage implementation
- **[QUICK-START.md](QUICK-START.md)** - Quick reference
- **[GET-DATABASE-PATH.md](GET-DATABASE-PATH.md)** - Database inspection

### External Resources
- [Tauri Documentation](https://tauri.app/v2/)
- [Nuxt 3 Documentation](https://nuxt.com)
- [VueUse Documentation](https://vueuse.org)
- [Diesel Documentation](https://diesel.rs)

## 📋 Prerequisites

- **Node.js 18+** ([Download](https://nodejs.org))
- **pnpm** (`npm install -g pnpm`)
- **Rust 1.70+** ([Install via rustup](https://rustup.rs))
- **System dependencies** ([Platform-specific](https://tauri.app/v2/guides/getting-started/prerequisites/))

## 📦 Distribution

```bash
# Build for production
pnpm tauri build
```

Outputs platform-specific installers (`.dmg`, `.msi`, `.deb`, `.AppImage`).

For detailed instructions on code signing, auto-updates, and app store distribution, see the [Distribution Guide](docs/distribution.md).

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
// Main reactive storage
useDocumentStorage<T>(key, initialValue?, options?)  

// Helper functions
removeDocument(key)       // Delete document
getDocumentKeys()         // Get all stored keys
getRegistryKeys()         // Get active registry keys
clearDocumentStorage()    // Clear all documents
```

## 🛣️ Roadmap & Extensions

**What's included:**
- ✅ Reactive document storage
- ✅ Multi-window sync
- ✅ SQLite persistence
- ✅ Full TypeScript
- ✅ Demo page

**Easy to add:**
- 🔲 REST API integration
- 🔲 Authentication
- 🔲 WebSockets
- 🔲 File system operations
- 🔲 Print/PDF export
- 🔲 Native dialogs

See the [Extending Backend Guide](docs/extending-backend.md) for TODO APIs and implementation examples:
- File system operations
- System information
- Native dialogs
- Print & export
- HTTP client
- Window management
- Logging
- Encryption

## 🧩 Common Patterns

Learn best practices and reusable patterns in the [Patterns Guide](docs/patterns.md):

- **Feature composables** - Domain-specific logic (useCustomers, useProducts)
- **Settings management** - Manual save mode for forms
- **Draft persistence** - Auto-save form state
- **API caching** - Local caching layer for API responses
- **Recent items** - Track recently accessed items
- **Multi-window state** - Shared state across windows
- **Offline queue** - Queue actions when offline
- **Search index** - Build local search functionality

**Quick example:**

```ts
// app/composables/useProducts.ts
export function useProducts() {
  const products = useDocumentStorage('products', [], {
    collection: 'products'
  })
  
  const addProduct = (data) => {
    products.value.push({ id: crypto.randomUUID(), ...data })
  }
  
  return { products, addProduct }
}
```

## 🧪 Development & Testing

```bash
# Start dev server with hot reload
pnpm tauri dev

# With Rust debug logs
RUST_LOG=debug pnpm tauri dev
```

**Database location:**
- macOS: `~/Library/Application Support/com.yourcompany.yourapp/`
- Linux: `~/.local/share/com.yourcompany.yourapp/`
- Windows: `%APPDATA%\com.yourcompany.yourapp\`

Connect with [DBeaver](https://dbeaver.io), DB Browser for SQLite, or TablePlus.

For detailed debugging, testing, profiling, and troubleshooting, see the [Development Guide](docs/development.md).

## ❓ FAQ

**Quick answers:**

- **vs Electron?** Smaller (~3-10 MB vs ~50-100 MB), faster, uses native webview
- **Other frameworks?** Yes! Backend is framework-agnostic (React, Svelte, etc.)
- **Add REST API?** Yes! Use Nuxt's `$fetch` for server data
- **Works offline?** Yes! Fully local-first with embedded SQLite
- **Production ready?** Yes! Includes error handling, optimizations, and multi-window sync
- **Database migrations?** Use Diesel: `diesel migration generate <name>`
- **Use with Pinia?** Yes! Pinia for UI state, useDocumentStorage for persistent data

See the [FAQ Guide](docs/faq.md) for detailed answers and troubleshooting.

## 🤝 Contributing

We welcome contributions! Whether you're:
- 🐛 Fixing bugs
- ✨ Adding new features
- 📚 Improving documentation
- 💡 Suggesting improvements

### How to Contribute

1. Fork this repository
2. Create a feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes (`git commit -m 'Add amazing feature'`)
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request

### Development Guidelines

- Follow TypeScript best practices
- Use Rust clippy for linting (`cargo clippy`)
- Test your changes in dev mode (`pnpm tauri dev`)
- Update documentation if needed

## 💬 Community & Support

- 💡 **Questions?** Open a [Discussion](https://github.com/yourusername/yourrepo/discussions)
- 🐛 **Found a bug?** Open an [Issue](https://github.com/yourusername/yourrepo/issues)
- ⭐ **Like this project?** Give it a star!
- 🔄 **Want updates?** Watch the repository

## 📝 License

MIT License - feel free to use this boilerplate for commercial or personal projects!

See [LICENSE](LICENSE) file for details.

## 🙏 Acknowledgments

Built with amazing open-source projects:
- [Tauri](https://tauri.app) - Native desktop framework
- [Nuxt](https://nuxt.com) - Vue framework
- [Diesel](https://diesel.rs) - Rust ORM
- [VueUse](https://vueuse.org) - Vue composition utilities
- [Tailwind CSS](https://tailwindcss.com) - Utility-first CSS

## 🚀 Show Your Work!

Built something cool with this boilerplate? We'd love to see it! Share:
- Your project repository
- Screenshots or demo videos
- Use case or success story

Open a discussion and show off what you've built! 🎉

---

<div align="center">

**[⬆ Back to Top](#nuxt--tauri-reactive-storage-boilerplate)**

Made with ❤️ for the developer community

</div>
