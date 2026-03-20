# Phase 02: File Import Logic
Status: ✅ Complete
Dependencies: Phase 01

## Objective
Xây dựng logic để upload và phân tích file text chứa danh sách Quy trình.

## Tasks
- [x] Render nút `Import TXT` hoặc thẻ `<input type="file" accept=".txt" />`.
- [x] Viết hàm `handleFileUpload` để đọc nội dung file bằng Web `FileReader`.
- [x] Xử lý tách chuỗi thành mảng các Quy trình theo dấu xuống dòng (`\n`), đồng thời dọn dẹp (trim) các dòng trống.
- [x] Gán danh sách này vào một state của Svelte (`$state` hoặc biến let) dưới dạng mảng các Object tạm như `{ id, name }`.
- [x] Hiển thị console.log xem danh sách đọc ra đã chuẩn chưa.
