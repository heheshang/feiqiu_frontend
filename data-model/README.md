# Data Model

## Entities

### User (用户)
使用飞秋的每个人，包含头像、昵称、个性签名、在线状态、所属部门等个人信息。

### Message (消息)
会话中发送的内容，可以是文字、表情、引用回复、文件附件等类型。

### Conversation (会话)
一个对话，可以是单聊会话或群聊会话，包含最后消息时间、未读数等摘要信息。

### Group (群组)
多人群组，如部门群、项目群、讨论组，包含群名称、成员列表、群主/创建者。

### File (文件)
通过局域网传输的文件或文件夹，包含文件名、大小、传输状态、传输进度。

### Department (部门)
组织架构中的部门，用于分类和管理用户。

### NetworkConfig (网络配置)
UDP端口设置、监听地址、网络连接参数等配置信息。

### ScreenShare (屏幕共享)
屏幕截图、屏幕共享和远程协助会话，包含发起者、接收者、状态。

## Relationships

- User has many Messages
- User has many Conversations (as participant)
- User belongs to a Department
- User has many ScreenShares (as发起者 or 接收者)
- Message belongs to a Conversation and a User (sender)
- Conversation has many Messages
- Conversation belongs to two Users (单聊) or one Group (群聊)
- Group has many Users (members)
- User has many Groups (as member)
- File belongs to a Message or Conversation
- ScreenShare belongs to a User (发起者) and a User (接收者)

## Usage

See `types.ts` for TypeScript interface definitions and `sample-data.json` for example data structures.

These types are used across all sections and should be implemented at the foundation stage.
