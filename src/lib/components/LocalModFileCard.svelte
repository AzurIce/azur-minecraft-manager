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

  import { afterUpdate, beforeUpdate, onDestroy, onMount } from "svelte";
  import { invoke } from "@tauri-apps/api";
  import { disableModFile, enableModFile } from "$lib/apis/mod_file";
    import { targetIndex } from "$lib/stores";

  onMount(() => {});
</script>

<div
  class="p-2 flex items-center 
    {file.enabled
    ? 'bg-white'
    : 'bg-gray-50'} rounded-md shadow-sm border gap-2"
>
  <!-- <div class="flex items-center justify-center gap-2"> -->
  <P>{file.filename}</P>

  <!-- {#if loadingVersion}
    <span
      class="inline-flex items-center px-2 py-1 rounded
      text-xs bg-gray-100 text-gray-800 dark:bg-gray-900 dark:text-gray-300 gap-2"
    >
      Loading<Spinner size="3" />
    </span>
  {:else  -->
  {#if version}
    <button
      class="inline-flex items-center px-2 py-1 rounded
      text-xs bg-blue-100 text-blue-800 dark:bg-blue-900 dark:text-blue-300
      hover:bg-blue-200 hover:text-blue-900 dark:hover:bg-blue-800 dark:hover:text-blue-300"
      on:click={() => {
        // console.log(version.project_id);
        invoke("add_mod_source_from_local_mod_file", {projectId: version.project_id, versionId: version.id, modFileHash: file.sha1}).then((res) => {
          console.log("added mod source from local file", res)
          invoke("add_mod_source_to_target", {targetId: $targetIndex, projectId: version.project_id}).then((res) => {
            console.log(res);
          }).catch((err) => {
            console.log(err);
          })
        }).catch((err) => {
          console.log(err);
        })
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
  </ButtonGroup>
  <button on:click={() => {}} />
  <!-- </div> -->
</div>
