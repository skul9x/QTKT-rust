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
  .custom-scrollbar::-webkit-scrollbar-thumb:hover {
    background: rgba(255, 255, 255, 0.1);
  }
  
  /* Thêm animation pulse nhè nhẹ cho vùng dropzone */
  @keyframes pulse-border {
    0%, 100% { border-color: rgba(14, 165, 233, 0.3); }
    50% { border-color: rgba(14, 165, 233, 0.6); }
  }
</style>

<script lang="ts">
  import { Wand2, Send, FileText, Download, Loader2, Sparkles, FileUp, RefreshCw, FolderOpen, Square } from "lucide-svelte";
  import { invoke } from "@tauri-apps/api/core";
  import { ask } from "@tauri-apps/plugin-dialog";
  import { onMount } from "svelte";
  
  interface Props {
    isGenerating?: boolean;
    isActive?: boolean;
  }

  let { isGenerating = $bindable(false), isActive = false }: Props = $props();
  let topic = $state("");
  let progress = $state(0);
  let logs: string[] = $state([]);
  let geminiResult = $state("");
  let batchQueue: string[] = $state([]);
  let currentBatchIndex = $state(-1);
  let hasApiKey = $state(true);
  let isCancelled = $state(false);
  
  // Trạng thái để biết đang ở mode làm lẻ hay làm nguyên lô
  let isSingleMode = $state(false);

  async function checkKey() {
    try {
      const keys: string[] = await invoke("get_api_key");
      hasApiKey = keys.length > 0;
    } catch (e) {
      hasApiKey = false;
    }
  }

  $effect(() => {
    if (isActive) checkKey();
  });

  onMount(checkKey);

  async function handleImport() {
    try {
      const list: string[] = await invoke("import_procedure_list");
      batchQueue = list;
      isSingleMode = false;
      if (list.length > 0) {
        topic = list[0]; 
        logs = [`Đã tải lên danh sách ${list.length} quy trình từ file tải lên.`];
        progress = 0;
        currentBatchIndex = -1;
      }
    } catch (error) {
      if (error !== "Hủy chọn file.") {
        alert("Lỗi import: " + error);
      }
    }
  }

  function resetView() {
    batchQueue = [];
    isSingleMode = false;
    logs = [];
    progress = 0;
    topic = "";
  }

  async function handleGenerate() {
    if (!topic) return;
    await checkKey();
    if (!hasApiKey) {
      alert("Bạn chưa cấu hình API Key! Vui lòng vào tab Cài đặt để thiết lập.");
      return;
    }
    
    isSingleMode = true;
    batchQueue = []; // Xóa hàng đợi để focus vào view làm lẻ
    isGenerating = true;
    isCancelled = false;
    progress = 5;
    logs = ["Bắt đầu tiến trình sinh QTKT cho: " + topic];
    geminiResult = "";
    
    try {
      await updateStatus("Đang kiểm tra API Key...", 15);
      if (isCancelled) throw "Hủy bởi người dùng";
      
      await updateStatus("Đang gửi yêu cầu đến AI...", 40);
      const result = await invoke("generate_qtkt", { topic }) as string;
      if (isCancelled) throw "Hủy bởi người dùng";
      geminiResult = result;
      
      await updateStatus("Đã nhận phản hồi từ AI. Đang chuẩn bị dữ liệu...", 70);
      if (isCancelled) throw "Hủy bởi người dùng";
      
      await updateStatus("Đang tự động lưu file Word vào thư mục đã cài đặt...", 85);
      try {
        const filePath = await invoke("export_to_docx", { title: topic, content: result });
        if (isCancelled) throw "Hủy bởi người dùng";
        logs = [...logs, `✅ Đã lưu file thành công tại: ${filePath}`];
      } catch (saveError) {
        if (saveError === "Hủy bởi người dùng") throw saveError;
        logs = [...logs, `⚠️ Lỗi tự động lưu file: ${saveError}`];
      }

      await updateStatus("Hoàn tất! Nội dung đã sẵn sàng.", 100);
      
    } catch (error) {
      if (error === "Hủy bởi người dùng") {
        logs = [...logs, "🛑 Đã dừng tiến trình."];
      } else {
        logs = [...logs, "❌ LỖI: " + error];
      }
      progress = 0;
    } finally {
      isGenerating = false;
    }
  }

  function handleStop() {
    isCancelled = true;
    isGenerating = false;
    logs = [...logs, "🛑 Đang dừng tiến trình..."];
  }

  async function handleBatch() {
    if (batchQueue.length === 0) return;
    await checkKey();
    if (!hasApiKey) {
      alert("Bạn chưa cấu hình API Key! Vui lòng vào tab Cài đặt để thiết lập.");
      return;
    }

    const confirmRun = await ask(`Bạn sẽ chạy tự động ${batchQueue.length} quy trình.\nMỗi file sẽ tự động lưu vào thư mục cài đặt khi hoàn thành.`, {
      title: 'Xác nhận chạy hàng loạt',
      kind: 'info',
    });
    
    if (!confirmRun) {
      resetView();
      return;
    }

    isCancelled = false;
    isGenerating = true;
    logs = [`🚀 Bắt đầu chạy Lô Hành Loạt: 0/${batchQueue.length}`];
    
    for (let i = 0; i < batchQueue.length; i++) {
      if (isCancelled) {
        logs = [...logs, "🛑 ĐÃ HỦY TIẾN TRÌNH BỞI NGƯỜI DÙNG."];
        break;
      }
      currentBatchIndex = i;
      const currentTopic = batchQueue[i];
      progress = Math.round(((i) / batchQueue.length) * 100);
      
      logs = [...logs, `--- [${i + 1}/${batchQueue.length}] Đang xử lý: ${currentTopic} ---`];
      
      try {
        const result = await invoke("generate_qtkt", { topic: currentTopic }) as string;
        if (isCancelled) break;
        
        const filePath = await invoke("export_to_docx", { title: currentTopic, content: result });
        if (isCancelled) break;
        
        logs = [...logs, `✅ Hoàn thành & Đã lưu: ${filePath}`];
      } catch (error) {
        if (isCancelled) break;
        logs = [...logs, `❌ Lỗi khi xử lý "${currentTopic}": ${error}`];
      }
      
      // Delay to update UI & prevent spamming API
      await new Promise(r => setTimeout(r, 1000));
    }

    currentBatchIndex = -1;
    progress = 100;
    logs = [...logs, "🏁 ĐÃ HOÀN THÀNH TOÀN BỘ DANH SÁCH!"];
    isGenerating = false;
  }

  async function handleDownload() {
    if (!geminiResult || !topic) return;
    try {
      const filePath = await invoke("export_to_docx", { title: topic, content: geminiResult });
      alert("Đã lưu lại file thành công tại:\n" + filePath);
    } catch (error) {
      alert("Lỗi khi xuất file Word: " + error);
    }
  }

  async function openFolder() {
    try {
      const path: string = await invoke("get_export_path");
      await invoke("open_folder", { path: path });
    } catch (error) {
       alert("Lỗi khi mở thư mục: " + error);
    }
  }

  async function updateStatus(message: string, targetProgress: number) {
    logs = [...logs, message];
    progress = targetProgress;
    await new Promise(r => setTimeout(r, 600));
  }
