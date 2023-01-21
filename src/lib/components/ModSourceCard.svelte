<script lang="ts">
  import {
    Badge,
    P,
    Img,
    Dropdown,
    DropdownItem,
    Chevron,
    Button,
    ButtonGroup,
    Tooltip,
  } from "flowbite-svelte";

  export let modSource: any;
  export let curModFile: any;

  import { onMount } from "svelte";
  import { getProject } from "$lib/apis/project";
  import {
    getVersion,
    getVersions,
    getVersionsFromHash,
  } from "$lib/apis/version";
  import Toggle from "./Toggle.svelte";
  import ModVersionModal from "./ModVersionModal.svelte";

  let project: any = {
    title: "",
    description: "",
    icon_url: "",
    client_side: "",
    server_side: "",
    versions: [],
  };

  // $: console.log(curModFile);

  onMount(async () => {
    project = await getProject(modSource.project_id);
    // console.log(project);
  });

  let show: boolean = true;
</script>

<div
  class="p-3 flex items-center {true
    ? 'bg-white'
    : 'bg-gray-50'} rounded-md shadow-sm border"
>
  <div class="flex flex-col gap-1 flex-1">
    <!-- head -->
    <div class="flex items-center gap-2">
      <P>{project.title}</P>
      <Badge color="dark">Modrinth</Badge>
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
        <Button color="light" on:click={() => {show = true;}}>
          当前版本：{curModFile.remote_version.name}
        </Button>
      </div>

      
      <ButtonGroup>
        {#if true}
          <Button size="xs" color="green" on:click={() => {}}>
            <i class="ri-check-line" />
          </Button>
        {:else}
          <Button size="xs" color="red" on:click={() => {}}>
            <i class="ri-close-line" />
          </Button>
        {/if}
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
  </div>
  {#if project.versions.length > 0}
  <ModVersionModal bind:show={show} {project} {curModFile}/>
  {/if}
</div>
