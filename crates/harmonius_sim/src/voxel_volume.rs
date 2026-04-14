//! Chunked `VoxelVolume` storage, raycasts, and palette-backed chunks.

use std::collections::HashMap;

use glam::Vec3;

use crate::coord::{ChunkCoord, VoxelCoord};

fn pack_chunk(c: ChunkCoord) -> u64 {
    (c.x as u64) << 42 | (c.y as u64) << 21 | (c.z as u64)
}

fn unpack_chunk_key(key: u64) -> ChunkCoord {
    ChunkCoord {
        x: (key >> 42) as u32,
        y: ((key >> 21) & ((1u64 << 21) - 1)) as u32,
        z: (key & ((1u64 << 21) - 1)) as u32,
    }
}

/// Single dense chunk of voxels.
#[derive(Clone, Debug)]
pub struct VoxelChunk<T> {
    size: u32,
    cells: Vec<T>,
    dirty: bool,
}

impl<T: Clone + Default> VoxelChunk<T> {
    /// Allocates a chunk filled with defaults.
    pub fn new(size: u32) -> Self {
        let len = (size as usize).pow(3);
        Self {
            size,
            cells: vec![T::default(); len],
            dirty: false,
        }
    }

    fn index(&self, local: VoxelCoord) -> Option<usize> {
        if local.x >= self.size || local.y >= self.size || local.z >= self.size {
            return None;
        }
        Some(
            (local.z as usize)
                .saturating_mul(self.size as usize)
                .saturating_mul(self.size as usize)
                + (local.y as usize).saturating_mul(self.size as usize)
                + local.x as usize,
        )
    }

    /// Reads a voxel inside the chunk using local coordinates.
    pub fn get_local(&self, local: VoxelCoord) -> Option<&T> {
        let i = self.index(local)?;
        self.cells.get(i)
    }

    /// Writes a voxel inside the chunk using local coordinates.
    pub fn set_local(&mut self, local: VoxelCoord, value: T) {
        let i = self
            .index(local)
            .unwrap_or_else(|| panic!("set_local out of bounds: {:?}", local));
        self.cells[i] = value;
        self.dirty = true;
    }

    /// Returns `true` when GPU sync is required.
    pub fn is_dirty(&self) -> bool {
        self.dirty
    }

    /// Clears the dirty flag after upload.
    pub fn clear_dirty(&mut self) {
        self.dirty = false;
    }

    /// Counts distinct values stored in the chunk (palette size).
    pub fn palette_len(&self) -> usize
    where
        T: PartialEq,
    {
        let mut uniq = Vec::new();
        for v in &self.cells {
            if !uniq.contains(v) {
                uniq.push(v.clone());
            }
        }
        uniq.len()
    }
}

/// Palette-compressed chunk used by authoring tools.
#[derive(Clone, Debug)]
pub struct PaletteVoxelChunk<T: Clone + Eq + std::hash::Hash> {
    size: u32,
    palette: Vec<T>,
    indices: Vec<u8>,
    _dirty: bool,
}

impl<T: Clone + Default + Eq + std::hash::Hash> PaletteVoxelChunk<T> {
    /// Builds a palette chunk from dense samples.
    pub fn from_samples(size: u32, samples: Vec<T>) -> Self {
        assert_eq!(samples.len(), (size as usize).pow(3));
        let mut palette = Vec::new();
        let mut indices = Vec::with_capacity(samples.len());
        for s in samples {
            let idx = match palette.iter().position(|p| *p == s) {
                Some(i) => i,
                None => {
                    palette.push(s.clone());
                    palette.len() - 1
                }
            };
            let idx_u8 = u8::try_from(idx).expect("palette fits in u8");
            indices.push(idx_u8);
        }
        Self {
            size,
            palette,
            indices,
            _dirty: false,
        }
    }

    /// Number of palette entries.
    pub fn palette_entries(&self) -> usize {
        self.palette.len()
    }

