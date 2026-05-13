# Noken POS - Tauri + Nuxt + SQLite

A Point of Sale application built with Tauri, Nuxt 3, and SQLite.

## Features

- ✅ Tauri v2 backend with Rust
- ✅ Nuxt 3 frontend with Vue 3
- ✅ SQLite database integration
- ✅ Full CRUD operations for Users table
- ✅ Tailwind CSS for styling
- ✅ TypeScript support

## Tech Stack

### Frontend
- **Nuxt 3** - Vue.js framework
- **Vue 3** - Progressive JavaScript framework
- **TypeScript** - Type-safe development
- **Tailwind CSS** - Utility-first CSS framework
- **Pinia** - State management
- **VueUse** - Collection of Vue composition utilities

### Backend
- **Tauri v2** - Desktop application framework
- **Rust** - Systems programming language
- **SQLite** - Embedded database (via rusqlite)

## Project Structure

```
noken-pos/
├── app/                          # Nuxt application
│   ├── assets/
│   │   └── css/
│   │       └── main.css         # Tailwind CSS imports
│   ├── composables/
│   │   └── useUsersApi.ts       # API composable for Tauri commands
│   ├── pages/
│   │   ├── index.vue            # Home page
│   │   └── users.vue            # Users CRUD page
│   └── app.vue                  # Root component
├── src-tauri/                    # Tauri Rust backend
│   ├── src/
│   │   ├── db.rs                # Database initialization & connection
│   │   ├── models.rs            # Data models (User, CreateUserInput, etc.)
│   │   ├── lib.rs               # Tauri commands (CRUD operations)
│   │   └── main.rs              # Application entry point
│   ├── Cargo.toml               # Rust dependencies
│   └── tauri.conf.json          # Tauri configuration
├── nuxt.config.ts               # Nuxt configuration
├── tailwind.config.js           # Tailwind CSS configuration
└── package.json                 # Node.js dependencies
```

## Database Schema

### Users Table

```sql
CREATE TABLE users (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    name TEXT NOT NULL,
    email TEXT NOT NULL UNIQUE,
    age INTEGER,
    created_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP
);
```

## Available Tauri Commands

All commands are defined in `src-tauri/src/lib.rs`:

- `get_users()` - Fetch all users
- `get_user(id)` - Fetch a single user by ID
- `create_user(input)` - Create a new user
- `update_user(id, input)` - Update an existing user
- `delete_user(id)` - Delete a user

## API Usage

The `useUsersApi()` composable provides TypeScript-safe methods to interact with Tauri commands:

```typescript
const { getUsers, getUser, createUser, updateUser, deleteUser } = useUsersApi()

// Get all users
const users = await getUsers()

// Create a user
const newUser = await createUser({
  name: 'John Doe',
  email: 'john@example.com',
  age: 30
})

// Update a user
const updated = await updateUser(1, {
  name: 'Jane Doe'
})

// Delete a user
await deleteUser(1)
```

## Development

### Prerequisites

- Node.js (v18 or higher)
- pnpm (v8 or higher)
- Rust (latest stable)
- Tauri CLI

### Install Dependencies

```bash
pnpm install
```

### Run Development Server

```bash
pnpm tauri dev
```

This will:
1. Start the Nuxt dev server on http://localhost:3000
2. Compile the Rust backend
3. Launch the Tauri desktop application

### Build for Production

```bash
pnpm tauri build
```

## Database Location

The SQLite database is created at:
- **macOS**: `~/Library/Application Support/com.tauri.dev/database.sqlite`
- **Windows**: `%APPDATA%\com.tauri.dev\database.sqlite`
- **Linux**: `~/.local/share/com.tauri.dev/database.sqlite`

## Pages

### Home (`/`)
Welcome page with information about the application and links to features.

### Users Management (`/users`)
Full CRUD interface for managing users:
- View all users in a table
- Create new users via modal form
- Edit existing users
- Delete users with confirmation
- Real-time updates after operations

## Configuration

### Tailwind CSS

Tailwind is configured in `tailwind.config.js` and included in `nuxt.config.ts`:

```javascript
// nuxt.config.ts
export default defineNuxtConfig({
  css: ['~/assets/css/main.css'],
  postcss: {
    plugins: {
      tailwindcss: {},
      autoprefixer: {},
    },
  },
})
```

## Extending the Application

### Adding a New Table

1. **Update Database Schema** (`src-tauri/src/db.rs`)
2. **Create Model** (`src-tauri/src/models.rs`)
3. **Add Tauri Commands** (`src-tauri/src/lib.rs`)
4. **Create Nuxt Composable** (`app/composables/`)
5. **Create UI** (`app/pages/`)

## License

MIT
