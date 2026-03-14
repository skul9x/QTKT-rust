# 💡 BRIEF: Cài đặt đường dẫn đặc biệt cho mục "Gộp File"

**Ngày tạo:** 15/03/2026

---

## 1. VẤN ĐỀ CẦN GIẢI QUYẾT
Hiện tại:
- Tất cả chức năng (Sinh QTKT và Gộp file DOCX) đều xài chung 1 hàm lấy đường dẫn xuất file là `get_export_path` trên Backend Rust. 
- Mà mặc định nếu chưa cài đặt gì, hàm này sẽ trả về thư mục **Downloads**.
- Người dùng muốn: Phần "Tạo QTKT" (sinh file AI) và phần "Gộp File" có cấu hình đường dẫn (Export Path) ĐỘC LẬP với nhau, để file AI và file Gộp không bị lộn xộn vào chung 1 chỗ. 
- Yêu cầu ứng dụng phải NHỚ được thiết lập đường dẫn này cho các lần mở app sau.

## 2. GIẢI PHÁP ĐỀ XUẤT
1.  **Backend (Rust)**:
    - Thêm một file lưu đường dẫn dành riêng cho Gộp file: `.merge_export_path.fallback` trong `app_data_dir`.
    - Tạo 2 lệnh mới bên Rust: `save_merge_export_path` và `get_merge_export_path`.
    - Lệnh `get_merge_export_path` vẫn có mặc định trả về thư mục **Downloads** nếu chưa cài đặt đường dẫn. 
    - Sửa logic ghép file (ví dụ hàm `merge_docx` nếu có) để dùng lệnh gọi path mới (hoặc truyền path mới tùy chọn từ frontend).
2.  **Frontend (Svelte)**:
    - Ở màn "Cài đặt" (Settings), thêm phần "Thư mục lưu file Gộp (Merge)".
    - Giao diện cung cấp ô input hoặc nút bấm gọi Dialog chọn thư mục (Browser Folder API từ Tauri).
    - Ở màn "Gộp File" (Merger), gọi hàm `get_merge_export_path` để quyết định thư mục đích cuối cùng của tệp gộp, tách bạch với thư mục của "Tạo QTKT".

## 3. ĐỐI TƯỢNG SỬ DỤNG
Người dùng sử dụng chức năng "Tạo QTKT hàng loạt" và "Gộp File", cần quản lý phân loại thư mục rõ ràng.

## 4. TÍNH NĂNG

### 🚀 MVP (Bắt buộc có):
- [ ] Thêm 2 Tauri command: `save_merge_export_path`, `get_merge_export_path` trong `secure_storage.rs`.
- [ ] Bổ sung UI vào phần Cài Đặt (Settings) cho phép chọn hoặc xem thư mục Gộp file.
- [ ] Thêm logic ở Backend/Frontend sao cho chức năng Merge lấy đúng đường dẫn rẽ nhánh này.

## 5. ƯỚC TÍNH SƠ BỘ
- **Độ phức tạp:** Đơn giản. Copy/paste mô hình đã có của Export Path cũ và đổi tên + tinh chỉnh UI.
- **Rủi ro:** Không có rủi ro lớn.

## 6. BƯỚC TIẾP THEO
→ Chạy `/plan` để lên thiết kế chi tiết (Phase setup).
