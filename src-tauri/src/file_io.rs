use std::fs;
use tauri_plugin_dialog::DialogExt;

#[tauri::command]
pub async fn import_procedure_list(app: tauri::AppHandle) -> Result<Vec<String>, String> {
    let file_path = app.dialog()
        .file()
        .add_filter("Text Files", &["txt"])
        .blocking_pick_file();

    match file_path {
        Some(path) => {
            let content = fs::read_to_string(path.to_string())
                .map_err(|e| format!("Không thể đọc file: {}", e))?;
            
            let list: Vec<String> = content
                .lines()
                .map(|s| s.trim().to_string())
                .filter(|s| !s.is_empty())
                .collect();
            
            if list.is_empty() {
                return Err("File trống hoặc không có nội dung hợp lệ.".to_string());
            }
            
            Ok(list)
        }
        None => Err("Hủy chọn file.".to_string()),
    }
}

#[tauri::command]
pub async fn open_folder(path: String) -> Result<(), String> {
    #[cfg(target_os = "windows")]
    {
        std::process::Command::new("explorer")
            .arg(&path)
            .spawn()
            .map_err(|e| e.to_string())?;
    }

    #[cfg(target_os = "linux")]
    {
        std::process::Command::new("xdg-open")
            .arg(&path)
            .spawn()
            .map_err(|e| e.to_string())?;
    }

    #[cfg(target_os = "macos")]
    {
        std::process::Command::new("open")
            .arg(&path)
            .spawn()
            .map_err(|e| e.to_string())?;
    }

    Ok(())
}
