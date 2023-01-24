import { writable } from 'svelte/store';
import type { ModFile } from './typing/typing';

import { Store } from 'tauri-plugin-store-api';

// Settings
export const store = new Store("amcm-settings.dat");
export const targetDir = writable<string>("");
store.onChange((k, v) => {
    if (k === "targetDir") targetDir.set(v as string);
})

// Cache
export const projectsCache = new Store("amcm-projects-cache.dat");
export const versionsCache = new Store("amcm-versions-cache.dat");
export const hash2versionCache = new Store("amcm-hash2version-cache.dat");

// Runtime data
export const modFiles = writable(new Array<ModFile>);