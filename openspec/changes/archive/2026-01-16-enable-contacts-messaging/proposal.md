# Change: Enable Contacts List Messaging

## Why

Users can currently click the "发消息" (Send Message) button on a contact in the contacts list, but this only works when the contact has a linked `peerId`. For contacts without a linked peer (offline contacts, manually added contacts, etc.), clicking the button does nothing. This creates an inconsistent user experience where some contacts can be messaged and others cannot.

## What Changes

- Add `ipAddress` field to Contact type and database entity for storing peer IP addresses
- Implement IP-to-peer resolution when starting a conversation from contacts
- Add fallback logic to handle contacts without active peer connections
- Update the "发消息" button to show appropriate state (disabled/online/offline)
- Add user feedback for messaging unavailable contacts
- Create backend command to find peer by IP address

## Impact

- **User-facing**: Users can initiate conversations from any contact, with clear feedback when contact is unavailable
- **Technical**: Extends contact schema and adds new peer resolution logic
- **Breaking**: None (additive changes)
- **Dependencies**: None

---

## Summary

**Change ID:** `enable-contacts-messaging`
**Status:** Draft
**Created:** 2025-01-16
**Author:** AI Assistant

## Problem Statement

### Current State

**Contacts List Current Behavior:**
- Clicking a contact opens `ContactDetailDialog`
- Dialog has "发消息" button that calls `handleSendMessage`
- `handleSendMessage` checks `if (contact.peerId && onStartConversation)`
- If `peerId` exists, it starts conversation and switches to chat tab
- If `peerId` is `undefined`, nothing happens

**Code from Contacts.tsx (lines 93-99):**
```typescript
const handleSendMessage = (contact: Contact) => {
  // Start conversation with this contact
  if (contact.peerId && onStartConversation) {
    onStartConversation(contact.peerId.toString())
  }
  setShowDetailDialog(false)
}
```

### The Gap

| Scenario | Current Behavior | Expected Behavior |
|----------|-----------------|-------------------|
| Contact with linked online peer | Works correctly | Works correctly |
| Contact with linked offline peer | Does nothing | Should allow messaging attempt |
| Manually added contact (no peerId) | Does nothing | Should allow messaging if peer found |
| Contact with no matching peer | Does nothing | Should show informative error |

**Root Causes:**
1. Contact type lacks `ipAddress` field to store peer IP
2. No fallback to find peer by name when `peerId` is missing
3. No user feedback when messaging is unavailable
4. "发消息" button state doesn't reflect contact availability

## Proposed Solution

### Phase 1: Data Model Enhancement

1. **Add `ipAddress` to Contact type**
   - Optional field to store associated peer IP address
   - Updated when peer is discovered
   - Allows direct peer lookup for messaging

2. **Update database schema**
   - Add `ip_address` column to `contacts` table
   - Create migration for existing data
   - Update backend entity and DTO

### Phase 2: Backend Enhancements

1. **Add `get_peer_by_ip` command**
   - Find peer by IP address from peer list
   - Returns peer or null if not found
   - Useful for contact-to-peer resolution

2. **Update contact creation/update**
   - Auto-link `peerId` when peer with matching name/IP is found
   - Store `ipAddress` when linking to peer

### Phase 3: Frontend Messaging Logic

1. **Update `handleSendMessage` with fallback logic:**
   ```typescript
   const handleSendMessage = async (contact: Contact) => {
     // Try peerId first
     if (contact.peerId) {
       onStartConversation(contact.peerId.toString())
       return
     }

     // Try finding peer by IP if stored
     if (contact.ipAddress) {
       const peer = await findPeerByIp(contact.ipAddress)
       if (peer) {
         onStartConversation(peer.ip)
         return
       }
     }

     // Try finding peer by name
     const peer = await findPeerByName(contact.name)
     if (peer) {
       // Update contact with linked peer
       await updateContact(contact.id, { peerId: peer.id, ipAddress: peer.ip })
       onStartConversation(peer.ip)
       return
     }

     // Show error: contact not available
     showNotification('该联系人当前不在线，无法发送消息', 'warning')
   }
   ```

2. **Update ContactDetailDialog button state**
   - Show "发消息" button with appropriate state
   - Disable button if contact is offline and no IP stored
   - Add tooltip explaining why disabled

