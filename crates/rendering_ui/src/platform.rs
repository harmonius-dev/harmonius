//! Per-platform UI draw budgets (design: Platform Considerations table).

/// Host platform class for draw-call budget helpers.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum PlatformKind {
    /// Desktop-class GPU.
    Desktop,
    /// Fixed home consoles.
    Console,
    /// Nintendo Switch–class limits.
    Switch,
    /// Mobile / handheld phone class.
    Mobile,
}

/// Maximum UI indirect draws recommended for the platform (TC-IR-3.6.2.B* / design table).
#[must_use]
pub const fn max_ui_draws(platform: PlatformKind) -> u32 {
    match platform {
        PlatformKind::Desktop | PlatformKind::Console => 50,
        PlatformKind::Switch => 40,
        PlatformKind::Mobile => 30,
    }
}

/// Default nameplate projection buffer capacity on desktop/console (IR-3.6.7).
#[must_use]
pub const fn default_nameplate_capacity(platform: PlatformKind) -> usize {
    match platform {
        PlatformKind::Desktop | PlatformKind::Console => 256,
        PlatformKind::Switch => 128,
        PlatformKind::Mobile => 64,
    }
}

#[cfg(test)]
mod tests {
    use super::{default_nameplate_capacity, max_ui_draws, PlatformKind};

    #[test]
    fn desktop_draw_budget_matches_design() {
        assert_eq!(max_ui_draws(PlatformKind::Desktop), 50);
    }

    #[test]
    fn mobile_draw_budget_matches_design() {
        assert_eq!(max_ui_draws(PlatformKind::Mobile), 30);
    }

    #[test]
    fn desktop_nameplate_cap_matches_design() {
        assert_eq!(default_nameplate_capacity(PlatformKind::Desktop), 256);
    }

    #[test]
    fn mobile_nameplate_cap_matches_design() {
        assert_eq!(default_nameplate_capacity(PlatformKind::Mobile), 64);
    }
}
