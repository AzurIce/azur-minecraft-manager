<script lang="ts">
  import type { ModFile, Mod } from "$lib/typing";
  import { Badge, P, Img } from "flowbite-svelte";

  enum BelongState {
    Unknown = "Unknown",
    Modrinth = "Modrinth",
  }

  export let file: { filename: string; sha1: string; belong_state: BelongState };
  export let targetPath: string;
  import { onMount } from "svelte";
  import { invoke } from "@tauri-apps/api";
  import { join } from "@tauri-apps/api/path";

  let version: any;
  let project: any = {
    title: "",
    description: "",
    icon_url: "",
  };

  onMount(async () => {
    if (file.belong_state == BelongState.Modrinth) {
      version = await invoke("get_version_from_hash", {hash: file.sha1});
      project = await invoke("get_project_from_hash", {hash: file.sha1});
    }
  });
</script>

<div
  class="bg-white rounded-md shadow-sm border w-full p-3 m-1 flex items-center"
>
  <div class="flex flex-col gap-1">
    <div class="flex items-center gap-2">
      {#if file.belong_state == BelongState.Modrinth}
      <P>{project.title}</P>
      <Badge>Modrinth</Badge>
      {:else}
      <P>{file.filename}</P>
      <Badge color="dark">Unknown</Badge>
      {/if}
      <!-- <span class="badge ml-2" v-if="isFabricMod">Fabric</span> -->
      <!-- <span class="badge badge-error ml-2" v-if="isBadJsonSyntax">BadJsonSyntax</span> -->
    </div>
    {#if file.belong_state == BelongState.Modrinth}
      <div class="flex h-16">
        <Img class="w-16 h-16 bg-gray-200 rounded text-center flex-none" src={project.icon_url}/>
        <div class="flex flex-col flex-1">
            <P class="ml-2" size="sm">文件名: {file.filename}</P>
            <P class="ml-2" size="sm">描述: {project.description}</P>
          <!-- <span class="ml-2">gameVersion: {{ gameVersion }}</span> -->
        </div>
        <div class="w-20">

        </div>
      </div>
    {/if}
    <!-- <span class="text-gray-400">modId: {{ modId }}</span> -->
  </div>

  <!-- <div class="flex-1" /> -->

  <!-- <div class="dropdown dropdown-end m-1">
    <div class="indicator">
      <span class="indicator-item badge badge-success" />
      <div class="tooltip" data-tip="New Version">
        <label tabindex="0" class="btn">Version</label>
      </div>
    </div>
    <ul
      tabindex="0"
      class="dropdown-content menu p-2 shadow bg-base-100 rounded-box w-52"
    >
      <li><a>Item 1</a></li>
      <li><a>Item 2</a></li>
    </ul>
  </div>

  <input type="checkbox" class="checkbox" v-model="checked" /> -->
</div>
