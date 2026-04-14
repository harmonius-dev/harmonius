//! Bridge entry points from physics events to audio commands.

use std::collections::HashMap;

use crate::command_queue::BoundedAudioCommandQueue;
use crate::commands::{AudioCommand, AudioTimestamp, BusId, VoiceParam, VoicePriority};
use crate::cooldown::ImpactCooldownTracker;
use crate::events::{
    CollisionEnded, CollisionPersisted, CollisionStarted, TriggerEnter, TriggerExit,
};
use crate::friction_track::ActiveFrictionSounds;
use crate::ids::{Entity, VoiceId, VoiceIdAllocator};
use crate::surface::SurfaceType;
use crate::tables::{FrictionSoundTable, ImpactSoundTable};
use crate::zones::TriggerZoneSnapshot;

/// Tracks ambient loop [`crate::ids::VoiceId`] values per trigger entity between enter and exit.
#[derive(Debug, Default)]
pub struct AmbientVoiceMap {
    voices: HashMap<u32, VoiceId>,
}

impl AmbientVoiceMap {
    /// Constructs an empty map.
    pub fn new() -> Self {
        Self::default()
    }

    /// Records a started ambient voice for `trigger`.
    pub fn insert(&mut self, trigger: Entity, voice: VoiceId) {
        self.voices.insert(trigger.0, voice);
    }

    /// Removes and returns the ambient voice for `trigger` when present.
    pub fn remove(&mut self, trigger: Entity) -> Option<VoiceId> {
        self.voices.remove(&trigger.0)
    }
}

/// Handles [`CollisionStarted`] by optionally enqueueing an impact [`AudioCommand::Play`].
pub fn handle_collision_impact<M>(
    event: &CollisionStarted,
    mut material: M,
    table: &ImpactSoundTable,
    queue: &mut BoundedAudioCommandQueue,
    cooldowns: &mut ImpactCooldownTracker,
    voices: &mut VoiceIdAllocator,
    clip_salt: u32,
) where
    M: FnMut(Entity) -> Option<SurfaceType>,
{
    if event.entity_a == event.entity_b {
        return;
    }
    if event.total_impulse == 0.0 {
        return;
    }
    let Some(contact) = event.contacts.first() else {
        return;
    };
    let surf_a = material(event.entity_a).unwrap_or(SurfaceType::Default);
    let surf_b = material(event.entity_b).unwrap_or(SurfaceType::Default);
    let set = table.get(surf_a, surf_b);
    if event.total_impulse < set.threshold {
        return;
    }
    if cooldowns.is_cooling(event.entity_a, event.entity_b) {
        return;
    }
    let Some(clip) = set.pick_clip(clip_salt) else {
        return;
    };
    let impulse = event.total_impulse;
    let gain = (impulse / 100.0).clamp(0.1, 1.0);
    let pitch = 0.9 + (impulse / 200.0).min(0.3);
    let _ = queue.send(AudioCommand::Play {
        voice_id: voices.transient(),
        clip,
        bus: BusId::Sfx,
        priority: VoicePriority::Medium,
        position: Some(contact.world_point),
        timestamp: AudioTimestamp::Immediate,
        gain,
        pitch,
    });
    cooldowns.start(event.entity_a, event.entity_b, set.cooldown_sec);
}

/// Handles [`CollisionPersisted`] / [`CollisionEnded`] for sliding friction audio.
pub fn handle_sliding_friction<M>(
    persisted: &[CollisionPersisted],
    ended: &[CollisionEnded],
    mut material: M,
    table: &FrictionSoundTable,
    queue: &mut BoundedAudioCommandQueue,
    active: &mut ActiveFrictionSounds,
    voices: &mut VoiceIdAllocator,
) where
    M: FnMut(Entity) -> Option<SurfaceType>,
{
    for event in persisted {
        let tang_speed = event.tangential_velocity().length();
        if tang_speed < 0.01 {
            if let Some(voice) = active.remove(event.entity_a, event.entity_b) {
                let _ = queue.send(AudioCommand::Stop {
                    voice_id: voice,
                    fade_samples: 960,
                    timestamp: AudioTimestamp::Immediate,
                });
            }
            continue;
        }
        let gain = (tang_speed / 10.0).clamp(0.05, 1.0);
        if let Some(voice) = active.get(event.entity_a, event.entity_b) {
            let _ = queue.send(AudioCommand::SetParam {
                voice_id: voice,
                param: VoiceParam::Gain,
                value: gain,
                timestamp: AudioTimestamp::Immediate,
            });
        } else {
            let surf_a = material(event.entity_a).unwrap_or(SurfaceType::Default);
            let surf_b = material(event.entity_b).unwrap_or(SurfaceType::Default);
            let clip = table.pick(surf_a, surf_b);
            let voice = voices.transient();
            let Some(contact) = event.contacts.first() else {
                continue;
            };
            let _ = queue.send(AudioCommand::Play {
                voice_id: voice,
                clip,
                bus: BusId::Sfx,
                priority: VoicePriority::Low,
                position: Some(contact.world_point),
                timestamp: AudioTimestamp::Immediate,
                gain,
                pitch: 1.0,
            });
            active.insert(event.entity_a, event.entity_b, voice);
        }
    }
    for event in ended {
        if let Some(voice) = active.remove(event.entity_a, event.entity_b) {
            let _ = queue.send(AudioCommand::Stop {
                voice_id: voice,
                fade_samples: 960,
                timestamp: AudioTimestamp::Immediate,
            });
        }
    }
}

/// Handles [`TriggerEnter`] / [`TriggerExit`] for reverb zones and ambient loops.
pub fn handle_trigger_zone<Z>(
    enter: &[TriggerEnter],
    exit: &[TriggerExit],
    mut zone_for: Z,
    ambient_voices: &mut AmbientVoiceMap,
    queue: &mut BoundedAudioCommandQueue,
    voices: &mut VoiceIdAllocator,
) where
    Z: FnMut(Entity) -> TriggerZoneSnapshot,
{
    for event in enter {
        let snap = zone_for(event.trigger_entity);
        if let Some(zone) = snap.reverb {
            let _ = queue.send(AudioCommand::ActivateReverb {
                zone_id: zone.id,
                params: zone.params,
            });
        }
        if let Some(ambient) = snap.ambient {
            let voice = voices.transient();
            let _ = queue.send(AudioCommand::Play {
                voice_id: voice,
                clip: ambient.clip,
                bus: BusId::Ambient,
                priority: VoicePriority::Low,
                position: None,
                timestamp: AudioTimestamp::Immediate,
                gain: ambient.gain,
                pitch: 1.0,
            });
            ambient_voices.insert(event.trigger_entity, voice);
        }
    }
    for event in exit {
        let snap = zone_for(event.trigger_entity);
        if let Some(zone) = snap.reverb {
            let _ = queue.send(AudioCommand::DeactivateReverb { zone_id: zone.id });
        }
        if let Some(ambient) = snap.ambient {
            let voice = ambient_voices
                .remove(event.trigger_entity)
                .or(ambient.active_voice);
            if let Some(voice_id) = voice {
                let _ = queue.send(AudioCommand::Stop {
                    voice_id,
                    fade_samples: 4800,
                    timestamp: AudioTimestamp::Immediate,
                });
            }
        }
    }
}
