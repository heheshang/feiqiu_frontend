// src-tauri/src/config/app.rs
use crate::error::{NeoLanError, Result};
use crate::storage::entities::settings;
use sea_orm::*;
use serde::{Deserialize, Serialize};

/// 网络配置默认值常量
impl AppConfig {
    /// IPMsg 标准默认 UDP 端口
    pub const DEFAULT_UDP_PORT: u16 = 2425;

    /// TCP 端口范围起始值（用于文件传输）
    pub const DEFAULT_TCP_PORT_START: u16 = 8000;

    /// TCP 端口范围结束值（用于文件传输）
    pub const DEFAULT_TCP_PORT_END: u16 = 9000;

    /// 默认绑定 IP 地址（0.0.0.0 表示所有网卡）
    pub const DEFAULT_BIND_IP: &'static str = "0.0.0.0";

    /// 广播地址（用于 LAN 发现）
    pub const BROADCAST_ADDR: &'static str = "255.255.255.255";

    /// UDP 接收缓冲区大小（64KB，最大 UDP 包大小）
    pub const UDP_BUFFER_SIZE: usize = 65535;

    /// TCP 传输缓冲区大小（4KB，用于文件传输）
    pub const TCP_BUFFER_SIZE: usize = 4096;

    /// 默认心跳间隔（秒）
    pub const DEFAULT_HEARTBEAT_INTERVAL: u64 = 30; // FeiQ expects 30 seconds

    /// 默认节点超时时间（秒）
    pub const DEFAULT_PEER_TIMEOUT: u64 = 180;

    /// 默认离线消息保留天数
    pub const DEFAULT_OFFLINE_MESSAGE_RETENTION_DAYS: u32 = 30;

    /// 默认日志级别
    pub const DEFAULT_LOG_LEVEL: &'static str = "info";
}

/// 应用程序配置
///
/// 存储应用程序的核心配置参数，包括网络设置、用户信息等
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AppConfig {
    /// 用户唯一标识（UUID，用于局域网内唯一识别用户）
    pub user_id: String,

    /// 用户名（显示给其他节点）
    pub username: String,

    /// 主机名
    pub hostname: String,

    /// 绑定 IP 地址
    pub bind_ip: String,

    /// UDP 端口（控制消息：上线/下线/心跳）
    pub udp_port: u16,

    /// TCP 端口范围（文件传输）
    pub tcp_port_start: u16,
    pub tcp_port_end: u16,

    /// 心跳间隔（秒）
    pub heartbeat_interval: u64,

    /// 超时时间（秒）- 超过此时间未活动的节点视为离线
    pub peer_timeout: u64,

    /// 是否启用加密（AES-256-GCM）
    pub encryption_enabled: bool,

    /// 加密密钥（Base64 编码）
    pub encryption_key: Option<String>,

    /// 离线消息保留天数
    pub offline_message_retention_days: u32,

    /// 是否自动接受文件传输
    pub auto_accept_files: bool,

    /// 文件保存目录
    pub file_save_dir: String,

    /// 日志级别：trace, debug, info, warn, error
    pub log_level: String,
}

impl AppConfig {
    /// 获取 UDP 接收缓冲区大小
    pub fn udp_buffer_size(&self) -> usize {
        Self::UDP_BUFFER_SIZE
    }

    /// 获取 TCP 传输缓冲区大小
    pub fn tcp_buffer_size(&self) -> usize {
        Self::TCP_BUFFER_SIZE
    }

    /// 获取广播地址
    pub fn broadcast_addr(&self) -> &'static str {
        Self::BROADCAST_ADDR
    }

    /// 验证配置的有效性
    pub fn validate(&self) -> Result<()> {
        // 验证 UDP 端口范围
        if self.udp_port == 0 {
            return Err(NeoLanError::Validation(
                "UDP port cannot be zero".to_string(),
            ));
        }

        // 验证 TCP 端口范围
        if self.tcp_port_start >= self.tcp_port_end {
            return Err(NeoLanError::Validation(
                "TCP port start must be less than port end".to_string(),
            ));
        }

        if self.tcp_port_start < 1024 {
            return Err(NeoLanError::Validation(
                "TCP port start must be >= 1024 (privileged ports are reserved)".to_string(),
            ));
        }

        // 验证超时设置
        if self.peer_timeout <= self.heartbeat_interval {
            return Err(NeoLanError::Validation(
                "Peer timeout must be greater than heartbeat interval".to_string(),
            ));
        }

        // 验证绑定 IP
        if self.bind_ip.is_empty() {
            return Err(NeoLanError::Validation(
                "Bind IP cannot be empty".to_string(),
            ));
        }

        Ok(())
    }
}

impl Default for AppConfig {
    fn default() -> Self {
        // Generate user_id if not exists
        Self {
            user_id: uuid::Uuid::new_v4().to_string(),
            username: whoami::username(),
            hostname: whoami::fallible::hostname().unwrap_or_else(|_| "localhost".to_string()),
            bind_ip: Self::DEFAULT_BIND_IP.to_string(),
            udp_port: Self::DEFAULT_UDP_PORT,
            tcp_port_start: Self::DEFAULT_TCP_PORT_START,
            tcp_port_end: Self::DEFAULT_TCP_PORT_END,
            heartbeat_interval: Self::DEFAULT_HEARTBEAT_INTERVAL,
            peer_timeout: Self::DEFAULT_PEER_TIMEOUT,
            encryption_enabled: false,
            encryption_key: None,
            offline_message_retention_days: Self::DEFAULT_OFFLINE_MESSAGE_RETENTION_DAYS,
            auto_accept_files: false,
            file_save_dir: dirs::download_dir()
                .unwrap_or_else(|| std::path::PathBuf::from("."))
                .to_string_lossy()
                .to_string(),
            log_level: Self::DEFAULT_LOG_LEVEL.to_string(),
        }
    }
}

