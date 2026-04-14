//! Tilemap chunk culling and autotile helpers.

use glam::{IVec2, Vec2};

/// Minimal chunk description for culling tests.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct TilemapChunkDesc {
    /// Chunk coordinate in chunk space.
    pub chunk_pos: IVec2,
    /// Tile count along X inside the chunk.
    pub width: u32,
    /// Tile count along Y inside the chunk.
    pub height: u32,
}

/// Return chunk indices whose world AABB intersects the axis-aligned view rectangle (`TC-10.5.6.1`).
#[must_use]
pub fn cull_tilemap_chunks(
    chunks: &[TilemapChunkDesc],
    tile_size: Vec2,
    view_min: Vec2,
    view_max: Vec2,
) -> Vec<IVec2> {
    let mut out = Vec::new();
    for c in chunks {
        let origin = Vec2::new(
            c.chunk_pos.x as f32 * c.width as f32 * tile_size.x,
            c.chunk_pos.y as f32 * c.height as f32 * tile_size.y,
        );
        let size = Vec2::new(c.width as f32 * tile_size.x, c.height as f32 * tile_size.y);
        let cmin = origin;
        let cmax = origin + size;
        if cmax.x > view_min.x && cmin.x < view_max.x && cmax.y > view_min.y && cmin.y < view_max.y
        {
            out.push(c.chunk_pos);
        }
    }
    out.sort_by(|a, b| a.x.cmp(&b.x).then_with(|| a.y.cmp(&b.y)));
    out
}

use crate::types::TileId;

/// Cardinal-neighbor autotile: if all four neighbors match `match_id`, rewrite center to `out_id`.
pub fn apply_auto_tile_cardinal(
    tiles: &mut [TileId],
    width: u32,
    height: u32,
    match_id: TileId,
    out_id: TileId,
) {
    let w = width as usize;
    let h = height as usize;
    if w < 3 || h < 3 {
        return;
    }
    let cx = 1usize;
    let cy = 1usize;
    let idx = cy * w + cx;
    if tiles[idx] != match_id {
        return;
    }
    let n = tiles[(cy - 1) * w + cx] == match_id;
    let s = tiles[(cy + 1) * w + cx] == match_id;
    let e = tiles[cy * w + (cx + 1)] == match_id;
    let wst = tiles[cy * w + (cx - 1)] == match_id;
    if n && s && e && wst {
        tiles[idx] = out_id;
    }
}
