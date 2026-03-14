# Phase 03: Rotation Logic Integration
Status: ⬜ Pending
Dependencies: Phase 02

## Objective
Triển khai xoay vòng key khi gọi Gemini API.

## Requirements
### Functional
- [ ] Round-robin selection.
- [ ] Tích hợp vào `generate_qtkt`.

## Implementation Steps
1. [ ] Cập nhật `AppState` trong `lib.rs` hoặc `gemini.rs`.
2. [ ] Chỉnh sửa hàm `generate_qtkt` để chọn key từ pool.

## Files to Create/Modify
- `src-tauri/src/gemini.rs`
- `src-tauri/src/lib.rs`
