//! Render layer bitmask inheritance for attachments (IR-5.8.1).

/// Bitmask of render layers (camera visibility).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct RenderLayers(pub u32);

/// Attached items inherit the parent mask, or intersect with an override when present.
#[must_use]
pub fn inherit_render_layers(
    parent: RenderLayers,
    override_layers: Option<RenderLayers>,
) -> RenderLayers {
    match override_layers {
        Some(layers) => RenderLayers(parent.0 & layers.0),
        None => parent,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// TC-IR-5.8.1.U4 — render layer inheritance with override.
    #[test]
    fn tc_ir_5_8_1_u4_bitwise_and_with_override() {
        let parent = RenderLayers(0b1100);
        let got = inherit_render_layers(parent, Some(RenderLayers(0b1010)));
        assert_eq!(got.0, 0b1000);
    }

    /// TC-IR-5.8.1.U5 — no override returns parent mask unchanged.
    #[test]
    fn tc_ir_5_8_1_u5_no_override_returns_parent() {
        let parent = RenderLayers(0b0110);
        let got = inherit_render_layers(parent, None);
        assert_eq!(got.0, 0b0110);
    }

    /// TC-IR-5.8.1.N4 / FM-7 — empty intersection yields zero mask (item invisible on all layers).
    #[test]
    fn tc_ir_5_8_1_n4_disjoint_override_yields_zero_mask() {
        let parent = RenderLayers(0b1100);
        let got = inherit_render_layers(parent, Some(RenderLayers(0b0011)));
        assert_eq!(got.0, 0);
    }
}
