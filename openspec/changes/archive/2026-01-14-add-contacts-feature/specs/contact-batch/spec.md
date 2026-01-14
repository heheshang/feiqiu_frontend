# Spec: Contact Batch Operations

**Capability:** `contact-batch`
**Change ID:** `add-contacts-feature`

## ADDED Requirements

### Requirement: Select multiple contacts for batch operations

The system MUST allow users to select multiple contacts using checkboxes or multi-select gestures.

#### Scenario: User selects contacts using checkboxes

**Given** the user is viewing the contacts list
**When** the user clicks checkboxes on 3 different contacts
**Then** all 3 contacts should be highlighted as selected
**And** a batch action toolbar should appear at the bottom of the screen
**And** the toolbar should show: "已选择 3 个联系人" (3 contacts selected)

#### Scenario: User uses Ctrl+Click to select contacts

**Given** the user is viewing the contacts list
**When** the user holds Ctrl and clicks on 5 different contacts
**Then** all 5 contacts should be highlighted as selected
**And** the batch action toolbar should appear

#### Scenario: User uses Shift+Click for range selection

**Given** the user is viewing the contacts list
**When** the user clicks the first contact in a list
**And** holds Shift and clicks the 10th contact
**Then** contacts 1-10 should all be selected
**And** the batch action toolbar should show 10 contacts selected

#### Scenario: User selects all contacts

**Given** the user is viewing the contacts list with 50 contacts
**When** the user clicks the "全选" (Select All) checkbox
**Then** all 50 contacts should be selected
**And** the checkbox should change to a checked state

#### Scenario: User deselects contacts

**Given** the user has selected 5 contacts
**When** the user clicks on one selected contact again
**Then** that contact should be deselected
**And** the selection count should update to "已选择 4 个联系人"

---

### Requirement: Batch delete multiple contacts

The system MUST allow users to delete multiple selected contacts in a single operation.

#### Scenario: User batch deletes selected contacts

**Given** the user has selected 5 contacts
**When** the user clicks the "删除" (Delete) button in the batch toolbar
**Then** the system should display a confirmation dialog: "确认删除选中的 5 个联系人？"
**And** if the user confirms:
  - All 5 contacts should be deleted from the database
  - The contacts list should refresh
  - A success message should be displayed: "已删除 5 个联系人"
**And** if any contacts are synced from active peers, a warning should be shown

#### Scenario: User cancels batch delete

**Given** the user has selected 5 contacts
**When** the user clicks the "删除" button
**And** clicks "取消" (Cancel) in the confirmation dialog
**Then** no contacts should be deleted
**And** all contacts should remain selected

---

### Requirement: Batch move contacts to a group

The system MUST allow users to add multiple selected contacts to a specific group.

#### Scenario: User batch moves contacts to a group

**Given** the user has selected 8 contacts
**And** a group "VIP" already exists
**When** the user clicks "添加到分组" (Add to Group) in the batch toolbar
**And** selects the "VIP" group from the dropdown
**Then** all 8 contacts should be added to the VIP group
**And** the VIP group member count should increase by 8
**And** a success message should be displayed: "已将 8 个联系人添加到 VIP 分组"
**And** the selection should be cleared

#### Scenario: User batch moves contacts to multiple groups

**Given** the user has selected 5 contacts
**When** the user clicks "添加到分组"
**And** selects multiple groups: "同事" and "重要联系人"
**Then** all 5 contacts should be added to both groups
**And** success message should confirm: "已将 5 个联系人添加到 2 个分组"

---

### Requirement: Batch remove contacts from groups

The system MUST allow users to remove multiple contacts from a specific group.

#### Scenario: User batch removes contacts from a group

**Given** the user is currently viewing the "VIP" group (which has 10 members)
**And** the user has selected 3 contacts within this group
**When** the user clicks "从分组移除" (Remove from Group)
**Then** the 3 contacts should be removed from the VIP group
**And** the VIP group member count should decrease to 7
**And** the contacts should remain in the main contact list
**And** a success message should be displayed

---

### Requirement: Batch export contacts

The system MUST allow users to export selected contacts to a file (CSV or JSON format).

#### Scenario: User exports selected contacts

**Given** the user has selected 10 contacts
**When** the user clicks the "导出" (Export) button in the batch toolbar
**And** selects "CSV" format from the export dialog
**Then** the system should generate a CSV file containing:
  - Name, nickname, phone, email, department, position, notes
  - One row per selected contact
**And** the file should be downloaded with name: "contacts_export_YYYYMMDD.csv"
**And** a success message should be displayed: "已导出 10 个联系人"

#### Scenario: User exports all contacts

**Given** the user is viewing all contacts (no selection)
**When** the user clicks "导出全部" (Export All)
**Then** all contacts in the current filter view should be exported
**And** if a filter is active, only filtered contacts should be exported

---

### Requirement: Batch mark contacts as favorites

The system MUST allow users to mark multiple selected contacts as favorites.

#### Scenario: User batch marks contacts as favorites

**Given** the user has selected 5 contacts
**When** the user clicks "标为收藏" (Mark as Favorite) in the batch toolbar
**Then** all 5 contacts should be marked as favorites
**And** all should display the filled star (★) icon
**And** the Favorites filter count should increase by 5
**And** a success message should be displayed: "已将 5 个联系人标为收藏"

#### Scenario: User batch unmarks contacts as favorites

**Given** the user has selected 5 contacts that are all favorites
**When** the user clicks "取消收藏" (Unmark as Favorite)
**Then** all 5 contacts should be unmarked as favorites
**And** the Favorites filter count should decrease by 5

---

### Requirement: Display batch operation progress

The system MUST show progress indicators for batch operations that may take time.

#### Scenario: Batch operation shows progress

**Given** the user has selected 100 contacts
**When** the user initiates a batch export operation
**Then** a progress bar should appear showing: "正在导出... 50/100"
**And** the progress should update in real-time
**And** when complete, the progress bar should disappear and show a success message
