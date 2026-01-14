# Spec: Contact CRUD Operations

**Capability:** `contact-crud`
**Change ID:** `add-contacts-feature`

## ADDED Requirements

### Requirement: Create new manual contact entries

The system MUST allow users to manually add contact records for users not currently on the LAN.

#### Scenario: User manually adds a new contact

**Given** the user is viewing the contacts list
**When** the user clicks the "+ 添加联系人" (Add Contact) button
**And** fills in the contact information:
  - Name: "王五"
  - Phone: "13912345678"
  - Email: "wangwu@example.com"
  - Department: "市场部"
  - Position: "市场经理"
  - Notes: "负责华东地区业务"
**And** clicks "保存" (Save)
**Then** a new contact should be created with the provided information
**And** the contact should appear in the contacts list
**And** the contact should be marked as offline (no peer association)
**And** a success notification should be displayed

#### Scenario: User creates a contact with duplicate name

**Given** a contact named "张三" already exists
**When** the user attempts to create another contact named "张三"
**Then** the system should display a warning: "联系人'张三'已存在，是否继续？"
**And** the user can choose to continue or cancel

---

### Requirement: Edit existing contact details

The system MUST allow users to edit all editable fields of a contact including nickname, notes, phone, and email.

#### Scenario: User edits a contact's nickname and notes

**Given** a contact exists with name "李四"
**When** the user clicks on the contact to open details
**And** changes the nickname to "老李"
**And** adds notes: "技术专家，擅长后端开发"
**And** clicks "保存" (Save)
**Then** the contact's nickname should update to "老李"
**And** the notes should be saved
**And** the contact should display "老李" instead of "李四" in the list
**And** a success notification should be displayed

#### Scenario: User edits a synced contact's information

**Given** a contact was auto-synced from a LAN peer with username "alice-pc"
**When** the user edits the name to "Alice Zhang"
**And** saves the changes
**Then** the user-edited name "Alice Zhang" should take precedence over the peer's username
**And** future peer sync events should NOT overwrite the user's edit
**And** the contact should maintain the user's custom name

---

### Requirement: Delete contacts with confirmation

The system MUST allow users to delete contacts with appropriate confirmation and warnings.

#### Scenario: User deletes a manually added contact

**Given** the user has a manually added contact "临时联系人"
**When** the user clicks delete on the contact
**And** confirms the deletion
**Then** the contact should be removed from the database
**And** the contact should disappear from the contacts list immediately
**And** a success notification should be displayed

#### Scenario: User attempts to delete a synced contact

**Given** a contact is synced from an active LAN peer
**When** the user clicks delete on the contact
**Then** the system should display a warning: "此联系人来自局域网发现，删除后当该用户再次上线时会自动重新添加。确认删除？"
**And** if confirmed, the contact should be removed
**And** if the peer comes online again, a new contact should be auto-created

---

### Requirement: Mark contacts as favorites

The system MUST allow users to mark contacts as favorites for quick access.

#### Scenario: User marks a contact as favorite

**Given** a contact "重要客户" exists in the list
**When** the user clicks the star icon on the contact card
**Then** the contact should be marked as favorite
**And** a filled star (★) icon should be displayed
**And** the contact should appear in the "Favorites" filter
**And** favorite contacts should be sorted to the top when "Favorites" is selected

#### Scenario: User unmarks a favorite contact

**Given** a contact is marked as favorite
**When** the user clicks the star icon again
**Then** the favorite status should be removed
**And** an outline star (☆) icon should be displayed
**And** the contact should be removed from the Favorites filter

---

### Requirement: View detailed contact information

The system MUST display a detailed view of a contact with all available information.

#### Scenario: User views contact details

**Given** the user clicks on a contact in the list
**When** the contact details modal opens
**Then** the following information should be displayed:
  - Avatar (large, editable)
  - Name / Nickname
  - Online status with last seen time
  - Department and Position
  - Phone number (with click-to-call action if supported)
  - Email address (with mailto: link)
  - Groups the contact belongs to
  - Notes
  - Created date
**And** edit and delete action buttons should be available

---

### Requirement: Merge duplicate contact entries

The system MUST allow users to merge duplicate contact entries when multiple records exist for the same person.

#### Scenario: User merges duplicate contacts

**Given** two contacts exist:
  - Contact A: "张三" (from peer discovery, no phone)
  - Contact B: "张三" (manually added, has phone "13800138000")
**When** the user selects both contacts
**And** clicks "合并联系人" (Merge Contacts)
**Then** the system should display a merge preview showing which fields will be kept
**And** upon confirmation, the contacts should be merged into one record
**And** the merged contact should contain:
  - Name: "张三"
  - Phone: "13800138000" (from Contact B)
  - Peer association: from Contact A
**And** one of the duplicate records should be deleted

#### Scenario: System detects potential duplicates

**Given** multiple contacts exist with similar names or phone numbers
**When** the user views the contacts list
**Then** the system should display a suggestion: "发现2个可能重复的联系人"
**And** the user can review and merge the suggested duplicates
