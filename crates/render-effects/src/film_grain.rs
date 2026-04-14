//! Procedural film grain (TC-2.9.6.1).

use crate::util::XorShift64;

/// Fills `out` with deterministic film-grain noise derived from `seed`.
pub fn write_film_grain_pattern(seed: u64, out: &mut [u8]) {
    let mut rng = XorShift64::new(seed);
    for b in out.iter_mut() {
        *b = (rng.next_f32() * 255.0) as u8;
    }
}
