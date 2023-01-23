<script lang="ts">
  import { onMount } from "svelte";
  import { Button, Hr, P } from "flowbite-svelte";
  import { getVersion } from "@tauri-apps/api/app";
  import { store, targetDir } from "$lib/stores";
  import { open } from "@tauri-apps/api/dialog";
  import { goto } from "$app/navigation";

  let appVersion: string;

  $: if ($targetDir !== "") {
    goto(`/main`);
  }

  onMount(async () => {
    $targetDir = (await store.get<string>("targetDir")) || "";
    appVersion = await getVersion();
  });

  async function onChooseDir() {
    const select = await open({
      defaultPath: $targetDir,
      directory: true,
      multiple: false,
    });
    console.log(select);
    if (select == null) return;

    store.set("targetDir", select);
  }
</script>

<div class="flex-1 flex flex-col items-center gap-2">
  <!-- <p class="font-extralight">ExtraLight | The quick brown fox ...</p>
  <p class="font-light">Light | The quick brown fox ...</p>
  <p class="font-normal">Normal | The quick brown fox ...</p>
  <p class="font-semibold">Semibold | The quick brown fox ...</p>
  <p class="font-bold">Bold | The quick brown fox ...</p> -->

  <!-- brand -->
  <div class="flex flex-col m-4">
    <img src="/AzurCraft.svg" class="-z-10 h-32" alt="AzurCraft Logo" />
    <P size="xl" align="center" space="widest" weight="semibold">AMCM</P>
    <P size="sm" align="center">版本 {appVersion}</P>
  </div>

  <Hr width="w-3/4" />

  <div class="flex flex-col w-full m-4">
    <div class="flex w-full justify-between pl-8 pr-8">
      <div class="flex flex-col gap-1">
        <P weight="semibold">管理本地目标</P>
        <P size="sm">
          将一个包含 .minecraft/ 的文件夹作为管理目标在 AMCM 中打开。
        </P>
      </div>

      <Button on:click={onChooseDir}>打开</Button>
    </div>
  </div>
</div>
