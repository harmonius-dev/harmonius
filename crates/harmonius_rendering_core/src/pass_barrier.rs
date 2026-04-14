//! Minimal render-pass barrier planning for render targets (R-2.3.8, TC-2.3.8.1).

/// Vulkan-style image layouts used in the design examples.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ImageLayout {
    /// Color attachment optimal (pass writes color to the texture).
    ColorAttachmentOptimal,
    /// Sampled image in a shader (non-depth/stencil).
    ShaderReadOnlyOptimal,
}

/// How a pass touches a texture id.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum PassTextureUse {
    /// Pass writes the texture as a color attachment.
    ColorAttachmentWrite,
    /// Pass reads the texture in a fragment/compute shader.
    ShaderSample,
}

/// One layout transition inserted between sequential passes.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct TextureLayoutBarrier {
    /// Stable texture id (fixture-level handle).
    pub texture: u32,
    /// Layout immediately before the transition.
    pub before: ImageLayout,
    /// Layout immediately after the transition.
    pub after: ImageLayout,
}

/// Plans barriers for strictly ordered passes. Each inner slice is one pass, in submission order.
#[must_use]
pub fn plan_sequential_texture_barriers(passes: &[&[(u32, PassTextureUse)]]) -> Vec<TextureLayoutBarrier> {
    let mut out = Vec::new();
    let mut last_layout: Vec<Option<(u32, ImageLayout)>> = Vec::new();
    for pass in passes {
        let mut current_pass: Vec<(u32, ImageLayout)> = Vec::new();
        for &(tex, use_) in *pass {
            let layout = match use_ {
                PassTextureUse::ColorAttachmentWrite => ImageLayout::ColorAttachmentOptimal,
                PassTextureUse::ShaderSample => ImageLayout::ShaderReadOnlyOptimal,
            };
            current_pass.push((tex, layout));
        }
        for &(tex, layout) in &current_pass {
            if let Some((prev_tex, prev_layout)) = last_layout.get(tex as usize).copied().flatten() {
                if prev_tex == tex && prev_layout != layout {
                    out.push(TextureLayoutBarrier {
                        texture: tex,
                        before: prev_layout,
                        after: layout,
                    });
                }
            }
        }
        for &(tex, layout) in &current_pass {
            let slot = tex as usize;
            if last_layout.len() <= slot {
                last_layout.resize(slot + 1, None);
            }
            last_layout[slot] = Some((tex, layout));
        }
    }
    out
}

#[cfg(test)]
mod tests {
    use super::{
        plan_sequential_texture_barriers, ImageLayout, PassTextureUse, TextureLayoutBarrier,
    };

    /// TC-2.3.8.1 — color write then shader read inserts exactly one transition for `T`.
    #[test]
    fn test_rt_barrier_auto_insert() {
        let pass_a = [(7_u32, PassTextureUse::ColorAttachmentWrite)];
        let pass_b = [(7_u32, PassTextureUse::ShaderSample)];
        let barriers = plan_sequential_texture_barriers(&[&pass_a[..], &pass_b[..]]);
        assert_eq!(
            barriers,
            vec![TextureLayoutBarrier {
                texture: 7,
                before: ImageLayout::ColorAttachmentOptimal,
                after: ImageLayout::ShaderReadOnlyOptimal,
            }]
        );
    }
}
