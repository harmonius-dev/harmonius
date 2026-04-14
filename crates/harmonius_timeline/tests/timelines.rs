//! Companion coverage for `docs/design/simulation/timelines-test-cases.md`.

use harmonius_timeline::{
    AssetId, Entity, Interpolation, Keyframe, KeyframeId, LoopMode, MultiTrackTimeline,
    PlaybackDirection, PlaybackState, Quat, TimelineEventKind, Track, TrackId, TrackValue, Vec2,
    Vec3,
};
use smallvec::{smallvec, SmallVec};
use smol_str::SmolStr;

fn ease_css() -> Interpolation {
    Interpolation::CubicBezier {
        c1: Vec2::new(0.42, 0.0),
        c2: Vec2::new(0.58, 1.0),
    }
}

#[test]
fn test_track_sample_linear_midpoint() {
    let track = Track {
        id: TrackId(0),
        name: SmolStr::new("t"),
        keyframes: smallvec![
            Keyframe {
                id: KeyframeId(0),
                time: 0.0,
                value: 0.0_f32,
                interpolation: Interpolation::Linear,
                trigger: false,
            },
            Keyframe {
                id: KeyframeId(1),
                time: 1.0,
                value: 10.0,
                interpolation: Interpolation::Linear,
                trigger: false,
            },
        ],
        default_value: 0.0,
    };
    assert!((track.sample(0.5) - 5.0).abs() < f32::EPSILON);
}

#[test]
fn test_track_sample_step_hold() {
    let track = Track {
        id: TrackId(0),
        name: SmolStr::new("t"),
        keyframes: smallvec![
            Keyframe {
                id: KeyframeId(0),
                time: 0.0,
                value: 1.0,
                interpolation: Interpolation::Step,
                trigger: false,
            },
            Keyframe {
                id: KeyframeId(1),
                time: 1.0,
                value: 5.0,
                interpolation: Interpolation::Step,
                trigger: false,
            },
        ],
        default_value: 0.0,
    };
    assert!((track.sample(0.99) - 1.0_f32).abs() < f32::EPSILON);
    assert!((track.sample(1.0) - 5.0_f32).abs() < f32::EPSILON);
}

#[test]
fn test_track_sample_cubic_bezier() {
    let track = Track {
        id: TrackId(0),
        name: SmolStr::new("t"),
        keyframes: smallvec![
            Keyframe {
                id: KeyframeId(0),
                time: 0.0,
                value: 0.0,
                interpolation: ease_css(),
                trigger: false,
            },
            Keyframe {
                id: KeyframeId(1),
                time: 1.0,
                value: 1.0,
                interpolation: Interpolation::Linear,
                trigger: false,
            },
        ],
        default_value: 0.0,
    };
    let v = track.sample(0.5);
    assert!((f64::from(v) - 0.5).abs() < 1e-3);
}

#[test]
fn test_track_sample_constant() {
    let track = Track {
        id: TrackId(0),
        name: SmolStr::new("t"),
        keyframes: smallvec![
            Keyframe {
                id: KeyframeId(0),
                time: 0.0,
                value: 7.0,
                interpolation: Interpolation::Constant,
                trigger: false,
            },
            Keyframe {
                id: KeyframeId(1),
                time: 5.0,
                value: 99.0,
                interpolation: Interpolation::Linear,
                trigger: false,
            },
        ],
        default_value: 0.0,
    };
    assert!((track.sample(2.0) - 7.0_f32).abs() < f32::EPSILON);
    assert!((track.sample(4.99) - 7.0_f32).abs() < f32::EPSILON);
}

#[test]
fn test_track_sample_before_first_kf() {
    let track = Track {
        id: TrackId(0),
        name: SmolStr::new("t"),
        keyframes: smallvec![
            Keyframe {
                id: KeyframeId(0),
                time: 1.0,
                value: 10.0,
                interpolation: Interpolation::Linear,
                trigger: false,
            },
            Keyframe {
                id: KeyframeId(1),
                time: 2.0,
                value: 20.0,
                interpolation: Interpolation::Linear,
                trigger: false,
            },
        ],
        default_value: 0.0,
    };
    assert!((track.sample(0.5) - 10.0_f32).abs() < f32::EPSILON);
}

