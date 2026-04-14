//! Minimal shape grammar used for building slice acceptance tests.

/// Counts horizontal divisions implied by `floor_count` stacked floors.
pub fn horizontal_divisions_from_floors(floor_count: u32) -> u32 {
    floor_count.max(1)
}
