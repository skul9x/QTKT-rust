# Phase 01: Backend Storage Refactor
Status: ✅ Complete
Dependencies: None

## Objective
Nâng cấp logic lưu trữ để hỗ trợ danh sách API Key thay vì chỉ một chuỗi đơn.

## Requirements
### Functional
- [ ] `save_api_key` chấp nhận JSON array.
- [ ] `get_api_key` trả về danh sách key.
- [ ] Xử lý dữ liệu cũ để không làm hỏng app.

### Non-Functional
- [ ] Đảm bảo tính bảo mật (dữ liệu vẫn được lưu trong Keychain).

## Implementation Steps
1. [ ] Chỉnh sửa `secure_storage.rs`: Đổi kiểu dữ liệu input/output.
2. [ ] Viết hàm helper parse dữ liệu cũ.
3. [ ] Cập nhật fallback storage mechanism.

## Files to Create/Modify
- `src-tauri/src/secure_storage.rs`

## Test Criteria
- [ ] Lưu 1 key, lấy ra được 1 key.
- [ ] Lưu 3 key, lấy ra được danh sách 3 key.
