//! Integration tests mapped from `docs/design/integration/audio-physics-test-cases.md`.

use std::collections::HashMap;

use crate::apply_set_param_wire;
use crate::bridge::{
    handle_collision_impact, handle_sliding_friction, handle_trigger_zone, AmbientVoiceMap,
};
use crate::command_queue::BoundedAudioCommandQueue;
use crate::commands::{AudioCommand, BusId, VoicePriority};
use crate::cooldown::ImpactCooldownTracker;
use crate::events::{
    CollisionEnded, CollisionPersisted, CollisionStarted, ContactPoint, TriggerEnter, TriggerExit,
};
use crate::friction_track::ActiveFrictionSounds;
use crate::ids::{AssetHandle, AudioClip, Entity, VoiceIdAllocator};
use crate::math::Vec3;
use crate::surface::SurfaceType;
use crate::tables::{FrictionSoundTable, ImpactSoundSet, ImpactSoundTable};
use crate::zones::{AmbientLoop, ReverbParams, ReverbZone, ReverbZoneId, TriggerZoneSnapshot};

fn empty_impact_table_with_default(default_clip: AssetHandle<AudioClip>) -> ImpactSoundTable {
    let empty_cell = ImpactSoundSet {
        clips: [None; 8],
        threshold: 0.0,
        cooldown_sec: 0.0,
    };
    let default_set = ImpactSoundSet {
        clips: [Some(default_clip), None, None, None, None, None, None, None],
        threshold: 1.0,
        cooldown_sec: 0.2,
    };
    ImpactSoundTable {
        entries: [[empty_cell; crate::SURFACE_TYPE_COUNT]; crate::SURFACE_TYPE_COUNT],
        default: default_set,
    }
}

fn table_with_pair(
    a: SurfaceType,
    b: SurfaceType,
    clip: AssetHandle<AudioClip>,
    threshold: f32,
    cooldown: f32,
) -> ImpactSoundTable {
    let mut t = empty_impact_table_with_default(AssetHandle::new(999));
    let (lo, hi) = if a.index() <= b.index() {
        (a.index(), b.index())
    } else {
        (b.index(), a.index())
    };
    t.entries[lo][hi] = ImpactSoundSet {
        clips: [Some(clip), None, None, None, None, None, None, None],
        threshold,
        cooldown_sec: cooldown,
    };
    t
}

#[test]
fn tc_ir_1_8_1_1_collision_sound() {
    let clip = AssetHandle::<AudioClip>::new(7);
    let table = table_with_pair(SurfaceType::Default, SurfaceType::Default, clip, 1.0, 0.0);
    let mut q = BoundedAudioCommandQueue::new(1024);
    let mut cd = ImpactCooldownTracker::new();
    let mut voices = VoiceIdAllocator::default();
    let event = CollisionStarted {
        entity_a: Entity(1),
        entity_b: Entity(2),
        contacts: vec![ContactPoint {
            world_point: Vec3::ZERO,
        }],
        normal: Vec3::new(0.0, 1.0, 0.0),
        total_impulse: 50.0,
    };
    handle_collision_impact(
        &event,
        |_| Some(SurfaceType::Default),
        &table,
        &mut q,
        &mut cd,
        &mut voices,
        0,
    );
    let drained = q.drain_all();
    assert!(
        drained
            .iter()
            .any(|c| matches!(c, AudioCommand::Play { .. })),
        "expected Play command"
    );
}

#[test]
fn tc_ir_1_8_1_2_contact_point_position() {
    let clip = AssetHandle::<AudioClip>::new(1);
    let table = table_with_pair(SurfaceType::Default, SurfaceType::Default, clip, 1.0, 0.0);
    let mut q = BoundedAudioCommandQueue::new(1024);
    let mut cd = ImpactCooldownTracker::new();
    let mut voices = VoiceIdAllocator::default();
    let p = Vec3::new(3.0, 0.0, 0.0);
    let event = CollisionStarted {
        entity_a: Entity(1),
        entity_b: Entity(2),
        contacts: vec![ContactPoint { world_point: p }],
        normal: Vec3::new(0.0, 1.0, 0.0),
        total_impulse: 50.0,
    };
    handle_collision_impact(
        &event,
        |_| Some(SurfaceType::Default),
        &table,
        &mut q,
        &mut cd,
        &mut voices,
        0,
    );
    let play = q.drain_all().into_iter().find_map(|c| match c {
        AudioCommand::Play { position, .. } => position,
        _ => None,
    });
    assert_eq!(play, Some(p));
}

