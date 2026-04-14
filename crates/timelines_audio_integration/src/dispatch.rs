//! Timeline track → audio command dispatch (game thread).

use std::sync::Arc;

use crate::types::BusId;
use crate::types::{AssetHandle, AudioClip};
use crate::types::{AudioCommand, AudioTimestamp, AudioTrackTarget, SubtitleEvent};
use crate::VoiceManager;
use crate::{DialogueLineId, SegmentId};

/// Environment hooks for deterministic tests (no global log).
pub struct DispatchContext<'a> {
    /// Returns whether a clip handle is resident in the asset system.
    pub is_clip_loaded: &'a mut dyn FnMut(AssetHandle<AudioClip>) -> bool,
    /// Returns whether a music segment id exists in the baked graph.
    pub is_segment_valid: &'a dyn Fn(SegmentId) -> bool,
    /// Captured warnings (empty in production unless tests enable it).
    pub warnings: &'a mut Vec<String>,
    /// Emitter world position for spatial one-shots.
    pub position: [f32; 3],
    /// Emitter world velocity for spatial one-shots.
    pub velocity: [f32; 3],
    /// Emitter orientation as `(x, y, z, w)`.
    pub orientation: [f32; 4],
}

/// Outputs produced in one simulation tick for one track sample.
#[derive(Clone, Debug, PartialEq)]
pub struct DispatchOutcome {
    /// Timeline→audio commands to enqueue.
    pub commands: Vec<AudioCommand>,
    /// Subtitle bus events emitted alongside VO.
    pub subtitles: Vec<SubtitleEvent>,
}

/// Dispatches a resolved `AudioTrackTarget` using the integration rules.
/// Emits `Stop` plus a matching subtitle hide for early VO cancellation.
pub fn dispatch_subtitle_hide_for_stop(
    voice: crate::VoiceId,
    fade_samples: u32,
    line_id: DialogueLineId,
) -> DispatchOutcome {
    DispatchOutcome {
        commands: vec![AudioCommand::Stop {
            voice_id: voice,
            fade_samples,
            timestamp: AudioTimestamp::Immediate,
        }],
        subtitles: vec![SubtitleEvent::Hide { line_id }],
    }
}

