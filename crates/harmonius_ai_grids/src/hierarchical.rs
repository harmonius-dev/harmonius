use crate::coord::CellCoord;

/// Multi-LOD 2D influence grid: each LOD is a complete [`UniformGrid`] at coarser resolution.
#[derive(Clone, Debug)]
pub struct HierarchicalGrid<T> {
    lods: Vec<crate::uniform::UniformGrid<T>>,
}

impl<T: Clone + Default> HierarchicalGrid<T> {
    /// Builds a hierarchical grid from pre-constructed LOD layers (LOD 0 is finest).
    #[must_use]
    pub fn new(lods: Vec<crate::uniform::UniformGrid<T>>) -> Self {
        Self { lods }
    }

    /// Borrow LOD layers from finest to coarsest.
    #[must_use]
    pub fn lods(&self) -> &[crate::uniform::UniformGrid<T>] {
        &self.lods
    }

    /// Samples the given LOD level at a cell coordinate valid for that LOD grid.
    #[must_use]
    pub fn sample_lod(&self, coord: CellCoord, lod: u8) -> Option<T> {
        let grid = self.lods.get(usize::from(lod))?;
        grid.get_front(coord)
    }
}
