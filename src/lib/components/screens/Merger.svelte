<style>
  .custom-scrollbar::-webkit-scrollbar {
    width: 6px;
  }
  .custom-scrollbar::-webkit-scrollbar-track {
    background: transparent;
  }
  .custom-scrollbar::-webkit-scrollbar-thumb {
    background: rgba(255, 255, 255, 0.05);
    border-radius: 10px;
  }
</style>

<script lang="ts">
  import { Files, UploadCloud, X, GripVertical, FileOutput, Play, Loader2, CheckCircle2, FolderOpen as FolderOpenIcon } from "lucide-svelte";
  import { flip } from "svelte/animate";
  import { invoke } from "@tauri-apps/api/core";
  import { open } from "@tauri-apps/plugin-dialog";
  import { dirname } from "@tauri-apps/api/path";

  interface FileItem {
    id: string;
    name: string;
    path: string;
    size: string;
  }

  let files: FileItem[] = $state([]);
  let outputName = $state("");
  let isMerging = $state(false);
  let showSuccessModal = $state(false);
  let lastMergedPath = $state("");

  function removeFile(id: string) {
    files = files.filter(f => f.id !== id);
  }

  async function selectFiles() {
    const selected = await open({
      multiple: true,
      filters: [{ name: "Word Documents", extensions: ["docx"] }]
    });
    
    if (selected && Array.isArray(selected)) {
      const newFiles = selected.map(path => ({
        id: Math.random().toString(),
        name: path.split("/").pop() || "Unknown",
        path: path,
        size: "Chờ gộp"
      }));
      files = [...files, ...newFiles];
    }
  }

  async function handleDrop(e: DragEvent) {
    e.preventDefault();
    await selectFiles();
  }

  async function startMerge() {
    if (files.length < 2 || !outputName) return;
    isMerging = true;
    
    try {
      const filePaths = files.map(f => f.path);
      const resultPath = await invoke("merge_docx_files", { 
        paths: filePaths, 
        outputName 
      }) as string;
      
      lastMergedPath = resultPath;
      showSuccessModal = true;
    } catch (error) {
      alert("Lỗi khi gộp file: " + error);
    } finally {
      isMerging = false;
    }
  }

  async function openFolder() {
    try {
      const dir = await dirname(lastMergedPath);
      await invoke("open_folder", { path: dir });
    } catch (e) {
      alert("Lỗi khi mở thư mục: " + e);
    }
  }
</script>

