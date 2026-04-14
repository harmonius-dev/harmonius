use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

/// Hashed runtime identifier for triggers and markers (integration design).
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct StringId(pub u64);

impl From<&str> for StringId {
    fn from(text: &str) -> Self {
        let mut hasher = DefaultHasher::new();
        text.hash(&mut hasher);
        Self(hasher.finish())
    }
}