#[test]
fn tc_ir_1_8_1_3_below_threshold() {
    let clip = AssetHandle::<AudioClip>::new(1);
    let table = table_with_pair(SurfaceType::Default, SurfaceType::Default, clip, 10.0, 0.0);
    let mut q = BoundedAudioCommandQueue::new(1024);
    let mut cd = ImpactCooldownTracker::new();
    let mut voices = VoiceIdAllocator::default();
    let event = CollisionStarted {
        entity_a: Entity(1),
        entity_b: Entity(2),
        contacts: vec![ContactPoint {
            world_point: Vec3::ZERO,
        }],
        normal: Vec3::new(0.0, 1.0, 0.0),
        total_impulse: 5.0,
    };
    handle_collision_impact(
        &event,
        |_| Some(SurfaceType::Default),
        &table,
        &mut q,
        &mut cd,
        &mut voices,
        0,
    );
    assert!(q.drain_all().is_empty());
}

#[test]
fn tc_ir_1_8_1_4_cooldown_suppresses_repeat() {
    let clip = AssetHandle::<AudioClip>::new(1);
    let table = table_with_pair(SurfaceType::Default, SurfaceType::Default, clip, 1.0, 1.0);
    let mut q = BoundedAudioCommandQueue::new(1024);
    let mut cd = ImpactCooldownTracker::new();
    let mut voices = VoiceIdAllocator::default();
    let event = CollisionStarted {
        entity_a: Entity(1),
        entity_b: Entity(2),
        contacts: vec![ContactPoint {
            world_point: Vec3::ZERO,
        }],
        normal: Vec3::new(0.0, 1.0, 0.0),
        total_impulse: 50.0,
    };
    handle_collision_impact(
        &event,
        |_| Some(SurfaceType::Default),
        &table,
        &mut q,
        &mut cd,
        &mut voices,
        0,
    );
    handle_collision_impact(
        &event,
        |_| Some(SurfaceType::Default),
        &table,
        &mut q,
        &mut cd,
        &mut voices,
        0,
    );
    let plays = q
        .drain_all()
        .into_iter()
        .filter(|c| matches!(c, AudioCommand::Play { .. }))
        .count();
    assert_eq!(plays, 1);
}

#[test]
fn tc_ir_1_8_1_5_cooldown_expiry_re_enables() {
    let clip = AssetHandle::<AudioClip>::new(1);
    let table = table_with_pair(SurfaceType::Default, SurfaceType::Default, clip, 1.0, 0.5);
    let mut q = BoundedAudioCommandQueue::new(1024);
    let mut cd = ImpactCooldownTracker::new();
    let mut voices = VoiceIdAllocator::default();
    let event = CollisionStarted {
        entity_a: Entity(1),
        entity_b: Entity(2),
        contacts: vec![ContactPoint {
            world_point: Vec3::ZERO,
        }],
        normal: Vec3::new(0.0, 1.0, 0.0),
        total_impulse: 50.0,
    };
    handle_collision_impact(
        &event,
        |_| Some(SurfaceType::Default),
        &table,
        &mut q,
        &mut cd,
        &mut voices,
        0,
    );
    cd.tick(0.6);
    handle_collision_impact(
        &event,
        |_| Some(SurfaceType::Default),
        &table,
        &mut q,
        &mut cd,
        &mut voices,
        0,
    );
    let plays = q
        .drain_all()
        .into_iter()
        .filter(|c| matches!(c, AudioCommand::Play { .. }))
        .count();
    assert_eq!(plays, 2);
}