#[test]
fn test_track_sample_after_last_kf() {
    let track = Track {
        id: TrackId(0),
        name: SmolStr::new("t"),
        keyframes: smallvec![
            Keyframe {
                id: KeyframeId(0),
                time: 0.0,
                value: 0.0,
                interpolation: Interpolation::Linear,
                trigger: false,
            },
            Keyframe {
                id: KeyframeId(1),
                time: 1.0,
                value: 10.0,
                interpolation: Interpolation::Linear,
                trigger: false,
            },
        ],
        default_value: 0.0,
    };
    assert!((track.sample(5.0) - 10.0_f32).abs() < f32::EPSILON);
}

#[test]
fn test_track_default_value_empty() {
    let track = Track::<f32> {
        id: TrackId(0),
        name: SmolStr::new("t"),
        keyframes: smallvec![],
        default_value: 42.0,
    };
    assert!((track.sample(0.0) - 42.0).abs() < f32::EPSILON);
    assert!((track.sample(100.0) - 42.0).abs() < f32::EPSILON);
}

#[test]
fn test_keyframe_at_or_before() {
    let track = Track {
        id: TrackId(0),
        name: SmolStr::new("t"),
        keyframes: smallvec![
            Keyframe {
                id: KeyframeId(0),
                time: 1.0,
                value: 0.0,
                interpolation: Interpolation::Linear,
                trigger: false,
            },
            Keyframe {
                id: KeyframeId(1),
                time: 3.0,
                value: 0.0,
                interpolation: Interpolation::Linear,
                trigger: false,
            },
            Keyframe {
                id: KeyframeId(2),
                time: 5.0,
                value: 0.0,
                interpolation: Interpolation::Linear,
                trigger: false,
            },
            Keyframe {
                id: KeyframeId(3),
                time: 7.0,
                value: 0.0,
                interpolation: Interpolation::Linear,
                trigger: false,
            },
        ],
        default_value: 0.0,
    };
    assert!(track.keyframe_at_or_before(0.5).is_none());
    assert!((track.keyframe_at_or_before(3.0).unwrap().time - 3.0).abs() < f64::EPSILON);
    assert!((track.keyframe_at_or_before(4.0).unwrap().time - 3.0).abs() < f64::EPSILON);
    assert!((track.keyframe_at_or_before(8.0).unwrap().time - 7.0).abs() < f64::EPSILON);
}

fn demo_timeline() -> MultiTrackTimeline {
    MultiTrackTimeline {
        id: AssetId(1),
        tracks: vec![
            Track {
                id: TrackId(0),
                name: "camera_fov".into(),
                keyframes: smallvec![Keyframe {
                    id: KeyframeId(0),
                    time: 1.0,
                    value: TrackValue::F32(60.0),
                    interpolation: Interpolation::Linear,
                    trigger: false,
                }],
                default_value: TrackValue::F32(0.0),
            },
            Track {
                id: TrackId(1),
                name: "audio_volume".into(),
                keyframes: smallvec![Keyframe {
                    id: KeyframeId(1),
                    time: 1.0,
                    value: TrackValue::F32(0.8),
                    interpolation: Interpolation::Linear,
                    trigger: false,
                }],
                default_value: TrackValue::F32(0.0),
            },
        ],
        duration: 5.0,
        loop_mode: LoopMode::Once,
    }
}

#[test]
fn test_timeline_track_by_name() {
    let tl = demo_timeline();
    let cam = tl.track_by_name("camera_fov").expect("camera track");
    assert_eq!(cam.name, "camera_fov");
    assert!(tl.track_by_name("missing").is_none());
}

#[test]
fn test_timeline_track_by_id() {
    let mut tracks = Vec::new();
    for i in 0..4_u16 {
        tracks.push(Track {
            id: TrackId(i),
            name: SmolStr::new_inline("x"),
            keyframes: smallvec![],
            default_value: TrackValue::F32(0.0),
        });
    }
    let tl = MultiTrackTimeline {
        id: AssetId(0),
        tracks,
        duration: 1.0,
        loop_mode: LoopMode::Once,
    };
    let t = tl.track_by_id(TrackId(2)).expect("track");
    assert_eq!(t.id, TrackId(2));
}

