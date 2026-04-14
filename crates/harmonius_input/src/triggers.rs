//! Trigger conditions (pressed, hold, tap, pulse).

use crate::value::ActionValue;

/// Declarative trigger attached to a binding.
#[derive(Clone, Debug, PartialEq)]
pub enum TriggerCondition {
    /// Fire on the down edge only.
    Pressed,
    /// Fire on the up edge only.
    Released,
    /// Fire once after held continuously for `duration` seconds.
    Hold {
        /// Seconds.
        duration: f32,
    },
    /// Fire on quick press-release within threshold seconds.
    Tap {
        /// Max press duration for a tap.
        threshold: f32,
    },
    /// Fire periodically while held.
    Pulse {
        /// Period in seconds.
        interval: f32,
    },
    /// Raw gate: true every frame while active.
    Down,
}

/// Runtime state for trigger evaluation.
#[derive(Clone, Debug, PartialEq)]
pub struct TriggerState {
    /// Current high-level phase.
    pub phase: TriggerPhase,
    /// Accumulated time in the active sub-state.
    pub elapsed: f32,
    /// Count of pulse firings in the current hold segment.
    pub pulse_count: u32,
    /// True after the down edge for one-shot triggers.
    pub armed: bool,
}

impl Default for TriggerState {
    fn default() -> Self {
        Self {
            phase: TriggerPhase::Idle,
            elapsed: 0.0,
            pulse_count: 0,
            armed: false,
        }
    }
}

/// Trigger evaluation phases.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum TriggerPhase {
    /// Idle / not satisfied.
    Idle,
    /// In progress (holding, waiting for tap release, ...).
    Ongoing,
    /// Fired this frame.
    Fired,
    /// Terminal completion for multi-phase triggers.
    Completed,
}

impl TriggerCondition {
    /// Evaluate trigger against a digital `input_active` gate and analog `value` (unused for most
    /// digital triggers).
    pub fn evaluate(
        &self,
        input_active: bool,
        _value: &ActionValue,
        dt: f32,
        state: &mut TriggerState,
    ) -> TriggerPhase {
        match self {
            TriggerCondition::Pressed => {
                if input_active && !state.armed {
                    state.armed = true;
                    TriggerPhase::Fired
                } else if !input_active {
                    state.armed = false;
                    TriggerPhase::Idle
                } else {
                    TriggerPhase::Idle
                }
            }
            TriggerCondition::Hold { duration } => {
                if input_active {
                    state.elapsed += dt;
                    if state.elapsed >= *duration {
                        if state.phase != TriggerPhase::Completed {
                            state.phase = TriggerPhase::Completed;
                            return TriggerPhase::Fired;
                        }
                        TriggerPhase::Idle
                    } else {
                        state.phase = TriggerPhase::Ongoing;
                        TriggerPhase::Ongoing
                    }
                } else {
                    state.elapsed = 0.0;
                    state.phase = TriggerPhase::Idle;
                    TriggerPhase::Idle
                }
            }
            TriggerCondition::Tap { threshold } => {
                if input_active {
                    state.elapsed += dt;
                    state.phase = TriggerPhase::Ongoing;
                    TriggerPhase::Ongoing
                } else if state.phase == TriggerPhase::Ongoing {
                    if state.elapsed <= *threshold {
                        state.phase = TriggerPhase::Idle;
                        state.elapsed = 0.0;
                        return TriggerPhase::Fired;
                    }
                    state.elapsed = 0.0;
                    state.phase = TriggerPhase::Idle;
                    TriggerPhase::Idle
                } else {
                    TriggerPhase::Idle
                }
            }
            TriggerCondition::Pulse { interval } => {
                if input_active {
                    state.elapsed += dt;
                    if state.elapsed >= *interval {
                        state.elapsed -= interval;
                        state.pulse_count += 1;
                        return TriggerPhase::Fired;
                    }
                    TriggerPhase::Ongoing
                } else {
                    state.elapsed = 0.0;
                    TriggerPhase::Idle
                }
            }
            TriggerCondition::Down => {
                if input_active {
                    TriggerPhase::Fired
                } else {
                    TriggerPhase::Idle
                }
            }
            TriggerCondition::Released => {
                if !input_active && state.armed {
                    state.armed = false;
                    TriggerPhase::Fired
                } else if input_active {
                    state.armed = true;
                    TriggerPhase::Idle
                } else {
                    TriggerPhase::Idle
                }
            }
        }
    }
}
