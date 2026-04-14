//! Error types for the editor ↔ runtime integration harness.

use core::fmt;

/// Recoverable or fatal errors surfaced by the integration harness.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum EditorCoreError {
    /// Play-in-editor failed before a `GameWorld` was created.
    PieCloneFailed,
    /// Channel backpressure was observed while sending editor mutations.
    BridgeBackpressure,
}

impl fmt::Display for EditorCoreError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::PieCloneFailed => write!(f, "play-in-editor clone failed"),
            Self::BridgeBackpressure => write!(f, "event bridge channel backpressure"),
        }
    }
}

impl std::error::Error for EditorCoreError {}

/// Undo / redo stack violations for the editor shadow world.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum UndoError {
    /// No operation was available to undo or redo.
    StackEmpty,
}

impl fmt::Display for UndoError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::StackEmpty => write!(f, "undo/redo stack was empty"),
        }
    }
}

impl std::error::Error for UndoError {}
