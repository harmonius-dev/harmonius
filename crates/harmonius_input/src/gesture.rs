//! Touch gesture recognition primitives used by gameplay/UI integration tests.

use glam::Vec2;

/// Gesture recognizer lifecycle phases.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum GesturePhase {
    /// Recognizer is watching but not committed.
    Possible,
    /// Gesture has started.
    Began,
    /// Gesture updated (movement / scale).
    Changed,
    /// Gesture completed normally.
    Ended,
    /// System cancelled tracking.
    Cancelled,
    /// Recognizer gave up (timeout / conflict).
    Failed,
}

/// High-level gesture classification emitted to consumers.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum GestureType {
    /// Discrete tap.
    Tap,
    /// Double tap.
    DoubleTap,
    /// Time-based press.
    LongPress,
    /// Directional swipe.
    Swipe {
        /// One of eight directions.
        dir: SwipeDirection,
    },
    /// Pinch zoom.
    Pinch,
    /// Two-finger pan translation.
    Pan,
}

/// Edge-relative swipe directions.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum SwipeDirection {
    /// Up.
    Up,
    /// Down.
    Down,
    /// Left.
    Left,
    /// Right.
    Right,
    /// Up-left diagonal.
    UpLeft,
    /// Up-right diagonal.
    UpRight,
    /// Down-left diagonal.
    DownLeft,
    /// Down-right diagonal.
    DownRight,
}

/// One concrete gesture emission.
#[derive(Clone, Debug, PartialEq)]
pub enum ResolvedGesture {
    /// Typed gesture with phase metadata.
    Emit {
        /// Gesture kind.
        gesture_type: GestureType,
        /// Lifecycle phase.
        phase: GesturePhase,
    },
}

/// Combined pinch + pan extraction result.
#[derive(Clone, Debug, PartialEq)]
pub struct PinchPanResult {
    /// Pinch scale multiplier (~ finger distance ratio).
    pub scale: f32,
    /// Centroid translation delta.
    pub pan_delta: Vec2,
}

/// Small state machine covering double-tap, long-press, swipe lifecycle, and pinch+pan.
#[derive(Clone, Debug)]
pub struct GestureEngine {
    phase: GesturePhase,
    tap_times: smallvec::SmallVec<[f32; 4]>,
    long_press_timer: f32,
    long_press_start: Vec2,
    swipe_peak_vel: f32,
    swipe_dir: Option<SwipeDirection>,
}

impl Default for GestureEngine {
    fn default() -> Self {
        Self {
            phase: GesturePhase::Possible,
            tap_times: smallvec::SmallVec::new(),
            long_press_timer: 0.0,
            long_press_start: Vec2::ZERO,
            swipe_peak_vel: 0.0,
            swipe_dir: None,
        }
    }
}

/// Stateful long-press recognizer (touch / pen).
#[derive(Clone, Debug, Default)]
pub struct LongPressRecognizer {
    t: f32,
    start: Vec2,
    fired: bool,
    max_drift: f32,
}

impl LongPressRecognizer {
    /// Reset recognizer between test cases.
    pub fn reset(&mut self) {
        *self = Self::default();
    }

    /// Advance with absolute position and release flag.
    pub fn tick(
        &mut self,
        pos: Vec2,
        dt: f32,
        released: bool,
        threshold_s: f32,
        drift_tol: f32,
    ) -> Option<GestureType> {
        if released {
            let out = if !self.fired && self.t >= threshold_s && self.max_drift <= drift_tol {
                Some(GestureType::LongPress)
            } else {
                None
            };
            *self = Self::default();
            return out;
        }
        if self.t == 0.0 {
            self.start = pos;
        }
        self.t += dt;
        self.max_drift = self.max_drift.max((pos - self.start).length());
        if self.t >= threshold_s && self.max_drift <= drift_tol && !self.fired {
            self.fired = true;
            return Some(GestureType::LongPress);
        }
        None
    }
}

impl GestureEngine {
    /// Reset all internal timers / phases.
    pub fn reset(&mut self) {
        *self = Self::default();
    }

