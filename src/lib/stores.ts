import { readable, writable } from 'svelte/store';
import type { ModFile } from './typing/typing';

export const targetIndex = writable<number>(0);

export const modFileList = writable(new Array<ModFile>);