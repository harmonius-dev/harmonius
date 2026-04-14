//! Particle light injection + brightest selection (IR-3.7.5).

use std::cmp::Ordering;
use std::cmp::Reverse;
use std::collections::BinaryHeap;

use crate::types::ParticleLight;

#[derive(Clone, Copy, Debug, PartialEq)]
struct HeapLight {
    intensity: f32,
    ordinal: u32,
}

impl Eq for HeapLight {}

impl Ord for HeapLight {
    fn cmp(&self, other: &Self) -> Ordering {
        self.intensity
            .total_cmp(&other.intensity)
            .then_with(|| other.ordinal.cmp(&self.ordinal))
    }
}

impl PartialOrd for HeapLight {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl From<&ParticleLight> for HeapLight {
    fn from(light: &ParticleLight) -> Self {
        Self {
            intensity: light.intensity,
            ordinal: light.ordinal,
        }
    }
}

impl From<HeapLight> for ParticleLight {
    fn from(value: HeapLight) -> Self {
        Self {
            intensity: value.intensity,
            ordinal: value.ordinal,
        }
    }
}

/// Selects up to `budget` brightest lights using a bounded min-heap (`O(N log budget)`).
///
/// TC-IR-3.7.5.2 — retains the `budget` largest intensities with deterministic tie-breaks.
pub fn select_brightest_particle_lights(
    lights: &[ParticleLight],
    budget: usize,
) -> Vec<ParticleLight> {
    if budget == 0 {
        return Vec::new();
    }
    let mut heap: BinaryHeap<Reverse<HeapLight>> = BinaryHeap::new();
    for light in lights {
        let cand = HeapLight::from(light);
        if heap.len() < budget {
            heap.push(Reverse(cand));
        } else if let Some(Reverse(dimmest)) = heap.peek() {
            if cand.cmp(dimmest) == Ordering::Greater {
                heap.pop();
                heap.push(Reverse(cand));
            }
        }
    }
    let mut out: Vec<ParticleLight> = heap.into_iter().map(|k| k.0.into()).collect();
    out.sort_by(|a, b| {
        b.intensity
            .total_cmp(&a.intensity)
            .then_with(|| a.ordinal.cmp(&b.ordinal))
    });
    out
}

/// Counts lights placed into a conceptual cluster grid (one cell per light for tests).
///
/// TC-IR-3.7.5.1 — each injected light occupies a cluster slot up to `lights.len()`.
pub fn cluster_slots_used(lights: &[ParticleLight]) -> usize {
    lights.len()
}

/// TC-IR-3.7.5.F1 — overflow counter: how many lights were dropped beyond `budget`.
pub fn particle_light_overflow_dropped(lights: &[ParticleLight], budget: usize) -> u32 {
    lights.len().saturating_sub(budget) as u32
}
