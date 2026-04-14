//! Stable interleaving of particles and sprites by depth.

/// Drawable kinds participating in a single 2D pass.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Drawable2d {
    /// Textured sprite instance.
    Sprite(u32),
    /// Particle instance.
    Particle(u32),
}

/// Sort drawables by ascending `z` (`TC-10.5.15.1`).
#[must_use]
pub fn sort_draw_order(items: &[(Drawable2d, f32)]) -> Vec<Drawable2d> {
    let mut idx: Vec<usize> = (0..items.len()).collect();
    idx.sort_by(|&a, &b| items[a].1.total_cmp(&items[b].1));
    idx.into_iter().map(|i| items[i].0).collect()
}
