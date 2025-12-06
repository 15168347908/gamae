use crate::types::base::Id;

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
