<script>
  import Greet from "$lib/Greet.svelte";
</script>

<h1>Welcome to Tauri!</h1>

<div class="row">
  <!-- <a href="https://vitejs.dev" target="_blank">
    <img src="/vite.svg" class="logo vite" alt="Vite Logo" />
  </a> -->
  <img src="/AzurCraft.svg" class="logo azurcraft" alt="AzurCraft Logo" />
  <!-- <a href="https://kit.svelte.dev" target="_blank">
    <img src="/svelte.svg" class="logo svelte" alt="Svelte Logo" />
  </a> -->
</div>

<p>欢迎使用 Azur Minecraft Manager，请先添加一个管理项</p>

<div class="row">
  <Greet />
</div>

<style>
  .logo.vite:hover {
    filter: drop-shadow(0 0 2em #747bff);
  }

  .logo.svelte:hover {
    filter: drop-shadow(0 0 2em #ff3e00);
  }
</style>
