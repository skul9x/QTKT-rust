# Phase 02: UI Update (Multi-key Input)
Status: ⬜ Pending
Dependencies: Phase 01

## Objective
Cập nhật giao diện để người dùng dán được nhiều API Key.

## Requirements
### Functional
- [ ] `textarea` thay cho `input`.
- [ ] Regex extraction logic.
- [ ] Badge hiển thị số lượng key.

## Implementation Steps
1. [ ] Sửa `Settings.svelte`: UI đổi mới.
2. [ ] Thêm logic Regex trên Svelte.
3. [ ] Sửa `Onboarding.svelte`.

## Files to Create/Modify
- `src/lib/components/screens/Settings.svelte`
- `src/lib/components/Onboarding.svelte`

## Test Criteria
- [ ] Dán đoạn văn bản rác chứa API Key, app lọc đúng key.
