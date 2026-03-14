<script lang="ts">
  import { invoke } from "@tauri-apps/api/core";
  import { Shield, Key, RefreshCw, CheckCircle2, AlertCircle, FolderOpen, Settings as SettingsIcon } from "lucide-svelte";
  import { onMount } from "svelte";
  import { open } from "@tauri-apps/plugin-dialog";
 
  interface Props {
    isGenerating?: boolean;
  }

  let { isGenerating = false }: Props = $props();
  let apiKey = $state("");
  let exportPath = $state("");
  let mergeExportPath = $state("");
  let status: "idle" | "loading" | "success" | "error" = $state("idle");
  let message = $state("");

  async function loadSettings() {
    try {
      const savedKeys: string[] = await invoke("get_api_key");
      apiKey = savedKeys.join("\n");
      
      const savedPath: string = await invoke("get_export_path");
      exportPath = savedPath;

      const savedMergePath: string = await invoke("get_merge_export_path");
      mergeExportPath = savedMergePath;
    } catch (e) {
      console.log("Error loading settings:", e);
    }
  }

  const API_KEY_REGEX = /AIzaSy[A-Za-z0-9_-]{33}/g;

  function extractKeys(text: string): string[] {
    const matches = text.match(API_KEY_REGEX);
    return matches ? [...new Set(matches)] : [];
  }

  let detectedKeysCount = $derived(extractKeys(apiKey).length);

  async function handleSave() {
    const keys = extractKeys(apiKey);
    if (keys.length === 0) {
      status = "error";
      message = "Không tìm thấy API Key hợp lệ nào trong văn bản!";
      return;
    }

    status = "loading";
    message = `Đang lưu ${keys.length} API Key...`;

    try {
      await invoke("save_api_key", { keys });
      status = "success";
      message = `Đã lưu thành công ${keys.length} API Key!`;
      setTimeout(() => { if (status === "success") status = "idle" }, 3000);
    } catch (e) {
      status = "error";
      message = "Lỗi khi lưu: " + e;
    }
  }

  async function handleDelete() {
    if (!confirm("Bạn có chắc chắn muốn xóa API Key khỏi hệ thống?")) return;
    try {
      await invoke("delete_api_key");
      apiKey = "";
      status = "idle";
      message = "Đã xóa API Key.";
    } catch (e) {
      alert("Lỗi khi xóa: " + e);
    }
  }

  async function handlePickDir() {
    try {
      const selected = await open({
        directory: true,
        multiple: false,
        defaultPath: exportPath,
      });

      if (selected && typeof selected === "string") {
        exportPath = selected;
        await invoke("save_export_path", { path: selected });
        status = "success";
        message = "Đã cập nhật thư mục lưu file Sinh.";
        setTimeout(() => { if (status === "success") status = "idle" }, 3000);
      }
    } catch (e) {
      status = "error";
      message = "Lỗi khi chọn thư mục: " + e;
    }
  }

  async function handlePickMergeDir() {
    try {
      const selected = await open({
        directory: true,
        multiple: false,
        defaultPath: mergeExportPath,
      });

      if (selected && typeof selected === "string") {
        mergeExportPath = selected;
        await invoke("save_merge_export_path", { path: selected });
        status = "success";
        message = "Đã cập nhật thư mục lưu file Gộp.";
        setTimeout(() => { if (status === "success") status = "idle" }, 3000);
      }
    } catch (e) {
      status = "error";
      message = "Lỗi khi chọn thư mục gộp: " + e;
    }
  }

  onMount(loadSettings);
</script>

