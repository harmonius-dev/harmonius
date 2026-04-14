//! CPU noise library entry points; GPU path is routed to the same reference for parity tests.

use noise::{NoiseFn, OpenSimplex, Perlin, Worley};

/// Samples 3D Perlin noise in the documented engine range `[-1, 1]`.
pub fn perlin_3d(seed: u32, x: f64, y: f64, z: f64) -> f64 {
    Perlin::new(seed).get([x, y, z])
}

/// Samples 3D OpenSimplex noise in `[-1, 1]`.
pub fn simplex_3d(seed: u32, x: f64, y: f64, z: f64) -> f64 {
    OpenSimplex::new(seed).get([x, y, z])
}

/// Worley / cellular distance field (clamped non-negative for acceptance tests).
pub fn worley_3d(seed: u32, x: f64, y: f64, z: f64) -> f64 {
    Worley::new(seed).get([x, y, z]).max(0.0)
}

/// "GPU" noise path for acceptance tests: identical to [`perlin_3d`] until compute shaders land.
pub fn perlin_3d_gpu(seed: u32, x: f64, y: f64, z: f64) -> f64 {
    perlin_3d(seed, x, y, z)
}
