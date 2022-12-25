<script lang="ts">
  import type { ModFile, Mod } from "$lib/typing/typing";
  import { Badge, P, Img } from "flowbite-svelte";
  import Toggle from "$lib/components/Toggle.svelte";

  enum BelongState {
    Unknown = "Unknown",
    Modrinth = "Modrinth",
  }

  export let file: {
    filename: string;
    sha1: string;
    enabled: boolean;
    belong_state: BelongState;
  };
  // let checked = true;

  import { afterUpdate, beforeUpdate, onDestroy, onMount } from "svelte";
  import { invoke } from "@tauri-apps/api";

  let version: any = {
    game_versions: [],
  };
  let project: any = {
    title: "",
    description: "",
    icon_url: "",
    client_side: "",
    server_side: "",
  };

  let prevState = file.belong_state;
  onMount(async () => {
    // console.log("-> CardModFile/onMount")
    getData();
    // console.log("<- CardModFile/onMount")
  });

  afterUpdate(async () => {
    // console.log("-> CardModFile/afterUpdate")
    if (prevState != file.belong_state) {
      // console.log("prevState != file.belong_state")
      getData();
      prevState = file.belong_state;
    }
    // console.log("<- CardModFile/afterUpdate")
  });

  async function getData() {
    // console.log("-> getData()")
    if (file.belong_state == BelongState.Modrinth) {
      // console.log("file.belong_state == BelongState.Modrinth")
      version = await invoke("get_version_from_hash", { hash: file.sha1 });
      // console.log(version);
      project = await invoke("get_project_from_hash", { hash: file.sha1 });
      // console.log(project);
    }
    // console.log("<- getData()")
  }

</script>

<div class="p-3 flex items-center {file.enabled ? "bg-white" : "bg-gray-50"} rounded-md shadow-sm border">
  <div class="flex flex-col gap-1 flex-1">
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
      <div class="flex flex-1 h-16 overflow-hidden mt-1 mb-2">
        <!-- <div class="avatar">
          <div class="w-16 rounded">
            <img alt="project_icon" src={project.icon_url} />
          </div>
        </div> -->
        <Img
          class="w-16 h-16 bg-gray-200 rounded text-center flex-none"
          src={project.icon_url}
        />
        <div
          class="flex flex-col flex-1 h-16 overflow-x-hidden overflow-y-auto"
        >
          <p class="ml-2 break-all text-sm">文件名: {file.filename}</p>
          <p class="ml-2 break-all text-sm">
            游戏版本: {version.game_versions}
          </p>
        </div>
      </div>
      <div class="flex gap-1">
        {#if project.client_side == "required"}
          <!-- <div class="badge">Client</div> -->
          <Badge color="red">Client</Badge>
        {:else if project.client_side == "optional"}
          <Badge color="red">Client</Badge>
        {:else}
          <Badge color="dark">Client: Unsupported</Badge>
        {/if}

        {#if project.server_side == "required"}
          <Badge color="red">Server: Required</Badge>
        {:else if project.server_side == "optional"}
          <Badge>Server: Optional</Badge>
        {:else}
          <Badge color="dark">Server: Unsupported</Badge>
        {/if}
      </div>
    {/if}
    <!-- <span class="text-gray-400">modId: {{ modId }}</span> -->
  </div>
  <div class="w-20 flex-none flex flex-col items-center">
    <!-- <Toggle checked={true}></Toggle> -->
    <Toggle checked={file.enabled}/>
    <!-- // TODO: create my own toggle -->
    <!-- <input
      id="enableToggle"
      type="checkbox"
      class="toggle"
      checked={file.enabled}
      
    /> -->
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
