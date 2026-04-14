//! Deterministic xxHash64 mixing for reproducible PCG.

use std::hash::{Hash, Hasher};
use twox_hash::XxHash64;

/// Derives a stable `u64` child seed from a parent seed and a logical domain tag.
pub fn derive_seed(parent: u64, tag: &str) -> u64 {
    let mut h = XxHash64::with_seed(parent);
    tag.hash(&mut h);
    h.finish()
}

/// Deterministic pseudo-random `f32` in `[0, 1)` from `(seed, index)`.
pub fn unit_float(seed: u64, index: u32) -> f32 {
    let mut h = XxHash64::with_seed(seed);
    index.hash(&mut h);
    let v = h.finish();
    (v as f32) / (u64::MAX as f32)
}

/// Folds `n` lanes with `derive_seed`; independent of Rayon worker count (associative `wrapping_add`).
pub fn eval_parallel_fold(seed: u64, n: usize, threads: usize) -> u64 {
    let pool = rayon::ThreadPoolBuilder::new()
        .num_threads(threads.max(1))
        .build()
        .expect("rayon thread pool");
    pool.install(|| {
        (0..n as u64)
            .into_par_iter()
            .map(|i| derive_seed(seed, &i.to_string()))
            .reduce(|| 0u64, |a, b| a.wrapping_add(b))
    })
}

/// Sequential reference for [`eval_parallel_fold`].
pub fn eval_sequential_fold(seed: u64, n: usize) -> u64 {
    (0..n as u64)
        .map(|i| derive_seed(seed, &i.to_string()))
        .fold(0u64, |a, b| a.wrapping_add(b))
}

use rayon::prelude::*;
