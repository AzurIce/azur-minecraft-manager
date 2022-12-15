<script>
  import "../app.css";

  import { page } from "$app/stores";
  import {
    Sidebar,
    SidebarGroup,
    SidebarItem,
    SidebarWrapper,
    SidebarBrand,
  } from "flowbite-svelte";
  // let spanClass = "flex-1 ml-3 whitespace-nowrap";
  $: activeUrl = $page.url.pathname;

  let site = {
    name: "AzurMCManager",
    href: "/",
    img: "/AzurCraft.svg",
  };
</script>

<div class="flex flex-row h-full items-stretch">
  <Sidebar asideClass="w-64 shrink-0">
    <SidebarWrapper
      divClass="h-full overflow-y-auto py-4 px-3 bg-white rounded dark:bg-gray-800 border-r"
    >
      <SidebarGroup>
        <SidebarBrand {site} />
        <SidebarItem label="主页" href="/" active={activeUrl === "/"}>
          <svelte:fragment slot="icon">
            <i class="ri-home-2-line" />
          </svelte:fragment>
        </SidebarItem>
        <SidebarItem
          label="我的资源"
          href="/resources"
          active={activeUrl === "/resources"}
        >
          <svelte:fragment slot="icon">
            <i class="ri-collage-line" />
          </svelte:fragment>
        </SidebarItem>
        <SidebarItem
          label="设置"
          href="/settings"
          active={activeUrl === "/settings"}
        >
          <svelte:fragment slot="icon">
            <i class="ri-settings-line" />
          </svelte:fragment>
        </SidebarItem>
      </SidebarGroup>
    </SidebarWrapper>
  </Sidebar>

  <slot />
</div>
