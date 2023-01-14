<script lang="ts">
  import { TargetType, type ModFile, type Target } from "$lib/typing/typing";
  import { Breadcrumb, BreadcrumbItem, Card } from "flowbite-svelte";
  import { Tabs, TabItem, Button, Spinner } from "flowbite-svelte";
  import { invoke } from "@tauri-apps/api";
  import { join } from "@tauri-apps/api/path";
  import { onDestroy, onMount } from "svelte";
  import { emit, listen, type UnlistenFn } from "@tauri-apps/api/event";
  import { getTarget } from "$lib/apis/target";
  import { updateModFiles, watchModFiles } from "$lib/apis/mod_file";

  import LocalModFileCard from "$lib/components/LocalModFileCard.svelte";

  import { targetIndex } from "$lib/stores";
    import ModSourceCard from "$lib/components/ModSourceCard.svelte";
  $: index = $targetIndex;
  let target: Target = {
    kind: TargetType.Local,
    location: "",
    mod_sources: new Array(),
  };
  
  let modFiles: Array<ModFile> = [];
  $: localModFiles = modFiles.filter((modFile) => !modFile.remote_version)
  $: remoteModFiles = modFiles.filter((modFile) => modFile.remote_version)

  let unlisten: UnlistenFn;
  let loading = true;
  onMount(async () => {
    console.log("-> onMount");

    console.log("    getting target...");
    target = await getTarget(index);

    console.log("    starting modFiles watch...");
    unlisten = await listen<Array<any>>("mod_files_updated", (event) => {
      console.log("mod_files_updated");
      modFiles = event.payload;
    });
    await watchModFiles(target.location);

    console.log("    updating mod files...");
    modFiles = await updateModFiles(target.location);
    loading = false;


    console.log("<- onMount");
  });

  onDestroy(() => {
    console.log("-> onDestroy");
    unlisten();
    emit("unwatch_mod_files");
  });


  enum Tab {
    RemoteMod = "远端 Mod",
    LocalMod = "本地 Mod",
    // Settings = "Settings",
    // Users = "Users",
    // Dashboard = "Dashboard",
  }

  let selectedTab = Tab.RemoteMod;
  $: tabs = Object.values(Tab);
</script>

<div class="flex flex-col flex-1 bg-white p-4 gap-2">
  <Breadcrumb aria-label="Default breadcrumb example">
    <BreadcrumbItem href="/" home>主页</BreadcrumbItem>
    <BreadcrumbItem>管理</BreadcrumbItem>
  </Breadcrumb>
  <!-- {targetPath} -->
  <Tabs contentClass="">
    {#each tabs as tab, i}
      {#if i == 0}
        <TabItem
          open
          title={tab}
          on:click={() => {
            selectedTab = tab;
          }}
        />
      {:else}
        <TabItem
          title={tab}
          on:click={() => {
            selectedTab = tab;
          }}
        />
      {/if}
    {/each}
  </Tabs>

  <!-- Tab Contents -->
  {#if selectedTab == Tab.RemoteMod}
    <div class="w-full bg-white border rounded-md p-2">
      <!-- {#if updatingData}
        <Button disabled>
          <Spinner class="mr-3" size="4" />更新 Modrinth 信息
        </Button>
      {:else} -->
      <!-- <Button on:click={() => {}}>更新 Modrinth 信息</Button> -->
      <!-- {/if} -->
    </div>
    <div class="w-full overflow-y-auto flex flex-col gap-1">
      <!-- {#if loading}Loading...{:else} -->
      {#each target.mod_sources as mod_source}
        <ModSourceCard modSource={mod_source}/>
        <!-- <LocalModFileCard {file} /> -->
      {/each}
      <!-- {/if} -->
    </div>
  {:else if selectedTab == Tab.LocalMod}
    <div class="w-full bg-white border rounded-md p-2">
      <!-- {#if updatingData}
        <Button disabled>
          <Spinner class="mr-3" size="4" />更新 Modrinth 信息
        </Button>
      {:else} -->
      <Button on:click={() => {}}>更新 Modrinth 信息</Button>
      <!-- {/if} -->
    </div>
    <div class="w-full overflow-y-auto flex flex-col gap-1">
      {#if loading}Loading...{:else}
      {#each localModFiles as file (file.sha1)}
        <LocalModFileCard {file} />
      {/each}
      {/if}
    </div>
    <!-- {:else if selectedTab == Tab.Settings}
    {selectedTab}
  {:else if selectedTab == Tab.Users}
    {selectedTab}
  {:else if selectedTab == Tab.Dashboard}
    {selectedTab} -->
  {/if}
</div>
