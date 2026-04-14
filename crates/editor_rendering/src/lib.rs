//! Editor ↔ rendering integration contracts and policy helpers.
//!
//! This crate captures the cross-thread snapshot types and deterministic
//! policy rules described in `docs/design/integration/editor-rendering.md`.
//! It is intentionally renderer-agnostic: no GPU handles, no windowing, and
//! no threading primitives live here yet.

#![forbid(unsafe_code)]
#![deny(clippy::all)]
#![deny(missing_docs)]
// Many items exist as cross-crate contracts before the engine wires them in.
#![allow(dead_code)]

mod policy;
mod types;

pub use policy::{
    add_viewport, buffer_vis_from_wire, cap_outline_selection, remap_unknown_gizmo_shape,
    should_half_resolution, should_skip_render, ViewportPolicyError, MAX_EDITOR_VIEWPORTS,
};
pub use types::{
    BufferVisMode, CameraSnapshot, EditorOverlayRegistry, EditorRenderView, EditorViewport, Entity,
    GizmoDrawCommand, GizmoShape, GizmoTypeId, LinearColor, Mat4, ProxyStore, RenderFrame,
    RenderLayers, RenderPath, RenderPhase, RenderProxy, RenderTarget, RenderTargetHandle,
    RenderView, SelectionOutlineData, SelectionSet, Vec3, ViewId,
};

#[cfg(test)]
mod snapshot_tests {
    use rkyv::{from_bytes, rancor::Error, to_bytes};

    use crate::{
        BufferVisMode, CameraSnapshot, EditorRenderView, EditorViewport, Entity, GizmoDrawCommand,
        GizmoShape, LinearColor, Mat4, ProxyStore, RenderFrame, RenderLayers, RenderPath,
        RenderProxy, RenderTargetHandle, RenderView, SelectionOutlineData, Vec3, ViewId,
    };

    #[test]
    fn render_frame_roundtrips_through_rkyv() {
        let view = RenderView {
            view_id: ViewId {
                index: 1,
                generation: 1,
            },
            camera: CameraSnapshot {
                view: Mat4 {
                    cols: [
                        [1.0, 0.0, 0.0, 0.0],
                        [0.0, 1.0, 0.0, 0.0],
                        [0.0, 0.0, 1.0, 0.0],
                        [0.0, 0.0, 0.0, 1.0],
                    ],
                },
                projection: Mat4 {
                    cols: [
                        [1.0, 0.0, 0.0, 0.0],
                        [0.0, 1.0, 0.0, 0.0],
                        [0.0, 0.0, 1.0, 0.0],
                        [0.0, 0.0, 0.0, 1.0],
                    ],
                },
                near: 0.1,
                far: 1000.0,
            },
            target: RenderTargetHandle {
                index: 3,
                generation: 2,
            },
        };

        let editor_view = EditorRenderView {
            view,
            render_layers: RenderLayers::EDITOR_GIZMOS,
            gizmo_commands: smallvec::smallvec![GizmoDrawCommand {
                shape: GizmoShape::Line {
                    start: Vec3 {
                        x: 0.0,
                        y: 0.0,
                        z: 0.0,
                    },
                    end: Vec3 {
                        x: 1.0,
                        y: 0.0,
                        z: 0.0,
                    },
                },
                transform: Mat4 {
                    cols: [
                        [1.0, 0.0, 0.0, 0.0],
                        [0.0, 1.0, 0.0, 0.0],
                        [0.0, 0.0, 1.0, 0.0],
                        [0.0, 0.0, 0.0, 1.0],
                    ],
                },
                color: LinearColor {
                    r: 1.0,
                    g: 1.0,
                    b: 1.0,
                    a: 1.0,
                },
                depth_test: true,
            }],
            selection_outline: Some(SelectionOutlineData {
                selected_entities: smallvec::smallvec![Entity {
                    index: 9,
                    generation: 1,
                }],
                outline_color: LinearColor {
                    r: 0.0,
                    g: 1.0,
                    b: 0.0,
                    a: 1.0,
                },
                outline_width: 2.0,
            }),
            buffer_vis: Some(BufferVisMode::WorldNormals),
        };

        let frame = RenderFrame {
            editor_views: smallvec::smallvec![editor_view],
            proxy_store: ProxyStore {
                proxies: vec![RenderProxy {
                    entity: Entity {
                        index: 2,
                        generation: 0,
                    },
                }],
            },
            frame_index: 42,
        };

        let bytes = to_bytes::<Error>(&frame).expect("serialize");
        let decoded: RenderFrame = from_bytes::<RenderFrame, Error>(&bytes).expect("deserialize");
        assert_eq!(decoded, frame);
    }

    #[test]
    fn editor_viewport_roundtrips_through_rkyv() {
        let viewport = EditorViewport {
            view_id: ViewId {
                index: 0,
                generation: 0,
            },
            camera_entity: Entity {
                index: 5,
                generation: 1,
            },
            render_path: RenderPath::Deferred,
            debug_mode: None,
            render_layers: RenderLayers::GAMEPLAY,
            show_grid: true,
            show_gizmos: false,
        };

        let bytes = to_bytes::<Error>(&viewport).expect("serialize");
        let decoded: EditorViewport =
            from_bytes::<EditorViewport, Error>(&bytes).expect("deserialize");
        assert_eq!(decoded, viewport);
    }
}
