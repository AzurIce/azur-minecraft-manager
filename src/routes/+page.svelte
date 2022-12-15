<script lang="ts">
  import { onMount } from "svelte";
  import { invoke } from "@tauri-apps/api";
  import ModalAddTarget from "$lib/components/ModalAddTarget.svelte";
  import CardTarget from "$lib/components/CardTarget.svelte";
  import { Button } from "flowbite-svelte";
  import type { TargetType, Target } from "$lib/config";

  let targetList: Target[] = [];
  let modalAddTarget = false;

  onMount(async () => {
    const res = await invoke<string>("get_target_list_json");
    targetList = JSON.parse(res);
    console.log(targetList);
  });
</script>

<!-- <div class="container mx-auto flex flex-col"> -->
<div class="flex-1 flex flex-col items-center">
  <img
    src="/AzurCraft.svg"
    class="-z-10 absolute h-full m-auto logo azurcraft"
    alt="AzurCraft Logo"
  />

  {#if targetList.length > 0}
    <div
      class="w-full flex flex-wrap flex-1 flex-row gap-4 items-center justify-center bg-white bg-opacity-50 backdrop-blur p-4"
    >
      {#each targetList as target (target.location)}
        <CardTarget targetType={target.kind} targetPath={target.location} />
        <!-- <CardTarget targetType={TargetType.Local} targetPath="adasda"/>
        <CardTarget targetType={TargetType.Server} targetPath="adasda"/>
        <CardTarget targetType={TargetType.Server} targetPath="adasda"/>
        <CardTarget targetType={TargetType.Local} targetPath="adasda"/> -->
      {/each}
      <Button
        on:click={() => {
          modalAddTarget = true;
        }}
      >
        + 添加管理项
      </Button>
    </div>
  {:else}
    <div
      class="w-full flex flex-1 flex-col items-center justify-center bg-white bg-opacity-50 backdrop-blur"
    >
      <p class="my-4">
        欢迎使用 Azur Minecraft Manager，首先请先添加一个管理项
      </p>
      <Button
        on:click={() => {
          modalAddTarget = true;
        }}
      >
        + 添加管理项
      </Button>
    </div>
  {/if}
</div>
<ModalAddTarget bind:show={modalAddTarget} />

<!-- </div> -->
<style>
  .logo.azurcraft:hover {
    filter: drop-shadow(0 0 2em #3381ff);
  }
</style>
