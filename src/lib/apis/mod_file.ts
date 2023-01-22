import type { ModFile } from "$lib/typing/typing";
import { invoke } from "@tauri-apps/api";
import { join } from "@tauri-apps/api/path";

export async function enableModFile(hash: string) {
    return await invoke("enable_mod_file", { hash: hash });
}

export async function disableModFile(hash: string) {
    return await invoke("disable_mod_file", { hash: hash });
}

export async function removeModFile(hash: string) {
    return await invoke("remove_mod_file", { hash: hash });
}

export async function updateModFiles(targetPath: string) {
    return await invoke<ModFile[]>("update_mod_files", { dir: targetPath });
}

export async function watchModFiles(targetPath: string) {
    return await invoke("watch_mod_files", {dir: targetPath});
}