#[test]
fn tc_ir_1_8_2_1_metal_on_wood_clip() {
    let metal_wood = AssetHandle::<AudioClip>::new(42);
    let mut table = empty_impact_table_with_default(AssetHandle::new(999));
    table.entries[SurfaceType::Metal as usize][SurfaceType::Wood as usize] = ImpactSoundSet {
        clips: [Some(metal_wood), None, None, None, None, None, None, None],
        threshold: 1.0,
        cooldown_sec: 0.0,
    };
    let mut q = BoundedAudioCommandQueue::new(1024);
    let mut cd = ImpactCooldownTracker::new();
    let mut voices = VoiceIdAllocator::default();
    let mut mats = HashMap::new();
    mats.insert(Entity(1), SurfaceType::Metal);
    mats.insert(Entity(2), SurfaceType::Wood);
    let event = CollisionStarted {
        entity_a: Entity(1),
        entity_b: Entity(2),
        contacts: vec![ContactPoint {
            world_point: Vec3::ZERO,
        }],
        normal: Vec3::new(0.0, 1.0, 0.0),
        total_impulse: 50.0,
    };
    handle_collision_impact(
        &event,
        |e| mats.get(&e).copied(),
        &table,
        &mut q,
        &mut cd,
        &mut voices,
        0,
    );
    let clip = q.drain_all().into_iter().find_map(|c| match c {
        AudioCommand::Play { clip, .. } => Some(clip),
        _ => None,
    });
    assert_eq!(clip, Some(metal_wood));
}

#[test]
fn tc_ir_1_8_2_2_unknown_fallback() {
    let def_clip = AssetHandle::<AudioClip>::new(200);
    let table = empty_impact_table_with_default(def_clip);
    let mut q = BoundedAudioCommandQueue::new(1024);
    let mut cd = ImpactCooldownTracker::new();
    let mut voices = VoiceIdAllocator::default();
    let event = CollisionStarted {
        entity_a: Entity(1),
        entity_b: Entity(2),
        contacts: vec![ContactPoint {
            world_point: Vec3::ZERO,
        }],
        normal: Vec3::new(0.0, 1.0, 0.0),
        total_impulse: 50.0,
    };
    handle_collision_impact(&event, |_| None, &table, &mut q, &mut cd, &mut voices, 0);
    let clip = q.drain_all().into_iter().find_map(|c| match c {
        AudioCommand::Play { clip, .. } => Some(clip),
        _ => None,
    });
    assert_eq!(clip, Some(def_clip));
}

#[test]
fn tc_ir_1_8_2_3_pair_order_symmetric() {
    let clip_ab = AssetHandle::<AudioClip>::new(55);
    let table = table_with_pair(SurfaceType::Wood, SurfaceType::Metal, clip_ab, 1.0, 0.0);
    let mut mats = HashMap::new();
    mats.insert(Entity(1), SurfaceType::Wood);
    mats.insert(Entity(2), SurfaceType::Metal);
    let mut q1 = BoundedAudioCommandQueue::new(1024);
    let mut cd1 = ImpactCooldownTracker::new();
    let v1 = VoiceIdAllocator::default();
    let event1 = CollisionStarted {
        entity_a: Entity(1),
        entity_b: Entity(2),
        contacts: vec![ContactPoint {
            world_point: Vec3::ZERO,
        }],
        normal: Vec3::ZERO,
        total_impulse: 40.0,
    };
    handle_collision_impact(
        &event1,
        |e| mats.get(&e).copied(),
        &table,
        &mut q1,
        &mut cd1,
        &v1,
        0,
    );
    let c1 = q1.drain_all().into_iter().find_map(|c| match c {
        AudioCommand::Play { clip, .. } => Some(clip),
        _ => None,
    });
    let mut q2 = BoundedAudioCommandQueue::new(1024);
    let mut cd2 = ImpactCooldownTracker::new();
    let v2 = VoiceIdAllocator::default();
    let event2 = CollisionStarted {
        entity_a: Entity(2),
        entity_b: Entity(1),
        contacts: vec![ContactPoint {
            world_point: Vec3::ZERO,
        }],
        normal: Vec3::ZERO,
        total_impulse: 40.0,
    };
    handle_collision_impact(
        &event2,
        |e| mats.get(&e).copied(),
        &table,
        &mut q2,
        &mut cd2,
        &v2,
        0,
    );
    let c2 = q2.drain_all().into_iter().find_map(|c| match c {
        AudioCommand::Play { clip, .. } => Some(clip),
        _ => None,
    });
    assert_eq!(c1, c2);
}

