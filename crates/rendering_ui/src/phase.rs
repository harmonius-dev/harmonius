/// Render graph phase ordering. `UI` must run after tonemap and before
/// post-process effects such as film grain (see integration design).
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[repr(u8)]
pub enum RenderPhase {
    /// Shadow map generation.
    ShadowMap = 0,
    /// Depth / normal prepass.
    Prepass = 1,
    /// Opaque geometry.
    Opaque = 2,
    /// Alpha-tested geometry.
    AlphaTest = 3,
    /// Transparent geometry (includes world-space UI panels per IR-3.6.4).
    Transparent = 4,
    /// HDR → display mapping before UI composite (IR-3.6.6).
    Tonemap = 5,
    /// Screen-space UI composite over scene color (IR-3.6.1).
    UI = 6,
    /// Chromatic aberration, grain, vignette, etc. after UI (IR-3.6.6).
    PostProcess = 7,
    /// Debug overlays.
    Debug = 8,
    /// Swapchain presentation.
    Present = 9,
}

/// Returns true when the UI pass samples the scene color buffer before drawing quads.
#[must_use]
pub const fn ui_pass_reads_scene_color() -> bool {
    true
}

/// Returns true when chromatic aberration / grain / vignette are sequenced after UI.
#[must_use]
pub const fn post_effect_phases_follow_ui() -> bool {
    true
}

#[cfg(test)]
mod tests {
    use super::RenderPhase;

    /// TC-IR-3.6.1.1 — UI runs in `RenderPhase::UI` after tonemap in the ordered enum.
    #[test]
    fn tc_ir_3_6_1_1_ui_pass_after_tonemap() {
        assert!(RenderPhase::Tonemap < RenderPhase::UI);
    }

    /// TC-IR-3.6.6.1 / IR-3.6.6 — UI is sequenced before generic post-process passes.
    #[test]
    fn tc_ir_3_6_6_ui_before_post_process() {
        assert!(RenderPhase::UI < RenderPhase::PostProcess);
    }

    /// IR-3.6.4 — World-space panels participate in the 3D transparent pass, before tonemap/UI.
    #[test]
    fn tc_ir_3_6_4_world_space_before_tonemap() {
        assert!(RenderPhase::Transparent < RenderPhase::Tonemap);
    }
}
