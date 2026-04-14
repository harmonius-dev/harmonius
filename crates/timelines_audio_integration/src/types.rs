//! Shared value types at the timelines ↔ audio boundary.

use std::marker::PhantomData;
use std::sync::Arc;

/// Stable asset identifier for typed handles.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct AssetHandle<T> {
    /// Opaque asset table index.
    pub id: u32,
    _marker: PhantomData<T>,
}

impl<T> AssetHandle<T> {
    /// Builds a handle; `id == 0` conventionally means “invalid / missing”.
    pub const fn new(id: u32) -> Self {
        Self {
            id,
            _marker: PhantomData,
        }
    }
}

/// Marker type for `AssetHandle<AudioClip>`.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct AudioClip;

/// Music graph segment identifier.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct SegmentId(pub u32);

/// Mixer bus identifier (numeric id matches authored graphs).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct BusId(pub u16);

impl BusId {
    /// Canonical voice bus from the audio design.
    pub const VOICE: BusId = BusId(2);
    /// Canonical SFX bus from the audio design.
    pub const SFX: BusId = BusId(1);
}

/// Mixer bus automation parameter.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum BusParam {
    /// Linear gain multiplier.
    Gain,
    /// Bus mute flag as 0.0 / 1.0 automation sample.
    Mute,
    /// Solo flag as 0.0 / 1.0 automation sample.
    Solo,
}

/// Effect parameter identifier (opaque slot id from authored graphs).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct ParamId(pub u32);

/// Mixer voice handle allocated by `VoiceManager`.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct VoiceId(pub u32);

/// Voice priority used for allocation and stealing.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum VoicePriority {
    /// Ambient beds; first to virtualize.
    Ambient,
    /// Default gameplay voices.
    Normal,
    /// Important lines.
    High,
    /// Never stolen except by `Critical`.
    Critical,
}

/// Stinger ducking request carried on `AudioTrackTarget::Stinger`.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct StingerRequest {
    /// Clip to play.
    pub clip: AssetHandle<AudioClip>,
    /// Duck amount in decibels.
    pub duck_db: f32,
    /// Duck duration in milliseconds.
    pub duck_ms: u32,
}

/// Localized string table id for subtitle speaker names.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct StringId(pub u32);

/// HRTF / listener slot index.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct ListenerId(pub u8);

/// Voice chat channel id.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct ChannelId(pub u16);

/// Local player slot id.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct PlayerId(pub u8);

/// Dialogue line priority for `AudioCommand::DialoguePlay`.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum DialoguePriority {
    /// Standard cinematic line.
    Normal,
    /// Overrides `Normal` when voices are contended.
    High,
}

/// Sample-accurate scheduling hint for one-shot voices.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum AudioTimestamp {
    /// Play as soon as the audio thread drains the command.
    Immediate,
    /// Play at the given output sample offset.
    SampleOffset(u64),
}

/// Per-voice parameter automation target.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum VoiceParam {
    /// Voice linear gain.
    Gain,
    /// Playback pitch ratio.
    Pitch,
    /// Doppler pitch multiplier.
    DopplerFactor,
    /// Occlusion attenuation in linear gain.
    OcclusionGain,
    /// Occlusion low-pass in Hz-ish normalized units.
    OcclusionLpf,
}

