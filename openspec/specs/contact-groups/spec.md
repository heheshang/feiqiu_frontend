# contact-groups Specification

## Purpose
TBD - created by archiving change add-contacts-feature. Update Purpose after archive.
## Requirements
### Requirement: Create custom contact groups

The system MUST allow users to create custom groups to organize their contacts (e.g., "同事", "VIP", "朋友").

#### Scenario: User creates a new custom group

**Given** the user is viewing the contacts list
**When** the user clicks the "+ 新建" (New Group) button in the groups sidebar
**And** enters a group name "重要客户" (Important Clients)
**And** optionally selects a color and icon for the group
**And** clicks "创建" (Create)
**Then** a new group should be created with the specified name
**And** the group should appear in the groups sidebar
**And** the group should have a member count of 0

---

### Requirement: Add and remove contacts from groups

The system MUST allow users to add contacts to groups and remove them.

#### Scenario: User adds contacts to a group

**Given** the user has created a group called "VIP"
**And** the user has selected 3 contacts from the contact list
**When** the user clicks "添加到分组" (Add to Group)
**And** selects the "VIP" group
**Then** the 3 contacts should be added to the VIP group
**And** the VIP group member count should update to show 3 members
**And** the contacts should display a group badge indicating they are in the VIP group

#### Scenario: User removes a contact from a group

**Given** a contact is a member of the "同事" (Colleagues) group
**When** the user views the contact details
**And** clicks "移除" (Remove) next to the "同事" group label
**Then** the contact should be removed from the Colleagues group
**And** the Colleagues group member count should decrement by 1

---

### Requirement: Edit and delete custom groups

The system MUST allow users to edit group properties (name, color, icon) and delete groups.

#### Scenario: User edits a group name and color

**Given** the user has a group called "Team A"
**When** the user right-clicks the group in the sidebar
**And** selects "编辑" (Edit)
**And** changes the name to "开发团队" (Dev Team)
**And** selects a blue color for the group
**And** clicks "保存" (Save)
**Then** the group name should update to "开发团队"
**And** the group icon/color should display in blue
**And** all contacts in this group should reflect the updated group name

#### Scenario: User deletes a group

**Given** the user has a group called "临时分组" (Temporary)
**And** the group contains 5 contacts
**When** the user right-clicks the group
**And** selects "删除" (Delete)
**And** confirms the deletion
**Then** the group should be removed from the sidebar
**And** the 5 contacts should remain in the main contact list
**And** the contacts should no longer display the deleted group label

---

### Requirement: Display group member count

The system MUST display the number of contacts in each group in the sidebar.

#### Scenario: Group member count updates when contacts are added

**Given** a group exists with 3 members
**When** the user adds 2 more contacts to the group
**Then** the group label should update to show "5" members
**And** the count should reflect in real-time

---

### Requirement: Support group-based navigation

The system MUST allow users to filter the contact list by selecting a specific group.

#### Scenario: User selects a group to filter contacts

**Given** the user has groups "同事" (8 members) and "朋友" (5 members)
**When** the user clicks on the "朋友" group in the sidebar
**Then** only the 5 contacts in the Friends group should be displayed
**And** the group should be highlighted as active
**And** the title should show "朋友 (5)"

#### Scenario: User clicks "全部" (All) to reset filter

**Given** the user has filtered contacts by the "同事" group
**When** the user clicks "全部" (All) in the groups sidebar
**Then** all contacts should be displayed (regardless of group membership)
**And** no group should be highlighted as active

