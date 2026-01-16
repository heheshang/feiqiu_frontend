// Tauri commands for contact management
//
// This module provides IPC interface between frontend and backend for contact operations.
// All commands are exposed to the frontend via Tauri's invoke system.

use crate::state::AppState;
use crate::storage::contact_repo::{
    ContactFilters, CreateContact, CreateGroup, UpdateContact, UpdateGroup,
};
use crate::NeoLanError;
use serde::{Deserialize, Serialize};
use tauri::State;

/// Contact data transfer object (DTO) for frontend serialization
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ContactDto {
    pub id: i32,
    pub peer_id: Option<i32>,
    pub name: String,
    pub nickname: Option<String>,
    pub avatar: Option<String>,
    pub phone: Option<String>,
    pub email: Option<String>,
    pub department: Option<String>,
    pub position: Option<String>,
    pub notes: Option<String>,
    pub is_favorite: bool,
    pub pinyin: Option<String>,
    pub is_online: bool,
    pub ip_address: Option<String>,
    pub last_seen: Option<i64>,
    pub created_at: i64,
    pub updated_at: Option<i64>,
    pub groups: Vec<String>,
}

impl From<crate::storage::entities::contacts::Model> for ContactDto {
    fn from(model: crate::storage::entities::contacts::Model) -> Self {
        Self {
            id: model.id,
            peer_id: model.peer_id,
            name: model.name,
            nickname: model.nickname,
            avatar: model.avatar,
            phone: model.phone,
            email: model.email,
            department: model.department,
            position: model.position,
            notes: model.notes,
            is_favorite: model.is_favorite,
            pinyin: model.pinyin,
            is_online: model.is_online,
            ip_address: model.peer_ip,
            last_seen: model.last_seen.map(|dt| dt.and_utc().timestamp_millis()),
            created_at: model.created_at.and_utc().timestamp_millis(),
            updated_at: model.updated_at.map(|dt| dt.and_utc().timestamp_millis()),
            groups: Vec::new(), // Will be populated separately if needed
        }
    }
}

/// Contact group data transfer object
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ContactGroupDto {
    pub id: i32,
    pub name: String,
    pub color: Option<String>,
    pub icon: Option<String>,
    pub sort_order: i32,
    pub created_at: i64,
    pub member_count: i64,
}

impl From<(crate::storage::entities::contact_groups::Model, i64)> for ContactGroupDto {
    fn from((model, count): (crate::storage::entities::contact_groups::Model, i64)) -> Self {
        Self {
            id: model.id,
            name: model.name,
            color: model.color,
            icon: model.icon,
            sort_order: model.sort_order,
            created_at: model.created_at.and_utc().timestamp_millis(),
            member_count: count,
        }
    }
}

/// Contact filters for querying
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ContactFiltersDto {
    pub search: Option<String>,
    pub is_online: Option<bool>,
    pub is_favorite: Option<bool>,
    pub department: Option<String>,
    pub group_id: Option<i32>,
}

impl From<ContactFiltersDto> for ContactFilters {
    fn from(dto: ContactFiltersDto) -> Self {
        Self {
            search: dto.search,
            is_online: dto.is_online,
            is_favorite: dto.is_favorite,
            department: dto.department,
            group_id: dto.group_id,
        }
    }
}

/// Get all contacts with optional filters
#[tauri::command]
pub async fn get_contacts(
    state: State<'_, AppState>,
    filters: Option<ContactFiltersDto>,
) -> std::result::Result<Vec<ContactDto>, String> {
    let repo = state
        .get_contact_repo()
        .ok_or_else(|| NeoLanError::Storage("Contact repository not initialized".to_string()))
        .map_err(|e| e.to_string())?;

    let filters = filters.map(|f| f.into());

    let contacts = repo.find_all(filters).await.map_err(|e| e.to_string())?;

    Ok(contacts.into_iter().map(ContactDto::from).collect())
}

/// Get a single contact by ID
#[tauri::command]
pub async fn get_contact(
    state: State<'_, AppState>,
    id: i32,
) -> std::result::Result<Option<ContactDto>, String> {
    let repo = state
        .get_contact_repo()
        .ok_or_else(|| NeoLanError::Storage("Contact repository not initialized".to_string()))
        .map_err(|e| e.to_string())?;

    let contact = repo.find_by_id(id).await.map_err(|e| e.to_string())?;

    Ok(contact.map(ContactDto::from))
}

