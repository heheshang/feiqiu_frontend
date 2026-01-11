# 组织架构 (Organization Chart)

## Overview

组织架构提供公司部门和同事的可视化视图。用户可以通过双栏布局浏览部门树和同事列表，查看实时在线状态，使用智能搜索快速找到特定同事，并直接发起聊天或查看详情。

## User Flows

- 在左侧部门树中浏览或搜索部门
- 右侧显示选中部门的同事列表
- 使用顶部搜索框快速搜索姓名、部门、职位
- 查看同事在线状态（在线、离开、离线）
- 点击同事卡片发起聊天或查看详细信息

## Data Used

**Entities:**
- Department
- User

**From global model:** Department, User, UserStatus

## Visual Reference

See `screenshot.png` for the target UI design.

## Components Provided

- `OrganizationChart.tsx` — Main organization chart with department tree and colleague list
- `DepartmentTree.tsx` — Recursive department tree component
- `UserCard.tsx` — Individual colleague card with status

## Callback Props

| Callback | Description |
|----------|-------------|
| `onDepartmentSelect` | Called when user selects a department |
| `onStartChat` | Called when user clicks to start a chat |
| `onViewDetails` | Called when user clicks to view colleague details |
| `onSearch` | Called when user searches for colleagues |
