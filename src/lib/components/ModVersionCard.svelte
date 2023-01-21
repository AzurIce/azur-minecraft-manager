<script lang="ts">
  import { getIsVersionDownloaded, getVersion, getVersionsFromHash } from "$lib/apis/version";
    import { targetIndex } from "$lib/stores";
  import { invoke } from "@tauri-apps/api";
  import { Badge, P, Spinner } from "flowbite-svelte";

  export let version: any;
  export let curModFile: any;
  let curVersion: any;

  enum State {
    NotDownloaded,
    Downloaded,
    Choosed,
  }

  $: {
    console.log("curModFile changed, updating curVersion");
    getVersionsFromHash(curModFile.sha1).then((res) => {
      curVersion = res;
    });
  }

  $: state = downloaded
    ? version.id == curVersion.id
      ? State.Choosed
      : State.Downloaded
    : State.NotDownloaded;

  let downloaded: boolean;
  $: {
    console.log("version or curVersion changed, updating downloaded");
    getIsVersionDownloaded(version.id).then((res) => {
      downloaded = res;
    });
  }
</script>

<div
  class="flex flex-col border rounded p-2 gap-1
  {state == State.Choosed
    ? 'border-sky-400 bg-sky-200 text-gray-700'
    : state == State.Downloaded
    ? 'text-gray-700 transition hover:border-gray-400 hover:cursor-pointer'
    : state == State.NotDownloaded
    ? 'transition hover:border-gray-400 hover:cursor-pointer text-gray-300'
    : ''}"
  on:click={() => {
    if (state == State.NotDownloaded) {
      invoke("download_version", { id: version.id }).then((res) => {
        console.log(res);
      });
    } else if (state == State.Downloaded) {
      invoke("delete_mod_file", {hash: curModFile.sha1}).then((res) => {
        console.log("delete_mod_file:", res);
        invoke("copy_version_to_target", {versionId: version.id, targetId: $targetIndex}).then((res) => {
          console.log("copy_version_to_target", res);
        })
      })
    }
    // Download
  }}
  on:keydown={() => {}}
>
  <!-- head -->
  <!-- <div class="flex items-center gap-2"> -->

  <!-- </div> -->

  <!-- mid -->
  <div class="flex flex-1 justify-between items-center">
    {version.name}

    {#if state == State.Choosed}
      <i class="ri-check-line text-green-600" />
    {:else if state == State.Downloaded}
      <i class="ri-check-line text-gray-300" />
    {:else if state == State.NotDownloaded}
      <i class="ri-download-line" />
    {/if}
  </div>

  <!-- foot -->
  <div class="flex flex-1 items-center gap-2">
    {#each version.game_versions as game_version}
      <Badge
        color={state == State.Choosed
          ? "blue"
          : state == State.Downloaded
          ? "blue"
          : state == State.NotDownloaded
          ? "dark"
          : "blue"}>{game_version}</Badge
      >
    {/each}
  </div>
</div>