<div class="max-w-4xl mx-auto p-8 animate-in fade-in slide-in-from-bottom-4 duration-500">
  <header class="mb-10">
    <h1 class="text-3xl font-bold text-text mb-2 flex items-center gap-3">
      <Shield class="text-primary w-8 h-8" />
      Cài đặt hệ thống
    </h1>
    <p class="text-text-muted">Quản lý bảo mật và cấu hình ứng dụng chuẩn Y tế.</p>
  </header>

  <div class="grid gap-6">
    <!-- Security Section -->
    <section class="bg-surface p-6 rounded-2xl border border-white/5 shadow-xl">
      <div class="flex items-center gap-3 mb-6">
        <div class="bg-primary/10 p-2 rounded-lg text-primary">
          <Key class="w-5 h-5" />
        </div>
        <h2 class="text-xl font-semibold text-text">Bảo mật API Gemini</h2>
      </div>

      <div class="space-y-4">
        <div>
          <label for="api-key" class="block text-sm font-medium text-text-muted mb-2 flex justify-between items-center">
            <span>Danh sách Gemini API Keys</span>
            {#if detectedKeysCount > 0}
              <span class="bg-primary/20 text-primary text-[10px] px-2 py-0.5 rounded-full font-bold animate-in zoom-in-50">
                Đã phát hiện {detectedKeysCount} Key
              </span>
            {/if}
          </label>
          <div class="relative">
            <textarea
              id="api-key"
              bind:value={apiKey}
              rows="5"
              placeholder="Dán mã API Key của bạn tại đây... (Có thể dán cả văn bản chứa Key, hệ thống sẽ tự lọc)"
              class="w-full bg-background border border-white/10 rounded-xl px-4 py-3 focus:ring-2 focus:ring-primary focus:border-transparent outline-none transition-all font-mono text-sm resize-none"
            ></textarea>
          </div>
          <p class="mt-2 text-xs text-text-muted italic">Mẹo: Bạn có thể dán danh sách Key cách nhau bởi dấu phẩy hoặc xuống dòng. Hệ thống sẽ tự động nhận diện mẫu AIzaSy... </p>
        </div>

        <div class="flex items-center gap-3">
          <button
            onclick={handleSave}
            disabled={status === "loading" || !apiKey || isGenerating}
            class="bg-primary hover:bg-primary-dark disabled:opacity-50 text-white px-6 py-2.5 rounded-xl font-semibold flex items-center gap-2 transition-all shadow-lg active:scale-95"
          >
            {#if status === "loading"}
              <RefreshCw class="w-5 h-5 animate-spin" />
              Đang lưu...
            {:else}
              Lưu khóa bảo mật
            {/if}
          </button>

          {#if apiKey}
            <button
              onclick={handleDelete}
              disabled={isGenerating}
              class="text-danger hover:bg-danger/10 disabled:opacity-30 px-4 py-2.5 rounded-xl text-sm transition-all"
            >
              Xóa khỏi máy
            </button>
          {/if}
        </div>

        {#if status !== "idle"}
          <div class="mt-4 p-4 rounded-xl flex items-center gap-3 animate-in zoom-in-95 duration-200 
            {status === 'success' ? 'bg-secondary/10 text-secondary border border-secondary/20' : 'bg-danger/10 text-danger border border-danger/20'}">
            {#if status === "success"}
              <CheckCircle2 class="w-5 h-5 shrink-0" />
            {:else}
              <AlertCircle class="w-5 h-5 shrink-0" />
            {/if}
            <span class="text-sm font-medium">{message}</span>
          </div>
        {/if}
      </div>
    </section>

    <!-- Export Path Section -->
    <section class="bg-surface p-6 rounded-2xl border border-white/5 shadow-xl">
      <div class="flex items-center gap-3 mb-6">
        <div class="bg-secondary/10 p-2 rounded-lg text-secondary">
          <FolderOpen class="w-5 h-5" />
        </div>
        <h2 class="text-xl font-semibold text-text">Thư mục xuất file</h2>
      </div>

      <div class="space-y-6">
        <!-- AI Export Path -->
        <div class="p-4 bg-background/40 rounded-xl border border-white/5">
          <label for="export-path" class="block text-xs font-bold text-text-muted uppercase tracking-wider mb-3">
             Thư mục lưu "Sinh QTKT" (Tạo lẻ & Hàng loạt)
          </label>
          <div class="flex gap-2">
            <input
              id="export-path"
              type="text"
              readonly
              bind:value={exportPath}
              placeholder="Chưa cài đặt (Mặc định: Downloads/QTKT)"
              class="flex-1 bg-background border border-white/10 rounded-xl px-4 py-2.5 outline-none text-sm text-text-muted cursor-default overflow-hidden text-ellipsis"
            />
            <button
              onclick={handlePickDir}
              disabled={isGenerating}
              class="bg-white/5 hover:bg-white/10 text-text disabled:opacity-30 px-4 py-2.5 rounded-xl font-medium flex items-center gap-2 transition-all border border-white/10 active:scale-95 whitespace-nowrap"
            >
              <FolderOpen class="w-4 h-4" />
              Chọn thư mục
            </button>
          </div>
        </div>

        <!-- Merge Export Path -->
        <div class="p-4 bg-background/40 rounded-xl border border-white/5">
          <label for="merge-path" class="block text-xs font-bold text-secondary-dark uppercase tracking-wider mb-3 opacity-80">
             Thư mục lưu "Gộp File DOCX"
          </label>
          <div class="flex gap-2">
            <input
              id="merge-path"
              type="text"
              readonly
              bind:value={mergeExportPath}
              placeholder="Chưa cài đặt (Mặc định: Downloads/QTKT)"
              class="flex-1 bg-background border border-white/10 rounded-xl px-4 py-2.5 outline-none text-sm text-text-muted cursor-default overflow-hidden text-ellipsis"
            />
            <button
              onclick={handlePickMergeDir}
              disabled={isGenerating}
              class="bg-secondary/5 hover:bg-secondary/10 text-secondary disabled:opacity-30 px-4 py-2.5 rounded-xl font-medium flex items-center gap-2 transition-all border border-secondary/20 active:scale-95 whitespace-nowrap"
            >
              <FolderOpen class="w-4 h-4" />
              Chọn thư mục
            </button>
          </div>
        </div>
      </div>
    </section>

    <!-- General Config -->
    <section class="bg-surface p-6 rounded-2xl border border-white/5 shadow-xl opacity-50 cursor-not-allowed">
      <div class="flex items-center gap-3 mb-6">
        <div class="bg-white/5 p-2 rounded-lg text-text-muted">
          <SettingsIcon class="w-5 h-5" />
        </div>
        <h2 class="text-xl font-semibold text-text">Cấu hình văn bản (Sắp ra mắt)</h2>
      </div>
      <div class="grid grid-cols-2 gap-4">
        <div class="bg-background/50 p-4 rounded-xl border border-white/5">
          <p class="text-xs text-text-muted uppercase tracking-tighter mb-1">Font chữ mặc định</p>
          <p class="text-sm text-text">Times New Roman</p>
        </div>
        <div class="bg-background/50 p-4 rounded-xl border border-white/5">
          <p class="text-xs text-text-muted uppercase tracking-tighter mb-1">Cỡ chữ</p>
          <p class="text-sm text-text">13pt</p>
        </div>
      </div>
    </section>
  </div>
</div>
