# 文件传输 (File Transfer)

## Overview

文件传输功能支持局域网内用户之间的文件传输。用户可以通过拖拽或文件选择器发送文件，在统一列表中管理所有传输（发送和接收），查看详细的传输进度信息，支持暂停、继续、取消操作。传输历史记录所有传输，包括失败的传输。

## User Flows

- 拖拽文件到聊天窗口或联系人头像进行发送
- 通过文件选择器选择文件发送
- 在传输列表中查看所有正在进行的传输
- 暂停、继续、取消正在传输的文件
- 接收文件时选择保存位置
- 查看传输历史记录，了解所有传输状态

## Data Used

**Entities:**
- FileTransfer
- User

**From global model:** FileTransfer, TransferDirection, TransferStatus

## Visual Reference

See `screenshot.png` for the target UI design.

## Components Provided

- `FileTransfer.tsx` — Main file transfer management interface
- `FileTransferItem.tsx` — Individual transfer item with progress

## Callback Props

| Callback | Description |
|----------|-------------|
| `onPause` | Called when user pauses a transfer |
| `onResume` | Called when user resumes a paused transfer |
| `onCancel` | Called when user cancels a transfer |
| `onRetry` | Called when user retries a failed transfer |
| `onOpenFolder` | Called when user wants to open the file folder |
| `onRedownload` | Called when user wants to re-download a file |
| `onSendFile` | Called when user sends new files |
