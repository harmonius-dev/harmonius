//! Index recycling for particle buffers (`TC-11.1.1.7`).

/// Statistics returned by [`FreelistAllocator::stats`].
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct FreelistStats {
    /// Number of live particle slots.
    pub alive: usize,
    /// Number of free slots available for reuse.
    pub free: usize,
}

/// Fixed-capacity slot allocator with a free list.
#[derive(Debug)]
pub struct FreelistAllocator {
    alive: Vec<bool>,
    free: Vec<usize>,
}

impl FreelistAllocator {
    /// Creates an allocator with `capacity` slots, all initially free.
    #[must_use]
    pub fn new(capacity: usize) -> Self {
        let mut free = Vec::with_capacity(capacity);
        for i in 0..capacity {
            free.push(capacity - 1 - i);
        }
        Self {
            alive: vec![false; capacity],
            free,
        }
    }

    /// Allocates up to `requested` indices; may return fewer if the free list is exhausted.
    pub fn allocate(&mut self, requested: usize) -> Vec<usize> {
        let mut out = Vec::with_capacity(requested);
        for _ in 0..requested {
            let Some(idx) = self.free.pop() else {
                break;
            };
            if let Some(slot) = self.alive.get_mut(idx) {
                *slot = true;
            }
            out.push(idx);
        }
        out
    }

    /// Marks `indices` as dead and returns them to the free list.
    pub fn release(&mut self, indices: &[usize]) {
        for &idx in indices {
            if let Some(slot) = self.alive.get_mut(idx) {
                if *slot {
                    *slot = false;
                    self.free.push(idx);
                }
            }
        }
    }

    /// Returns live and free counts.
    #[must_use]
    pub fn stats(&self) -> FreelistStats {
        let alive = self.alive.iter().filter(|&&v| v).count();
        FreelistStats {
            alive,
            free: self.free.len(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// `TC-11.1.1.7` — emit 100, kill 50, emit 50 more; free list recycles.
    #[test]
    fn tc_11_1_1_7_free_list_allocation() {
        let mut a = FreelistAllocator::new(128);
        let first = a.allocate(100);
        assert_eq!(first.len(), 100);
        assert_eq!(
            a.stats(),
            FreelistStats {
                alive: 100,
                free: 28
            }
        );

        let killed: Vec<usize> = first.iter().copied().take(50).collect();
        a.release(&killed);
        assert_eq!(
            a.stats(),
            FreelistStats {
                alive: 50,
                free: 78
            }
        );

        let second = a.allocate(50);
        assert_eq!(second.len(), 50);
        assert_eq!(
            a.stats(),
            FreelistStats {
                alive: 100,
                free: 28
            }
        );

        let recycled: std::collections::HashSet<usize> = second.iter().copied().collect();
        let killed_set: std::collections::HashSet<usize> = killed.iter().copied().collect();
        let overlap = recycled.intersection(&killed_set).count();
        assert_eq!(
            overlap, 50,
            "expected second wave to reuse all freed indices"
        );
    }
}
