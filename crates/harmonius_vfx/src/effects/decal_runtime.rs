//! Decal ordering and pool reclaim for deferred decals.

/// Minimal decal record used for sort and pool tests.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DecalSortKey {
    /// Lower renders first when sorting ascending.
    pub priority: u8,
    /// Stable insertion order for deterministic ties.
    pub spawn_index: u32,
}

/// Returns render indices sorted ascending by priority (**TC-11.2.4.2**).
pub fn sort_decal_indices(decals: &[DecalSortKey]) -> Vec<usize> {
    let mut idx: Vec<usize> = (0..decals.len()).collect();
    idx.sort_by(|&a, &b| {
        decals[a]
            .priority
            .cmp(&decals[b].priority)
            .then_with(|| decals[a].spawn_index.cmp(&decals[b].spawn_index))
    });
    idx
}

/// Pool entry for reclaim tests.
#[derive(Clone, Copy, Debug)]
pub struct PooledDecal {
    /// Priority used for reclaim policy.
    pub priority: u8,
    /// Monotonic spawn time; lower is older.
    pub spawn_time: u32,
}

/// Fixed-size decal pool with priority reclaim (**TC-11.2.4.3**).
#[derive(Debug)]
pub struct DecalPool {
    slots: Vec<Option<PooledDecal>>,
}

impl DecalPool {
    /// Creates an empty pool with `max` slots.
    pub fn new(max: usize) -> Self {
        Self {
            slots: vec![None; max],
        }
    }

    /// Attempts to insert `decal`, reclaiming the oldest lowest-priority resident if full.
    pub fn try_push(&mut self, decal: PooledDecal) -> bool {
        if let Some(i) = self.slots.iter().position(|s| s.is_none()) {
            self.slots[i] = Some(decal);
            return true;
        }
        let victim = self
            .slots
            .iter()
            .enumerate()
            .filter_map(|(i, s)| s.map(|d| (i, d)))
            .min_by_key(|&(_, d)| (d.priority, d.spawn_time));
        if let Some((i, _)) = victim {
            if self.slots[i].unwrap().priority < decal.priority {
                self.slots[i] = Some(decal);
                return true;
            }
        }
        false
    }

    /// Returns true if any slot holds `priority`.
    pub fn contains_priority(&self, priority: u8) -> bool {
        self.slots
            .iter()
            .filter_map(|s| *s)
            .any(|d| d.priority == priority)
    }
}

#[cfg(test)]
mod tests {
    use super::{DecalPool, DecalSortKey, PooledDecal, sort_decal_indices};

    /// **TC-11.2.4.2** — ascending priority sort order.
    #[test]
    fn tc_11_2_4_2_decal_priority_sorting() {
        let decals = [
            DecalSortKey {
                priority: 3,
                spawn_index: 1,
            },
            DecalSortKey {
                priority: 1,
                spawn_index: 0,
            },
            DecalSortKey {
                priority: 5,
                spawn_index: 2,
            },
        ];
        let order = sort_decal_indices(&decals);
        assert_eq!(order, vec![1, 0, 2]);
    }

    /// **TC-11.2.4.3** — reclaim oldest low priority when pool is exhausted.
    #[test]
    fn tc_11_2_4_3_decal_pool_reclaim() {
        let mut p = DecalPool::new(2);
        assert!(p.try_push(PooledDecal {
            priority: 1,
            spawn_time: 0,
        }));
        assert!(p.try_push(PooledDecal {
            priority: 2,
            spawn_time: 1,
        }));
        assert!(!p.contains_priority(9));
        assert!(p.try_push(PooledDecal {
            priority: 9,
            spawn_time: 2,
        }));
        assert!(p.contains_priority(9));
        assert!(!p.contains_priority(1));
    }
}
