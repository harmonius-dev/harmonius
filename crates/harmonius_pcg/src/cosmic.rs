//! Hierarchical cosmic addressing (128-bit) and sparse octree bookkeeping.

use glam::DVec3;

/// 128-bit hierarchical cell key (fixed-point quantization of a position inside a root cube).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct CosmicKey(pub u128);

const AXIS_BITS: u32 = 42;

fn quantize_axis(v: f64, root: f64) -> u128 {
    let t = (v / root).clamp(0.0, 1.0);
    let max = ((1u128 << AXIS_BITS) - 1) as f64;
    (t * max).round() as u128 & ((1u128 << AXIS_BITS) - 1)
}

fn dequantize_axis(q: u128, root: f64) -> f64 {
    let max = ((1u128 << AXIS_BITS) - 1) as f64;
    (q as f64 / max) * root
}

impl CosmicKey {
    /// Encodes a position inside `[0, root_extent_m)^3`.
    pub fn encode_position_m(p: DVec3, root_extent_m: f64) -> Self {
        let qx = quantize_axis(p.x, root_extent_m);
        let qy = quantize_axis(p.y, root_extent_m);
        let qz = quantize_axis(p.z, root_extent_m);
        Self((qx << 84) | (qy << 42) | qz)
    }

    /// Decodes quantized position (matches [`Self::encode_position_m`] within one cell).
    pub fn decode_position_m(self, root_extent_m: f64) -> DVec3 {
        let mask = (1u128 << AXIS_BITS) - 1;
        let qx = (self.0 >> 84) & mask;
        let qy = (self.0 >> 42) & mask;
        let qz = self.0 & mask;
        DVec3::new(
            dequantize_axis(qx, root_extent_m),
            dequantize_axis(qy, root_extent_m),
            dequantize_axis(qz, root_extent_m),
        )
    }
}

/// Extremely sparse octree: empty tree is a single `None` root.
#[derive(Debug)]
pub struct SparseOctree<T> {
    #[allow(dead_code)]
    root: Option<Box<Node<T>>>,
}

impl<T> Default for SparseOctree<T> {
    fn default() -> Self {
        Self { root: None }
    }
}

#[derive(Debug)]
#[allow(dead_code)]
enum Node<T> {
    Leaf(T),
    Internal([Option<Box<Node<T>>>; 8]),
}

impl<T> SparseOctree<T> {
    /// New empty tree.
    pub fn new() -> Self {
        Self::default()
    }

    /// Approximate resident memory (test hook).
    pub fn approx_bytes(&self) -> usize {
        std::mem::size_of_val(self)
    }
}
