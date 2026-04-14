//! Recast-style NavMesh tile generation (simplified heightfield + erosion for unit tests).

use glam::{Vec2, Vec3};

use super::links::{LinkDirection, OffMeshLink};
use super::tile::{NavMeshTile, NavPoly};
use super::types::{AnimTag, AreaType, LayerId, PolyFlags, PolyRef, TileCoord, TileKey};
/// Agent dimensions for one NavMesh layer.
#[derive(Clone, Debug)]
pub struct NavMeshAgentConfig {
    /// Capsule radius (meters).
    pub radius: f32,
    /// Capsule height (meters).
    pub height: f32,
    /// Maximum step height (meters).
    pub max_climb: f32,
    /// Maximum walkable slope in degrees from horizontal.
    pub max_slope_degrees: f32,
}

/// Build-time parameters for voxelization and meshing.
#[derive(Clone, Debug)]
pub struct NavMeshBuildConfig {
    /// XZ cell size for the heightfield (meters).
    pub cell_size: f32,
    /// Vertical quantization (meters).
    pub cell_height: f32,
    /// World tile edge length (meters).
    pub tile_size: f32,
    /// One config entry per layer id (index = `LayerId.0`).
    pub agent_configs: Vec<NavMeshAgentConfig>,
    /// Minimum region area in cells² (pruning threshold).
    pub min_region_area: u32,
    /// Merge threshold for small regions.
    pub merge_region_area: u32,
    /// Maximum simplified edge length (unused in stub mesher).
    pub max_edge_len: f32,
    /// Contour simplification tolerance (unused).
    pub max_simplification_error: f32,
    /// Maximum vertices per polygon.
    pub max_verts_per_poly: u8,
}

impl Default for NavMeshBuildConfig {
    fn default() -> Self {
        Self {
            cell_size: 0.3,
            cell_height: 0.2,
            tile_size: 64.0,
            agent_configs: vec![NavMeshAgentConfig {
                radius: 0.4,
                height: 1.8,
                max_climb: 0.45,
                max_slope_degrees: 45.0,
            }],
            min_region_area: 8,
            merge_region_area: 20,
            max_edge_len: 12.0,
            max_simplification_error: 1.3,
            max_verts_per_poly: 6,
        }
    }
}

/// Indexed triangle soup in world space.
#[derive(Clone, Debug, Default)]
pub struct InputGeometry {
    /// Unique vertices.
    pub vertices: Vec<Vec3>,
    /// Triangle corner indices.
    pub indices: Vec<u32>,
    /// Per-triangle area type.
    pub area_types: Vec<AreaType>,
}

/// Generation failures.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum NavMeshGenError {
    /// No walkable span remained after filtering.
    EmptyTile,
    /// Layer id out of range for `agent_configs`.
    InvalidLayer,
    /// Degenerate input.
    InvalidGeometry,
}

/// NavMesh tile builder.
#[derive(Clone, Debug)]
pub struct NavMeshGenerator {
    config: NavMeshBuildConfig,
}

impl NavMeshGenerator {
    /// Creates a generator bound to `config`.
    #[must_use]
    pub fn new(config: NavMeshBuildConfig) -> Self {
        Self { config }
    }

    /// World-space edge length of one streaming tile (meters).
    #[must_use]
    pub fn tile_size(&self) -> f32 {
        self.config.tile_size
    }

