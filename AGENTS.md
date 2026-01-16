# AGENTS.md

This file provides guidelines for AI agents working on the FeiQiu project.

## Project Overview

FeiQiu is a LAN instant messaging tool built with Tauri 2.0 + React + Vite, compatible with IPMsg protocol.

## Build Commands

### Frontend (feiqiu/)
```bash
bun install           # Install dependencies
bun dev               # Start Vite dev server
bun run build         # Build frontend (TypeScript + Vite)
bun run preview       # Preview production build
```

### Tauri/Rust Backend (feiqiu/src-tauri/)
```bash
cd feiqiu/src-tauri

# Development
cargo check           # Check compilation without building
cargo test --lib      # Run library unit tests only
cargo test --lib -- --nocapture  # Run with output
cargo test -- --test-threads=1   # Run tests sequentially

# Build
cargo build           # Debug build
cargo build --release # Release build
bun tauri build       # Build Tauri app bundle

# Single test
cargo test test_peer_node_new                                    # By function name
cargo test modules::peer::types::tests::test_display_name        # By full path
cargo test peer_repo_tests --test-threads=1                      # Integration test file
```

## Code Style Guidelines

### Modules & Imports
- Use `mod` for private modules, `pub mod` for public exports
- Internal imports: `use crate::module::Item;`
- External imports: Direct crate names
- Group imports by origin (std, external, internal)

### Error Handling
- Use `thiserror` for error enums with `#[derive(Error, Debug)]`
- Define `pub type Result<T> = std::result::Result<T, YourError>;`
- Use `#[from]` for automatic conversion
- Add context with `.with_context()` method pattern

```rust
#[derive(Error, Debug)]
pub enum MyError {
    #[error("Operation failed: {0}")]
    Operation(String),
}

pub type Result<T> = std::result::Result<T, MyError>;
```

### Structs & Types
- Default derive: `#[derive(Clone, Debug, Serialize, Deserialize)]`
- Document public fields with `/// Field description`
- Use doc comments (`///`) for public APIs
- Use module docs (`//!`) for internal documentation

### Naming Conventions
- Types: `PascalCase` (structs, enums, traits)
- Functions/variables: `snake_case`
- Constants: `SCREAMING_SNAKE_CASE` or `snake_case` for local const
- Modules: `snake_case`
- Prefixed private fields: `_field_name` where applicable

### Testing
- Place unit tests in `#[cfg(test)] mod tests` within the same file
- Place integration tests in `tests/` directory at crate root
- Test files: `*_tests.rs` naming convention
- Use `assert!`, `assert_eq!`, `assert_matches!` for assertions

### Conventions
- Use `Result<T, Error>` for fallible operations
- Use `Option<T>` for optional values
- Prefer ` anyhow` or `thiserror` for application errors
- Use `Arc<Mutex<T>>` for shared mutable state across threads
- Use `tokio::sync` primitives for async concurrency

## Project Structure

```
feiqiu/
├── src/                 # React frontend
├── src-tauri/           # Rust backend
│   ├── src/
│   │   ├── commands/    # Tauri command handlers
│   │   ├── modules/     # Business logic (peer, message, file_transfer)
│   │   ├── network/     # UDP/TCP, protocol parsing
│   │   ├── storage/     # SeaORM entities & repositories
│   │   └── state/       # AppState, events
│   ├── tests/           # Integration tests
│   └── examples/        # Standalone examples
```

## Common Patterns

### Tauri Command
```rust
#[tauri::command]
pub fn get_data(state: State<AppState>) -> Result<Value> {
    // Implementation
}
```

### Repository Pattern
```rust
pub struct Repository {
    db: DatabaseConnection,
}

impl Repository {
    pub fn new(db: DatabaseConnection) -> Self { Self { db } }
}
```

## Additional Notes

- Frontend uses Zustand for state management
- Backend uses SeaORM with SQLite
- Protocol: IPMsg compatible (UDP broadcast + TCP transfers)
- Default UDP port: 2425
