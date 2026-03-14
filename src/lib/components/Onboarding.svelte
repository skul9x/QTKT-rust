<script lang="ts">
  import { ShieldCheck, Sparkles, Key, ArrowRight, Loader2 } from "lucide-svelte";
  import { invoke } from "@tauri-apps/api/core";

  interface Props {
    onComplete: () => void;
  }

  let { onComplete }: Props = $props();
  let apiKey = $state("");
  let isTesting = $state(false);
  let error = $state("");

  const API_KEY_REGEX = /AIzaSy[A-Za-z0-9_-]{33}/g;
  function extractKeys(text: string): string[] {
    const matches = text.match(API_KEY_REGEX);
    return matches ? [...new Set(matches)] : [];
  }
  let detectedKeysCount = $derived(extractKeys(apiKey).length);

  async function handleStart() {
    const keys = extractKeys(apiKey);
    if (keys.length === 0) {
      error = "Không tìm thấy API Key hợp lệ nào!";
      isTesting = false;
      return;
    }

    try {
      // Simulate testing key
      await new Promise(r => setTimeout(r, 1000));
      await invoke("save_api_key", { keys });
      onComplete();
    } catch (e) {
      error = "Không thể lưu API Key. Vui lòng thử lại.";
    } finally {
      isTesting = false;
    }
  }
</script>

<div class="fixed inset-0 z-50 bg-background flex items-center justify-center p-6">
  <!-- Grid background decoration -->
  <div class="absolute inset-0 bg-[linear-gradient(to_right,#1e293b_1px,transparent_1px),linear-gradient(to_bottom,#1e293b_1px,transparent_1px)] bg-[size:4rem_4rem] [mask-image:radial-gradient(ellipse_60%_50%_at_50%_0%,#000_70%,transparent_100%)] opacity-20"></div>

  <div class="max-w-xl w-full bg-surface rounded-[2.5rem] border border-white/5 shadow-2xl p-10 relative overflow-hidden animate-in zoom-in-95 duration-500">
    <div class="absolute top-0 right-0 p-8 opacity-10">
      <Sparkles class="w-24 h-24 text-primary" />
    </div>

    <div class="relative z-10 flex flex-col items-center text-center">
      <div class="w-16 h-16 bg-primary/10 rounded-2xl flex items-center justify-center mb-8 text-primary shadow-inner">
        <ShieldCheck class="w-10 h-10" />
      </div>

      <h1 class="text-3xl font-bold text-text mb-4 tracking-tight">Chào mừng Bạn!</h1>
      <p class="text-text-muted text-center max-w-md mb-8 leading-relaxed">
        Để hệ thống có thể bắt đầu hỗ trợ bạn soạn thảo Quy trình kỹ thuật, vui lòng nhập 
        <span class="text-primary font-semibold">Google Gemini API Key</span> của bạn bên dưới.
      </p>

      <div class="w-full space-y-4 mb-4">
        <div class="relative group">
          <textarea
            bind:value={apiKey}
            rows="4"
            placeholder="Dán mã API Key của bạn tại đây... (Có thể dán nhiều Key cùng lúc)"
            class="w-full bg-background border border-white/10 rounded-2xl px-6 py-4 focus:ring-2 focus:ring-primary outline-none transition-all text-sm font-mono shadow-inner resize-none"
          ></textarea>
          {#if detectedKeysCount > 0}
            <div class="absolute right-4 bottom-4 bg-primary/20 text-primary text-[10px] px-2 py-0.5 rounded-full font-bold animate-in fade-in zoom-in-50">
              Phát hiện {detectedKeysCount} Key
            </div>
          {/if}
        </div>
        {#if error}
          <p class="text-danger text-sm font-medium">{error}</p>
        {/if}
      </div>

      <div class="w-full flex flex-col gap-4">
        <button
          onclick={handleStart}
          disabled={!apiKey || isTesting}
          class="w-full bg-primary hover:bg-primary-dark disabled:opacity-50 text-white py-4 rounded-2xl font-bold flex items-center justify-center gap-2 transition-all shadow-lg active:scale-95 group"
        >
          {#if isTesting}
            <Loader2 class="w-5 h-5 animate-spin" />
            Đang thẩm định...
          {:else}
            Bắt đầu ngay
            <ArrowRight class="w-5 h-5 group-hover:translate-x-1 transition-transform" />
          {/if}
        </button>
        
        <p class="text-xs text-text-muted">
          Thông tin được bảo mật cực kỳ an toàn bởi hệ thống Keychain của máy tính.
        </p>
      </div>
    </div>
  </div>
</div>