#[test]
fn tc_ir_1_8_3_1_hard_hit_loud() {
    let clip = AssetHandle::<AudioClip>::new(1);
    let table = table_with_pair(SurfaceType::Default, SurfaceType::Default, clip, 1.0, 0.0);
    let mut q = BoundedAudioCommandQueue::new(1024);
    let mut cd = ImpactCooldownTracker::new();
    let mut voices = VoiceIdAllocator::default();
    let event = CollisionStarted {
        entity_a: Entity(1),
        entity_b: Entity(2),
        contacts: vec![ContactPoint {
            world_point: Vec3::ZERO,
        }],
        normal: Vec3::ZERO,
        total_impulse: 100.0,
    };
    handle_collision_impact(
        &event,
        |_| Some(SurfaceType::Default),
        &table,
        &mut q,
        &mut cd,
        &mut voices,
        0,
    );
    let gain = q.drain_all().into_iter().find_map(|c| match c {
        AudioCommand::Play { gain, .. } => Some(gain),
        _ => None,
    });
    assert!((gain.unwrap() - 1.0).abs() < 1e-3);
}

#[test]
fn tc_ir_1_8_3_2_soft_tap_quiet() {
    let clip = AssetHandle::<AudioClip>::new(1);
    let table = table_with_pair(SurfaceType::Default, SurfaceType::Default, clip, 1.0, 0.0);
    let mut q = BoundedAudioCommandQueue::new(1024);
    let mut cd = ImpactCooldownTracker::new();
    let mut voices = VoiceIdAllocator::default();
    let event = CollisionStarted {
        entity_a: Entity(1),
        entity_b: Entity(2),
        contacts: vec![ContactPoint {
            world_point: Vec3::ZERO,
        }],
        normal: Vec3::ZERO,
        total_impulse: 5.0,
    };
    handle_collision_impact(
        &event,
        |_| Some(SurfaceType::Default),
        &table,
        &mut q,
        &mut cd,
        &mut voices,
        0,
    );
    let gain = q.drain_all().into_iter().find_map(|c| match c {
        AudioCommand::Play { gain, .. } => Some(gain),
        _ => None,
    });
    assert!((gain.unwrap() - 0.1).abs() < 1e-3);
}

#[test]
fn tc_ir_1_8_3_3_pitch_scales() {
    let clip = AssetHandle::<AudioClip>::new(1);
    let table = table_with_pair(SurfaceType::Default, SurfaceType::Default, clip, 1.0, 0.0);
    let mut q = BoundedAudioCommandQueue::new(1024);
    let mut cd = ImpactCooldownTracker::new();
    let mut voices = VoiceIdAllocator::default();
    let event = CollisionStarted {
        entity_a: Entity(1),
        entity_b: Entity(2),
        contacts: vec![ContactPoint {
            world_point: Vec3::ZERO,
        }],
        normal: Vec3::ZERO,
        total_impulse: 80.0,
    };
    handle_collision_impact(
        &event,
        |_| Some(SurfaceType::Default),
        &table,
        &mut q,
        &mut cd,
        &mut voices,
        0,
    );
    let pitch = q.drain_all().into_iter().find_map(|c| match c {
        AudioCommand::Play { pitch, .. } => Some(pitch),
        _ => None,
    });
    assert!(pitch.unwrap() > 1.0);
}

#[test]
fn tc_ir_1_8_4_1_trigger_enter_reverb() {
    let mut q = BoundedAudioCommandQueue::new(1024);
    let mut voices = VoiceIdAllocator::default();
    let mut ambient_rt = AmbientVoiceMap::new();
    let enter = [TriggerEnter {
        trigger_entity: Entity(10),
        other_entity: Entity(20),
    }];
    let zone = ReverbZone {
        id: ReverbZoneId(5),
        params: ReverbParams { wet: 0.3 },
    };
    handle_trigger_zone(
        &enter,
        &[],
        |_e| TriggerZoneSnapshot {
            reverb: Some(zone),
            ambient: None,
        },
        &mut ambient_rt,
        &mut q,
        &mut voices,
    );
    assert!(q.drain_all().iter().any(|c| matches!(
        c,
        AudioCommand::ActivateReverb { zone_id, .. } if *zone_id == ReverbZoneId(5)
    )));
}

