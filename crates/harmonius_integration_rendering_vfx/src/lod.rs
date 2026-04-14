//! Emitter distance LOD with hysteresis (IR-3.7.7).

use crate::types::{EmitterLodComponent, LodTier};

fn raw_tier(distance: f32, c: &EmitterLodComponent) -> LodTier {
    if distance <= c.full_distance {
        LodTier::Full
    } else if distance <= c.reduced_distance {
        LodTier::Reduced
    } else if distance <= c.cull_distance {
        LodTier::Impostor
    } else {
        LodTier::Culled
    }
}

fn transition_blocked_by_hysteresis(
    distance: f32,
    from: LodTier,
    to: LodTier,
    c: &EmitterLodComponent,
    h: f32,
) -> bool {
    let band = |b: f32| b > 0.0 && (distance - b).abs() < b * h;
    match (from, to) {
        (LodTier::Full, LodTier::Reduced) | (LodTier::Reduced, LodTier::Full) => {
            band(c.full_distance)
        }
        (LodTier::Reduced, LodTier::Impostor) | (LodTier::Impostor, LodTier::Reduced) => {
            band(c.reduced_distance)
        }
        (LodTier::Impostor, LodTier::Culled) | (LodTier::Culled, LodTier::Impostor) => {
            band(c.cull_distance)
        }
        _ => false,
    }
}

/// Resolves the emitter LOD tier for `distance`, updating `comp.current_tier`.
///
/// TC-IR-3.7.7.1 / TC-IR-3.7.7.2 / TC-IR-3.7.7.3 — distance bands plus hysteresis at boundaries.
pub fn evaluate_emitter_lod(distance: f32, comp: &mut EmitterLodComponent) -> LodTier {
    let raw = raw_tier(distance, comp);
    let h = comp.hysteresis_pct.clamp(0.0, 1.0);
    if raw != comp.current_tier
        && transition_blocked_by_hysteresis(distance, comp.current_tier, raw, comp, h)
    {
        return comp.current_tier;
    }
    comp.current_tier = raw;
    raw
}
