# Phase 03: Cập nhật logic màn Gộp File
Status: ⬜ Pending

## Objective
Gắn đường dẫn mới (Merge Export Path) vào tính năng gộp file.

## Requirements
### Functional
- [ ] Mở màn `Merger.svelte`, tìm đến button "Gộp tất cả". 
- [ ] Trong logic `handleMerge`, sửa hàm `openFolder` để mở đúng `get_merge_export_path` thay vì path cũ (`get_export_path`).
- [ ] Backend Rust (`merge_files` cmd): Truyền thêm tham số path vào hàm `merge_docx` (hoặc để Backend tự gọi config tùy cấu trúc hiện tại). 
    - Nếu `merge_docx` (trong `docx_merger.rs`) hiện đang đọc `get_export_path`, phải sửa thành đọc `get_merge_export_path`.

## Implementation Steps
1. [ ] Kiểm tra `src-tauri/src/docx_merger.rs` và `src-tauri/src/commands.rs`. Tìm nơi gọi logic Gộp file và đường dẫn được tiêm vào từ đâu.
2. [ ] Thay đổi đường dẫn thành hàm `get_merge_export_path` truyền từ app context.
3. [ ] Cập nhật `openFolder` trong `Merger.svelte` để mở đúng chỗ chứa file gộp.
4. [ ] Test kỹ quy trình cả Gộp file và Sinh AI file độc lập.

## Files to Modify
- `src-tauri/src/docx_merger.rs` (Nếu path nằm ở đây)
- `src/lib/components/screens/Merger.svelte`
