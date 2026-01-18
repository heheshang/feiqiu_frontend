// src-tauri/src/storage/contact_repo.rs
use crate::error::NeoLanError;
use crate::storage::entities::{contact_group_members, contact_groups, contacts, peers};
use sea_orm::{
    ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter, QueryOrder, Set,
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateContact {
    pub peer_id: Option<i32>,
    pub name: String,
    pub nickname: Option<String>,
    pub avatar: Option<String>,
    pub phone: Option<String>,
    pub email: Option<String>,
    pub department: Option<String>,
    pub position: Option<String>,
    pub notes: Option<String>,
    pub pinyin: Option<String>,
    pub ip_address: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateContact {
    pub name: Option<String>,
    pub nickname: Option<String>,
    pub avatar: Option<String>,
    pub phone: Option<String>,
    pub email: Option<String>,
    pub department: Option<String>,
    pub position: Option<String>,
    pub notes: Option<String>,
    pub pinyin: Option<String>,
    pub is_favorite: Option<bool>,
    pub ip_address: Option<String>,
    pub peer_id: Option<i32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContactFilters {
    pub search: Option<String>,
    pub is_online: Option<bool>,
    pub is_favorite: Option<bool>,
    pub department: Option<String>,
    pub group_id: Option<i32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateGroup {
    pub name: String,
    pub color: Option<String>,
    pub icon: Option<String>,
    pub sort_order: Option<i32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateGroup {
    pub name: Option<String>,
    pub color: Option<String>,
    pub icon: Option<String>,
    pub sort_order: Option<i32>,
}

#[derive(Clone)]
pub struct ContactRepository {
    db: DatabaseConnection,
}

impl ContactRepository {
    pub fn new(db: DatabaseConnection) -> Self {
        Self { db }
    }

    // ========== Contact CRUD ==========

    pub async fn find_all(
        &self,
        filters: Option<ContactFilters>,
    ) -> Result<Vec<contacts::Model>, NeoLanError> {
        let mut query = contacts::Entity::find();

        let filters_ref = filters.as_ref();

        if let Some(filters) = filters_ref {
            if let Some(is_online) = filters.is_online {
                query = query.filter(contacts::Column::IsOnline.eq(is_online));
            }
            if let Some(is_favorite) = filters.is_favorite {
                query = query.filter(contacts::Column::IsFavorite.eq(is_favorite));
            }
            if let Some(department) = &filters.department {
                query = query.filter(contacts::Column::Department.eq(department));
            }
        }

        let result = query
            .order_by_desc(contacts::Column::CreatedAt)
            .all(&self.db)
            .await
            .map_err(|e| NeoLanError::Storage(format!("Failed to fetch contacts: {}", e)))?;

        // Post-filter by group_id if needed
        if let Some(filters) = filters_ref {
            if let Some(group_id) = filters.group_id {
                let contact_ids: Vec<i32> = contact_group_members::Entity::find()
                    .filter(contact_group_members::Column::GroupId.eq(group_id))
                    .all(&self.db)
                    .await
                    .map_err(|e| {
                        NeoLanError::Storage(format!("Failed to fetch group members: {}", e))
                    })?
                    .into_iter()
                    .map(|m| m.contact_id)
                    .collect();

                return Ok(result
                    .into_iter()
                    .filter(|c| contact_ids.contains(&c.id))
                    .collect());
            }
        }

        Ok(result)
    }

    pub async fn find_by_id(&self, id: i32) -> Result<Option<contacts::Model>, NeoLanError> {
        let result = contacts::Entity::find_by_id(id)
            .one(&self.db)
            .await
            .map_err(|e| NeoLanError::Storage(format!("Failed to find contact: {}", e)))?;

        Ok(result)
    }

    pub async fn create(&self, contact: CreateContact) -> Result<contacts::Model, NeoLanError> {
        let now = chrono::Utc::now().naive_utc();
        let new_contact = contacts::ActiveModel {
            peer_id: Set(contact.peer_id),
            name: Set(contact.name),
            nickname: Set(contact.nickname),
            avatar: Set(contact.avatar),
            phone: Set(contact.phone),
            email: Set(contact.email),
            department: Set(contact.department),
            position: Set(contact.position),
            notes: Set(contact.notes),
            is_favorite: Set(false),
            pinyin: Set(contact.pinyin),
            is_online: Set(true),
            peer_ip: Set(contact.ip_address),
            last_seen: Set(Some(now)),
            created_at: Set(now),
            ..Default::default()
        };

        let result = new_contact
            .insert(&self.db)
            .await
            .map_err(|e| NeoLanError::Storage(format!("Failed to create contact: {}", e)))?;

        Ok(result)
    }

    pub async fn update(
        &self,
        id: i32,
        contact: UpdateContact,
    ) -> Result<contacts::Model, NeoLanError> {
        let existing = self
            .find_by_id(id)
            .await?
            .ok_or_else(|| NeoLanError::Storage("Contact not found".to_string()))?;

        let mut active: contacts::ActiveModel = existing.into();

        if let Some(name) = contact.name {
            active.name = Set(name);
        }
        if let Some(nickname) = contact.nickname {
            active.nickname = Set(Some(nickname));
        }
        if let Some(avatar) = contact.avatar {
            active.avatar = Set(Some(avatar));
        }
        if let Some(phone) = contact.phone {
            active.phone = Set(Some(phone));
        }
        if let Some(email) = contact.email {
            active.email = Set(Some(email));
        }
        if let Some(department) = contact.department {
            active.department = Set(Some(department));
        }
        if let Some(position) = contact.position {
            active.position = Set(Some(position));
        }
        if let Some(notes) = contact.notes {
            active.notes = Set(Some(notes));
        }
        if let Some(pinyin) = contact.pinyin {
            active.pinyin = Set(Some(pinyin));
        }
        if let Some(is_favorite) = contact.is_favorite {
            active.is_favorite = Set(is_favorite);
        }
        if let Some(ip_address) = contact.ip_address {
            active.peer_ip = Set(Some(ip_address));
        }
        if let Some(peer_id) = contact.peer_id {
            active.peer_id = Set(Some(peer_id));
        }

        let now = chrono::Utc::now().naive_utc();
        active.updated_at = Set(Some(now));

        let result = active
            .update(&self.db)
            .await
            .map_err(|e| NeoLanError::Storage(format!("Failed to update contact: {}", e)))?;

        Ok(result)
    }

    pub async fn delete(&self, id: i32) -> Result<(), NeoLanError> {
        let existing = self
            .find_by_id(id)
            .await?
            .ok_or_else(|| NeoLanError::Storage("Contact not found".to_string()))?;

        let active: contacts::ActiveModel = existing.into();
        active
            .delete(&self.db)
            .await
            .map_err(|e| NeoLanError::Storage(format!("Failed to delete contact: {}", e)))?;

        Ok(())
    }

    // ========== Group CRUD ==========

    pub async fn find_all_groups(&self) -> Result<Vec<contact_groups::Model>, NeoLanError> {
        let result = contact_groups::Entity::find()
            .order_by_asc(contact_groups::Column::SortOrder)
            .all(&self.db)
            .await
            .map_err(|e| NeoLanError::Storage(format!("Failed to fetch groups: {}", e)))?;

        Ok(result)
    }

    pub async fn find_contacts_by_group(
        &self,
        group_id: i32,
    ) -> Result<Vec<contacts::Model>, NeoLanError> {
        let contact_ids: Vec<i32> = contact_group_members::Entity::find()
            .filter(contact_group_members::Column::GroupId.eq(group_id))
            .all(&self.db)
            .await
            .map_err(|e| NeoLanError::Storage(format!("Failed to fetch group members: {}", e)))?
            .into_iter()
            .map(|m| m.contact_id)
            .collect();

        if contact_ids.is_empty() {
            return Ok(vec![]);
        }

        let result = contacts::Entity::find()
            .filter(contacts::Column::Id.is_in(contact_ids))
            .all(&self.db)
            .await
            .map_err(|e| NeoLanError::Storage(format!("Failed to fetch group contacts: {}", e)))?;

        Ok(result)
    }

    pub async fn create_group(
        &self,
        group: CreateGroup,
    ) -> Result<contact_groups::Model, NeoLanError> {
        let now = chrono::Utc::now().naive_utc();
        let new_group = contact_groups::ActiveModel {
            name: Set(group.name),
            color: Set(group.color),
            icon: Set(group.icon),
            sort_order: Set(group.sort_order.unwrap_or(0)),
            created_at: Set(now),
            ..Default::default()
        };

        let result = new_group
            .insert(&self.db)
            .await
            .map_err(|e| NeoLanError::Storage(format!("Failed to create group: {}", e)))?;

        Ok(result)
    }

    pub async fn update_group(
        &self,
        id: i32,
        group: UpdateGroup,
    ) -> Result<contact_groups::Model, NeoLanError> {
        let existing = contact_groups::Entity::find_by_id(id)
            .one(&self.db)
            .await
            .map_err(|e| NeoLanError::Storage(format!("Failed to find group: {}", e)))?
            .ok_or_else(|| NeoLanError::Storage("Group not found".to_string()))?;

        let mut active: contact_groups::ActiveModel = existing.into();

        if let Some(name) = group.name {
            active.name = Set(name);
        }
        if let Some(color) = group.color {
            active.color = Set(Some(color));
        }
        if let Some(icon) = group.icon {
            active.icon = Set(Some(icon));
        }
        if let Some(sort_order) = group.sort_order {
            active.sort_order = Set(sort_order);
        }

        let result = active
            .update(&self.db)
            .await
            .map_err(|e| NeoLanError::Storage(format!("Failed to update group: {}", e)))?;

        Ok(result)
    }

    pub async fn delete_group(&self, id: i32) -> Result<(), NeoLanError> {
        let existing = contact_groups::Entity::find_by_id(id)
            .one(&self.db)
            .await
            .map_err(|e| NeoLanError::Storage(format!("Failed to find group: {}", e)))?
            .ok_or_else(|| NeoLanError::Storage("Group not found".to_string()))?;

        let active: contact_groups::ActiveModel = existing.into();
        active
            .delete(&self.db)
            .await
            .map_err(|e| NeoLanError::Storage(format!("Failed to delete group: {}", e)))?;

        Ok(())
    }

    // ========== Group Membership ==========

    pub async fn add_to_group(&self, contact_id: i32, group_id: i32) -> Result<(), NeoLanError> {
        // Check if contact exists
        let _contact = self
            .find_by_id(contact_id)
            .await?
            .ok_or_else(|| NeoLanError::Storage("Contact not found".to_string()))?;

        // Check if group exists
        let _group = contact_groups::Entity::find_by_id(group_id)
            .one(&self.db)
            .await
            .map_err(|e| NeoLanError::Storage(format!("Failed to find group: {}", e)))?
            .ok_or_else(|| NeoLanError::Storage("Group not found".to_string()))?;

        // Check if already a member
        let existing = contact_group_members::Entity::find()
            .filter(contact_group_members::Column::ContactId.eq(contact_id))
            .filter(contact_group_members::Column::GroupId.eq(group_id))
            .one(&self.db)
            .await
            .map_err(|e| NeoLanError::Storage(format!("Failed to check membership: {}", e)))?;

        if existing.is_some() {
            return Ok(()); // Already a member
        }

        let now = chrono::Utc::now().naive_utc();
        let new_member = contact_group_members::ActiveModel {
            contact_id: Set(contact_id),
            group_id: Set(group_id),
            joined_at: Set(now),
            ..Default::default()
        };

        new_member
            .insert(&self.db)
            .await
            .map_err(|e| NeoLanError::Storage(format!("Failed to add to group: {}", e)))?;

        Ok(())
    }

    pub async fn remove_from_group(
        &self,
        contact_id: i32,
        group_id: i32,
    ) -> Result<(), NeoLanError> {
        let existing = contact_group_members::Entity::find()
            .filter(contact_group_members::Column::ContactId.eq(contact_id))
            .filter(contact_group_members::Column::GroupId.eq(group_id))
            .one(&self.db)
            .await
            .map_err(|e| NeoLanError::Storage(format!("Failed to find membership: {}", e)))?
            .ok_or_else(|| NeoLanError::Storage("Group membership not found".to_string()))?;

        let active: contact_group_members::ActiveModel = existing.into();
        active
            .delete(&self.db)
            .await
            .map_err(|e| NeoLanError::Storage(format!("Failed to remove from group: {}", e)))?;

        Ok(())
    }

    // ========== Search ==========

    pub async fn search(&self, query: &str) -> Result<Vec<contacts::Model>, NeoLanError> {
        let pattern = format!("%{}%", query);

        let result = contacts::Entity::find()
            .filter(
                sea_orm::Condition::any()
                    .add(contacts::Column::Name.like(&pattern))
                    .add(contacts::Column::Nickname.like(&pattern))
                    .add(contacts::Column::Pinyin.like(&pattern))
                    .add(contacts::Column::Department.like(&pattern))
                    .add(contacts::Column::Position.like(&pattern))
                    .add(contacts::Column::Phone.like(&pattern))
                    .add(contacts::Column::Email.like(&pattern))
                    .add(contacts::Column::Notes.like(&pattern)),
            )
            .order_by_desc(contacts::Column::CreatedAt)
            .all(&self.db)
            .await
            .map_err(|e| NeoLanError::Storage(format!("Failed to search contacts: {}", e)))?;

        Ok(result)
    }

    // ========== Sync from Peers ==========

    pub async fn sync_from_peers(&self, peer_models: Vec<peers::Model>) -> Result<(), NeoLanError> {
        for peer in peer_models {
            // Check if contact already exists for this peer by peer_ip
            let existing = contacts::Entity::find()
                .filter(contacts::Column::PeerIp.eq(&peer.ip))
                .one(&self.db)
                .await
                .map_err(|e| NeoLanError::Storage(format!("Failed to check contact: {}", e)))?;

            if let Some(contact) = existing {
                // Update online status and last_seen
                let mut active: contacts::ActiveModel = contact.into();
                let now = chrono::Utc::now().naive_utc();
                active.is_online = Set(true);
                active.last_seen = Set(Some(now));

                active.update(&self.db).await.map_err(|e| {
                    NeoLanError::Storage(format!("Failed to update contact: {}", e))
                })?;
            } else {
                // Create new contact from peer
                let now = chrono::Utc::now().naive_utc();
                let new_contact = contacts::ActiveModel {
                    peer_id: Set(Some(peer.id)),
                    peer_ip: Set(Some(peer.ip.clone())),
                    name: Set(peer
                        .username
                        .unwrap_or_else(|| peer.hostname.clone().unwrap_or_default())),
                    nickname: Set(peer.nickname),
                    avatar: Set(peer.avatar),
                    phone: Set(None),
                    email: Set(None),
                    department: Set(None),
                    position: Set(None),
                    notes: Set(None),
                    is_favorite: Set(false),
                    pinyin: Set(None),
                    is_online: Set(true),
                    last_seen: Set(Some(peer.last_seen)),
                    created_at: Set(now),
                    ..Default::default()
                };

                new_contact.insert(&self.db).await.map_err(|e| {
                    NeoLanError::Storage(format!("Failed to create contact from peer: {}", e))
                })?;
            }
        }

        Ok(())
    }
}
