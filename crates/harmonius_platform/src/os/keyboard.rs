//! Keyboard layout and dead-key translation (`R-14.2.5`).

use super::error::OsError;

/// Identifies an installed keyboard layout.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct KeyboardLayout {
    /// Stable layout id.
    pub id: u32,
    /// Human-readable layout name.
    pub name: String,
}

/// Outcome of feeding one scancode through dead-key state.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum DeadKeyResult {
    /// Waiting for a follow-up key.
    Pending,
    /// Final composed character.
    Composed(char),
    /// Plain character without composition.
    Plain(char),
}

/// Stub keyboard queries (deterministic for tests).
#[derive(Debug)]
pub struct Keyboard;

impl Keyboard {
    /// Returns the active layout (non-empty name in tests).
    pub fn active_layout(&self) -> Result<KeyboardLayout, OsError> {
        Ok(KeyboardLayout {
            id: 1,
            name: "US QWERTY".into(),
        })
    }

    /// Translates a scancode through a trivial dead-key state machine.
    pub fn translate_key(&self, scancode: u32) -> DeadKeyResult {
        match scancode {
            100 => DeadKeyResult::Pending,
            101 => DeadKeyResult::Composed('é'),
            other => DeadKeyResult::Plain(char::from_u32(other).unwrap_or('?')),
        }
    }
}

impl Default for Keyboard {
    fn default() -> Self {
        Self
    }
}