    /// Sample value at local voxel coordinates.
    pub fn get_local(&self, local: VoxelCoord) -> Option<T> {
        let i = (local.z as usize)
            .saturating_mul(self.size as usize)
            .saturating_mul(self.size as usize)
            + (local.y as usize).saturating_mul(self.size as usize)
            + local.x as usize;
        let id = *self.indices.get(i)? as usize;
        self.palette.get(id).cloned()
    }
}

/// Runtime sparse voxel volume.
#[derive(Clone, Debug)]
pub struct VoxelVolume<T> {
    /// Volume dimensions in voxels.
    pub dimensions: VoxelCoord,
    /// Cubic chunk edge length in voxels.
    pub chunk_size: u32,
    chunks: HashMap<u64, VoxelChunk<T>>,
}

impl<T: Clone + Default> VoxelVolume<T> {
    /// Creates an empty sparse volume.
    pub fn new(dimensions: VoxelCoord, chunk_size: u32) -> Self {
        assert!(chunk_size > 0);
        let mut chunks = HashMap::new();
        let voxels = (dimensions.x as u64)
            .saturating_mul(dimensions.y as u64)
            .saturating_mul(dimensions.z as u64);
        if voxels <= 1_000_000 {
            let cx = dimensions.x.div_ceil(chunk_size);
            let cy = dimensions.y.div_ceil(chunk_size);
            let cz = dimensions.z.div_ceil(chunk_size);
            for z in 0..cz {
                for y in 0..cy {
                    for x in 0..cx {
                        let c = ChunkCoord { x, y, z };
                        chunks.insert(pack_chunk(c), VoxelChunk::new(chunk_size));
                    }
                }
            }
        }
        Self {
            dimensions,
            chunk_size,
            chunks,
        }
    }

    fn in_bounds(&self, coord: VoxelCoord) -> bool {
        coord.x < self.dimensions.x && coord.y < self.dimensions.y && coord.z < self.dimensions.z
    }

    fn split(&self, coord: VoxelCoord) -> (ChunkCoord, VoxelCoord) {
        let cs = self.chunk_size;
        let cx = coord.x / cs;
        let cy = coord.y / cs;
        let cz = coord.z / cs;
        let chunk = ChunkCoord {
            x: cx,
            y: cy,
            z: cz,
        };
        let local = VoxelCoord {
            x: coord.x - cx * cs,
            y: coord.y - cy * cs,
            z: coord.z - cz * cs,
        };
        (chunk, local)
    }

    /// Returns a voxel reference.
    pub fn get(&self, coord: VoxelCoord) -> Option<&T> {
        if !self.in_bounds(coord) {
            return None;
        }
        let (chunk, local) = self.split(coord);
        let key = pack_chunk(chunk);
        self.chunks.get(&key)?.get_local(local)
    }

    /// Writes a voxel, allocating its chunk on demand.
    pub fn set(&mut self, coord: VoxelCoord, value: T) {
        assert!(
            self.in_bounds(coord),
            "set out of bounds: {:?} / {:?}",
            coord,
            self.dimensions
        );
        let (chunk, local) = self.split(coord);
        let key = pack_chunk(chunk);
        let entry = self
            .chunks
            .entry(key)
            .or_insert_with(|| VoxelChunk::new(self.chunk_size));
        entry.set_local(local, value);
    }

    /// Lists dirty chunk coordinates.
    pub fn dirty_chunks(&self) -> Vec<ChunkCoord> {
        let mut out = Vec::new();
        for (&key, chunk) in &self.chunks {
            if chunk.is_dirty() {
                out.push(unpack_chunk_key(key));
            }
        }
        out
    }

    /// Clears dirty flags on every allocated chunk.
    pub fn clear_dirty(&mut self) {
        for c in self.chunks.values_mut() {
            c.clear_dirty();
        }
    }

    /// Maps a voxel coordinate to its chunk coordinate.
    pub fn chunk_at(&self, coord: VoxelCoord) -> Option<ChunkCoord> {
        if !self.in_bounds(coord) {
            return None;
        }
        let (c, _) = self.split(coord);
        Some(c)
    }

