// Tauri commands for configuration management
//
// This module provides IPC interface between frontend and backend for configuration operations.
// All commands are exposed to the frontend via Tauri's invoke system.

use crate::config::AppConfig;
use crate::state::AppState;
use crate::{NeoLanError, Result};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Configuration data transfer object (DTO) for frontend serialization
///
/// This struct is designed to be JSON-serializable and contains all configuration
/// that the frontend needs to read and modify.
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ConfigDto {
    /// User unique ID (user_id)
    pub user_id: String,

    /// User profile
    pub username: String,
    pub hostname: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub avatar: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,

    /// Network settings
    pub bind_ip: String,
    pub udp_port: u16,
    pub tcp_port_start: u16,
    pub tcp_port_end: u16,

    /// Peer discovery settings
    pub heartbeat_interval: u64,
    pub peer_timeout: u64,

    /// Security settings
    pub encryption_enabled: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encryption_key: Option<String>,

    /// Message settings
    pub offline_message_retention_days: u32,

    /// File transfer settings
    pub auto_accept_files: bool,
    pub file_save_dir: String,

    /// Application settings
    pub log_level: String,
}

impl ConfigDto {
    /// Create a new ConfigDto with default values (kept for test purposes and future use)
    #[allow(dead_code)]
    pub fn new() -> Self {
        Self::default()
    }

    /// Convert from AppConfig
    pub fn from_app_config(config: &AppConfig) -> Self {
        Self {
            user_id: config.user_id.clone(),
            username: config.username.clone(),
            hostname: config.hostname.clone(),
            avatar: None,
            status: None,
            bind_ip: config.bind_ip.clone(),
            udp_port: config.udp_port,
            tcp_port_start: config.tcp_port_start,
            tcp_port_end: config.tcp_port_end,
            heartbeat_interval: config.heartbeat_interval,
            peer_timeout: config.peer_timeout,
            encryption_enabled: config.encryption_enabled,
            encryption_key: config.encryption_key.clone(),
            offline_message_retention_days: config.offline_message_retention_days,
            auto_accept_files: config.auto_accept_files,
            file_save_dir: config.file_save_dir.clone(),
            log_level: config.log_level.clone(),
        }
    }

    /// Convert to AppConfig
    pub fn to_app_config(&self) -> AppConfig {
        AppConfig {
            user_id: self.user_id.clone(),
            username: self.username.clone(),
            hostname: self.hostname.clone(),
            bind_ip: self.bind_ip.clone(),
            udp_port: self.udp_port,
            tcp_port_start: self.tcp_port_start,
            tcp_port_end: self.tcp_port_end,
            heartbeat_interval: self.heartbeat_interval,
            peer_timeout: self.peer_timeout,
            encryption_enabled: self.encryption_enabled,
            encryption_key: self.encryption_key.clone(),
            offline_message_retention_days: self.offline_message_retention_days,
            auto_accept_files: self.auto_accept_files,
            file_save_dir: self.file_save_dir.clone(),
            log_level: self.log_level.clone(),
        }
    }

    /// Create from HashMap (for flexible configuration) (kept for test purposes)
    #[allow(dead_code)]
    pub fn from_map(map: &HashMap<String, String>) -> Self {
        Self {
            user_id: map
                .get("user_id")
                .cloned()
                .unwrap_or_else(|| uuid::Uuid::new_v4().to_string()),
            username: map
                .get("username")
                .cloned()
                .unwrap_or_else(whoami::username),
            hostname: map.get("hostname").cloned().unwrap_or_else(|| {
                whoami::fallible::hostname().unwrap_or_else(|_| "localhost".to_string())
            }),
            avatar: map.get("avatar").cloned(),
            status: map.get("status").cloned(),
            bind_ip: map
                .get("bind_ip")
                .cloned()
                .unwrap_or_else(|| "0.0.0.0".to_string()),
            udp_port: map
                .get("udp_port")
                .and_then(|s| s.parse().ok())
                .unwrap_or(2425),
            tcp_port_start: map
                .get("tcp_port_start")
                .and_then(|s| s.parse().ok())
                .unwrap_or(8000),
            tcp_port_end: map
                .get("tcp_port_end")
                .and_then(|s| s.parse().ok())
                .unwrap_or(9000),
            heartbeat_interval: map
                .get("heartbeat_interval")
                .and_then(|s| s.parse().ok())
                .unwrap_or(60),
            peer_timeout: map
                .get("peer_timeout")
                .and_then(|s| s.parse().ok())
                .unwrap_or(180),
            encryption_enabled: map
                .get("encryption_enabled")
                .and_then(|s| s.parse().ok())
                .unwrap_or(false),
            encryption_key: map.get("encryption_key").cloned(),
            offline_message_retention_days: map
                .get("offline_message_retention_days")
                .and_then(|s| s.parse().ok())
                .unwrap_or(30),
            auto_accept_files: map
                .get("auto_accept_files")
                .and_then(|s| s.parse().ok())
                .unwrap_or(false),
            file_save_dir: map.get("file_save_dir").cloned().unwrap_or_else(|| {
                dirs::download_dir()
                    .unwrap_or_else(|| std::path::PathBuf::from("."))
                    .to_string_lossy()
                    .to_string()
            }),
            log_level: map
                .get("log_level")
                .cloned()
                .unwrap_or_else(|| "info".to_string()),
        }
    }

