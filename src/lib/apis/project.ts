import { invoke } from "@tauri-apps/api";

export async function getProject(id: string) {
    return await invoke<any>("get_project", {id: id});
}