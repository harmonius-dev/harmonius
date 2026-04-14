//! Procedural delta saves (TC-13.3.6.6 / TC-13.3.6.7).

use std::collections::BTreeMap;

/// Fixed-size base terrain with sparse edits (no `HashMap` in hot paths).
#[derive(Clone, Debug, Default)]
pub struct VoxelTerrainState {
    pub base: Vec<u8>,
    pub delta: BTreeMap<u32, u8>,
}

impl VoxelTerrainState {
    /// Serialize only changed voxels (sorted by index).
    pub fn save_delta(&self) -> Vec<u8> {
        let mut out = Vec::new();
        for (&idx, &v) in self.delta.iter() {
            out.extend_from_slice(&idx.to_le_bytes());
            out.push(v);
        }
        out
    }

    /// Apply a delta onto a clone of `base`.
    pub fn load_from_base(base: Vec<u8>, delta: &[u8]) -> Result<Self, &'static str> {
        let mut m = BTreeMap::new();
        let mut i = 0usize;
        while i < delta.len() {
            if i + 5 > delta.len() {
                return Err("truncated voxel delta");
            }
            let idx = u32::from_le_bytes(delta[i..i + 4].try_into().unwrap());
            let v = delta[i + 4];
            m.insert(idx, v);
            i += 5;
        }
        Ok(Self { base, delta: m })
    }

    /// Materialized voxel grid view (for assertions).
    pub fn materialize(&self, len: usize) -> Vec<u8> {
        let mut out = self.base.clone();
        if out.len() < len {
            out.resize(len, 0);
        }
        for (&i, &v) in self.delta.iter() {
            let ii = i as usize;
            if ii < out.len() {
                out[ii] = v;
            }
        }
        out
    }
}

/// Mesh deformation stored as vertex offset triples.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct MeshDeformState {
    pub offsets: Vec<(u32, f32, f32, f32)>,
}

impl MeshDeformState {
    /// Serialize as packed `(idx, dx, dy, dz)` little-endian floats.
    pub fn to_bytes(&self) -> Vec<u8> {
        let mut out = Vec::new();
        for &(idx, dx, dy, dz) in &self.offsets {
            out.extend_from_slice(&idx.to_le_bytes());
            out.extend_from_slice(&dx.to_le_bytes());
            out.extend_from_slice(&dy.to_le_bytes());
            out.extend_from_slice(&dz.to_le_bytes());
        }
        out
    }

    /// Parse [`MeshDeformState::to_bytes`] output.
    pub fn from_bytes(data: &[u8]) -> Result<Self, &'static str> {
        if !data.len().is_multiple_of(16) {
            return Err("bad mesh deform length");
        }
        let mut offsets = Vec::new();
        let mut i = 0usize;
        while i < data.len() {
            let idx = u32::from_le_bytes(data[i..i + 4].try_into().unwrap());
            let dx = f32::from_le_bytes(data[i + 4..i + 8].try_into().unwrap());
            let dy = f32::from_le_bytes(data[i + 8..i + 12].try_into().unwrap());
            let dz = f32::from_le_bytes(data[i + 12..i + 16].try_into().unwrap());
            offsets.push((idx, dx, dy, dz));
            i += 16;
        }
        Ok(Self { offsets })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// TC-13.3.6.6 Voxel delta save only touches changed cells.
    #[test]
    fn tc_13_3_6_6_voxel_delta() {
        let base = vec![0u8; 10_000];
        let mut state = VoxelTerrainState {
            base: base.clone(),
            delta: BTreeMap::new(),
        };
        for i in 0..100u32 {
            state.delta.insert(i * 10, 9);
        }
        let delta = state.save_delta();
        assert_eq!(delta.len(), 100 * 5);
        let restored = VoxelTerrainState::load_from_base(base, &delta).unwrap();
        assert_eq!(restored.materialize(10_000), state.materialize(10_000));
    }

    /// TC-13.3.6.7 Mesh deformation roundtrip.
    #[test]
    fn tc_13_3_6_7_mesh_deform() {
        let mesh = MeshDeformState {
            offsets: vec![(0, 0.1, -0.2, 0.05), (3, 1.0, 0.0, 0.0)],
        };
        let b = mesh.to_bytes();
        assert_eq!(MeshDeformState::from_bytes(&b).unwrap(), mesh);
    }
}