    /// Generates one tile for `coord` / `layer` from `geometry`.
    pub fn generate_tile(
        &self,
        coord: TileCoord,
        layer: LayerId,
        geometry: &InputGeometry,
    ) -> Result<NavMeshTile, NavMeshGenError> {
        let agent = self
            .config
            .agent_configs
            .get(usize::from(layer.0))
            .ok_or(NavMeshGenError::InvalidLayer)?;
        if !geometry.indices.len().is_multiple_of(3) {
            return Err(NavMeshGenError::InvalidGeometry);
        }
        let tile_size = self.config.tile_size;
        let cell = self.config.cell_size.max(0.05);
        let x0 = coord.x as f32 * tile_size;
        let z0 = coord.z as f32 * tile_size;
        let nx = (tile_size / cell).ceil().max(1.0) as usize;
        let nz = (tile_size / cell).ceil().max(1.0) as usize;
        let mut walkable = vec![false; nx * nz];
        let mut height_at = vec![0.0f32; nx * nz];
        for iz in 0..nz {
            for ix in 0..nx {
                let cx = x0 + (ix as f32 + 0.5) * cell;
                let cz = z0 + (iz as f32 + 0.5) * cell;
                if let Some((h, n, _area)) = sample_geometry(geometry, cx, cz) {
                    let slope_deg = rad_to_deg(n.y.clamp(-1.0, 1.0).acos());
                    if slope_deg <= agent.max_slope_degrees + 1e-3 {
                        walkable[iz * nx + ix] = true;
                        height_at[iz * nx + ix] = h;
                    }
                }
            }
        }
        erode_walkable_grid(
            &mut walkable,
            nx,
            nz,
            (agent.radius / cell).ceil().max(1.0) as i32,
        );
        prune_small_regions(&mut walkable, nx, nz, self.config.min_region_area as usize);
        let polys = build_rect_polys_from_components(
            &walkable,
            &height_at,
            nx,
            nz,
            x0,
            z0,
            cell,
            self.config.max_verts_per_poly,
        )?;
        if polys.is_empty() {
            return Err(NavMeshGenError::EmptyTile);
        }
        let mut vertices = Vec::new();
        let mut polygons = Vec::new();
        for p in polys {
            let base = vertices.len() as u16;
            if p.corners.len() > usize::from(self.config.max_verts_per_poly) {
                return Err(NavMeshGenError::InvalidGeometry);
            }
            for c in &p.corners {
                vertices.push(*c);
            }
            let mut idxs = smallvec::SmallVec::<[u16; 6]>::new();
            for i in 0..p.corners.len() {
                idxs.push(base + i as u16);
            }
            let area = p.area_type;
            polygons.push(NavPoly {
                vertex_indices: idxs,
                area_type: area,
                neighbors: smallvec::SmallVec::new(),
                flags: PolyFlags::WALKABLE,
            });
        }
        wire_neighbors_within_tile(&mut polygons, &vertices, coord, layer);
        let bounds_min = Vec3::new(
            x0,
            vertices.iter().map(|v| v.y).fold(f32::INFINITY, f32::min),
            z0,
        );
        let bounds_max = Vec3::new(
            x0 + tile_size,
            vertices
                .iter()
                .map(|v| v.y)
                .fold(f32::NEG_INFINITY, f32::max),
            z0 + tile_size,
        );
        Ok(NavMeshTile {
            coord,
            layer,
            bounds_min,
            bounds_max,
            polygons,
            vertices,
            detail_meshes: vec![],
            links: Vec::new(),
        })
    }

    /// Generates every tile overlapping `geometry` bounds (single pass: one tile per coord).
    pub fn generate_region(
        &self,
        bounds_min: Vec3,
        bounds_max: Vec3,
        geometry: &InputGeometry,
    ) -> Result<Vec<NavMeshTile>, NavMeshGenError> {
        let ts = self.config.tile_size;
        let min_x = (bounds_min.x / ts).floor() as i32;
        let max_x = (bounds_max.x / ts).floor() as i32;
        let min_z = (bounds_min.z / ts).floor() as i32;
        let max_z = (bounds_max.z / ts).floor() as i32;
        let mut out = Vec::new();
        for x in min_x..=max_x {
            for z in min_z..=max_z {
                for layer_id in 0..self.config.agent_configs.len() {
                    let layer = LayerId(layer_id as u8);
                    if let Ok(tile) = self.generate_tile(TileCoord { x, z }, layer, geometry) {
                        out.push(tile);
                    }
                }
            }
        }
        Ok(out)
    }
}

struct PolyBuild {
    corners: Vec<Vec3>,
    area_type: AreaType,
}

