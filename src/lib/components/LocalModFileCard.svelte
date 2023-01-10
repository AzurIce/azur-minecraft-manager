<script lang="ts">
  import type { ModFile } from "$lib/typing/typing";
  import { Badge, P } from "flowbite-svelte";
  import Toggle from "$lib/components/Toggle.svelte";

  export let file: ModFile;
  let version: any;
  // let project: any;
  // let checked = true;

  import { afterUpdate, beforeUpdate, onDestroy, onMount } from "svelte";
  import { invoke } from "@tauri-apps/api";

  onMount(() => {
    invoke("get_version_from_hash", {hash: file.sha1 }).then((res) => {
      version = res;
    }).catch((err) => {
      // console.error(err);
    })
    // console.log(file.belongState)
    // console.log(version);
  });
</script>

<div
  class="p-3 flex items-center 
    {file.enabled ? 'bg-white' : 'bg-gray-50'} rounded-md shadow-sm border gap-2"
>
  <!-- <div class="flex items-center justify-center gap-2"> -->
  <P>{file.filename}</P>
  {#if version}
    <Badge>Modrinth</Badge>
  {:else}
    <Badge color="dark">Unknown</Badge>
  {/if}
  <div class="flex-1" />
  <Toggle
    checked={file.enabled}
    on:click={() => {
      invoke("set_mod_file_enabled", {
        hash: file.sha1,
        enabled: !file.enabled,
      }).catch((err) => {
        console.error(err);
      });
      console.log("clicked");
    }}
  />
  <!-- </div> -->
</div>