/// Dispatches a resolved `AudioTrackTarget` using the integration rules.
pub fn dispatch_track_target(
    target: &AudioTrackTarget,
    voices: &mut VoiceManager,
    ctx: &mut DispatchContext<'_>,
) -> DispatchOutcome {
    let mut commands = Vec::new();
    let mut subtitles = Vec::new();

    match target {
        AudioTrackTarget::MusicCue { segment } => {
            if (ctx.is_segment_valid)(*segment) {
                commands.push(AudioCommand::MusicTransition { target: *segment });
            } else {
                ctx.warnings
                    .push(format!("unknown music segment {}", segment.0));
            }
        }
        AudioTrackTarget::VoiceOver {
            clip,
            priority,
            line_id,
            text,
            speaker,
            duration_ms,
        } => {
            if !(ctx.is_clip_loaded)(*clip) {
                ctx.warnings.push(format!("missing VO clip {}", clip.id));
                return DispatchOutcome {
                    commands,
                    subtitles,
                };
            }
            let Some(voice) = voices.allocate(*priority) else {
                ctx.warnings.push("voice allocation failed".to_string());
                return DispatchOutcome {
                    commands,
                    subtitles,
                };
            };
            commands.push(AudioCommand::Play {
                voice_id: voice,
                clip: *clip,
                bus: BusId::VOICE,
                priority: *priority,
                timestamp: AudioTimestamp::Immediate,
            });
            subtitles.push(SubtitleEvent::Show {
                line_id: *line_id,
                text: Arc::clone(text),
                speaker: *speaker,
                duration_ms: *duration_ms,
            });
        }
        AudioTrackTarget::OneShot {
            clip,
            bus,
            priority,
        } => {
            if !(ctx.is_clip_loaded)(*clip) {
                ctx.warnings.push(format!("missing SFX clip {}", clip.id));
                return DispatchOutcome {
                    commands,
                    subtitles,
                };
            }
            let Some(voice) = voices.allocate(*priority) else {
                ctx.warnings.push("voice allocation failed".to_string());
                return DispatchOutcome {
                    commands,
                    subtitles,
                };
            };
            commands.push(AudioCommand::Play {
                voice_id: voice,
                clip: *clip,
                bus: *bus,
                priority: *priority,
                timestamp: AudioTimestamp::Immediate,
            });
            commands.push(AudioCommand::UpdateSpatial {
                voice_id: voice,
                position: ctx.position,
                velocity: ctx.velocity,
                orientation: ctx.orientation,
            });
        }
        AudioTrackTarget::BusParam { bus, param } => {
            commands.push(AudioCommand::SetBusParam {
                bus_id: *bus,
                param: *param,
                value: 0.0,
            });
        }
        AudioTrackTarget::BusEffectParam {
            bus,
            node_index,
            param,
        } => {
            commands.push(AudioCommand::SetEffectParam {
                bus: *bus,
                node_index: *node_index,
                param: *param,
                value: 0.0,
            });
        }
        AudioTrackTarget::Stinger { request } => {
            commands.push(AudioCommand::TriggerStinger { request: *request });
        }
    }

    DispatchOutcome {
        commands,
        subtitles,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{AssetHandle, BusId, BusParam, StringId, VoicePriority};

    /// TC-IR-4.7.1.1 — music cue produces a transition command.
    #[test]
    fn tc_ir_4_7_1_1_music_cue_fires_transition() {
        let mut vm = VoiceManager::new(4);
        let mut warnings = Vec::new();
        let mut loaded = |_h: AssetHandle<AudioClip>| true;
        let mut ctx = DispatchContext {
            is_clip_loaded: &mut loaded,
            is_segment_valid: &|_| true,
            warnings: &mut warnings,
            position: [0.0; 3],
            velocity: [0.0; 3],
            orientation: [0.0, 0.0, 0.0, 1.0],
        };
        let out = dispatch_track_target(
            &AudioTrackTarget::MusicCue {
                segment: SegmentId(7),
            },
            &mut vm,
            &mut ctx,
        );
        assert_eq!(
            out.commands,
            vec![AudioCommand::MusicTransition {
                target: SegmentId(7)
            }]
        );
        assert!(out.subtitles.is_empty());
    }

    /// TC-IR-4.7.5.3 — unknown `SegmentId` produces a warning and no command.
    #[test]
    fn tc_ir_4_7_5_3_unknown_segment_no_op() {
        let mut vm = VoiceManager::new(1);
        let mut warnings = Vec::new();
        let mut loaded = |_h: AssetHandle<AudioClip>| true;
        let mut ctx = DispatchContext {
            is_clip_loaded: &mut loaded,
            is_segment_valid: &|_| false,
            warnings: &mut warnings,
            position: [0.0; 3],
            velocity: [0.0; 3],
            orientation: [0.0, 0.0, 0.0, 1.0],
        };
        let out = dispatch_track_target(
            &AudioTrackTarget::MusicCue {
                segment: SegmentId(99),
            },
            &mut vm,
            &mut ctx,
        );
        assert!(out.commands.is_empty());
        assert_eq!(warnings.len(), 1);
    }

    /// TC-IR-4.7.6.1 — VO emits `Play` and `SubtitleEvent::Show` together.
    #[test]
    fn tc_ir_4_7_6_1_dialogue_subtitle_same_frame() {
        let mut vm = VoiceManager::new(2);
        let mut warnings = Vec::new();
        let mut loaded = |_h: AssetHandle<AudioClip>| true;
        let mut ctx = DispatchContext {
            is_clip_loaded: &mut loaded,
            is_segment_valid: &|_| true,
            warnings: &mut warnings,
            position: [0.0; 3],
            velocity: [0.0; 3],
            orientation: [0.0, 0.0, 0.0, 1.0],
        };
        let text: Arc<str> = Arc::from("hello");
        let out = dispatch_track_target(
            &AudioTrackTarget::VoiceOver {
                clip: AssetHandle::new(5),
                priority: VoicePriority::High,
                line_id: DialogueLineId(42),
                text: Arc::clone(&text),
                speaker: Some(StringId(3)),
                duration_ms: 1234,
            },
            &mut vm,
            &mut ctx,
        );
        assert_eq!(out.subtitles.len(), 1);
        assert_eq!(
            out.subtitles[0],
            SubtitleEvent::Show {
                line_id: DialogueLineId(42),
                text: Arc::clone(&text),
                speaker: Some(StringId(3)),
                duration_ms: 1234,
            }
        );
        assert!(matches!(
            out.commands[0],
            AudioCommand::Play {
                bus: BusId::VOICE,
                ..
            }
        ));
    }

    /// TC-IR-4.7.6.2 — subtitle duration matches the VO metadata.
    #[test]
    fn tc_ir_4_7_6_2_subtitle_duration_matches_clip_metadata() {
        let mut vm = VoiceManager::new(1);
        let mut warnings = Vec::new();
        let mut loaded = |_h: AssetHandle<AudioClip>| true;
        let mut ctx = DispatchContext {
            is_clip_loaded: &mut loaded,
            is_segment_valid: &|_| true,
            warnings: &mut warnings,
            position: [0.0; 3],
            velocity: [0.0; 3],
            orientation: [0.0, 0.0, 0.0, 1.0],
        };
        let out = dispatch_track_target(
            &AudioTrackTarget::VoiceOver {
                clip: AssetHandle::new(1),
                priority: VoicePriority::Normal,
                line_id: DialogueLineId(1),
                text: Arc::from("x"),
                speaker: None,
                duration_ms: 5000,
            },
            &mut vm,
            &mut ctx,
        );
        assert_eq!(
            out.subtitles[0],
            SubtitleEvent::Show {
                line_id: DialogueLineId(1),
                text: Arc::from("x"),
                speaker: None,
                duration_ms: 5000,
            }
        );
    }

    /// TC-IR-4.7.3.1 / TC-IR-4.7.3.2 — one-shot emits spatial update with entity pose.
    #[test]
    fn tc_ir_4_7_3_1_and_3_2_sfx_spatial() {
        let mut vm = VoiceManager::new(2);
        let mut warnings = Vec::new();
        let mut loaded = |_h: AssetHandle<AudioClip>| true;
        let mut ctx = DispatchContext {
            is_clip_loaded: &mut loaded,
            is_segment_valid: &|_| true,
            warnings: &mut warnings,
            position: [1.0, 2.0, 3.0],
            velocity: [0.1, 0.0, 0.0],
            orientation: [0.0, 0.0, 0.0, 1.0],
        };
        let out = dispatch_track_target(
            &AudioTrackTarget::OneShot {
                clip: AssetHandle::new(2),
                bus: BusId::SFX,
                priority: VoicePriority::Normal,
            },
            &mut vm,
            &mut ctx,
        );
        assert_eq!(out.commands.len(), 2);
        assert!(matches!(out.commands[0], AudioCommand::Play { .. }));
        assert_eq!(
            out.commands[1],
            AudioCommand::UpdateSpatial {
                voice_id: match out.commands[0] {
                    AudioCommand::Play { voice_id, .. } => voice_id,
                    _ => unreachable!(),
                },
                position: [1.0, 2.0, 3.0],
                velocity: [0.1, 0.0, 0.0],
                orientation: [0.0, 0.0, 0.0, 1.0],
            }
        );
    }

    /// TC-IR-4.7.5.1 — bus gain automation maps to `SetBusParam`.
    #[test]
    fn tc_ir_4_7_5_1_bus_gain_automation() {
        let mut vm = VoiceManager::new(1);
        let mut warnings = Vec::new();
        let mut loaded = |_h: AssetHandle<AudioClip>| true;
        let mut ctx = DispatchContext {
            is_clip_loaded: &mut loaded,
            is_segment_valid: &|_| true,
            warnings: &mut warnings,
            position: [0.0; 3],
            velocity: [0.0; 3],
            orientation: [0.0, 0.0, 0.0, 1.0],
        };
        let out = dispatch_track_target(
            &AudioTrackTarget::BusParam {
                bus: BusId(10),
                param: BusParam::Gain,
            },
            &mut vm,
            &mut ctx,
        );
        assert_eq!(
            out.commands,
            vec![AudioCommand::SetBusParam {
                bus_id: BusId(10),
                param: BusParam::Gain,
                value: 0.0,
            }]
        );
    }

    /// TC-IR-4.7.5.2 — effect automation maps to `SetEffectParam`.
    #[test]
    fn tc_ir_4_7_5_2_effect_param_automation() {
        let mut vm = VoiceManager::new(1);
        let mut warnings = Vec::new();
        let mut loaded = |_h: AssetHandle<AudioClip>| true;
        let mut ctx = DispatchContext {
            is_clip_loaded: &mut loaded,
            is_segment_valid: &|_| true,
            warnings: &mut warnings,
            position: [0.0; 3],
            velocity: [0.0; 3],
            orientation: [0.0, 0.0, 0.0, 1.0],
        };
        let out = dispatch_track_target(
            &AudioTrackTarget::BusEffectParam {
                bus: BusId(4),
                node_index: 0,
                param: crate::ParamId(77),
            },
            &mut vm,
            &mut ctx,
        );
        assert_eq!(
            out.commands,
            vec![AudioCommand::SetEffectParam {
                bus: BusId(4),
                node_index: 0,
                param: crate::ParamId(77),
                value: 0.0,
            }]
        );
    }

    /// TC-IR-4.7.3.3 — missing asset logs a warning and does not enqueue `Play`.
    #[test]
    fn tc_ir_4_7_3_3_missing_asset_no_play() {
        let mut vm = VoiceManager::new(1);
        let mut warnings = Vec::new();
        let mut loaded = |_h: AssetHandle<AudioClip>| false;
        let mut ctx = DispatchContext {
            is_clip_loaded: &mut loaded,
            is_segment_valid: &|_| true,
            warnings: &mut warnings,
            position: [0.0; 3],
            velocity: [0.0; 3],
            orientation: [0.0, 0.0, 0.0, 1.0],
        };
        let out = dispatch_track_target(
            &AudioTrackTarget::OneShot {
                clip: AssetHandle::new(123),
                bus: BusId::SFX,
                priority: VoicePriority::Normal,
            },
            &mut vm,
            &mut ctx,
        );
        assert!(out.commands.is_empty());
        assert_eq!(warnings.len(), 1);
    }

    /// TC-IR-4.7.N.5 — invalid clip id path is treated as missing without panic.
    #[test]
    fn tc_ir_4_7_n_5_unknown_asset_fallback() {
        let mut vm = VoiceManager::new(1);
        let mut warnings = Vec::new();
        let mut loaded = |_h: AssetHandle<AudioClip>| false;
        let mut ctx = DispatchContext {
            is_clip_loaded: &mut loaded,
            is_segment_valid: &|_| true,
            warnings: &mut warnings,
            position: [0.0; 3],
            velocity: [0.0; 3],
            orientation: [0.0, 0.0, 0.0, 1.0],
        };
        let out = dispatch_track_target(
            &AudioTrackTarget::OneShot {
                clip: AssetHandle::new(0),
                bus: BusId::SFX,
                priority: VoicePriority::Normal,
            },
            &mut vm,
            &mut ctx,
        );
        assert!(out.commands.is_empty());
        assert!(!warnings.is_empty());
    }

    /// TC-IR-4.7.6.3 — early `Stop` pairs with `SubtitleEvent::Hide`.
    #[test]
    fn tc_ir_4_7_6_3_subtitle_hide_on_stop() {
        let out = dispatch_subtitle_hide_for_stop(crate::VoiceId(3), 256, DialogueLineId(9));
        assert_eq!(
            out.commands,
            vec![AudioCommand::Stop {
                voice_id: crate::VoiceId(3),
                fade_samples: 256,
                timestamp: AudioTimestamp::Immediate,
            }]
        );
        assert_eq!(
            out.subtitles,
            vec![SubtitleEvent::Hide {
                line_id: DialogueLineId(9)
            }]
        );
    }
}
