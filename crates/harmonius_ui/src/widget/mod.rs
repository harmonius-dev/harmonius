//! Widget identity, tree structure, and diff metadata.

pub mod event;
pub mod id;
pub mod node;
pub mod tree;

pub use event::{BubbleDisposition, BubblePhase, EventRouter, Rect};
pub use id::{Entity, WidgetId};
pub use node::{CustomWidgetId, DirtyFlags, StringId, WidgetKey, WidgetKind, WidgetNode};
pub use tree::{RemoveLeafError, UnknownParentError, WidgetTree};
