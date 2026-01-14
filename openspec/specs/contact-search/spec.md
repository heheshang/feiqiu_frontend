# contact-search Specification

## Purpose
TBD - created by archiving change add-contacts-feature. Update Purpose after archive.
## Requirements
### Requirement: Full-text search across all contact fields

The system MUST provide a search box that performs full-text search across name, nickname, pinyin, department, position, phone, email, and notes fields.

#### Scenario: User searches by name

**Given** the user has 50+ contacts in the list
**When** the user types "张" in the search box
**Then** only contacts with "张" in their name, nickname, or pinyin should be displayed
**And** the search should work in real-time (debounced 300ms)
**And** matching characters should be highlighted in the results

#### Scenario: User searches by pinyin

**Given** a contact exists with name "张三" (pinyin: "zhangsan")
**When** the user types "zhang" or "zs" in the search box
**Then** the contact "张三" should appear in the search results
**And** contacts without matching pinyin should be filtered out

#### Scenario: User searches by phone number

**Given** a contact exists with phone "13812345678"
**When** the user types "138" in the search box
**Then** contacts with phone numbers containing "138" should be displayed
**And** the matching portion should be highlighted

#### Scenario: User searches by department

**Given** multiple contacts exist in the "技术部" (Technology Department)
**When** the user types "技术" in the search box
**Then** all contacts in the Technology Department should be displayed
**And** contacts from other departments should be hidden

#### Scenario: User clears search

**Given** the user has entered a search query
**When** the user clears the search box or clicks the X button
**Then** all contacts should be displayed again
**And** the previous filter state should be restored

---

### Requirement: Filter contacts by status

The system MUST provide quick filters to show only online, offline, or favorite contacts.

#### Scenario: User applies online status filter

**Given** the user is viewing all contacts
**When** the user clicks the "在线" (Online) filter button
**Then** only contacts with online status should be displayed
**And** the filter button should be highlighted as active
**And** the result count should be shown (e.g., "23 个在线联系人")

#### Scenario: User combines status filter with search

**Given** the user has applied the "在线" filter
**When** the user then searches for "张"
**Then** only online contacts with "张" in their name should be displayed
**And** both filters should be active

#### Scenario: User applies favorites filter

**Given** the user has marked 8 contacts as favorites
**When** the user clicks the "★ 收藏" (Favorites) filter
**Then** only favorite contacts should be displayed
**And** the results should be sorted by most recently added to favorites

---

### Requirement: Filter contacts by group

The system MUST allow filtering contacts by selecting a specific group from the sidebar.

#### Scenario: User selects a group to filter

**Given** the user has groups "同事" (15 members) and "客户" (8 members)
**When** the user clicks the "客户" (Clients) group in the sidebar
**Then** only the 8 contacts in the Clients group should be displayed
**And** the group should be highlighted as active
**And** a breadcrumb should show "分组 > 客户"

---

### Requirement: Filter contacts by department

The system MUST allow filtering contacts by department when in Department view mode.

#### Scenario: User filters by department in Department view

**Given** the user is in Department view mode
**When** the user selects a department node in the department tree
**Then** only contacts belonging to that department should be displayed
**And** sub-departments should be included in the results
**And** the department path should be shown in the breadcrumb

---

### Requirement: Display search result count

The system MUST display the number of contacts matching the current search or filter criteria.

#### Scenario: Search shows result count

**Given** the user types a search query
**When** the search results are displayed
**Then** a message should show: "找到 X 个联系人" (Found X contacts)
**And** if no results are found: "未找到匹配的联系人" (No matching contacts found)

---

### Requirement: Save recent searches

The system MUST save and display recent search queries for quick access.

#### Scenario: User views recent searches

**Given** the user has previously searched for "李四" and "技术部"
**When** the user clicks in the search box
**Then** a dropdown should show recent searches:
  - "李四"
  - "技术部"
**And** clicking a recent search should reapply that filter

#### Scenario: User clears search history

**Given** the user has accumulated search history
**When** the user clicks "清除搜索历史" (Clear Search History)
**Then** all recent searches should be cleared
**And** the dropdown should be empty