    /// Convert to HashMap
    pub fn to_map(&self) -> HashMap<String, String> {
        let mut map = HashMap::new();
        map.insert("username".to_string(), self.username.clone());
        map.insert("hostname".to_string(), self.hostname.clone());
        map.insert("bind_ip".to_string(), self.bind_ip.clone());
        map.insert("udp_port".to_string(), self.udp_port.to_string());
        map.insert(
            "tcp_port_start".to_string(),
            self.tcp_port_start.to_string(),
        );
        map.insert("tcp_port_end".to_string(), self.tcp_port_end.to_string());
        map.insert(
            "heartbeat_interval".to_string(),
            self.heartbeat_interval.to_string(),
        );
        map.insert("peer_timeout".to_string(), self.peer_timeout.to_string());
        map.insert(
            "encryption_enabled".to_string(),
            self.encryption_enabled.to_string(),
        );
        if let Some(ref key) = self.encryption_key {
            map.insert("encryption_key".to_string(), key.clone());
        }
        map.insert(
            "offline_message_retention_days".to_string(),
            self.offline_message_retention_days.to_string(),
        );
        map.insert(
            "auto_accept_files".to_string(),
            self.auto_accept_files.to_string(),
        );
        map.insert("file_save_dir".to_string(), self.file_save_dir.clone());
        map.insert("log_level".to_string(), self.log_level.clone());
        map
    }

    /// Validate configuration
    pub fn validate(&self) -> Result<()> {
        // Validate port ranges
        if self.udp_port < 1024 {
            return Err(NeoLanError::Validation(
                "udp_port must be >= 1024".to_string(),
            ));
        }

        if self.tcp_port_start < 1024 || self.tcp_port_end < 1024 {
            return Err(NeoLanError::Validation(
                "tcp_port must be >= 1024".to_string(),
            ));
        }

        if self.tcp_port_start >= self.tcp_port_end {
            return Err(NeoLanError::Validation(
                "tcp_port_start must be less than tcp_port_end".to_string(),
            ));
        }

        // Validate heartbeat and timeout
        if self.heartbeat_interval == 0 {
            return Err(NeoLanError::Validation(
                "heartbeat_interval must be > 0".to_string(),
            ));
        }

        if self.peer_timeout < self.heartbeat_interval {
            return Err(NeoLanError::Validation(
                "peer_timeout must be >= heartbeat_interval".to_string(),
            ));
        }

        // Validate log level
        match self.log_level.as_str() {
            "trace" | "debug" | "info" | "warn" | "error" => {}
            _ => {
                return Err(NeoLanError::Validation(format!(
                    "Invalid log_level: {}. Must be one of: trace, debug, info, warn, error",
                    self.log_level
                )))
            }
        }

        // Validate file save directory exists
        if !std::path::Path::new(&self.file_save_dir).exists() {
            tracing::warn!("File save directory does not exist: {}", self.file_save_dir);
            // Note: We don't error here, as the directory might be created later
        }

        Ok(())
    }
}

impl Default for ConfigDto {
    fn default() -> Self {
        Self {
            user_id: uuid::Uuid::new_v4().to_string(),
            username: whoami::username(),
            hostname: whoami::fallible::hostname().unwrap_or_else(|_| "localhost".to_string()),
            avatar: None,
            status: None,
            bind_ip: "0.0.0.0".to_string(),
            udp_port: 2425,
            tcp_port_start: 8000,
            tcp_port_end: 9000,
            heartbeat_interval: 60,
            peer_timeout: 180,
            encryption_enabled: false,
            encryption_key: None,
            offline_message_retention_days: 30,
            auto_accept_files: false,
            file_save_dir: dirs::download_dir()
                .unwrap_or_else(|| std::path::PathBuf::from("."))
                .to_string_lossy()
                .to_string(),
            log_level: "info".to_string(),
        }
    }
}

/// Get current application configuration
///
/// This command returns the current configuration. If no configuration is saved,
/// it returns the default configuration.
///
/// # Frontend Usage
/// ```typescript
/// import { invoke } from "@tauri-apps/api/core";
/// const config = await invoke<ConfigDto>("get_config");
/// console.log(`Username: ${config.username}`);
/// ```
#[tauri::command]
pub fn get_config(state: tauri::State<AppState>) -> Result<ConfigDto> {
    tracing::info!("get_config called");

    let config = state.get_config();
    Ok(ConfigDto::from_app_config(&config))
}

/// Set application configuration
///
/// This command updates the application configuration with the provided values.
/// Only the fields included in the partial config will be updated.
///
/// # Frontend Usage
/// ```typescript
/// import { invoke } from "@tauri-apps/api/core";
/// await invoke("set_config", { config: { username: "Alice", udp_port: 2426 } });
/// ```
#[tauri::command]
pub fn set_config(state: tauri::State<AppState>, config: ConfigDto) -> Result<()> {
    tracing::info!("set_config called with: {:?}", config);

    // Validate configuration
    config.validate()?;

    // Convert and set
    let app_config = config.to_app_config();
    state.set_config(app_config);

    Ok(())
}

