//! Widget identity, tree structure, and diff metadata.

pub mod id;
pub mod node;
pub mod tree;

pub use id::{Entity, WidgetId};
pub use node::{DirtyFlags, WidgetKey, WidgetKind, WidgetNode};
pub use tree::{UnknownParentError, WidgetTree};
