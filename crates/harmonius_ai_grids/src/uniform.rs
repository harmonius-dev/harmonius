use crate::coord::CellCoord;

/// Fixed-resolution 2D uniform grid with axis-aligned cells.
#[derive(Clone, Debug)]
pub struct UniformGrid<T> {
    width: u32,
    height: u32,
    cell_size: f32,
    origin_x: f32,
    origin_y: f32,
    default_value: T,
    front: Vec<T>,
    back: Vec<T>,
    propagation_skipped: bool,
}

impl<T: Clone + Default> UniformGrid<T> {
    /// Creates a grid filled with `initial` values in both buffers.
    #[must_use]
    pub fn new(
        width: u32,
        height: u32,
        cell_size: f32,
        origin_x: f32,
        origin_y: f32,
        default_value: T,
        initial: T,
    ) -> Self {
        let len = usize::try_from(width.checked_mul(height).unwrap_or(0)).unwrap_or(0);
        let front = vec![initial.clone(); len];
        let back = vec![initial; len];
        Self {
            width,
            height,
            cell_size,
            origin_x,
            origin_y,
            default_value,
            front,
            back,
            propagation_skipped: false,
        }
    }

    /// Marks whether propagation was skipped for the last tick (FM-2 / TC-E8).
    pub fn set_propagation_skipped(&mut self, skipped: bool) {
        self.propagation_skipped = skipped;
    }

    /// Returns whether propagation was skipped.
    #[must_use]
    pub const fn propagation_skipped(&self) -> bool {
        self.propagation_skipped
    }

    /// Swaps front/back after Phase 3 completes.
    pub fn swap_buffers(&mut self) {
        std::mem::swap(&mut self.front, &mut self.back);
    }

    /// Copies the front snapshot into the back buffer before drain + propagation.
    pub fn sync_back_from_front(&mut self)
    where
        T: Clone,
    {
        self.back.clone_from_slice(&self.front);
    }

    /// Grid width in cells.
    #[must_use]
    pub const fn width(&self) -> u32 {
        self.width
    }

    /// Grid height in cells.
    #[must_use]
    pub const fn height(&self) -> u32 {
        self.height
    }

    /// Mutable access to the back buffer for simulation writes.
    pub fn back_mut(&mut self) -> &mut [T] {
        &mut self.back
    }

    /// Immutable read access to the back buffer.
    #[must_use]
    pub fn back_slice(&self) -> &[T] {
        &self.back
    }

    /// Immutable read access to the front buffer (Phase 4 snapshot).
    #[must_use]
    pub fn front_slice(&self) -> &[T] {
        &self.front
    }

    /// Writes a cell in the front buffer (test / authoring helper).
    pub fn set_front(&mut self, coord: CellCoord, value: T) -> bool {
        if let Some(idx) = self.index(coord) {
            self.front[idx] = value;
            true
        } else {
            false
        }
    }

    /// Reads a cell from the front buffer.
    #[must_use]
    pub fn get_front(&self, coord: CellCoord) -> Option<T> {
        self.index(coord).map(|idx| self.front[idx].clone())
    }

    /// Reads a cell from the back buffer (used by the drain applier).
    #[must_use]
    pub fn get_back(&self, coord: CellCoord) -> Option<T> {
        self.index(coord).map(|idx| self.back[idx].clone())
    }

    /// Overwrites a back-buffer cell during drain.
    pub fn set_back(&mut self, coord: CellCoord, value: T) -> bool {
        if let Some(idx) = self.index(coord) {
            self.back[idx] = value;
            true
        } else {
            false
        }
    }

    /// Converts world coordinates to cell coordinates.
    #[must_use]
    pub fn world_to_cell(&self, world_x: f32, world_y: f32) -> Option<CellCoord> {
        let fx = (world_x - self.origin_x) / self.cell_size;
        let fy = (world_y - self.origin_y) / self.cell_size;
        let x = fx.floor() as i32;
        let y = fy.floor() as i32;
        if x >= 0 && y >= 0 && (x as u32) < self.width && (y as u32) < self.height {
            Some(CellCoord { x, y })
        } else {
            None
        }
    }

    /// Default value used when sampling out of bounds (FM-1).
    #[must_use]
    pub fn default_value(&self) -> T {
        self.default_value.clone()
    }

    fn index(&self, coord: CellCoord) -> Option<usize> {
        if coord.x < 0 || coord.y < 0 {
            return None;
        }
        let ux = u32::try_from(coord.x).ok()?;
        let uy = u32::try_from(coord.y).ok()?;
        if ux >= self.width || uy >= self.height {
            return None;
        }
        usize::try_from(uy * self.width + ux).ok()
    }
}
