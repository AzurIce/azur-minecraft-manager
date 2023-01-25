<script lang="ts">
  import { Modal } from "flowbite-svelte";
  import { getVersions } from "$lib/apis/version";
  import ModVersionCard from "./ModVersionCard.svelte";
    import { onMount } from "svelte";

  export let show: boolean;
  export let versionIds: Array<string>;
  export let curModFile: any;

  let versions = new Array<any>();

  onMount(() => {
    getVersions(versionIds).then((res) => {
      versions = res;
      versions.sort((a, b) => a.date_published > b.date_published ? -1 : 1);
      versions = versions;
    });
  });
</script>

<Modal title="管理版本" bind:open={show} class="flex w-5/6">
  {#each versions as version}
    <ModVersionCard {version} {curModFile} />
  {/each}
</Modal>
