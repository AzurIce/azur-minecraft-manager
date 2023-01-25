import { projectsCache } from "$lib/stores";
import { invoke } from "@tauri-apps/api";
import { updateVersions } from "./version";

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

export async function updateProjects(ids: Array<string>) {
    const projects = await invoke<Array<any>>("get_projects", { ids: ids });
    return await Promise.all(projects.map(async project => {
        projectsCache.set(project.id, project);
        return await updateVersions(project.versions);
    }));
}