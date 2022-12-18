<script lang="ts">
  import type { ModFile, Mod } from "$lib/typing";
  import { Card } from "flowbite-svelte";

  export let filename: string;
  export let targetPath: string;
  let modFile: any = {};
  import { onMount } from "svelte";
  import { invoke } from "@tauri-apps/api";
  import { join } from "@tauri-apps/api/path";

  let loading: boolean = true;

  onMount(async () => {
    invoke<ModFile>("get_belonged_mod_file", {
      path: await join(targetPath, "mods", filename),
    }).then((res) => {
      loading = false;
      modFile = res;
      console.log(modFile);
    });
  });
</script>

<div
  class="bg-white rounded-md shadow-sm border w-full p-2 m-1 flex items-center"
>
  <div class="flex flex-col gap-1">
    <div class="flex items-center">
      <h2>{filename}</h2>
      <!-- <span class="badge ml-2" v-if="isFabricMod">Fabric</span> -->
      <!-- <span class="badge badge-error ml-2" v-if="isBadJsonSyntax">BadJsonSyntax</span> -->
    </div>
    <div class="flex">
      <div class="w-16 h-16 bg-gray-200 rounded text-center">Icon</div>
      <div class="flex flex-col">
        {#if !loading}
        <span class="ml-2">filename: {modFile.filename}</span>
        <span class="ml-2">modDesc: { modFile.belong_mod.desc }</span>
        {/if}
        <!-- <span class="ml-2">gameVersion: {{ gameVersion }}</span> -->
      </div>
    </div>
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
