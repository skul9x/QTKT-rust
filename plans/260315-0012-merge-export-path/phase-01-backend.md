# Phase 01: API Backend (Rust)
Status: ⬜ Pending

## Objective
Tạo các lệnh Tauri để lưu và lấy đường dẫn xuất file dành riêng cho tính năng Gộp (Merge).

## Requirements
### Functional
- [ ] Thêm lệnh `save_merge_export_path` để lưu chuỗi đường dẫn vào file `.merge_export_path.fallback` trong `app_data_dir`.
- [ ] Thêm lệnh `get_merge_export_path` để đọc đường dẫn. Trả về thư mục `Downloads` mặc định nếu file chưa tồn tại.
- [ ] Đăng ký 2 lệnh mới vào `tauri::Builder` trong `lib.rs`.

## Implementation Steps
1. [ ] Cập nhật `src-tauri/src/secure_storage.rs`:
   - Khai báo hằng số `MERGE_EXPORT_PATH_FILE = ".merge_export_path.fallback"`.
   - Viết hàm `save_merge_export_path(app: AppHandle, path: String) -> StorageResult<()>`.
   - Viết hàm `get_merge_export_path(app: AppHandle) -> StorageResult<String>`.
2. [ ] Cập nhật `src-tauri/src/lib.rs`: Thêm 2 hàm này vào `.invoke_handler(tauri::generate_handler![...])`.

## Files to Modify
- `src-tauri/src/secure_storage.rs`
- `src-tauri/src/lib.rs`
