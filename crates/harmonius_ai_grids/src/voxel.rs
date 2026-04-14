use crate::coord::VoxelCoord;

/// Fixed-resolution 3D voxel volume backed by a dense buffer.
#[derive(Clone, Debug)]
pub struct VoxelVolume<T> {
    width: u32,
    height: u32,
    depth: u32,
    cell_size: f32,
    origin_x: f32,
    origin_y: f32,
    origin_z: f32,
    default_value: T,
    front: Vec<T>,
    back: Vec<T>,
}

impl<T: Clone + Default> VoxelVolume<T> {
    /// Creates a volume filled with `initial` values in both buffers.
    #[must_use]
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        width: u32,
        height: u32,
        depth: u32,
        cell_size: f32,
        origin_x: f32,
        origin_y: f32,
        origin_z: f32,
        default_value: T,
        initial: T,
    ) -> Self {
        let len = usize::try_from(
            width
                .checked_mul(height)
                .and_then(|v| v.checked_mul(depth))
                .unwrap_or(0),
        )
        .unwrap_or(0);
        let front = vec![initial.clone(); len];
        let back = vec![initial; len];
        Self {
            width,
            height,
            depth,
            cell_size,
            origin_x,
            origin_y,
            origin_z,
            default_value,
            front,
            back,
        }
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

    /// Mutable access to the back buffer for simulation writes.
    pub fn back_mut(&mut self) -> &mut [T] {
        &mut self.back
    }

    /// Writes a voxel in the front buffer (test / authoring helper).
    pub fn set_front(&mut self, coord: VoxelCoord, value: T) -> bool {
        if let Some(idx) = self.index(coord) {
            self.front[idx] = value;
            true
        } else {
            false
        }
    }

    /// Reads a voxel from the front buffer.
    #[must_use]
    pub fn get_front(&self, coord: VoxelCoord) -> Option<T> {
        self.index(coord).map(|idx| self.front[idx].clone())
    }

    /// Reads a voxel from the back buffer.
    #[must_use]
    pub fn get_back(&self, coord: VoxelCoord) -> Option<T> {
        self.index(coord).map(|idx| self.back[idx].clone())
    }

    /// Overwrites a back-buffer voxel during drain.
    pub fn set_back(&mut self, coord: VoxelCoord, value: T) -> bool {
        if let Some(idx) = self.index(coord) {
            self.back[idx] = value;
            true
        } else {
            false
        }
    }

    /// Converts world coordinates to voxel coordinates.
    #[must_use]
    pub fn world_to_voxel(&self, world_x: f32, world_y: f32, world_z: f32) -> Option<VoxelCoord> {
        let fx = (world_x - self.origin_x) / self.cell_size;
        let fy = (world_y - self.origin_y) / self.cell_size;
        let fz = (world_z - self.origin_z) / self.cell_size;
        let x = fx.floor() as i32;
        let y = fy.floor() as i32;
        let z = fz.floor() as i32;
        if x >= 0
            && y >= 0
            && z >= 0
            && (x as u32) < self.width
            && (y as u32) < self.height
            && (z as u32) < self.depth
        {
            Some(VoxelCoord { x, y, z })
        } else {
            None
        }
    }

    /// Default value used when sampling out of bounds (FM-1).
    #[must_use]
    pub fn default_value(&self) -> T {
        self.default_value.clone()
    }

    fn index(&self, coord: VoxelCoord) -> Option<usize> {
        if coord.x < 0 || coord.y < 0 || coord.z < 0 {
            return None;
        }
        let ux = u32::try_from(coord.x).ok()?;
        let uy = u32::try_from(coord.y).ok()?;
        let uz = u32::try_from(coord.z).ok()?;
        if ux >= self.width || uy >= self.height || uz >= self.depth {
            return None;
        }
        let stride_y = self.width;
        let stride_z = self.width.checked_mul(self.height)?;
        usize::try_from(uz * stride_z + uy * stride_y + ux).ok()
    }
}
