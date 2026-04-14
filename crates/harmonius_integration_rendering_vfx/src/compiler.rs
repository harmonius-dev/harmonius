//! Load-time `VfxGraph` → pass descriptor compilation (pure data).

use crate::types::BlendMode;
use crate::types::GpuBufferView;
use crate::types::ParticleRenderPassDesc;
use crate::types::RenderMode;
use crate::types::ScreenEffectKind;
use crate::types::ScreenEffectPassDesc;
use crate::types::SortKey;
use crate::types::VfxGraph;
use crate::types::VfxNodeKind;

/// Output row from [`VfxGraphCompiler::compile`].
#[derive(Clone, Debug, PartialEq)]
pub enum CompiledPassDesc {
    /// Particle sim + draw registration bundle.
    Particle(ParticleRenderPassDesc),
    /// Screen-space effect descriptor.
    Screen(ScreenEffectPassDesc),
    /// Decal-only graphs (stub until full decal compilation lands).
    DecalStub,
    /// Froxel-only graphs (stub until volumetric compilation lands).
    FroxelStub,
}

/// Compiles authoring graphs into transient pass descriptors.
#[derive(Clone, Copy, Debug, Default)]
pub struct VfxGraphCompiler;

impl VfxGraphCompiler {
    /// Walks `graph.nodes` in order and emits one [`CompiledPassDesc`] per node.
    ///
    /// TC-IR-3.7.1.S2 — three distinct node kinds produce three descriptors.
    pub fn compile(graph: &VfxGraph) -> Vec<CompiledPassDesc> {
        let dummy = GpuBufferView {
            id: 1,
            generation: 1,
        };
        graph
            .nodes
            .iter()
            .map(|node| match node.kind {
                VfxNodeKind::ParticleSim => CompiledPassDesc::Particle(ParticleRenderPassDesc {
                    particle_buffer: dummy,
                    alive_list: GpuBufferView {
                        id: 2,
                        generation: 1,
                    },
                    indirect_args: GpuBufferView {
                        id: 3,
                        generation: 1,
                    },
                    render_mode: RenderMode::Sprite,
                    sort_key: SortKey::BackToFront,
                    blend_mode: BlendMode::AlphaBlend,
                }),
                VfxNodeKind::Decal => CompiledPassDesc::DecalStub,
                VfxNodeKind::FroxelVolume => CompiledPassDesc::FroxelStub,
                VfxNodeKind::ScreenEffect => {
                    CompiledPassDesc::Screen(ScreenEffectPassDesc {
                        effect: ScreenEffectKind::HeatHaze,
                        params: [0.0; 8],
                        blend_mode: BlendMode::Additive,
                    })
                }
            })
            .collect()
    }
}
