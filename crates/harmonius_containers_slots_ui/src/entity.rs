/// Generational entity identifier shared with the integration design (`Entity` in ECS).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Entity(pub u64);
