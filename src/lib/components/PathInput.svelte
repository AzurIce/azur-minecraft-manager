<script lang="ts">
    import { invoke } from "@tauri-apps/api";
    import { open } from "@tauri-apps/api/dialog";
    import { Input, Label, Helper, Button } from "flowbite-svelte";

    export let valid = true;
    export let path: string;
    export let label: string;
    export let placeholder: string;

    let inputValue: string = "";
    $: helper = valid || inputValue.length == 0 ? "" : "路径不存在";

    async function onChooseDir() {
        const select = await open({
            defaultPath: path.toString(),
            directory: true,
            multiple: false,
        });
        console.log(select);
        if (select == null) return;

        valid = true;
        inputValue = <string>select;
        path = <string>select;
    }
    async function onInputPathChanged() {
        valid = await invoke("is_path_exist", { path: inputValue });
        if (valid) {
            path = inputValue;
        }
    }
</script>

<div class="m-2">
    <Label for="input" color={ valid || inputValue.length == 0 ? undefined : "red"} class='block mb-2'>{label}</Label>
    <div class="flex">
        <Input
            id="input"
            class="bg-white"
            color={ valid || inputValue.length == 0 ? undefined : "red"}
            bind:value={inputValue}
            on:input={onInputPathChanged}
            placeholder={placeholder}
        />
        <!-- <input type="file" id="fileInput" @change="modDirInput" hidden webkitdirectory/> -->
        <Button color="light" on:click={onChooseDir}><i class="ri-folder-line" /></Button>
    </div>
    <Helper color={ valid || inputValue.length == 0 ? undefined : "red"}>{helper}</Helper>
</div>
