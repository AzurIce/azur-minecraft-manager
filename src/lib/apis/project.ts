import { projectsCache } from "$lib/stores";
import { invoke } from "@tauri-apps/api";

export async function updateProject(id: string) {
    const project = await invoke<any>("get_project", { id: id });
    projectsCache.set(id, project);
    return project;
}

export async function getProject(id: string) {
    const project = await projectsCache.get<any>(id);
    if (project === null) return updateProject(id);
    return project;
}