fn sample_geometry(geom: &InputGeometry, x: f32, z: f32) -> Option<(f32, Vec3, AreaType)> {
    let mut best: Option<(f32, Vec3, AreaType, f32)> = None;
    for tri in 0..(geom.indices.len() / 3) {
        let i0 = geom.indices[tri * 3] as usize;
        let i1 = geom.indices[tri * 3 + 1] as usize;
        let i2 = geom.indices[tri * 3 + 2] as usize;
        let a = *geom.vertices.get(i0)?;
        let b = *geom.vertices.get(i1)?;
        let c = *geom.vertices.get(i2)?;
        let mut n = (b - a).cross(c - a);
        let len = n.length();
        if len < 1e-8 {
            continue;
        }
        n /= len;
        let nn = if n.y < 0.0 { -n } else { n };
        if point_in_tri_xz(a, b, c, Vec3::new(x, 0.0, z)) {
            let y = plane_y_at_xz(a, nn, x, z);
            let area = geom
                .area_types
                .get(tri)
                .copied()
                .unwrap_or(AreaType::Ground);
            let key = y; // prefer highest walkable support
            match best {
                None => best = Some((y, nn, area, key)),
                Some((_, _, _, bk)) if key > bk => best = Some((y, nn, area, key)),
                _ => {}
            }
        }
    }
    best.map(|(y, n, a, _)| (y, n, a))
}

fn plane_y_at_xz(p0: Vec3, n: Vec3, x: f32, z: f32) -> f32 {
    // n·(X - p0) = 0 -> ny*y = -nx*(x-p0x) - nz*(z-p0z)
    if n.y.abs() < 1e-5 {
        return p0.y;
    }
    p0.y - (n.x * (x - p0.x) + n.z * (z - p0.z)) / n.y
}

fn point_in_tri_xz(a: Vec3, b: Vec3, c: Vec3, p: Vec3) -> bool {
    let v0 = Vec2::new(c.x - a.x, c.z - a.z);
    let v1 = Vec2::new(b.x - a.x, b.z - a.z);
    let v2 = Vec2::new(p.x - a.x, p.z - a.z);
    let dot00 = v0.dot(v0);
    let dot01 = v0.dot(v1);
    let dot02 = v0.dot(v2);
    let dot11 = v1.dot(v1);
    let dot12 = v1.dot(v2);
    let den = dot00 * dot11 - dot01 * dot01;
    if den.abs() < 1e-12 {
        return false;
    }
    let inv = 1.0 / den;
    let u = (dot11 * dot02 - dot01 * dot12) * inv;
    let v = (dot00 * dot12 - dot01 * dot02) * inv;
    (u >= -1e-5) && (v >= -1e-5) && (u + v <= 1.0 + 1e-5)
}

fn erode_walkable_grid(w: &mut [bool], nx: usize, nz: usize, radius_cells: i32) {
    if radius_cells <= 0 {
        return;
    }
    let orig = w.to_vec();
    for iz in 0..nz {
        for ix in 0..nx {
            if !orig[iz * nx + ix] {
                continue;
            }
            let mut ok = true;
            'outer: for dz in -radius_cells..=radius_cells {
                for dx in -radius_cells..=radius_cells {
                    if dx * dx + dz * dz > radius_cells * radius_cells {
                        continue;
                    }
                    let nix = ix as i32 + dx;
                    let niz = iz as i32 + dz;
                    if nix < 0 || niz < 0 || nix >= nx as i32 || niz >= nz as i32 {
                        ok = false;
                        break 'outer;
                    }
                    if !orig[niz as usize * nx + nix as usize] {
                        ok = false;
                        break 'outer;
                    }
                }
            }
            w[iz * nx + ix] = ok;
        }
    }
}

fn prune_small_regions(w: &mut [bool], nx: usize, nz: usize, min_area: usize) {
    if min_area <= 1 {
        return;
    }
    let mut vis = vec![false; w.len()];
    for iz in 0..nz {
        for ix in 0..nx {
            let idx = iz * nx + ix;
            if !w[idx] || vis[idx] {
                continue;
            }
            let mut stack = vec![(ix, iz)];
            let mut cells = Vec::new();
            vis[idx] = true;
            while let Some((cx, cz)) = stack.pop() {
                cells.push((cx, cz));
                for (dx, dz) in [(0i32, 1i32), (0, -1), (1, 0), (-1, 0)] {
                    let nx2 = cx as i32 + dx;
                    let nz2 = cz as i32 + dz;
                    if nx2 < 0 || nz2 < 0 || nx2 >= nx as i32 || nz2 >= nz as i32 {
                        continue;
                    }
                    let nidx = nz2 as usize * nx + nx2 as usize;
                    if w[nidx] && !vis[nidx] {
                        vis[nidx] = true;
                        stack.push((nx2 as usize, nz2 as usize));
                    }
                }
            }
            if cells.len() < min_area {
                for (cx, cz) in cells {
                    w[cz * nx + cx] = false;
                }
            }
        }
    }
}

