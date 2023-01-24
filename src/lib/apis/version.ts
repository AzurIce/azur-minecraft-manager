import { hash2versionCache, versionsCache } from "$lib/stores";
import { invoke } from "@tauri-apps/api";

export async function getVersionsFromHashes(hashes: Array<string>) {
    return new Map(Object.entries(await invoke<any>("get_versions_from_hashes", { hashes: hashes })));
}

export async function updateVersionFromHash(hash: string) {
    const version = await invoke<any>("get_version_from_hash", { hash: hash });
    hash2versionCache.set(hash, version);
}

export async function getVersionFromHash(hash: string) {
    const version = await hash2versionCache.get<any>(hash);
    if (version === null) return updateVersionFromHash(hash);
    return version;
}

export async function updateVersion(id: string) {
    const version = await invoke<any>("get_version", { id: id });
    versionsCache.set(id, version);
    return version;
}

export async function getVersion(id: string) {
    const version = await versionsCache.get<any>(id);
    if (version === null) return updateVersion(id);
    return version;
}

export async function getVersions(ids: Array<string>) {
    return await Promise.all(ids.map(async (id) => await getVersion(id)));
    // return await invoke<any[]>("get_versions", { ids: ids });
}

export async function getIsVersionDownloaded(target_dir: string, version_id: string) {
    return await invoke<boolean>("is_version_downloaded", { targetDir: target_dir, versionId: version_id });
}