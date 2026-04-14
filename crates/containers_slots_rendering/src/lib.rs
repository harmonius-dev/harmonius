//! Pure helpers for containers/slots ↔ rendering integration (IR-5.8.x).
//!
//! Maps to `docs/design/integration/containers-slots-rendering.md` and companion test cases.

#![deny(clippy::all)]
#![deny(unsafe_code)]
#![warn(missing_docs)]

mod attachment_transform;
mod preview_debug;
mod render_layers;
mod snap_hysteresis;
mod visual_binding;
mod visual_override;

pub use attachment_transform::{compute_attachment_transform, GlobalTransform, SocketDefinition};
pub use preview_debug::{DebugConfig, DebugSocketGizmo, Rgba8, SnapPreview, StaticStr};
pub use render_layers::{inherit_render_layers, RenderLayers};
pub use snap_hysteresis::snap_preview_is_active;
pub use visual_binding::{visual_binding_write_transform, PreviousGlobalTransform};
pub use visual_override::{AssetHandle, Material, Mesh, VisualOverride};