#[test]
fn tc_ir_1_8_4_2_trigger_exit_reverb() {
    let mut q = BoundedAudioCommandQueue::new(1024);
    let mut voices = VoiceIdAllocator::default();
    let mut ambient_rt = AmbientVoiceMap::new();
    let exit = [TriggerExit {
        trigger_entity: Entity(10),
        other_entity: Entity(20),
    }];
    let zone = ReverbZone {
        id: ReverbZoneId(5),
        params: ReverbParams { wet: 0.3 },
    };
    handle_trigger_zone(
        &[],
        &exit,
        |_e| TriggerZoneSnapshot {
            reverb: Some(zone),
            ambient: None,
        },
        &mut ambient_rt,
        &mut q,
        &mut voices,
    );
    assert!(q.drain_all().iter().any(|c| matches!(
        c,
        AudioCommand::DeactivateReverb { zone_id } if *zone_id == ReverbZoneId(5)
    )));
}

#[test]
fn tc_ir_1_8_4_3_ambient_loop_start() {
    let clip = AssetHandle::<AudioClip>::new(77);
    let mut q = BoundedAudioCommandQueue::new(1024);
    let mut voices = VoiceIdAllocator::default();
    let mut ambient_rt = AmbientVoiceMap::new();
    let enter = [TriggerEnter {
        trigger_entity: Entity(10),
        other_entity: Entity(20),
    }];
    let ambient = AmbientLoop {
        clip,
        gain: 0.5,
        active_voice: None,
    };
    handle_trigger_zone(
        &enter,
        &[],
        |_e| TriggerZoneSnapshot {
            reverb: None,
            ambient: Some(ambient),
        },
        &mut ambient_rt,
        &mut q,
        &mut voices,
    );
    assert!(q.drain_all().iter().any(|c| matches!(
        c,
        AudioCommand::Play {
            bus: BusId::Ambient,
            ..
        }
    )));
}

#[test]
fn tc_ir_1_8_4_4_ambient_loop_stop() {
    let clip = AssetHandle::<AudioClip>::new(77);
    let mut q = BoundedAudioCommandQueue::new(1024);
    let mut voices = VoiceIdAllocator::default();
    let mut ambient_rt = AmbientVoiceMap::new();
    let enter = [TriggerEnter {
        trigger_entity: Entity(10),
        other_entity: Entity(20),
    }];
    let ambient = AmbientLoop {
        clip,
        gain: 0.5,
        active_voice: None,
    };
    handle_trigger_zone(
        &enter,
        &[],
        |_e| TriggerZoneSnapshot {
            reverb: None,
            ambient: Some(ambient),
        },
        &mut ambient_rt,
        &mut q,
        &mut voices,
    );
    q.drain_all();
    let exit = [TriggerExit {
        trigger_entity: Entity(10),
        other_entity: Entity(20),
    }];
    handle_trigger_zone(
        &[],
        &exit,
        |_e| TriggerZoneSnapshot {
            reverb: None,
            ambient: Some(ambient),
        },
        &mut ambient_rt,
        &mut q,
        &mut voices,
    );
    let cmds = q.drain_all();
    assert!(cmds.iter().any(|c| matches!(
        c,
        AudioCommand::Stop {
            fade_samples: 4800,
            ..
        }
    )));
}

#[test]
fn tc_ir_1_8_5_1_sliding_friction_play() {
    let clip = AssetHandle::<AudioClip>::new(3);
    let mut ft = FrictionSoundTable {
        entries: [[None; crate::SURFACE_TYPE_COUNT]; crate::SURFACE_TYPE_COUNT],
        default: clip,
    };
    ft.entries[0][0] = Some(clip);
    let mut q = BoundedAudioCommandQueue::new(1024);
    let mut active = ActiveFrictionSounds::new();
    let mut voices = VoiceIdAllocator::default();
    let persisted = [CollisionPersisted {
        entity_a: Entity(1),
        entity_b: Entity(2),
        contacts: vec![ContactPoint {
            world_point: Vec3::ZERO,
        }],
        normal: Vec3::new(0.0, 1.0, 0.0),
        total_impulse: 1.0,
        tangential_velocity: Vec3::new(2.0, 0.0, 0.0),
    }];
    handle_sliding_friction(
        &persisted,
        &[],
        |_| Some(SurfaceType::Default),
        &ft,
        &mut q,
        &mut active,
        &mut voices,
    );
    assert!(q
        .drain_all()
        .iter()
        .any(|c| matches!(c, AudioCommand::Play { .. })));
}

