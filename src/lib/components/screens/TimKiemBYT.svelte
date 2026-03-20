<script lang="ts">
  import { FileUp, FileText, CheckCircle2, Copy, Check } from 'lucide-svelte';
  
  interface ProcessItem {
    id: number;
    name: string;
    isCopied: boolean;
  }

  let fileInput: HTMLInputElement;
  let processList = $state<ProcessItem[]>([]);
  let isDragging = $state(false);
  let showToast = $state(false);
  let toastMessage = $state('');

  const generatePromptJSON = (name: string) => {
    const data = {
      "task": "Tìm kiếm online trên mạng các văn bản Bộ Y tế",
      "input_parameters": {
        "Ten_Quy_Trinh": name
      },
      "description": `Tìm xem quy trình kỹ thuật có tên "${name}" có xuất hiện trong văn bản nào của Bộ Y tế.`,
      "search_rules": {
        "document_type": "Quyết định của Bộ Y tế",
        "exclude": ["Thông tư"],
        "search_scope": "Danh mục quy trình kỹ thuật trong các quyết định ban hành hoặc cập nhật danh mục kỹ thuật"
      },
      "required_output": {
        "format": "table",
        "columns": [
          "Số thứ tự văn bản tìm thấy",
          "Tên văn bản (tên Quyết định của Bộ Y tế)",
          "Số thứ tự của danh mục kỹ thuật trong văn bản đó",
          "Ghi chú (nếu có)"
        ]
      }
    };
    return JSON.stringify(data, null, 2);
  };

  const copyToClipboard = async (item: ProcessItem) => {
    const jsonStr = generatePromptJSON(item.name);
    try {
      await navigator.clipboard.writeText(jsonStr);
      
      // Update state
      processList = processList.map(p => 
        p.id === item.id ? { ...p, isCopied: true } : p
      );
      
      // Show toast
      toastMessage = `Đã copy prompt cho: ${item.name.substring(0, 20)}${item.name.length > 20 ? '...' : ''}`;
      showToast = true;
      setTimeout(() => {
        showToast = false;
      }, 3000);
      
      console.log('✅ Đã copy JSON vào clipboard');
    } catch (err) {
      console.error('❌ Lỗi khi copy:', err);
      toastMessage = 'Lỗi khi copy vào clipboard';
      showToast = true;
      setTimeout(() => {
        showToast = false;
      }, 3000);
    }
  };

  const handleFileUpload = (event: Event) => {
    const input = event.target as HTMLInputElement;
    if (!input.files || input.files.length === 0) return;

    const file = input.files[0];
    const reader = new FileReader();

    reader.onload = (e) => {
      const content = e.target?.result as string;
      const lines = content.split('\n')
        .map(line => line.trim())
        .filter(line => line.length > 0);
      
      processList = lines.map((name, index) => ({
        id: index + 1,
        name,
        isCopied: false
      }));

      console.log('✅ Đã import danh sách quy trình:', processList);
    };

    reader.readAsText(file);
  };

  const triggerFileInput = () => {
    fileInput.click();
  };

  const handleDragOver = (e: DragEvent) => {
    e.preventDefault();
    isDragging = true;
  };

  const handleDragLeave = () => {
    isDragging = false;
  };

  const handleDrop = (e: DragEvent) => {
    e.preventDefault();
    isDragging = false;
    
    if (e.dataTransfer?.files && e.dataTransfer.files.length > 0) {
      const file = e.dataTransfer.files[0];
      if (file.type === 'text/plain' || file.name.endsWith('.txt')) {
        const reader = new FileReader();
        reader.onload = (event) => {
          const content = event.target?.result as string;
          const lines = content.split('\n')
            .map(line => line.trim())
            .filter(line => line.length > 0);
          
          processList = lines.map((name, index) => ({
            id: index + 1,
            name,
            isCopied: false
          }));
          console.log('✅ Đã import từ drop:', processList);
        };
        reader.readAsText(file);
      }
    }
  };
