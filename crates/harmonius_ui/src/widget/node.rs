//! Widget node payload: kind, stable key, and hierarchical dirty flags.
//!
//! Types follow `docs/design/ui/ui-framework.md` § Widget Identity.

bitflags::bitflags! {
    /// Bitset of invalidation reasons propagated through the widget hierarchy.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    pub struct DirtyFlags: u16 {
        /// Style resolution needs to rerun for this subtree.
        const STYLE = 0b0000_0001;
        /// Layout constraints changed for this subtree.
        const LAYOUT = 0b0000_0010;
        /// Paint geometry or materials need rebuilding.
        const PAINT = 0b0000_0100;
        /// Data binding recompute is required.
        const BINDING = 0b0000_1000;
        /// Structural child list changed under this node.
        const CHILDREN = 0b0001_0000;
        /// Active animations touched this node.
        const ANIMATION = 0b0010_0000;
        /// Union of every flag above — full invalidation.
        const ALL = 0b0011_1111;
    }
}

/// Opaque stable identifier for editor-authored string table entries.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct StringId(pub u32);

/// Codegen identifier for custom widget kinds emitted by the visual editor pipeline.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct CustomWidgetId(pub u32);

/// Stable key used for reconciliation (`F-10.1.12`).
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub enum WidgetKey {
    /// Positional key for static child lists.
    Index(u32),
    /// Editor-assigned stable name.
    Named(StringId),
}

/// Built-in and codegen-extended widget discriminant (`F-10.1.1`, static dispatch).
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum WidgetKind {
    /// Root container and grouping surface.
    Panel,
    /// Single-line read-only text.
    Label,
    /// Clickable control.
    Button,
    /// Editable text field.
    TextInput,
    /// Scalar value control.
    Slider,
    /// Boolean toggle with explicit checked state.
    Checkbox,
    /// Boolean toggle without separate “unchecked” chrome.
    Toggle,
    /// Scrollable region.
    ScrollView,
    /// Selectable vertical list.
    ListView,
    /// Bitmap or vector image view.
    Image,
    /// Determinate/indeterminate progress display.
    ProgressBar,
    /// Inline search + selection control.
    ComboBox,
    /// Editor-defined widget implementation linked in the middleman dylib.
    Custom(CustomWidgetId),
}

/// Per-widget metadata carried alongside ECS components in the full engine.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WidgetNode {
    /// Concrete widget implementation kind.
    pub kind: WidgetKind,
    /// Reconciliation key for virtualized and dynamic trees.
    pub key: WidgetKey,
    /// Cached invalidation state for this node.
    pub dirty: DirtyFlags,
}
