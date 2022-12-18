<script lang="ts">
  import type { Target } from "$lib/config";
  import { Breadcrumb, BreadcrumbItem } from "flowbite-svelte";
  import { Tabs, TabItem, Button } from "flowbite-svelte";
  import { invoke } from "@tauri-apps/api";
  import { join } from "@tauri-apps/api/path";

  /** @type {import('./$types').PageData} */
  export let data: { target: Target };
  $: targetType = data.target.kind;
  $: targetPath = data.target.location;

  let modFileList = [];

  import { onMount } from "svelte";
  onMount(() => {
    invoke("get_mod_filename_list", {path: targetPath}).then((res) => {
      modFileList = (res as Array<string>).map((v) => {
        
      })
    })
  })
</script>

<div class="flex flex-col bg-white p-4 gap-2">
  <Button
    on:click={async () => {
      // invoke("get_mod_from_hash", {sha1:"27AB78BB9DBA64010AA63D1F0B06108132569EAC"}).then((res) => {
      //   console.log(res);
      // })
      console.log(await join(targetPath, "mods"));
      invoke("get_mod_file_list", {
        path: await join(targetPath, "mods"),
      }).then((res) => {
        console.log(res);
      });
    }}
  />
  <Breadcrumb aria-label="Default breadcrumb example">
    <BreadcrumbItem href="/" home>主页</BreadcrumbItem>
    <BreadcrumbItem>管理</BreadcrumbItem>
  </Breadcrumb>
  <Tabs>
    <TabItem open title="Mod">
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