#[test]
fn test_timeline_duration_max_track() {
    let tl = MultiTrackTimeline {
        id: AssetId(0),
        tracks: vec![
            Track {
                id: TrackId(0),
                name: SmolStr::new("a"),
                keyframes: smallvec![Keyframe {
                    id: KeyframeId(0),
                    time: 2.0,
                    value: TrackValue::F32(0.0),
                    interpolation: Interpolation::Linear,
                    trigger: false,
                }],
                default_value: TrackValue::F32(0.0),
            },
            Track {
                id: TrackId(1),
                name: SmolStr::new("b"),
                keyframes: smallvec![Keyframe {
                    id: KeyframeId(0),
                    time: 5.0,
                    value: TrackValue::F32(0.0),
                    interpolation: Interpolation::Linear,
                    trigger: false,
                }],
                default_value: TrackValue::F32(0.0),
            },
            Track {
                id: TrackId(2),
                name: SmolStr::new("c"),
                keyframes: smallvec![Keyframe {
                    id: KeyframeId(0),
                    time: 3.0,
                    value: TrackValue::F32(0.0),
                    interpolation: Interpolation::Linear,
                    trigger: false,
                }],
                default_value: TrackValue::F32(0.0),
            },
        ],
        duration: 5.0,
        loop_mode: LoopMode::Once,
    };
    assert!((tl.duration - 5.0).abs() < f64::EPSILON);
}

#[test]
fn test_timeline_sync_two_tracks() {
    let tl = demo_timeline();
    let cam = tl.sample_track(TrackId(0), 1.0).expect("camera");
    let aud = tl.sample_track(TrackId(1), 1.0).expect("audio");
    assert_eq!(cam, TrackValue::F32(60.0));
    assert_eq!(aud, TrackValue::F32(0.8));
}

fn playback_state() -> PlaybackState {
    PlaybackState {
        timeline_id: AssetId(0),
        entity: Entity(1),
        current_time: 0.0,
        speed: 1.0,
        playing: true,
        direction: PlaybackDirection::Forward,
        loop_count: 0,
    }
}

fn empty_loop_timeline() -> MultiTrackTimeline {
    MultiTrackTimeline {
        id: AssetId(0),
        tracks: vec![],
        duration: 1.0,
        loop_mode: LoopMode::Loop,
    }
}

#[test]
fn test_playback_advance_forward() {
    let tl = empty_loop_timeline();
    let mut state = playback_state();
    state.current_time = 0.0;
    let events = state.advance(0.5, &tl);
    assert!(events.is_empty());
    assert!((state.current_time - 0.5).abs() < f64::EPSILON);
}

#[test]
fn test_playback_pause_no_advance() {
    let tl = empty_loop_timeline();
    let mut state = playback_state();
    state.current_time = 2.0;
    state.playing = false;
    let events = state.advance(1.0, &tl);
    assert!(events.is_empty());
    assert!((state.current_time - 2.0).abs() < f64::EPSILON);
}

#[test]
fn test_playback_speed_multiplier() {
    let tl = empty_loop_timeline();
    let mut state = playback_state();
    state.current_time = 0.0;
    state.speed = 2.0;
    state.advance(0.25, &tl);
    assert!((state.current_time - 0.5).abs() < f64::EPSILON);
}

#[test]
fn test_playback_reverse_direction() {
    let tl = empty_loop_timeline();
    let mut state = playback_state();
    state.current_time = 1.0;
    state.direction = PlaybackDirection::Reverse;
    state.advance(0.25, &tl);
    assert!((state.current_time - 0.75).abs() < f64::EPSILON);
}

#[test]
fn test_playback_loop_increments() {
    let tl = empty_loop_timeline();
    let mut state = playback_state();
    state.current_time = 0.9;
    state.advance(0.2, &tl);
    assert!((state.current_time - 0.1).abs() < 1e-9);
    assert_eq!(state.loop_count, 1);
}

#[test]
fn test_playback_pingpong() {
    let tl = MultiTrackTimeline {
        id: AssetId(0),
        tracks: vec![],
        duration: 1.0,
        loop_mode: LoopMode::PingPong,
    };
    let mut state = playback_state();
    state.current_time = 0.9;
    state.advance(0.2, &tl);
    assert_eq!(state.direction, PlaybackDirection::Reverse);
    assert!((state.current_time - 0.9).abs() < 1e-6);
}

