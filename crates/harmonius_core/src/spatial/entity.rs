/// Lightweight entity identifier used by spatial indices before full ECS wiring.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct Entity(pub u32);
