//! Type aliases for commonly used third-party containers.

/// Fixed-size bitset used by ECS component masks, layer masks, and dirty bit tracking.
pub type FixedBitSet = fixedbitset::FixedBitSet;

/// Inline-allocated small vector. Prefer capacity 4, 8, or 16.
pub type SmallVec<T, const N: usize> = smallvec::SmallVec<[T; N]>;
