use keyring::Entry;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;
use tauri::{AppHandle, Manager};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum StorageError {
    KeyNotFound,
    AccessDenied,
    SystemError(String),
}

impl std::fmt::Display for StorageError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            StorageError::KeyNotFound => write!(f, "Không tìm thấy dữ liệu."),
            StorageError::AccessDenied => write!(f, "Truy cập bị từ chối."),
            StorageError::SystemError(e) => write!(f, "Lỗi hệ thống: {}", e),
        }
    }
}

type StorageResult<T> = Result<T, StorageError>;

const SERVICE_NAME: &str = "qtkt-rust";
const KEY_NAME: &str = "gemini_api_key";
const FALLBACK_FILE: &str = ".api_key.fallback";
const EXPORT_PATH_FILE: &str = ".export_path.fallback";
const MERGE_EXPORT_PATH_FILE: &str = ".merge_export_path.fallback";

/// Lấy đường dẫn file dự phòng trong thư mục app_data
fn get_fallback_path(app: &AppHandle) -> StorageResult<PathBuf> {
    let mut path = app.path().app_data_dir()
        .map_err(|e| StorageError::SystemError(format!("Không lấy được app_data_dir: {}", e)))?;
    
    // Tạo thư mục nếu chưa có
    if !path.exists() {
        fs::create_dir_all(&path).map_err(|e| StorageError::SystemError(e.to_string()))?;
    }
    
    path.push(FALLBACK_FILE);
    Ok(path)
}

#[tauri::command]
pub fn save_api_key(app: AppHandle, keys: Vec<String>) -> StorageResult<()> {
    let json_keys = serde_json::to_string(&keys)
        .map_err(|e| StorageError::SystemError(format!("Serialization failed: {}", e)))?;

    println!("💾 Attempting to save {} API keys...", keys.len());

    // 1. Luôn lưu vào file dự phòng trên Linux để đảm bảo tính ổn định
    let path = get_fallback_path(&app)?;
    println!("📂 Saving to fallback file: {:?}", path);
    fs::write(&path, &json_keys).map_err(|e| StorageError::SystemError(format!("Fallback save failed: {}", e)))?;

    // 2. Thử dùng Keyring (Bổ sung tính bảo mật nếu có thể)
    let entry = Entry::new(SERVICE_NAME, KEY_NAME).map_err(|e| StorageError::SystemError(e.to_string()))?;
    match entry.set_password(&json_keys) {
        Ok(_) => {
            println!("✅ Keys also saved to system Keyring.");
        },
        Err(e) => {
            println!("⚠️ Keyring save failed (ignorable since fallback worked): {}", e);
        }
    }
    
    Ok(())
}

#[tauri::command]
pub fn get_api_key(app: AppHandle) -> StorageResult<Vec<String>> {
    println!("🔍 Attempting to retrieve API keys...");
    // 1. Thử lấy từ Keyring
    let entry = Entry::new(SERVICE_NAME, KEY_NAME).map_err(|e| StorageError::SystemError(e.to_string()))?;
    match entry.get_password() {
        Ok(key_data) => {
            println!("✅ Keys retrieved from system Keyring.");
            Ok(parse_keys(key_data))
        },
        Err(e) => {
            println!("⚠️ Keyring read error: {}. Trying fallback file...", e);
            // 2. Nếu không thấy hoặc lỗi, thử tìm ở file dự phòng
            let path = get_fallback_path(&app)?;
            println!("📂 Checking fallback file at: {:?}", path);
            if path.exists() {
                let key_data = fs::read_to_string(&path).map_err(|e| StorageError::SystemError(format!("Fallback read failed: {}", e)))?;
                println!("✅ Keys retrieved from fallback file.");
                Ok(parse_keys(key_data))
            } else {
                println!("❌ No API keys found at: {:?}", path);
                Err(StorageError::KeyNotFound)
            }
        }
    }
}

// Helper function to parse either a JSON array or a legacy raw string
fn parse_keys(raw: String) -> Vec<String> {
    match serde_json::from_str::<Vec<String>>(&raw) {
        Ok(keys) => keys,
        Err(_) => {
            // If parsing fails, it's likely a legacy single-key string
            if raw.trim().is_empty() {
                vec![]
            } else {
                vec![raw]
            }
        }
    }
}

