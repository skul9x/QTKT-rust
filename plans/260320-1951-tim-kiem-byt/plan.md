# Plan: Tìm kiếm BYT Feature
Created: 2026-03-20
Status: 🟡 In Progress

## Overview
Tạo tab "Tìm kiếm BYT" trong ứng dụng Qtkt Maker (SvelteKit + Tauri) để thay thế script `prompt.py`. 
App sẽ nhận file text `.txt`, phân tách thành danh sách quy trình, hiển thị lên bảng chống vỡ layout (kèm marquee), 
cung cấp nút Copy sinh ra JSON tương ứng, đồng thời lưu giữ trạng thái "Dấu tích / Đã copy" liên tục cho đến khi tải file mới.

## Tech Stack
- Frontend: SvelteKit.
- Styling: Tailwind CSS.
- Logic: JS Web API (FileReader, Clipboard).

## Phases

| Phase | Name | Status | Progress |
|-------|------|--------|----------|
| 01 | Setup Route & Layout | ✅ Complete | 100% |
| 02 | File Import Logic | ✅ Complete | 100% |
| 03 | Table & UI Effects | ✅ Complete | 100% |
| 04 | JSON Generation & Copy | ✅ Complete | 100% |
| 05 | UX State Persistence | ✅ Complete | 100% |


## Quick Commands
- Bắt đầu Phase 1: `/code phase-01`
- Kiểm tra tiến độ: `/next`
