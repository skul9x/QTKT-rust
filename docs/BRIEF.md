# 💡 BRIEF: Tính năng "Dừng" sinh quy trình (Stop Button)

**Ngày tạo:** 14/03/2026

---

## 1. VẤN ĐỀ CẦN GIẢI QUYẾT
Hiện tại, khi ứng dụng đang gọi AI để sinh nội dung (đặc biệt là khi chạy lô hàng loạt nhiều quy trình), hệ thống bị khóa lại (isGenerating = true) cho đến khi hoàn thành hoặc có lỗi. Người dùng không có cách nào để dừng khẩn cấp giữa chừng nếu lỡ cấu hình sai, muốn sửa lại tên, hoặc đổi ý không muốn chạy tiếp nữa.

## 2. GIẢI PHÁP ĐỀ XUẤT
Thêm một nút **"Dừng" (Stop/Cancel)** màu đỏ nổi bật xuất hiện bên cạnh tiến trình khi ứng dụng đang trong trạng thái `isGenerating`.

**Cơ chế hoạt động:**
- **Chế độ hàng loạt (Batch Mode):** Dừng ngay lập tức việc gọi các quy trình tiếp theo trong danh sách. Quy trình nào đang gọi dở sẽ đợi hoàn tất rồi hệ thống dừng hẳn.
- **Chế độ làm lẻ (Single Mode):** Ngắt trạng thái chờ trên màn hình, báo cho người dùng biết đã hủy lệnh, cho phép người dùng bấm tạo quy trình mới ngay lập tức.
- **Backend (Rust):** Việc hủy hoàn toàn (Cancel) kết nối HTTP tới Google Gemini khi đang chạy là có thể làm được nhưng phức tạp. Giải pháp tối ưu cho UX là xử lý ở Frontend (Svelte):
    - Khi bấm "Dừng", UI phải cập nhật ngay lập tức (ẩn nút Dừng, hiện lại nút Chạy).
    - Chốt chặn (isCancelled) được kiểm tra gắt gao sau mỗi bước chờ (await) để ngưng mọi hành động ghi file hay log tiếp theo.
    - Đảm bảo khi chạy Batch, nút "Dừng" xuất hiện ngay từ đầu.

## 3. ĐỐI TƯỢNG SỬ DỤNG
- Người dùng đang dùng chế độ "Tạo QTKT Hàng Loạt" cần hủy ngang.
- Người dùng đợi lâu do mạng chậm muốn hủy tiến trình hiện tại.

## 4. NGHIÊN CỨU THỊ TRƯỜNG & UX
- Hầu hết các công cụ AI (như ChatGPT, Claude) đều có nút "Stop generating" để tiết kiệm thời gian chờ và quota api.

## 5. TÍNH NĂNG

### 🚀 MVP (Bắt buộc có):
- [ ] Thêm nút **Dừng / Hủy** xuất hiện khi `isGenerating = true`.
- [ ] Xử lý logic ngắt vòng lặp (`break`) trong hàm `handleBatch` ở frontend nếu người dùng bấm dừng.
- [ ] Xử lý logic reset trạng thái UI ở hàm `handleGenerate` nếu người dùng bấm dừng.
- [ ] In thông báo log "🛑 Đã hủy tiến trình bởi người dùng" ra màn hình.

### 🎁 Phase 2 (Cân nhắc làm sau):
- [ ] Truyền tín hiệu (Cancel Token) xuống tận Backend Rust để ngắt luôn connection HTTP reqwest tới Gemini, giúp tiết kiệm triệt để quota API nếu file đang sinh dở.

## 6. ƯỚC TÍNH SƠ BỘ
- **Độ phức tạp:** **ĐƠN GIẢN** (chỉ tốn khoảng 30 phút - 1 tiếng).
- **Rủi ro:** Cần chú ý file đang làm dở có thể vẫn lưu vào máy nếu chạy ở Backend, cần quản lý kỹ thông báo log để người dùng không bối rối.

## 7. BƯỚC TIẾP THEO
→ Chạy `/plan` để lên thiết kế chi tiết các file cần sửa.
