//! Deterministic editor shell primitives from `docs/design/tools/editor-core.md`.
//!
//! This crate hosts dock layout, viewports, undo, selection, gizmos, preferences, plugin wiring,
//! and VR mode helpers used by the Harmonius editor.

#![deny(clippy::all)]
#![deny(missing_docs)]
#![deny(unsafe_code)]

mod dock;
mod gizmo;
mod plugin;
mod preferences;
mod selection;
mod undo;
mod viewport;
mod vr;

pub use dock::{DockError, DockNode, DockTree, DockZone, FloatingPanel, PanelId, SplitDirection};
pub use gizmo::{
    measurement_distance, GizmoConstraint, GizmoDelta, GizmoError, GizmoFrame, GizmoManager,
    GizmoTool, TransformCommand,
};
pub use plugin::{
    DependencyResolver, DependencyResolverError, EditorWidgetFactory, EditorWidgetRegistry,
    HotReloadError, IsolationScope, PluginDependency, PluginHandle, PluginHost, PluginMetadata,
    PluginState, SemanticVersion,
};
pub use preferences::{PreferenceError, PreferenceKey, PreferenceStore};
pub use selection::{
    AssetId, Entity, Selectable, SelectionError, SelectionModifier, SelectionSet, SelectionState,
    SubObjectElement,
};
pub use undo::{add_command, CommandError, EditorCommand, Transaction, UndoStack, World};
pub use viewport::{CameraMode, Ray, ViewportConfig, ViewportError, ViewportId, ViewportManager};
pub use vr::{
    ControllerState, EditorAction, FollowError, FollowTarget, Hand, HandGesture, HandState,
    VrAvatarSystem, VrError, VrFollowMode, VrInputAdapter, VrInputMode, VrModeManager, VrModeState,
    VrPanelAnchor, VrPanelSystem, VrPerfBudget,
};