    /// Feed a tap timestamp (`inter_tap_ms` max separation for double tap).
    pub fn on_tap(&mut self, t: f32, inter_tap_ms: f32) -> Vec<ResolvedGesture> {
        let mut out = Vec::new();
        let gap_ms = self.tap_times.last().map(|p| (t - p) * 1000.0).unwrap_or(f32::MAX);
        self.tap_times.push(t);
        if self.tap_times.len() == 2 && gap_ms <= inter_tap_ms {
            out.push(ResolvedGesture::Emit {
                gesture_type: GestureType::DoubleTap,
                phase: GesturePhase::Ended,
            });
            self.tap_times.clear();
        }
        out
    }

    /// Finalize tap stream after quiet period — ensures no stray single taps for double-tap pair.
    pub fn flush_taps_after_quiet(&mut self, _quiet_ms: f32) -> Vec<ResolvedGesture> {
        let out = Vec::new();
        self.tap_times.clear();
        out
    }

    /// Long-press recognizer (`hold_s` seconds, `max_drift` logical px).
    pub fn on_long_press_probe(
        &mut self,
        hold_s: f32,
        max_drift: f32,
        pos: Vec2,
        dt: f32,
        released: bool,
    ) -> Vec<ResolvedGesture> {
        let mut out = Vec::new();
        if self.long_press_timer == 0.0 {
            self.long_press_start = pos;
        }
        self.long_press_timer += dt;
        let drift = (pos - self.long_press_start).length();
        if released {
            if self.long_press_timer >= hold_s && drift <= max_drift {
                out.push(ResolvedGesture::Emit {
                    gesture_type: GestureType::LongPress,
                    phase: GesturePhase::Ended,
                });
            }
            self.long_press_timer = 0.0;
        } else if self.long_press_timer >= hold_s && drift > max_drift {
            self.long_press_timer = 0.0;
        }
        out
    }

    /// Classify swipe direction from velocity vector (px/s).
    pub fn swipe_from_velocity(vel: Vec2) -> SwipeDirection {
        let a = vel.y.atan2(vel.x);
        let deg = a.to_degrees();
        match deg {
            d if (-22.5..22.5).contains(&d) => SwipeDirection::Right,
            d if (22.5..67.5).contains(&d) => SwipeDirection::UpRight,
            d if (67.5..112.5).contains(&d) => SwipeDirection::Up,
            d if (112.5..157.5).contains(&d) => SwipeDirection::UpLeft,
            d if d.abs() >= 157.5 => SwipeDirection::Left,
            d if (-157.5..-112.5).contains(&d) => SwipeDirection::DownLeft,
            d if (-112.5..-67.5).contains(&d) => SwipeDirection::Down,
            _ => SwipeDirection::DownRight,
        }
    }

    /// Emit all eight swipe directions for test matrices.
    pub fn emit_swipe_matrix(vel: f32) -> [SwipeDirection; 8] {
        let dirs = [
            Vec2::new(1.0, 0.0),
            Vec2::new(1.0, 1.0),
            Vec2::new(0.0, 1.0),
            Vec2::new(-1.0, 1.0),
            Vec2::new(-1.0, 0.0),
            Vec2::new(-1.0, -1.0),
            Vec2::new(0.0, -1.0),
            Vec2::new(1.0, -1.0),
        ];
        dirs.map(|v| Self::swipe_from_velocity(v.normalize() * vel))
    }

    /// Extract pinch scale and centroid pan from two-finger traces.
    pub fn pinch_pan(
        a0: Vec2,
        a1: Vec2,
        b0: Vec2,
        b1: Vec2,
    ) -> PinchPanResult {
        let d0 = a0.distance(a1).max(1e-4);
        let d1 = b0.distance(b1).max(1e-4);
        let scale = d1 / d0;
        let c0 = (a0 + a1) * 0.5;
        let c1 = (b0 + b1) * 0.5;
        PinchPanResult {
            scale,
            pan_delta: c1 - c0,
        }
    }

    /// Track swipe lifecycle phases for a single swipe gesture.
    pub fn swipe_lifecycle(&mut self, vel: f32, threshold: f32) -> Vec<GesturePhase> {
        let mut log = Vec::new();
        self.phase = GesturePhase::Possible;
        log.push(self.phase);
        if vel >= threshold {
            self.swipe_peak_vel = vel;
            self.swipe_dir = Some(Self::swipe_from_velocity(Vec2::new(vel, 0.0)));
            self.phase = GesturePhase::Began;
            log.push(self.phase);
            self.phase = GesturePhase::Changed;
            log.push(self.phase);
            self.phase = GesturePhase::Ended;
            log.push(self.phase);
        }
        log
    }
}
