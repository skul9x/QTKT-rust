# 🩺 QTKT Maker

**QTKT Maker** là một ứng dụng Desktop mạnh mẽ được xây dựng bằng **Rust** và **Tauri**, được thiết kế để tự động hóa quy trình soạn thảo và quản lý các văn bản "Quy trình kỹ thuật" (QTKT) chuẩn Y tế Việt Nam.

![Version](https://img.shields.io/badge/version-1.0.0-blue.svg)
![License](https://img.shields.io/badge/license-MIT-green.svg)
![Platform](https://img.shields.io/badge/platform-Windows%20%7C%20Linux%20%7C%20macOS-lightgrey.svg)

## ✨ Tính năng chính

- **🤖 Soạn thảo AI:** Sử dụng mô hình **Gemini 3 Flash Preview** để tự động sinh nội dung 6 mục quy trình chuẩn Bộ Y tế.
- **🚀 Chạy hàng loạt (Batch Mode):** Nhập danh sách từ file `.txt` để tạo hàng trăm quy trình chỉ với một cú click.
- **📄 Xuất file DOCX chuẩn:** Tự động định dạng file Word theo chuẩn hành chính (Font Times New Roman, lề trang chính xác, in đậm đề mục tự động).
- **🔗 Gộp tệp DOCX:** Hợp nhất nhiều file quy trình riêng lẻ thành một cuốn danh mục duy nhất mà không làm hỏng định dạng.
- **⚙️ Quản lý API Key:** Hỗ trợ lưu trữ và luân phiên sử dụng nhiều API Key để tránh giới hạn hạn mức (rate limit).
- **📂 Tùy chỉnh đường dẫn:** Thiết lập thư mục lưu trữ riêng biệt cho tệp AI sinh ra và tệp gộp.

## 🛠️ Công nghệ sử dụng

- **Backend:** [Rust](https://www.rust-lang.org/) & [Tauri v2](https://v2.tauri.app/)
- **Frontend:** [SvelteKit](https://kit.svelte.dev/), [TailwindCSS](https://tailwindcss.com/)
- **AI Engine:** Google Gemini AI (v1beta API)
- **Formatting:** Regex-based auto-bolding & document styling

## 🚀 Hướng dẫn cài đặt

### Yêu cầu hệ thống
- [Rust & Cargo](https://rustup.rs/)
- [Node.js & npm](https://nodejs.org/)

### Các bước cài đặt
1. Clone repository:
   ```bash
   git clone https://github.com/skul9x/QTKT-rust.git
   cd QTKT-rust
   ```
2. Cài đặt dependencies:
   ```bash
   npm install
   ```
3. Chạy chế độ phát triển (Dev mode):
   ```bash
   npm run tauri dev
   ```
4. Build bản chính thức:
   ```bash
   npm run tauri build
   ```

## 📖 Hướng dẫn nhanh

1. **API Key:** Truy cập tab **Cài đặt** để dán Gemini API Key của bạn.
2. **Tạo file:** 
   - Điền tên quy trình vào ô nhập liệu để tạo lẻ.
   - Hoặc tải file `.txt` danh sách lên để tạo hàng loạt.
3. **Gộp file:** Chuyển sang tab **Gộp File**, chọn thư mục và sắp xếp thứ tự trước khi hợp nhất.

## 📄 Bản quyền
© 2026 Nguyễn Duy Trường. Phát triển với ❤️ dành cho ngành Y tế.

---
*Ghi chú: Ứng dụng này yêu cầu kết nối Internet để gọi API Gemini.*
