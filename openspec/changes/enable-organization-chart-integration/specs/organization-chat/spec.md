# organization-chat Specification

## Purpose
Enable users to navigate from the organization chart to start chat conversations with colleagues.

## Requirements

### ADDED Requirement: Display organization chart with peer data

The system MUST display an organization chart showing all LAN peers grouped by department when the user navigates to the "organization" tab.

#### Scenario: User views organization chart for the first time

**Given** the user has opened the FeiQiu application
**And** the user has navigated to the "组织架构" (Organization) tab
**When** the organization chart loads
**Then** the user should see:
  - A hierarchical department tree in the left sidebar
  - A grid of user cards in the main content area
  - Users organized by their respective departments
**And** the chart should display:
  - Online and offline peers discovered on the LAN
  - Each peer's avatar, name, position, and department
  - Online status indicator (green for online, amber for away, gray for offline)

#### Scenario: Organization chart handles users without department information

**Given** the organization chart is displayed
**And** some peers on the LAN have no department/group information
**When** the organization chart loads
**Then** those users should be grouped under a "未分组" (Uncategorized) department
**And** the "未分组" department should appear at the top level of the department tree

#### Scenario: Organization chart updates when peer list changes

**Given** the user is viewing the organization chart
**And** a new peer comes online on the LAN
**When** the peer discovery event is received
**Then** the organization chart should update within 2 seconds to include the new peer
**And** the new peer should appear in their respective department

---

### ADDED Requirement: Navigate to chat from organization chart

The system MUST allow users to start a chat conversation by clicking the "聊天" button on a user card in the organization chart.

#### Scenario: User clicks chat button on a user card

**Given** the user is viewing the organization chart
**And** a user card is displayed for a colleague
**When** the user clicks the "聊天" (Chat) button on the user card
**Then** the application should:
  1. Navigate to the "chat" (messaging) tab
  2. Create or select the conversation with that user
  3. Focus the message input field
**And** the conversation should be identified by the user's IP address

#### Scenario: Chat with offline user from organization chart

**Given** the user is viewing the organization chart
**And** the target user is currently offline
**When** the user clicks the "聊天" button on the offline user's card
**Then** the application should:
  - Navigate to the messaging tab
  - Select/create the conversation for that user
  - Display a visual indicator that the user is offline
**And** messages sent should be queued for delivery when the user comes online

#### Scenario: User searches for colleague in organization chart

**Given** the user is viewing the organization chart
**When** the user types a search query in the search box
**Then** the system should filter users by:
  - Name (exact or partial match)
  - Pinyin (for Chinese names)
  - Department name
**And** the filtered results should update in real-time as the user types

---

### MODIFIED Requirement: Messaging conversation initialization

The system MUST support starting conversations from multiple entry points including contacts and organization chart.

#### Scenario: User starts conversation from organization chart

**Given** the user is in the organization chart view
**And** the user clicks the "聊天" button on a user with IP "192.168.1.100"
**When** the chat initialization handler is triggered
**Then** the system should:
  - Add "192.168.1.100" to the manually added conversations set
  - Switch the active tab to "chat"
  - Set the active conversation ID to "192.168.1.100"
**And** the conversation should appear in the messaging sidebar
**And** the conversation should be selected and displayed in the main view

#### Scenario: Conversation persists after navigation

**Given** the user has started a conversation from the organization chart
**And** the user has sent a message to that conversation
**When** the user navigates away and back to the messaging tab
**Then** the conversation should still appear in the conversation list
**And** the conversation should contain the sent message

---

### ADDED Requirement: Department hierarchy display

The system MUST display a hierarchical department tree in the organization chart sidebar.

#### Scenario: User expands a department node

**Given** the organization chart is displayed
**And** the department tree contains a parent department with child departments
**When** the user clicks on a parent department node
**Then** the child departments should expand below the parent
**And** the expand/collapse icon should update to indicate the expanded state

#### Scenario: User filters by department selection

**Given** the organization chart is displayed
**When** the user clicks on a department in the tree
**Then** the user grid should filter to show only users in that department
**And** the department selection should be visually highlighted
**And** the header should display the department name and member count

---

## Related Specifications

- `contacts-list`: The organization chart follows a similar pattern to the contacts list for user display and interaction
- `messaging`: Uses the same conversation initialization pattern as the contact-to-messaging bridge
