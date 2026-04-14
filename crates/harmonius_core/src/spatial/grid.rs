use std::collections::HashMap;

use glam::{UVec2, Vec2};
use smallvec::SmallVec;

use super::entity::Entity;
use super::error::SpatialError;

/// Integer cell coordinate inside a uniform grid.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct GridCoord {
    /// Cell index along X.
    pub x: u32,
    /// Cell index along Y.
    pub y: u32,
}

/// Configuration for a 2D uniform grid.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct GridConfig {
    /// World-space size of each cell edge.
    pub cell_size: f32,
    /// Number of cells along each axis.
    pub dimensions: UVec2,
    /// World-space position of cell (0,0)'s minimum corner.
    pub origin: Vec2,
}

/// Uniform 2D grid storing dense cells of entities.
pub struct UniformGrid {
    cells: Vec<SmallVec<[Entity; 32]>>,
    entity_cells: HashMap<u32, GridCoord>,
    positions: HashMap<u32, Vec2>,
    config: GridConfig,
}

impl UniformGrid {
    /// Creates an empty grid from configuration.
    #[must_use]
    pub fn new(config: GridConfig) -> Self {
        let count = (config.dimensions.x as usize).saturating_mul(config.dimensions.y as usize);
        let mut cells = Vec::with_capacity(count);
        cells.resize_with(count, SmallVec::new);
        Self {
            cells,
            entity_cells: HashMap::new(),
            positions: HashMap::new(),
            config,
        }
    }

    /// Inserts an entity at `position`, returning its cell coordinate.
    pub fn insert(&mut self, entity: Entity, position: Vec2) -> Result<GridCoord, SpatialError> {
        let coord = self
            .world_to_cell(position)
            .ok_or(SpatialError::OutOfBounds {
                coord: GridCoord { x: 0, y: 0 },
                dimensions: self.config.dimensions,
            })?;
        let idx = self.index(coord)?;
        self.cells[idx].push(entity);
        self.entity_cells.insert(entity.0, coord);
        self.positions.insert(entity.0, position);
        Ok(coord)
    }

    /// Removes an entity from a known cell.
    pub fn remove(&mut self, entity: Entity, cell: GridCoord) -> Result<(), SpatialError> {
        let idx = self.index(cell)?;
        let list = &mut self.cells[idx];
        if let Some(pos) = list.iter().position(|e| *e == entity) {
            list.swap_remove(pos);
        }
        self.entity_cells.remove(&entity.0);
        self.positions.remove(&entity.0);
        Ok(())
    }

    /// Moves an entity from `old_cell` to `new_position`.
    pub fn update(
        &mut self,
        entity: Entity,
        old_cell: GridCoord,
        new_position: Vec2,
    ) -> Result<GridCoord, SpatialError> {
        let new_cell = self
            .world_to_cell(new_position)
            .ok_or(SpatialError::OutOfBounds {
                coord: GridCoord { x: 0, y: 0 },
                dimensions: self.config.dimensions,
            })?;
        if new_cell != old_cell {
            self.remove(entity, old_cell)?;
            self.insert(entity, new_position)?;
        } else {
            self.positions.insert(entity.0, new_position);
        }
        Ok(new_cell)
    }

    /// Returns the slice of entities stored in `coord`.
    #[must_use]
    pub fn query_cell(&self, coord: GridCoord) -> &[Entity] {
        if let Ok(idx) = self.index(coord) {
            self.cells.get(idx).map(|v| v.as_slice()).unwrap_or(&[])
        } else {
            &[]
        }
    }

    /// Returns all entities whose cell centers lie within `radius` of `center`.
    #[must_use]
    pub fn query_radius(&self, center: Vec2, radius: f32) -> Vec<Entity> {
        let mut out = Vec::new();
        let r2 = radius * radius;
        let min = center - Vec2::splat(radius);
        let max = center + Vec2::splat(radius);
        let min_cell = self.world_to_cell_clamp(min);
        let max_cell = self.world_to_cell_clamp(max);
        for y in min_cell.y..=max_cell.y {
            for x in min_cell.x..=max_cell.x {
                let coord = GridCoord { x, y };
                for e in self.query_cell(coord) {
                    if let Some(p) = self.positions.get(&e.0) {
                        if p.distance_squared(center) <= r2 {
                            out.push(*e);
                        }
                    }
                }
            }
        }
        out.sort_by_key(|e| e.0);
        out.dedup();
        out
    }

    /// Converts world space to cell coordinate when in bounds.
    #[must_use]
    pub fn world_to_cell(&self, position: Vec2) -> Option<GridCoord> {
        let local = position - self.config.origin;
        if !local.x.is_finite() || !local.y.is_finite() {
            return None;
        }
        let xf = (local.x / self.config.cell_size).floor();
        let yf = (local.y / self.config.cell_size).floor();
        if xf < 0.0 || yf < 0.0 {
            return None;
        }
        let x = xf as u32;
        let y = yf as u32;
        if x >= self.config.dimensions.x || y >= self.config.dimensions.y {
            return None;
        }
        Some(GridCoord { x, y })
    }

    /// Total number of cells allocated.
    #[must_use]
    pub fn cell_count(&self) -> u32 {
        self.config.dimensions.x * self.config.dimensions.y
    }

    fn index(&self, coord: GridCoord) -> Result<usize, SpatialError> {
        if coord.x >= self.config.dimensions.x || coord.y >= self.config.dimensions.y {
            return Err(SpatialError::OutOfBounds {
                coord,
                dimensions: self.config.dimensions,
            });
        }
        Ok(coord.y as usize * self.config.dimensions.x as usize + coord.x as usize)
    }

    fn world_to_cell_clamp(&self, position: Vec2) -> GridCoord {
        let local = position - self.config.origin;
        let xf = (local.x / self.config.cell_size)
            .floor()
            .clamp(0.0, (self.config.dimensions.x.saturating_sub(1)) as f32);
        let yf = (local.y / self.config.cell_size)
            .floor()
            .clamp(0.0, (self.config.dimensions.y.saturating_sub(1)) as f32);
        GridCoord {
            x: xf as u32,
            y: yf as u32,
        }
    }
}
