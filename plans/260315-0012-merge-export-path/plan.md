# Plan: Tách đường dẫn xuất file Gộp (Merge)
Created: 2026-03-15
Status: 🟡 In Progress

## Overview
Tính năng này cấp cho người dùng khả năng thiết lập một đường dẫn thư mục riêng biệt cho các tệp Word sinh ra từ mục "Gộp File", tách biệt hoàn toàn so với mục "Sinh QTKT". Cấu hình này sẽ được lưu cố định cho các lần mở app sau. Mặc định vẫn dùng thư mục Downloads nếu chưa cấu hình.

## Tech Stack
- Frontend: Svelte
- Backend: Rust (Tauri commands, `fs`)

## Phases

| Phase | Name | Status | Progress |
|-------|------|--------|----------|
| 01 | API Backend (Rust) | ✅ Complete | 100% |
| 02 | Màn hình Cài đặt (Frontend) | ✅ Complete | 100% |
| 03 | Cập nhật logic màn Gộp File | ✅ Complete | 100% |

## Quick Commands
- Start Phase 1: `/code phase-01`
- Check progress: `/next`
- Save context: `/save-brain`
