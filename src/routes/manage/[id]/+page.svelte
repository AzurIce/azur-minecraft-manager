<script lang="ts">
  import type { Target } from "$lib/config";
  import { Breadcrumb, BreadcrumbItem } from "flowbite-svelte";
  import { Tabs, TabItem } from "flowbite-svelte";

  /** @type {import('./$types').PageData} */
  export let data: { target: Target };
  $: targetType = data.target.kind;
  $: targetPath = data.target.location;
</script>

<div class="flex flex-col bg-white p-4 gap-2">
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