#[test]
fn tc_ir_1_8_5_2_slide_stop_ends() {
    let clip = AssetHandle::<AudioClip>::new(3);
    let ft = FrictionSoundTable {
        entries: [[None; crate::SURFACE_TYPE_COUNT]; crate::SURFACE_TYPE_COUNT],
        default: clip,
    };
    let mut q = BoundedAudioCommandQueue::new(1024);
    let mut active = ActiveFrictionSounds::new();
    let mut voices = VoiceIdAllocator::default();
    let persisted = [CollisionPersisted {
        entity_a: Entity(1),
        entity_b: Entity(2),
        contacts: vec![ContactPoint {
            world_point: Vec3::ZERO,
        }],
        normal: Vec3::new(0.0, 1.0, 0.0),
        total_impulse: 1.0,
        tangential_velocity: Vec3::new(2.0, 0.0, 0.0),
    }];
    handle_sliding_friction(
        &persisted,
        &[],
        |_| Some(SurfaceType::Default),
        &ft,
        &mut q,
        &mut active,
        &mut voices,
    );
    q.drain_all();
    let ended = [CollisionEnded {
        entity_a: Entity(1),
        entity_b: Entity(2),
    }];
    handle_sliding_friction(
        &[],
        &ended,
        |_| Some(SurfaceType::Default),
        &ft,
        &mut q,
        &mut active,
        &mut voices,
    );
    assert!(q
        .drain_all()
        .iter()
        .any(|c| matches!(c, AudioCommand::Stop { .. })));
}

#[test]
fn tc_ir_1_8_5_3_slide_speed_gain() {
    let clip = AssetHandle::<AudioClip>::new(3);
    let ft = FrictionSoundTable {
        entries: [[None; crate::SURFACE_TYPE_COUNT]; crate::SURFACE_TYPE_COUNT],
        default: clip,
    };
    let mut q = BoundedAudioCommandQueue::new(1024);
    let mut active = ActiveFrictionSounds::new();
    let mut voices = VoiceIdAllocator::default();
    let slow = [CollisionPersisted {
        entity_a: Entity(1),
        entity_b: Entity(2),
        contacts: vec![ContactPoint {
            world_point: Vec3::ZERO,
        }],
        normal: Vec3::new(0.0, 1.0, 0.0),
        total_impulse: 1.0,
        tangential_velocity: Vec3::new(0.5, 0.0, 0.0),
    }];
    handle_sliding_friction(
        &slow,
        &[],
        |_| Some(SurfaceType::Default),
        &ft,
        &mut q,
        &mut active,
        &mut voices,
    );
    q.drain_all();
    let fast = [CollisionPersisted {
        entity_a: Entity(1),
        entity_b: Entity(2),
        contacts: vec![ContactPoint {
            world_point: Vec3::ZERO,
        }],
        normal: Vec3::new(0.0, 1.0, 0.0),
        total_impulse: 1.0,
        tangential_velocity: Vec3::new(50.0, 0.0, 0.0),
    }];
    handle_sliding_friction(
        &fast,
        &[],
        |_| Some(SurfaceType::Default),
        &ft,
        &mut q,
        &mut active,
        &mut voices,
    );
    let gain = q.drain_all().into_iter().find_map(|c| match c {
        AudioCommand::SetParam { value, .. } => Some(value),
        _ => None,
    });
    assert!(gain.unwrap() > 0.2);
}

