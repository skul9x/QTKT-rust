# Phase 01: Cập nhật UI (Nút Dừng)
Status: ⬜ Pending

## Objective
Thêm nút "Dừng" vào màn hình Generator khi tiến trình đang chạy (`isGenerating = true`). Thêm trạng thái (state) để nhận biết cờ hủy ngang.

## Requirements
### Functional
- [ ] Khai báo biến state `isCancelled = $state(false)`
- [ ] Thêm nút bấm "🛑 Dừng lại" bên cạnh thanh tiến trình khi `isGenerating == true`
- [ ] Khi bấm nút, set `isCancelled = true`

## Implementation Steps
1. [ ] Cập nhật `src/lib/components/screens/Generator.svelte`: Thêm state `isCancelled`.
2. [ ] Thiết kế nút Dừng (màu đỏ) ở vị trí dễ nhìn trên thanh điều khiển tiến trình.
3. [ ] Hàm `handleStop` (hoặc inline) đổi giá trị `isCancelled`.

## Files to Modify
- `src/lib/components/screens/Generator.svelte`

---
Next Phase: phase-02-logic.md
