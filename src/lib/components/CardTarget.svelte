<script lang="ts">
  import { Card, Button } from "flowbite-svelte";
  import { type Target, TargetType } from "$lib/typing";
  import { goto } from "$app/navigation";

  export let target: Target;
  export let id: number;
  $: targetType = target.kind;
  $: targetPath = target.location;

  $: icon = targetType == TargetType.Local ? "ri-save-line" : "ri-cloud-line";
</script>

<Card>
  <div
    class="w-30 flex gap-2 mb-2 text-xl font-bold tracking-tight text-gray-900 dark:text-white"
  >
    <i class={icon} />
    <span>{targetType == TargetType.Local ? "本地" : "服务器"}</span>
  </div>

  <div
    class="w-30 mb-3 font-normal text-gray-700 dark:text-gray-400 leading-tight"
  >
    {targetPath}
  </div>
  <Button class="w-fit" on:click={() => {goto(`/manage/${id}`)}}>
    管理
  </Button>
</Card>
