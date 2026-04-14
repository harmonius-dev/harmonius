//! Dynamic mesh split/merge decisions (`R-8.7.3`).

use std::collections::HashMap;

/// Identifier for a spatial cell tracked by [`MeshController`].
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct CellId(pub u32);

/// High-level mesh topology change.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum MeshEvent {
    /// A dense cell was split into two children.
    Split {
        /// Parent cell that was overloaded.
        parent: CellId,
        /// New child cells receiving reassigned load.
        children: [CellId; 2],
    },
    /// Two underutilized cells merged into one.
    Merge {
        /// Resulting merged cell id.
        result: CellId,
        /// Source cells that were combined.
        sources: [CellId; 2],
    },
}

/// Tracks per-cell entity counts and emits split/merge events.
#[derive(Debug)]
pub struct MeshController {
    split_threshold: u32,
    merge_threshold: u32,
    merge_cooldown_ticks: u32,
    cells: HashMap<CellId, u32>,
    neighbors: HashMap<CellId, CellId>,
    under_ticks: HashMap<CellId, u32>,
    pub events: Vec<MeshEvent>,
    next_cell_id: u32,
}

impl MeshController {
    /// Creates a controller with the provided density thresholds.
    pub fn new(split_threshold: u32, merge_threshold: u32, merge_cooldown_ticks: u32) -> Self {
        Self {
            split_threshold,
            merge_threshold,
            merge_cooldown_ticks,
            cells: HashMap::new(),
            neighbors: HashMap::new(),
            under_ticks: HashMap::new(),
            events: Vec::new(),
            next_cell_id: 10_000,
        }
    }

    /// Registers `count` entities inside `cell`, optionally adjacent to `neighbor`.
    pub fn register_cell(&mut self, cell: CellId, count: u32, neighbor: Option<CellId>) {
        self.cells.insert(cell, count);
        if let Some(n) = neighbor {
            self.neighbors.insert(cell, n);
            self.neighbors.insert(n, cell);
        }
    }

    /// Advances controller logic for one tick.
    pub fn tick(&mut self) {
        self.try_splits();
        self.try_merges();
    }

    fn allocate_child_id(&mut self) -> CellId {
        let id = CellId(self.next_cell_id);
        self.next_cell_id = self.next_cell_id.saturating_add(1);
        id
    }

    fn try_splits(&mut self) {
        let mut parents: Vec<(CellId, u32)> = self.cells.iter().map(|(&k, &v)| (k, v)).collect();
        parents.sort_by_key(|(c, _)| c.0);
        for (parent, count) in parents {
            if count <= self.split_threshold {
                continue;
            }
            let left = self.allocate_child_id();
            let right = self.allocate_child_id();
            let half = count / 2;
            self.cells.insert(left, half);
            self.cells.insert(right, count - half);
            self.cells.remove(&parent);
            self.events.push(MeshEvent::Split {
                parent,
                children: [left, right],
            });
        }
    }

    fn try_merges(&mut self) {
        let pairs: Vec<(CellId, CellId)> = self
            .neighbors
            .iter()
            .filter_map(|(&a, &b)| if a.0 < b.0 { Some((a, b)) } else { None })
            .collect();
        for (a, b) in pairs {
            let ca = *self.cells.get(&a).unwrap_or(&0);
            let cb = *self.cells.get(&b).unwrap_or(&0);
            if ca + cb > self.merge_threshold {
                self.under_ticks.remove(&a);
                self.under_ticks.remove(&b);
                continue;
            }
            let ticks_a = self
                .under_ticks
                .get(&a)
                .copied()
                .unwrap_or(0)
                .saturating_add(1);
            let ticks_b = self
                .under_ticks
                .get(&b)
                .copied()
                .unwrap_or(0)
                .saturating_add(1);
            self.under_ticks.insert(a, ticks_a);
            self.under_ticks.insert(b, ticks_b);
            if ticks_a < self.merge_cooldown_ticks || ticks_b < self.merge_cooldown_ticks {
                continue;
            }
            let merged = self.allocate_child_id();
            let total = ca + cb;
            self.cells.remove(&a);
            self.cells.remove(&b);
            self.cells.insert(merged, total);
            self.neighbors.remove(&a);
            self.neighbors.remove(&b);
            self.under_ticks.remove(&a);
            self.under_ticks.remove(&b);
            self.events.push(MeshEvent::Merge {
                result: merged,
                sources: [a, b],
            });
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// `TC-8.7.3.1` `test_mesh_split_overload`
    #[test]
    fn test_mesh_split_overload() {
        let mut mesh = MeshController::new(1000, 100, 1);
        let cell = CellId(1);
        mesh.register_cell(cell, 1500, None);
        mesh.tick();
        let split = mesh
            .events
            .iter()
            .find_map(|e| match e {
                MeshEvent::Split { parent, children } if *parent == cell => Some(children),
                _ => None,
            })
            .expect("split event");
        let left = mesh.cells.get(&split[0]).copied().unwrap_or(0);
        let right = mesh.cells.get(&split[1]).copied().unwrap_or(0);
        assert_eq!(left + right, 1500);
        assert!((750..=850).contains(&left) || (650..=750).contains(&left));
    }

    /// `TC-8.7.3.2` `test_mesh_merge_underutilized`
    #[test]
    fn test_mesh_merge_underutilized() {
        let mut mesh = MeshController::new(2000, 100, 1);
        let a = CellId(1);
        let b = CellId(2);
        mesh.register_cell(a, 50, Some(b));
        mesh.register_cell(b, 50, Some(a));
        mesh.tick();
        mesh.tick();
        assert!(mesh.events.iter().any(|e| matches!(e, MeshEvent::Merge { .. })));
        let total: u32 = mesh.cells.values().sum();
        assert_eq!(total, 100);
    }
}
