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
- **Vite 5** - Build tool and dev server (runs on port 1420)
- **Tailwind CSS 4** - Utility-first styling (using @import + @theme syntax)
- **Zustand 5** - State management for frontend state

### Backend
- **Tauri** - Desktop app framework (Rust backend)
- **Sea-ORM** - ORM for database operations
- **SQLite** - Embedded database
- **Tokio** - Async runtime

### Development Tools
- **Bun** - Frontend package management (preferred over npm for faster operations)
- **Node.js** (18+) - Alternative package manager and runtime
- **Cargo** - Rust package management
- **TypeScript 5** - Type-safe frontend code
- **Rust** - Systems programming for backend (edition 2021)

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
- Use `cn()` utility (from `lib/utils.ts`) for conditional class merging
- Document hooks with JSDoc comments including @example usage
- Handle loading states and errors in custom hooks

#### Backend (Rust)
- Use async/await with `tokio::runtime`
- Follow Rust naming conventions: `snake_case` for functions/variables, `PascalCase` for types
- Use `#[tauri::command]` macro for IPC command handlers
- Return `Result<T, Error>` for fallible operations
- Use `Arc<Mutex<T>>` for shared state
- Use `#[serde(rename_all = "camelCase")]` for DTOs sent to frontend
- Include usage examples in command documentation: `# Frontend Usage` with TypeScript code
- Use `tracing` for logging (info!, warn!, error!, debug!)

#### Naming Conventions
- Components: `PascalCase` (e.g., `MessageList.tsx`)
- Hooks: `camelCase` with `use` prefix (e.g., `useDarkMode.ts`)
- Types/Interfaces: `PascalCase` (e.g., `User`, `Message`)
- Commands: `snake_case` (e.g., `send_message`, `get_peers`)
- DTOs: `PascalCase` with `Dto` suffix (e.g., `PeerDto`)
- Events: `snake_case` with hyphens (e.g., `message-received`, `peer-online`)

### Configuration Constants

**Network Configuration:**
- `DEFAULT_UDP_PORT`: 2425 (IPMsg standard)
- `DEFAULT_TCP_PORT_START`: 8000
- `DEFAULT_TCP_PORT_END`: 9000
- `DEFAULT_BIND_IP`: "0.0.0.0"
- `BROADCAST_ADDR`: "255.255.255.255"

**Buffer Sizes:**
- `UDP_BUFFER_SIZE`: 65535 bytes (max UDP packet)
- `TCP_BUFFER_SIZE`: 4096 bytes (file transfer chunks)

**Timing:**
- `DEFAULT_HEARTBEAT_INTERVAL`: 60 seconds
- `DEFAULT_PEER_TIMEOUT`: 180 seconds (3 minutes)
- `DEFAULT_OFFLINE_MESSAGE_RETENTION_DAYS`: 30 days

All constants defined in `src-tauri/src/config/app.rs` as `impl AppConfig` associated constants.

### Architecture Patterns

