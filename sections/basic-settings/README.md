# 基础设置 (Basic Settings)

## Overview

基础设置页面提供单页面多选项卡界面，让用户管理网络配置和个人信息。用户可以修改UDP端口、编辑个人资料、更改在线状态，并查看当前网络连接状态。

## User Flows

- 打开基础设置页面，默认显示"个人信息"选项卡
- 在个人信息选项卡中，查看和编辑头像、昵称、个性签名
- 更改在线状态（在线、忙碌、离开、离线）
- 查看网络状态卡片（IP地址、连接状态、端口信息）
- 切换到"网络设置"选项卡
- 修改UDP端口、绑定地址等网络配置
- 保存更改，系统提示网络配置需要重启服务才能生效

## Data Used

**Entities:**
- User
- NetworkConfig
- NetworkStatus

**From global model:** User, NetworkConfig, ConnectionStatus

## Visual Reference

See `screenshot.png` for the target UI design.

## Components Provided

- `BasicSettings.tsx` — Main settings page with tabs
- `NetworkStatusCard.tsx` — Network status display card

## Callback Props

| Callback | Description |
|----------|-------------|
| `onTabChange` | Called when user switches between profile and network tabs |
| `onUpdateUser` | Called when user saves profile changes |
| `onUploadAvatar` | Called when user uploads a new avatar |
| `onStatusChange` | Called when user changes online status |
| `onSaveNetworkConfig` | Called when user saves network configuration |
| `onCancelNetworkConfig` | Called when user cancels network configuration changes |
