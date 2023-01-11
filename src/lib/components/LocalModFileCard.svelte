<script lang="ts">
  import type { ModFile } from "$lib/typing/typing";
  import {
    Badge,
    ButtonGroup,
    Button,
    P,
    Spinner,
    Dropdown,
    DropdownItem,
    Chevron,
    Tooltip,
  } from "flowbite-svelte";
  import Toggle from "$lib/components/Toggle.svelte";

  export let file: ModFile;
  let version: any;
  let loadingVersion = true;
  // let project: any;
  // let checked = true;

  import { afterUpdate, beforeUpdate, onDestroy, onMount } from "svelte";
  import { invoke } from "@tauri-apps/api";
  import { disableModFile, enableModFile } from "$lib/api";
    import { targetIndex } from "$lib/stores";

  onMount(() => {
    invoke("get_version_from_hash", { hash: file.sha1 })
      .then((res) => {
        version = res;
      })
      .catch((err) => {
        // console.error(err);
      })
      .finally(() => {
        loadingVersion = false;
      });
  });
</script>

<div
  class="p-2 flex items-center 
    {file.enabled
    ? 'bg-white'
    : 'bg-gray-50'} rounded-md shadow-sm border gap-2"
>
  <!-- <div class="flex items-center justify-center gap-2"> -->
  <P>{file.filename}</P>

  {#if loadingVersion}
    <span
      class="inline-flex items-center px-2 py-1 rounded
      text-xs bg-gray-100 text-gray-800 dark:bg-gray-900 dark:text-gray-300 gap-2"
    >
      Loading<Spinner size="3" />
    </span>
  {:else if version}
    <!-- <Badge
      on:click={() => {
        console.log("clicked");
      }}>Modrinth | +</Badge
    > -->
    <button
      class="inline-flex items-center px-2 py-1 rounded
      text-xs bg-blue-100 text-blue-800 dark:bg-blue-900 dark:text-blue-300
      hover:bg-blue-200 hover:text-blue-900 dark:hover:bg-blue-800 dark:hover:text-blue-300"
      on:click={() => {
        // console.log(version.project_id);
        invoke("add_target_mod_source", {target_id: $targetIndex, project_id: version.project_id})
      }}
    >
      Modrinth<i class="ri-add-line" />
    </button>
    <Tooltip
      style="custom"
      defaultClass=""
      class="p-2 text-xs bg-white text-gray-900
      border border-gray-200 rounded-lg shadow-sm"
    >
      添加到远端 Mod
    </Tooltip>
  {:else}
    <span
      class="inline-flex items-center px-2 py-1 rounded
      text-xs bg-gray-100 text-gray-800 dark:bg-gray-900 dark:text-gray-300"
    >
      Unknown
    </span>
  {/if}
  <div class="flex-1" />
  <!-- <Toggle
    checked={file.enabled}
    on:click={() => {
      if (file.enabled) {
        disableModFile(file.sha1);
      } else {
        enableModFile(file.sha1);
      }
    }}
  /> -->
  <ButtonGroup>
    {#if file.enabled}
      <Button
        size="xs"
        color="green"
        on:click={() => disableModFile(file.sha1)}
      >
        <i class="ri-check-line" />
      </Button>
    {:else}
      <Button size="xs" color="red" on:click={() => enableModFile(file.sha1)}>
        <i class="ri-close-line" />
      </Button>
    {/if}
    <Button size="xs"><i class="ri-more-2-line" /></Button>
    <!-- <Dropdown>
      <DropdownItem>Add</DropdownItem>
      <DropdownItem>Settings</DropdownItem>
      <DropdownItem>Earnings</DropdownItem>
      <DropdownItem>Sign out</DropdownItem>
    </Dropdown> -->
  </ButtonGroup>
  <button on:click={() => {}} />
  <!-- </div> -->
</div>
