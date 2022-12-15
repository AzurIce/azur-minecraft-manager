
enum TargetType {
    Local = "Local",
    Server = "Server",
}

class Target {
    kind: TargetType;
    location: string;

    constructor(kind: TargetType, location: string) {
        this.kind = kind;
        this.location = location;
    }
}

export { TargetType, Target }