use super::base::Id;
struct Artifact {
    id: Id<ArtifactType>,
}

enum ArtifactType {
    Steel,
    Wood,
    Plastic,
    Seed,
    Mineral,
}

struct Slot {
    id: Id<String>,
    status: SlotStatus,
}

enum SlotStatus {
    Idle,
    Working,
}

struct Factory {
    id: Id<FactoryType>,
    slots: Vec<Slot>,
}

enum FactoryType {
    Basic,
    Intermediate,
    Senior,
}
