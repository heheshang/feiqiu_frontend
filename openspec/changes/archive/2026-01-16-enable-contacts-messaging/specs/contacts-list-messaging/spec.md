# contacts-list-messaging Specification

## Purpose

Extends the contacts list functionality to enable sending messages to contacts regardless of their peer link status. Provides fallback mechanisms to find and message peers through multiple resolution strategies.

## ADDED Requirements

### Requirement: Resolve contact to peer for messaging

The system MUST resolve a contact to an active peer using multiple strategies when the user initiates a conversation.

#### Scenario: User sends message to contact with peerId

**Given** the user has a contact with a linked peerId
**And** the peer is currently online
**When** the user clicks the "发消息" button
**Then** the system should start a conversation using the peerId
**And** switch to the chat tab
**And** select the conversation

#### Scenario: User sends message to contact with ipAddress but no peerId

**Given** the user has a contact with a stored ipAddress
**And** the contact has no linked peerId
**And** a peer with that IP address is currently online
**When** the user clicks the "发消息" button
**Then** the system should lookup the peer by IP address
**And** start a conversation with the found peer
**And** switch to the chat tab

#### Scenario: User sends message to contact with no peer link but matching peer by name

**Given** the user has a contact with no peerId or ipAddress
**And** an online peer exists with the same name as the contact
**When** the user clicks the "发消息" button
**Then** the system should find the peer by name
**And** update the contact with the peer's information (peerId, ipAddress)
**And** start a conversation with the peer
**And** switch to the chat tab

#### Scenario: User sends message to unavailable contact

**Given** the user has a contact with no peerId, ipAddress, or matching peer
**When** the user clicks the "发消息" button
**Then** the system should display a notification: "该联系人当前不在线，无法发送消息"
**And** NOT switch to the chat tab
**And** close the contact detail dialog

---

### Requirement: Display contact messaging availability

The system MUST provide visual feedback about whether a contact can be messaged based on their current state.

#### Scenario: Contact is online with peerId

**Given** a contact has a linked peerId
**And** the peer is online
**When** the contact detail dialog is displayed
**Then** the "发消息" button should be enabled
**And** the button should have no tooltip or a positive tooltip

#### Scenario: Contact is offline with stored IP address

**Given** a contact has a stored ipAddress
**And** the contact has no linked peerId
**And** no peer with that IP is currently online
**When** the contact detail dialog is displayed
**Then** the "发消息" button should be enabled
**And** hovering the button should show a tooltip: "该联系人当前离线，点击尝试发送"

#### Scenario: Contact has no peer link

**Given** a contact has no peerId or ipAddress
**When** the contact detail dialog is displayed
**Then** the "发消息" button should be enabled
**And** hovering the button should show a tooltip: "点击尝试查找并发送"

---

### Requirement: Store IP address with contacts

The system MUST store the IP address of peers when they are linked to contacts.

#### Scenario: Creating contact with existing peer

**Given** a peer exists on the network with IP "192.168.1.100" and name "张三"
**When** the user creates a contact with name "张三"
**Then** the system should link the contact to the peer
**And** store the peer's IP address in the contact's ipAddress field
**And** store the peer's ID in the contact's peerId field

#### Scenario: Updating contact when peer comes online

**Given** a contact exists with name "李四" but no peer link
**And** a peer comes online with name "李四" and IP "192.168.1.101"
**When** the user sends a message to this contact
**Then** the system should update the contact with peerId and ipAddress
**And** future message sends should use the stored peerId

---

### Requirement: Provide peer lookup by IP address

The system MUST provide a backend command to find a peer by their IP address.

#### Scenario: Looking up online peer by IP

**Given** a peer is online with IP address "192.168.1.50"
**When** the frontend calls `get_peer_by_ip("192.168.1.50")`
**Then** the backend should return the peer's PeerDto
**And** the PeerDto should contain the peer's name, IP, status, and other fields

#### Scenario: Looking up non-existent peer by IP

**Given** no peer exists with IP address "192.168.1.999"
**When** the frontend calls `get_peer_by_ip("192.168.1.999")`
**Then** the backend should return null
**And** the frontend should handle the null result gracefully

---

## Related Capabilities

- **contact-crud**: Depends on IP address storage for create/update operations
- **messaging**: Consumes peer resolution results to start conversations
- **peer-discovery**: Provides the peer list used for contact-to-peer matching