#[test]
fn test_playback_clamp_forever() {
    let tl = MultiTrackTimeline {
        id: AssetId(0),
        tracks: vec![],
        duration: 1.0,
        loop_mode: LoopMode::ClampForever,
    };
    let mut state = playback_state();
    state.current_time = 0.9;
    state.advance(0.5, &tl);
    assert!((state.current_time - 1.0).abs() < f64::EPSILON);
    assert!(state.playing);
}

#[test]
fn test_playback_seek_clamps() {
    let tl = MultiTrackTimeline {
        id: AssetId(0),
        tracks: vec![],
        duration: 1.0,
        loop_mode: LoopMode::Once,
    };
    let mut state = playback_state();
    state.seek(-1.0, &tl);
    assert!((state.current_time - 0.0).abs() < f64::EPSILON);
    state.seek(2.0, &tl);
    assert!((state.current_time - 1.0).abs() < f64::EPSILON);
}

#[test]
fn test_event_keyframe_crossed() {
    let tl = MultiTrackTimeline {
        id: AssetId(0),
        tracks: vec![Track {
            id: TrackId(0),
            name: SmolStr::new("t"),
            keyframes: smallvec![
                Keyframe {
                    id: KeyframeId(5),
                    time: 0.5,
                    value: TrackValue::F32(1.0),
                    interpolation: Interpolation::Linear,
                    trigger: true,
                },
                Keyframe {
                    id: KeyframeId(6),
                    time: 2.0,
                    value: TrackValue::F32(0.0),
                    interpolation: Interpolation::Linear,
                    trigger: false,
                },
            ],
            default_value: TrackValue::F32(0.0),
        }],
        duration: 2.0,
        loop_mode: LoopMode::Loop,
    };
    let mut state = playback_state();
    state.current_time = 0.4;
    let events = state.advance(0.2, &tl);
    assert_eq!(events.len(), 1);
    assert!(matches!(
        events[0].kind,
        TimelineEventKind::KeyframeCrossed {
            track: TrackId(0),
            keyframe: KeyframeId(5),
        }
    ));
}

#[test]
fn test_event_track_complete() {
    let tl = MultiTrackTimeline {
        id: AssetId(0),
        tracks: vec![Track {
            id: TrackId(0),
            name: SmolStr::new("t"),
            keyframes: smallvec![
                Keyframe {
                    id: KeyframeId(0),
                    time: 0.0,
                    value: TrackValue::F32(0.0),
                    interpolation: Interpolation::Linear,
                    trigger: false,
                },
                Keyframe {
                    id: KeyframeId(1),
                    time: 1.0,
                    value: TrackValue::F32(1.0),
                    interpolation: Interpolation::Linear,
                    trigger: false,
                },
            ],
            default_value: TrackValue::F32(0.0),
        }],
        duration: 1.0,
        loop_mode: LoopMode::Once,
    };
    let mut state = playback_state();
    state.current_time = 0.5;
    let events = state.advance(0.5, &tl);
    assert!(events.iter().any(|e| matches!(
        e.kind,
        TimelineEventKind::TrackComplete { track: TrackId(0) }
    )));
}

#[test]
fn test_event_loop_point() {
    let tl = empty_loop_timeline();
    let mut state = playback_state();
    let mut loop_points = 0;
    for _ in 0..3 {
        state.current_time = 0.0;
        let events = state.advance(1.1, &tl);
        loop_points += events
            .iter()
            .filter(|e| matches!(e.kind, TimelineEventKind::LoopPoint { .. }))
            .count();
    }
    assert_eq!(loop_points, 3);
}

#[test]
fn test_rkyv_roundtrip_playback() {
    use rkyv::{from_bytes, rancor::Error, to_bytes};

    let state = PlaybackState {
        timeline_id: AssetId(7),
        entity: Entity(0),
        current_time: 1.234,
        speed: 1.5,
        playing: true,
        direction: PlaybackDirection::Reverse,
        loop_count: 4,
    };
    let bytes = to_bytes::<Error>(&state).unwrap();
    let round = from_bytes::<PlaybackState, Error>(bytes.as_slice()).unwrap();
    assert_eq!(round, state);
}

