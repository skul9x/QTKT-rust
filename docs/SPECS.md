# 🛠️ SPECS: API Key Rotation System

## 1. Executive Summary
Hệ thống xoay vòng API Key cho phép ứng dụng vượt qua giới hạn rate limit của Google Gemini bằng cách sử dụng nhiều tài khoản/key khác nhau. Người dùng có thể dán một đoạn văn bản thô, ứng dụng sẽ tự động trích lọc các Key hợp lệ.

## 2. Logic trích xuất (Regex)
- **Mẫu:** `AIzaSy[A-Za-z0-9_-]{33}`
- **Hành động:** Tìm tất cả các chuỗi khớp trong văn bản và loại bỏ trùng lặp.

## 3. Database & Storage Design
- **Key:** `gemini_api_key` (trong OS Keychain).
- **Vật mang:** Chuỗi JSON string mã hóa mảng `Vec<String>`.
- **Fallback:** Tương tự, lưu JSON array vào file `.api_key.fallback`.

## 4. Logic xoay vòng (Rotation)
- **Thuật toán:** Round-robin.
- **Trạng thái:** Lưu `current_index` trong bộ nhớ ứng dụng (AppState).
- **Xử lý lỗi:** Nếu một Key báo lỗi Auth/Expire, có thể đánh dấu để bỏ qua (Phase 2).

## 5. UI/UX
- **Textarea:** `rows={5}` tối thiểu, tự động co giãn.
- **Validation:** Hiển thị số lượng Key tìm thấy ngay lập tức khi gõ/dán.
- **Bảo mật:** Dùng `type="password"` cho input đơn, nhưng với textarea sẽ dùng cơ chế làm mờ hoặc ẩn nội dung sau khi lưu.

## 6. Tech Stack
- Rust (Tauri commands, secure-storage logic).
- Svelte 5 (Runes cho state management).
- reqwest (API calls).
