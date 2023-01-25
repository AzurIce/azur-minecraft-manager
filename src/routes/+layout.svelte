<script>
  import "../app.css";

  import { page } from "$app/stores";
  $: activeUrl = $page.url.pathname;

  let site = {
    name: "AzurMCManager",
    href: "/",
    img: "/AzurCraft.svg",
  };
</script>

<div class="absolute inset-0 flex flex-row h-full items-stretch overflow-hidden font-sans">
  <slot />
</div>
