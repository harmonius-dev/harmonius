/// Stable entity identifier (ECS stand-in for unit tests).
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, Ord, PartialOrd)]
#[repr(transparent)]
pub struct Entity(pub u32);

/// Minimal component view for spatial query predicates.
#[derive(Clone, Debug)]
pub struct EntityRef {
    pub entity: Entity,
    pub layers: u64,
    pub tags: u64,
    pub health: f32,
}

impl EntityRef {
    pub fn new(entity: Entity, layers: u64, tags: u64, health: f32) -> Self {
        Self {
            entity,
            layers,
            tags,
            health,
        }
    }
}
