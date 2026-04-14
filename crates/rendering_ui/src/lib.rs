//! Rendering–UI integration primitives aligned with
//! `docs/design/integration/rendering-ui.md`.
#![deny(clippy::all)]
#![deny(missing_docs)]
#![forbid(unsafe_code)]

mod batch;
mod nameplate;
mod nine_slice;
mod phase;
mod platform;

pub use batch::{build_batch_descriptors, BatchDescriptor, BlendState, QuadDrawKey};
pub use nameplate::{Entity, NameplateBuffer, NameplateScreenPos, NAMEPLATE_CAPACITY};
pub use nine_slice::{AtlasRegion, Edges, NineSliceResult, NineSliceSolver, UvRect};
pub use phase::{post_effect_phases_follow_ui, ui_pass_reads_scene_color, RenderPhase};
pub use platform::{default_nameplate_capacity, max_ui_draws, PlatformKind};

/// Bounded MPSC depth for UI snapshots (worker → render thread).
pub const MPSC_UI_SNAPSHOT_CAPACITY: usize = 2;
