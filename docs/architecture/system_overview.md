# System Architecture - QTKT-Rust

## 🏛️ Tổng quan Kiến trúc
Ứng dụng được xây dựng trên nền tảng **Tauri v2**, phân tách rạch ròi giữa lớp giao diện (Frontend) và lớp xử lý hệ thống (Backend).

## 🎨 Frontend Layer (UI/UX)
- **Framework**: SvelteKit (Svelte 5 Runes).
- **Styling**: TailwindCSS v3.
- **Components**:
  - `Sidebar`: Quản lý trạng thái điều hướng chính.
  - `screens/`: Chứa các module chức năng độc lập (Generator, Merger, Settings).
  - `Onboarding`: Quản lý luồng người dùng mới.
- **Communication**: Sử dụng `@tauri-apps/api/core` để gọi Rust Commands qua IPC.

## ⚙️ Backend Layer (Rust)
- **Core modules**:
  - `secure_storage`: Quản lý API Key an toàn thông qua thư viện `keyring`, giao tiếp với OS Keychain.
  - `lib.rs`: Đăng ký và điều phối các commands cho Frontend.
- **Security**: Không lưu API Key dưới dạng văn bản thô. Sử dụng mã hóa mức hệ điều hành.

## 🔄 Luồng dữ liệu (Data Flow)
1. User nhập API Key -> Frontend gọi `save_api_key`.
2. Rust nhận key -> Gọi `keyring` -> Lưu vào Treasure Chest của OS.
3. Khi cần gọi AI -> Rust gọi `get_api_key` -> Trả về API Key cho Logic xử lý Network.

## ⚠️ Giới hạn & Lưu ý
- **Linux Dependency**: Yêu cầu môi trường có `libsecret` hoặc dịch vụ D-Bus Secret Service tương đương.
