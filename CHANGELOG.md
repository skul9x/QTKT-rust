## [2026-03-20 20:25]
### Fixed
- **Sửa lỗi hiển thị cột Tên quy trình**:
  - Khắc phục lỗi text bị mờ (fading) ở đầu dòng cho các mục ngắn.
  - Tối ưu hóa CSS Mask Gradient: Chỉ kích hoạt hiệu ứng fade khi nội dung đủ dài và đang chạy hiệu ứng Marquee (`:has(.is-scrolling)`).

## [2026-03-20 20:11]
### Added
- **Hoàn thành Phase 05 - UX State Persistence**:
  - Nâng cấp nút Copy thành trạng thái nhãn **"✔️ Done"** sau khi nhấn.
  - Áp dụng màu xanh **Emerald** đậm (bg-emerald-500) và hiệu ứng shadow cho các mục đã hoàn thành để tăng độ nhận diện (bám mắt).
  - Triển khai logic auto-reset: Toàn bộ danh sách và trạng thái "Done" sẽ được xóa sạch khi người dùng tải lên file mới hoặc kéo thả file mới.
  - Tăng độ dày icon Check (`strokeWidth={3}`) để hiển thị rõ nét hơn trên nền màu emerald.

## [2026-03-20 20:07]
### Added
- **Hoàn thành Phase 04 - JSON Generation & Copy**:
  - Tích hợp bộ máy sinh prompt JSON chuẩn từ `prompt.py`.
  - Triển khai tính năng Copy vào Clipboard với phản hồi thị giác (đổi màu nút, icon Check).
  - Thêm hệ thống thông báo **Toast** (phía dưới màn hình) xác nhận sao chép thành công.
  - Tối ưu hóa hàm `generatePromptJSON` đảm bảo format JSON 100% khớp bản Python.

## [2026-03-20 20:05]
### Added
- **Hoàn thành Phase 03 - Table & UI Effects**:
  - Triển khai giao diện bảng (Table) 3 cột: STT, Tên quy trình, Copy.
  - Tích hợp hiệu ứng **Marquee (Chữ chạy)** tự động khi hover vào các tên quy trình dài (>50 ký tự).
  - Áp dụng `mask-image` (linear-gradient) để tạo hiệu ứng fade-out mượt mà cho text ở hai đầu.
  - Tối ưu hóa layout bằng `table-fixed` và `max-w` để chống vỡ giao diện 100%.
  - Thêm thanh trạng thái (Footer) hiển thị tổng số lượng mục đã tải.
  - Cải thiện trải nghiệm di chuyển (Smooth scrolling) trong vùng bảng dữ liệu.

## [2026-03-20 20:03]
### Added
- **Khởi động Phase 02 - Tìm kiếm BYT**:
  - Triển khai logic nhập liệu từ file `.txt`.
  - Hỗ trợ Kéo thả (Drag & Drop) file trực tiếp vào giao diện.
  - Sử dụng Svelte 5 `$state` để quản lý danh sách quy trình động.
  - Tự động lọc các dòng trống và chuẩn hóa (trim) dữ liệu đầu vào.
  - Tích hợp icon từ `lucide-svelte` (FileUp, FileText, CheckCircle2).
  - Thêm tính năng "Thay đổi file" để người dùng dễ dàng cập nhật danh sách mới.


## [2026-03-15 00:20]
### Added
- Thêm tab **HDSD (Hướng dẫn sử dụng)** chi tiết, giới thiệu về công nghệ (Rust + Tauri) và cách vận hành app.
- Thêm nút **"Dừng" (Stop Button)** trong tab Tạo QTKT (Xử lý đơn & Hàng loạt).
- Hỗ trợ dừng ngay lập tức quá trình gọi API AI và xuất file DOCX.
- Thêm cấu hình **Thư mục xuất file Gộp** riêng biệt trong tab Cài đặt.
- Lưu cấu hình path gộp file vào `.merge_export_path.fallback`.
- Thêm lệnh Tauri `save_merge_export_path` và `get_merge_export_path`.

### Changed
- Cải thiện UI tab Cài đặt: Tách riêng Section cho path lưu tệp AI và tệp Gộp.
- Cập nhật logic màn `Merger` để tự động mở đúng thư mục đã cài đặt tùy chỉnh.
- Đổi tên ứng dụng (Brand name) từ "tauri-app/QTKT Rust" thành **"QTKT Maker"** đồng bộ trên toàn hệ thống (Window title, Sidebar, Build config).
- Làm sạch code: Xóa các import không sử dụng trong `docx_merger.rs`.
- Cập nhật phiên bản chính thức lên **v1.0**.
- Thêm thông tin bản quyền **"© 2026 Nguyễn Duy Trường"** vào Sidebar.

### Fixed
- Sửa lỗi nút **"Cancel"** trong dialog không dừng quy trình (vẫn chạy tiếp dù đã bấm hủy).
- Đã đồng bộ hóa logic phản hồi của Tauri Dialog (`ask`) để đảm bảo quy trình chỉ chạy khi người dùng nhấn "OK".
- Thêm logic tự động quay lại bước chọn file `.txt` (Reset View) khi nhấn "Cancel" để cải thiện UX.
- Sửa lỗi hiển thị nút Dừng không xuất hiện ngay khi bắt đầu chạy Hàng loạt (Batch mode 0%).
- Sửa lỗi cú pháp dấu ngoặc nhọn dư thừa trong `secure_storage.rs`.

## [2026-03-14 23:40]
### Fixed
- Sửa lỗi gộp DOCX: Bảo toàn khoảng trắng (`trim_text(false)`), giữ nguyên Section Properties cuối cùng và lọc sectPr lớp body.
- Sửa cảnh báo compiler (unused variable) trong `docx_merger.rs`.

### Added
- Thêm lệnh Rust native `open_folder` để mở thư mục cài đặt tùy chỉnh (vượt qua giới hạn permission của Tauri v2 Frontend).

### Changed
- Cập nhật màn hình `Generator` và `Merger` sử dụng lệnh `open_folder` mới.
- Tối ưu hóa file `capabilities/default.json` về trạng thái tối giản ổn định.

## [2026-03-14 23:20]
### Added
- Thêm Hộp thoại Thành công (Success Modal) sau khi gộp file.
- Thêm nút "Mở thư mục chứa file" bằng Tauri Opener & Core Path API.

### Changed
- Cập nhật README.md khớp với model AI thực tế đang dùng (`gemini-3-flash-preview`).
- Kích hoạt capability `core:path:default` trong cấu hình Tauri.

## [2026-03-14]
### Added
- Thêm dependency `regex` và `lazy_static` vào backend Rust.

### Changed
- Đồng bộ hóa hoàn toàn định dạng DOCX của bản Rust với bản Python chuẩn.
- Cập nhật lề trang chuẩn BYT: Trên 2.5cm, Dưới 2.0cm, Trái 3.0cm, Phải 2.5cm.
- Cập nhật font chữ chuẩn: Times New Roman, Nội dung 13pt, Tiêu đề 14pt Bold.
- Cập nhật logic in đậm đề mục tự động bằng Regex (`BOLD_PATTERNS`) khớp 100% với bản Python.
- Cập nhật Paragraph Spacing: 6pt Before cho mỗi đoạn văn.
