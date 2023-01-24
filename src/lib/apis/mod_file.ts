import type { ModFile } from "$lib/typing/typing";
import { invoke } from "@tauri-apps/api";
import { basename } from "@tauri-apps/api/path";

export async function enableModFile(modfile: ModFile) {
    // console.log(modfile.path);
    // console.log();
    return await invoke("rename_file", {
        src: modfile.path,
        dst: modfile.path.slice(0, modfile.path.lastIndexOf('.'))
    });
    // return await invoke("enable_mod_file", { hash: hash });
}

export async function disableModFile(modfile: ModFile) {
    return await invoke("rename_file", {
        src: modfile.path,
        dst: `${modfile.path}.disabled`
    });
    // return await invoke("disable_mod_file", { hash: hash });
}

export async function removeModFile(hash: string) {
    return await invoke("remove_mod_file", { hash: hash });
}

export async function updateModFiles(targetPath: string) {
    return await invoke<ModFile[]>("update_mod_files", { dir: targetPath });
}

export async function watchModFiles(targetPath: string) {
    return await invoke("watch_mod_files", { dir: targetPath });
}

