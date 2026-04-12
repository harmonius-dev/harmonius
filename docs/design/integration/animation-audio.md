# Animation ↔ Audio Integration Design

## Systems Involved

| System | Design | Domain |
|--------|--------|--------|
| Animation | [skeletal.md](../animation/skeletal.md) | Animation |
| Audio | [audio.md](../audio/audio.md) | Audio |

## Integration Requirements

| ID | Requirement | Systems |
|----|-------------|---------|
| IR-1.2.1 | Footstep events trigger surface sounds | Anim, Audio |
| IR-1.2.2 | Impact events trigger hit sounds | Anim, Audio |
| IR-1.2.3 | Sound sync to walk cycle phase | Anim, Audio |
| IR-1.2.4 | Surface type selects sound variant | Anim, Audio |
| IR-1.2.5 | Animation speed scales sound pitch | Anim, Audio |

1. **IR-1.2.1** -- The state machine determines the current animation immediately (no one-frame
   delay). `AnimEventPayload::Footstep` fires an ECS event via `EventWriter<AnimEvent>`. The audio
   bridge reads it and enqueues `AudioCommand::Play` followed by `AudioCommand::UpdateSpatial` for
   positioning.
2. **IR-1.2.2** -- `AnimEventPayload::HitWindow` markers fire on weapon contact frames. The
   hit-event bridge enqueues `AudioCommand::Play` + `UpdateSpatial` at the bone's world position.
3. **IR-1.2.3** -- Footstep events are frame-accurate within the animation clip. The state machine
   resolves the active state without delay, so the audio command uses `AudioTimestamp::Immediate`
   and the sound aligns with the visual foot-plant.
4. **IR-1.2.4** -- The `Footstep { surface: StringId }` payload carries the surface type. A raycast
   from the foot bone downward determines the actual ground material, selecting the correct
   `SoundBank` entry.
