<script lang="ts">
  import { Button } from "flowbite-svelte";

  import { invoke } from "@tauri-apps/api";
</script>

<div>
  <Button
    on:click={() => {
      invoke("load");
    }}
  >
    Reload
  </Button>
</div>