/// Reset configuration to default values
///
/// This command resets all configuration values to their defaults.
///
/// # Frontend Usage
/// ```typescript
/// import { invoke } from "@tauri-apps/api/core";
/// await invoke("reset_config");
/// ```
#[tauri::command]
pub fn reset_config(state: tauri::State<AppState>) -> Result<ConfigDto> {
    tracing::info!("reset_config called");

    let default_config = AppConfig::default();
    state.set_config(default_config.clone());

    Ok(ConfigDto::from_app_config(&default_config))
}

/// Get a single configuration value by key
///
/// This command returns a single configuration value by its key.
/// Returns null if the key doesn't exist.
///
/// # Frontend Usage
/// ```typescript
/// import { invoke } from "@tauri-apps/api/core";
/// const username = await invoke<string | null>("get_config_value", { key: "username" });
/// ```
#[tauri::command]
pub fn get_config_value(state: tauri::State<AppState>, key: String) -> Result<Option<String>> {
    tracing::info!("get_config_value called with key: {}", key);

    let config = state.get_config();
    let map = ConfigDto::from_app_config(&config).to_map();
    Ok(map.get(&key).cloned())
}

/// Set a single configuration value by key
///
/// This command sets a single configuration value by its key.
///
/// # Frontend Usage
/// ```typescript
/// import { invoke } from "@tauri-apps/api/core";
/// await invoke("set_config_value", { key: "username", value: "Alice" });
/// ```
#[tauri::command]
pub fn set_config_value(state: tauri::State<AppState>, key: String, value: String) -> Result<()> {
    tracing::info!(
        "set_config_value called with key: {}, value: {}",
        key,
        value
    );

    // Validate key and value
    match key.as_str() {
        "udp_port" => {
            let port: u16 = value
                .parse()
                .map_err(|_| NeoLanError::Validation(format!("Invalid port value: {}", value)))?;
            if port < 1024 {
                return Err(NeoLanError::Validation("port must be >= 1024".to_string()));
            }
            state.update_config(|c| c.udp_port = port)?;
        }
        "tcp_port_start" => {
            let port: u16 = value
                .parse()
                .map_err(|_| NeoLanError::Validation(format!("Invalid port value: {}", value)))?;
            if port < 1024 {
                return Err(NeoLanError::Validation("port must be >= 1024".to_string()));
            }
            state.update_config(|c| c.tcp_port_start = port)?;
        }
        "tcp_port_end" => {
            let port: u16 = value
                .parse()
                .map_err(|_| NeoLanError::Validation(format!("Invalid port value: {}", value)))?;
            if port < 1024 {
                return Err(NeoLanError::Validation("port must be >= 1024".to_string()));
            }
            state.update_config(|c| c.tcp_port_end = port)?;
        }
        "heartbeat_interval" => {
            let val: u64 = value
                .parse()
                .map_err(|_| NeoLanError::Validation(format!("Invalid number value: {}", value)))?;
            if val == 0 {
                return Err(NeoLanError::Validation("value must be > 0".to_string()));
            }
            state.update_config(|c| c.heartbeat_interval = val)?;
        }
        "peer_timeout" => {
            let val: u64 = value
                .parse()
                .map_err(|_| NeoLanError::Validation(format!("Invalid number value: {}", value)))?;
            if val == 0 {
                return Err(NeoLanError::Validation("value must be > 0".to_string()));
            }
            state.update_config(|c| c.peer_timeout = val)?;
        }
        "encryption_enabled" => {
            if value != "true" && value != "false" {
                return Err(NeoLanError::Validation(
                    "value must be 'true' or 'false'".to_string(),
                ));
            }
            let enabled = value == "true";
            state.update_config(|c| c.encryption_enabled = enabled)?;
        }
        "auto_accept_files" => {
            if value != "true" && value != "false" {
                return Err(NeoLanError::Validation(
                    "value must be 'true' or 'false'".to_string(),
                ));
            }
            let enabled = value == "true";
            state.update_config(|c| c.auto_accept_files = enabled)?;
        }
        "log_level" => match value.as_str() {
            "trace" | "debug" | "info" | "warn" | "error" => {
                state.update_config(|c| c.log_level = value)?;
            }
            _ => {
                return Err(NeoLanError::Validation(format!(
                    "Invalid log_level: {}",
                    value
                )))
            }
        },
        "username" => {
            state.update_config(|c| c.username = value)?;
        }
        "hostname" => {
            state.update_config(|c| c.hostname = value)?;
        }
        "bind_ip" => {
            state.update_config(|c| c.bind_ip = value)?;
        }
        "file_save_dir" => {
            state.update_config(|c| c.file_save_dir = value)?;
        }
        _ => {
            // Unknown key - ignore but log
            tracing::warn!("Unknown configuration key: {}", key);
        }
    }

    Ok(())
}
