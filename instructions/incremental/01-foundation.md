# Milestone 1: Foundation (Project Setup)

## Overview

Set up the development environment, configure the design system, and establish the project structure using Rust Tauri 2.0.

## Prerequisites

Before starting, decide:
- Frontend framework: React, Vue, Svelte, or SolidJS?
- Package manager: npm, pnpm, or yarn?
- State management: Tauri stores (leverage Rust), Zustand, Jotai, or Context API?
- Database: SQLite (Tauri native), PostgreSQL, or separate service?

---

## Step 1: Initialize Tauri 2.0 Project

### Prerequisites

Install Rust if not already installed:

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

Install Tauri CLI:

```bash
cargo install tauri-cli
```

### Create Tauri Project

```bash
npm create tauri-app@latest feiqiu
```

When prompted:
- **Which bundle preset would you like to use?** → Select `rust` (or `both` if you want a separate Rust core)
- **What UI template would you like to use?** → Select your preferred framework:
  - `react` (recommended for this project)
  - `vue`
  - `svelte`
  - `solid`
  - `vanilla`
- **Add typeScript support?** → Yes
- **Add ESLint for code linting?** → Yes

```bash
cd feiqiu
```

### Verify Installation

```bash
cargo tauri info
```

Expected output should show:
- Tauri: 2.0.x
- Rust: 1.70+
- Frontend: [your choice]

---

## Step 2: Configure Tailwind CSS v4

### Install Dependencies

```bash
npm install -D tailwindcss@next @tailwindcss/postcss@next
```

### Configure PostCSS

Create or update `postcss.config.js` in the frontend root:

```javascript
export default {
  plugins: {
    '@tailwindcss/postcss': {},
  },
}
```

### Configure Design Tokens in CSS

For **React/Vue/Svelte** frontend (`src/styles.css` or equivalent):

```css
@import "tailwindcss";

@theme {
  --color-primary-50: #ecfdf5;
  --color-primary-100: #d1fae5;
  --color-primary-500: #10b981;
  --color-primary-600: #059669;

  --color-secondary-500: #3b82f6;
  --color-secondary-600: #2563eb;

  --font-sans: 'Inter', sans-serif;
  --font-mono: 'IBM Plex Mono', monospace;
}

:root {
  color-scheme: light dark;
}

.dark {
  color-scheme: dark;
}
```

---

## Step 3: Configure Dark Mode

### Create Dark Mode Hook

Create `src/hooks/useDarkMode.ts`:

```typescript
import { useEffect, useState } from 'react';

type Theme = 'light' | 'dark';

export function useDarkMode() {
  const [theme, setTheme] = useState<Theme>(() => {
    const saved = localStorage.getItem('theme');
    if (saved) return saved as Theme;
    return window.matchMedia('(prefers-color-scheme: dark)').matches ? 'dark' : 'dark';
  });

  useEffect(() => {
    const root = document.documentElement;
    root.classList.remove('light', 'dark');
    root.classList.add(theme);
    localStorage.setItem('theme', theme);
  }, [theme]);

  const toggleTheme = () => {
    setTheme((prev) => (prev === 'light' ? 'dark' : 'light'));
  };

  return { theme, toggleTheme };
}
```

### Persist Theme to Tauri Store (Optional)

For cross-session persistence, create `src/stores/themeStore.ts`:

```typescript
import { get, set } from '@tauri-apps/store';

export async function getTheme(): Promise<'light' | 'dark'> {
  const saved = await get<string>('theme');
  return (saved as 'light' | 'dark') || 'dark';
}

export async function setTheme(theme: 'light' | 'dark'): Promise<void> {
  await set('theme', theme);
}
```

---

## Step 4: Configure TypeScript

Update `tsconfig.json` in frontend:

```json
{
  "compilerOptions": {
    "target": "ES2020",
    "lib": ["ES2020", "DOM", "DOM.Iterable"],
    "jsx": "preserve",
    "module": "ESNext",
    "moduleResolution": "bundler",
    "resolveJsonModule": true,
    "allowJs": true,
    "strict": true,
    "noEmit": true,
    "esModuleInterop": true,
    "skipLibCheck": true,
    "forceConsistentCasingInFileNames": true,
    "isolatedModules": true,
    "paths": {
      "@/*": ["./src/*"]
    }
  },
  "include": ["src"],
  "exclude": ["node_modules"]
}
```

---

## Step 5: Create Folder Structure

### Frontend Structure

```bash
mkdir -p src/components/shell
mkdir -p src/components/basic-settings
mkdir -p src/components/messaging
mkdir -p src/components/file-transfer
mkdir -p src/components/collaboration
mkdir -p src/components/organization
mkdir -p src/lib/types
mkdir -p src/lib/utils
mkdir -p src/hooks
mkdir -p src/stores
mkdir -p src/assets
```

