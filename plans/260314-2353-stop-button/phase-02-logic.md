# Phase 02: Xử lý Logic Hủy
Status: ⬜ Pending
Dependencies: Phase 01

## Objective
Thay đổi logic của các hàm sinh (Batch và Single) để ngắt tiến trình khi biến `isCancelled` được kích hoạt.

## Requirements
### Functional
- [ ] Trong `handleBatch`: Kiểm tra cờ `isCancelled` ở mỗi đầu vòng lặp VÀ ngay sau mỗi `await invoke`.
- [ ] Trong `handleGenerate`: Kiểm tra `isCancelled` ngay sau mỗi `await invoke`. Nếu bị hủy, dừng log và không save file.
- [ ] Reset `isCancelled = false` và `isGenerating = true` đồng thời khi bắt đầu chạy.

## Implementation Steps
1. [ ] Cập nhật `handleBatch`: Thêm logic kiểm tra `if isCancelled` và thoát vòng lặp, thay log báo hiệu người dùng chủ động ngắt. Đảm bảo set `isGenerating = false`.
2. [ ] Cập nhật `handleGenerate`: Nếu `isCancelled = true` thì chỉ update trạng thái thông báo, ẩn thanh progress, không tiến hành save file hoặc load file. 
3. [ ] Bổ sung reset `isCancelled = false` mỗi khi gọi `handleGenerate` hoặc `handleBatch` (Bắt đầu một process mới).

## Files to Modify
- `src/lib/components/screens/Generator.svelte`

---
Next Phase: phase-03-testing.md
