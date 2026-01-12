// src-tauri/src/storage/database.rs
use sea_orm::{Database, DatabaseConnection};
use std::env;
use std::fs;
use std::path::PathBuf;

/// 获取跨平台应用数据目录
///
/// # 返回值
/// 返回应用数据目录路径：
/// - Windows: `%APPDATA%` (通常为 `C:\Users\<username>\AppData\Roaming`)
/// - macOS: `$HOME/Library/Application Support`
/// - Linux: `$HOME/.local/share`
fn get_app_data_dir() -> PathBuf {
    // 优先使用环境变量覆盖（用于测试）
    if let Ok(custom_dir) = env::var("NEOLAN_DATA_DIR") {
        return PathBuf::from(custom_dir);
    }

    // 根据操作系统返回标准数据目录
    #[cfg(target_os = "windows")]
    {
        if let Ok(appdata) = env::var("APPDATA") {
            return PathBuf::from(appdata);
        }
    }

    #[cfg(target_os = "macos")]
    {
        if let Ok(home) = env::var("HOME") {
            return PathBuf::from(home).join("Library").join("Application Support");
        }
    }

    #[cfg(target_os = "linux")]
    {
        if let Ok(home) = env::var("HOME") {
            return PathBuf::from(home).join(".local").join("share");
        }
    }

    // 默认使用当前目录
    PathBuf::from(".")
}

/// 建立数据库连接
///
/// # 功能
/// - 获取跨平台数据目录（支持环境变量覆盖）
/// - 创建数据库目录（如果不存在）
/// - 建立 SQLite 数据库连接
///
/// # 数据目录位置
/// - 默认：
///   - Windows: `%APPDATA%\neolan\`
///   - macOS: `~/Library/Application Support/neolan/`
///   - Linux: `~/.local/share/neolan/`
/// - 环境变量覆盖：`NEOLAN_DATA_DIR`
///
/// # 返回值
/// - `Ok(DatabaseConnection)`: 数据库连接对象
/// - `Err(String)`: 连接失败时的错误描述
pub async fn establish_connection() -> Result<DatabaseConnection, String> {
    // 1. 获取应用数据目录
    let base_dir = get_app_data_dir();
    let db_dir = base_dir.join("neolan");

    // 2. 创建数据库目录（如果不存在）
    fs::create_dir_all(&db_dir).map_err(|e| {
        format!("Failed to create database directory: {}", e)
    })?;

    tracing::info!("Database directory: {}", db_dir.display());

    let db_path = db_dir.join("neolan.db");

    // 预先创建数据库文件（如果不存在）
    // 这有助于诊断权限问题
    if !db_path.exists() {
        tracing::info!("Creating database file: {}", db_path.display());
        if let Err(e) = std::fs::File::create(&db_path) {
            return Err(format!("Failed to create database file: {}", e));
        }
    } else {
        tracing::info!("Database file already exists: {}", db_path.display());
    }

    // 3. 建立数据库连接
    // SeaORM sqlx SQLite URL 格式:
    // - 绝对路径: sqlite:///absolute/path (三个斜杠)
    // - 只编码空格，不编码斜杠
    let db_path_str = db_path.to_string_lossy();

    // 构建 URL: 三个斜杠表示绝对路径
    let database_url = format!("sqlite://{}", db_path_str.replace(' ', "%20"));

    tracing::info!("Database path: {}", db_path_str);
    tracing::info!("Connecting to database: {}", database_url);

    let db = Database::connect(&database_url)
        .await
        .map_err(|e| format!("Failed to connect to database: {}", e))?;

    tracing::info!("Database connection established successfully");

    Ok(db)
}

/// 获取数据库路径（用于测试和调试）
///
/// # 返回值
/// 返回数据库文件的完整路径
pub fn get_db_path() -> PathBuf {
    let base_dir = get_app_data_dir();
    base_dir.join("neolan").join("neolan.db")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_db_path() {
        let db_path = get_db_path();

        // 验证路径以 neolan.db 结尾
        assert!(db_path.ends_with("neolan.db"));
    }

    #[test]
    fn test_get_db_path_with_env_override() {
        // 设置环境变量
        env::set_var("NEOLAN_DATA_DIR", "/tmp/test_neolan");

        let db_path = get_db_path();

        // 验证路径使用了环境变量
        assert!(db_path.starts_with("/tmp/test_neolan"));

        // 清理环境变量
        env::remove_var("NEOLAN_DATA_DIR");
    }
}
