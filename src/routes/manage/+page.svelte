<script lang="ts">
  import { TargetType, type ModFile, type Target } from "$lib/typing/typing";
  import { Breadcrumb, BreadcrumbItem, Card } from "flowbite-svelte";
  import { Tabs, TabItem, Button, Spinner } from "flowbite-svelte";
  import { invoke } from "@tauri-apps/api";
  import { join } from "@tauri-apps/api/path";

  import LocalModFileCard from "$lib/components/LocalModFileCard.svelte";

  import { targetIndex } from "$lib/stores";
  $: index = $targetIndex;
  let target: Target = {
    kind: TargetType.Local,
    location: "",
    mod_sources: [],
  };
  $: targetType = target.kind;
  $: targetPath = target.location;


  import { getLocalModFiles, getVersionsFromHashes } from "$lib/api";
  // let updatingData = false;
  import { onDestroy, onMount } from "svelte";
  import { afterNavigate, beforeNavigate } from "$app/navigation";

  let localModFiles: Array<ModFile> = [];
  let unlisten: UnlistenFn;
  onMount(async () => {
    console.log("-> onMount");

    console.log("    getting target...");
    target = await invoke("get_target", { index: index });

    console.log("    getting localModFiles...");
    localModFiles = await invoke("update_local_mod_files", { dir: await join(target.location, "mods") });

    console.log("    starting localModFiles watch...");
    listen<Array<any>>("mod_files_updated", (event) => {
      console.log("mod_files_updated");
      // console.log(event.payload);
      localModFiles = event.payload;
    }).then((res) => {
      unlisten = res;
      invoke("target_watch_mod_files", { index: index });
    });

    console.log("<- onMount");
  });

  onDestroy(() => {
    console.log("-> onDestroy");
    unlisten();
    emit("unwatch_mod_files");
  });

  let loading = true;
  import { emit, listen, type UnlistenFn } from "@tauri-apps/api/event";
  // Leaving
  beforeNavigate((navigation) => {
    // console.log("-> beforeNavigate");
  });
  // Entering
  afterNavigate(async (navigation) => {
    // console.log("-> afterNavigate");
    // await getLocalModFiles();
    // loading = false;
    // unlisten = await listen<Array<any>>("mod_files_updated", (event) => {
    //   console.log("mod_files_updated");
    //   localModFiles = event.payload;
    // });

    // invoke("target_watch_mod_files", { index: index });
  });

  

  // function getVersionFromHash(hash: string): any {
  //   return versions.get(hash);
  // }

  function test() {
    invoke("test");
    // invoke("get_belonged_mod_file", {
    //   path: await join(targetPath, "mods", "appleskin-fabric-mc1.19-2.4.1.jar"),
    // }).then((res) => {
    //   console.log(res);
    //   console.log(res as ModFile);
    // });
  }

  enum Tab {
    Mod = "Mod",
    Settings = "Settings",
    Users = "Users",
    Dashboard = "Dashboard",
  }

  let selectedTab = Tab.Mod;
  $: tabs = Object.values(Tab);

  // console.log(Object.keys(Tab));
  // let tab = 1;
</script>

<div class="flex flex-col flex-1 bg-white p-4 gap-2">
  <Button on:click={test} />
  <Breadcrumb aria-label="Default breadcrumb example">
    <BreadcrumbItem href="/" home>主页</BreadcrumbItem>
    <BreadcrumbItem>管理</BreadcrumbItem>
  </Breadcrumb>
  {targetPath}
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
  {#if selectedTab == Tab.Mod}
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
      <!-- {#if loading}Loading...{:else} -->
        {#each localModFiles as file (file.sha1)}
          <LocalModFileCard {file} />
        {/each}
      <!-- {/if} -->
    </div>
  {:else if selectedTab == Tab.Settings}
    {selectedTab}
  {:else if selectedTab == Tab.Users}
    {selectedTab}
  {:else if selectedTab == Tab.Dashboard}
    {selectedTab}
  {/if}
</div>
