//! IME composition bridge (`R-14.2.6`).

use std::collections::VecDeque;
use std::sync::Mutex;

use super::error::OsError;

/// Candidate window anchor.
#[derive(Clone, Debug, PartialEq)]
pub struct ImePosition {
    /// X in screen points.
    pub x: f32,
    /// Y in screen points.
    pub y: f32,
    /// Caret line height.
    pub line_height: f32,
}

/// IME surface events.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ImeEvent {
    /// In-progress composition string.
    Composition {
        /// Intermediate UTF-8 text.
        text: String,
        /// Caret index inside `text`.
        cursor: u32,
    },
    /// Final committed string.
    Commit {
        /// Final UTF-8 text.
        text: String,
    },
}

/// Stub IME controller for tests.
#[derive(Debug)]
pub struct Ime {
    events: Mutex<VecDeque<ImeEvent>>,
    pos: Mutex<ImePosition>,
}

impl Ime {
    /// Creates an IME bridge.
    pub fn new() -> Self {
        Self {
            events: Mutex::new(VecDeque::new()),
            pos: Mutex::new(ImePosition {
                x: 0.0,
                y: 0.0,
                line_height: 0.0,
            }),
        }
    }

    /// Queues composition + commit events (test hook).
    pub fn simulate_composition(&self, text: &str, cursor: u32) -> Result<(), OsError> {
        let mut q = self.events.lock().expect("ime mutex poisoned");
        q.push_back(ImeEvent::Composition {
            text: text.into(),
            cursor,
        });
        Ok(())
    }

    /// Queues a commit event.
    pub fn simulate_commit(&self, text: &str) -> Result<(), OsError> {
        let mut q = self.events.lock().expect("ime mutex poisoned");
        q.push_back(ImeEvent::Commit { text: text.into() });
        Ok(())
    }

    /// Pops the next IME event.
    pub fn poll_event(&self) -> Option<ImeEvent> {
        self.events.lock().expect("ime mutex poisoned").pop_front()
    }

    /// Updates candidate window placement.
    pub fn set_position(&self, pos: &ImePosition) -> Result<(), OsError> {
        *self.pos.lock().expect("ime mutex poisoned") = pos.clone();
        Ok(())
    }
}

impl Default for Ime {
    fn default() -> Self {
        Self::new()
    }
}
