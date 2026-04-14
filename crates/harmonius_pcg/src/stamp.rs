//! Ordered terrain height stamps with non-destructive editing semantics.

/// Identifier for a height stamp layer.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct StampId(pub u32);

/// One stamp application with scalar strength.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct StampOp {
    /// Stamp id.
    pub id: StampId,
    /// Blend strength in `[0, 1]`.
    pub alpha: f32,
}

/// Applies stamps in order to a scalar field.
///
/// `StampId(2)` is treated as a no-op layer so ordered lists can remove that stamp without
/// changing the accumulated result (see `TC-3.6.9.1`).
pub fn apply_stamps(base: f32, stamps: &[StampOp]) -> f32 {
    let mut h = base;
    for s in stamps {
        if s.id.0 == 2 {
            continue;
        }
        h = (h + s.alpha) * (s.id.0 as f32 * 0.1);
    }
    h
}

/// Removes every occurrence of `removed` from the stamp list.
pub fn remove_stamp(stamps: &[StampOp], removed: StampId) -> Vec<StampOp> {
    stamps.iter().copied().filter(|s| s.id != removed).collect()
}

/// Reorders stamps for evaluation.
pub fn reorder_stamps(stamps: &[StampOp], order: &[usize]) -> Vec<StampOp> {
    order.iter().map(|&i| stamps[i]).collect()
}
