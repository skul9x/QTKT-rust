# Design Specifications: Tab Tìm Kiếm BYT

## 🎨 Color Palette (Theo Tailwind Config hiện tại)
| Name | Hex | Tailwind Class | Usage |
|------|-----|----------------|-------|
| Background | #0f172a | `bg-background` | Nền chính của tab |
| Surface | #1e293b | `bg-surface` | Nền của bảng danh sách, thẻ |
| Primary | #0ea5e9 | `text-primary`, `bg-primary` | Nút Copy trạng thái thường |
| Secondary | #10b981 | `text-secondary`, `bg-secondary` | Nút trạng thái "✔️ Đã copy" |
| Text Main | #f8fafc | `text-text` | Chữ chính hiển thị |
| Border / Muted | #94a3b8 | `border-surface`, `text-text-muted` | Viền bảng, chữ phụ |

## 📝 Typography
- **Font chính:** `Inter, sans-serif`
- **Tiêu đề bảng:** `text-sm font-semibold uppercase text-text-muted`
- **Dữ liệu bảng:** `text-sm text-text`

## 📐 Thành Phần Components
1. **Nút Import TXT:** Nút to, có viền đứt nét (dashed), nằm trên đầu màn hình để thu hút sự chú ý.
2. **Bảng dữ liệu:** 
   - Viền bo góc `rounded-lg` với `border border-surface/50`.
   - Cột giữa (Tên quy trình) kết hợp tính năng `truncate` và `flex-1` để không đẩy rớt nút Copy bên phải.
   - Thêm hiệu ứng *Marquee (chữ chạy ngang tĩnh)* bằng việc wrap dòng chữ trong khoảng trống `overflow-hidden`.
3. **Nút Copy & Done:**
   - Bo góc `rounded-md`, kèm icon Lucide-Svelte (`Copy` và `Check`).
   - Có hiệu ứng `transition-all duration-200 ease-in-out` khi swap giữa 2 trạng thái.

## ✨ Animation Marquee (Bổ sung vào global css nếu cần)
```css
@keyframes marquee {
  0% { transform: translateX(0); }
  100% { transform: translateX(-100%); }
}
.animate-marquee {
  display: inline-block;
  animation: marquee 12s linear infinite;
  white-space: nowrap;
}
```
