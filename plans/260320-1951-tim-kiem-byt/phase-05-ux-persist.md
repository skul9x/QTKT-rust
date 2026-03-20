# Phase 05: UX State Persistence
Status: ✅ Completed
Dependencies: Phase 04

## Objective
Nâng cấp trải nghiệm nút Copy thành trạng thái đánh dấu liên tục theo yêu cầu của User.

## Tasks
- [x] Nâng cấp cấu trúc dữ liệu State mỗi hàng thành `{ id, name, isCopied: false }`.
- [x] Cập nhật hàm nhấn Copy: Khi nhấn, đổi `isCopied` của obj đó thành `true`.
- [x] Phản hồi UI rẽ nhánh: 
  - Nếu `!isCopied` -> Hiện chữ "Copy".
  - Nếu `isCopied` -> Hiện "Dấu tích ✔️ Done" (đổi màu xanh bám mắt).
- [x] Viết logic hàm Upload file ở Phase 2: Bất cứ khi nào 1 file MỚI được tải lên, toàn bộ danh sách bị thiết lập lại và `isCopied` trở về `false` tất cả.
