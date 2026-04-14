//! Grid occupancy and first-fit placement.

use std::collections::HashMap;

use fixedbitset::FixedBitSet;

use crate::entity::Entity;
use crate::transfer::TransferError;

/// Row-major occupancy bitmap with rectangle operations.
#[derive(Clone, Debug)]
pub struct GridOccupancy {
    cells: FixedBitSet,
    width: u16,
    height: u16,
}

impl GridOccupancy {
    /// Builds an empty occupancy map for `width` × `height`.
    #[must_use]
    pub fn new(width: u16, height: u16) -> Self {
        let len = usize::from(width) * usize::from(height);
        Self {
            cells: FixedBitSet::with_capacity(len),
            width,
            height,
        }
    }

    /// Grid width in cells.
    #[must_use]
    pub fn width(&self) -> u16 {
        self.width
    }

    /// Grid height in cells.
    #[must_use]
    pub fn height(&self) -> u16 {
        self.height
    }

    /// Returns true when every cell in the axis-aligned region is free.
    #[must_use]
    pub fn is_region_free(&self, x: u16, y: u16, w: u16, h: u16) -> bool {
        if x.saturating_add(w) > self.width || y.saturating_add(h) > self.height {
            return false;
        }
        for yy in y..y + h {
            for xx in x..x + w {
                if self.cells.contains(Self::idx(self.width, xx, yy)) {
                    return false;
                }
            }
        }
        true
    }

    /// Marks a region occupied.
    pub fn occupy(&mut self, x: u16, y: u16, w: u16, h: u16) {
        for yy in y..y + h {
            for xx in x..x + w {
                let idx = Self::idx(self.width, xx, yy);
                self.cells.insert(idx);
            }
        }
    }

    /// Clears a region.
    pub fn vacate(&mut self, x: u16, y: u16, w: u16, h: u16) {
        for yy in y..y + h {
            for xx in x..x + w {
                let idx = Self::idx(self.width, xx, yy);
                self.cells.set(idx, false);
            }
        }
    }

    /// Returns true when a cell is occupied.
    #[must_use]
    pub fn is_occupied(&self, x: u16, y: u16) -> bool {
        self.cells.contains(Self::idx(self.width, x, y))
    }

    fn idx(width: u16, x: u16, y: u16) -> usize {
        usize::from(y) * usize::from(width) + usize::from(x)
    }
}

/// Grid-backed container with first-fit placement.
#[derive(Clone, Debug)]
pub struct GridContainer {
    occupancy: GridOccupancy,
    placements: HashMap<Entity, (u16, u16, u16, u16)>,
}

impl GridContainer {
    /// Creates an empty `width` × `height` grid container.
    #[must_use]
    pub fn new(width: u16, height: u16) -> Self {
        Self {
            occupancy: GridOccupancy::new(width, height),
            placements: HashMap::new(),
        }
    }

    /// Inserts `item` with footprint `w`×`h`, returning the chosen origin.
    pub fn insert(&mut self, w: u16, h: u16, item: Entity) -> Result<(u16, u16), TransferError> {
        let width = self.occupancy.width();
        let height = self.occupancy.height();
        if w == 0 || h == 0 {
            return Err(TransferError::NoFreeRegion);
        }
        for y in 0..=height.saturating_sub(h) {
            for x in 0..=width.saturating_sub(w) {
                if self.occupancy.is_region_free(x, y, w, h) {
                    self.occupancy.occupy(x, y, w, h);
                    self.placements.insert(item, (x, y, w, h));
                    return Ok((x, y));
                }
            }
        }
        Err(TransferError::NoFreeRegion)
    }

    /// Removes an item and clears its footprint.
    pub fn remove(&mut self, item: Entity) -> Result<(), TransferError> {
        let Some((x, y, w, h)) = self.placements.remove(&item) else {
            return Err(TransferError::ItemNotFound);
        };
        self.occupancy.vacate(x, y, w, h);
        Ok(())
    }

    /// Borrowed occupancy map for assertions.
    #[must_use]
    pub fn occupancy(&self) -> &GridOccupancy {
        &self.occupancy
    }
}
