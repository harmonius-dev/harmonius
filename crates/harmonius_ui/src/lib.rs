//! Retained widget tree and related types for the Harmonius UI framework.
//!
//! Implements APIs described in `docs/design/ui/ui-framework.md`.

#![deny(clippy::all)]
#![deny(missing_docs)]
#![deny(unsafe_code)]

pub mod widget;

pub use widget::{
    BubbleDisposition, BubblePhase, CustomWidgetId, DirtyFlags, Entity, EventRouter, Rect,
    RemoveLeafError, StringId, UnknownParentError, WidgetId, WidgetKey, WidgetKind, WidgetNode,
    WidgetTree,
};