#### Frontend Structure
```
src/
├── components/           # Organized by feature section
│   ├── shell/           # App shell (navigation, layout, user menu)
│   ├── basic-settings/  # Settings pages (network config, personal info)
│   ├── messaging/       # Chat components (bubbles, input, conversation list)
│   ├── file-transfer/   # File transfer UI (progress, file list)
│   ├── collaboration/   # Screen share/capture tools
│   ├── contacts/       # Contact management (list, filters, dialogs, groups)
│   └── organization/   # Org chart tree (department tree, user cards)
├── lib/
│   ├── api/            # API client wrappers (peers, messages, config, etc.)
│   ├── types/          # Type definitions per section
│   ├── converters/     # Data transformation utilities
│   ├── events/         # Real-time event manager and type definitions
│   └── utils.ts        # Shared utilities (cn, formatFileSize, formatSpeed)
├── hooks/              # Custom React hooks (useConfig, usePeers, useMessages, etc.)
└── main.tsx            # Application entry point
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
4. State changes emit events via `app_handle.emit('event_name', payload)` for real-time frontend updates
5. Frontend listens via `eventsManager.onEvent('event_name', handler)` for real-time updates
6. Frontend polls events via `poll_events()` command for fallback/initial events

#### Database Layer
- Entities defined in `storage/entities/`
- Migrations in `migration/`
- Repository functions in `storage/*_repo.rs`

#### Real-time Event System

**Backend (Rust):**
- Events are emitted via `app_handle.emit('event_name', &payload)`
- Common events: `message-received`, `peer-online`, `peer-offline`, `file-transfer-request`, `peers-discovered`, `message-receipt-ack`
- Background task forwards events from `mpsc::channel` to frontend via Tauri's event emitter

**Frontend (TypeScript):**
- Event manager in `lib/events/manager.ts` handles real-time updates
- Use `eventsManager.onEvent<T>('event_name', handler)` to subscribe
- Use `poll_events()` command to fetch queued events (fallback/initial)
- Custom hooks auto-subscribe to events (e.g., `useConfig`, `usePeers`, `useMessages`)

**Event Types:**
```typescript
// Example event usage
onEvent<MessageReceivedEvent>('message-received', (event) => {
  // Handle new message
  console.log(`Message from ${event.senderName}: ${event.content}`)
})

// Command for polling fallback
await invoke<events[]>('poll_events')
```

#### Frontend State Management Pattern

**Custom Hooks Pattern:**
- State is managed via custom hooks in `hooks/` directory
- Each hook fetches data, manages loading/error states, and auto-subscribes to events
- Example hooks: `useConfig`, `usePeers`, `useMessages`, `useContacts`, `useFileTransfers`
- Hooks return `{ data, isLoading, error, refresh }` pattern

**Zustand for UI State:**
- Use Zustand stores for transient UI state (modals, selections, local UI state)
- Keep business logic data in custom hooks with event subscriptions
- Example: dialog open/close states, selected items, theme preferences

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

#### Code Quality & Linting

**Frontend:**
- No ESLint/Prettier configuration (intentional - rely on TypeScript strict mode)
- Use TypeScript strict mode (`strict: true` in tsconfig.json)
- Use `cn()` utility for consistent class name merging
- Follow existing code patterns in components

**Backend:**
- Use `cargo clippy` for linting
- Use `cargo fmt` for formatting
- Run tests with `cargo test`
- Follow Rust naming conventions and idiomatic patterns
- Use `#[allow(dead_code)]` for test-only code

**Pre-commit:**
- Run `cargo test` before committing Rust changes
- Run `npm run build` (or `bun run build`) to verify TypeScript compilation
- No automated pre-commit hooks configured

## Git Workflow

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
4. Create DTOs with `#[serde(rename_all = "camelCase")]` for frontend serialization
5. Add TypeScript usage examples in Rust documentation comments

**Example IPC Command Pattern:**
```rust
#[tauri::command]
pub fn get_peers(state: tauri::State<AppState>) -> Result<Vec<PeerDto>> {
    // Frontend Usage:
    // const peers = await invoke<PeerDto[]>("get_peers");
    let peers = state.get_peers();
    Ok(peers.iter().map(PeerDto::from_peer_node).collect())
}
```

### Creating Custom Hooks

1. Create hook in `feiqiu/src/hooks/use[Feature].ts`
2. Add JSDoc documentation with @example usage
3. Return `{ data, isLoading, error, refresh }` pattern
4. Subscribe to relevant events using `eventsManager.onEvent()`
5. Use `useRef` for mounted state to prevent state updates after unmount

**Example Hook Pattern:**
```typescript
export function useFeature(options: Options = {}): UseFeatureResult {
  const [data, setData] = useState(null)
  const [isLoading, setIsLoading] = useState(false)
  const [error, setError] = useState(null)

  const fetchData = useCallback(async () => {
    // Fetch logic
  }, [dependencies])

  // Subscribe to events
  useEffect(() => {
    const sub = onEvent<EventType>('event-name', (event) => {
      setData(event.data)
    })
    return () => sub.remove()
  }, [])

  return { data, isLoading, error, refresh: fetchData }
}
```

## Theme & Design System

### Custom Animations
Defined in `src/index.css`:
- `.animate-fade-in` - Slide up + fade in (0.3s)
- `.animate-slide-in` - Slide from left (0.3s)
- `.animate-scale-in` - Scale up + fade in (0.2s)
- `pulse-ring` - Pulse animation for status indicators

### Custom Scrollbar
- Width/height: 8px
- Track: slate-100 (light) / slate-800 (dark)
- Thumb: slate-300 / slate-600, rounded, hover state
- Consistent across all scrollable containers

### Typography
- **Sans**: Inter, system fonts
- **Mono**: IBM Plex Mono, Fira Code
- Apply font via Tailwind classes or CSS variables

### Interaction Transitions
- All interactive elements (buttons, inputs, links) have `transition-colors duration-150`
- Focus outline: `outline-2 outline-offset-2 outline-emerald-500`

### Dark Mode
- Enabled via `color-scheme: light dark` in CSS
- Use `dark:` prefix for dark mode styles
- Toggle by adding/removing `.dark` class on `html` or `body`

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

**Encoding Compatibility:**
- IPMsg protocol uses GBK encoding (Chinese Windows standard)
- FeiQiu uses UTF-8 internally for better Unicode support
- `encoding_rs` crate used for encoding conversion when communicating with legacy IPMsg clients
- UTF-8 → GBK conversion when sending to legacy clients
- GBK → UTF-8 conversion when receiving from legacy clients

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

## Responsive Design & Accessibility

### Breakpoints
- **Mobile**: `max-width: 768px`
- **Tablet**: `max-width: 1024px`
- **Desktop**: `min-width: 1025px`

### Accessibility Features
- **Reduced Motion**: All animations respect `prefers-reduced-motion: reduce`
- **Focus Visible**: Custom outline style with `outline-2 outline-emerald-500`
- **Color Contrast**: Follow WCAG AA guidelines (emerald/blue on slate backgrounds)
- **Keyboard Navigation**: All interactive elements accessible via keyboard
- **ARIA Support**: Add appropriate ARIA attributes for screen readers

### Responsive Pattern Example
```typescript
// Mobile: Bottom navigation bar
// Tablet: Left sidebar (collapsed)
// Desktop: Left sidebar (full width)

const isMobile = useMediaQuery('(max-width: 768px)')
const isTablet = useMediaQuery('(max-width: 1024px)')

if (isMobile) return <MobileLayout />
if (isTablet) return <TabletLayout />
return <DesktopLayout />
```

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
- **Tauri 2** - Desktop framework (bundled with app)
- **Tauri Plugins**: store, shell, dialog, opener
- **SQLite** - Embedded database (via sea-orm)

### Build Dependencies
- **Bun** (recommended) or Node.js 18+ - Package management
- **Rust toolchain** (cargo, rustc, edition 2021)
- **System dependencies**:
  - **macOS**: Xcode Command Line Tools
  - **Linux**: libwebkit2gtk-4.0-dev, build-essential, curl, wget, file, libssl-dev
  - **Windows**: Microsoft C++ Build Tools + WebView2

### Key Rust Crates
- `tauri` v2 - Desktop framework
- `tauri-plugin-*` - Shell, dialog, opener plugins
- `sea-orm` v1 - Database ORM with SQLite support
- `tokio` v1 - Async runtime (full features)
- `serde` v1 + `serde_json` - Serialization
- `tracing` + `tracing-subscriber` - Logging
- `chrono` - Date/time handling
- `uuid` v1 - UUID generation
- `whoami` - System username/hostname detection
- `dirs` - Cross-platform directory paths
- `encoding_rs` - GBK encoding for IPMsg compatibility

### Key Frontend Packages
- `react` + `react-dom` v18 - UI framework
- `@tauri-apps/api` + plugins - Tauri APIs
- `tailwindcss` v4 - Styling (no config file required)
- `vite` v5 - Build tool
- `zustand` v5 - State management
- `lucide-react` - Icon library
- `date-fns` - Date/time utilities
 - `clsx` + `tailwind-merge` - Class name utilities

## Development Commands Reference

```bash
# Package manager (Bun recommended, npm also supported)
cd feiqiu
bun install              # Or: npm install

# Frontend dev server (http://localhost:1420)
cd feiqiu && bun run dev          # Or: npm run dev

# Full Tauri app (frontend + backend)
cd feiqiu && bun run tauri dev    # Or: npm run tauri dev

# Build for production
cd feiqiu && bun run tauri build  # Or: npm run tauri build

# Frontend build only
cd feiqiu && bun run build        # TypeScript + Vite
cd feiqiu && bun run preview      # Preview production build

# Rust backend only
cd feiqiu/src-tauri && cargo build
cd feiqiu/src-tauri && cargo test
cd feiqiu/src-tauri && cargo clippy

# Run Rust examples (e.g., peer discovery)
cd feiqiu/src-tauri && cargo run --example feiq_discovery
```

## Related Documentation

- `CLAUDE.md` - Main project overview and patterns
- `design-system/` - Color palette, typography, UI tokens
- `sections/` - Component specifications by feature
- `instructions/` - Implementation guides
