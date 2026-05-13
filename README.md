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

## CI/CD - GitHub Actions

This project includes automated build workflows for multiple platforms.

### Build Workflow (`.github/workflows/build.yml`)

Automatically builds the app on every push to `main` or `develop` branches.

**Supported Platforms:**
- ✅ Windows (x86_64) - `.msi` and `.exe` installers
- ✅ macOS (Apple Silicon & Intel) - `.dmg` installer
- ✅ Linux (x86_64) - `.deb` and `.AppImage`
- ✅ Android - `.apk` and `.aab`

**Artifacts:** Build artifacts are uploaded and available for download from the Actions tab for 90 days.

### Release Workflow (`.github/workflows/release.yml`)

Creates GitHub releases with all platform builds attached.

**How to Create a Release:**

1. **Using Git Tags:**
   ```bash
   git tag v1.0.0
   git push origin v1.0.0
   ```

2. **Manual Trigger:**
   - Go to Actions → Release Build → Run workflow
   - Enter version number (e.g., `1.0.0`)
   - Click "Run workflow"

**Release Assets:**
- Windows: `noken-pos_1.0.0_x64.msi`, `noken-pos_1.0.0_x64-setup.exe`
- macOS: `noken-pos_1.0.0_aarch64.dmg`, `noken-pos_1.0.0_x64.dmg`
- Linux: `noken-pos_1.0.0_amd64.deb`, `noken-pos_1.0.0_amd64.AppImage`
- Android: `noken-pos_1.0.0_universal.apk`, `noken-pos_1.0.0_universal.aab`

### Downloading Build Artifacts

**From Actions Tab:**
1. Go to the repository → Actions tab
2. Click on a completed workflow run
3. Scroll to "Artifacts" section
4. Download the artifact for your platform:
   - `noken-pos-windows-x86_64` - Windows builds
   - `noken-pos-macos-aarch64` - macOS Apple Silicon
   - `noken-pos-macos-x86_64` - macOS Intel
   - `noken-pos-linux-x86_64` - Linux builds
   - `noken-pos-android` - Android builds

**From Releases:**
1. Go to the repository → Releases
2. Click on the latest release
3. Download the appropriate file for your platform

### Installation Instructions

**Windows:**
```bash
# Run the MSI installer
noken-pos_1.0.0_x64.msi

# Or run the NSIS installer
noken-pos_1.0.0_x64-setup.exe
```

**macOS:**
```bash
# Open the DMG and drag to Applications
open noken-pos_1.0.0_aarch64.dmg
```

**Linux (Debian/Ubuntu):**
```bash
# Install DEB package
sudo dpkg -i noken-pos_1.0.0_amd64.deb

# Or run AppImage directly
chmod +x noken-pos_1.0.0_amd64.AppImage
./noken-pos_1.0.0_amd64.AppImage
```

**Android:**
```bash
# Enable "Install from Unknown Sources" in Settings
# Then install the APK
adb install noken-pos_1.0.0_universal.apk
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
