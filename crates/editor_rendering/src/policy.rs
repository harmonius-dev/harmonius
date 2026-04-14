//! Deterministic policy helpers referenced by the integration test matrix.

use smallvec::SmallVec;

use crate::types::{BufferVisMode, Entity, GizmoShape, Vec3};

/// Maximum number of simultaneous editor viewports (failure mode 3).
pub const MAX_EDITOR_VIEWPORTS: usize = 8;

/// Errors returned by viewport bookkeeping helpers.
#[derive(Debug, Clone, Eq, PartialEq)]
pub enum ViewportPolicyError {
    /// Opening another viewport would exceed [`MAX_EDITOR_VIEWPORTS`].
    TooManyViewports,
}

/// Validates that another viewport can be opened without exceeding the cap.
///
/// This models **TC-IR-5.5.1.N1**: opening a ninth viewport must fail without
/// allocating render targets in higher-level code.
pub fn add_viewport(current_open: usize) -> Result<(), ViewportPolicyError> {
    if current_open >= MAX_EDITOR_VIEWPORTS {
        return Err(ViewportPolicyError::TooManyViewports);
    }
    Ok(())
}

/// Returns `true` when a viewport should render at half resolution (failure mode 3).
///
/// Viewports with index **four or higher** (the fifth panel and beyond) degrade
/// to half-resolution rendering.
pub fn should_half_resolution(viewport_index: usize) -> bool {
    viewport_index >= 4
}

/// Returns `true` when rendering should be skipped for degenerate surfaces.
///
/// This models **TC-IR-5.5.5.N1**: a `0×0` viewport must not panic higher-level
/// scheduling code.
pub fn should_skip_render(width_px: u32, height_px: u32) -> bool {
    width_px == 0 || height_px == 0
}

/// Interprets a wire-format `BufferVisMode` discriminator with a safe fallback.
///
/// Corrupt or unknown values map to [`BufferVisMode::Albedo`] per **TC-IR-5.5.6.N1**.
pub fn buffer_vis_from_wire(raw: u8) -> BufferVisMode {
    match raw {
        0 => BufferVisMode::Albedo,
        1 => BufferVisMode::WorldNormals,
        2 => BufferVisMode::Roughness,
        3 => BufferVisMode::Metallic,
        4 => BufferVisMode::AmbientOcclusion,
        5 => BufferVisMode::Wireframe,
        6 => BufferVisMode::Overdraw,
        7 => BufferVisMode::MeshletId,
        8 => BufferVisMode::LodLevel,
        9 => BufferVisMode::UvChecker,
        _ => BufferVisMode::Albedo,
    }
}

/// Caps outline selection to 256 entities, returning the overflow count for logging.
///
/// This mirrors **TC-IR-5.5.3.N2** / failure mode 5: only the nearest 256 entities
/// participate in the Sobel outline pass; additional selections are handled by
/// separate wireframe overlays in higher-level editor code.
pub fn cap_outline_selection(entities: &[Entity]) -> (SmallVec<[Entity; 256]>, usize) {
    let mut out = SmallVec::new();
    let capped_len = entities.len().min(256);
    out.extend_from_slice(&entities[..capped_len]);
    let overflow = entities.len().saturating_sub(capped_len);
    (out, overflow)
}

/// Remaps unknown custom gizmo identifiers to a conservative cube primitive.
///
/// This models **TC-IR-5.5.2.N3**: stale `.dylib` identifiers fall back to
/// `GizmoShape::Cube` after hot reload when the registry no longer contains the id.
pub fn remap_unknown_gizmo_shape(shape: GizmoShape, registry_len: u32) -> GizmoShape {
    match shape {
        GizmoShape::Custom(id) if id.index >= registry_len => GizmoShape::Cube {
            half_extents: Vec3 {
                x: 1.0,
                y: 1.0,
                z: 1.0,
            },
        },
        other => other,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::GizmoTypeId;

    #[test]
    fn tc_ir_5_5_1_n1_ninth_viewport_rejected() {
        assert_eq!(add_viewport(7), Ok(()));
        assert_eq!(add_viewport(8), Err(ViewportPolicyError::TooManyViewports));
    }

    #[test]
    fn tc_ir_5_5_1_n2_fifth_viewport_half_resolution() {
        assert!(!should_half_resolution(3));
        assert!(should_half_resolution(4));
    }

    #[test]
    fn tc_ir_5_5_5_n1_zero_area_skips_render() {
        assert!(should_skip_render(0, 0));
        assert!(should_skip_render(10, 0));
        assert!(!should_skip_render(10, 10));
    }

    #[test]
    fn tc_ir_5_5_6_n1_invalid_buffer_vis_falls_back_to_albedo() {
        assert_eq!(buffer_vis_from_wire(255), BufferVisMode::Albedo);
    }

    #[test]
    fn tc_ir_5_5_3_n2_selection_cap_reports_overflow() {
        let mut ents = Vec::new();
        for i in 0..300 {
            ents.push(Entity {
                index: i,
                generation: 0,
            });
        }
        let (capped, overflow) = cap_outline_selection(&ents);
        assert_eq!(capped.len(), 256);
        assert_eq!(overflow, 44);
    }

    #[test]
    fn tc_ir_5_5_2_n3_unknown_custom_gizmo_maps_to_cube() {
        let shape = GizmoShape::Custom(GizmoTypeId { index: 99 });
        let remapped = remap_unknown_gizmo_shape(shape, 10);
        assert!(matches!(
            remapped,
            GizmoShape::Cube { half_extents } if half_extents.x == 1.0
        ));
    }
}
