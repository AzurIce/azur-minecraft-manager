<script lang="ts">
  import type { ModFile, Target } from "$lib/typing/typing";
  import { Breadcrumb, BreadcrumbItem, Card } from "flowbite-svelte";
  import { Tabs, TabItem, Button } from "flowbite-svelte";
  import { invoke } from "@tauri-apps/api";
  import { join } from "@tauri-apps/api/path";

  import CardModFile from "$lib/components/CardModFile.svelte";

  /** @type {import('./$types').PageData} */
  export let data: { index: number; target: Target };
  $: targetType = data.target.kind;
  $: targetPath = data.target.location;
  $: index = data.index;

  let fileList: Array<any> = [];

  import { emit, listen, type UnlistenFn } from "@tauri-apps/api/event";

  let unlisten: UnlistenFn;
  let loading = true;
  import { onDestroy, onMount } from "svelte";
  import { afterNavigate, beforeNavigate } from "$app/navigation";
  onMount(async () => {});

  onDestroy(() => {});

  // Leaving
  beforeNavigate((navigation) => {
    console.log("beforeNavigate");
    unlisten();
    emit("unwatch_mod_files");
  });
  // Entering
  afterNavigate(async (navigation) => {
    console.log("afterNavigate");
    await getModFileList();
    loading = false;
    unlisten = await listen<Array<any>>("mod_files_updated", (event) => {
      console.log("mod_files_updated");
      fileList = event.payload;
    });

    invoke("watch_mod_files", { dir: await join(targetPath, "mods") });
  });

  async function getModFileList() {
    // fileList = await invoke<Array<any>>("get_mod_file_list", {
    //   path: await join(targetPath, "mods"),
    // });
    fileList = await invoke<Array<any>>("get_mod_files");
  }

  async function updateData() {
    invoke("update_data_from_hashes", {
      hashes: fileList.map((file) => file.sha1),
    })
      .then(async (res) => {
        console.log("Updated data from hashes");
        await invoke("update_mod_files", {dir: await join(targetPath, "mods")});
        getModFileList();
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
</script>

<div class="flex flex-col flex-1 bg-white p-4 gap-2 overflow-y-auto">
  <Button on:click={test} />
  <Breadcrumb aria-label="Default breadcrumb example">
    <BreadcrumbItem href="/" home>主页</BreadcrumbItem>
    <BreadcrumbItem>管理</BreadcrumbItem>
  </Breadcrumb>
  <Tabs>
    <TabItem open title="Mod">
      <div class="m-1 mb-2">
        <Button on:click={updateData}>更新 Modrinth 信息</Button>
      </div>
      {#if loading}Loading...{:else}
        {#each fileList as file (file.sha1)}
          <CardModFile {file} {targetPath} />
        {/each}
      {/if}
      <p class="text-sm text-gray-500 dark:text-gray-400">
        <b>Profile:</b> Lorem ipsum dolor sit amet, consectetur adipiscing elit,
        sed do eiusmod tempor incididunt ut labore et dolore magna aliqua.
      </p>
    </TabItem>
    <TabItem ttile="Settings">
      <p class="text-sm text-gray-500 dark:text-gray-400">
        <b>Settings:</b> Lorem ipsum dolor sit amet, consectetur adipiscing elit,
        sed do eiusmod tempor incididunt ut labore et dolore magna aliqua.
      </p>
    </TabItem>
    <TabItem title="Users">
      <p class="text-sm text-gray-500 dark:text-gray-400">
        <b>Users:</b> Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed
        do eiusmod tempor incididunt ut labore et dolore magna aliqua.
      </p>
    </TabItem>
    <TabItem title="Dashboard">
      <p class="text-sm text-gray-500 dark:text-gray-400">
        <b>Dashboard:</b> Lorem ipsum dolor sit amet, consectetur adipiscing elit,
        sed do eiusmod tempor incididunt ut labore et dolore magna aliqua.
      </p>
    </TabItem>
    <TabItem disabled>
      <span slot="title" class="text-gray-400 dark:text-gray-500">Disabled</span
      >
      <p class="text-sm text-gray-500 dark:text-gray-400">
        <b>Disabled:</b> Lorem ipsum dolor sit amet, consectetur adipiscing elit,
        sed do eiusmod tempor incididunt ut labore et dolore magna aliqua.
      </p>
    </TabItem>
  </Tabs>
</div>
