<script lang="ts">
  import { getIsVersionDownloaded } from "$lib/apis/version";
  import { targetDir } from "$lib/stores";
  import type { ModFile } from "$lib/typing/typing";
  import { invoke } from "@tauri-apps/api";
  import { join } from "@tauri-apps/api/path";
  import { Badge, P, Spinner } from "flowbite-svelte";
  import { onMount } from "svelte";
  import moment from "moment";

  export let version: any;
  export let curModFile: ModFile;
  let curVersion: any;

  enum State {
    NotDownloaded,
    Downloaded,
    Downloading,
    Choosed,
  }

  $: curVersion = curModFile ? curModFile.remote_version : undefined;

  $: state = downloaded
    ? curVersion && version.id == curVersion.id
      ? State.Choosed
      : State.Downloaded
    : State.NotDownloaded;

  let downloaded: boolean = false;
  onMount(() => {
    getIsVersionDownloaded($targetDir, version.project_id, version.id).then((res) => {
      downloaded = res;
    });
  });
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
  on:click={async () => {
    if (state == State.NotDownloaded) {
      state = State.Downloading;
      invoke("download_version", {
        targetDir: $targetDir,
        projectId: version.project_id,
        versionId: version.id,
        fileUrl: version.files[0].url
      }).then((res) => {
        console.log(res);
        state = State.Downloaded;
        downloaded = true;
      });
    } else if (state == State.Downloaded) {
      if (curModFile) {
        await invoke("delete_file", {
          path: curModFile.path,
        });
        console.log("deleted file");
      }
      await invoke("copy_file", {
        src: await join(
          $targetDir,
          ".amcm",
          "storage",
          version.project_id,
          `${version.id}.jar`
        ),
        dst: await join(
          $targetDir,
          ".minecraft",
          "mods",
          `${version.files[0].filename}.tmp`
        ),
      });
      await invoke("rename_file", {
        src: await join(
          $targetDir,
          ".minecraft",
          "mods",
          `${version.files[0].filename}.tmp`
        ),
        dst: await join(
          $targetDir,
          ".minecraft",
          "mods",
          `${version.files[0].filename}`
        ),
      });
      console.log("copied file");
    }
    // Download
  }}
  on:keydown={() => {}}
>

  <!-- head -->
  <div class="flex flex-1 items-center gap-2">
    {version.name}
    <Badge
      color={state == State.Choosed
        ? "blue"
        : state == State.Downloaded
        ? "blue"
        : state == State.NotDownloaded
        ? "dark"
        : "blue"}>{version.id}</Badge
    >
    <div class="flex-1" />

    {#if state == State.Choosed}
      <i class="ri-check-line text-green-600" />
    {:else if state == State.Downloaded}
      <i class="ri-check-line text-gray-300" />
    {:else if state == State.Downloading}
      <Spinner size={4}/>
    {:else if state == State.NotDownloaded}
      <i class="ri-download-line" />
    {/if}
  </div>

  <!-- mid -->
  <div class="flex items-center gap-2 text-sm font-light">
    发布时间：{moment(version.date_published).format("YYYY-M-D H:mm:ss")}
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