/// 配置存储键名常量
mod keys {
    pub const CONFIG: &str = "app_config";
}

/// 配置仓库
///
/// 负责从 settings 表加载和保存配置
#[derive(Clone)]
pub struct ConfigRepository {
    db: DatabaseConnection,
}

impl ConfigRepository {
    /// 创建新的 ConfigRepository
    pub fn new(db: DatabaseConnection) -> Self {
        Self { db }
    }

    /// 加载应用配置
    ///
    /// 如果数据库中没有配置，则返回默认配置
    pub async fn load_app_config(&self) -> Result<AppConfig> {
        let setting = settings::Entity::find()
            .filter(settings::Column::Key.eq(keys::CONFIG))
            .one(&self.db)
            .await
            .map_err(|e| NeoLanError::Storage(format!("Failed to load config: {}", e)))?;

        match setting {
            Some(s) => {
                // 从 JSON 反序列化配置
                serde_json::from_str(&s.value)
                    .map_err(|e| NeoLanError::Config(format!("Failed to parse config JSON: {}", e)))
            }
            None => Ok(AppConfig::default()),
        }
    }

    /// 保存应用配置
    pub async fn save_app_config(&self, config: &AppConfig) -> Result<()> {
        let json_value = serde_json::to_string(config)
            .map_err(|e| NeoLanError::Config(format!("Failed to serialize config: {}", e)))?;

        // 检查是否已存在配置
        let existing = settings::Entity::find()
            .filter(settings::Column::Key.eq(keys::CONFIG))
            .one(&self.db)
            .await
            .map_err(|e| NeoLanError::Storage(format!("Failed to query config: {}", e)))?;

        let now = chrono::Utc::now().naive_utc();

        if let Some(existing_setting) = existing {
            // 更新现有配置
            let mut active_model: settings::ActiveModel = existing_setting.into();
            active_model.value = Set(json_value);
            active_model.updated_at = Set(now);

            settings::Entity::update(active_model)
                .exec(&self.db)
                .await
                .map_err(|e| NeoLanError::Storage(format!("Failed to update config: {}", e)))?;
        } else {
            // 插入新配置
            let active_model = settings::ActiveModel {
                key: Set(keys::CONFIG.to_string()),
                value: Set(json_value),
                updated_at: Set(now),
            };

            settings::Entity::insert(active_model)
                .exec(&self.db)
                .await
                .map_err(|e| NeoLanError::Storage(format!("Failed to insert config: {}", e)))?;
        }

        Ok(())
    }

    /// 重置为默认配置
    pub async fn reset_to_default(&self) -> Result<()> {
        self.save_app_config(&AppConfig::default()).await
    }

    /// 获取单个配置值
    ///
    /// # 参数
    /// - `key`: 配置键名
    ///
    /// # 返回
    /// 配置值的 JSON 字符串
    pub async fn get_value(&self, key: &str) -> Result<Option<String>> {
        let setting = settings::Entity::find()
            .filter(settings::Column::Key.eq(key))
            .one(&self.db)
            .await
            .map_err(|e| NeoLanError::Storage(format!("Failed to get value: {}", e)))?;

        Ok(setting.map(|s| s.value))
    }

    /// 设置单个配置值
    ///
    /// # 参数
    /// - `key`: 配置键名
    /// - `value`: 配置值（JSON 字符串）
    pub async fn set_value(&self, key: &str, value: &str) -> Result<()> {
        let now = chrono::Utc::now().naive_utc();

        // 检查是否已存在
        let existing = settings::Entity::find()
            .filter(settings::Column::Key.eq(key))
            .one(&self.db)
            .await
            .map_err(|e| NeoLanError::Storage(format!("Failed to query value: {}", e)))?;

        if let Some(existing_setting) = existing {
            // 更新
            let mut active_model: settings::ActiveModel = existing_setting.into();
            active_model.value = Set(value.to_string());
            active_model.updated_at = Set(now);

            settings::Entity::update(active_model)
                .exec(&self.db)
                .await
                .map_err(|e| NeoLanError::Storage(format!("Failed to update value: {}", e)))?;
        } else {
            // 插入
            let active_model = settings::ActiveModel {
                key: Set(key.to_string()),
                value: Set(value.to_string()),
                updated_at: Set(now),
            };

            settings::Entity::insert(active_model)
                .exec(&self.db)
                .await
                .map_err(|e| NeoLanError::Storage(format!("Failed to insert value: {}", e)))?;
        }

        Ok(())
    }

    /// 删除配置值
    pub async fn delete_value(&self, key: &str) -> Result<()> {
        let result = settings::Entity::delete_many()
            .filter(settings::Column::Key.eq(key))
            .exec(&self.db)
            .await
            .map_err(|e| NeoLanError::Storage(format!("Failed to delete value: {}", e)))?;

        if result.rows_affected == 0 {
            return Err(NeoLanError::Config(format!(
                "Config key not found: {}",
                key
            )));
        }

        Ok(())
    }

    /// 获取所有配置项
    pub async fn get_all_settings(&self) -> Result<Vec<settings::Model>> {
        let result = settings::Entity::find()
            .all(&self.db)
            .await
            .map_err(|e| NeoLanError::Storage(format!("Failed to get all settings: {}", e)))?;

        Ok(result)
    }
}
