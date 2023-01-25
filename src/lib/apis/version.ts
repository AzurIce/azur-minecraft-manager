import { hash2versionCache, versionsCache } from "$lib/stores";
import { invoke } from "@tauri-apps/api";

export async function getVersionsFromHashes(hashes: Array<string>) {
    return new Map(Object.entries(await invoke<any>("get_versions_from_hashes", { hashes: hashes })));
}

export async function updateVersionFromHash(hash: string) {
    const version = await invoke<any>("get_version_from_hash", { hash: hash });
    hash2versionCache.set(hash, version);
    return version;
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
    console.log("version.ts/getVersions: ", ids.length);
    let cachedVersions = new Array<any>();
    let uncachedIds = new Array<string>();
    await Promise.all(ids.map(async (id) => {
        const version = await versionsCache.get<any>(id);
        // console.log(version);
        if (version === null) uncachedIds.push(id);
        if (version !== null) cachedVersions.push(version);
    }));

    console.log("uncachedIds: ", uncachedIds);
    if (uncachedIds.length > 0) {
        let uncachedVersions = await updateVersions(uncachedIds);
        // console.log("uncachedIds: ", uncachedIds);
        // console.log("getVersions: ", cachedVersions, uncachedVersions);
        console.log(cachedVersions, uncachedVersions)
        return [...cachedVersions, ...uncachedVersions];
    }

    return cachedVersions;
}

export async function updateVersions(ids: Array<string>) {
    let versions = await invoke<Array<any>>("get_versions", { ids: ids });

    versions.forEach((version) => {
        versionsCache.set(version.id, version);
    })

    return versions;
}

export async function getIsVersionDownloaded(target_dir: string, project_id: string, version_id: string) {
    return await invoke<boolean>("is_version_downloaded", { targetDir: target_dir, projectId: project_id, versionId: version_id });
}