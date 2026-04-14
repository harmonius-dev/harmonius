//! High-level integration architecture surface types.
//!
//! Normative source: `docs/design/integration/high-level.md` (class diagram, channel table,
//! per-frame phases).

#![deny(clippy::all)]
#![deny(missing_docs)]
#![deny(unsafe_code)]

/// Row-major 4x4 matrix used by integration snapshots.
pub type Mat4 = [f32; 16];

/// Two-component vector (e.g. camera jitter).
pub type Vec2 = [f32; 2];

/// Game loop phases in fixed execution order (Phase 1..=8).
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[repr(u8)]
pub enum GameLoopPhase {
    /// Phase 1 — Input processing (variable timestep).
    InputProcessing = 1,
    /// Phase 2 — Network receive (variable timestep).
    NetworkReceive = 2,
    /// Phase 3 — Simulation tick (fixed timestep).
    SimulationTick = 3,
    /// Phase 4 — AI update (fixed timestep).
    AiUpdate = 4,
    /// Phase 5 — Physics step (fixed timestep).
    PhysicsStep = 5,
    /// Phase 6 — Animation update (variable timestep).
    AnimationUpdate = 6,
    /// Phase 7 — Frame snapshot (variable timestep).
    FrameSnapshot = 7,
    /// Phase 8 — Frame end (variable timestep).
    FrameEnd = 8,
}

/// Count of sequential game loop phases in the high-level integration design.
pub const GAME_LOOP_PHASE_COUNT: usize = 8;

/// Returns phases in execution order.
pub const fn game_loop_phases_ordered() -> [GameLoopPhase; GAME_LOOP_PHASE_COUNT] {
    [
        GameLoopPhase::InputProcessing,
        GameLoopPhase::NetworkReceive,
        GameLoopPhase::SimulationTick,
        GameLoopPhase::AiUpdate,
        GameLoopPhase::PhysicsStep,
        GameLoopPhase::AnimationUpdate,
        GameLoopPhase::FrameSnapshot,
        GameLoopPhase::FrameEnd,
    ]
}

/// Input events (Main -> Workers) MPSC capacity from the channel buffer lengths table.
pub const INPUT_EVENTS_MPSC_CAPACITY: usize = 0;

/// Network packets (Main -> Workers) MPSC capacity.
pub const NETWORK_PACKETS_MPSC_CAPACITY: usize = 0;

/// Render frame triple-buffer slot count (Workers -> Render).
pub const RENDER_FRAME_TRIPLE_BUFFER_SLOTS: usize = 0;

/// Audio commands (Workers -> Audio RT) MPSC capacity.
pub const AUDIO_COMMANDS_MPSC_CAPACITY: usize = 0;

/// I/O requests (Workers -> Main) MPSC capacity.
pub const IO_REQUESTS_MPSC_CAPACITY: usize = 0;

/// Save writes (Workers -> Main) MPSC capacity.
pub const SAVE_WRITES_MPSC_CAPACITY: usize = 0;