3. **Add notification system**
   - Show toast messages for messaging errors
   - Use existing notification pattern or add new one

## Affected Components

### Frontend Files to Modify

| Path | Changes |
|------|---------|
| `feiqiu/src/lib/types/contacts.ts` | Add `ipAddress?: string` to Contact interface |
| `feiqiu/src/components/contacts/Contacts.tsx` | Update `handleSendMessage` with fallback logic |
| `feiqiu/src/components/contacts/ContactDialogs.tsx` | Update button state in ContactDetailDialog |
| `feiqiu/src/lib/api/contacts.ts` | Add `findPeerForContact` function |
| `feiqiu/src/lib/api/peers.ts` | Add `getPeerByIp` function |

### Backend Files to Modify

| Path | Changes |
|------|---------|
| `src-tauri/src/storage/entities/contacts.rs` | Add `ip_address: Option<String>` field |
| `src-tauri/src/migration/src/m*.rs` | Create migration for `ip_address` column |
| `src-tauri/src/commands/peer.rs` | Add `get_peer_by_ip` command |
| `src-tauri/src/commands/contacts.rs` | Update create/update to store IP address |
| `src-tauri/src/types/dtos.rs` | Update `ContactDto` with `ip_address` field |

## Dependencies

### Required Packages

None - all changes use existing dependencies.

## Success Criteria

### Functional Requirements

- [ ] Contact with `peerId` can start conversation (existing behavior maintained)
- [ ] Contact with `ipAddress` but no `peerId` can start conversation if peer is online
- [ ] Contact without either shows appropriate error message
- [ ] "发消息" button state reflects contact availability
- [ ] IP address is stored when contact is linked to a peer
- [ ] Database migration adds `ip_address` column successfully

### Technical Requirements

- [ ] Type safety maintained for Contact interface
- [ ] Error handling for peer lookup failures
- [ ] No memory leaks from peer lookups
- [ ] Migration is reversible

### User Experience Requirements

- [ ] Clear visual feedback when messaging is unavailable
- [ ] Notification message is user-friendly (Chinese)
- [ ] No console errors from failed messaging attempts

## Alternatives Considered

### Alternative 1: Require Peer Link for Messaging

Only allow messaging from contacts that have a `peerId`. Show error for others.

**Pros:**
- Simpler implementation
- Clear distinction between contacts and active peers

**Cons:**
- Poor UX for offline contacts
- Doesn't solve the core problem
- Users confused why some contacts can't be messaged

**Decision:** Rejected - doesn't meet user needs.

### Alternative 2: Always Create Conversation Without Peer Check

Allow starting conversation with any contact using their IP or a placeholder.

**Pros:**
- Always allows messaging attempt
- Simpler logic

**Cons:**
- Could send messages to non-existent peers
- No feedback about actual availability
- Messages could be lost

**Decision:** Rejected - doesn't provide proper user feedback.

### Alternative 3: Add Contact Availability Indicator

Show online/offline status next to each contact in the list with different actions.

**Pros:**
- Clear visual indication
- Prevents invalid actions

**Cons:**
- Requires more UI changes
- Doesn't solve offline contact messaging
- Additional complexity

**Decision:** Valid enhancement, but separate from this change.

## Risks and Mitigations

| Risk | Impact | Mitigation |
|------|--------|------------|
| IP address changes (DHCP) | Medium | Store peerId for reliable lookup, IP is cache |
| Migration fails | Low | Test migration, provide rollback |
| Peer lookup performance | Low | Simple hash map lookup, negligible |
| Contact name collision | Low | Use IP as primary, name as fallback |

## Related Specs

This change will modify the following specs:

- **MODIFY:** `contacts-list` - Add contact-to-peer resolution requirements
- **MODIFY:** `contact-crud` - Add IP address field to create/update operations

## Open Questions

1. Should we store the last known IP address even after peer goes offline?
   - **Proposed:** Yes, allows attempting connection when peer comes back

2. Should the "发消息" button be disabled for offline contacts?
   - **Proposed:** No, keep enabled but show warning message

3. Should we automatically link peers to contacts by name?
   - **Proposed:** Yes, when sending message, if peer with matching name is found, link it

4. What if multiple peers have the same name?
   - **Proposed:** Show selection dialog or prefer most recently active
