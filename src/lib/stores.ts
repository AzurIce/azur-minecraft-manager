import { readable, writable } from 'svelte/store';
import type { ModFile } from './typing/typing';

import { Store } from 'tauri-plugin-store-api';
export const store = new Store("amcm-settings.dat");

store.onChange((k, v) => {
    if (k == "targetDir") targetDir.set(v as string);
})

export const targetDir = writable<string>("");

export const targetIndex = writable<number>(0);

export const modFiles = writable(new Array<ModFile>);