# Contributing to Nuxt + Tauri Boilerplate

Thank you for your interest in contributing! 🎉

## How to Contribute

### Reporting Bugs 🐛

1. Check if the bug has already been reported in [Issues](https://github.com/yourusername/nuxt-tauri-boilerplate/issues)
2. If not, create a new issue with:
   - Clear title and description
   - Steps to reproduce
   - Expected vs actual behavior
   - Screenshots (if applicable)
   - Environment details (OS, Node version, etc.)

### Suggesting Features 💡

1. Check [Discussions](https://github.com/yourusername/nuxt-tauri-boilerplate/discussions) first
2. Create a new discussion or issue describing:
   - The problem you're trying to solve
   - Your proposed solution
   - Why it would benefit other users

### Pull Requests 🔄

1. **Fork** the repository
2. **Clone** your fork locally
3. **Create a branch** from `main`:
   ```bash
   git checkout -b feature/your-feature-name
   ```
4. **Make your changes** following the guidelines below
5. **Test** your changes thoroughly
6. **Commit** with clear messages:
   ```bash
   git commit -m "feat: add amazing feature"
   git commit -m "fix: resolve storage bug"
   git commit -m "docs: update README examples"
   ```
7. **Push** to your fork:
   ```bash
   git push origin feature/your-feature-name
   ```
8. **Open a Pull Request** with:
   - Clear description of changes
   - Related issue numbers (if any)
   - Screenshots/GIFs for UI changes

## Development Guidelines

### Code Style

**TypeScript/Vue:**
- Use TypeScript for type safety
- Follow Vue 3 Composition API patterns
- Use VueUse utilities when available
- Keep components focused and small

**Rust:**
- Run `cargo fmt` before committing
- Run `cargo clippy` and fix warnings
- Add comments for complex logic
- Keep functions small and focused

### Project Structure

```
app/
  ├── composables/    # Reusable Vue composables
  ├── components/     # Vue components
  ├── pages/          # Nuxt pages
  ├── types/          # TypeScript types
  └── utils/          # Utility functions

src-tauri/
  ├── src/
  │   ├── core/       # Core functionality (db, errors)
  │   └── features/   # Feature modules
  └── migrations/     # Diesel migrations
```

### Testing

Before submitting a PR:

```bash
# Test development build
pnpm tauri dev

# Check Rust code
cd src-tauri
cargo fmt --check
cargo clippy -- -D warnings
cargo test

# Test production build
pnpm tauri build
```

### Commit Convention

We follow [Conventional Commits](https://www.conventionalcommits.org/):

- `feat:` - New feature
- `fix:` - Bug fix
- `docs:` - Documentation changes
- `style:` - Code style changes (formatting, etc.)
- `refactor:` - Code refactoring
- `test:` - Adding or updating tests
- `chore:` - Maintenance tasks

Examples:
```
feat: add multi-collection query support
fix: resolve multi-window sync race condition
docs: add migration guide to README
refactor: simplify storage registry logic
```

## Areas for Contribution

### High Priority

- [ ] Unit tests for composables
- [ ] Integration tests for Rust commands
- [ ] Improved error handling
- [ ] Performance optimizations
- [ ] Better documentation examples

### Good First Issues

- [ ] Add more demo examples
- [ ] Improve TypeScript types
- [ ] Add JSDoc comments
- [ ] Update documentation
- [ ] Add code snippets

### Advanced Features

- [ ] Collection-based queries
- [ ] Full-text search
- [ ] Data encryption
- [ ] Remote sync engine
- [ ] Migration utilities

## Questions?

- 💬 Start a [Discussion](https://github.com/yourusername/nuxt-tauri-boilerplate/discussions)
- 📧 Open an [Issue](https://github.com/yourusername/nuxt-tauri-boilerplate/issues)

## Code of Conduct

- Be respectful and inclusive
- Provide constructive feedback
- Focus on what's best for the community
- Show empathy towards others

Thank you for contributing! 🙏