</script>

<style>
  .marquee-container {
    width: 100%;
    overflow: hidden;
    white-space: nowrap;
  }

  /* Chỉ fade khi text dài (marquee scrolling) */
  .marquee-container:has(.is-scrolling) {
    mask-image: linear-gradient(to right, transparent, black 15px, black calc(100% - 15px), transparent);
  }

  .marquee-content {
    display: inline-block;
    padding-left: 15px;
    padding-right: 15px;
  }

  /* Hiệu ứng scroll khi hover vào dòng */
  .group:hover .marquee-content.is-scrolling {
    animation: scroll-text 15s linear infinite;
  }

  @keyframes scroll-text {
    0% { transform: translateX(0); }
    100% { transform: translateX(-50%); }
  }

  /* Custom scrollbar cho vung table */
  .table-container::-webkit-scrollbar {
    width: 6px;
  }
  .table-container::-webkit-scrollbar-track {
    background: transparent;
  }
  .table-container::-webkit-scrollbar-thumb {
    background: rgba(255, 255, 255, 0.05);
    border-radius: 10px;
  }
  .table-container::-webkit-scrollbar-thumb:hover {
    background: rgba(255, 255, 255, 0.1);
  }

  /* Toast Animation */
  .toast-enter {
    animation: slide-up 0.3s ease-out forwards;
  }

  @keyframes slide-up {
    from { transform: translateY(100%); opacity: 0; }
    to { transform: translateY(0); opacity: 1; }
  }
</style>

