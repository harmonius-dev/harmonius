//! Frame-based sprite animation playback.

/// How a clip repeats when it reaches its last frame.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PlaybackMode {
    /// Wrap to the first frame after the last.
    Loop,
    /// Reverse direction at ends.
    PingPong,
    /// Stop on the last frame and emit finished once.
    OneShot,
}

/// High-level playback state for a clip instance.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PlaybackState {
    /// Actively advancing frames.
    Playing,
    /// `OneShot` reached the terminal frame.
    Finished,
}

/// Runtime animation component state.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SpriteAnimationState {
    /// Current frame index in `[0, frame_count)`.
    pub frame_index: u32,
    /// Time accumulated inside the current frame in seconds.
    pub elapsed: f32,
    /// Repeat behavior.
    pub mode: PlaybackMode,
    /// Terminal state for one-shot clips.
    pub state: PlaybackState,
}

/// Static clip metadata used by the tick function.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ClipMeta {
    /// Frames per second.
    pub fps: f32,
    /// Number of frames in the clip.
    pub frame_count: u32,
}

/// Named marker emitted when a frame boundary is crossed.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct AnimationEvent {
    /// Event label from clip data.
    pub name: String,
}

/// Advance `state` by `dt` seconds for a looping clip (`TC-10.5.2.1`).
pub fn advance_animation(
    state: &mut SpriteAnimationState,
    clip: &ClipMeta,
    dt: f32,
    events_out: &mut Vec<AnimationEvent>,
    clip_events: &[(u32, &str)],
) {
    events_out.clear();
    if state.mode != PlaybackMode::Loop || clip.frame_count == 0 || clip.fps <= 0.0 {
        return;
    }
    let frame_dt = 1.0 / clip.fps;
    let mut time_left = dt;
    while time_left > 0.0 {
        let need = frame_dt - state.elapsed;
        if time_left < need {
            state.elapsed += time_left;
            time_left = 0.0;
        } else {
            time_left -= need;
            state.frame_index = (state.frame_index + 1) % clip.frame_count;
            state.elapsed = 0.0;
            emit_enter_frame_events(state.frame_index, clip_events, events_out);
        }
    }
}

fn emit_enter_frame_events(
    entered: u32,
    clip_events: &[(u32, &str)],
    events_out: &mut Vec<AnimationEvent>,
) {
    for &(frame, name) in clip_events {
        if frame == entered {
            events_out.push(AnimationEvent {
                name: name.to_string(),
            });
        }
    }
}

/// Advance a one-shot clip; sets `Finished` and preserves the terminal frame (`TC-10.5.2.3`).
pub fn advance_animation_oneshot(
    state: &mut SpriteAnimationState,
    clip: &ClipMeta,
    dt: f32,
    events_out: &mut Vec<AnimationEvent>,
    clip_events: &[(u32, &str)],
) {
    events_out.clear();
    if state.state == PlaybackState::Finished || clip.frame_count == 0 || clip.fps <= 0.0 {
        return;
    }
    let frame_dt = 1.0 / clip.fps;
    let last = clip.frame_count.saturating_sub(1);
    let mut time_left = dt;
    while time_left > 0.0 && state.state == PlaybackState::Playing {
        if state.frame_index == last {
            let room = frame_dt - state.elapsed;
            if time_left >= room {
                time_left -= room;
                state.elapsed = frame_dt;
                state.state = PlaybackState::Finished;
                events_out.push(AnimationEvent {
                    name: "AnimationFinished".to_string(),
                });
                let _ = clip_events;
            } else {
                state.elapsed += time_left;
                time_left = 0.0;
            }
        } else {
            let need = frame_dt - state.elapsed;
            if time_left < need {
                state.elapsed += time_left;
                time_left = 0.0;
            } else {
                time_left -= need;
                state.frame_index += 1;
                state.elapsed = 0.0;
                emit_enter_frame_events(state.frame_index, clip_events, events_out);
            }
        }
    }
}

/// Record frame indices for repeated ping-pong ticks (`TC-10.5.2.2`).
#[must_use]
pub fn pingpong_sequence(frame_count: u32, steps: usize) -> Vec<u32> {
    if frame_count == 0 {
        return Vec::new();
    }
    let last = i32::try_from(frame_count.saturating_sub(1)).unwrap_or(0);
    let mut out = Vec::with_capacity(steps);
    let mut idx: i32 = 0;
    let mut dir: i32 = 1;
    for _ in 0..steps {
        out.push(idx as u32);
        if frame_count == 1 {
            continue;
        }
        if idx + dir < 0 || idx + dir > last {
            dir = -dir;
        }
        idx += dir;
    }
    out
}
