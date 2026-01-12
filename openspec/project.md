# Project Context

## Purpose

**FeiQiu (飞秋)** is a LAN-based instant messaging desktop application designed for enterprise internal communication. It provides a secure, offline-capable communication platform with features including:

- Real-time messaging between peers on the same LAN
- File transfer capabilities
- Screen capture and screen sharing for collaboration
- Organization chart viewing for company structure
- Network and personal settings configuration
- Full desktop app experience (not browser-dependent)

The application is compatible with the IPMsg (IP Messenger) protocol, allowing interoperability with existing IPMsg clients.

## Tech Stack

### Frontend
- **React 18** - UI framework with TypeScript
- **Vite** - Build tool and dev server
- **Tailwind CSS** - Utility-first styling
- **Zustand** (planned) - State management

### Backend
- **Tauri** - Desktop app framework (Rust backend)
- **Sea-ORM** - ORM for database operations
- **SQLite** - Embedded database
- **Tokio** - Async runtime

### Development Tools
- **Node.js** - Frontend package management
- **Cargo** - Rust package management
- **TypeScript** - Type-safe frontend code
- **Rust** - Systems programming for backend

### IPC Communication
- **Tauri Commands** - Frontend-backend communication via invoke/async commands

## Project Conventions

### Code Style

#### Frontend (TypeScript/React)
- Use functional components with hooks
- Prefer composition over inheritance
- Use Tailwind CSS classes for styling (avoid inline styles)
- Follow the existing color system: emerald (primary), blue (secondary), slate (neutral)
- Support dark mode with `dark:` prefix classes
- Use `@/` path alias for imports within the `feiqiu/` directory

#### Backend (Rust)
- Use async/await with `tokio::runtime`
- Follow Rust naming conventions: `snake_case` for functions/variables, `PascalCase` for types
- Use `#[tauri::command]` macro for IPC command handlers
- Return `Result<T, Error>` for fallible operations
- Use `Arc<Mutex<T>>` for shared state

#### Naming Conventions
- Components: `PascalCase` (e.g., `MessageList.tsx`)
- Hooks: `camelCase` with `use` prefix (e.g., `useDarkMode.ts`)
- Types/Interfaces: `PascalCase` (e.g., `User`, `Message`)
- Commands: `snake_case` (e.g., `send_message`, `get_peers`)

### Architecture Patterns

#### Frontend Structure
```
src/
├── components/           # Organized by feature section
│   ├── shell/           # App shell (navigation, layout)
│   ├── basic-settings/  # Settings pages
│   ├── messaging/       # Chat components
│   ├── file-transfer/   # File transfer UI
│   ├── collaboration/   # Screen share/capture
│   └── organization/    # Org chart tree
├── lib/types/           # Type definitions per section
└── hooks/               # Custom React hooks
```

#### Backend Structure
```
src-tauri/src/
├── commands/            # Tauri IPC command handlers
├── modules/             # Core business logic
│   ├── peer/           # Peer discovery & heartbeat
│   ├── message/        # Message handling
│   ├── file_transfer/  # File transfer logic
│   └── group/          # Group management
├── network/            # Network layer (UDP/TCP)
├── storage/            # Database (entities, repos)
├── state/              # Global app state (AppState)
└── lib.rs              # Tauri setup & IPC registration
```

#### Communication Flow
1. Frontend calls `invoke('command_name', { args })`
2. Tauri routes to registered command handler in `commands/`
3. Command handler uses modules in `modules/` for business logic
4. State changes emit events via `poll_events` for frontend updates

#### Database Layer
- Entities defined in `storage/entities/`
- Migrations in `migration/`
- Repository functions in `storage/*_repo.rs`

### Testing Strategy

#### Frontend Tests
- Component tests with React Testing Library (planned)
- Integration tests for user flows

#### Backend Tests
- Unit tests for Rust modules via `cargo test`
- Integration tests for database operations

#### End-to-End Tests
- Full application flow tests via Tauri test framework

#### Coverage Goals
- Critical business logic: 80%+ coverage
- UI components: 60%+ coverage

### Git Workflow