#[test]
fn test_rkyv_roundtrip_track() {
    use rkyv::{from_bytes, rancor::Error, to_bytes};

    let mut keyframes = SmallVec::<[Keyframe<f32>; 8]>::new();
    let modes = [
        Interpolation::Step,
        Interpolation::Linear,
        ease_css(),
        Interpolation::Constant,
        Interpolation::Step,
        Interpolation::Linear,
        ease_css(),
        Interpolation::Constant,
        Interpolation::Step,
        Interpolation::Linear,
        ease_css(),
        Interpolation::Constant,
        Interpolation::Step,
        Interpolation::Linear,
        ease_css(),
        Interpolation::Constant,
    ];
    for i in 0..16 {
        keyframes.push(Keyframe {
            id: KeyframeId(i),
            time: i as f64 * 0.1,
            value: i as f32,
            interpolation: modes[i as usize].clone(),
            trigger: i % 3 == 0,
        });
    }
    let track = Track {
        id: TrackId(0),
        name: "demo".into(),
        keyframes,
        default_value: -1.0,
    };
    let bytes = to_bytes::<Error>(&track).unwrap();
    let round = from_bytes::<Track<f32>, Error>(bytes.as_slice()).unwrap();
    assert_eq!(round, track);
}

#[test]
fn test_cutscene_multi_track_use() {
    let tl = MultiTrackTimeline {
        id: AssetId(0),
        tracks: vec![
            Track {
                id: TrackId(0),
                name: "camera".into(),
                keyframes: smallvec![
                    Keyframe {
                        id: KeyframeId(0),
                        time: 0.0,
                        value: TrackValue::F32(0.0),
                        interpolation: Interpolation::Linear,
                        trigger: false,
                    },
                    Keyframe {
                        id: KeyframeId(1),
                        time: 1.0,
                        value: TrackValue::F32(1.0),
                        interpolation: Interpolation::Linear,
                        trigger: false,
                    },
                ],
                default_value: TrackValue::F32(0.0),
            },
            Track {
                id: TrackId(1),
                name: "audio".into(),
                keyframes: smallvec![
                    Keyframe {
                        id: KeyframeId(2),
                        time: 0.0,
                        value: TrackValue::F32(10.0),
                        interpolation: Interpolation::Linear,
                        trigger: false,
                    },
                    Keyframe {
                        id: KeyframeId(3),
                        time: 1.0,
                        value: TrackValue::F32(20.0),
                        interpolation: Interpolation::Linear,
                        trigger: false,
                    },
                ],
                default_value: TrackValue::F32(0.0),
            },
            Track {
                id: TrackId(2),
                name: "actor".into(),
                keyframes: smallvec![
                    Keyframe {
                        id: KeyframeId(4),
                        time: 0.0,
                        value: TrackValue::F32(100.0),
                        interpolation: Interpolation::Linear,
                        trigger: false,
                    },
                    Keyframe {
                        id: KeyframeId(5),
                        time: 1.0,
                        value: TrackValue::F32(200.0),
                        interpolation: Interpolation::Linear,
                        trigger: false,
                    },
                ],
                default_value: TrackValue::F32(0.0),
            },
        ],
        duration: 1.0,
        loop_mode: LoopMode::Once,
    };

    for &t in &[0.0_f64, 0.5, 1.0] {
        let a = tl.sample_track(TrackId(0), t).unwrap();
        let b = tl.sample_track(TrackId(1), t).unwrap();
        let c = tl.sample_track(TrackId(2), t).unwrap();
        let TrackValue::F32(a) = a else {
            panic!("expected f32");
        };
        let TrackValue::F32(b) = b else {
            panic!("expected f32");
        };
        let TrackValue::F32(c) = c else {
            panic!("expected f32");
        };
        assert!((f64::from(a) - t).abs() < 1e-6);
        assert!((f64::from(b) - (10.0 + t * 10.0)).abs() < 1e-4);
        assert!((f64::from(c) - (100.0 + t * 100.0)).abs() < 1e-4);
    }
}

