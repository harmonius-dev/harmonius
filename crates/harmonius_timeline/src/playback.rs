//! Mutable playback controls for timeline assets.

use smallvec::SmallVec;

use crate::asset::{LoopMode, MultiTrackTimeline};
use crate::event::{TimelineEvent, TimelineEventKind};
use crate::ids::{AssetId, Entity};

/// Direction of playback integration along the timeline axis.
#[derive(
    Clone, Copy, Debug, Eq, Hash, PartialEq, rkyv::Archive, rkyv::Deserialize, rkyv::Serialize,
)]
pub enum PlaybackDirection {
    /// Increasing time.
    Forward,
    /// Decreasing time.
    Reverse,
}

/// Runtime state for a timeline bound to an entity.
#[derive(Clone, Debug, PartialEq, rkyv::Archive, rkyv::Deserialize, rkyv::Serialize)]
pub struct PlaybackState {
    /// Asset id for the timeline being played.
    pub timeline_id: AssetId,
    /// Entity that owns this playback component.
    pub entity: Entity,
    /// Current evaluation time in seconds.
    pub current_time: f64,
    /// Scalar multiplier applied to `dt` while advancing.
    pub speed: f32,
    /// When false, `advance` is a no-op for time integration.
    pub playing: bool,
    /// Playback direction for integration.
    pub direction: PlaybackDirection,
    /// Number of loop wraps observed for diagnostics.
    pub loop_count: u32,
}

impl PlaybackState {
    /// Starts or resumes playback integration.
    pub fn play(&mut self) {
        self.playing = true;
    }

    /// Freezes playback without resetting time.
    pub fn pause(&mut self) {
        self.playing = false;
    }

    /// Stops playback and rewinds to the start.
    pub fn stop(&mut self) {
        self.playing = false;
        self.current_time = 0.0;
    }

    /// Seeks to `time`, respecting `LoopMode` and duration.
    pub fn seek(&mut self, time: f64, timeline: &MultiTrackTimeline) {
        let duration = timeline.duration.max(0.0);
        self.current_time = match timeline.loop_mode {
            LoopMode::Loop if duration > f64::EPSILON => {
                let mut t = time.rem_euclid(duration);
                if t < 0.0 {
                    t += duration;
                }
                t
            }
            _ => time.clamp(0.0, duration),
        };
    }

    /// Updates the playback speed multiplier.
    pub fn set_speed(&mut self, speed: f32) {
        self.speed = speed;
    }

    /// Integrates time and emits any discrete timeline events for this step.
    pub fn advance(
        &mut self,
        dt: f64,
        timeline: &MultiTrackTimeline,
    ) -> SmallVec<[TimelineEvent; 4]> {
        let mut events = SmallVec::new();
        if !self.playing || dt == 0.0 {
            return events;
        }

        let direction_sign = match self.direction {
            PlaybackDirection::Forward => 1.0,
            PlaybackDirection::Reverse => -1.0,
        };
        let effective_dt = dt * f64::from(self.speed) * direction_sign;
        if effective_dt.abs() <= f64::EPSILON {
            return events;
        }

        let forward_move = effective_dt > 0.0;
        let mut new_time = self.current_time + effective_dt;

        for track in &timeline.tracks {
            let last_time = track.keyframes.last().map(|kf| kf.time).unwrap_or(0.0);

            for kf in &track.keyframes {
                if !kf.trigger {
                    continue;
                }

                if directional_cross(self.current_time, new_time, kf.time, forward_move) {
                    events.push(TimelineEvent {
                        kind: TimelineEventKind::KeyframeCrossed {
                            track: track.id,
                            keyframe: kf.id,
                        },
                        time: kf.time,
                        entity: self.entity,
                    });
                }
            }

            if forward_move && self.current_time < last_time && new_time >= last_time {
                events.push(TimelineEvent {
                    kind: TimelineEventKind::TrackComplete { track: track.id },
                    time: last_time,
                    entity: self.entity,
                });
            }
        }

        let duration = timeline.duration.max(0.0);

        match timeline.loop_mode {
            LoopMode::Once => {
                if new_time > duration {
                    new_time = duration;
                    self.playing = false;
                    events.push(TimelineEvent {
                        kind: TimelineEventKind::TimelineComplete,
                        time: duration,
                        entity: self.entity,
                    });
                } else if new_time < 0.0 {
                    new_time = 0.0;
                    self.playing = false;
                }
            }
            LoopMode::Loop => {
                if duration <= f64::EPSILON {
                    new_time = 0.0;
                } else if forward_move {
                    while new_time > duration {
                        new_time -= duration;
                        self.loop_count = self.loop_count.saturating_add(1);
                        events.push(TimelineEvent {
                            kind: TimelineEventKind::LoopPoint {
                                count: self.loop_count,
                            },
                            time: new_time,
                            entity: self.entity,
                        });
                    }
                } else {
                    while new_time < 0.0 {
                        new_time += duration;
                        self.loop_count = self.loop_count.saturating_add(1);
                        events.push(TimelineEvent {
                            kind: TimelineEventKind::LoopPoint {
                                count: self.loop_count,
                            },
                            time: new_time,
                            entity: self.entity,
                        });
                    }
                }
            }
            LoopMode::PingPong => {
                if duration <= f64::EPSILON {
                    new_time = 0.0;
                } else {
                    let mut guard = 0;
                    while new_time > duration || new_time < 0.0 {
                        guard += 1;
                        if guard > 1024 {
                            break;
                        }

                        if new_time > duration {
                            let overshoot = new_time - duration;
                            self.direction = match self.direction {
                                PlaybackDirection::Forward => PlaybackDirection::Reverse,
                                PlaybackDirection::Reverse => PlaybackDirection::Forward,
                            };
                            self.loop_count = self.loop_count.saturating_add(1);
                            new_time = duration - overshoot;
                            events.push(TimelineEvent {
                                kind: TimelineEventKind::LoopPoint {
                                    count: self.loop_count,
                                },
                                time: new_time,
                                entity: self.entity,
                            });
                        } else if new_time < 0.0 {
                            let undershoot = -new_time;
                            self.direction = match self.direction {
                                PlaybackDirection::Forward => PlaybackDirection::Reverse,
                                PlaybackDirection::Reverse => PlaybackDirection::Forward,
                            };
                            self.loop_count = self.loop_count.saturating_add(1);
                            new_time = undershoot;
                            events.push(TimelineEvent {
                                kind: TimelineEventKind::LoopPoint {
                                    count: self.loop_count,
                                },
                                time: new_time,
                                entity: self.entity,
                            });
                        }
                    }
                }
            }
            LoopMode::ClampForever => {
                new_time = new_time.clamp(0.0, duration);
            }
        }

        self.current_time = new_time;
        events
    }

    /// Returns true when `LoopMode::Once` playback has stopped at the end.
    pub fn is_complete(&self, timeline: &MultiTrackTimeline) -> bool {
        matches!(timeline.loop_mode, LoopMode::Once) && !self.playing
    }

    /// Normalized playback fraction in `[0, 1]` for UI scrubbers.
    pub fn normalized_time(&self, timeline: &MultiTrackTimeline) -> f64 {
        let duration = timeline.duration.max(f64::EPSILON);
        self.current_time / duration
    }
}

fn directional_cross(old: f64, new: f64, mark: f64, forward: bool) -> bool {
    if forward {
        old < mark && new >= mark
    } else {
        old > mark && new <= mark
    }
}
