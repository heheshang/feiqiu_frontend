# 协作工具 (Collaboration Tools)

## Overview

协作工具提供屏幕截图和标注功能。用户可以通过全屏、区域选择或活动窗口方式截图，使用箭头、矩形框、文字和画笔进行标注，然后保存、复制或发送给联系人。

## User Flows

- 点击工具栏上的截图按钮（全屏/区域/窗口）
- 截图完成后自动打开标注编辑器
- 使用各种标注工具编辑截图
- 完成后选择保存、复制或发送

## Data Used

**Entities:**
- Screenshot
- Annotation

**From global model:** Screenshot, Annotation, ScreenshotType, ScreenshotStatus

## Visual Reference

See `screenshot.png` for the target UI design.

## Components Provided

- `CollaborationTools.tsx` — Main collaboration tools interface with screenshot toolbar and history

## Callback Props

| Callback | Description |
|----------|-------------|
| `onScreenshot` | Called when user initiates a screenshot |
| `onAddAnnotation` | Called when user adds an annotation |
| `onDeleteAnnotation` | Called when user deletes an annotation |
| `onSave` | Called when user saves a screenshot |
| `onCopy` | Called when user copies a screenshot |
| `onSendToContact` | Called when user sends screenshot to a contact |
| `onSendToChat` | Called when user sends screenshot to a chat |
| `onUndo` | Called when user undoes the last action |
| `onRedo` | Called when user redoes the last action |
