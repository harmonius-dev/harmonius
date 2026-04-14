//! Input buffering for cancel windows and priority resolution.

use crate::actions::ActionId;

/// Coarse action categories for buffer arbitration.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum ActionCategory {
    /// Movement actions.
    Movement = 0,
    /// Primary attacks.
    Attack = 1,
    /// Special moves.
    Special = 2,
    /// Defensive dodge / roll.
    Dodge = 3,
    /// Wildcard / lowest priority.
    Any = 4,
}

/// Inclusive frame window where buffered inputs may cancel recovery.
#[derive(Clone, Debug, PartialEq)]
pub struct CancelWindow {
    /// First frame where consumption is allowed.
    pub start_frame: u32,
    /// Last frame where consumption is allowed.
    pub end_frame: u32,
    /// Categories permitted in this window.
    pub permitted: Vec<ActionCategory>,
}

#[derive(Clone, Debug)]
struct Buffered {
    action_id: ActionId,
    category: ActionCategory,
    time: f32,
    frame: u32,
}

/// Single-slot buffer with deterministic arbitration.
#[derive(Clone, Debug)]
pub struct InputBuffer {
    buf: Option<Buffered>,
    /// Max age to retain a buffered press without a window (seconds).
    pub duration: f32,
}

impl Default for InputBuffer {
    fn default() -> Self {
        Self {
            buf: None,
            duration: 1.0,
        }
    }
}

impl InputBuffer {
    /// Build an empty buffer with a maximum buffered age.
    pub fn with_duration(duration: f32) -> Self {
        Self {
            buf: None,
            duration,
        }
    }

    /// Record a candidate action at `time` / `frame`.
    pub fn push(&mut self, action_id: ActionId, category: ActionCategory, time: f32, frame: u32) {
        self.buf = Some(Buffered {
            action_id,
            category,
            time,
            frame,
        });
    }

    /// Attempt to consume the buffered action inside a cancel window.
    pub fn try_consume(
        &mut self,
        window: &CancelWindow,
        current_frame: u32,
        current_time: f32,
    ) -> Option<ActionId> {
        let b = self.buf.as_ref()?;
        if current_frame < window.start_frame || current_frame > window.end_frame {
            return None;
        }
        if !window.permitted.contains(&b.category) {
            return None;
        }
        if b.frame > window.end_frame {
            return None;
        }
        if current_time - b.time > self.duration {
            return None;
        }
        let id = b.action_id;
        self.buf = None;
        Some(id)
    }

    /// Resolve two competing buffered intents at window open using category ordering.
    pub fn resolve_priority_at_window(
        first: (ActionId, ActionCategory, f32),
        second: (ActionId, ActionCategory, f32),
    ) -> ActionId {
        let (a_id, a_cat, a_t) = first;
        let (b_id, b_cat, b_t) = second;
        if a_cat > b_cat {
            a_id
        } else if b_cat > a_cat {
            b_id
        } else if a_t < b_t {
            a_id
        } else {
            b_id
        }
    }
}