    /// DDA raycast through solid voxels.
    pub fn raycast(
        &self,
        origin: Vec3,
        dir: Vec3,
        max_dist: f32,
        solid: impl Fn(&T) -> bool,
    ) -> Option<VoxelHit> {
        if dir.length_squared() < f32::EPSILON {
            return None;
        }
        let dir = dir.normalize();
        if dir.y.abs() < f32::EPSILON && dir.z.abs() < f32::EPSILON {
            return self.raycast_axis_x(origin, dir, max_dist, solid);
        }
        self.raycast_generic(origin, dir, max_dist, solid)
    }

    fn raycast_axis_x(
        &self,
        origin: Vec3,
        dir: Vec3,
        max_dist: f32,
        solid: impl Fn(&T) -> bool,
    ) -> Option<VoxelHit> {
        let step = dir.x.signum() as i32;
        if step == 0 {
            return None;
        }
        let y0 = origin.y.floor() as i32;
        let z0 = origin.z.floor() as i32;
        if y0 < 0 || z0 < 0 || y0 >= self.dimensions.y as i32 || z0 >= self.dimensions.z as i32 {
            return None;
        }
        let y = y0 as u32;
        let z = z0 as u32;
        let mut x = origin.x.floor() as i32;
        if step > 0 {
            if x < 0 {
                x = 0;
            }
            while (x as u32) < self.dimensions.x {
                let coord = VoxelCoord { x: x as u32, y, z };
                let hit = match self.get(coord) {
                    Some(v) => solid(v),
                    None => false,
                };
                if hit {
                    let t_hit = ((coord.x as f32) - origin.x) / dir.x;
                    let t_hit = t_hit.max(0.0);
                    if t_hit <= max_dist {
                        let pos = origin + dir * t_hit;
                        return Some(VoxelHit {
                            coord,
                            position: pos,
                            normal: Vec3::new(-(step.signum() as f32), 0.0, 0.0),
                            distance: t_hit,
                        });
                    }
                }
                x += 1;
            }
        } else {
            if x >= self.dimensions.x as i32 {
                x = self.dimensions.x as i32 - 1;
            }
            while x >= 0 {
                let coord = VoxelCoord { x: x as u32, y, z };
                let hit = match self.get(coord) {
                    Some(v) => solid(v),
                    None => false,
                };
                if hit {
                    let t_hit = ((coord.x as f32) + 1.0 - origin.x) / dir.x;
                    let t_hit = t_hit.max(0.0);
                    if t_hit <= max_dist {
                        let pos = origin + dir * t_hit;
                        return Some(VoxelHit {
                            coord,
                            position: pos,
                            normal: Vec3::new(-(step.signum() as f32), 0.0, 0.0),
                            distance: t_hit,
                        });
                    }
                }
                x -= 1;
            }
        }
        None
    }

