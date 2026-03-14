# Phase 03: Testing & Hoàn thiện
Status: ⬜ Pending
Dependencies: Phase 02

## Objective
Đảm bảo nút Dừng hoạt động ổn định, không gây crash ứng dụng hay lỗi vòng đời quá trình (batch loop). Cập nhật giao diện mượt mà.

## Test Criteria
- [ ] Khởi chạy lệnh build frontend `npm run tauri dev`.
- [ ] Test 1: Bấm Dừng khi đang chạy lô (Batch). Đảm bảo dừng ngay sau quy trình hiện tại, tệp chưa chạy sẽ không được tạo. Thanh progress biến mất hoặc dừng lại, hiện log thông báo ngừng.
- [ ] Test 2: Bấm Dừng khi chạy lẻ (Single). Có thể phải đợi API call hoàn thành do backend không drop connection, nhưng màn hình báo "Đã Hủy" và không tự động bật lưu hoặc mở file.
- [ ] Test 3: Chạy lại tiến trình mới sau khi đã Dừng. Mọi thứ bắt đầu lại bình thường từ 0%, biến `isCancelled` reset thành false.

## Notes
- Do giới hạn Backend khó ngắt trực tiếp kết nối TCP đang gọi cho proxy mà không thiết lập signal phức tạp, việc hủy MVP sẽ giải quyết chủ yếu bằng cách ngắt vòng lặp chạy lô trên Frontend. Điều này rất thực tế và đáp ứng tốt 90% nhu cầu người dùng.