/// Data transfer object for creating contacts
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateContactDto {
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

/// Create a new contact
#[tauri::command]
pub async fn create_contact(
    state: State<'_, AppState>,
    contact: CreateContactDto,
) -> std::result::Result<ContactDto, String> {
    let repo = state
        .get_contact_repo()
        .ok_or_else(|| NeoLanError::Storage("Contact repository not initialized".to_string()))
        .map_err(|e| e.to_string())?;

    let new_contact = CreateContact {
        peer_id: contact.peer_id,
        name: contact.name,
        nickname: contact.nickname,
        avatar: contact.avatar,
        phone: contact.phone,
        email: contact.email,
        department: contact.department,
        position: contact.position,
        notes: contact.notes,
        pinyin: contact.pinyin,
        ip_address: contact.ip_address,
    };

    let result = repo.create(new_contact).await.map_err(|e| e.to_string())?;

    Ok(ContactDto::from(result))
}

/// Data transfer object for updating contacts
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateContactDto {
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

/// Update an existing contact
#[tauri::command]
pub async fn update_contact(
    state: State<'_, AppState>,
    id: i32,
    contact: UpdateContactDto,
) -> std::result::Result<ContactDto, String> {
    let repo = state
        .get_contact_repo()
        .ok_or_else(|| NeoLanError::Storage("Contact repository not initialized".to_string()))
        .map_err(|e| e.to_string())?;

    let update = UpdateContact {
        name: contact.name,
        nickname: contact.nickname,
        avatar: contact.avatar,
        phone: contact.phone,
        email: contact.email,
        department: contact.department,
        position: contact.position,
        notes: contact.notes,
        pinyin: contact.pinyin,
        is_favorite: contact.is_favorite,
        ip_address: contact.ip_address,
        peer_id: contact.peer_id,
    };

    let result = repo.update(id, update).await.map_err(|e| e.to_string())?;

    Ok(ContactDto::from(result))
}

/// Delete a contact
#[tauri::command]
pub async fn delete_contact(
    state: State<'_, AppState>,
    id: i32,
) -> std::result::Result<(), String> {
    let repo = state
        .get_contact_repo()
        .ok_or_else(|| NeoLanError::Storage("Contact repository not initialized".to_string()))
        .map_err(|e| e.to_string())?;

    repo.delete(id).await.map_err(|e| e.to_string())?;

    Ok(())
}

/// Get all contact groups
#[tauri::command]
pub async fn get_contact_groups(
    state: State<'_, AppState>,
) -> std::result::Result<Vec<ContactGroupDto>, String> {
    let repo = state
        .get_contact_repo()
        .ok_or_else(|| NeoLanError::Storage("Contact repository not initialized".to_string()))
        .map_err(|e| e.to_string())?;

    let groups = repo.find_all_groups().await.map_err(|e| e.to_string())?;

    // Get member counts for each group
    let mut result = Vec::new();
    for group in groups {
        let members = repo
            .find_contacts_by_group(group.id)
            .await
            .map_err(|e| e.to_string())?;
        result.push((group, members.len() as i64));
    }

    Ok(result.into_iter().map(ContactGroupDto::from).collect())
}

/// Data transfer object for creating groups
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateContactGroupDto {
    pub name: String,
    pub color: Option<String>,
    pub icon: Option<String>,
    pub sort_order: Option<i32>,
}

/// Create a new contact group
#[tauri::command]
pub async fn create_contact_group(
    state: State<'_, AppState>,
    group: CreateContactGroupDto,
) -> std::result::Result<ContactGroupDto, String> {
    let repo = state
        .get_contact_repo()
        .ok_or_else(|| NeoLanError::Storage("Contact repository not initialized".to_string()))
        .map_err(|e| e.to_string())?;

    let new_group = CreateGroup {
        name: group.name,
        color: group.color,
        icon: group.icon,
        sort_order: group.sort_order,
    };

    let result = repo
        .create_group(new_group)
        .await
        .map_err(|e| e.to_string())?;

    Ok(ContactGroupDto::from((result, 0)))
}

/// Data transfer object for updating groups
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateContactGroupDto {
    pub name: Option<String>,
    pub color: Option<String>,
    pub icon: Option<String>,
    pub sort_order: Option<i32>,
}

/// Update a contact group
#[tauri::command]
pub async fn update_contact_group(
    state: State<'_, AppState>,
    id: i32,
    group: UpdateContactGroupDto,
) -> std::result::Result<ContactGroupDto, String> {
    let repo = state
        .get_contact_repo()
        .ok_or_else(|| NeoLanError::Storage("Contact repository not initialized".to_string()))
        .map_err(|e| e.to_string())?;

    let update = UpdateGroup {
        name: group.name,
        color: group.color,
        icon: group.icon,
        sort_order: group.sort_order,
    };

    let result = repo
        .update_group(id, update)
        .await
        .map_err(|e| e.to_string())?;

    // Get member count
    let members = repo
        .find_contacts_by_group(result.id)
        .await
        .map_err(|e| e.to_string())?;

    Ok(ContactGroupDto::from((result, members.len() as i64)))
}

/// Delete a contact group
#[tauri::command]
pub async fn delete_contact_group(
    state: State<'_, AppState>,
    id: i32,
) -> std::result::Result<(), String> {
    let repo = state
        .get_contact_repo()
        .ok_or_else(|| NeoLanError::Storage("Contact repository not initialized".to_string()))
        .map_err(|e| e.to_string())?;

    repo.delete_group(id).await.map_err(|e| e.to_string())?;

    Ok(())
}

/// Add contacts to a group
#[tauri::command]
pub async fn add_contacts_to_group(
    state: State<'_, AppState>,
    group_id: i32,
    contact_ids: Vec<i32>,
) -> std::result::Result<(), String> {
    let repo = state
        .get_contact_repo()
        .ok_or_else(|| NeoLanError::Storage("Contact repository not initialized".to_string()))
        .map_err(|e| e.to_string())?;

    for contact_id in contact_ids {
        repo.add_to_group(contact_id, group_id)
            .await
            .map_err(|e| e.to_string())?;
    }

    Ok(())
}

/// Remove contacts from a group
#[tauri::command]
pub async fn remove_contacts_from_group(
    state: State<'_, AppState>,
    group_id: i32,
    contact_ids: Vec<i32>,
) -> std::result::Result<(), String> {
    let repo = state
        .get_contact_repo()
        .ok_or_else(|| NeoLanError::Storage("Contact repository not initialized".to_string()))
        .map_err(|e| e.to_string())?;

    for contact_id in contact_ids {
        repo.remove_from_group(contact_id, group_id)
            .await
            .map_err(|e| e.to_string())?;
    }

    Ok(())
}

/// Search contacts by query string
#[tauri::command]
pub async fn search_contacts(
    state: State<'_, AppState>,
    query: String,
) -> std::result::Result<Vec<ContactDto>, String> {
    let repo = state
        .get_contact_repo()
        .ok_or_else(|| NeoLanError::Storage("Contact repository not initialized".to_string()))
        .map_err(|e| e.to_string())?;

    let contacts = repo.search(&query).await.map_err(|e| e.to_string())?;

    Ok(contacts.into_iter().map(ContactDto::from).collect())
}

/// Contact statistics data transfer object
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ContactStatsDto {
    pub total: i64,
    pub online: i64,
    pub offline: i64,
    pub favorites: i64,
    pub by_department: std::collections::HashMap<String, i64>,
}

/// Get contact statistics
#[tauri::command]
pub async fn get_contact_stats(
    state: State<'_, AppState>,
) -> std::result::Result<ContactStatsDto, String> {
    let repo = state
        .get_contact_repo()
        .ok_or_else(|| NeoLanError::Storage("Contact repository not initialized".to_string()))
        .map_err(|e| e.to_string())?;

    let all = repo.find_all(None).await.map_err(|e| e.to_string())?;
    let online = repo
        .find_all(Some(ContactFilters {
            search: None,
            is_online: Some(true),
            is_favorite: None,
            department: None,
            group_id: None,
        }))
        .await
        .map_err(|e| e.to_string())?;
    let favorites = repo
        .find_all(Some(ContactFilters {
            search: None,
            is_online: None,
            is_favorite: Some(true),
            department: None,
            group_id: None,
        }))
        .await
        .map_err(|e| e.to_string())?;

    // Calculate department breakdown
    let mut by_department = std::collections::HashMap::new();
    for contact in &all {
        if let Some(dept) = &contact.department {
            *by_department.entry(dept.clone()).or_insert(0) += 1;
        }
    }

    Ok(ContactStatsDto {
        total: all.len() as i64,
        online: online.len() as i64,
        offline: (all.len() - online.len()) as i64,
        favorites: favorites.len() as i64,
        by_department,
    })
}
