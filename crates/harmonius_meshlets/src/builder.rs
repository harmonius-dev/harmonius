use crate::asset::AssetId;
use crate::asset::BufferView;
use crate::asset::LodGroup;
use crate::asset::Meshlet;
use crate::asset::MeshletAsset;
use crate::mesh_input::NormalizedMesh;
use crate::topology::validate_manifold;
use bytemuck::bytes_of;
use bytemuck::cast_slice;
use glam::Vec3;
use meshopt::clusterize;
use meshopt::clusterize::PositionDataAdapter;
use meshopt::optimize;
use meshopt::remap;
use meshopt::simplify;
use meshopt::typed_to_bytes;
use meshopt::SimplifyOptions;
use meshopt::Vertex;
use meshopt::VertexDataAdapter;
use sha2::Digest;
use sha2::Sha256;
use std::mem::size_of;

/// Errors produced while constructing a [`MeshletAsset`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BuildError {
    /// Index buffer topology is not a manifold triangle soup.
    InvalidTopology,
    /// Mesh simplification could not produce another LOD (`TC-2.4.4.4`).
    SimplifyFailed {
        /// 1-based LOD index being built.
        level: u8,
        /// Stable diagnostic string.
        reason: &'static str,
    },
    /// Clustering exceeded configured meshlet capacity.
    VertexOverflow,
    /// Degenerate bounds (collapsed geometry).
    BoundsDegenerate,
}

impl std::fmt::Display for BuildError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            BuildError::InvalidTopology => write!(f, "invalid mesh topology for meshlet build"),
            BuildError::SimplifyFailed { level, reason } => {
                write!(f, "mesh simplification failed at LOD {level}: {reason}")
            }
            BuildError::VertexOverflow => write!(f, "meshlet vertex or triangle cap exceeded"),
            BuildError::BoundsDegenerate => write!(f, "mesh bounds are degenerate"),
        }
    }
}

impl std::error::Error for BuildError {}

/// Configures and runs the deterministic meshlet bake (`R-2.4.4`).
#[derive(Debug, Clone)]
pub struct MeshletBuilder {
    input: NormalizedMesh,
    max_vertices: u8,
    max_triangles: u8,
    simplify_lods: u8,
    cone_weight: f32,
    id: AssetId,
    version: u32,
}

impl MeshletBuilder {
    /// Creates a builder with Harmonius defaults (64 / 124 caps, no extra LODs).
    #[must_use]
    pub fn new(input: NormalizedMesh) -> Self {
        Self {
            input,
            max_vertices: 64,
            max_triangles: 124,
            simplify_lods: 0,
            cone_weight: 0.5,
            id: AssetId(0),
            version: 1,
        }
    }

    /// Overrides the asset id assigned to the output.
    #[must_use]
    pub fn id(mut self, id: AssetId) -> Self {
        self.id = id;
        self
    }

    /// Overrides the asset version field.
    #[must_use]
    pub fn version(mut self, version: u32) -> Self {
        self.version = version;
        self
    }

    /// Sets the maximum vertices per meshlet (≤ 64 for native mesh shaders).
    #[must_use]
    pub fn max_vertices(mut self, n: u8) -> Self {
        self.max_vertices = n;
        self
    }

    /// Sets the maximum triangles per meshlet (≤ 124, multiple of four for meshoptimizer).
    #[must_use]
    pub fn max_triangles(mut self, n: u8) -> Self {
        self.max_triangles = n;
        self
    }

    /// Number of simplified LOD levels **after** the base LOD (`R-2.4.3`).
    #[must_use]
    pub fn simplify_lods(mut self, n: u8) -> Self {
        self.simplify_lods = n;
        self
    }

    /// Cone weight forwarded to meshoptimizer (`R-2.4.8`).
    #[must_use]
    pub fn cone_weight(mut self, w: f32) -> Self {
        self.cone_weight = w;
        self
    }

