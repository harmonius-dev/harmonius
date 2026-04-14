//! Animation timeline, curves, blend spaces, and state machines.

use crate::material_graph::AssetId;
use glam::Vec2;

/// Single scalar keyframe.
#[derive(Debug, Clone, PartialEq)]
pub struct Keyframe {
    pub time: f32,
    pub value: f32,
    pub tangent_mode: TangentMode,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TangentMode {
    Bezier,
    Hermite,
    Linear,
    Stepped,
    Auto,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AnimChannel {
    LocX,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct BoneId(pub u32);

#[derive(Debug, Clone)]
pub struct AnimTrack {
    pub bone: BoneId,
    pub channel: AnimChannel,
    pub keyframes: Vec<Keyframe>,
}

#[derive(Debug, Clone)]
pub struct AnimClip {
    pub id: AssetId,
    pub duration: f32,
    pub sample_rate: f32,
    pub tracks: Vec<AnimTrack>,
}

/// Multi-track editor state.
#[derive(Debug, Clone)]
pub struct AnimTimeline {
    pub time: f32,
    pub tracks: Vec<AnimTrack>,
    pub playing: bool,
    pub preview_bone_overlay_count: u32,
    pub preview_velocity_overlay: bool,
}

impl AnimTimeline {
    /// Empty timeline.
    pub fn new() -> Self {
        Self {
            time: 0.0,
            tracks: Vec::new(),
            playing: false,
            preview_bone_overlay_count: 0,
            preview_velocity_overlay: false,
        }
    }

    /// Imports clip tracks.
    pub fn load_clip(&mut self, clip: &AnimClip) {
        self.tracks = clip.tracks.clone();
    }

    /// Sets the playhead time.
    pub fn set_time(&mut self, time: f32) {
        self.time = time;
    }

    /// Starts playback flag.
    pub fn play(&mut self) {
        self.playing = true;
    }

    /// Pauses playback flag.
    pub fn pause(&mut self) {
        self.playing = false;
    }

    /// Inserts a keyframe sorted by time on a track.
    pub fn add_keyframe(&mut self, track: usize, value: f32) {
        let t = self.tracks.get_mut(track).expect("track");
        t.keyframes.push(Keyframe {
            time: self.time,
            value,
            tangent_mode: TangentMode::Linear,
        });
        t.keyframes.sort_by(|a, b| {
            a.time
                .partial_cmp(&b.time)
                .unwrap_or(std::cmp::Ordering::Equal)
        });
    }

    /// Enables debug overlays for preview.
    pub fn set_preview_debug(&mut self, bones: u32, velocity: bool) {
        self.preview_bone_overlay_count = bones;
        self.preview_velocity_overlay = velocity;
    }
}

impl Default for AnimTimeline {
    fn default() -> Self {
        Self::new()
    }
}

/// Applies a tangent preset to the first keyframe on a track.
pub fn apply_curve_preset(track: &mut AnimTrack, mode: TangentMode) {
    if let Some(k) = track.keyframes.first_mut() {
        k.tangent_mode = mode;
    }
}

/// Evaluates Bezier-like tangents using linear chord for tests.
pub fn eval_curve_tangent(from: &Keyframe, to: &Keyframe, t: f32) -> f32 {
    from.value + (to.value - from.value) * t.clamp(0.0, 1.0)
}

#[derive(Debug, Clone)]
pub struct Skeleton {
    pub bones: Vec<BoneId>,
    pub selected: Option<BoneId>,
}

impl Skeleton {
    /// Selects a bone by id when present.
    pub fn select_bone(&mut self, bone: BoneId) {
        if self.bones.contains(&bone) {
            self.selected = Some(bone);
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BlendSpaceDimension {
    OneD,
    TwoD,
}

#[derive(Debug, Clone)]
pub struct BlendSample {
    pub coord: Vec2,
    pub clip_weight: f32,
}

#[derive(Debug, Clone)]
pub struct BlendSpace {
    pub id: AssetId,
    pub dimension: BlendSpaceDimension,
    pub samples: Vec<BlendSample>,
}

impl BlendSpace {
    /// 1D lerp between two samples by x coordinate.
    pub fn sample_1d(&self, x: f32) -> f32 {
        assert_eq!(self.dimension, BlendSpaceDimension::OneD);
        let s0 = self.samples[0].coord.x;
        let s1 = self.samples[1].coord.x;
        let w0 = self.samples[0].clip_weight;
        let w1 = self.samples[1].clip_weight;
        let t = ((x - s0) / (s1 - s0).max(1e-6)).clamp(0.0, 1.0);
        w0 * (1.0 - t) + w1 * t
    }

    /// Barycentric blend inside the triangle spanned by three samples.
    pub fn sample_2d(&self, p: Vec2) -> f32 {
        assert_eq!(self.dimension, BlendSpaceDimension::TwoD);
        let a = self.samples[0].coord;
        let b = self.samples[1].coord;
        let c = self.samples[2].coord;
        let v0 = b - a;
        let v1 = c - a;
        let v2 = p - a;
        let d00 = v0.dot(v0);
        let d01 = v0.dot(v1);
        let d11 = v1.dot(v1);
        let d20 = v2.dot(v0);
        let d21 = v2.dot(v1);
        let denom = d00 * d11 - d01 * d01;
        let v = (d11 * d20 - d01 * d21) / denom.max(1e-6);
        let w = (d00 * d21 - d01 * d20) / denom.max(1e-6);
        let u = 1.0 - v - w;
        u * self.samples[0].clip_weight
            + v * self.samples[1].clip_weight
            + w * self.samples[2].clip_weight
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct StateId(pub u32);

#[derive(Debug, Clone)]
pub struct AnimState {
    pub id: StateId,
    pub name: String,
}

#[derive(Debug, Clone)]
pub struct AnimTransition {
    pub from: StateId,
    pub to: StateId,
    pub condition_met: bool,
}

#[derive(Debug, Clone)]
pub struct AnimStateMachine {
    pub id: AssetId,
    pub states: Vec<AnimState>,
    pub transitions: Vec<AnimTransition>,
    pub default_state: StateId,
    pub current: StateId,
}

impl AnimStateMachine {
    /// Applies the first transition whose condition is true.
    pub fn step_transition(&mut self) -> Option<StateId> {
        for tr in &self.transitions {
            if tr.condition_met && tr.from == self.current {
                self.current = tr.to;
                return Some(tr.to);
            }
        }
        None
    }
}

/// Runtime FSM compiled from the AI state machine editor.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CompiledFsm {
    pub state_count: usize,
}

pub fn compile_ai_state_machine(sm: &AnimStateMachine) -> CompiledFsm {
    CompiledFsm {
        state_count: sm.states.len(),
    }
}

/// Source and target bone ids for retargeting.
#[derive(Debug, Clone)]
pub struct BoneMapping {
    pub src: BoneId,
    pub dst: BoneId,
}

/// Applies a mapping table to produce target ids.
pub fn apply_retarget(mappings: &[BoneMapping], src: BoneId) -> Option<BoneId> {
    mappings.iter().find(|m| m.src == src).map(|m| m.dst)
}
