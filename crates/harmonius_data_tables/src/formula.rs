//! Native formula hooks (codegen targets in full engine builds).

/// Codegen'd DPS helper used by tests (`TC-16.3.10.1`).
#[inline]
pub fn formula_item_dps(damage: f32, attack_speed: f32) -> f32 {
    damage * attack_speed
}
