<script lang="ts">
  import type { ModFile, Target } from "$lib/typing";
  import { Breadcrumb, BreadcrumbItem, Card } from "flowbite-svelte";
  import { Tabs, TabItem, Button } from "flowbite-svelte";
  import { invoke } from "@tauri-apps/api";
  import { join } from "@tauri-apps/api/path";

  import CardModFile from "$lib/components/CardModFile.svelte";

  /** @type {import('./$types').PageData} */
  export let data: { target: Target };
  $: targetType = data.target.kind;
  $: targetPath = data.target.location;

  let fileList: Array<any> = [];

  import { onMount } from "svelte";
  onMount(async () => {
    // invoke<Array<ModFile>>("get_mod_file_list", {
    //   path: await join(targetPath, "mods"),
    // }).then((res) => {
    //   modFileList = res;
    //   modFileList.forEach(async (modFile) => {
    //     invoke<ModFile>("get_belonged_mod_file", {
    //       path: await join(targetPath, "mods", modFile.filename),
    //     }).then((res) => {
    //       modFile.belong_mod = res.belong_mod;
    //       console.log(modFileList);
    //     });
    //   });
    // });
    fileList = await invoke<Array<any>>("get_mod_file_list", {path: await join(targetPath, "mods")})
  });

  async function test() {
    invoke("get_belonged_mod_file", {
      path: await join(targetPath, "mods", "appleskin-fabric-mc1.19-2.4.1.jar"),
    }).then((res) => {
      console.log(res);
      console.log(res as ModFile);
    });
    // console.log(await join(targetPath, "mods"));
    // invoke("get_mod_file_list", {path: await join(targetPath, "mods")}).then((res) => {
    //   console.log(res);
    // })
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
      {#each fileList as file (file.sha1)}
        <CardModFile {file} {targetPath} />
      {/each}
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