#[test]
fn tc_ir_1_8_5_4_slide_below_tangential_stops() {
    let clip = AssetHandle::<AudioClip>::new(3);
    let ft = FrictionSoundTable {
        entries: [[None; crate::SURFACE_TYPE_COUNT]; crate::SURFACE_TYPE_COUNT],
        default: clip,
    };
    let mut q = BoundedAudioCommandQueue::new(1024);
    let mut active = ActiveFrictionSounds::new();
    let mut voices = VoiceIdAllocator::default();
    let persisted = [CollisionPersisted {
        entity_a: Entity(1),
        entity_b: Entity(2),
        contacts: vec![ContactPoint {
            world_point: Vec3::ZERO,
        }],
        normal: Vec3::new(0.0, 1.0, 0.0),
        total_impulse: 1.0,
        tangential_velocity: Vec3::new(2.0, 0.0, 0.0),
    }];
    handle_sliding_friction(
        &persisted,
        &[],
        |_| Some(SurfaceType::Default),
        &ft,
        &mut q,
        &mut active,
        &mut voices,
    );
    q.drain_all();
    let slow = [CollisionPersisted {
        entity_a: Entity(1),
        entity_b: Entity(2),
        contacts: vec![ContactPoint {
            world_point: Vec3::ZERO,
        }],
        normal: Vec3::new(0.0, 1.0, 0.0),
        total_impulse: 1.0,
        tangential_velocity: Vec3::new(0.001, 0.0, 0.0),
    }];
    handle_sliding_friction(
        &slow,
        &[],
        |_| Some(SurfaceType::Default),
        &ft,
        &mut q,
        &mut active,
        &mut voices,
    );
    assert!(q
        .drain_all()
        .iter()
        .any(|c| matches!(c, AudioCommand::Stop { .. })));
}

#[test]
fn tc_ir_1_8_n1_missing_material_defaults() {
    let def_clip = AssetHandle::<AudioClip>::new(200);
    let table = empty_impact_table_with_default(def_clip);
    let mut q = BoundedAudioCommandQueue::new(1024);
    let mut cd = ImpactCooldownTracker::new();
    let mut voices = VoiceIdAllocator::default();
    let event = CollisionStarted {
        entity_a: Entity(1),
        entity_b: Entity(2),
        contacts: vec![ContactPoint {
            world_point: Vec3::ZERO,
        }],
        normal: Vec3::ZERO,
        total_impulse: 50.0,
    };
    handle_collision_impact(&event, |_| None, &table, &mut q, &mut cd, &mut voices, 0);
    assert!(q
        .drain_all()
        .iter()
        .any(|c| matches!(c, AudioCommand::Play { .. })));
}

#[test]
fn tc_ir_1_8_n2_empty_pair_uses_default_clip() {
    let def_clip = AssetHandle::<AudioClip>::new(200);
    let table = empty_impact_table_with_default(def_clip);
    let mut q = BoundedAudioCommandQueue::new(1024);
    let mut cd = ImpactCooldownTracker::new();
    let mut voices = VoiceIdAllocator::default();
    let event = CollisionStarted {
        entity_a: Entity(1),
        entity_b: Entity(2),
        contacts: vec![ContactPoint {
            world_point: Vec3::ZERO,
        }],
        normal: Vec3::ZERO,
        total_impulse: 50.0,
    };
    handle_collision_impact(
        &event,
        |_| Some(SurfaceType::Metal),
        &table,
        &mut q,
        &mut cd,
        &mut voices,
        0,
    );
    let clip = q.drain_all().into_iter().find_map(|c| match c {
        AudioCommand::Play { clip, .. } => Some(clip),
        _ => None,
    });
    assert_eq!(clip, Some(def_clip));
}

#[test]
fn tc_ir_1_8_n3_queue_overflow_keeps_high_priority() {
    let q = BoundedAudioCommandQueue::new(1024);
    for i in 0..1024 {
        let _ = q.send(AudioCommand::Play {
            voice_id: crate::ids::VoiceId(i),
            clip: AssetHandle::new(i),
            bus: BusId::Sfx,
            priority: VoicePriority::Low,
            position: None,
            timestamp: crate::commands::AudioTimestamp::Immediate,
            gain: 1.0,
            pitch: 1.0,
        });
    }
    for i in 0..500 {
        let _ = q.send(AudioCommand::Play {
            voice_id: crate::ids::VoiceId(10_000 + i),
            clip: AssetHandle::new(10_000 + i),
            bus: BusId::Sfx,
            priority: VoicePriority::High,
            position: None,
            timestamp: crate::commands::AudioTimestamp::Immediate,
            gain: 1.0,
            pitch: 1.0,
        });
    }
    let high_count = q
        .drain_all()
        .iter()
        .filter(|c| {
            matches!(
                c,
                AudioCommand::Play {
                    priority: VoicePriority::High,
                    ..
                }
            )
        })
        .count();
    assert_eq!(high_count, 500);
}

