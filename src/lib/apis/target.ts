import type { Target } from "$lib/typing/typing";
import { invoke } from "@tauri-apps/api";

export async function getTargets() {
    return await invoke<Array<Target>>("get_targets");
}

export async function getTarget(index: number) {
    return await invoke<Target>("get_target", { index: index });
}

export async function addTarget(target: Target) {
    return await invoke<Target[]>("add_target", { target: target });
}

export async function delTarget(index: number) {
    return await invoke<Target[]>("del_target", { index: index });
}