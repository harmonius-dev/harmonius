//! Trigger conditions (pressed, hold, tap, pulse, chord, combo).

use crate::actions::InputSource;
use crate::value::ActionValue;
use smallvec::SmallVec;

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
    /// All listed sources must be held; first partial to all-down must finish within `window`
    /// seconds.
    Chord {
        /// Chord members (parallel to booleans in [`TriggerCondition::evaluate_chord`]).
        inputs: SmallVec<[InputSource; 4]>,
        /// Max seconds from first partial press to all members down.
        window: f32,
    },
    /// Ordered sequence; call [`TriggerCondition::evaluate_combo`] with each step edge in order.
    Combo {
        /// Ordered sources (semantic labels for tooling; matching uses `step_just_matched`).
        sequence: SmallVec<[InputSource; 8]>,
        /// Max idle time between consecutive matched steps.
        window_per_step: f32,
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
    /// Scratch flags for chord progress (optional; reserved for tooling).
    pub chord_active: SmallVec<[bool; 4]>,
    /// Current combo sequence step (0 = waiting first edge).
    pub combo_step: u8,
}

impl Default for TriggerState {
    fn default() -> Self {
        Self {
            phase: TriggerPhase::Idle,
            elapsed: 0.0,
            pulse_count: 0,
            armed: false,
            chord_active: SmallVec::new(),
            combo_step: 0,
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
    ///
    /// [`TriggerCondition::Chord`] and [`TriggerCondition::Combo`] return [`TriggerPhase::Idle`]
    /// here; use [`TriggerCondition::evaluate_chord`] / [`TriggerCondition::evaluate_combo`].
    pub fn evaluate(
        &self,
        input_active: bool,
        _value: &ActionValue,
        dt: f32,
        state: &mut TriggerState,
    ) -> TriggerPhase {
        match self {
            TriggerCondition::Chord { .. } | TriggerCondition::Combo { .. } => TriggerPhase::Idle,
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

    /// Chord evaluation: `inputs_down[i]` parallels `Chord.inputs[i]`.
    pub fn evaluate_chord(
        &self,
        inputs_down: &[bool],
        dt: f32,
        state: &mut TriggerState,
    ) -> TriggerPhase {
        let TriggerCondition::Chord {
            inputs: sources,
            window,
        } = self
        else {
            return TriggerPhase::Idle;
        };
        if sources.is_empty() || inputs_down.len() != sources.len() {
            return TriggerPhase::Idle;
        }

        state.chord_active.clear();
        state.chord_active.extend(inputs_down.iter().copied());

        let any = inputs_down.iter().any(|&v| v);
        let all = inputs_down.iter().all(|&v| v);

        if matches!(state.phase, TriggerPhase::Completed) {
            if !any {
                *state = TriggerState::default();
            }
            return TriggerPhase::Idle;
        }

        if !any {
            *state = TriggerState::default();
            return TriggerPhase::Idle;
        }

        if !state.armed {
            state.armed = true;
            state.elapsed = 0.0;
        } else {
            state.elapsed += dt;
        }

        if all {
            if state.elapsed <= *window {
                state.phase = TriggerPhase::Completed;
                state.armed = false;
                state.elapsed = 0.0;
                return TriggerPhase::Fired;
            }
            *state = TriggerState::default();
            return TriggerPhase::Idle;
        }

        if state.elapsed > *window {
            *state = TriggerState::default();
            return TriggerPhase::Idle;
        }

        TriggerPhase::Ongoing
    }

    /// Combo evaluation: call with `true` on the rising edge for the current expected step.
    pub fn evaluate_combo(
        &self,
        step_just_matched: bool,
        dt: f32,
        state: &mut TriggerState,
    ) -> TriggerPhase {
        let TriggerCondition::Combo {
            sequence,
            window_per_step,
        } = self
        else {
            return TriggerPhase::Idle;
        };
        if sequence.is_empty() {
            return TriggerPhase::Idle;
        }

        if step_just_matched {
            let _ = dt;
            state.combo_step = state.combo_step.saturating_add(1);
            state.elapsed = 0.0;
            if state.combo_step as usize == sequence.len() {
                state.combo_step = 0;
                return TriggerPhase::Fired;
            }
            return TriggerPhase::Ongoing;
        }

        if state.combo_step > 0 {
            state.elapsed += dt;
            if state.elapsed > *window_per_step {
                *state = TriggerState::default();
                return TriggerPhase::Idle;
            }
            return TriggerPhase::Ongoing;
        }

        TriggerPhase::Idle
    }
}
