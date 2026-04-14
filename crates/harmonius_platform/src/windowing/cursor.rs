//! Cursor icon selection for window-backed input.

/// Cursor image requested for a window.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum CursorIcon {
    /// Default arrow cursor.
    #[default]
    Default,
    /// Text insertion cursor.
    Text,
    /// Hand / pointer cursor.
    Pointer,
}