<div class="p-8 max-w-6xl mx-auto flex flex-col h-full overflow-hidden animate-in fade-in slide-in-from-bottom-4 duration-500 ease-out relative">
  <div class="mb-8 flex items-end justify-between shrink-0">
    <div>
      <div class="flex items-center gap-3 mb-2">
        <div class="p-2 bg-primary/10 rounded-lg">
          <FileText size={24} class="text-primary" />
        </div>
        <h1 class="text-3xl font-bold bg-gradient-to-r from-primary to-primary-dark bg-clip-text text-transparent">
          Tìm Kiếm Quy Trình Y Tế
        </h1>
      </div>
      <p class="text-text-muted">Tính năng sinh JSON prompt nhanh dựa trên danh sách quy trình y tế.</p>
    </div>

    {#if processList.length > 0}
      <button 
        onclick={triggerFileInput}
        class="flex items-center gap-2 px-4 py-2 bg-surface hover:bg-surface-hover border border-white/10 rounded-xl transition-all duration-300 text-sm font-medium hover:shadow-lg hover:shadow-primary/5 active:scale-95"
      >
        <FileUp size={16} class="text-primary" />
        <span>Thay đổi file</span>
      </button>
    {/if}
  </div>

  <input 
    type="file" 
    accept=".txt" 
    bind:this={fileInput} 
    onchange={handleFileUpload} 
    class="hidden" 
  />

  <div class="flex-1 overflow-hidden min-h-0 flex flex-col">
    {#if processList.length === 0}
      <button
        onclick={triggerFileInput}
        ondragover={handleDragOver}
        ondragleave={handleDragLeave}
        ondrop={handleDrop}
        class="w-full h-full flex flex-col items-center justify-center border-2 border-dashed {isDragging ? 'border-primary bg-primary/5' : 'border-white/5 bg-surface'} rounded-3xl transition-all duration-500 group"
      >
        <div class="w-20 h-20 rounded-2xl bg-primary/10 flex items-center justify-center mb-6 group-hover:scale-110 group-hover:bg-primary/20 transition-all duration-500">
          <FileText size={40} class="text-primary" />
        </div>
        <h3 class="text-xl font-semibold mb-2 text-text">Tải lên danh sách quy trình</h3>
        <p class="text-text-muted mb-8 max-w-sm text-center">Chọn hoặc kéo thả file <span class="text-primary">.txt</span> chứa danh sách tên quy trình để bắt đầu.</p>
        
        <div class="px-6 py-3 bg-primary hover:bg-primary-dark text-white rounded-xl font-medium transition-all duration-300 shadow-lg shadow-primary/20 hover:scale-105 active:scale-95">
          Chọn file TXT
        </div>
      </button>
    {:else}
      <div class="flex-1 bg-surface border border-white/5 rounded-2xl overflow-hidden flex flex-col shadow-2xl">
        <div class="table-container flex-1 overflow-y-auto min-h-0">
          <table class="w-full text-left border-collapse table-fixed">
            <thead class="sticky top-0 z-10 bg-surface border-b border-white/5 shadow-sm">
              <tr>
                <th class="px-6 py-4 text-xs font-semibold text-text-muted uppercase tracking-wider w-20">STT</th>
                <th class="px-6 py-4 text-xs font-semibold text-text-muted uppercase tracking-wider">Tên quy trình</th>
                <th class="px-6 py-4 text-xs font-semibold text-text-muted uppercase tracking-wider w-36 text-center">Copy</th>
              </tr>
            </thead>
            <tbody class="divide-y divide-white/5">
              {#each processList as item (item.id)}
                <tr class="group hover:bg-white/[0.02] transition-colors h-16">
                  <td class="px-6 py-4 text-sm text-text-muted font-medium">{item.id}</td>
                  <td class="px-6 py-4">
                    <div class="marquee-container">
                      {#if item.name.length > 70}
                        <div class="marquee-content flex w-max is-scrolling items-center group-hover:text-primary transition-colors duration-300">
                          <span class="pr-16 text-sm text-text group-hover:text-primary transition-colors">{item.name}</span>
                          <span class="pr-16 text-sm text-text group-hover:text-primary transition-colors">{item.name}</span>
                        </div>
                      {:else}
                        <div class="truncate text-sm text-text group-hover:text-primary transition-colors duration-300 w-full" style="max-width: 400px;">
                          {item.name}
                        </div>
                      {/if}
                    </div>
                  </td>
                  <td class="px-6 py-4 text-center">
                    <button 
                      onclick={() => copyToClipboard(item)}
                      class="inline-flex items-center gap-2 justify-center px-4 py-1.5 {item.isCopied ? 'bg-emerald-500 text-white shadow-lg shadow-emerald-500/20' : 'bg-primary/10 text-primary hover:bg-primary/20'} rounded-lg text-xs font-bold transition-all duration-300 hover:scale-105 active:scale-95 min-w-[100px]"
                    >
                      {#if item.isCopied}
                        <Check size={14} strokeWidth={3} />
                        <span>✔️ Done</span>
                      {:else}
                        <Copy size={14} />
                        <span>Copy</span>
                      {/if}
                    </button>
                  </td>
                </tr>
              {/each}
            </tbody>
          </table>
        </div>
        
        <div class="p-4 bg-white/[0.02] border-t border-white/5 flex justify-between items-center shrink-0">
          <span class="text-xs text-text-muted">
            Tổng cộng <span class="text-text font-medium">{processList.length}</span> mục
          </span>
          <div class="flex gap-2">
            <div class="flex items-center gap-1.5 px-3 py-1 bg-white/5 rounded-full text-[10px] text-text-muted uppercase tracking-wider">
               <div class="w-1.5 h-1.5 rounded-full bg-primary animate-pulse"></div>
               JSON Gen Ready
            </div>
          </div>
        </div>
      </div>
    {/if}
  </div>

  {#if showToast}
    <div class="absolute bottom-12 left-1/2 -translate-x-1/2 bg-surface-dark border border-white/10 px-6 py-3 rounded-2xl shadow-2xl flex items-center gap-3 z-50 toast-enter">
      <div class="p-1.5 bg-green-500/20 rounded-full">
        <CheckCircle2 size={18} class="text-green-500" />
      </div>
      <span class="text-sm font-medium">{toastMessage}</span>
    </div>
  {/if}
</div>