#[allow(clippy::too_many_arguments)] // Heightfield rect builder; split would obscure the algorithm.
fn build_rect_polys_from_components(
    walkable: &[bool],
    heights: &[f32],
    nx: usize,
    nz: usize,
    x0: f32,
    z0: f32,
    cell: f32,
    maxv: u8,
) -> Result<Vec<PolyBuild>, NavMeshGenError> {
    let mut vis = vec![false; walkable.len()];
    let mut out = Vec::new();
    for iz in 0..nz {
        for ix in 0..nx {
            let idx = iz * nx + ix;
            if !walkable[idx] || vis[idx] {
                continue;
            }
            let mut stack = vec![(ix, iz)];
            vis[idx] = true;
            let mut cells = Vec::new();
            let mut sum_y = 0.0f64;
            while let Some((cx, cz)) = stack.pop() {
                cells.push((cx, cz));
                sum_y += f64::from(heights[cz * nx + cx]);
                for (dx, dz) in [(0i32, 1i32), (0, -1), (1, 0), (-1, 0)] {
                    let nx2 = cx as i32 + dx;
                    let nz2 = cz as i32 + dz;
                    if nx2 < 0 || nz2 < 0 || nx2 >= nx as i32 || nz2 >= nz as i32 {
                        continue;
                    }
                    let nidx = nz2 as usize * nx + nx2 as usize;
                    if walkable[nidx] && !vis[nidx] {
                        vis[nidx] = true;
                        stack.push((nx2 as usize, nz2 as usize));
                    }
                }
            }
            let mut min_x = usize::MAX;
            let mut max_x = 0usize;
            let mut min_z = usize::MAX;
            let mut max_z = 0usize;
            for (cx, cz) in &cells {
                min_x = min_x.min(*cx);
                max_x = max_x.max(*cx);
                min_z = min_z.min(*cz);
                max_z = max_z.max(*cz);
            }
            let y = (sum_y / cells.len() as f64) as f32;
            let corners = vec![
                Vec3::new(x0 + min_x as f32 * cell, y, z0 + min_z as f32 * cell),
                Vec3::new(x0 + (max_x + 1) as f32 * cell, y, z0 + min_z as f32 * cell),
                Vec3::new(
                    x0 + (max_x + 1) as f32 * cell,
                    y,
                    z0 + (max_z + 1) as f32 * cell,
                ),
                Vec3::new(x0 + min_x as f32 * cell, y, z0 + (max_z + 1) as f32 * cell),
            ];
            if corners.len() > usize::from(maxv) {
                return Err(NavMeshGenError::InvalidGeometry);
            }
            out.push(PolyBuild {
                corners,
                area_type: AreaType::Ground,
            });
        }
    }
    Ok(out)
}

fn wire_neighbors_within_tile(
    polys: &mut [NavPoly],
    vertices: &[Vec3],
    coord: TileCoord,
    layer: LayerId,
) {
    let n_polys = polys.len();
    for i in 0..n_polys {
        let mut neigh: smallvec::SmallVec<[Option<PolyRef>; 6]> =
            smallvec::smallvec![None; polys[i].vertex_indices.len()];
        for j in 0..n_polys {
            if i == j {
                continue;
            }
            if shared_edge_xz(vertices, &polys[i].vertex_indices, &polys[j].vertex_indices) {
                for si in 0..neigh.len() {
                    if neigh[si].is_none() {
                        neigh[si] = Some(PolyRef {
                            tile: TileKey { coord, layer },
                            poly_index: j as u16,
                        });
                        break;
                    }
                }
            }
        }
        polys[i].neighbors = neigh;
    }
}

fn shared_edge_xz(
    verts: &[Vec3],
    a: &smallvec::SmallVec<[u16; 6]>,
    b: &smallvec::SmallVec<[u16; 6]>,
) -> bool {
    let tol = 1e-2f32;
    for i in 0..a.len() {
        let a0 = verts[usize::from(a[i])];
        let a1 = verts[usize::from(a[(i + 1) % a.len()])];
        for j in 0..b.len() {
            let b0 = verts[usize::from(b[j])];
            let b1 = verts[usize::from(b[(j + 1) % b.len()])];
            if (dist_xz(a0, b1) < tol && dist_xz(a1, b0) < tol)
                || (dist_xz(a0, b0) < tol && dist_xz(a1, b1) < tol)
            {
                return true;
            }
        }
    }
    false
}

