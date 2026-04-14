//! Command types crossing the game thread → audio thread boundary.
//!
//! Intentionally minimal for this integration crate; extend or unify with the canonical audio
//! command enum when that crate is wired in.

use glam::{Quat, Vec3};

/// Stable identifier for a local split-screen listener slot.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct ListenerId(pub u8);

/// Game-thread audio commands drained by the dedicated audio thread.
#[derive(Clone, Debug, PartialEq)]
pub enum AudioCommand {
    /// Updates listener pose and derived listener velocity for spatial audio.
    UpdateListener {
        /// Listener slot (local player index).
        listener_id: ListenerId,
        /// World-space listener position.
        position: Vec3,
        /// World-space listener orientation.
        orientation: Quat,
        /// Listener velocity in world space (m/s), already clamped upstream.
        velocity: Vec3,
    },
}

/// Surrogate for the lock-free MPSC producer used by the listener sync bridge.
pub trait AudioCommandSink {
    /// Enqueues a command; on failure returns the command for drop policies upstream.
    fn try_send(&mut self, cmd: AudioCommand) -> Result<(), AudioCommand>;
}
