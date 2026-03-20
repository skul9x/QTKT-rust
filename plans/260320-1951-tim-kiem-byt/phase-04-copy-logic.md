# Phase 04: JSON Generation & Copy
Status: ✅ Complete
Dependencies: Phase 03

## Objective
Gắn bộ máy gen prompt JSON và kết nối chức năng sao chép (Clipboard) vào nút nhấn.

## Tasks
- [x] Chuyển format JSON chuẩn từ file `prompt.py` vào Svelte.
- [x] Viết hàm `generatePromptJSON(name)` thực hiện nhận tên quy trình và trả về chuỗi String JSON chuẩn như bản Python.
- [x] Viết hàm `copyToClipboard` nối chuỗi JSON vào Clipboard của thiết bị.
- [x] Gắn sự kiện OnClick vào nút Copy ở cột 3 cho từng dong.
- [x] Thêm thông báo nhẹ (Toast / alert) xác nhận khi đã copy xong.

