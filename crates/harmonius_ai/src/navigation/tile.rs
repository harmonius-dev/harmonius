//! NavMesh tile geometry: polygons, vertices, and local queries.

use glam::Vec3;
use smallvec::SmallVec;

use super::links::OffMeshLink;
use super::types::{AreaType, PolyFlags, PolyRef, TileKey};

/// Height detail for one polygon (optional refinement).
#[derive(Clone, Debug, Default, PartialEq)]
pub struct DetailMesh {
    /// Sample vertices in tile space.
    pub vertices: Vec<Vec3>,
    /// Indexed triangles.
    pub triangles: Vec<[u16; 3]>,
}

/// One convex polygon on a NavMesh tile.
#[derive(Clone, Debug, PartialEq)]
pub struct NavPoly {
    /// Indices into the parent tile vertex buffer.
    pub vertex_indices: SmallVec<[u16; 6]>,
    /// Area classification for costs.
    pub area_type: AreaType,
    /// Neighbor polygon refs per edge; `None` marks tile boundary / open edge.
    pub neighbors: SmallVec<[Option<PolyRef>; 6]>,
    /// Query and carve flags.
    pub flags: PolyFlags,
}

/// One streamable NavMesh chunk with polygons and optional off-mesh links.
#[derive(Clone, Debug, PartialEq)]
pub struct NavMeshTile {
    /// Tile coordinate in the streaming grid.
    pub coord: super::types::TileCoord,
    /// Agent-size layer.
    pub layer: super::types::LayerId,
    /// Axis-aligned bounds in world space (minimum corner).
    pub bounds_min: Vec3,
    /// Axis-aligned bounds in world space (maximum corner).
    pub bounds_max: Vec3,
    /// Convex polygons.
    pub polygons: Vec<NavPoly>,
    /// Shared vertex pool.
    pub vertices: Vec<Vec3>,
    /// Per-polygon detail meshes (aligned with `polygons`).
    pub detail_meshes: Vec<DetailMesh>,
    /// Off-mesh links registered on this tile.
    pub links: Vec<OffMeshLink>,
}

impl NavMeshTile {
    /// Returns the number of polygons in this tile.
    #[must_use]
    pub fn polygon_count(&self) -> u32 {
        self.polygons.len() as u32
    }

    /// Borrows a polygon by local index.
    #[must_use]
    pub fn polygon(&self, index: u16) -> Option<&NavPoly> {
        self.polygons.get(usize::from(index))
    }

    /// Centroid of polygon `index` in world space.
    #[must_use]
    pub fn polygon_center(&self, index: u16) -> Option<Vec3> {
        let poly = self.polygon(index)?;
        let mut acc = Vec3::ZERO;
        let mut n = 0u32;
        for &vi in poly.vertex_indices.iter() {
            let v = self.vertices.get(usize::from(vi))?;
            acc += *v;
            n += 1;
        }
        if n == 0 {
            return None;
        }
        Some(acc / n as f32)
    }

    /// True if `point` lies inside polygon `index` on the XZ plane (ignores Y for containment).
    #[must_use]
    pub fn point_in_polygon(&self, index: u16, point: Vec3) -> bool {
        let Some(poly) = self.polygon(index) else {
            return false;
        };
        let mut verts: SmallVec<[Vec3; 8]> = SmallVec::new();
        for &vi in poly.vertex_indices.iter() {
            if let Some(v) = self.vertices.get(usize::from(vi)) {
                verts.push(*v);
            }
        }
        point_in_poly_xz(&verts, point)
    }

    /// Nearest polygon to `pos` among this tile's polygons (XZ distance to centroid tie-break).
    #[must_use]
    pub fn find_nearest_poly(&self, pos: Vec3, _extent: Vec3) -> Option<(PolyRef, Vec3)> {
        let key = TileKey {
            coord: self.coord,
            layer: self.layer,
        };
        let mut best: Option<(PolyRef, Vec3, f32)> = None;
        for (i, _) in self.polygons.iter().enumerate() {
            let idx = u16::try_from(i).ok()?;
            let center = self.polygon_center(idx)?;
            let d = (Vec3::new(center.x, 0.0, center.z) - Vec3::new(pos.x, 0.0, pos.z))
                .length_squared();
            let pref = PolyRef {
                tile: key,
                poly_index: idx,
            };
            match best {
                None => best = Some((pref, center, d)),
                Some((_, _, bd)) if d < bd => best = Some((pref, center, d)),
                _ => {}
            }
        }
        best.map(|(p, c, _)| (p, c))
    }
}

/// Cast a segment on the XZ plane against polygon edges (returns first edge hit fraction).
#[must_use]
pub fn raycast_poly_xz(tile: &NavMeshTile, index: u16, from: Vec3, to: Vec3) -> Option<f32> {
    if !tile.point_in_polygon(index, from) {
        return None;
    }
    let dir = Vec3::new(to.x - from.x, 0.0, to.z - from.z);
    let len = dir.length();
    if len < 1e-6 {
        return Some(0.0);
    }
    let dir_n = dir / len;
    let mut best_t = 1.0f32;
    let poly = tile.polygon(index)?;
    let n = poly.vertex_indices.len();
    for i in 0..n {
        let a = *tile.vertices.get(usize::from(poly.vertex_indices[i]))?;
        let j = (i + 1) % n;
        let b = *tile.vertices.get(usize::from(poly.vertex_indices[j]))?;
        if let Some(t) = ray_segment_xz(from, dir_n, len, a, b) {
            if t < best_t {
                best_t = t;
            }
        }
    }
    Some(best_t)
}

fn point_in_poly_xz(verts: &[Vec3], p: Vec3) -> bool {
    if verts.len() < 3 {
        return false;
    }
    let mut inside = false;
    let mut j = verts.len() - 1;
    for i in 0..verts.len() {
        let vi = verts[i];
        let vj = verts[j];
        let intersect = ((vi.z > p.z) != (vj.z > p.z))
            && (p.x < (vj.x - vi.x) * (p.z - vi.z) / (vj.z - vi.z + f32::EPSILON) + vi.x);
        if intersect {
            inside = !inside;
        }
        j = i;
    }
    inside
}

fn ray_segment_xz(origin: Vec3, dir: Vec3, max_t: f32, a: Vec3, b: Vec3) -> Option<f32> {
    // 2D line-line intersection in XZ
    let ox = origin.x;
    let oz = origin.z;
    let dx = dir.x;
    let dz = dir.z;
    let x1 = a.x;
    let z1 = a.z;
    let x2 = b.x;
    let z2 = b.z;
    let den = dx * (z2 - z1) - dz * (x2 - x1);
    if den.abs() < 1e-8 {
        return None;
    }
    let t = ((x1 - ox) * (z2 - z1) - (z1 - oz) * (x2 - x1)) / den;
    if !(0.0..=max_t).contains(&t) {
        return None;
    }
    let u = ((x1 - ox) * dz - (z1 - oz) * dx) / den;
    if (0.0..=1.0).contains(&u) {
        Some(t)
    } else {
        None
    }
}