#[tauri::command]
pub fn delete_api_key(app: AppHandle) -> StorageResult<()> {
    // 1. Xóa trong Keyring
    let entry = Entry::new(SERVICE_NAME, KEY_NAME).map_err(|e| StorageError::SystemError(e.to_string()))?;
    let _ = entry.delete_credential();

    // 2. Xóa file dự phòng
    let path = get_fallback_path(&app)?;
    if path.exists() {
        let _ = fs::remove_file(path);
    }
    
    Ok(())
}

#[tauri::command]
pub fn save_export_path(app: AppHandle, path: String) -> StorageResult<()> {
    let mut config_path = app.path().app_data_dir()
        .map_err(|e| StorageError::SystemError(format!("Không lấy được app_data_dir: {}", e)))?;
    
    if !config_path.exists() {
        fs::create_dir_all(&config_path).map_err(|e| StorageError::SystemError(e.to_string()))?;
    }
    
    config_path.push(EXPORT_PATH_FILE);
    fs::write(&config_path, path).map_err(|e| StorageError::SystemError(format!("Lưu đường dẫn thất bại: {}", e)))?;
    
    Ok(())
}
#[tauri::command]
pub fn get_export_path(app: AppHandle) -> StorageResult<String> {
    let mut config_path = app.path().app_data_dir()
        .map_err(|e| StorageError::SystemError(format!("Không lấy được app_data_dir: {}", e)))?;
    
    config_path.push(EXPORT_PATH_FILE);
    if config_path.exists() {
        let path = fs::read_to_string(&config_path).map_err(|e| StorageError::SystemError(format!("Đọc đường dẫn thất bại: {}", e)))?;
        Ok(path.trim().to_string())
    } else {
        // Mặc định là thư mục Downloads/QTKT nếu chưa cài đặt
        let mut download_dir = app.path().download_dir()
            .map_err(|e| StorageError::SystemError(format!("Không tìm thấy thư mục Downloads: {}", e)))?;
        download_dir.push("QTKT");
        Ok(download_dir.to_string_lossy().to_string())
    }
}

#[tauri::command]
pub fn save_merge_export_path(app: AppHandle, path: String) -> StorageResult<()> {
    let mut config_path = app.path().app_data_dir()
        .map_err(|e| StorageError::SystemError(format!("Không lấy được app_data_dir: {}", e)))?;
    
    if !config_path.exists() {
        fs::create_dir_all(&config_path).map_err(|e| StorageError::SystemError(e.to_string()))?;
    }
    
    config_path.push(MERGE_EXPORT_PATH_FILE);
    fs::write(&config_path, path).map_err(|e| StorageError::SystemError(format!("Lưu đường dẫn thất bại: {}", e)))?;
    
    Ok(())
}

#[tauri::command]
pub fn get_merge_export_path(app: AppHandle) -> StorageResult<String> {
    let mut config_path = app.path().app_data_dir()
        .map_err(|e| StorageError::SystemError(format!("Không lấy được app_data_dir: {}", e)))?;
    
    config_path.push(MERGE_EXPORT_PATH_FILE);
    if config_path.exists() {
        let path = fs::read_to_string(&config_path).map_err(|e| StorageError::SystemError(format!("Đọc đường dẫn thất bại: {}", e)))?;
        Ok(path.trim().to_string())
    } else {
        // Mặc định là thư mục Downloads/QTKT nếu chưa cài đặt
        let mut download_dir = app.path().download_dir()
            .map_err(|e| StorageError::SystemError(format!("Không tìm thấy thư mục Downloads: {}", e)))?;
        download_dir.push("QTKT");
        Ok(download_dir.to_string_lossy().to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_keys_json() {
        let json = r#"["key1", "key2"]"#.to_string();
        let keys = parse_keys(json);
        assert_eq!(keys, vec!["key1".to_string(), "key2".to_string()]);
    }

    #[test]
    fn test_parse_keys_legacy() {
        let legacy = "single_key_123".to_string();
        let keys = parse_keys(legacy);
        assert_eq!(keys, vec!["single_key_123".to_string()]);
    }

    #[test]
    fn test_parse_keys_empty() {
        let empty = "  ".to_string();
        let keys = parse_keys(empty);
        let expected: Vec<String> = vec![];
        assert_eq!(keys, expected);
    }
}
