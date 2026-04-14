//! Sight / hearing perception helpers and propagation snapshot.
//!
//! When a sense definition is missing, [`apply_sight_perception`] increments
//! `missing_sense_warnings` instead of emitting logs (FM-2); engine wiring may map this counter
//! to `tracing` later.

use std::collections::{BTreeMap, BTreeSet};

pub use crate::types::{Entity, Vec3};

/// Which sense produced a [`PerceivedEntity`] row.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum PerceptionSense {
    /// Vision cone sense.
    Sight,
    /// Hearing sphere sense.
    Hearing,
}

/// One perceived target row in [`AiPerception`].
#[derive(Clone, Debug, PartialEq)]
pub struct PerceivedEntity {
    /// Perceived entity.
    pub entity: Entity,
    /// Last observed world position.
    pub last_seen_pos: Vec3,
    /// Simulation time when last observed.
    pub last_seen_time: f32,
    /// Originating sense.
    pub sense: PerceptionSense,
}

/// Agent-side perception memory (design `AiPerception` subset used in tests).
#[derive(Clone, Debug, PartialEq)]
pub struct AiPerception {
    /// Seconds to retain a target after last observation.
    pub memory_duration: f32,
    /// Remembered targets.
    pub known_entities: Vec<PerceivedEntity>,
}

impl AiPerception {
    /// Creates empty perception memory.
    pub fn new(memory_duration: f32) -> Self {
        Self {
            memory_duration,
            known_entities: Vec::new(),
        }
    }

    /// Removes entries whose targets are not in `alive` and whose memory expired at `now`.
    pub fn apply_memory_decay(&mut self, now: f32, alive: &BTreeSet<Entity>) {
        self.known_entities.retain(|p| {
            let alive_ok = alive.contains(&p.entity);
            let fresh = now - p.last_seen_time <= self.memory_duration;
            alive_ok && fresh
        });
    }

    /// Upserts a perceived entity (replaces same entity+sense rows).
    pub fn upsert(&mut self, perceived: PerceivedEntity) {
        if let Some(idx) = self
            .known_entities
            .iter()
            .position(|p| p.entity == perceived.entity && p.sense == perceived.sense)
        {
            self.known_entities[idx] = perceived;
        } else {
            self.known_entities.push(perceived);
        }
    }
}

/// ECS snapshot of propagated audio intensity at the listener (per source entity).
#[derive(Clone, Debug, Default, PartialEq)]
pub struct PropagationResultStore {
    /// Source entity to intensity at the listener position.
    pub intensities: BTreeMap<Entity, f32>,
}

impl PropagationResultStore {
    /// Looks up intensity for `source`.
    pub fn intensity_for_source(&self, source: Entity) -> Option<f32> {
        self.intensities.get(&source).copied()
    }
}

/// Inputs for deterministic sight scoring.
#[derive(Clone, Copy, Debug)]
pub struct SightQueryInput {
    /// Agent forward (world space), not required to be unit length.
    pub agent_forward: Vec3,
    /// Vector from agent to target (target - agent).
    pub offset_to_target: Vec3,
    /// Maximum sight range.
    pub max_range: f32,
    /// Half-angle of the sight cone (radians).
    pub half_angle_rad: f32,
    /// When false, line of sight is fully blocked.
    pub line_of_sight_clear: bool,
    /// Multiplier in `0.0..=1.0` for partial occlusion (1.0 clear).
    pub occlusion_factor: f32,
}

/// Result of a sight evaluation for one candidate.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SenseQueryResult {
    /// Whether the candidate is inside the sense frustum and unblocked.
    pub perceived: bool,
    /// Combined `0.0..=1.0` score (distance, angle, occlusion).
    pub score: f32,
}

/// Computes in-cone + line-of-sight sight with a smooth distance falloff.
pub fn evaluate_sight(input: SightQueryInput) -> SenseQueryResult {
    let dist = input.offset_to_target.length();
    if dist > input.max_range || !input.line_of_sight_clear {
        return SenseQueryResult {
            perceived: false,
            score: 0.0,
        };
    }
    let dir = input.offset_to_target.normalize_or_zero();
    let forward = input.agent_forward.normalize_or_zero();
    let cos_limit = input.half_angle_rad.cos();
    let cos_angle = dir.dot(forward);
    if cos_angle < cos_limit {
        return SenseQueryResult {
            perceived: false,
            score: 0.0,
        };
    }
    let dist_score = 1.0 - (dist / input.max_range).clamp(0.0, 1.0);
    let angle_score = ((cos_angle - cos_limit) / (1.0 - cos_limit).max(1e-6)).clamp(0.0, 1.0);
    let score = (dist_score * 0.5 + angle_score * 0.5) * input.occlusion_factor.clamp(0.0, 1.0);
    SenseQueryResult {
        perceived: score > 0.0,
        score,
    }
}

/// Inputs for hearing against one source.
#[derive(Clone, Copy, Debug)]
pub struct HearingQueryInput {
    /// Sound source entity.
    pub source: Entity,
    /// Minimum intensity required to register perception.
    pub threshold: f32,
}

/// Returns perceived intensity when `store` has propagation data above `threshold`.
pub fn evaluate_hearing(store: &PropagationResultStore, input: HearingQueryInput) -> Option<f32> {
    store
        .intensity_for_source(input.source)
        .filter(|&i| i >= input.threshold)
}

/// Neutral geometric score when faction data is missing (IR-1.10.N.2).
pub fn neutral_target_score(base: f32) -> f32 {
    base
}

/// Merges a sight query into `perception` when `sense_loaded` is true; otherwise increments
/// `missing_sense_warnings` once per skipped evaluation (FM-2 / TC-IR-1.10.N.4).
pub fn apply_sight_perception(
    perception: &mut AiPerception,
    target: Entity,
    target_pos: Vec3,
    now: f32,
    input: SightQueryInput,
    sense_loaded: bool,
    missing_sense_warnings: &mut u32,
) -> SenseQueryResult {
    if !sense_loaded {
        *missing_sense_warnings = missing_sense_warnings.saturating_add(1);
        return SenseQueryResult {
            perceived: false,
            score: 0.0,
        };
    }
    let result = evaluate_sight(input);
    if result.perceived {
        perception.upsert(PerceivedEntity {
            entity: target,
            last_seen_pos: target_pos,
            last_seen_time: now,
            sense: PerceptionSense::Sight,
        });
    }
    result
}

/// Merges hearing when propagation exists; otherwise does nothing (FM-6 / TC-IR-1.10.N.3).
pub fn apply_hearing_perception(
    perception: &mut AiPerception,
    store: &PropagationResultStore,
    target_pos: Vec3,
    now: f32,
    input: HearingQueryInput,
) -> bool {
    if let Some(_intensity) = evaluate_hearing(store, input) {
        perception.upsert(PerceivedEntity {
            entity: input.source,
            last_seen_pos: target_pos,
            last_seen_time: now,
            sense: PerceptionSense::Hearing,
        });
        true
    } else {
        false
    }
}
