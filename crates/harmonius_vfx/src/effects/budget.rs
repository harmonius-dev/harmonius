//! Global [`EffectBudget`] caps for decals, debris, overlays, and shockwaves.

/// Target hardware profile used to select default caps.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PlatformTier {
    /// Mobile / handheld profile.
    Mobile,
    /// Desktop-class GPU profile.
    Desktop,
}

/// Resource limits shared across VFX subsystems.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct EffectBudget {
    /// Maximum decals resident in the decal pool.
    pub decal_pool_max: u32,
    /// Maximum concurrent debris fragments.
    pub debris_budget_max: u32,
    /// Maximum concurrent full-screen overlays.
    pub overlay_max: u32,
    /// Maximum concurrent shockwave distortion instances.
    pub shockwave_max: u32,
    /// Active decal slots (runtime counter).
    decals_in_use: u32,
    /// Active debris fragments (runtime counter).
    debris_in_use: u32,
    /// Active overlays (runtime counter).
    overlays_in_use: u32,
    /// Active shockwaves (runtime counter).
    shockwaves_in_use: u32,
}

impl EffectBudget {
    /// Default caps for a platform tier (**TC-11.5.0.1**).
    pub fn for_tier(tier: PlatformTier) -> Self {
        match tier {
            PlatformTier::Mobile => Self {
                decal_pool_max: 256,
                debris_budget_max: 32,
                overlay_max: 8,
                shockwave_max: 1,
                decals_in_use: 0,
                debris_in_use: 0,
                overlays_in_use: 0,
                shockwaves_in_use: 0,
            },
            PlatformTier::Desktop => Self {
                decal_pool_max: 2048,
                debris_budget_max: 256,
                overlay_max: 32,
                shockwave_max: 4,
                decals_in_use: 0,
                debris_in_use: 0,
                overlays_in_use: 0,
                shockwaves_in_use: 0,
            },
        }
    }

    /// Attempts to reserve one decal slot.
    pub fn try_alloc_decal(&mut self) -> bool {
        if self.decals_in_use < self.decal_pool_max {
            self.decals_in_use += 1;
            return true;
        }
        false
    }

    /// Releases one decal slot (paired with a successful [`EffectBudget::try_alloc_decal`]).
    pub fn free_decal(&mut self) {
        self.decals_in_use = self.decals_in_use.saturating_sub(1);
    }

    /// Attempts to reserve `count` debris fragments; returns granted count (may be partial).
    pub fn try_alloc_debris(&mut self, count: u32) -> u32 {
        let remaining = self.debris_budget_max.saturating_sub(self.debris_in_use);
        let grant = remaining.min(count);
        self.debris_in_use += grant;
        grant
    }

    /// Releases `count` debris fragments.
    pub fn free_debris(&mut self, count: u32) {
        self.debris_in_use = self.debris_in_use.saturating_sub(count);
    }

    /// Attempts to reserve one overlay slot (**TC-11.3.6.2**).
    pub fn try_alloc_overlay(&mut self) -> bool {
        if self.overlays_in_use < self.overlay_max {
            self.overlays_in_use += 1;
            return true;
        }
        false
    }

    /// Releases one overlay slot.
    pub fn free_overlay(&mut self) {
        self.overlays_in_use = self.overlays_in_use.saturating_sub(1);
    }

    /// Attempts to reserve one shockwave slot.
    pub fn try_alloc_shockwave(&mut self) -> bool {
        if self.shockwaves_in_use < self.shockwave_max {
            self.shockwaves_in_use += 1;
            return true;
        }
        false
    }

    /// Releases one shockwave slot.
    pub fn free_shockwave(&mut self) {
        self.shockwaves_in_use = self.shockwaves_in_use.saturating_sub(1);
    }

    /// Current debris usage (for assertions).
    pub fn debris_in_use(&self) -> u32 {
        self.debris_in_use
    }
}

#[cfg(test)]
mod tests {
    use super::{EffectBudget, PlatformTier};

    /// **TC-11.5.0.1** — platform tier defaults for debris and shockwaves.
    #[test]
    fn tc_11_5_0_1_budget_per_platform() {
        let m = EffectBudget::for_tier(PlatformTier::Mobile);
        assert_eq!(m.debris_budget_max, 32);
        assert_eq!(m.shockwave_max, 1);

        let d = EffectBudget::for_tier(PlatformTier::Desktop);
        assert_eq!(d.debris_budget_max, 256);
        assert_eq!(d.shockwave_max, 4);
    }

    /// **TC-11.5.1.1** — debris never exceeds the platform cap when over-requested.
    #[test]
    fn tc_11_5_1_1_debris_budget_cap() {
        let mut b = EffectBudget::for_tier(PlatformTier::Mobile);
        let g1 = b.try_alloc_debris(40);
        assert_eq!(g1, 32);
        assert_eq!(b.debris_in_use(), 32);
        let g2 = b.try_alloc_debris(10);
        assert_eq!(g2, 0);
        assert_eq!(b.debris_in_use(), 32);
    }

    /// **TC-11.3.6.2** — overlays fail once `overlay_max` is reached.
    #[test]
    fn tc_11_3_6_2_overlay_count_cap() {
        let mut b = EffectBudget::for_tier(PlatformTier::Mobile);
        for _ in 0..b.overlay_max {
            assert!(b.try_alloc_overlay());
        }
        assert!(!b.try_alloc_overlay());
    }
}
