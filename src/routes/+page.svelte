<script lang="ts">
  import { onMount } from "svelte";
  import ModalAddTarget from "$lib/components/ModalAddTarget.svelte";
  import CardTarget from "$lib/components/CardTarget.svelte";
  import { Button } from "flowbite-svelte";
  import type { Target } from "$lib/typing/typing";
  import Toggle from "$lib/components/Toggle.svelte";
    import { getTargets } from "$lib/apis/target";

  let targetList: Target[] = [];
  let modalAddTarget = false;

  onMount(async () => {
    targetList = await getTargets();
  });

  function onUpdateTargets(event: CustomEvent) {
    targetList = event.detail.targets;
  }
</script>

<div class="flex-1 flex flex-col items-center">
  <img
    src="/AzurCraft.svg"
    class="-z-10 absolute h-full m-auto logo azurcraft"
    alt="AzurCraft Logo"
  />

  {#if targetList.length > 0}
    <div
      class="w-full flex flex-wrap flex-1 flex-row gap-4 items-center justify-start content-start bg-white bg-opacity-50 backdrop-blur p-4"
    >
      {#each targetList as target, i (target)}
        <CardTarget {target} id={i} on:deleted={onUpdateTargets} />
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
  <Toggle/>
</div>
<ModalAddTarget bind:show={modalAddTarget} on:added={onUpdateTargets} />

<!-- </div> -->
<style>
  .logo.azurcraft:hover {
    filter: drop-shadow(0 0 2em #3381ff);
  }
</style>
