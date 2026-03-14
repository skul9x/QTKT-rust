// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
mod secure_storage;
mod gemini;
mod docx_generator;
mod docx_merger;

mod file_io;

#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

use std::collections::HashMap;
use std::sync::{atomic::AtomicUsize, Mutex};
use std::time::Instant;

pub struct KeyStatus {
    pub exhausted: bool,
    pub cooldown_until: Option<Instant>,
}

pub struct AppState {
    pub key_index: AtomicUsize,
    pub key_statuses: Mutex<HashMap<String, KeyStatus>>,
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .manage(AppState {
            key_index: AtomicUsize::new(0),
            key_statuses: Mutex::new(HashMap::new()),
        })
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_dialog::init())
        .invoke_handler(tauri::generate_handler![
            greet,
            secure_storage::save_api_key,
            secure_storage::get_api_key,
            secure_storage::delete_api_key,
            secure_storage::save_export_path,
            secure_storage::get_export_path,
            secure_storage::save_merge_export_path,
            secure_storage::get_merge_export_path,
            gemini::generate_qtkt,
            docx_generator::export_to_docx,
            docx_merger::merge_docx_files,
            file_io::import_procedure_list,
            file_io::open_folder
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
