<script lang="ts">
  import { FileText, Files, Settings, Wand2, BookOpen } from "lucide-svelte";

  interface Props {
    activeTab: "generator" | "merger" | "settings" | "guide";
    onTabChange: (tab: "generator" | "merger" | "settings" | "guide") => void;
  }

  let { activeTab, onTabChange }: Props = $props();

  const menuItems = [
    { id: "generator", label: "Tạo QTKT", icon: Wand2 },
    { id: "merger", label: "Gộp File", icon: Files },
    { id: "settings", label: "Cài Đặt", icon: Settings },
    { id: "guide", label: "HDSD", icon: BookOpen },
  ];
</script>

<aside class="w-64 bg-surface border-r border-white/5 flex flex-col h-screen">
  <div class="p-6">
    <div class="flex items-center gap-3 text-primary font-bold text-xl uppercase tracking-wider">
      <FileText class="w-8 h-8" />
      <span>QTKT Maker</span>
    </div>
  </div>

  <nav class="flex-1 px-4 py-4 space-y-2">
    {#each menuItems as item}
      <button
        onclick={() => onTabChange(item.id as any)}
        class="w-full flex items-center gap-3 px-4 py-3 rounded-xl transition-all duration-200 group
          {activeTab === item.id 
            ? 'bg-primary text-white shadow-lg shadow-primary/20' 
            : 'text-text-muted hover:bg-white/5 hover:text-text'}"
      >
        <item.icon class="w-5 h-5 {activeTab === item.id ? '' : 'group-hover:scale-110 transition-transform'}" />
        <span class="font-medium">{item.label}</span>
      </button>
    {/each}
  </nav>

  <div class="p-4 border-t border-white/5 space-y-4">
    <div class="flex flex-col gap-0.5 px-2 opacity-40 hover:opacity-100 transition-opacity">
        <p class="text-[10px] font-bold tracking-widest text-primary uppercase">Version 1.0.1</p>
        <p class="text-[10px] text-text-muted leading-relaxed">© 2026 Nguyễn Duy Trường</p>
    </div>
  </div>
</aside>