#[test]
fn tc_ir_1_8_n4_burst_underrun_counter() {
    let q = BoundedAudioCommandQueue::new(1024);
    for i in 0..1000 {
        let _ = q.send(AudioCommand::Play {
            voice_id: crate::ids::VoiceId(i),
            clip: AssetHandle::new(i),
            bus: BusId::Sfx,
            priority: VoicePriority::Medium,
            position: None,
            timestamp: crate::commands::AudioTimestamp::Immediate,
            gain: 1.0,
            pitch: 1.0,
        });
    }
    assert_eq!(q.len(), 1000);
    let _ = q.drain_processing(512);
    assert!(q.underrun_drops() >= 488);
}

#[test]
fn tc_ir_1_8_n5_zero_impulse_no_play() {
    let clip = AssetHandle::<AudioClip>::new(1);
    let table = table_with_pair(SurfaceType::Default, SurfaceType::Default, clip, 0.0, 0.0);
    let mut q = BoundedAudioCommandQueue::new(1024);
    let mut cd = ImpactCooldownTracker::new();
    let mut voices = VoiceIdAllocator::default();
    let event = CollisionStarted {
        entity_a: Entity(1),
        entity_b: Entity(2),
        contacts: vec![ContactPoint {
            world_point: Vec3::ZERO,
        }],
        normal: Vec3::ZERO,
        total_impulse: 0.0,
    };
    handle_collision_impact(
        &event,
        |_| Some(SurfaceType::Default),
        &table,
        &mut q,
        &mut cd,
        &mut voices,
        0,
    );
    assert!(q.drain_all().is_empty());
}

#[test]
fn tc_ir_1_8_n6_self_collision_no_play() {
    let clip = AssetHandle::<AudioClip>::new(1);
    let table = table_with_pair(SurfaceType::Default, SurfaceType::Default, clip, 0.0, 0.0);
    let mut q = BoundedAudioCommandQueue::new(1024);
    let mut cd = ImpactCooldownTracker::new();
    let mut voices = VoiceIdAllocator::default();
    let event = CollisionStarted {
        entity_a: Entity(1),
        entity_b: Entity(1),
        contacts: vec![ContactPoint {
            world_point: Vec3::ZERO,
        }],
        normal: Vec3::ZERO,
        total_impulse: 50.0,
    };
    handle_collision_impact(
        &event,
        |_| Some(SurfaceType::Default),
        &table,
        &mut q,
        &mut cd,
        &mut voices,
        0,
    );
    assert!(q.drain_all().is_empty());
}

#[test]
fn tc_ir_1_8_n7_cooldown_capacity_fifo() {
    let mut cd = ImpactCooldownTracker::new();
    for i in 0..300 {
        let a = Entity(i * 2);
        let b = Entity(i * 2 + 1);
        cd.start(a, b, 1.0);
    }
    assert_eq!(cd.len(), 256);
    assert!(
        !cd.is_cooling(Entity(0), Entity(1)),
        "oldest pair should FIFO-evict when capacity is reached"
    );
    assert!(
        cd.is_cooling(Entity(88), Entity(89)),
        "pair from first surviving window should remain tracked"
    );
}

#[test]
fn tc_ir_1_8_n8_trigger_no_zone_no_command() {
    let mut q = BoundedAudioCommandQueue::new(1024);
    let mut voices = VoiceIdAllocator::default();
    let mut ambient_rt = AmbientVoiceMap::new();
    let enter = [TriggerEnter {
        trigger_entity: Entity(10),
        other_entity: Entity(20),
    }];
    handle_trigger_zone(
        &enter,
        &[],
        |_e| TriggerZoneSnapshot::default(),
        &mut ambient_rt,
        &mut q,
        &mut voices,
    );
    assert!(q.drain_all().is_empty());
}

#[test]
fn tc_ir_1_8_n9_unknown_voice_param_ignored() {
    assert!(!apply_set_param_wire(99, 1.0));
}
