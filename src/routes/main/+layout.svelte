<script lang="ts">
  import { goto } from "$app/navigation";
  import { page } from "$app/stores";
  import { watchModFiles } from "$lib/apis/mod_file";
  import { getVersionFromHash } from "$lib/apis/version";
  import { modFiles, store, targetDir } from "$lib/stores";
  import type { ModFile } from "$lib/typing/typing";
  import { listen, type UnlistenFn } from "@tauri-apps/api/event";
  import {
    Sidebar,
    SidebarGroup,
    SidebarItem,
    SidebarWrapper,
    Tooltip,
  } from "flowbite-svelte";
  import { onDestroy, onMount } from "svelte";

  const spanClass = "";
  const aClass =
    "w-8 h-8 flex items-center p-2 text-base font-normal text-gray-900 rounded-lg dark:text-white hover:bg-gray-100 dark:hover:bg-gray-700";
  const activeClass =
    "w-8 h-8 flex items-center p-2 text-base font-normal text-gray-900 bg-gray-200 dark:bg-gray-700 rounded-lg dark:text-white hover:bg-gray-100 dark:hover:bg-gray-700";

  $: activeUrl = $page.url.pathname;
  $: console.log($targetDir);

  $: if ($targetDir === "") goto("/");

  let unlisten1: UnlistenFn;
  let unlisten2: UnlistenFn;
  onMount(async () => {
    $modFiles = [];
    console.log("    starting modFiles watch...");
    unlisten1 = await listen<any>("mod_file_updated", (event) => {
      console.log("mod_file_updated: ", event.payload);

      for (let i = 0; i < $modFiles.length; i++) {
        if ($modFiles[i].sha1 == event.payload.sha1) {
          let remote_version = $modFiles[i].remote_version;
          $modFiles[i] = event.payload;
          $modFiles[i].remote_version = remote_version;
          $modFiles = $modFiles;
          return;
        }
      }
      let modFile: ModFile = event.payload;
      getVersionFromHash(modFile.sha1).then((res) => {
        console.log("got version: ", res)
        modFile.remote_version = res;
        $modFiles.push(modFile);
        $modFiles = $modFiles;
      });
    });
    unlisten2 = await listen<any>("mod_file_deleted", (event) => {
      console.log("mod_file_deleted");
      for (let i = 0; i < $modFiles.length; i++) {
        if ($modFiles[i].path == event.payload) {
          $modFiles.splice(i, 1);
          $modFiles = $modFiles;
          console.log("modFiles_deleted: ", $modFiles)
          return;
        }
      }
    });
    await watchModFiles($targetDir);
  });

  onDestroy(() => {
    unlisten1();
    unlisten2();
  });
</script>

<Sidebar asideClass="w-min">
  <SidebarWrapper
    divClass="h-full overflow-y-auto py-4 px-3 bg-white rounded dark:bg-gray-800 border-r"
  >
    <SidebarGroup ulClass="flex flex-col gap-2 items-center h-full">
      <SidebarItem
        href="/main"
        active={activeUrl === "/main"}
        {spanClass}
        {aClass}
        {activeClass}
      >
        <svelte:fragment slot="icon">
          <i class="ri-home-2-line" />
        </svelte:fragment>
      </SidebarItem>
      <SidebarItem
        href="/main/resources"
        active={activeUrl === "/main/resources"}
        {spanClass}
        {aClass}
        {activeClass}
      >
        <svelte:fragment slot="icon">
          <i class="ri-collage-line" />
        </svelte:fragment>
      </SidebarItem>
      <div class="flex-1" />
      <SidebarItem
        on:click={() => {
          store.set("targetDir", "");
        }}
        {spanClass}
        {aClass}
        {activeClass}
      >
        <svelte:fragment slot="icon">
          <i class="ri-logout-box-line" />
        </svelte:fragment>
      </SidebarItem>
      <Tooltip placement="right">退出当前目标</Tooltip>
      <SidebarItem
        href="/main/settings"
        active={activeUrl === "/main/settings"}
        {spanClass}
        {aClass}
        {activeClass}
      >
        <svelte:fragment slot="icon">
          <i class="ri-settings-line" />
        </svelte:fragment>
      </SidebarItem>
    </SidebarGroup>
  </SidebarWrapper>
</Sidebar>

<slot />
