<script lang="ts">
  import { TargetType, type ModFile, type Target } from "$lib/typing/typing";
  import { Breadcrumb, BreadcrumbItem, Card } from "flowbite-svelte";
  import { Tabs, TabItem, Button, Spinner } from "flowbite-svelte";
  import { invoke } from "@tauri-apps/api";
  import { join } from "@tauri-apps/api/path";

  import CardModFile from "$lib/components/CardModFile.svelte";

  import { targetIndex } from "$lib/stores";
  $: index = $targetIndex;
  let target: Target = {
    kind: TargetType.Local,
    location: "",
  };
  $: targetType = target.kind;
  $: targetPath = target.location;

  let fileList: Array<any> = [];
  let updatingData = false;

  import { emit, listen, type UnlistenFn } from "@tauri-apps/api/event";

  let unlisten: UnlistenFn;
  let loading = true;
  import { onDestroy, onMount } from "svelte";
  import { afterNavigate, beforeNavigate } from "$app/navigation";
  onMount(async () => {
    console.log("-> onMount");
    target = await invoke("get_target", { index: Number(index) });
  });

  onDestroy(() => {
    console.log("-> onDestroy");
  });

  // Leaving
  beforeNavigate((navigation) => {
    console.log("-> beforeNavigate");
    unlisten();
    emit("unwatch_mod_files");
  });
  // Entering
  afterNavigate(async (navigation) => {
    console.log("-> afterNavigate");
    await getModFileList();
    loading = false;
    unlisten = await listen<Array<any>>("mod_files_updated", (event) => {
      console.log("mod_files_updated");
      fileList = event.payload;
    });

    invoke("target_watch_mod_files", { index: index });
  });

  async function getModFileList() {
    // fileList = await invoke<Array<any>>("get_mod_file_list", {
    //   path: await join(targetPath, "mods"),
    // });
    fileList = await invoke<Array<any>>("get_mod_files");
  }

  async function updateData() {
    updatingData = true;
    invoke("update_data_from_hashes", {
      hashes: fileList.map((file) => file.sha1),
    })
      .then(async (res) => {
        console.log("Updated data from hashes");
        await invoke("update_mod_files", {
          dir: await join(targetPath, "mods"),
        });
        await getModFileList();
        updatingData = false;
      })
      .catch((err) => {
        console.log(err);
      });
  }

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
      {#if updatingData}
        <Button disabled>
          <Spinner class="mr-3" size="4" />更新 Modrinth 信息
        </Button>
      {:else}
        <Button on:click={updateData}>更新 Modrinth 信息</Button>
      {/if}
    </div>
    <div class="w-full overflow-y-auto flex flex-col gap-1">
      {#if loading}Loading...{:else}
        {#each fileList as file (file.sha1)}
          <CardModFile {file} />
        {/each}
      {/if}
    </div>
  {:else if selectedTab == Tab.Settings}
    {selectedTab}
  {:else if selectedTab == Tab.Users}
    {selectedTab}
  {:else if selectedTab == Tab.Dashboard}
    {selectedTab}
  {/if}
</div>