### Rust Core Structure (if using separate Rust core)

```bash
mkdir -p src-tauri/src/core
mkdir -p src-tauri/src/ipc
mkdir -p src-tauri/src/models
mkdir -p src-tauri/src/services
```

---

## Step 6: Install Additional Dependencies

```bash
# Icons
npm install lucide-react

# Date handling
npm install date-fns

# State management (optional - can use Rust stores)
npm install zustand

# Forms (optional)
npm install react-hook-form zod

# Utilities
npm install clsx tailwind-merge

# Tauri plugins (optional)
npm install @tauri-apps/plugin-store
npm install @tauri-apps/plugin-shell
npm install @tauri-apps/plugin-dialog
```

---

## Step 7: Create Utility Functions

Create `src/lib/utils.ts`:

```typescript
import { type ClassValue, clsx } from 'clsx';
import { twMerge } from 'tailwind-merge';

export function cn(...inputs: ClassValue[]) {
  return twMerge(clsx(inputs));
}

export function formatFileSize(bytes: number): string {
  if (bytes === 0) return '0 B';
  const k = 1024;
  const sizes = ['B', 'KB', 'MB', 'GB'];
  const i = Math.floor(Math.log(bytes) / Math.log(k));
  return `${parseFloat((bytes / Math.pow(k, i)).toFixed(2))} ${sizes[i]}`;
}

export function formatSpeed(bytesPerSecond: number): string {
  return formatFileSize(bytesPerSecond) + '/s';
}
```

---

## Step 8: Configure Tauri Permissions

Update `src-tauri/capabilities/default.json` to add required permissions:

```json
{
  "identifier": "default",
  "description": "Default capabilities for the main window",
  "windows": ["main"],
  "permissions": [
    "core:default",
    "core:window:default",
    "core:window:app:default",
    "core:app:default",
    "shell:allow-open",
    {
      "identifier": "network:default",
      "allow": [
        {
          "host": "0.0.0.0",
          "port": "0-65535",
          "type": "tcp"
        },
        {
          "host": "0.0.0.0",
          "port": "0-65535",
          "type": "udp"
        }
      ]
    }
  ]
}
```

Update `src-tauri/tauri.conf.json`:

```json
{
  "productName": "feiqiu",
  "version": "0.1.0",
  "identifier": "com.feiqiu.app",
  "build": {
    "beforeDevCommand": "npm run dev",
    "devUrl": "http://localhost:1420",
    "beforeBuildCommand": "npm run build",
    "frontendDist": "../dist"
  },
  "app": {
    "windows": [
      {
        "title": "飞秋",
        "width": 1200,
        "height": 800,
        "minWidth": 800,
        "minHeight": 600,
        "resizable": true,
        "fullscreen": false
      }
    ],
    "security": {
      "csp": null
    }
  },
  "bundle": {
    "active": true,
    "targets": "all",
    "icon": [
      "icons/32x32.png",
      "icons/128x128.png",
      "icons/128x128@2x.png",
      "icons/icon.icns",
      "icons/icon.ico"
    ]
  }
}
```

---

## Step 9: Create Index File

Create `src/lib/types/index.ts` to export all types:

```typescript
// Types will be added in each milestone
export * from './basic-settings'
export * from './messaging'
export * from './file-transfer'
export * from './collaboration'
export * from './organization'
```

---

## Step 10: Build Rust Core (Optional)

If using separate Rust core for networking, create `src-tauri/src/lib.rs`:

```rust
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

pub mod ipc;
pub mod services;

use tauri::Manager;

#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! Welcome to 飞秋!", name)
}

#[tauri::command]
async fn get_system_info() -> serde::Serialize {
    #[cfg(target_os = "windows")]
    let platform = "windows";
    #[cfg(target_os = "macos")]
    let platform = "macos";
    #[cfg(target_os = "linux")]
    let platform = "linux";

    serde_json::json!({
        "platform": platform,
        "version": "1.0.0"
    })
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![greet, get_system_info])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
```

---

## Verification

Test that your setup is working:

1. **Start dev server**: `npm run tauri dev`
2. **Verify**:
   - [ ] App window opens without errors
   - [ ] TypeScript compiles successfully
   - [ ] Tailwind CSS classes work
   - [ ] Dark mode toggle works
   - [ ] Rust backend responds to IPC calls

---

## Next Steps

Once foundation is complete, proceed to **[Milestone 2: Application Shell](./02-shell.md)**.

---

## Tech Stack Summary

| Layer | Technology |
|-------|------------|
| Runtime | Tauri 2.0 |
| Backend | Rust |
| Frontend Framework | React 18+ (or Vue/Svelte/Solid) |
| Styling | Tailwind CSS v4 |
| Language | TypeScript + Rust |
| Package Manager | pnpm (recommended) or npm/yarn |
| State Management | Rust stores + Zustand (optional) |
