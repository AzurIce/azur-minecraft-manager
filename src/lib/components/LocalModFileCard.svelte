<script lang="ts">
  import type { ModFile } from "$lib/typing/typing";
  import {
    Badge,
    ButtonGroup,
    Button,
    P,
    Spinner,
    Tooltip,
  } from "flowbite-svelte";

  export let file: ModFile;
  $: version = file.remote_version;
  // let loadingVersion = true;

  import { createEventDispatcher, onMount } from "svelte";
  import { disableModFile, enableModFile } from "$lib/apis/mod_file";
  import { fade } from "svelte/transition";

  const dispatch = createEventDispatcher();

  function onToggleEnable() {

    if (file.enabled) {
      disableModFile(file.sha1);
    } else {
      enableModFile(file.sha1);
    }
  }
</script>

<div
  class="p-2 flex items-center 
    {file.enabled
    ? 'bg-white'
    : 'bg-gray-50'} rounded-md shadow-sm border gap-2"
  in:fade
>
  <P>{file.filename}</P>
  {#if version}
    <button
      class="inline-flex items-center px-2 py-1 rounded
      text-xs bg-blue-100 text-blue-800 dark:bg-blue-900 dark:text-blue-300
      hover:bg-blue-200 hover:text-blue-900 dark:hover:bg-blue-800 dark:hover:text-blue-300"
      on:click={() => {
        dispatch("addToRemoteMod", {
          version_id: version.id,
          project_id: version.project_id,
        });
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
  <ButtonGroup>
    <Button
      size="xs"
      color={file.enabled ? "green" : "red"}
      on:click={() => {
        onToggleEnable();
      }}
    >
      <i class={file.enabled ? "ri-check-line" : "ri-close-line"} />
    </Button>
    <Button size="xs"><i class="ri-more-2-line" /></Button>
  </ButtonGroup>
  <!-- <button on:click={() => {}} /> -->
  <!-- </div> -->
</div>