/// Canonical audio command enum (subset names aligned with `docs/design/audio/audio.md`).
#[derive(Clone, Debug, PartialEq)]
pub enum AudioCommand {
    /// Starts a one-shot or looping voice on a bus.
    Play {
        /// Allocated voice slot.
        voice_id: VoiceId,
        /// Clip to decode.
        clip: AssetHandle<AudioClip>,
        /// Output bus.
        bus: BusId,
        /// Priority used for stealing decisions.
        priority: VoicePriority,
        /// Scheduling hint.
        timestamp: AudioTimestamp,
    },
    /// Stops a voice with optional fade length in samples.
    Stop {
        /// Voice to stop.
        voice_id: VoiceId,
        /// Fade length in output samples.
        fade_samples: u32,
        /// Scheduling hint.
        timestamp: AudioTimestamp,
    },
    /// Pauses a voice.
    Pause {
        /// Voice to pause.
        voice_id: VoiceId,
        /// Scheduling hint.
        timestamp: AudioTimestamp,
    },
    /// Resumes a paused voice.
    Resume {
        /// Voice to resume.
        voice_id: VoiceId,
        /// Scheduling hint.
        timestamp: AudioTimestamp,
    },
    /// Sets a per-voice parameter value.
    SetParam {
        /// Voice to modify.
        voice_id: VoiceId,
        /// Parameter slot.
        param: VoiceParam,
        /// Target value.
        value: f32,
        /// Scheduling hint.
        timestamp: AudioTimestamp,
    },
    /// Sets a bus-level automation value.
    SetBusParam {
        /// Target bus.
        bus_id: BusId,
        /// Bus parameter.
        param: BusParam,
        /// Target value.
        value: f32,
    },
    /// Updates spatialization inputs for a voice.
    UpdateSpatial {
        /// Voice to spatialize.
        voice_id: VoiceId,
        /// World-space position in meters.
        position: [f32; 3],
        /// World-space velocity in meters per second.
        velocity: [f32; 3],
        /// World-space orientation.
        orientation: [f32; 4],
    },
    /// Updates listener transform for a local player.
    UpdateListener {
        /// Listener slot.
        listener_id: ListenerId,
        /// World-space position in meters.
        position: [f32; 3],
        /// World-space velocity in meters per second.
        velocity: [f32; 3],
        /// World-space orientation.
        orientation: [f32; 4],
    },
    /// Prefetch decode buffers for a clip.
    Prefetch {
        /// Clip to warm.
        clip: AssetHandle<AudioClip>,
    },
    /// Sets an insert effect parameter on a bus.
    SetEffectParam {
        /// Bus hosting the insert.
        bus: BusId,
        /// Insert index in the bus graph.
        node_index: u32,
        /// Effect parameter id.
        param: ParamId,
        /// Target value.
        value: f32,
    },
    /// Inserts an effect node (stringly-typed authoring hook).
    InsertEffect {
        /// Target bus.
        bus: BusId,
        /// Insert index.
        index: u32,
        /// Serialized effect type name.
        node_type: String,
    },
    /// Removes an insert effect node.
    RemoveEffect {
        /// Target bus.
        bus: BusId,
        /// Insert index.
        index: u32,
    },
    /// Starts adaptive music on a segment.
    MusicPlay {
        /// Entry cue segment.
        cue: SegmentId,
    },
    /// Requests a transition to a new music segment.
    MusicTransition {
        /// Target segment id.
        target: SegmentId,
    },
    /// Sets adaptive music intensity scalar.
    MusicSetIntensity {
        /// Intensity in 0..1.
        value: f32,
    },
    /// Stops adaptive music with a fade time in milliseconds.
    MusicStop {
        /// Fade time in milliseconds.
        fade_ms: u32,
    },
    /// Triggers a music stinger with ducking metadata.
    TriggerStinger {
        /// Stinger payload.
        request: StingerRequest,
    },
    /// Joins a networked voice channel.
    VoiceChannelJoin {
        /// Channel id.
        channel: ChannelId,
        /// Local player id.
        player: PlayerId,
    },
    /// Leaves a networked voice channel.
    VoiceChannelLeave {
        /// Channel id.
        channel: ChannelId,
        /// Local player id.
        player: PlayerId,
    },
    /// High-level dialogue play hook (graph-driven VO).
    DialoguePlay {
        /// Speaking entity id (opaque).
        entity: u64,
        /// Line asset.
        line: AssetHandle<AudioClip>,
        /// Priority hint.
        priority: DialoguePriority,
    },
}

/// Dialogue subtitle event emitted alongside VO commands.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum SubtitleEvent {
    /// Shows a subtitle line for a duration or until hidden.
    Show {
        /// Stable line id for pairing hide events.
        line_id: DialogueLineId,
        /// Subtitle text (immutable shared payload).
        text: Arc<str>,
        /// Optional localized speaker name.
        speaker: Option<StringId>,
        /// Minimum display time in milliseconds.
        duration_ms: u32,
    },
    /// Hides a previously shown subtitle line.
    Hide {
        /// Line id to clear.
        line_id: DialogueLineId,
    },
}

/// Stable id for pairing subtitle show/hide events.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct DialogueLineId(pub u32);

/// Resolved audio routing for a timeline track (baked asset data).
#[derive(Clone, Debug, PartialEq)]
pub enum AudioTrackTarget {
    /// Music segment cue.
    MusicCue {
        /// Target music segment.
        segment: SegmentId,
    },
    /// Voice-over with subtitle metadata.
    VoiceOver {
        /// Clip to play.
        clip: AssetHandle<AudioClip>,
        /// Allocation priority.
        priority: VoicePriority,
        /// Subtitle line id.
        line_id: DialogueLineId,
        /// Subtitle body (also used for duration checks in tests).
        text: Arc<str>,
        /// Optional speaker string id.
        speaker: Option<StringId>,
        /// VO duration in milliseconds.
        duration_ms: u32,
    },
    /// One-shot SFX clip on a bus.
    OneShot {
        /// Clip to play.
        clip: AssetHandle<AudioClip>,
        /// Output bus.
        bus: BusId,
        /// Allocation priority.
        priority: VoicePriority,
    },
    /// Mixer bus automation binding.
    BusParam {
        /// Target bus.
        bus: BusId,
        /// Bus parameter.
        param: BusParam,
    },
    /// Insert effect automation binding.
    BusEffectParam {
        /// Target bus.
        bus: BusId,
        /// Insert node index.
        node_index: u32,
        /// Effect parameter id.
        param: ParamId,
    },
    /// Adaptive music stinger.
    Stinger {
        /// Stinger request payload.
        request: StingerRequest,
    },
}
