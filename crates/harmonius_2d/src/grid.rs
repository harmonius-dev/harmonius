//! Isometric diamond and axial hex coordinate transforms.

use glam::{IVec2, Vec2};

/// World-space position of the top-left corner of tile `t` for diamond isometric (`TC-10.5.7.1`).
#[must_use]
pub fn iso_diamond_tile_to_screen(t: IVec2, tile_size: Vec2) -> Vec2 {
    let x = (t.x as f32 - t.y as f32) * 0.5 * tile_size.x;
    let y = (t.x as f32 + t.y as f32) * 0.5 * tile_size.y;
    Vec2::new(x, y)
}

/// Inverse of [`iso_diamond_tile_to_screen`].
#[must_use]
pub fn iso_diamond_screen_to_tile(screen: Vec2, tile_size: Vec2) -> IVec2 {
    let a = screen.x / (0.5 * tile_size.x);
    let b = screen.y / (0.5 * tile_size.y);
    let q = (0.5 * (a + b)).round() as i32;
    let r = (0.5 * (b - a)).round() as i32;
    IVec2::new(q, r)
}

/// Six axial neighbor coordinates for pointy-top hexes (`TC-10.5.7.2`).
///
/// Uses axial `(q, r)` with neighbor offsets from Red Blob Games (pointy-top).
#[must_use]
pub fn hex_neighbors_pointy_top_axial(cell: IVec2) -> [IVec2; 6] {
    let q = cell.x;
    let r = cell.y;
    [
        IVec2::new(q + 1, r),
        IVec2::new(q + 1, r - 1),
        IVec2::new(q, r - 1),
        IVec2::new(q - 1, r),
        IVec2::new(q - 1, r + 1),
        IVec2::new(q, r + 1),
    ]
}
