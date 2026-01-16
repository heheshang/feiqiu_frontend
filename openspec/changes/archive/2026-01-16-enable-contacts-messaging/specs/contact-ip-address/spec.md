# contact-ip-address Specification

## Purpose

Adds IP address storage to the contact data model, enabling direct peer lookup and messaging resolution for contacts.

## ADDED Requirements

### Requirement: Store IP address in contact

The contact entity MUST support storing an optional IP address field for peer association.

#### Scenario: Contact is created with IP address

**Given** the user is creating a new contact
**And** the user enters a known IP address for the contact
**When** the contact is saved
**Then** the ipAddress field should be stored in the database
**And** the contact should be retrievable with the ipAddress field populated

#### Scenario: Contact is created without IP address

**Given** the user is creating a new contact
**And** the user does not enter an IP address
**When** the contact is saved
**Then** the ipAddress field should be null in the database
**And** the contact should be retrievable with ipAddress as undefined

#### Scenario: Contact IP address is updated

**Given** a contact exists with ipAddress set to "192.168.1.10"
**When** the user updates the contact's IP address to "192.168.1.20"
**Then** the database should store the new IP address
**And** subsequent contact queries should return the updated IP address

---

### Requirement: Auto-link peer to contact by name

When creating or updating a contact, the system MUST automatically link to an existing peer with a matching name.

#### Scenario: Auto-link on contact create

**Given** a peer exists on the network with name "王五" and IP "192.168.1.30"
**When** the user creates a contact with name "王五"
**Then** the system should search for a peer with matching name
**And** if found, set the contact's peerId to the peer's ID
**And** set the contact's ipAddress to the peer's IP address

#### Scenario: No matching peer on contact create

**Given** no peer exists with name "赵六"
**When** the user creates a contact with name "赵六"
**Then** the contact should be created without a peerId
**And** the ipAddress should be null
**And** the contact should still be functional for manual IP entry

#### Scenario: Multiple peers with same name

**Given** two peers exist with name "小明"
**And** peer A has IP "192.168.1.40" (last seen 1 minute ago)
**And** peer B has IP "192.168.1.41" (last seen 1 hour ago)
**When** the user creates a contact with name "小明"
**Then** the system should link to the most recently active peer (peer A)
**And** store peer A's peerId and ipAddress

---

### Requirement: Database migration for IP address field

The database schema MUST be migrated to add the ip_address column to the contacts table.

#### Scenario: Running migration on existing database

**Given** a database exists with a contacts table without ip_address column
**When** the migration is run
**Then** the ip_address column should be added as Option<String>
**And** an index should be created on ip_address for queries
**And** existing contacts should have null values for ip_address
**And** the migration should complete without errors

#### Scenario: Rolling back migration

**Given** the database has been migrated with ip_address column
**When** the migration is rolled back
**Then** the ip_address column should be removed
**And** the index on ip_address should be dropped
**And** existing data should remain intact

---

## MODIFIED Requirements

### Requirement: Contact DTO serialization

The ContactDto MUST include the ipAddress field for frontend-backend communication.

#### Scenario: Serializing contact with IP address

**Given** a contact exists with ipAddress "192.168.1.50"
**When** the contact is serialized to ContactDto
**Then** the DTO should include ipAddress as "192.168.1.50"
**And** the field should be named "ipAddress" (camelCase) for frontend

#### Scenario: Serializing contact without IP address

**Given** a contact exists with null ipAddress
**When** the contact is serialized to ContactDto
**Then** the DTO should include ipAddress as null
**And** the frontend should receive it as undefined

---

### Requirement: Contact creation input

The CreateContactInput MUST accept an optional ipAddress field.

#### Scenario: Creating contact with IP in frontend

**Given** the user is on the contact creation form
**And** the user enters IP address "192.168.1.60"
**When** the form is submitted
**Then** the CreateContactInput should include ipAddress: "192.168.1.60"
**And** the backend should receive and store this value

---

### Requirement: Contact update input

The UpdateContactInput MUST accept an optional ipAddress field for updates.

#### Scenario: Updating contact IP in frontend

**Given** a contact exists without an IP address
**And** the user opens the edit contact dialog
**And** the user enters IP address "192.168.1.70"
**When** the update is submitted
**Then** the UpdateContactInput should include ipAddress: "192.168.1.70"
**And** the backend should update the contact's IP address

---

## Related Capabilities

- **contacts-list-messaging**: Uses ipAddress for peer resolution
- **peer-discovery**: Provides peer data for auto-linking
- **contact-crud**: Base CRUD operations extended with IP field
