# 消息通讯 (Messaging)

## Overview

消息通讯是飞秋的核心功能，提供双栏布局的聊天界面，支持单聊和群聊。用户可以发送文字、表情、图片消息，支持@提醒、消息撤回、引用回复等增强功能。

## User Flows

- 打开消息通讯，左侧显示会话列表（包含单聊和群聊）
- 点击会话进入聊天窗口，查看历史消息
- 在输入框输入文字，或通过工具栏添加表情、图片、@提醒
- 发送消息（回车键），消息显示在聊天窗口
- 长按消息可以撤回、引用回复或添加表情反应
- 已发送消息显示已读/未读状态
- 会话列表显示最后消息预览、时间和未读数

## Data Used

**Entities:**
- Conversation
- Message
- User
- Group

**From global model:** Conversation, Message, User, MessageType, MessageStatus

## Visual Reference

See `screenshot.png` for the target UI design.

## Components Provided

- `Messaging.tsx` — Main chat interface with conversation list and message view
- `ConversationItem.tsx` — Individual conversation list item
- `MessageBubble.tsx` — Message bubble component for chat
- `MessageInput.tsx` — Message input with emoji picker and file upload

## Callback Props

| Callback | Description |
|----------|-------------|
| `onSendMessage` | Called when user sends a text message |
| `onSendImage` | Called when user uploads an image |
| `onConversationSelect` | Called when user clicks on a conversation |
| `onMessageReply` | Called when user replies to a message |
| `onMessageReact` | Called when user adds emoji reaction |
| `onMessageRetract` | Called when user retracts a message |
