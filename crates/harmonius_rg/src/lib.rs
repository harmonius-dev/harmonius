//! Declarative render graph compilation: ordering, barriers, queues, aliasing, and caching.
//!
//! Implements `docs/design/rendering/render-pipeline.md` render graph slice (F-2.2.x / R-2.2.x).

#![deny(clippy::all)]
#![deny(unsafe_code)]
#![warn(missing_docs)]

mod builder;
mod cache;
mod compiler;
mod encoder;
mod types;

pub use builder::{GraphBuilder, RenderGraph};
pub use cache::GraphCompileCache;
pub use compiler::{
    compile, compile_with_cache, split_barriers_for_fixture, BarrierBatch, CompilationConfig,
    CompiledPassMeta, DeviceCapabilities, ExecutionPlan, FencePlacement, QueueKind,
    RenderGraphError, SplitBarrier, StreamingState,
};
pub use encoder::{with_encoder, Encoder};
pub use types::{
    Capability, PassHandle, PassPriority, QueueAffinity, ResourceId, ResourceLifetime,
};