fn dist_xz(p: Vec3, q: Vec3) -> f32 {
    (p.x - q.x).abs() + (p.z - q.z).abs()
}

fn rad_to_deg(r: f32) -> f32 {
    r * 180.0 / std::f32::consts::PI
}

/// Connects two tiles across a vertical jump with an auto-generated link (TC-7.1.11.2).
#[allow(dead_code)] // Used by editor tooling and integration tests outside this crate.
pub fn auto_link_ramp(
    from_tile: &NavMeshTile,
    from_poly: u16,
    to_tile: &NavMeshTile,
    to_poly: u16,
) -> OffMeshLink {
    let start = from_tile.polygon_center(from_poly).unwrap_or(Vec3::ZERO);
    let end = to_tile.polygon_center(to_poly).unwrap_or(Vec3::ZERO);
    OffMeshLink {
        start_pos: start,
        end_pos: end,
        start_poly: PolyRef {
            tile: TileKey {
                coord: from_tile.coord,
                layer: from_tile.layer,
            },
            poly_index: from_poly,
        },
        end_poly: PolyRef {
            tile: TileKey {
                coord: to_tile.coord,
                layer: to_tile.layer,
            },
            poly_index: to_poly,
        },
        radius: 0.5,
        cost: 1.0,
        direction: LinkDirection::Bidirectional,
        area_type: AreaType::Ground,
        animation_tag: AnimTag(0),
        precondition: None,
        ability_id: None,
    }
}

#[cfg(test)]
mod tests {
    #![allow(clippy::field_reassign_with_default)]

    use glam::Vec3;

    use super::*;
    use crate::navigation::types::TileCoord;

    fn plane_100x100() -> InputGeometry {
        let y = 0.0f32;
        InputGeometry {
            vertices: vec![
                Vec3::new(0.0, y, 0.0),
                Vec3::new(100.0, y, 0.0),
                Vec3::new(100.0, y, 100.0),
                Vec3::new(0.0, y, 100.0),
            ],
            indices: vec![0, 1, 3, 1, 2, 3],
            area_types: vec![AreaType::Ground; 2],
        }
    }

    /// TC-7.1.1.1 #1 — flat plane yields merged walkable surface.
    #[test]
    fn tc_7_1_1_1_flat_single_polygon() {
        let mut cfg = NavMeshBuildConfig::default();
        cfg.tile_size = 100.0;
        cfg.cell_size = 0.5;
        cfg.agent_configs[0].radius = 0.5;
        cfg.agent_configs[0].height = 2.0;
        let gen = NavMeshGenerator::new(cfg);
        let tile = gen
            .generate_tile(TileCoord { x: 0, z: 0 }, LayerId(0), &plane_100x100())
            .expect("tile");
        assert_eq!(tile.polygon_count(), 1);
    }

    /// TC-7.1.1.1 #2 — coplanar walkable polygons at y=0.
    #[test]
    fn tc_7_1_1_1_cell_size_polygons_coplanar() {
        let mut cfg = NavMeshBuildConfig::default();
        cfg.tile_size = 100.0;
        cfg.cell_size = 0.3;
        let gen = NavMeshGenerator::new(cfg);
        let tile = gen
            .generate_tile(TileCoord { x: 0, z: 0 }, LayerId(0), &plane_100x100())
            .expect("tile");
        assert!(tile.polygon_count() > 0);
        for v in &tile.vertices {
            assert!(v.y.abs() < 0.05, "y={}", v.y);
        }
    }

    fn ramp_geom(deg: f32) -> InputGeometry {
        let rad = deg.to_radians();
        let w = 80.0f32;
        let d = 64.0f32;
        let h = d * rad.tan();
        // ramp surface: triangle (0,0,0)-(w,0,0)-(w,h,d)
        InputGeometry {
            vertices: vec![
                Vec3::new(0.0, 0.0, 0.0),
                Vec3::new(w, 0.0, 0.0),
                Vec3::new(w, h, d),
            ],
            indices: vec![0, 1, 2],
            area_types: vec![AreaType::Ground],
        }
    }

