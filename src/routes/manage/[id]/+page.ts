// import { error } from '@sveltejs/kit';
import { invoke } from '@tauri-apps/api';

/** @type {import('./$types').PageLoad} */
export async function load({ params }: { params: any }) {
  return {
    target: JSON.parse(await invoke("get_target_json", { id: Number(params.id) }))
  };
  // throw error(404, 'Not found');
}