</script>

<div class="max-w-4xl mx-auto p-8 h-full flex flex-col">
  <header class="mb-6 animate-in fade-in slide-in-from-top-4 duration-500">
    <h1 class="text-3xl font-bold text-text mb-2 flex items-center gap-3">
      <Wand2 class="text-primary w-8 h-8" />
      Soạn thảo AI
    </h1>
    <p class="text-text-muted">Sử dụng trí tuệ nhân tạo để sinh 6 mục Quy trình kỹ thuật chuẩn.</p>
  </header>

  {#if !hasApiKey}
    <div class="mb-6 p-4 bg-amber-500/10 border border-amber-500/20 rounded-2xl flex items-center gap-4 animate-in fade-in slide-in-from-top-2">
      <div class="bg-amber-500/20 p-2 rounded-xl text-amber-500">
        <Sparkles class="w-5 h-5" />
      </div>
      <div class="flex-1">
        <p class="text-amber-200 font-medium text-sm">Chưa có API Key hoạt động</p>
        <p class="text-amber-200/60 text-xs">Bạn cần thiết lập Gemini API Key trong tab <span class="font-bold underline">Cài đặt</span> để sử dụng.</p>
      </div>
    </div>
  {/if}

  <div class="flex-1 min-h-0 flex flex-col gap-6">
    <!-- KHU VỰC VÀNG (VEDETTE) - BATCH PROCESSING -->
    <section class="flex-1 bg-surface rounded-3xl border border-white/5 shadow-2xl overflow-hidden flex flex-col relative animate-in fade-in zoom-in-95 duration-500">
      {#if batchQueue.length === 0 && !isSingleMode}
        <!-- DROPZONE -->
        <button 
          onclick={handleImport}
          class="flex-1 m-6 rounded-2xl border-2 border-dashed border-primary/30 bg-primary/5 hover:bg-primary/10 hover:border-primary/50 transition-all flex flex-col items-center justify-center p-8 group cursor-pointer"
        >
          <div class="w-24 h-24 bg-primary/10 rounded-full flex items-center justify-center mb-6 group-hover:scale-110 group-hover:bg-primary/20 transition-all">
            <FileUp class="w-12 h-12 text-primary" />
          </div>
          <h2 class="text-2xl font-bold text-text mb-3">Tạo QTKT Hàng Loạt</h2>
          <p class="text-text-muted text-center max-w-sm text-sm">
            Nhấp vào đây để chọn tệp .txt chứa danh sách các quy trình. Hệ thống sẽ tự động xử lý toàn bộ nhanh chóng và lưu file Word về máy.
          </p>
        </button>
      {:else}
        <!-- LOG AND PROGRESS VIEW (For both Single and Batch) -->
        <div class="flex-1 p-8 flex flex-col min-h-0">
          <div class="flex justify-between items-start mb-6 gap-4">
            <div>
              <h2 class="text-xl font-bold text-text flex items-center gap-2 mb-1">
                <Loader2 class="w-5 h-5 {isGenerating ? 'animate-spin text-primary' : (progress === 100 ? 'text-secondary' : 'text-text-muted')}" />
                {#if isSingleMode}
                  Tiến trình xử lý: {topic}
                {:else}
                  Lô {batchQueue.length} quy trình ({currentBatchIndex >= 0 ? currentBatchIndex + 1 : 0}/{batchQueue.length})
                {/if}
              </h2>
              <p class="text-xs text-text-muted">
                {isSingleMode ? 'Chế độ tạo lẻ' : 'Chế độ tạo hàng loạt'} 
                {#if progress === 100} • <span class="text-secondary font-medium">Hoàn tất!</span>{/if}
              </p>
            </div>
            
            <div class="flex items-center gap-3">
              {#if progress === 100 || (!isGenerating && logs.length > 0)}
                <button 
                  onclick={openFolder}
                  class="flex items-center gap-2 px-4 py-2.5 bg-secondary/10 hover:bg-secondary/20 text-secondary rounded-xl font-bold text-sm transition-all border border-secondary/20 shadow-sm"
                  title="Mở thư mục chứa file"
                >
                  <FolderOpen class="w-4 h-4" />
                  Mở thư mục
                </button>

                <button 
                  onclick={resetView}
                  class="p-2 text-text-muted hover:text-white bg-white/5 hover:bg-white/10 rounded-lg transition-colors border border-white/5 shadow-sm"
                  title="Làm mới"
                >
                  <RefreshCw class="w-5 h-5" />
                </button>
              {/if}
              
              {#if !isGenerating && batchQueue.length > 0 && currentBatchIndex === -1 && !isSingleMode && progress === 0}
                 <button 
                    onclick={handleBatch}
                    class="bg-primary hover:bg-primary-dark text-white px-6 py-2.5 rounded-xl font-bold text-sm shadow-xl shadow-primary/20 active:scale-95 transition-all flex items-center gap-2"
                  >
                    <Sparkles class="w-4 h-4" />
                    Chạy toàn bộ
                  </button>
              {/if}
              {#if isGenerating || logs.length > 0}
                  <div class="flex items-center gap-2">
                    {#if isGenerating}
                      <button 
                        onclick={handleStop}
                        class="bg-red-500/10 hover:bg-red-500/20 text-red-500 border border-red-500/20 px-4 py-2 rounded-xl font-bold text-sm transition-all flex items-center gap-2 shadow-sm active:scale-95"
                      >
                        <Square class="w-3 h-3 fill-current" />
                        Dừng
                      </button>
                    {/if}
                    {#if progress > 0}
                      <span class="text-primary font-black text-2xl h-full flex items-center bg-primary/10 px-4 rounded-xl border border-primary/20">
                        {progress}%
                      </span>
                    {/if}
                  </div>
              {/if}
            </div>
          </div>

          <div class="flex-1 bg-background/50 rounded-2xl border border-white/5 p-5 overflow-y-auto font-mono text-sm space-y-3 custom-scrollbar flex flex-col shadow-inner">
             {#each logs as log, i}
              <div class="flex gap-3 animate-in fade-in slide-in-from-left-2" style="animation-delay: {Math.min(i, 20) * 50}ms">
                <span class="text-text-muted/50 shrink-0">[{new Date().toLocaleTimeString()}]</span>
                <span class={
                  log.includes('✅') || log.includes('ĐÃ HOÀN THÀNH') 
                  ? 'text-secondary font-semibold' 
                  : log.includes('❌') || log.includes('⚠️')
                    ? 'text-red-400 font-semibold'
                    : i === logs.length - 1 && isGenerating 
                      ? 'text-primary font-medium' 
                      : 'text-text-muted'
                }>{log}</span>
              </div>
            {/each}
            {#if isGenerating}
              <div class="w-1.5 h-4 bg-primary animate-pulse mt-2 ml-24 rounded-full"></div>
            {/if}
          </div>
          
          {#if progress > 0}
          <div class="mt-6 h-1.5 bg-white/5 rounded-full overflow-hidden w-full shrink-0">
            <div class="h-full bg-primary transition-all duration-500 shadow-[0_0_15px_rgba(14,165,233,0.6)]" style="width: {progress}%"></div>
          </div>
          {/if}

          {#if !isGenerating && isSingleMode && geminiResult && progress === 100}
            <div class="mt-5 flex justify-end">
               <button 
                  onclick={handleDownload}
                  class="bg-secondary/10 hover:bg-secondary/20 text-secondary border border-secondary/20 px-6 py-2 rounded-xl font-bold flex items-center justify-center gap-2 transition-all shadow-md active:scale-95"
                >
                  <Download class="w-4 h-4" />
                  Lưu bản sao
                </button>
            </div>
          {/if}
        </div>
      {/if}
    </section>

    <!-- KHU VỰC PHỤ - SINGLE GENERATION -->
    <section class="bg-surface rounded-2xl p-5 border border-white/5 shadow-lg flex-shrink-0 animate-in fade-in slide-in-from-bottom-4 duration-700 relative z-10">
      <h3 class="text-sm font-medium text-text-muted mb-3 flex items-center gap-2">
         Làm lẻ (Tạo từng quy trình)
      </h3>
      <form class="flex gap-4" onsubmit={(e) => { e.preventDefault(); handleGenerate(); }}>
        <input
          id="topic"
          bind:value={topic}
          placeholder="Ví dụ: Thay băng vết thương..."
          class="flex-1 bg-background border border-white/10 rounded-xl px-4 py-3 focus:ring-2 focus:ring-secondary/50 focus:border-secondary/50 outline-none transition-all text-sm shadow-inner placeholder:text-white/20"
        />
        <button
          type="submit"
          disabled={isGenerating || !topic}
          class="bg-secondary hover:bg-secondary-dark disabled:opacity-50 text-white px-8 py-3 rounded-xl font-bold flex items-center gap-2 transition-all shadow-lg active:scale-95 whitespace-nowrap"
        >
          {#if isGenerating && batchQueue.length === 0 && isSingleMode}
            <Loader2 class="w-4 h-4 animate-spin" />
          {:else}
            <Sparkles class="w-4 h-4" />
          {/if}
          Tạo
        </button>
      </form>
    </section>
  </div>
</div>