#### Branching Strategy
- `main` - Stable production code
- Feature branches: `feature/feature-name` or `section/section-name`
- Fix branches: `fix/bug-description`

#### Commit Conventions
- Use conventional commits with prefixes:
  - `feat:` - New features
  - `fix:` - Bug fixes
  - `refactor:` - Code refactoring
  - `chore:` - Build/config changes
  - `docs:` - Documentation

Example: `feat(messaging): add message reaction support`

#### Code Review
- All changes to `main` via pull requests
- Ensure tests pass before merging
- Update documentation for API changes

### Path Aliases & Imports

Within the `feiqiu/` directory, use the `@/` alias configured in Vite:
```typescript
import { Button } from '@/components/ui/button'
import { User } from '@/lib/types'
import { useDarkMode } from '@/hooks/useDarkMode'
```

### Creating New Components

1. Add component to `feiqiu/src/components/[section]/`
2. Create types in `feiqiu/src/lib/types/[section].ts`
3. Use Tailwind classes for styling
4. Accept data/callbacks via props
5. Export from `index.ts`: `export * from './ComponentName'`

### Adding New IPC Commands

1. Create command function in `feiqiu/src-tauri/src/commands/[module].rs`
2. Register in `feiqiu/src-tauri/src/lib.rs` invoke_handler
3. Call from frontend using `invoke()`

## Domain Context

### Network Protocol (IPMsg Compatible)

The application uses the IPMsg protocol for LAN communication:

- **Default UDP Port**: 2425
- **Packet Format**: `version:packet_id:sender_name:sender_host:msg_type:content[:ext_fields]`
- **Message Types**:
  - `0x0001` - Broadcast presence (online)
  - `0x0020` - Send message
  - `0x0048` - File send request
  - `0x0050` - File accept

See `src-tauri/src/network/protocol.rs` for full implementation.

### Peer Discovery

- Uses UDP broadcast for peer discovery on LAN
- Heartbeat mechanism to detect online/offline status
- Peer list cached in SQLite database

### Message Handling

- Messages stored in SQLite with sender, receiver, timestamp
- Supports text messages and file transfer metadata
- Real-time updates via event polling

### File Transfer

- Large files sent via TCP (not UDP)
- Progress tracking for transfers
- File accept/reject flow

## Important Constraints

### Technical Constraints
- Must work offline (LAN-only, no internet required)
- Must be compatible with IPMsg protocol for interoperability
- Desktop app only (Windows, macOS, Linux support via Tauri)
- Embedded SQLite database (no external database server)

### Performance Constraints
- Should support 100+ concurrent peers on same LAN
- File transfers should handle files up to several GB
- Low latency for message delivery (< 100ms on LAN)

### Security Considerations
- LAN traffic is unencrypted (typical for enterprise LAN)
- Consider adding optional encryption for sensitive data
- Validate all incoming network packets

## External Dependencies

### Runtime Dependencies
- **Tauri** - Desktop framework (bundled with app)
- **Node.js/npm** - For development only (not runtime)

### Build Dependencies
- **Rust toolchain** (cargo, rustc)
- **Node.js** (for Vite frontend build)
- **Tauri CLI** (`npm run tauri build`)

### Key Rust Crates
- `tauri` - Desktop framework
- `sea-orm` - Database ORM
- `sqlx` or `sqlite` - Database driver
- `tokio` - Async runtime
- `serde` - Serialization

### Key Frontend Packages
- `react` + `react-dom`
- `@tauri-apps/api/*` - Tauri APIs
- `tailwindcss` - Styling
- `vite` - Build tool

## Development Commands Reference

```bash
# Frontend dev server (http://localhost:1420)
cd feiqiu && npm run dev

# Full Tauri app (frontend + backend)
cd feiqiu && npm run tauri dev

# Build for production
cd feiqiu && npm run tauri build

# Rust backend only
cd feiqiu/src-tauri && cargo build
cd feiqiu/src-tauri && cargo test
cd feiqiu/src-tauri && cargo clippy
```

## Related Documentation

- `CLAUDE.md` - Main project overview and patterns
- `design-system/` - Color palette, typography, UI tokens
- `sections/` - Component specifications by feature
- `instructions/` - Implementation guides