    fn raycast_generic(
        &self,
        origin: Vec3,
        dir: Vec3,
        max_dist: f32,
        solid: impl Fn(&T) -> bool,
    ) -> Option<VoxelHit> {
        let mut t = 0.0f32;
        let mut gx = origin.x.floor() as i32;
        let mut gy = origin.y.floor() as i32;
        let mut gz = origin.z.floor() as i32;
        let step_x = dir.x.signum() as i64;
        let step_y = dir.y.signum() as i64;
        let step_z = dir.z.signum() as i64;
        let inv_x = if dir.x.abs() < f32::EPSILON {
            f32::INFINITY
        } else {
            1.0 / dir.x
        };
        let inv_y = if dir.y.abs() < f32::EPSILON {
            f32::INFINITY
        } else {
            1.0 / dir.y
        };
        let inv_z = if dir.z.abs() < f32::EPSILON {
            f32::INFINITY
        } else {
            1.0 / dir.z
        };
        let mut t_max_x = if dir.x > 0.0 {
            ((gx + 1) as f32 - origin.x) * inv_x
        } else if dir.x < 0.0 {
            (gx as f32 - origin.x) * inv_x
        } else {
            f32::INFINITY
        };
        let mut t_max_y = if dir.y > 0.0 {
            ((gy + 1) as f32 - origin.y) * inv_y
        } else if dir.y < 0.0 {
            (gy as f32 - origin.y) * inv_y
        } else {
            f32::INFINITY
        };
        let mut t_max_z = if dir.z > 0.0 {
            ((gz + 1) as f32 - origin.z) * inv_z
        } else if dir.z < 0.0 {
            (gz as f32 - origin.z) * inv_z
        } else {
            f32::INFINITY
        };
        let t_delta_x = inv_x.abs();
        let t_delta_y = inv_y.abs();
        let t_delta_z = inv_z.abs();
        while t <= max_dist {
            if gx >= 0
                && gy >= 0
                && gz >= 0
                && (gx as u32) < self.dimensions.x
                && (gy as u32) < self.dimensions.y
                && (gz as u32) < self.dimensions.z
            {
                let coord = VoxelCoord {
                    x: gx as u32,
                    y: gy as u32,
                    z: gz as u32,
                };
                let hit = match self.get(coord) {
                    Some(v) => solid(v),
                    None => false,
                };
                if hit {
                    let pos = origin + dir * t;
                    return Some(VoxelHit {
                        coord,
                        position: pos,
                        normal: -dir,
                        distance: t,
                    });
                }
            }
            if t_max_x < t_max_y {
                if t_max_x < t_max_z {
                    gx += step_x as i32;
                    t = t_max_x;
                    t_max_x += t_delta_x;
                } else {
                    gz += step_z as i32;
                    t = t_max_z;
                    t_max_z += t_delta_z;
                }
            } else if t_max_y < t_max_z {
                gy += step_y as i32;
                t = t_max_y;
                t_max_y += t_delta_y;
            } else {
                gz += step_z as i32;
                t = t_max_z;
                t_max_z += t_delta_z;
            }
        }
        None
    }
}

/// Hit information returned by `VoxelVolume::raycast`.
#[derive(Clone, Debug, PartialEq)]
pub struct VoxelHit {
    /// Voxel index that was hit.
    pub coord: VoxelCoord,
    /// World-space hit position.
    pub position: Vec3,
    /// Approximate face normal.
    pub normal: Vec3,
    /// Distance from ray origin.
    pub distance: f32,
}

#[cfg(test)]
mod tests {
    use std::time::Instant;

    use glam::Vec3;

    use super::*;

    #[test]
    fn test_voxel_get_valid() {
        let vol = VoxelVolume::<u8>::new(
            VoxelCoord {
                x: 16,
                y: 16,
                z: 16,
            },
            8,
        );
        assert_eq!(vol.get(VoxelCoord { x: 8, y: 8, z: 8 }), Some(&0));
    }

    #[test]
    fn test_voxel_get_out_of_bounds() {
        let vol = VoxelVolume::<u8>::new(
            VoxelCoord {
                x: 16,
                y: 16,
                z: 16,
            },
            8,
        );
        assert_eq!(vol.get(VoxelCoord { x: 16, y: 0, z: 0 }), None);
    }

    #[test]
    fn test_voxel_set_valid() {
        let mut vol = VoxelVolume::<u8>::new(
            VoxelCoord {
                x: 16,
                y: 16,
                z: 16,
            },
            8,
        );
        vol.set(VoxelCoord { x: 5, y: 5, z: 5 }, 3);
        assert_eq!(vol.get(VoxelCoord { x: 5, y: 5, z: 5 }), Some(&3));
    }

    #[test]
    fn test_voxel_raycast_hit() {
        let mut vol = VoxelVolume::<u8>::new(
            VoxelCoord {
                x: 16,
                y: 16,
                z: 16,
            },
            8,
        );
        vol.set(VoxelCoord { x: 8, y: 8, z: 8 }, 1);
        let hit = vol.raycast(Vec3::new(0.5, 8.5, 8.5), Vec3::X, 32.0, |v| *v != 0);
        assert_eq!(hit.unwrap().coord, VoxelCoord { x: 8, y: 8, z: 8 });
    }

