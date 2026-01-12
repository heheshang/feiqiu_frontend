<!-- OPENSPEC:START -->
# OpenSpec Instructions

These instructions are for AI assistants working in this project.

Always open `@/openspec/AGENTS.md` when the request:
- Mentions planning or proposals (words like proposal, spec, change, plan)
- Introduces new capabilities, breaking changes, architecture shifts, or big performance/security work
- Sounds ambiguous and you need the authoritative spec before coding

Use `@/openspec/AGENTS.md` to learn:
- How to create and apply change proposals
- Spec format and conventions
- Project structure and guidelines

Keep this managed block so 'openspec update' can refresh the instructions.

<!-- OPENSPEC:END -->

# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

**飞秋 (FeiQiu)** is a LAN-based instant messaging desktop application built with Tauri (Rust backend) + React (TypeScript frontend). It's designed for enterprise internal communication with features including messaging, file transfer, collaboration tools, and organization chart viewing.

## Development Commands

### Frontend (React + Vite)
```bash
cd feiqiu
npm run dev          # Start dev server on http://localhost:1420
npm run build        # Build for production (outputs to dist/)
```

### Rust Backend
```bash
cd feiqiu
npm run tauri dev    # Run full Tauri app (frontend + backend)
npm run tauri build  # Build distributable desktop app
```

### Rust Commands (in feiqiu/src-tauri/)
```bash
cargo build                    # Build Rust backend
cargo build --release          # Build optimized release
cargo test                     # Run tests
cargo clippy                   # Lint
```

## Project Structure

```
feiqiu_frontend/
├── feiqiu/                          # Main Tauri application
│   ├── src/                         # React frontend
│   │   ├── components/              # React components by section
│   │   │   ├── shell/               # App shell components
│   │   │   ├── basic-settings/      # Network & personal settings
│   │   │   ├── messaging/           # Chat functionality
│   │   │   ├── file-transfer/       # File transfer UI
│   │   │   ├── collaboration/       # Screen capture/sharing
│   │   │   └── organization/        # Department tree view
│   │   ├── lib/
│   │   │   └── types/               # TypeScript type definitions
│   │   ├── hooks/                   # Custom React hooks
│   │   ├── App.tsx                  # Main app with routing
│   │   └── main.tsx                 # Entry point
│   ├── src-tauri/                   # Rust backend
│   │   └── src/
│   │       ├── commands/            # Tauri IPC commands (exposed to frontend)
│   │       ├── modules/             # Core business logic
│   │       │   ├── peer/            # Peer discovery, heartbeat
│   │       │   ├── message/         # Message handling
│   │       │   ├── file_transfer/   # File transfer logic
│   │       │   └── group/           # Group management
│   │       ├── network/             # Network layer (UDP/TCP)
│   │       ├── storage/             # Database (Sea-ORM + SQLite)
│   │       ├── state/               # Global app state
│   │       └── lib.rs               # Tauri setup (IPC commands registration)
│   ├── package.json
│   ├── vite.config.ts               # Vite config with path aliases (@/)
│   └── tsconfig.json
│
├── data-model/                      # Shared type definitions
│   └── types.ts                     # Global types (User, Message, etc.)
├── design-system/                   # Design tokens
│   ├── tailwind-colors.md           # Color palette (emerald/blue/slate)
│   └── fonts.md                     # Typography (Inter, IBM Plex Mono)
├── sections/                        # Component specifications
│   ├── basic-settings/
│   ├── messaging/
│   ├── file-transfer/
│   ├── collaboration/
│   └── organization/
├── shell/                           # Shell specifications
├── instructions/                    # Implementation guides
└── prompts/                         # AI implementation prompts
```

## Architecture

### Frontend-Backend Communication

The frontend communicates with Rust backend via **Tauri IPC commands**:

```typescript
// Frontend (React)
import { invoke } from '@tauri-apps/api/core';

const result = await invoke('command_name', { arg1: value1 });
```

```rust
// Backend (Rust) - registered in lib.rs
#[tauri::command]
async fn command_name(arg1: Type) -> Result<Type, Error> {
    // ...
}
```

Key IPC commands (in `feiqiu/src-tauri/src/commands/`):
- `config.rs`: `get_config`, `update_config`
- `peer.rs`: `get_peers`, `refresh_peers`
- `message.rs`: `send_message`, `get_messages`
- `file_transfer.rs`: `send_file`, `accept_transfer`
- `events.rs`: `poll_events` (for real-time updates)

### Application State Management

**Frontend**: Zustand stores in `src/lib/` (planned)

**Backend**: `AppState` struct with `Arc<Mutex<>>` in `src-tauri/src/state/app_state.rs`

### Network Protocol

Compatible with **IPMsg (IP Messenger)** protocol:
- Default UDP port: 2425
- Format: `version:packet_id:sender_name:sender_host:msg_type:content[:ext_fields]`
- See `src-tauri/src/network/protocol.rs` for implementation

### Database

SQLite with Sea-ORM:
- Entities in `src-tauri/src/storage/entities/`
- Migrations in `src-tauri/src/migration/`
- Repositories in `src-tauri/src/storage/`

## Design System

### Colors
- **Primary**: emerald (buttons, links, key accents)
- **Secondary**: blue (secondary actions)
- **Neutral**: slate (backgrounds, text, borders)

### Dark Mode
Uses `dark:` prefix with Tailwind classes. Toggle via `useDarkMode` hook.

### Typography
- Sans: Inter
- Mono: IBM Plex Mono

## Path Aliases

In `feiqiu/`, use `@/` prefix for imports:
```typescript
import { Button } from '@/components/ui/button'
import { User } from '@/lib/types'
```

## Common Development Patterns

### Creating a New Component

1. Add component to `feiqiu/src/components/[section]/`
2. Create types in `feiqiu/src/lib/types/[section].ts`
3. Use Tailwind classes for styling (see `design-system/tailwind-colors.md`)
4. Accept data/callbacks via props (no internal state management)
5. Export from index.ts: `export * from './ComponentName'`

### Adding a New IPC Command

1. Create command function in `feiqiu/src-tauri/src/commands/[module].rs`:
```rust
#[tauri::command]
async fn my_command(param: String) -> Result<String, Error> {
    Ok(format!("Processed: {}", param))
}
```

2. Register in `feiqiu/src-tauri/src/lib.rs`:
```rust
.invoke_handler(tauri::generate_handler![
    // ... existing commands
    my_command,
])
```

3. Call from frontend:
```typescript
import { invoke } from '@tauri-apps/api/core';
const result = await invoke('my_command', { param: 'value' });
```

### Working with Database

1. Define entity in `src-tauri/src/storage/entities/`
2. Add migration in `src-tauri/src/migration/`
3. Create repository functions in `src-tauri/src/storage/*_repo.rs`
4. Use from command handlers

## Implementation Guides

- `instructions/one-shot-instructions.md` - Full implementation guide
- `instructions/incremental/` - Section-by-section guides

## Component Specifications

Each section in `sections/` contains:
- `spec.md` - Component specification
- `README.md` - Overview
- Component files (reference implementations)

## Known Issues

- The `dist/` folder doesn't exist by default (created on build)
- Some Rust warnings about unused imports in `src-tauri/src/network/tcp.rs`
