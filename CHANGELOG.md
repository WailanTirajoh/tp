# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [1.0.0] - 2026-05-14

### Added
- Initial boilerplate release
- Reactive document storage system with `useDocumentStorage` composable
- Multi-window synchronization via Tauri events
- SQLite persistence with Diesel ORM
- Feature-based Rust backend architecture
- Demo page showcasing all storage features
- Comprehensive documentation (README, guides)
- TypeScript support throughout
- VueUse integration
- Tailwind CSS configuration
- Storage registry for shared reactive refs
- Debounced persistence (configurable)
- Manual save mode option
- Collection-based document grouping
- Version-based optimistic concurrency control

### Frontend
- Nuxt 3.15+ setup
- Vue 3 Composition API
- TypeScript 5.0+
- VueUse 14.3.0
- Tailwind CSS
- Pinia state management

### Backend
- Tauri 2.0
- Rust 1.70+
- Diesel 2.2 ORM
- SQLite with WAL mode
- Parking_lot for efficient locking
- Tokio async runtime

### Documentation
- Complete README with use cases
- Quick start guide
- Architecture documentation
- API reference
- Common patterns and examples
- FAQ section
- Contributing guide
- MIT License

## Future Releases

See [CONTRIBUTING.md](CONTRIBUTING.md) for planned features and how to contribute.

---

[Unreleased]: https://github.com/yourusername/nuxt-tauri-boilerplate/compare/v1.0.0...HEAD
[1.0.0]: https://github.com/yourusername/nuxt-tauri-boilerplate/releases/tag/v1.0.0
