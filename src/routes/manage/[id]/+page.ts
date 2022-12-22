// import { error } from '@sveltejs/kit';
import { invoke } from '@tauri-apps/api';

/** @type {import('./$types').PageLoad} */
export async function load({ params }: { params: any }) {
  return {
    index: Number(params.id),
    target: await invoke("get_target", { index: Number(params.id) })
  };
  // throw error(404, 'Not found');
}