<script>
  import Greet from "$lib/Greet.svelte";
  import ModalAddTarget from "$lib/ModalAddTarget.svelte";
  import { Button } from "flowbite-svelte";


  let modalAddTarget = true;
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
  {modalAddTarget}
  <Button on:click={()=> {modalAddTarget = true;}}>Add</Button>
  <ModalAddTarget bind:show={modalAddTarget}/>
</div>

<style>
.logo.azurcraft:hover {
  filter: drop-shadow(0 0 2em #3381ff);
}


</style>