    /// Runs remap → optimize → cluster → pack (`R-2.4.1`–`R-2.4.5`, `R-2.4.8`, `R-2.4.9`).
    #[must_use = "builder returns a new asset or error"]
    pub fn build(self) -> Result<MeshletAsset, BuildError> {
        if !self.input.is_attribute_aligned() {
            return Err(BuildError::InvalidTopology);
        }
        if self.input.positions.is_empty() {
            return Err(BuildError::InvalidTopology);
        }
        if self.input.indices.len() % 3 != 0 {
            return Err(BuildError::InvalidTopology);
        }
        validate_manifold(&self.input.indices)?;
        if self.max_vertices == 0 || self.max_triangles == 0 {
            return Err(BuildError::InvalidTopology);
        }
        if (self.max_triangles as usize) % 4 != 0 {
            return Err(BuildError::InvalidTopology);
        }

        if all_positions_identical(&self.input.positions) {
            return Err(BuildError::BoundsDegenerate);
        }

        let vertices = meshopt_vertices(&self.input);
        let (unique_count, remap) =
            remap::generate_vertex_remap(&vertices, Some(&self.input.indices));
        let remapped_indices =
            remap::remap_index_buffer(Some(&self.input.indices), vertices.len(), &remap);
        let compact_vertices: Vec<Vertex> =
            remap::remap_vertex_buffer(&vertices, unique_count, &remap);
        let optimized_indices = optimize::optimize_vertex_cache(&remapped_indices, unique_count);

        let adapter =
            VertexDataAdapter::new(typed_to_bytes(&compact_vertices), size_of::<Vertex>(), 0)
                .map_err(|_| BuildError::InvalidTopology)?;

        let mut lod_indices_chain: Vec<Vec<u32>> = Vec::new();
        lod_indices_chain.push(optimized_indices);

        let mut per_pass_error: Vec<f32> = Vec::new();

        for level in 1..=self.simplify_lods {
            let parent = lod_indices_chain
                .last()
                .ok_or(BuildError::InvalidTopology)?;
            if parent.len() < 12 {
                return Err(BuildError::SimplifyFailed {
                    level,
                    reason: "mesh too small to simplify further",
                });
            }
            let target_index_count = (parent.len() / 2 / 3) * 3;
            let target_index_count = target_index_count.max(3);
            let mut pass_error = 0.0f32;
            let simplified = simplify::simplify(
                parent,
                &adapter,
                target_index_count,
                1.0,
                SimplifyOptions::None,
                Some(&mut pass_error),
            );
            if simplified.len() < 3 || simplified.len() >= parent.len() {
                return Err(BuildError::SimplifyFailed {
                    level,
                    reason: "simplify did not reduce index count",
                });
            }
            per_pass_error.push(pass_error.max(1e-8));
            lod_indices_chain.push(optimize::optimize_vertex_cache(&simplified, unique_count));
        }

        let mut cumulative = vec![0.0f32];
        for e in &per_pass_error {
            let base = cumulative.last().copied().unwrap_or(0.0);
            cumulative.push(base + e);
        }

        let mut lod_groups: Vec<LodGroup> = Vec::new();
        let mut index_data: Vec<u8> = Vec::new();
        let mut meshlet_vertex_index_data: Vec<u8> = Vec::new();
        let mut meshlet_triangle_data: Vec<u8> = Vec::new();
        let mut all_meshlets: Vec<Meshlet> = Vec::new();

        for (lod_idx, indices) in lod_indices_chain.iter().enumerate() {
            let lod_index_byte_start = index_data.len() as u64;
            for tri in indices.chunks_exact(3) {
                for idx in tri {
                    index_data.extend_from_slice(&idx.to_le_bytes());
                }
            }
            let lod_index_byte_count = index_data.len() as u64 - lod_index_byte_start;

            let meshlets = clusterize::build_meshlets(
                indices,
                &adapter,
                self.max_vertices as usize,
                self.max_triangles as usize,
                self.cone_weight,
            );

            if meshlets.len() > u16::MAX as usize {
                return Err(BuildError::VertexOverflow);
            }

            let (lg_center, lg_radius) = lod_bounds_sphere(indices, &compact_vertices)?;
            let screen_error = *cumulative
                .get(lod_idx)
                .ok_or(BuildError::BoundsDegenerate)?;

            let mut harmonius_meshlets: Vec<Meshlet> = Vec::new();
            for mi in 0..meshlets.len() {
                let meshlet = meshlets.get(mi);
                let ffi = &meshlets.meshlets[mi];
                if ffi.vertex_count as usize > self.max_vertices as usize
                    || ffi.triangle_count as usize > self.max_triangles as usize
                {
                    return Err(BuildError::VertexOverflow);
                }
                let bounds = clusterize::compute_meshlet_bounds(meshlet, &adapter);
                let (center, radius, apex, axis, angle) = map_cone_bounds(bounds)?;

                let tri_byte_start = meshlet_triangle_data.len() as u32;
                meshlet_triangle_data.extend_from_slice(meshlet.triangles);

                let vertex_index_byte_start = meshlet_vertex_index_data.len() as u32;
                for &gv in meshlet.vertices {
                    meshlet_vertex_index_data.extend_from_slice(&gv.to_le_bytes());
                }

                harmonius_meshlets.push(Meshlet {
                    vertex_start: vertex_index_byte_start,
                    vertex_count: ffi.vertex_count as u8,
                    pad_vertex: [0; 3],
                    triangle_start: tri_byte_start,
                    triangle_count: ffi.triangle_count as u8,
                    pad_triangle: [0; 3],
                    cone_apex: apex.to_array(),
                    cone_axis: axis.to_array(),
                    cone_angle: angle,
                    bounds_center: center.to_array(),
                    bounds_radius: radius,
                    pad_tail: [0; 4],
                });
            }

            all_meshlets.extend(harmonius_meshlets.iter().copied());
            lod_groups.push(LodGroup {
                level: lod_idx as u8,
                screen_error,
                index_byte_start: lod_index_byte_start,
                index_byte_count: lod_index_byte_count,
                meshlets: harmonius_meshlets,
                bounds_center: lg_center.to_array(),
                bounds_radius: lg_radius,
            });
        }

        let meshlet_data = meshlet_structs_to_bytes(&all_meshlets);
        let index_count: u32 = lod_indices_chain
            .iter()
            .map(|ix| ix.len() as u32)
            .sum::<u32>();

        let vertex_bytes = typed_to_bytes(&compact_vertices).to_vec();
        let meshlet_stride = size_of::<Meshlet>();

        let vb = BufferView {
            offset: 0,
            size: vertex_bytes.len() as u64,
            stride: crate::vertex_stride_bytes(),
        };
        let ib = BufferView {
            offset: 0,
            size: index_data.len() as u64,
            stride: 4,
        };
        let mb = BufferView {
            offset: 0,
            size: meshlet_data.len() as u64,
            stride: meshlet_stride as u32,
        };
        let mvb = BufferView {
            offset: 0,
            size: meshlet_vertex_index_data.len() as u64,
            stride: 4,
        };
        let mtb = BufferView {
            offset: 0,
            size: meshlet_triangle_data.len() as u64,
            stride: 1,
        };

        Ok(MeshletAsset {
            id: self.id,
            version: self.version,
            vertex_count: unique_count as u32,
            index_count,
            lod_groups,
            vertex_buffer: vb,
            index_buffer: ib,
            meshlet_buffer: mb,
            meshlet_vertex_index_buffer: mvb,
            meshlet_triangle_buffer: mtb,
            source_hash: hash_mesh(&self.input),
            vertex_data: vertex_bytes,
            index_data,
            meshlet_data,
            meshlet_vertex_index_data,
            meshlet_triangle_data,
        })
    }
}

