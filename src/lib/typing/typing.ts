
export enum TargetType {
    Local = "Local",
    Server = "Server",
}

export interface Target {
    kind: TargetType;
    location: string;
}

export enum ModLoader {
    Forge = "Forge",
    Fabric = "Fabric",
    Quilt = "Quilt",
    Unknown = "Unknown",
}

export enum ModEnv {
    Client = "Client",
    Server = "Server",
    Both = "Both",
    Unknown = "Unknown",
}

export interface Modrinth {
    Modrinth: string
}

export enum ModSource {
    Modrinth,
    Unknown = "Unknown"
}

export interface Mod {
    source: ModSource,
    name: string,
    desc: string,
    env: ModEnv,
}

export interface ModFile {
    filename: string, // From file
    sha1: string,

    belong_mod: Mod, // From sha1 + modrinth API
    game_versions: Array<string>,
}