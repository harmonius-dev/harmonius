//! Dirty flags for SoA render proxies (incremental GPU upload, R-2.10.2).

/// CPU-side proxy table with per-element dirty bits.
#[derive(Clone, Debug)]
pub struct ProxyDirtyTable {
    dirty: Vec<bool>,
}

impl ProxyDirtyTable {
    /// Allocates `capacity` proxies, all clean.
    #[must_use]
    pub fn new(capacity: usize) -> Self {
        Self {
            dirty: vec![false; capacity],
        }
    }

    /// Marks proxy `index` dirty for the next upload pass.
    pub fn mark_dirty(&mut self, index: usize) {
        self.dirty[index] = true;
    }

    /// Clears every dirty bit without counting writes (full refresh path).
    pub fn clear_all_dirty(&mut self) {
        for d in &mut self.dirty {
            *d = false;
        }
    }

    /// Simulates a GPU upload pass: counts dirty entries, then clears those bits.
    ///
    /// Returns how many proxy slots would be written for this pass.
    pub fn flush_dirty_writes(&mut self) -> usize {
        let mut n = 0_usize;
        for d in &mut self.dirty {
            if *d {
                n += 1;
                *d = false;
            }
        }
        n
    }
}

#[cfg(test)]
mod tests {
    use super::ProxyDirtyTable;

    /// TC-2.10.2.1 — only dirty proxies contribute to the simulated upload byte count.
    #[test]
    fn test_proxy_dirty_incremental() {
        let mut table = ProxyDirtyTable::new(10_000);
        for i in 0..10 {
            table.mark_dirty(i);
        }
        assert_eq!(table.flush_dirty_writes(), 10);
        assert_eq!(table.flush_dirty_writes(), 0);
    }
}
