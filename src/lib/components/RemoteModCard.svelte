<script lang="ts">
  import { Badge, P, Img, Button, ButtonGroup, Spinner } from "flowbite-svelte";

  export let project_id: string;

  import { getProject } from "$lib/apis/project";
  import ModVersionModal from "./ModVersionModal.svelte";
  import { modFiles } from "$lib/stores";
  import { disableModFile, enableModFile } from "$lib/apis/mod_file";

  $: curModFile = $modFiles.find(
    (modFile) =>
      modFile.remote_version != undefined &&
      modFile.remote_version.project_id == project_id
  );

  // $: console.log("curModFile: ", curModFile);

  function onToggleEnable() {
    if (!curModFile) return;

    if (curModFile.enabled) {
      disableModFile(curModFile);
    } else {
      enableModFile(curModFile);
    }
  }

  let show: boolean = false;
</script>

<div
  class="p-3 flex flex-col gap-1 flex-1 {true
    ? 'bg-white'
    : 'bg-gray-50'} rounded-md shadow-sm border"
>
{#await getProject(project_id)}
  <div class="flex w-full justify-center">
    <Spinner size={10} />
  </div>
{:then project}
  <!-- head -->
  <div class="flex items-center gap-2">
    <P>{project.title}</P>
    <Badge color="dark">{project.id}</Badge>
  </div>

  <!-- mid -->
  <div class="flex flex-1 h-16 items-center overflow-hidden mt-1 mb-2 gap-2">
    <Img
      class="w-16 h-16 bg-gray-200 rounded text-center flex-none"
      src={project.icon_url}
    />

    <div
      class="flex flex-col flex-1 h-16 overflow-x-hidden overflow-y-auto justify-center"
    >
      <Button
        color="light"
        on:click={() => {
          show = true;
        }}
      >
        当前版本：{#if curModFile != undefined}{curModFile.remote_version
            .name}{:else}无{/if}
      </Button>
    </div>

    <ButtonGroup>
      <Button
        size="xs"
        color={curModFile && curModFile.enabled ? "green" : "red"}
        disabled={curModFile == undefined}
        on:click={() => {
          onToggleEnable();
        }}
      >
        <i
          class={curModFile && curModFile.enabled
            ? "ri-check-line"
            : "ri-close-line"}
        />
      </Button>
      <Button size="xs"><i class="ri-more-2-line" /></Button>
    </ButtonGroup>
  </div>

  <!-- foot -->
  <div class="flex gap-1">
    {#if project.client_side == "required"}
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
  <ModVersionModal bind:show versionIds={project.versions} {curModFile} />
  {/await}
</div>
