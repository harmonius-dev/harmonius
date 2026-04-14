//! Deferred decal ordering, fade, and atlas LRU (IR-3.7.4).

use std::collections::VecDeque;

use glam::Vec3;

use crate::types::DecalEntry;

/// Stable sort: lower [`DecalEntry::priority`] first (bottom), higher last (top).
///
/// TC-IR-3.7.4.2 — priority-2 decal composites after priority-1.
pub fn sort_decals_by_priority(decals: &mut [DecalEntry]) {
    let mut decorated: Vec<(u32, usize)> = decals
        .iter()
        .enumerate()
        .map(|(i, d)| (d.priority, i))
        .collect();
    decorated.sort_by(|a, b| a.0.cmp(&b.0).then_with(|| a.1.cmp(&b.1)));
    let mut out = Vec::with_capacity(decals.len());
    for (_, orig) in decorated {
        out.push(decals[orig].clone());
    }
    decals.clone_from_slice(&out);
}

/// Alpha factor for slope-based decal fade.
///
/// TC-IR-3.7.4.3 — at 80° slope with 60° threshold band fading by 80°, alpha reaches 0.
pub fn decal_alpha_for_slope(slope_deg: f32, fade_start_deg: f32, fully_faded_deg: f32) -> f32 {
    if slope_deg <= fade_start_deg {
        return 1.0;
    }
    if slope_deg >= fully_faded_deg {
        return 0.0;
    }
    (fully_faded_deg - slope_deg) / (fully_faded_deg - fade_start_deg)
}

/// Composites decal albedo over a scalar G-buffer channel for a single texel.
///
/// TC-IR-3.7.4.1 — decal modifies stored albedo versus baseline.
pub fn apply_decal_albedo_channel(
    baseline: f32,
    decal_albedo: f32,
    decal_alpha: f32,
) -> f32 {
    let a = decal_alpha.clamp(0.0, 1.0);
    baseline * (1.0 - a) + decal_albedo * a
}

/// Fixed-capacity decal atlas with LRU eviction by monotonic slot id.
#[derive(Clone, Debug)]
pub struct DecalAtlas {
    /// `(insertion_id, entry)` per slot; `None` is free.
    slots: Vec<Option<(u64, DecalEntry)>>,
    next_id: u64,
}

impl DecalAtlas {
    /// Creates an atlas with `capacity` decal slots.
    pub fn new(capacity: usize) -> Self {
        Self {
            slots: vec![None; capacity],
            next_id: 0,
        }
    }

    /// TC-IR-3.7.4.F1 — insert with LRU eviction of the oldest occupied slot when full.
    pub fn insert_or_evict(&mut self, entry: DecalEntry) -> u64 {
        self.next_id = self.next_id.wrapping_add(1);
        let id = self.next_id;
        if let Some((free_idx, _)) = self.slots.iter().enumerate().find(|(_, s)| s.is_none()) {
            self.slots[free_idx] = Some((id, entry));
            return id;
        }
        let mut oldest_idx = 0usize;
        let mut oldest_key = u64::MAX;
        for (i, s) in self.slots.iter().enumerate() {
            if let Some((k, _)) = s {
                if *k < oldest_key {
                    oldest_key = *k;
                    oldest_idx = i;
                }
            }
        }
        self.slots[oldest_idx] = Some((id, entry));
        oldest_key
    }
}

/// Returns the visible albedo after stacking decals in slice order (caller sorts).
pub fn stack_decal_albedo_on_wall(baseline: Vec3, decals: &[DecalEntry]) -> Vec3 {
    let mut c = baseline;
    for d in decals {
        let a = decal_alpha_for_slope(d.surface_slope_deg, 60.0, 80.0);
        c = Vec3::new(
            apply_decal_albedo_channel(c.x, d.albedo.x, a),
            apply_decal_albedo_channel(c.y, d.albedo.y, a),
            apply_decal_albedo_channel(c.z, d.albedo.z, a),
        );
    }
    c
}

/// Debug helper: collect eviction order for tests.
pub fn lru_eviction_fifo_probe(capacity: usize, push_count: usize) -> VecDeque<u64> {
    let mut atlas = DecalAtlas::new(capacity);
    let mut evicted = VecDeque::new();
    for i in 0..push_count {
        let e = DecalEntry {
            priority: i as u32,
            albedo: Vec3::ONE,
            surface_slope_deg: 0.0,
        };
        let ev = atlas.insert_or_evict(e);
        if i >= capacity {
            evicted.push_back(ev);
        }
    }
    evicted
}
