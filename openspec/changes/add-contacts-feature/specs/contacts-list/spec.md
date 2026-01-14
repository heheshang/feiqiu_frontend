# Spec: Contacts List Management

**Capability:** `contacts-list`
**Change ID:** `add-contacts-feature`

## ADDED Requirements

### Requirement: Display all contacts in a centralized list

The system MUST display a comprehensive list of all contacts including both online peers and offline historical contacts in the Contacts section of the application.

#### Scenario: User views the contacts list for the first time

**Given** the user has opened the FeiQiu application
**And** the user has navigated to the "通讯录" (Contacts) tab
**When** the contacts list loads
**Then** the user should see all contacts including:
  - Online peers (currently active on the LAN)
  - Offline contacts (historical peers from database)
  - Manually added contacts
**And** each contact should display:
  - Avatar (or default if none)
  - Name/nickname (nickname takes precedence)
  - Online status indicator (green/yellow/gray dot)
  - Department and position (if available)
  - Last seen timestamp (for offline contacts)

#### Scenario: Contact list shows real-time online status updates

**Given** the user is viewing the contacts list
**And** a peer comes online on the LAN
**When** the peer event is received
**Then** the contact's online status should update to "online" within 1 second
**And** the status indicator should change to green

---

### Requirement: Sort and filter contacts by multiple criteria

The system MUST allow users to sort and filter contacts by status, group, and other attributes.

#### Scenario: User filters to show only online contacts

**Given** the user is viewing the contacts list
**When** the user selects the "在线" (Online) filter
**Then** only contacts with online status should be displayed
**And** the filter label should show the count of online contacts (e.g., "在线 (23)")

#### Scenario: User filters to show only favorite contacts

**Given** the user has marked several contacts as favorites
**When** the user selects the "★ 收藏" (Favorites) filter
**Then** only favorite contacts should be displayed
**And** the favorite star (★) should be visible on each contact card

#### Scenario: User sorts contacts by last seen time

**Given** the user is viewing the contacts list
**When** the user selects "最近活跃" (Recently Active) sort option
**Then** contacts should be ordered by their `last_seen` timestamp in descending order
**And** contacts seen more recently should appear at the top

---

### Requirement: Display contact statistics summary

The system MUST display a summary of contact statistics including total count, online count, and department breakdown.

#### Scenario: User views contact statistics

**Given** the user is viewing the contacts list
**When** the contacts list loads
**Then** a statistics summary should be displayed showing:
  - Total number of contacts
  - Number of online contacts
  - Number of favorite contacts
  - Breakdown by department (if contacts have department info)

---

### Requirement: Support dual organization view modes

The system MUST support switching between "Groups" (custom groups) and "Department" (organization structure) view modes.

#### Scenario: User switches to Department view mode

**Given** the user is viewing the contacts list
**And** the current view mode is "Groups"
**When** the user selects the "Department" view mode
**Then** the sidebar should display department hierarchy
**And** contacts should be organized under their respective departments
**And** selecting a department should filter the list to show only contacts in that department

#### Scenario: User switches back to Groups view mode

**Given** the user is in Department view mode
**When** the user selects the "Groups" view mode
**Then** the sidebar should display custom user-created groups
**And** the previous group selection should be restored
