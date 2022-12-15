<script lang="ts">
    import { Modal, Button, Tabs, TabItem } from "flowbite-svelte";
    import { open } from "@tauri-apps/api/dialog";

    enum TargetType {
        Local,
        Server,
    }

    export let show: boolean;
    let targetType: TargetType = TargetType.Local;
    // import { invoke } from "@tauri-apps/api/tauri";

    // let name = "";
    // let greetMsg = "";

    // async function greet() {
    //   // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
    //   // greetMsg = await invoke("greet", { name });
    //   greetMsg = await invoke("get_target_list");
    // }
    async function addTarget() {}
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
        </TabItem>
        <!-- <TabItem on:click={() => { targetType = TargetType.Server; }}>
          <span slot="title">使用ACH管理的服务器</span>
          <p class="text-sm text-gray-500 dark:text-gray-400"><b>Dashboard:</b> Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua.</p>
        </TabItem> -->
    </Tabs>
    <svelte:fragment slot="footer">
        <Button
            on:click={() => {
                addTarget().then(() => {
                    // TODO: 显示结果，成功便关闭Modal
                });
            }}
        >
            添加
        </Button>
        <Button color="alternative">取消</Button>
    </svelte:fragment>
</Modal>