    #[test]
    fn test_voxel_raycast_edge() {
        let mut vol = VoxelVolume::<u8>::new(VoxelCoord { x: 4, y: 4, z: 4 }, 2);
        vol.set(VoxelCoord { x: 0, y: 0, z: 0 }, 1);
        let hit = vol.raycast(
            Vec3::new(0.1, 0.1, 0.1),
            Vec3::new(1.0, 0.0, 0.0).normalize(),
            10.0,
            |v| *v != 0,
        );
        assert_eq!(hit.unwrap().coord, VoxelCoord { x: 0, y: 0, z: 0 });
    }

    #[test]
    fn test_voxel_raycast_miss() {
        let vol = VoxelVolume::<u8>::new(
            VoxelCoord {
                x: 16,
                y: 16,
                z: 16,
            },
            8,
        );
        assert!(vol
            .raycast(Vec3::new(0.5, 8.5, 8.5), Vec3::X, 32.0, |v| *v != 0)
            .is_none());
    }

    #[test]
    fn test_chunk_dirty_on_write() {
        let mut chunk = VoxelChunk::<u8>::new(4);
        assert!(!chunk.is_dirty());
        chunk.set_local(VoxelCoord { x: 0, y: 0, z: 0 }, 1);
        assert!(chunk.is_dirty());
    }

    #[test]
    fn test_chunk_clear_dirty() {
        let mut chunk = VoxelChunk::<u8>::new(4);
        chunk.set_local(VoxelCoord { x: 0, y: 0, z: 0 }, 1);
        chunk.clear_dirty();
        assert!(!chunk.is_dirty());
    }

    #[test]
    fn test_voxel_chunk_lifecycle() {
        let mut vol = VoxelVolume::<u8>::new(
            VoxelCoord {
                x: 16,
                y: 16,
                z: 16,
            },
            8,
        );
        vol.set(VoxelCoord { x: 1, y: 1, z: 1 }, 9);
        assert!(!vol.dirty_chunks().is_empty());
        vol.clear_dirty();
        assert!(vol.dirty_chunks().is_empty());
        vol.set(VoxelCoord { x: 1, y: 1, z: 1 }, 2);
        assert!(!vol.dirty_chunks().is_empty());
    }

    #[test]
    fn test_voxel_raycast_512_budget() {
        let mut vol = VoxelVolume::<u8>::new(
            VoxelCoord {
                x: 512,
                y: 512,
                z: 512,
            },
            32,
        );
        vol.set(
            VoxelCoord {
                x: 256,
                y: 256,
                z: 256,
            },
            1,
        );
        let start = Instant::now();
        let hit = vol.raycast(Vec3::new(0.5, 256.5, 256.5), Vec3::X, 600.0, |v| *v != 0);
        let ms = start.elapsed().as_secs_f64() * 1000.0;
        let limit = if cfg!(debug_assertions) { 50.0 } else { 2.0 };
        assert_eq!(
            hit.unwrap().coord,
            VoxelCoord {
                x: 256,
                y: 256,
                z: 256
            }
        );
        assert!(ms < limit, "raycast too slow: {ms} ms");
    }

    #[test]
    fn test_voxel_volume_chunk_palette() {
        let size = 8u32;
        let len = (size as usize).pow(3);
        let a = vec![1u8; len];
        let mut b = vec![2u8; len / 2];
        b.extend(vec![3u8; len / 2]);
        let chunk_a = PaletteVoxelChunk::from_samples(size, a);
        let chunk_b = PaletteVoxelChunk::from_samples(size, b);
        assert_eq!(chunk_a.palette_entries(), 1);
        assert_eq!(chunk_b.palette_entries(), 2);
    }
}
