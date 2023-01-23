<script lang="ts">
  import { invoke } from "@tauri-apps/api";
  import {
    Modal,
    Button,
    Tabs,
    TabItem,
    Fileupload,
    Label,
  } from "flowbite-svelte";
  import PathInput from "./PathInput.svelte";
  import { TargetType, type Target } from "../typing/typing";
  import { createEventDispatcher } from "svelte";
  import { onMount } from "svelte";
  import {
    getVersion,
    getVersions,
    getVersionFromHash,
  } from "$lib/apis/version";
  import ModVersionCard from "./ModVersionCard.svelte";

  const dispatch = createEventDispatcher();

  export let show: boolean;
  export let project: any;
  export let curModFile: any;

  let versions = new Array<any>();
  // let curVersion: any = { sha1: "" };

  $: {
    console.log("project changed, updating versions");
    getVersions(project.versions).then((res) => {
      versions = res;
      // console.log(res);
    });
  }
</script>

<Modal title="管理版本" bind:open={show} class="flex">
  <!-- <div class="flex flex-col h-80 overflow-y-auto"> -->
  <!-- {#if curModFile} -->
    {#each versions as version}
      <ModVersionCard {version} {curModFile} />
    {/each}
  <!-- {/if} -->
  <!-- </div> -->
</Modal>
