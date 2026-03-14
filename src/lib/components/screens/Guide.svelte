<script lang="ts">
  import { BookOpen, Info, Key, FileText, Files, Settings, Wand2, Rocket, Share2, ShieldCheck, Heart } from "lucide-svelte";
</script>

<div class="max-w-4xl mx-auto p-8 h-full overflow-y-auto custom-scrollbar">
  <header class="mb-10 animate-in fade-in slide-in-from-top-4 duration-500">
    <div class="flex items-center gap-4 mb-4">
      <div class="bg-primary/20 p-3 rounded-2xl text-primary shadow-lg shadow-primary/10">
        <BookOpen class="w-8 h-8" />
      </div>
      <div>
        <h1 class="text-3xl font-bold text-text">Hướng dẫn sử dụng</h1>
        <p class="text-text-muted">Mọi thứ bạn cần biết để làm chủ QTKT Maker.</p>
      </div>
    </div>
  </header>

  <div class="space-y-12 pb-12">
    <!-- GIỚI THIỆU -->
    <section class="animate-in fade-in slide-in-from-bottom-4 duration-500 delay-100">
      <div class="flex items-center gap-2 mb-6 text-primary">
        <Rocket class="w-5 h-5" />
        <h2 class="text-xl font-bold tracking-tight">Giới thiệu về QTKT Maker</h2>
      </div>
      <div class="bg-surface border border-white/5 rounded-3xl p-8 shadow-xl relative overflow-hidden group">
        <div class="absolute top-0 right-0 p-8 opacity-5 group-hover:opacity-10 transition-opacity">
          <Share2 class="w-32 h-32" />
        </div>
        <div class="relative z-10">
          <p class="text-text-muted leading-relaxed mb-6">
            <span class="text-white font-bold">QTKT Maker</span> là một ứng dụng Desktop mạnh mẽ được thiết kế chuyên biệt để tự động hóa quy trình soạn thảo "Quy trình kỹ thuật" (QTKT) chuẩn Y tế. Ứng dụng kết hợp sức mạnh phi thường của AI với hiệu năng vượt trội của mã nguồn hệ thống.
          </p>
          <div class="grid grid-cols-1 md:grid-cols-2 gap-4">
            <div class="bg-white/5 rounded-2xl p-4 border border-white/5">
              <h3 class="text-primary font-bold text-sm mb-2 flex items-center gap-2">
                 <ShieldCheck class="w-4 h-4" /> Core Technology
              </h3>
              <p class="text-xs text-text-muted leading-relaxed">
                Được xây dựng trên nền tảng <span class="text-orange-400 font-medium">Rust</span> mang lại tốc độ xử lý file cực nhanh và an toàn bộ nhớ tuyệt đối. Giao diện mượt mà bởi <span class="text-red-400 font-medium">SvelteKit</span> và runtime <span class="text-blue-400 font-medium">Tauri v2</span>.
              </p>
            </div>
            <div class="bg-white/5 rounded-2xl p-4 border border-white/5">
              <h3 class="text-secondary font-bold text-sm mb-2 flex items-center gap-2">
                 <Rocket class="w-4 h-4" /> Artificial Intelligence
              </h3>
              <p class="text-xs text-text-muted leading-relaxed">
                Tích hợp mô hình <span class="text-secondary font-medium">Gemini 3 Flash Preview</span> từ Google, tối ưu hóa tốc độ sinh nội dung chuyên môn Y khoa chính xác và nhanh chóng.
              </p>
            </div>
          </div>
        </div>
      </div>
    </section>

    <!-- BƯỚC 1: CẤU HÌNH -->
    <section class="animate-in fade-in slide-in-from-bottom-4 duration-500 delay-200">
      <div class="flex items-center gap-2 mb-6 text-primary">
        <Settings class="w-5 h-5" />
        <h2 class="text-xl font-bold tracking-tight">Bước 1: Thiết lập ban đầu</h2>
      </div>
      <div class="grid grid-cols-1 md:grid-cols-2 gap-6">
        <div class="bg-surface border border-white/5 rounded-3xl p-6 hover:border-primary/30 transition-colors">
          <div class="w-10 h-10 bg-amber-500/10 rounded-xl flex items-center justify-center text-amber-500 mb-4">
            <Key class="w-6 h-6" />
          </div>
          <h3 class="font-bold mb-2">Cấu hình API Key</h3>
          <p class="text-sm text-text-muted leading-relaxed">
            Vào tab <strong>Cài đặt</strong>, dán Gemini API Key của bạn. Bạn có thể thêm nhiều Key để hệ thống tự động luân phiên khi một Key hết hạn mức.
          </p>
        </div>
        <div class="bg-surface border border-white/5 rounded-3xl p-6 hover:border-primary/30 transition-colors">
          <div class="w-10 h-10 bg-blue-500/10 rounded-xl flex items-center justify-center text-blue-500 mb-4">
            <Files class="w-6 h-6" />
          </div>
          <h3 class="font-bold mb-2">Đường dẫn lưu file</h3>
          <p class="text-sm text-text-muted leading-relaxed">
            Thiết lập thư mục mặc định để lưu file AI sinh ra và file sau khi gộp. App sẽ tự động tạo thư mục nếu chưa tồn tại.
          </p>
        </div>
      </div>
    </section>

    <!-- BƯỚC 2: SỬ DỤNG -->
    <section class="animate-in fade-in slide-in-from-bottom-4 duration-500 delay-300">
      <div class="flex items-center gap-2 mb-6 text-primary">
        <Wand2 class="w-5 h-5" />
        <h2 class="text-xl font-bold tracking-tight">Bước 2: Soạn thảo AI</h2>
      </div>
      <div class="space-y-4">
        <div class="bg-surface border border-white/5 rounded-3xl p-6 flex gap-6 items-start">
          <div class="bg-primary/10 p-3 rounded-2xl text-primary shrink-0">
             <FileText class="w-6 h-6" />
          </div>
          <div>
            <h3 class="font-bold mb-1">Tạo lẻ từng quy trình</h3>
            <p class="text-sm text-text-muted leading-relaxed">
              Nhập tên quy trình (Ví dụ: "Kỹ thuật đặt Catheter tĩnh mạc trung tâm") vào ô nhập liệu ở dưới cùng và nhấn <strong>Tạo</strong>. AI sẽ soạn thảo đầy đủ 6 mục chuẩn theo cấu trúc Bộ Y tế.
            </p>
          </div>
        </div>
        <div class="bg-surface border border-white/5 rounded-3xl p-6 flex gap-6 items-start">
          <div class="bg-secondary/10 p-3 rounded-2xl text-secondary shrink-0">
             <Rocket class="w-6 h-6" />
          </div>
          <div>
            <h3 class="font-bold mb-1">Tạo hàng loạt (Batch Mode)</h3>
            <p class="text-sm text-text-muted leading-relaxed">
              Chuẩn bị một file <code>.txt</code> chứa danh sách tên quy trình (mỗi tên một dòng). Nhấn vào vùng <strong>Dropzone</strong> để tải file lên, sau đó nhấn <strong>Chạy toàn bộ</strong>. Hệ thống sẽ tự động xử lý và lưu từng file word vào máy.
            </p>
          </div>
        </div>
      </div>
    </section>

    <!-- BƯỚC 3: GỘP FILE -->
    <section class="animate-in fade-in slide-in-from-bottom-4 duration-500 delay-400">
      <div class="flex items-center gap-2 mb-6 text-primary">
        <Files class="w-5 h-5" />
        <h2 class="text-xl font-bold tracking-tight">Bước 3: Gộp tệp DOCX</h2>
      </div>
      <div class="bg-surface border border-white/5 rounded-3xl p-8">
        <p class="text-sm text-text-muted leading-relaxed mb-4">
          Tính năng gộp file giúp bạn chuyển hàng trăm file đơn lẻ thành một tập danh mục QTKT duy nhất, bảo toàn hoàn hảo định dạng, font chữ và lề trang.
        </p>
        <ul class="space-y-3">
          <li class="flex items-start gap-3 text-sm text-text-muted">
            <span class="w-6 h-6 bg-white/5 rounded-full flex items-center justify-center shrink-0 text-primary font-bold text-xs">1</span>
            Chọn thư mục chứa các file DOCX cần gộp.
          </li>
          <li class="flex items-start gap-3 text-sm text-text-muted">
            <span class="w-6 h-6 bg-white/5 rounded-full flex items-center justify-center shrink-0 text-primary font-bold text-xs">2</span>
            Kéo thả để sắp xếp thứ tự các file trong danh sách.
          </li>
          <li class="flex items-start gap-3 text-sm text-text-muted">
            <span class="w-6 h-6 bg-white/5 rounded-full flex items-center justify-center shrink-0 text-primary font-bold text-xs">3</span>
            Nhấn <strong>Bắt đầu gộp</strong> và đợi kết quả trong giây lát.
          </li>
        </ul>
      </div>
    </section>

    <!-- FOOTER -->
    <footer class="pt-8 border-t border-white/5 text-center animate-in fade-in slide-in-from-bottom-4 duration-500 delay-500">
      <div class="flex justify-center gap-2 text-text-muted mb-4">
        <Heart class="w-4 h-4 text-red-500 fill-current" />
        <span class="text-xs">Phát triển bởi Nguyễn Duy Trường - 2026</span>
      </div>
      <p class="text-[10px] text-text-muted/50 uppercase tracking-[0.2em]">QTKT Maker • Professional Tool for Healthcare</p>
    </footer>
  </div>
</div>
