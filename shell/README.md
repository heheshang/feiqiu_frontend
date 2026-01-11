# Application Shell

## Overview

飞秋采用微信风格的三栏布局，专为局域网即时通讯设计。左侧主导航、中间列表区域、右侧内容区域，提供清晰的信息层级和高效的操作体验。

## Navigation Structure

- **聊天** → 显示会话列表（私聊、群聊）
- **通讯录** → 显示联系人/同事列表
- **组织架构** → 显示部门树形视图

## User Menu

**位置:** 右上角

**内容:**
- 用户头像
- 用户昵称
- 下拉菜单（个人设置、网络设置、退出登录）

## Layout Pattern

微信风格三栏布局：
- **左侧窄栏（60px）:** 主导航图标，垂直排列
- **中间栏（280px）:** 列表区域（会话/联系人/部门）
- **右侧宽栏（flex-1）:** 内容区域（聊天界面/详情/设置）

## Responsive Behavior

- **桌面端（≥1024px）:** 完整三栏显示
- **平板端（768px-1023px）:** 左侧导航可折叠为图标栏
- **移动端（<768px）:** 单栏显示，滑动切换视图

## Design Notes

- 参考微信PC版布局，用户熟悉度高
- 文件传输功能集成在聊天界面内
- 基础设置（网络配置、个人信息）作为设置页面
- 使用 emerald 主色调呼应微信绿色风格
- 支持 light/dark 模式

## Components Provided

- `AppShell.tsx` — Main layout wrapper with three-column structure
- `MainNav.tsx` — Vertical navigation component with icons
- `UserMenu.tsx` — User avatar dropdown menu with status indicator

## Props

### AppShellProps

| Prop | Type | Description |
|------|------|-------------|
| children | React.ReactNode | Content to display in the main area |
| navigationItems | Array | Navigation items with id, label, icon, href, isActive |
| currentUser | Object | Current user info (name, avatarUrl, status) |
| activeNav | string | ID of the currently active navigation item |
| onNavigate | function | Callback when navigation is clicked |
| onLogout | function | Callback when logout is clicked |
| onSettings | function | Callback when settings is clicked |

### UserMenuProps

| Prop | Type | Description |
|------|------|-------------|
| user | Object | User info (name, avatarUrl, status) |
| onSettings | function | Callback for settings click |
| onLogout | function | Callback for logout click |

## Visual Reference

See `screenshot.png` for the target UI design.
