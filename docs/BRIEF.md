# 💡 BRIEF: Tính năng Tìm kiếm BYT (Clone từ prompt.py)

**Ngày tạo:** 2026-03-20
**Brainstorm cùng:** User

---

## 1. VẤN ĐỀ CẦN GIẢI QUYẾT
Người dùng hiện đang dùng script Python (`prompt.py`) để sinh các đoạn prompt tìm kiếm quyết định BYT dựa trên file txt danh sách "Quy trình kỹ thuật". 

## 2. GIẢI PHÁP ĐỀ XUẤT
Tạo hoàn toàn một Tab mới có tên là **"Tìm kiếm BYT"** ngay trong ứng dụng. Tab này cho phép đọc file `.txt` list sẵn, render ra bảng, và xuất JSON prompt siêu tốc với cải tiến UX tốt hơn so với script python cũ.

## 3. ĐỐI TƯỢNG SỬ DỤNG
- **Primary:** Người dùng làm việc với văn bản pháp quy tế (tra cứu BYT).

## 4. TÍNH NĂNG

### 🚀 MVP (Bắt buộc có):
- [x] **Giao diện tab:** Thêm Navigation item cho tab **"Tìm kiếm BYT"**.
- [ ] **Import File:** Tính năng upload 1 file `.txt` chứa danh sách quy trình thành file.
- [ ] **Giao diện Bảng:** 3 cột chức năng: 
  - **Cột 1:** STT.
  - **Cột 2:** Tên Quy trình (Text). Trường hợp text siêu dài -> tự động áp dụng hiệu ứng trượt chữ (marquee) để bảo vệ layout bảng không bị phình.
  - **Cột 3:** Nút "Copy". Đặc biệt, **Cải tiến UX trạng thái:** Khi user click "Copy", nút sẽ chuyển thành hiệu ứng "Dấu tích / Done" (Đã Copy) để user dễ theo dõi. Trạng thái "Đã Copy" này sẽ được lưu giữ liên tục trên UI cho đến khi user import một file `.txt` mới thì mới reset lại.
- [ ] **Logic Gen Prompt JSON & Copy:** Khi bấm, sinh JSON dựa trên format có sẵn với dòng tương ứng rồi chép thẳng vô Clipboard (Kèm Toast "Đã theo tác..").

### 💭 Backlog (Có thể cập nhật sau này):
- Xem chi tiết source JSON.

## 5. ƯỚC TÍNH SƠ BỘ
- **Độ phức tạp:** Đơn giản
- **Rủi ro:** Không có. JSON rule được hardcode 1:1 từ Python.

## 6. BƯỚC TIẾP THEO
→ Chạy `/plan` để lên thiết kế chi tiết
