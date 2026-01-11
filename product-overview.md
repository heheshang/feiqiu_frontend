# 飞秋 — Product Overview

## Summary

飞秋是一款面向办公内网的局域网即时通讯工具，通过现代化的UI设计和智能的组织架构，解决企业内跨部门协作沟通难题，让同事间的沟通更高效、文件传输更便捷。

## Problems & Solutions

### Problem 1: 跨部门沟通困难
提供清晰的**组织架构视图**和同事在线状态，让用户快速找到并联系任何人。

### Problem 2: 文件传输麻烦
支持**拖拽发送**、局域网高速传输、传输进度可视化，替代U盘和邮件的低效传输方式。

### Problem 3: 消息通知混乱
**智能消息分类**（私聊、群聊、系统通知）、消息已读未读、@提醒功能，确保重要消息不被遗漏。

### Problem 4: 找不到人
**实时在线状态**显示、智能搜索同事（按姓名、部门），快速定位需要联系的人。

## Key Features
- 单聊/私聊 - 一对一即时消息沟通
- 群聊/讨论组 - 支持部门群、项目群等多人群组沟通
- 文件传输 - 局域网高速传输文件、文件夹
- 屏幕共享/远程 - 支持屏幕截图、共享和远程协助
- 个人信息管理 - 头像、昵称、签名、在线状态设置
- 网络配置 - UDP端口设置、网络连接检测
- 消息增强功能 - 消息撤回、引用回复、表情回复、已读未读
- 组织架构视图 - 按部门浏览同事，快速发起沟通

## Planned Sections

1. **[基础设置]** — 网络配置（UDP端口）、个人信息管理（头像、昵称、签名、状态）
2. **[消息通讯]** — 单聊、群聊、消息增强功能（撤回、引用、表情、已读未读）
3. **[文件传输]** — 局域网文件/文件夹传输、拖拽发送、传输进度可视化
4. **[协作工具]** — 屏幕截图、屏幕共享、远程协助功能
5. **[组织架构]** — 部门视图、同事列表、在线状态、智能搜索

## Data Model

**Entities:**
- User (用户)
- Message (消息)
- Conversation (会话)
- Group (群组)
- File (文件)
- Department (部门)
- NetworkConfig (网络配置)
- ScreenShare (屏幕共享)

## Design System

**Colors:**
- Primary: emerald
- Secondary: blue
- Neutral: slate

**Typography:**
- Heading: Inter
- Body: Inter
- Mono: IBM Plex Mono

## Implementation Sequence

Build this product in milestones:

1. **Foundation** — Set up design tokens, data model types, and application shell
2. **基础设置** — Network configuration and personal profile management
3. **消息通讯** — Single/group chat with message enhancement features
4. **文件传输** — LAN file/folder transfer with progress visualization
5. **协作工具** — Screen capture, sharing, and remote assistance
6. **组织架构** — Department tree view and colleague list with status

Each milestone has a dedicated instruction document in `product-plan/instructions/`.
