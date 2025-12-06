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
