# 飞秋 (FeiQiu) - Product Handoff Package

Complete product design and implementation guide for 飞秋, a LAN instant messaging tool for enterprise internal communication.

## Quick Start

This package contains everything you need to implement 飞秋:

1. **Read the Product Overview** - Start with `product-overview.md` to understand the product vision
2. **Choose Your Implementation Approach**:
   - **One-Shot**: Use `prompts/one-shot-prompt.md` for full implementation in a single session
   - **Incremental**: Use `prompts/section-prompt.md` with milestone-by-milestone instructions
3. **Follow the Instructions**: Detailed implementation guides in `instructions/`
4. **Use the Components**: Ready-to-use React components in `sections/` and `shell/`

## Package Contents

### 📋 Product Definition
- `product-overview.md` - Product description, problems/solutions, features
- Design tokens (colors, typography)
- Data model with entity definitions

### 🏗️ Application Shell
- Shell specification and components
- Three-column layout (60px nav, 280px middle, flex-1 right)
- Navigation and user menu

### 📦 Sections (5 total)
1. **Basic Settings** (`sections/basic-settings/`)
   - Personal info management
   - Network configuration (UDP port settings)
2. **Messaging** (`sections/messaging/`)
   - Single/group chat
   - Message enhancement features (recall, reactions, read status)
3. **File Transfer** (`sections/file-transfer/`)
   - LAN file/folder transfer
   - Progress visualization
4. **Collaboration Tools** (`sections/collaboration/`)
   - Screen capture and sharing
   - Remote assistance
5. **Organization Chart** (`sections/organization/`)
   - Department tree view
   - Colleague list with status

### 📝 Implementation Guides
- `prompts/one-shot-prompt.md` - Full implementation prompt
- `prompts/section-prompt.md` - Section-by-section template
- `instructions/one-shot-instructions.md` - Combined milestone guide
- `instructions/incremental/` - Individual milestone instructions

### 🧪 Test Instructions
Each section includes `tests.md` with TDD specifications.

## Design System

### Colors
- **Primary**: Emerald (emerald-500, emerald-600)
- **Secondary**: Blue (blue-500, blue-600)
- **Neutral**: Slate (slate-50 to slate-900)

### Typography
- **Headings**: Inter
- **Body**: Inter
- **Mono**: IBM Plex Mono

## Component Architecture

All components are:
- **Props-based**: Data and callbacks passed via props
- **Portable**: Framework-agnostic design patterns
- **Responsive**: Mobile-first with Tailwind CSS v4
- **Dark Mode**: Full light/dark mode support

## Implementation Order

### One-Shot Approach
1. Foundation (setup + shell)
2. All sections together
3. Integration testing

### Incremental Approach
1. `01-foundation.md` - Project setup and shell
2. `02-basic-settings.md` - Basic Settings section
3. `03-messaging.md` - Messaging section
4. `04-file-transfer.md` - File Transfer section
5. `05-collaboration.md` - Collaboration Tools section
6. `06-organization.md` - Organization Chart section

## Tech Stack

- **Framework**: React 18+ (can be adapted to any framework)
- **Styling**: Tailwind CSS v4
- **Language**: TypeScript
- **Runtime**: Node.js 18+

## Questions?

Before implementation, clarify with your team:
1. **Authentication**: How will users authenticate?
2. **User Modeling**: How will user sessions be managed?
3. **Tech Stack**: Confirm framework (Next.js, Vite, CRA, etc.)
4. **State Management**: Redux, Zustand, Context, or other?
5. **Backend**: What backend services for messaging, file transfer?

## Support

For questions or issues with this handoff package, refer to the original design team or product manager.

---

**Version**: 1.0.0
**Last Updated**: 2025
