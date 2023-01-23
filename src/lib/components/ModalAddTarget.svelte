<script lang="ts">
  import { invoke } from "@tauri-apps/api";
  import {
    Modal,
    Button,
    Tabs,
    TabItem,
    Fileupload,
    Label,
  } from "flowbite-svelte";
  import PathInput from "./PathInput.svelte";
  import { TargetType, type Target } from "../typing/typing";
  import { createEventDispatcher } from "svelte";

  const dispatch = createEventDispatcher();

  export let show: boolean;
  let targetType: TargetType = TargetType.Local;

  // For local
  let path: string = "";
  let pathValid: boolean = false;
  $: validLocal = pathValid;

  // For server
  let validServer: boolean = false;

  $: valid = targetType == TargetType.Local ? validLocal : validServer;

  async function addTarget() {
    let target: Target = {
      kind: targetType,
      location: path,
      mod_sources: []
    };

    invoke("add_target", {
      target: target,
    }).then((res) => {
      show = false;
      dispatch("added", { targets: res });
    });
    // TODO: 显示结果，成功便关闭Modal
  }
</script>

<Modal title="添加一个管理项" bind:open={show}>
  <Tabs style="pill">
    <TabItem
      open
      on:click={() => {
        targetType = TargetType.Local;
      }}
    >
      <span slot="title">本地 .minecraft/ 目录</span>
      <div class="mb-6">
        <PathInput
          bind:path
          bind:valid={pathValid}
          placeholder="输入你的 .minecraft/ 目录的路径"
          label=".minecraft 目录路径"
        />
      </div>
    </TabItem>
    <TabItem
      on:click={() => {
        targetType = TargetType.Server;
      }}
    >
      <span slot="title">使用 ACH 管理的服务器</span>
      <p class="text-sm text-gray-500 dark:text-gray-400">
        <b>没做呢:</b
        >外币外币外币外币外币外币外币外币外币外币外币外币外币外币外币.
      </p>
    </TabItem>
  </Tabs>
  <svelte:fragment slot="footer">
    {#if !valid}
      <Button disabled>添加</Button>
    {:else}
      <Button on:click={addTarget}>添加</Button>
    {/if}
    <Button color="alternative">取消</Button>
  </svelte:fragment>
</Modal>