fn all_positions_identical(positions: &[Vec3]) -> bool {
    let first = positions[0];
    positions.iter().all(|p| *p == first)
}

fn meshopt_vertices(mesh: &NormalizedMesh) -> Vec<Vertex> {
    mesh.positions
        .iter()
        .enumerate()
        .map(|(i, p)| Vertex {
            p: p.to_array(),
            n: mesh.normals[i].to_array(),
            t: mesh.uvs[i].to_array(),
        })
        .collect()
}

fn map_cone_bounds(
    b: meshopt::clusterize::Bounds,
) -> Result<(Vec3, f32, Vec3, Vec3, f32), BuildError> {
    let center = Vec3::from_array(b.center);
    let radius = b.radius;
    if !radius.is_finite() {
        return Err(BuildError::BoundsDegenerate);
    }
    let radius = radius.max(1e-6);
    let axis = Vec3::from_array(b.cone_axis);
    let axis_n = if axis.length_squared() < 1e-20 {
        Vec3::Y
    } else {
        axis.normalize()
    };
    let apex = Vec3::from_array(b.cone_apex);
    let angle = if axis.length_squared() < 1e-20 {
        std::f32::consts::PI
    } else {
        b.cone_cutoff.clamp(-1.0, 1.0).acos()
    };
    if !angle.is_finite() {
        return Err(BuildError::BoundsDegenerate);
    }
    Ok((center, radius, apex, axis_n, angle))
}

fn lod_bounds_sphere(indices: &[u32], compact: &[Vertex]) -> Result<(Vec3, f32), BuildError> {
    let mut used: Vec<u32> = indices.to_vec();
    used.sort_unstable();
    used.dedup();
    if used.len() == 1 {
        let p = Vec3::from(
            compact
                .get(used[0] as usize)
                .ok_or(BuildError::BoundsDegenerate)?
                .p,
        );
        return Ok((p, 1e-6));
    }
    let mut pack: Vec<u8> = Vec::with_capacity(used.len() * 12);
    for &ix in &used {
        let v = compact
            .get(ix as usize)
            .ok_or(BuildError::BoundsDegenerate)?;
        pack.extend_from_slice(cast_slice(&v.p));
    }
    let pad = PositionDataAdapter {
        data: &pack,
        position_count: used.len(),
        position_stride: 12,
        position_offset: 0,
    };
    let s = clusterize::compute_sphere_bounds(pad, None);
    let center = Vec3::from_array(s.center);
    let r = s.radius.max(1e-6);
    if !r.is_finite() {
        return Err(BuildError::BoundsDegenerate);
    }
    Ok((center, r))
}

fn hash_mesh(mesh: &NormalizedMesh) -> [u8; 32] {
    let mut h = Sha256::new();
    h.update(b"HARMONIUS_MESHLET_ASSET_V1");
    for p in &mesh.positions {
        for c in p.to_array() {
            h.update(c.to_le_bytes());
        }
    }
    for n in &mesh.normals {
        for c in n.to_array() {
            h.update(c.to_le_bytes());
        }
    }
    for uv in &mesh.uvs {
        for c in uv.to_array() {
            h.update(c.to_le_bytes());
        }
    }
    for i in &mesh.indices {
        h.update(i.to_le_bytes());
    }
    h.finalize().into()
}

fn meshlet_structs_to_bytes(meshlets: &[Meshlet]) -> Vec<u8> {
    meshlets
        .iter()
        .flat_map(|m| bytes_of(m).iter().copied())
        .collect()
}
