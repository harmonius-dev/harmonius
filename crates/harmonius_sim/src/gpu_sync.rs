//! CPU-side dirty region tracking for GPU texture uploads.

use rkyv_derive::{Archive, Deserialize, Serialize};

use crate::coord::CellCoord;

/// Inclusive axis-aligned dirty rectangle in cell space.
#[derive(Archive, Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct DirtyRegion {
    /// Minimum corner (inclusive).
    pub min: CellCoord,
    /// Maximum corner (inclusive).
    pub max: CellCoord,
}

/// Accumulates dirty rectangles and merges overlaps.
#[derive(Clone, Debug, Default)]
pub struct GpuGridSync {
    pending: Vec<DirtyRegion>,
}

impl GpuGridSync {
    /// Creates an empty sync tracker.
    pub fn new() -> Self {
        Self {
            pending: Vec::new(),
        }
    }

    /// Marks `[min, max]` dirty, merging overlaps into fewer regions.
    pub fn mark_dirty(&mut self, min: CellCoord, max: CellCoord) {
        self.pending.push(DirtyRegion { min, max });
        self.merge_pending();
    }

    fn merge_pending(&mut self) {
        loop {
            let mut merged = false;
            let mut rects = std::mem::take(&mut self.pending);
            let mut out: Vec<DirtyRegion> = Vec::new();
            'next_rect: for r in rects.drain(..) {
                for t in &mut out {
                    if rects_intersect(*t, r) {
                        t.min.x = t.min.x.min(r.min.x);
                        t.min.y = t.min.y.min(r.min.y);
                        t.max.x = t.max.x.max(r.max.x);
                        t.max.y = t.max.y.max(r.max.y);
                        merged = true;
                        continue 'next_rect;
                    }
                }
                out.push(r);
            }
            self.pending = out;
            if !merged {
                break;
            }
        }
    }

    /// Returns pending regions and clears the queue.
    pub fn drain(&mut self) -> Vec<DirtyRegion> {
        std::mem::take(&mut self.pending)
    }

    /// Returns `true` when uploads are pending.
    pub fn has_pending(&self) -> bool {
        !self.pending.is_empty()
    }
}

fn rects_intersect(a: DirtyRegion, b: DirtyRegion) -> bool {
    !(a.max.x < b.min.x || b.max.x < a.min.x || a.max.y < b.min.y || b.max.y < a.min.y)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_gpu_sync_merge_overlap() {
        let mut sync = GpuGridSync::new();
        sync.mark_dirty(CellCoord { x: 0, y: 0 }, CellCoord { x: 2, y: 2 });
        sync.mark_dirty(CellCoord { x: 1, y: 1 }, CellCoord { x: 3, y: 3 });
        let drained = sync.drain();
        assert_eq!(drained.len(), 1);
        assert_eq!(
            drained[0],
            DirtyRegion {
                min: CellCoord { x: 0, y: 0 },
                max: CellCoord { x: 3, y: 3 },
            }
        );
    }

    #[test]
    fn test_gpu_sync_merge_adjacent() {
        let mut sync = GpuGridSync::new();
        sync.mark_dirty(CellCoord { x: 0, y: 0 }, CellCoord { x: 1, y: 1 });
        sync.mark_dirty(CellCoord { x: 2, y: 2 }, CellCoord { x: 3, y: 3 });
        let drained = sync.drain();
        assert_eq!(drained.len(), 2);
    }
}
