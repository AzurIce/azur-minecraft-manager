import { invoke } from "@tauri-apps/api";

export async function getVersionsFromHashes(hashes: Array<string>) {
    return new Map(Object.entries(await invoke<any>("get_versions_from_hashes", { hashes: hashes })));
}

export async function getVersionsFromHash(hash: string) {
    return await invoke<any>("get_version_from_hash", { hash: hash });
}

export async function getVersion(id: string) {
    return await invoke<any>("get_version", { id: id });
}

export async function getVersions(ids: Array<string>) {
    return await invoke<any[]>("get_versions", { ids: ids });
}

export async function getIsVersionDownloaded(id: string) {
    return await invoke<boolean>("is_version_downloaded", {id: id});
}