<div class="max-w-4xl mx-auto p-8 h-full flex flex-col">
  <header class="mb-10 animate-in fade-in slide-in-from-top-4 duration-500">
    <h1 class="text-3xl font-bold text-text mb-2 flex items-center gap-3">
      <Files class="text-primary w-8 h-8" />
      Gộp File Word
    </h1>
    <p class="text-text-muted">Kéo thả các tệp DOCX để gộp thành một bộ quy trình duy nhất.</p>
  </header>

  <div class="grid grid-cols-1 md:grid-cols-2 gap-8 flex-1 min-h-0">
    <!-- Drop Zone and List -->
    <div class="flex flex-col gap-4 min-h-0">
      <button 
        type="button"
        onclick={selectFiles}
        ondragover={(e) => e.preventDefault()}
        ondrop={handleDrop}
        class="w-full border-2 border-dashed border-white/10 rounded-3xl p-10 flex flex-col items-center justify-center text-center gap-4 hover:border-primary hover:bg-primary/5 transition-all cursor-pointer group shrink-0"
      >
        <div class="bg-primary/10 p-4 rounded-full text-primary group-hover:scale-110 transition-transform">
          <UploadCloud class="w-8 h-8" />
        </div>
        <div>
          <p class="text-text font-semibold">Thả file vào đây</p>
          <p class="text-text-muted text-sm">Hỗ trợ file .docx, .doc</p>
        </div>
      </button>

      <div class="flex-1 bg-surface rounded-3xl border border-white/5 overflow-hidden flex flex-col shadow-xl">
        <div class="p-4 border-b border-white/5 bg-white/5 flex justify-between items-center">
          <span class="text-sm font-semibold text-text-muted uppercase tracking-wider">Danh sách file ({files.length})</span>
          {#if files.length > 0}
            <button onclick={() => files = []} class="text-xs text-danger hover:underline">Xóa tất cả</button>
          {/if}
        </div>
        
        <div class="flex-1 overflow-y-auto p-4 space-y-2 custom-scrollbar">
          {#if files.length === 0}
            <div class="h-full flex flex-col items-center justify-center text-text-muted opacity-30 italic">
              <Files class="w-12 h-12 mb-2" />
              <p>Chưa có file nào được chọn</p>
            </div>
          {:else}
            {#each files as file (file.id)}
              <div animate:flip={{ duration: 300 }} class="bg-background/80 border border-white/5 p-4 rounded-2xl flex items-center gap-4 group">
                <GripVertical class="w-5 h-5 text-white/10 group-hover:text-white/40 cursor-grab" />
                <div class="flex-1">
                  <p class="text-sm font-medium text-text truncate">{file.name}</p>
                  <p class="text-xs text-text-muted">{file.size}</p>
                </div>
                <button 
                  onclick={() => removeFile(file.id)}
                  class="p-2 text-text-muted hover:text-danger hover:bg-danger/10 rounded-lg transition-colors"
                >
                  <X class="w-4 h-4" />
                </button>
              </div>
            {/each}
          {/if}
        </div>
      </div>
    </div>

    <!-- Export Settings -->
    <div class="flex flex-col gap-6 animate-in slide-in-from-right-8 duration-700">
      <section class="bg-surface p-8 rounded-3xl border border-white/5 shadow-2xl space-y-6">
        <h2 class="text-xl font-bold text-text flex items-center gap-2">
          <FileOutput class="w-6 h-6 text-primary" />
          Cấu hình đầu ra
        </h2>

        <div>
          <label for="output" class="block text-sm font-medium text-text-muted mb-2">Tên file bộ quy trình</label>
          <input
            id="output"
            bind:value={outputName}
            placeholder="Ví dụ: Bo_Quy_Trinh_Technique_2026"
            class="w-full bg-background border border-white/10 rounded-xl px-4 py-3 focus:ring-2 focus:ring-primary outline-none transition-all"
          />
        </div>

        <div class="p-4 bg-primary/5 border border-primary/10 rounded-2xl">
          <p class="text-sm text-text-muted mb-3">Tự động xử lý:</p>
          <ul class="space-y-2">
            <li class="flex items-center gap-2 text-xs text-text">
              <span class="w-1.5 h-1.5 bg-primary rounded-full"></span>
              Chèn ngắt trang (Page Break) giữa mỗi file
            </li>
            <li class="flex items-center gap-2 text-xs text-text">
              <span class="w-1.5 h-1.5 bg-primary rounded-full"></span>
              Chuẩn hóa lề và Font chữ (Times New Roman)
            </li>
          </ul>
        </div>

        <button
          onclick={startMerge}
          disabled={files.length < 2 || !outputName || isMerging}
          class="w-full bg-primary hover:bg-primary-dark disabled:opacity-30 text-white py-5 rounded-2xl font-bold text-lg flex items-center justify-center gap-3 transition-all shadow-xl active:scale-95"
        >
          {#if isMerging}
            <Loader2 class="w-6 h-6 animate-spin" />
            Đang xử lý...
          {:else}
            <Play class="w-6 h-6 fill-current" />
            Bắt đầu Gộp File
          {/if}
        </button>
      </section>
    </div>
  </div>
</div>

{#if showSuccessModal}
  <div class="fixed inset-0 z-50 flex items-center justify-center p-4 bg-background/80 backdrop-blur-sm animate-in fade-in duration-300">
    <div class="bg-surface border border-white/10 rounded-3xl p-8 max-w-md w-full shadow-2xl animate-in zoom-in-95 slide-in-from-bottom-8 duration-500">
      <div class="flex flex-col items-center text-center gap-6">
        <div class="bg-secondary/10 p-5 rounded-full text-secondary">
          <CheckCircle2 class="w-16 h-16" />
        </div>
        
        <div class="space-y-2">
          <h3 class="text-2xl font-bold text-text">Gộp file thành công!</h3>
          <p class="text-text-muted text-sm break-all">
            Bộ quy trình của bạn đã được xuất ra tại:<br/>
            <span class="text-primary font-mono">{lastMergedPath}</span>
          </p>
        </div>

        <div class="flex flex-col w-full gap-3 mt-2">
          <button 
            onclick={openFolder}
            class="w-full bg-primary hover:bg-primary-dark text-white py-4 rounded-2xl font-bold flex items-center justify-center gap-2 transition-all active:scale-95 shadow-lg"
          >
            <FolderOpenIcon class="w-5 h-5" />
            Mở thư mục chứa file
          </button>
          
          <button 
            onclick={() => showSuccessModal = false}
            class="w-full bg-white/5 hover:bg-white/10 text-text py-4 rounded-2xl font-semibold transition-all border border-white/5"
          >
            Đóng
          </button>
        </div>
      </div>
    </div>
  </div>
{/if}