    /// TC-7.1.1.2 #1 — slope steeper than max slope excluded.
    #[test]
    fn tc_7_1_1_2_slope_46_excluded() {
        let mut cfg = NavMeshBuildConfig::default();
        cfg.tile_size = 64.0;
        cfg.cell_size = 0.2;
        cfg.agent_configs[0].max_slope_degrees = 45.0;
        let gen = NavMeshGenerator::new(cfg);
        let res = gen.generate_tile(TileCoord { x: 0, z: 0 }, LayerId(0), &ramp_geom(46.0));
        assert_eq!(res, Err(NavMeshGenError::EmptyTile));
    }

    /// TC-7.1.1.2 #2 — shallower ramp included.
    #[test]
    fn tc_7_1_1_2_slope_44_included() {
        let mut cfg = NavMeshBuildConfig::default();
        cfg.tile_size = 64.0;
        cfg.cell_size = 0.2;
        cfg.agent_configs[0].max_slope_degrees = 45.0;
        let gen = NavMeshGenerator::new(cfg);
        let tile = gen
            .generate_tile(TileCoord { x: 0, z: 0 }, LayerId(0), &ramp_geom(44.0))
            .expect("tile");
        assert!(tile.polygon_count() > 0);
    }

    /// TC-7.1.1.3 — narrow corridor erosion vs wider agent.
    #[test]
    fn tc_7_1_1_3_corridor_width_vs_radius() {
        let mut geom = InputGeometry::default();
        let y = 0.0f32;
        let w = 1.0f32;
        let len = 64.0f32;
        // floor strip along Z between x=0 and x=w
        geom.vertices = vec![
            Vec3::new(0.0, y, 0.0),
            Vec3::new(w, y, 0.0),
            Vec3::new(w, y, len),
            Vec3::new(0.0, y, len),
        ];
        geom.indices = vec![0, 1, 3, 1, 2, 3];
        geom.area_types = vec![AreaType::Ground; 2];
        let mut cfg = NavMeshBuildConfig::default();
        cfg.tile_size = 64.0;
        cfg.cell_size = 0.05;
        cfg.min_region_area = 1;
        cfg.agent_configs[0].radius = 0.6;
        let gen = NavMeshGenerator::new(cfg.clone());
        assert_eq!(
            gen.generate_tile(TileCoord { x: 0, z: 0 }, LayerId(0), &geom),
            Err(NavMeshGenError::EmptyTile)
        );
        cfg.agent_configs[0].radius = 0.3;
        let gen2 = NavMeshGenerator::new(cfg);
        let tile = gen2
            .generate_tile(TileCoord { x: 0, z: 0 }, LayerId(0), &geom)
            .expect("tile");
        assert!(tile.polygon_count() > 0);
    }

    /// TC-7.1.11.2 — auto-link connects two tile polygons across height.
    #[test]
    fn tc_7_1_11_2_auto_link_ramp() {
        let mut cfg = NavMeshBuildConfig::default();
        cfg.tile_size = 20.0;
        cfg.cell_size = 0.5;
        let gen = NavMeshGenerator::new(cfg);
        let g0 = InputGeometry {
            vertices: vec![
                Vec3::new(0.0, 0.0, 0.0),
                Vec3::new(20.0, 0.0, 0.0),
                Vec3::new(20.0, 0.0, 20.0),
                Vec3::new(0.0, 0.0, 20.0),
            ],
            indices: vec![0, 1, 3, 1, 2, 3],
            area_types: vec![AreaType::Ground; 2],
        };
        let g1 = InputGeometry {
            vertices: vec![
                Vec3::new(0.0, 3.0, 20.0),
                Vec3::new(20.0, 3.0, 20.0),
                Vec3::new(20.0, 3.0, 40.0),
                Vec3::new(0.0, 3.0, 40.0),
            ],
            indices: vec![0, 1, 3, 1, 2, 3],
            area_types: vec![AreaType::Ground; 2],
        };
        let t0 = gen
            .generate_tile(TileCoord { x: 0, z: 0 }, LayerId(0), &g0)
            .expect("t0");
        let t1 = gen
            .generate_tile(TileCoord { x: 0, z: 1 }, LayerId(0), &g1)
            .expect("t1");
        let link = auto_link_ramp(&t0, 0, &t1, 0);
        assert!((link.start_pos.y - link.end_pos.y).abs() > 1.0);
    }
}
