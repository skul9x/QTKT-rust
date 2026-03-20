<style>
  :global(.animate-loading-bar) {
    animation: loading 1.5s infinite ease-in-out;
    width: 30%;
  }

  @keyframes loading {
    0% { transform: translateX(-100%); }
    100% { transform: translateX(333%); }
  }

  /* Custom scrollbar global */
  :global(::-webkit-scrollbar) {
    width: 8px;
  }
  :global(::-webkit-scrollbar-track) {
    background: transparent;
  }
  :global(::-webkit-scrollbar-thumb) {
    background: rgba(255, 255, 255, 0.05);
    border-radius: 10px;
  }
</style>

<script lang="ts">
  import { invoke } from "@tauri-apps/api/core";
  import { onMount } from "svelte";
  import Sidebar from "$lib/components/Sidebar.svelte";
  import Generator from "$lib/components/screens/Generator.svelte";
  import Merger from "$lib/components/screens/Merger.svelte";
  import Settings from "$lib/components/screens/Settings.svelte";
  import Guide from "$lib/components/screens/Guide.svelte";
  import TimKiemByt from "$lib/components/screens/TimKiemBYT.svelte";
  import Onboarding from "$lib/components/Onboarding.svelte";

  let activeTab: "generator" | "merger" | "tim_kiem" | "settings" | "guide" = $state("generator");
  let isLoading = $state(true);
  let isGenerating = $state(false);

  async function checkApiKey() {
    try {
      await invoke("get_api_key");
    } catch (e) {
      console.log("No API key found yet.");
    } finally {
      isLoading = false;
    }
  }

  onMount(checkApiKey);
</script>

{#if isLoading}
  <div class="h-screen w-full flex items-center justify-center bg-background">
     <div class="w-16 h-1 bg-white/10 overflow-hidden rounded-full">
        <div class="h-full bg-primary animate-loading-bar"></div>
     </div>
  </div>
{:else}
  <div class="flex h-screen bg-background text-text overflow-hidden font-inter select-none">
    <Sidebar {activeTab} onTabChange={(tab: "generator" | "merger" | "tim_kiem" | "settings" | "guide") => activeTab = tab} />

    <main class="flex-1 relative overflow-y-auto bg-[radial-gradient(circle_at_top_right,rgba(14,165,233,0.05),transparent_40%)]">
      <div class="h-full">
        <div class={activeTab === 'generator' ? 'h-full' : 'hidden'}>
          <Generator bind:isGenerating isActive={activeTab === 'generator'} />
        </div>
        <div class={activeTab === 'merger' ? 'h-full' : 'hidden'}>
          <Merger />
        </div>
        <div class={activeTab === 'tim_kiem' ? 'h-full' : 'hidden'}>
          <TimKiemByt />
        </div>
        <div class={activeTab === 'settings' ? 'h-full' : 'hidden'}>
          <Settings {isGenerating} />
        </div>
        <div class={activeTab === 'guide' ? 'h-full' : 'hidden'}>
          <Guide />
        </div>
      </div>
    </main>
  </div>
{/if}
