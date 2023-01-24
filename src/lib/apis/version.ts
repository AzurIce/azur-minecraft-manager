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
    let cachedVersions = new Array<any>();
    let uncachedIds = await Promise.all(ids.filter(async (id) => {
        const version = await versionsCache.get<any>(id);
        if (version !== null) cachedVersions.push(version);
        return version === null;
    }));

    if (uncachedIds.length > 0) {
        cachedVersions = cachedVersions.concat(await invoke<Array<any>>("get_versions", { ids: uncachedIds }));
    }

    return cachedVersions;
}

export async function getIsVersionDownloaded(target_dir: string, project_id: string, version_id: string) {
    return await invoke<boolean>("is_version_downloaded", { targetDir: target_dir, projectId: project_id, versionId: version_id });
}