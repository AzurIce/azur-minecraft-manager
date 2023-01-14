import { invoke } from "@tauri-apps/api";
import type { ModFile } from "../typing/typing";

export async function getLocalModFiles() {
    return await invoke<ModFile[]>("get_local_mod_files");
}

export async function getVersionsFromHashes(hashes: Array<string>) {
    return new Map(Object.entries(await invoke<any>("get_versions_from_hashes", { hashes: hashes })));
}

export async function getVersionsFromHash(hash: string) {
    return await invoke<any>("get_version_from_hash", {hash: hash});
}

