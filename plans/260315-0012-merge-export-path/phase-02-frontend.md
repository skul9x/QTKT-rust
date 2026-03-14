# Phase 02: Màn hình Cài đặt (Frontend)
Status: ⬜ Pending

## Objective
Thêm UI vào tab Cài đặt để người dùng chọn và xem thư mục sẽ dùng để xuất file Gộp (Merge).

## Requirements
### Functional
- [ ] Mở rộng giao diện trong `Settings.svelte`.
- [ ] Thêm biến `mergeExportPath` quản lý state.
- [ ] Khi khởi chạy Settings, gọi cả `get_export_path` và `get_merge_export_path` để hiển thị.
- [ ] Thêm nút Browse/Chọn thư mục gọi Tauri command (giống phần Tạo QTKT). Khi người dùng chọn xong, gọi `save_merge_export_path`.

## Implementation Steps
1. [ ] Cập nhật `src/lib/components/screens/Settings.svelte`:
   - Thêm phần tử DOM cho "Thư mục xuất File Gộp" nằm dưới hoặc nằm cạnh "Thư mục xuất File Sinh".
   - Tái sử dụng logic gọi `open({ directory: true })` từ plugin dialog, viết riêng 1 hàm `handleMergePathChange`.
2. [ ] Đảm bảo UI thống nhất với theme hiện tại.

## Files to Modify
- `src/lib/components/screens/Settings.svelte`
