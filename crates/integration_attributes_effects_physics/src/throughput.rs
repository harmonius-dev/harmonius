//! Batch helpers for performance-style tests (`TC-IR-2.6.*.B1`, `TC-IR-2.6.0.B1`).

use crate::sync::{
    gravity_override_from_scale, mass_inverse_from_scales, scale_gravity_force, PhysicsMaterial,
};
use glam::Vec3;

/// Counts entities touched when only `changed` entries run work (`TC-IR-2.6.0.B1`).
#[must_use]
pub fn idle_sync_touch_count(changed: &[bool]) -> usize {
    changed.iter().filter(|&&c| c).count()
}

/// Runs [`gravity_override_from_scale`] only for changed rows; returns touched count.
#[must_use]
pub fn sync_gravity_batch(scales: &[f32], changed: &[bool]) -> usize {
    assert_eq!(scales.len(), changed.len());
    let mut touched = 0usize;
    for (&scale, &is_changed) in scales.iter().zip(changed.iter()) {
        if !is_changed {
            continue;
        }
        let _ = gravity_override_from_scale(scale);
        touched += 1;
    }
    touched
}

/// Computes scaled gravity forces for the first `len` entries (integration bench helper).
#[must_use]
pub fn bench_gravity_force_batch(global_g: Vec3, scales: &[f32], len: usize) -> Vec3 {
    let mut acc = Vec3::ZERO;
    for &s in scales.iter().take(len) {
        acc += scale_gravity_force(global_g, s);
    }
    acc
}

/// Computes friction overrides for benchmarking (`TC-IR-2.6.3.B1`).
#[must_use]
pub fn bench_friction_batch(material: &PhysicsMaterial, scales: &[f32], len: usize) -> usize {
    let mut n = 0usize;
    for &scale in scales.iter().take(len) {
        if crate::sync::friction_override_from_material(material, scale).is_some() {
            n += 1;
        }
    }
    n
}

/// Sums inverse mass after sync for micro-benchmarks (`TC-IR-2.6.2.B1`).
#[must_use]
pub fn bench_mass_inverse_batch(base_inverse: f32, scales: &[f32], len: usize) -> f32 {
    let mut acc = 0.0_f32;
    for &s in scales.iter().take(len) {
        acc += mass_inverse_from_scales(base_inverse, s);
    }
    acc
}

/// Alternates override presence to mimic expiry churn (`TC-IR-2.6.6.B1`).
#[must_use]
pub fn bench_gravity_override_churn(cycles: usize) -> usize {
    let mut touched = 0usize;
    for i in 0..cycles {
        let scale = if i % 2 == 0 { 0.5_f32 } else { 1.0_f32 };
        if gravity_override_from_scale(scale).is_some() {
            touched += 1;
        }
    }
    touched
}