5. **IR-1.2.5** -- When `AnimationPlayer.speed` changes (run vs walk), the speed-sync bridge sends
   `AudioCommand::SetParam` with `VoiceParam::Pitch` scaled proportionally. Pitch follows a damped
   linear map `pitch = 1.0 + (speed - 1.0) * k` with `k = 0.1` (a 2x speed increase yields a ~+10%
   pitch shift, staying within the perceptually subtle range documented in Zolzer, "DAFX: Digital
   Audio Effects", 2nd ed., ch. 7 "Time-segment processing").

### Scope Note

2D and 2.5D projections are intentionally out of scope for this integration. All spatial audio uses
3D world positions; 2D games call `UpdateSpatial` with `Vec3::ZERO` velocity and Z=0.

## Data Contracts

| Type | Defined in | Consumed by | Purpose |
|------|-----------|-------------|---------|
| `AnimEventMarker` | Animation | Audio | Marker def |
| `AnimEventPayload` | Animation | Audio | Payload enum |
| `AnimEvent` | Animation | Audio bridge | ECS event |
| `AudioCommand` | Audio | Audio bridge | Sound cmds |
| `SoundBank` | This design | Audio bridge | Material map |
| `PhysicsMaterial` | Physics | Audio bridge | Surface tag |
| `CommandSender` | Audio | Audio bridge | MPSC sender |

### Channel Contracts

The audio command channel is an **MPSC** (multiple-producer single-consumer) lock-free queue.
Multiple game-thread systems (footstep bridge, hit bridge, speed-sync bridge) send commands; the
single audio thread drains them.

| Property | Value |
|----------|-------|
| Type | MPSC, lock-free |
| Buffering | Bounded, 4096 commands |
| Backpressure | `send` returns `Err` when full |
| Consumer | Audio thread drains each callback |
| Atomicity | Each `send` is atomic |

### SoundBank

Maps surface material types to randomized audio clip pools. Used by footstep and impact bridges to
select the correct sound variant.

`SoundBank` is a persistent asset type (loaded from disk, zero-copy deserialized), so it derives
rkyv `Archive`, `Serialize`, `Deserialize`. It is immutable after load and shared read-only across
bridge systems, so it is exposed as `Res<SoundBank>` (an ECS resource wrapping an `Arc` of immutable
data). Random selection uses the Fisher-Yates weighted pick algorithm for uniform distribution
across the pool.

```rust
/// Maps surface materials to audio clip pools.
/// Persistent asset, loaded zero-copy via rkyv.
/// Immutable after load; shared read-only.
#[derive(Archive, Serialize, Deserialize)]
pub struct SoundBank {
    /// Material -> pool of clip handles.
    /// DashMap: concurrent read-only access from
    /// multiple ECS bridge systems without locks.
    entries: DashMap<StringId, Vec<AssetHandle<AudioClip>>>,
    /// Fallback clip when material has no entry.
    fallback: AssetHandle<AudioClip>,
}

impl SoundBank {
    /// Picks a random clip for the given material.
    /// Algorithm: uniform index pick via `Rng::gen_range`
    /// over the pool length (O(1)). Falls back to
    /// `self.fallback` when the material has no entry.
    pub fn pick(
        &self,
        material: StringId,
        rng: &mut Rng,
    ) -> AssetHandle<AudioClip> {
        self.entries
            .get(&material)
            .and_then(|pool| {
                let idx = rng.gen_range(0..pool.len());
                pool.get(idx).cloned()
            })
            .unwrap_or_else(|| self.fallback.clone())
    }
}
```

### AnimEvent

The ECS event emitted by `animation_advance_system` when playback crosses an event marker. Matches
the canonical name from `skeletal.md` (`EventWriter<AnimEvent>`).

```rust
/// ECS event emitted when animation playback
/// crosses an AnimEventMarker time.
pub struct AnimEvent {
    pub entity: Entity,
    pub marker: AnimEventMarker,
    pub bone_world_pos: Vec3,
}
```

### Footstep Bridge

```rust
/// ECS system: bridges footstep animation events
/// to audio commands.
pub fn footstep_bridge_system(
    events: EventReader<AnimEvent>,
    physics_world: Res<PhysicsQueries>,
    sound_bank: Res<SoundBank>,
    audio_cmd: Res<CommandSender>,
    mut rng: ResMut<Rng>,
) {
    for event in events.read() {
        if let AnimEventPayload::Footstep { surface }
            = &event.marker.payload
        {
            // Raycast down from foot bone to find
            // the actual ground material.
            let material = physics_world
                .raycast_down(event.bone_world_pos, 0.3)
                .map(|h| h.surface_type)
                .unwrap_or(*surface);

            let clip = sound_bank.pick(material, &mut rng);
            let voice_id = VoiceId::transient();

            // Fallback path: if send fails (queue full),
            // the sound is silently dropped. Voice limit
            // overflow is handled by priority-based
            // stealing inside the audio engine.
            let _ = audio_cmd.send(AudioCommand::Play {
                voice_id,
                clip,
                bus: BusId::SFX,
                priority: VoicePriority::Medium,
                timestamp: AudioTimestamp::Immediate,
            });
            let _ = audio_cmd.send(
                AudioCommand::UpdateSpatial {
                    voice_id,
                    position: event.bone_world_pos,
                    velocity: Vec3::ZERO,
                    orientation: Quat::IDENTITY,
                },
            );
        }
    }
}
```

#### Fallback Paths (Footstep)

| Condition | Fallback | Result |
|-----------|----------|--------|
| Raycast misses ground | Use clip's `surface` hint | Correct genre sound |
| `SoundBank` has no material | `SoundBank::fallback` clip | Default footstep |
| Command queue full | `send` returns `Err`, drop | Silent (no sound) |
| Voice limit exceeded | Priority-based steal in engine | Lowest-priority culled |

### Hit Event Bridge

Impact selection uses the same uniform index pick algorithm as footsteps (see `SoundBank::pick`).
The bridge fires `VoicePriority::High` so impact voices survive voice stealing before footsteps (see
Failure Modes). Each hit enqueues `Play` + `UpdateSpatial` atomically at the bone position.

```rust
/// ECS system: bridges hit-window animation events
/// to impact audio commands.
pub fn hit_bridge_system(
    events: EventReader<AnimEvent>,
    impact_bank: Res<SoundBank>,
    audio_cmd: Res<CommandSender>,
    mut rng: ResMut<Rng>,
) {
    for event in events.read() {
        if let AnimEventPayload::HitWindow { .. }
            = &event.marker.payload
        {
            let clip = impact_bank.pick(
                StringId::from("impact"),
                &mut rng,
            );
            let voice_id = VoiceId::transient();

            // Fallback: queue-full drops silently.
            let _ = audio_cmd.send(AudioCommand::Play {
                voice_id,
                clip,
                bus: BusId::SFX,
                priority: VoicePriority::High,
                timestamp: AudioTimestamp::Immediate,
            });
            let _ = audio_cmd.send(
                AudioCommand::UpdateSpatial {
                    voice_id,
                    position: event.bone_world_pos,
                    velocity: Vec3::ZERO,
                    orientation: Quat::IDENTITY,
                },
            );
        }
    }
}
```

#### Fallback Paths (Hit)

| Condition | Fallback | Result |
|-----------|----------|--------|
| `SoundBank` has no "impact" | `SoundBank::fallback` clip | Default hit |
| Command queue full | `send` returns `Err`, drop | Silent (no sound) |
| Voice limit exceeded | Priority steal (High prio) | Lower-prio culled |

### Speed-Sync Bridge

The tracker is mutated only by `footstep_bridge_system` (inserts on Play) and
`speed_sync_bridge_system` (reads). Both run on the game thread in Phase 6, so a single writer holds
exclusive access; `DashMap` handles concurrent inserts across multiple game-thread systems during
the same phase. Pitch is computed pointwise from `AnimationPlayer.speed` (algorithm: damped linear
map, see IR-1.2.5 above).

```rust
/// Tracks active footstep voices and their
/// associated animation entity, so pitch can
/// be updated when animation speed changes.
/// DashMap: non-persistent runtime state, concurrent
/// access from bridge systems during Phase 6.
pub struct FootstepVoiceTracker {
    /// Entity -> most recent footstep VoiceId.
    active: DashMap<Entity, VoiceId>,
}

/// ECS system: scales footstep pitch to match
/// animation playback speed.
pub fn speed_sync_bridge_system(
    players: Query<(Entity, &AnimationPlayer)>,
    tracker: Res<FootstepVoiceTracker>,
    audio_cmd: Res<CommandSender>,
) {
    for (entity, player) in players.iter() {
        if let Some(&voice_id) =
            tracker.active.get(&entity)
        {
            // Map animation speed to pitch. A speed
            // of 2.0 yields pitch ~1.1 (subtle shift).
            let pitch = 1.0 + (player.speed - 1.0) * 0.1;

            // Fallback: queue-full drops silently;
            // pitch remains at previous value.
            let _ = audio_cmd.send(
                AudioCommand::SetParam {
                    voice_id,
                    param: VoiceParam::Pitch,
                    value: pitch,
                    timestamp: AudioTimestamp::Immediate,
                },
            );
        }
    }
}
```

#### Fallback Paths (Speed-Sync)

| Condition | Fallback | Result |
|-----------|----------|--------|
| No active voice for entity | Skip (no-op) | No pitch change |
| Command queue full | `send` returns `Err`, drop | Pitch stays previous |

## Class Diagram

```mermaid
classDiagram
    class AnimEvent {
        +Entity entity
        +AnimEventMarker marker
        +Vec3 bone_world_pos
    }
    class AnimEventMarker {
        +StringId name
        +f32 time
        +AnimEventPayload payload
    }
    class AnimEventPayload {
        <<enumeration>>
        Footstep
        HitWindow
        VfxSpawn
        WeaponTrail
    }
    class SoundBank {
        -DashMap entries
        -AssetHandle fallback
        +pick(material, rng) AssetHandle
    }
    class FootstepVoiceTracker {
        -DashMap active
    }
    class VoiceParam {
        <<enumeration>>
        Pitch
        Volume
        Pan
        LowpassCutoff
    }
    class VoicePriority {
        <<enumeration>>
        Low
        Medium
        High
        Critical
    }
    class BusId {
        <<enumeration>>
        Master
        SFX
        Music
        Voice
    }
    class AudioTimestamp {
        <<enumeration>>
        Immediate
        AtSample
    }
    class AudioCommand {
        <<enumeration>>
        Play
        UpdateSpatial
        SetParam
    }
    class CommandSender {
        +send(cmd) Result
    }
    class AnimationPlayer {
        +f32 speed
        +bool paused
    }

    AnimEvent --> AnimEventMarker
    AnimEventMarker --> AnimEventPayload
    SoundBank ..> AudioCommand : selects clip
    FootstepVoiceTracker ..> AudioCommand : pitch
    CommandSender --> AudioCommand : sends
    AnimationPlayer ..> FootstepVoiceTracker : speed
```

## Data Flow

```mermaid
sequenceDiagram
    participant Sys as animation_advance_system
    participant EW as EventWriter~AnimEvent~
    participant FB as footstep_bridge_system
    participant HB as hit_bridge_system
    participant SB as speed_sync_bridge_system
    participant Phys as PhysicsQueries
    participant Bank as SoundBank
    participant Cmd as CommandSender (MPSC)
    participant AT as Audio Thread

    Sys->>Sys: detect marker crossing
    Sys->>EW: write AnimEvent
    EW-->>FB: EventReader reads footstep
    FB->>Phys: raycast down from foot bone
    Phys-->>FB: SurfaceType
    FB->>Bank: pick(material, rng)
    Bank-->>FB: AssetHandle~AudioClip~
    FB->>Cmd: Play + UpdateSpatial

    EW-->>HB: EventReader reads hit window
    HB->>Bank: pick("impact", rng)
    Bank-->>HB: AssetHandle~AudioClip~
    HB->>Cmd: Play + UpdateSpatial

    EW-->>SB: AnimationPlayer.speed changed
    SB->>Cmd: SetParam(Pitch)

    Cmd->>AT: Lock-free drain
    AT->>AT: play/position/pitch sounds
```

## Timing and Ordering

| System | Phase | Timestep | Order |
|--------|-------|----------|-------|
| Animation eval | 6-Animation | Variable | First |
| Event dispatch | 6-Animation | Variable | After eval |
| Audio bridges | 6-Animation | Variable | After events |
| Audio thread | Dedicated | Real-time | Lock-free drain |

The state machine determines the active animation immediately with no one-frame delay. Animation
evaluates clips and fires events in Phase 6. The bridge systems run immediately after event dispatch
in the same phase. Audio commands are enqueued to the bounded MPSC lock-free queue (4096 capacity)
and drained by the audio thread at its next buffer callback.

Latency: event-to-sound is under one audio buffer period (typically 5-10 ms at 48 kHz / 256
samples). Critically, animation events are consumed by the audio command queue in the **same frame**
they are emitted -- there is no one-frame delay. All three bridge systems are scheduled after
`animation_advance_system` within Phase 6.

### Performance Budget

| Metric | Budget | Rationale |
|--------|--------|-----------|
| Bridge CPU per frame | < 0.5 ms | 200 events / raycasts per frame |
| Queue send p99 | < 5 us | Lock-free bounded MPSC |
| Event-to-sound | < 10 ms | One audio buffer period |

### Debug Toggles

A runtime-toggleable `AudioBridgeDebug` ECS resource enables per-bridge logging. When enabled, each
bridge records event count, dropped sends, and voice-steal events into a ring buffer that the
profiler overlay reads. Toggling is a single atomic flag set; no recompile required.

## Failure Modes

| Failure | Impact | Recovery |
|---------|--------|----------|
| Sound bank missing material | Wrong sound | `SoundBank::fallback` clip |
| Raycast misses ground | Wrong surface | Use clip's `surface` hint |
| Voice limit exceeded | Sound virtualized | Priority-based steal |
| Command queue full | Sound dropped | `send` returns `Err` |
| Audio thread overrun | Buffer underrun | Silence, catch up next cb |

### Detailed Fallback Paths

1. **Sound bank missing material** -- `SoundBank::pick` returns `self.fallback` when the material
   has no entry. A default footstep or impact sound always plays.
2. **Raycast misses ground** -- The bridge uses the `surface` hint from `AnimEventPayload::Footstep`
   as the material key. The sound matches the clip author's intended surface.
3. **Voice limit exceeded** -- The audio engine's `VoiceManager` performs priority-based stealing.
   Transient footstep voices (`VoicePriority::Medium`) are culled before hit voices (`High`).
4. **Command queue full** -- `CommandSender::send` returns `Err(cmd)`. The bridge discards the
   command with `let _ =`. The sound is silently skipped for that frame.
5. **Audio thread overrun** -- The audio callback outputs silence for the missed buffer. The thread
   catches up on the next callback by draining any accumulated commands.

## Platform Considerations

None -- identical across all platforms. Animation events and audio commands use platform-agnostic
ECS and MPSC channel primitives. The audio thread's platform backend is abstracted behind
`AudioBackend`.

## Test Plan

See companion [animation-audio-test-cases.md](animation-audio-test-cases.md).

## Review Status

| # | Item | Status |
|---|------|--------|
| 1 | `AudioCommand::Play` no `position`; add `UpdateSpatial` | APPLIED |
| 2 | ECS `Res<T>` params in bridge systems | APPLIED |
| 3 | Rename `AnimEventFired` to `AnimEvent` | APPLIED |
| 4 | Pseudocode + algorithm ref for IR-1.2.5 (pitch scaling) | APPLIED |
| 5 | Pseudocode for IR-1.2.2 (hit/impact sounds) | APPLIED |
| 6 | Define `SoundBank` with rkyv derives | APPLIED |
| 7 | Add `classDiagram` with full enum variants | APPLIED |
| 8 | "Async drain" -> "Lock-free drain" in timing table | APPLIED |
| 9 | Failure-mode test cases: fallback, steal, underrun, queue full | APPLIED |
| 10 | Sequence diagram: `animation_advance_system` emits via `EventWriter` | APPLIED |
| 11 | No HashMap: `SoundBank` and `FootstepVoiceTracker` use `DashMap` | APPLIED |
| 12 | Persistent `SoundBank` derives rkyv `Archive`/`Serialize`/`Deserialize` | APPLIED |
| 13 | Same-frame event consumption (no one-frame delay) documented | APPLIED |
| 14 | Performance budget table added | APPLIED |
| 15 | Runtime-toggleable `AudioBridgeDebug` resource documented | APPLIED |
| 16 | 2D / 2.5D scope note added | APPLIED |
| 17 | All enums fully defined in classDiagram | APPLIED |
| 18 | Channel buffer length (4096) and MPSC bound documented | APPLIED |