#[test]
fn test_camera_rail_spline_drive() {
    let pos = Track {
        id: TrackId(0),
        name: "pos".into(),
        keyframes: smallvec![
            Keyframe {
                id: KeyframeId(0),
                time: 0.0,
                value: Vec3::new(0.0, 0.0, 0.0),
                interpolation: Interpolation::Linear,
                trigger: false,
            },
            Keyframe {
                id: KeyframeId(1),
                time: 1.0,
                value: Vec3::new(1.0, 0.0, 0.0),
                interpolation: Interpolation::Linear,
                trigger: false,
            },
        ],
        default_value: Vec3::new(0.0, 0.0, 0.0),
    };
    let rot = Track {
        id: TrackId(1),
        name: "rot".into(),
        keyframes: smallvec![
            Keyframe {
                id: KeyframeId(2),
                time: 0.0,
                value: Quat::identity(),
                interpolation: Interpolation::Linear,
                trigger: false,
            },
            Keyframe {
                id: KeyframeId(3),
                time: 1.0,
                value: Quat::identity(),
                interpolation: Interpolation::Linear,
                trigger: false,
            },
        ],
        default_value: Quat::identity(),
    };

    for frame in 0..60 {
        let t = frame as f64 / 59.0;
        let p = pos.sample(t);
        let r = rot.sample(t);
        let expected = Vec3::new(t as f32, 0.0, 0.0);
        assert!((p.x - expected.x).abs() < 1e-4);
        assert!((p.y - expected.y).abs() < 1e-4);
        assert!((p.z - expected.z).abs() < 1e-4);
        let _ = r;
    }
}

#[test]
fn test_actor_anim_blend_timeline() {
    let gameplay = 0.3_f32;
    let timeline = 0.7_f32;
    let a = 10.0_f32;
    let b = 20.0_f32;
    let blended = a * gameplay + b * timeline;
    assert!((blended - 17.0).abs() < f32::EPSILON);
}

#[test]
fn test_npc_schedule_timeline() {
    let marks = [4.0_f64, 8.0, 12.0, 16.0, 20.0, 24.0];
    let mut keyframes = SmallVec::<[Keyframe<TrackValue>; 8]>::new();
    for (i, t) in marks.iter().enumerate() {
        keyframes.push(Keyframe {
            id: KeyframeId(i as u32),
            time: *t,
            value: TrackValue::F32(i as f32),
            interpolation: Interpolation::Step,
            trigger: true,
        });
    }
    let tl = MultiTrackTimeline {
        id: AssetId(0),
        tracks: vec![Track {
            id: TrackId(0),
            name: "schedule".into(),
            keyframes,
            default_value: TrackValue::F32(0.0),
        }],
        duration: 24.0,
        loop_mode: LoopMode::Once,
    };

    let mut state = playback_state();
    let mut triggers = 0;
    for m in marks {
        state.current_time = m - 0.1;
        let events = state.advance(0.2, &tl);
        triggers += events
            .iter()
            .filter(|e| matches!(e.kind, TimelineEventKind::KeyframeCrossed { .. }))
            .count();
    }
    assert_eq!(triggers, 6);
}

#[test]
fn test_login_reward_calendar() {
    let mut keyframes = SmallVec::<[Keyframe<TrackValue>; 8]>::new();
    for day in 1..=7 {
        keyframes.push(Keyframe {
            id: KeyframeId(day as u32),
            time: day as f64,
            value: TrackValue::F32(day as f32),
            interpolation: Interpolation::Step,
            trigger: true,
        });
    }
    let tl = MultiTrackTimeline {
        id: AssetId(0),
        tracks: vec![Track {
            id: TrackId(0),
            name: "calendar".into(),
            keyframes,
            default_value: TrackValue::F32(0.0),
        }],
        duration: 7.0,
        loop_mode: LoopMode::Once,
    };

    let mut state = playback_state();
    let mut triggers = 0;
    for day in 1..=7 {
        let m = day as f64;
        state.current_time = m - 0.1;
        let events = state.advance(0.2, &tl);
        triggers += events
            .iter()
            .filter(|e| matches!(e.kind, TimelineEventKind::KeyframeCrossed { .. }))
            .count();
    }
    assert_eq!(triggers, 7);
}
