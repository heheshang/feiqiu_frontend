# Proposal: Add Contacts (Address Book) Feature

**Change ID:** `add-contacts-feature`
**Status:** Draft
**Created:** 2025-01-14

## Overview

Develop a comprehensive Contacts (Address Book / 通讯录) feature for the FeiQiu LAN messaging application. This feature will allow users to manage all peers (both online and offline) from a centralized interface with support for custom grouping, department-based organization, and full CRUD operations on contact records.

## Problem Statement

Currently, the FeiQiu application has a placeholder "通讯录功能开发中" (Contacts feature under development) in the navigation. Users cannot:
- View a comprehensive list of all contacts (online + offline/historical)
- Organize contacts into custom groups
- Add manual contact entries for users not currently online
- Set notes or favorites on contacts
- Search and filter contacts efficiently
- Perform batch operations on contacts

The existing `peers` system only tracks currently visible LAN peers, with no persistent contact management capabilities.

## Goals

### Primary Goals
1. **Contact Management**: View, add, edit, and delete contact records
2. **Multiple Organization Views**: Support both custom groups and department-based views
3. **Search & Filter**: Quick lookup by name, department, position, pinyin, IP
4. **Favorites & Notes**: Mark important contacts and add personal notes
5. **Batch Operations**: Select multiple contacts for batch actions

### Non-Goals
- Contact synchronization with external systems (outside scope)
- Advanced privacy settings (future consideration)
- Contact sharing between users (future consideration)

## Proposed Solution

### Capabilities

This change introduces the following capabilities:

1. **Contact List Management** (`contacts-list`)
   - Display all contacts (online + historical)
   - Real-time online status updates
   - Sort and filter capabilities

2. **Contact Groups** (`contact-groups`)
   - Create custom groups (e.g., "Team", "Friends", "VIP")
   - Add/remove contacts from groups
   - Group-based navigation

3. **Contact CRUD Operations** (`contact-crud`)
   - Add manual contact entries
   - Edit contact details (nickname, notes, phone, email)
   - Delete contacts
   - Merge duplicate contacts

4. **Contact Search & Filter** (`contact-search`)
   - Full-text search across all contact fields
   - Pinyin search support
   - Filter by status, group, department

5. **Batch Operations** (`contact-batch`)
   - Multi-select interface
   - Batch delete, move to group, export

### Architecture Decisions

See `design.md` for detailed architectural decisions.

## Impact

### User Interface
- New main navigation tab: "通讯录" (Contacts)
- New components in `src/components/contacts/`
- New types in `src/lib/types/contacts.ts`

### Backend (Rust)
- New storage entity: `contacts` table
- New commands: `get_contacts`, `create_contact`, `update_contact`, `delete_contact`
- New storage repository: `contact_repo.rs`

### Database Schema
- New table: `contacts` (extends `peers` with user-editable fields)
- New table: `contact_groups` (custom groups)
- New table: `contact_group_members` (many-to-many relationship)

### Dependencies
- None (builds on existing `peers` infrastructure)

## Success Criteria

1. Users can view all historical contacts (not just online peers)
2. Users can create custom groups and organize contacts
3. Users can search contacts by name, pinyin, department, position
4. Users can add/edit notes and nicknames on contacts
5. Users can perform batch operations on selected contacts
6. Contact data persists across app restarts

## Open Questions

None - requirements clarified via user questionnaire.

## Related Changes

- Depends on: Existing `peers` system
- Enables: Future contact sharing/sync features
- Related to: Organization chart feature (department data)

## Alternatives Considered

1. **Groups-only approach**: Rejected - users need both group AND department views
2. **Online-only contacts**: Rejected - users want to manage offline contacts too
3. **Auto-organization only**: Rejected - users want manual control over groups

## References

- Existing peers system: `src-tauri/src/modules/peer/`
- Organization chart: `sections/organization/`
- Data model: `data-model/types.ts`
