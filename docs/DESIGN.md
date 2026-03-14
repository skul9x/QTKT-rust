# 🎨 DESIGN: QTKT-Rust (Production Level)

**Ngày tạo:** 14/03/2026
**Dựa trên:** [BRIEF.md](file:///home/skul9x/Desktop/Test_code/QTKT-Rust/docs/BRIEF.md)
**Mục tiêu:** Xây dựng Desktop App chuẩn Production (Bảo mật, Hiệu năng cao, Maintainable)

---

## 1. Kiến trúc Hệ Thống (Architecture)

Vì yêu cầu là "Level Production", app sẽ được chia làm 2 lớp rõ rệt, giao tiếp qua Tauri IPC (Inter-Process Communication):

*   **Frontend (UI Layer):** SvelteKit + TailwindCSS.
    *   *Nhiệm vụ:* Hiển thị giao diện, nhận input của user, validate dữ liệu cơ bản (không bỏ trống, đúng định dạng).
    *   *Trạng thái (State):* Quản lý tiến trình (0-100%), lịch sử log hiển thị trên màn hình.
*   **Backend (Rust Layer):**
    *   *Nhiệm vụ:* Xử lý logic nặng, gọi API Gemini, tạo và gộp file DOCX, mã hóa API Key.
    *   *An toàn:* Toàn bộ code xử lý file/network phải bọc trong `Result<T, E>` để bắt mọi lỗi (Internet rớt, file bị khóa, API lỗi) và đẩy message lỗi dễ hiểu lên UI.

## 2. Cách Lưu Thông Tin (Local Storage & Security)

App không cần Database lớn (như SQL) vì không lưu dữ liệu user dùng chung. Tuy nhiên, cần lưu cấu hình và khóa API.

**📦 SƠ ĐỒ LƯU TRỮ LOCAL:**

```text
┌─────────────────────────────────────────────────────────────┐
│  ⚙️ APP CONFIG (Cấu hình) - Lưu tại `app_data_dir/config.json`
│  ├── default_font: "Times New Roman"                        │
│  ├── default_margin: { top: 3.0, bottom: 2.5, ...}         │
│  └── theme: "dark" | "light"                                │
└─────────────────────────────────────────────────────────────┘

┌─────────────────────────────────────────────────────────────┐
│  🔐 SECURITY (Bảo mật) - Tránh lộ API Key                     │
│  Dùng crate `keyring` của Rust để lưu API Key trực tiếp     │
│  vào OS Credential Manager (Windows Credential, Mac         │
│  Keychain, Linux Secret Service). Cực kỳ an toàn!           │
└─────────────────────────────────────────────────────────────┘
```

## 3. Danh Sách Màn Hình (UI Pages)

| # | Màn hình | Mục đích | Các thành phần (Components) |
|---|----------|----------|-----------------------------|
| 1 | **Tạo QTKT** | Sinh văn bản quy trình mới bằng AI | Input Tên QTKT, Nút "Bắt đầu tạo", Textbox hiển thị Log/Tiến trình thực thi theo thời gian thực. |
| 2 | **Gộp File** | Ghép nhiều file DOCX thành 1 bộ | Vùng Kéo-Thả (Drag & Drop) file, Danh sách file có thể sắp xếp (kéo lên xuống), Input Tên file đầu ra, Nút "Gộp file". |
| 3 | **Cài Đặt** | Quản lý API Key & Thay đổi cấu hình | Input API Key (ẩn `***`, nút show/hide), Nút Test API, Tùy chọn giao diện. |

## 4. Luồng Hoạt Động (User Journey)

━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
📍 HÀNH TRÌNH 1: Setup Lần Đầu (Onboarding)
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
1️⃣ Mở app lần đầu → Trực tiếp hiển thị Popup "Chào mừng! Vui lòng nhập API Key Gemini để bắt đầu".
2️⃣ User nhập API Key → Rust test thử gọi 1 câu "Hello" tới Google (Timeout 5s).
3️⃣ Nếu Pass → Rust gọi `keyring` lưu khóa an toàn → Báo thành công, vào màn hình chính.
4️⃣ Nếu Fail → Báo "Key không hợp lệ hoặc lỗi mạng".

━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
📍 HÀNH TRÌNH 2: Sinh QTKT (Happy Path)
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
1️⃣ Vào Tab "Tạo QTKT", nhập "Thay băng bỏng".
2️⃣ Bấm "Tạo!" → Khóa nút bấm (tránh spam).
3️⃣ Bắt đầu stream log từ Rust lên UI:
   - *Log:* "Đang gọi Gemini AI..."
   - *Log:* "Đã nhận kết quả. Đang dàn trang Word..."
4️⃣ Rust dùng `docx-rs` tạo file lưu vào thư mục `Downloads/QTKT_ok`.
5️⃣ *Log:* "Thành công! 🎉" → Mở sáng nút "Mở thư mục" cho user click.

━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
📍 HÀNH TRÌNH 3: Gộp File Word (Production UX)
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
1️⃣ Đang ở máy tính, user bôi đen 5 file DOCX, kéo thẳng vào Tab "Gộp File" của app.
2️⃣ Màn hình hiển thị danh sách 5 file. Dùng chuột kéo rê để đảo vị trí.
3️⃣ Nhập tên file "So_tay_đieu_duong_2026".
4️⃣ Bấm "Gộp File".
5️⃣ Rust đọc từng file, ghép nội dung, tự động chèn 1 dòng `Paragraph` có thuộc tính `PageBreak`.
6️⃣ Báo hoàn thành và mở thư mục đích.

## 5. Quy Tắc Kiểm Tra (Acceptance Criteria - Production Ready)

📋 **CHECKLIST: Tính năng "Tạo QTKT"**

✅ Cơ bản:
- [ ] Nhập từ khóa, gọi được Gemini API trả về đủ 6 mục.
- [ ] Render file DOCX với Lề chuẩn Bộ Y Tế và Font Times New Roman 13.
- [ ] File DOCX sinh ra không bị báo lỗi "Corrupted" khi mở bằng MS Word.

✅ Production (Bảo mật & Trải nghiệm):
- [ ] Không lưu API Key dưới dạng raw text trong thư mục app. Bắt buộc dùng `keyring`.
- [ ] Ứng dụng không bị "đơ" (No UI Blocking) trong suốt quá trình gọi API chờ phản hồi (Dùng `async/await` trong Rust).
- [ ] Bắt lỗi mạng: Nếu gọi API mất mạng, phải hiện lỗi "Vui lòng kiểm tra kết nối mạng" thay vì crash app.

📋 **CHECKLIST: Tính năng "Gộp File"**
- [ ] Đọc được các file DOCX hợp lệ. Các file không phải Word bị loại bỏ ngay khi kéo thả.
- [ ] Nội dung sau khi gộp giữ nguyên được Text. Thêm Page Break đúng vị trí giữa các file.

## 6. Lựa Chọn Thư Viện (Tech Stack & Crates)

**Rust Backend (Cargo.toml):**
*   `tauri = { version = "2.0.0", features = ["api-all"] }`
*   `reqwest = { version = "0.12", features = ["json"] }` (gọi AI)
*   `docx-rs = "0.4.x"` hoặc `dotox` (thao tác Word)
*   `keyring = "2.3.x"` (Hệ thống khóa OS bảo mật)
*   `serde_json`, `serde` (Xử lý Data)
*   `tokio` (Async runtime)

**Frontend:** `SvelteKit` + `TailwindCSS` + `Lucide-svelte` (Icon).

---
*Tiếp theo: Chúng ta có thể bắt đầu /code phase-01 (Setup Project).*
