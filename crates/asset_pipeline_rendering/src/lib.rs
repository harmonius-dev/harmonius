//! Data contracts and render-thread tables for asset pipeline ↔ rendering integration.
//!
//! Implements the testable surface from `docs/design/integration/asset-pipeline-rendering.md`
//! and `docs/design/integration/asset-pipeline-rendering-test-cases.md`.

#![forbid(unsafe_code)]
#![deny(clippy::all)]

pub mod contracts;
pub mod meshlet;
pub mod pipeline_state;
pub mod stream_request;
pub mod texture_profile;

pub use contracts::{
    ArchivedBakedTexture, BakedMeshlet, BakedTexture, BlendState, CBufferBinding, CompiledShader,
    DepthStencilState, GpuTextureFormat, MeshletBounds, NormalCone, RasterizerState, ShaderParam,
    ShaderReflection, ShaderStage, TargetPlatform, TextureBinding, TextureDimension, VertexLayout,
};
pub use meshlet::{meshlet_cluster_count, normal_cone_half_angle_flat};
pub use pipeline_state::{PipelineStateDesc, PipelineStateHandle, PipelineStateTable, VariantKey};
pub use stream_request::{StreamHandle, StreamRequest, StreamRequestState, StreamRequestTable};
pub use texture_profile::gpu_texture_format_for_png_rgba8;
