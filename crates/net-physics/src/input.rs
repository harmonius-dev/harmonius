use rkyv_derive::{Archive, Deserialize, Serialize};

/// Single frame of player input used for prediction and rollback replay.
#[derive(Clone, Copy, Debug, PartialEq, Archive, Serialize, Deserialize)]
pub struct InputFrame {
    /// Monotonic tick this input applies to.
    pub tick: u64,
    /// Normalized movement intent along X.
    pub move_x: f32,
    /// Normalized movement intent along Y (vertical for 3D games using XZ plane).
    pub move_y: f32,
    /// Normalized movement intent along Z.
    pub move_z: f32,
}

impl InputFrame {
    /// Zeroed input for a tick (explicit stand-still).
    #[must_use]
    pub const fn zero(tick: u64) -> Self {
        Self {
            tick,
            move_x: 0.0,
            move_y: 0.0,
            move_z: 0.0,
        }
    }
}
