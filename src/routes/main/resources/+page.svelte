<script lang="ts">
  import type { Target } from "$lib/typing/typing";
  import { Breadcrumb, BreadcrumbItem } from "flowbite-svelte";
  import { Tabs, TabItem, Button } from "flowbite-svelte";
</script>

<div class="flex flex-col flex-1 bg-white p-4">
  <Tabs contentClass="h-full p-4 bg-gray-50 rounded-lg dark:bg-gray-800 mt-4">
    <TabItem open title="Mod">
      <div class="flex">
        <div class="flex flex-col basis-1/2 border-r m-2">Mod组</div>
        <div class="flex flex-col basis-1/2 m-2">
          <div class="flex">
            <div class="flex-1">Mod</div>
            <Button pill={true} class="!p-2 fill-white"><svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" width="24" height="24"><path fill="none" d="M0 0h24v24H0z"/><path d="M11 11V5h2v6h6v2h-6v6h-2v-6H5v-2z"/></svg></Button>
          </div>
        </div>
      </div>
    </TabItem>
  </Tabs>
</div>
