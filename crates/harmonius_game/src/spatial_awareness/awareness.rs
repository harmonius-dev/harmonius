//! Awareness state machine, decay, and transition events.

use std::collections::HashMap;

use glam::Vec3;

use super::sense::{AwarenessLevel, Entity, SenseDefinitionId, SenseResult};

/// Thresholds governing score-driven transitions.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct AwarenessTransition {
    /// Minimum score to enter [`AwarenessLevel::Suspicious`].
    pub suspicious_threshold: f32,
    /// Minimum score to enter [`AwarenessLevel::Alert`].
    pub alert_threshold: f32,
    /// Minimum score to enter [`AwarenessLevel::Tracking`].
    pub tracking_threshold: f32,
    /// Score removed per scaled tick when no stimulus arrives.
    pub decay_rate: f32,
    /// Ticks without contact before [`AwarenessLevel::Tracking`] becomes
    /// [`AwarenessLevel::Lost`].
    pub lost_timeout_ticks: u32,
}

/// Global knobs applied by [`update_awareness`].
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct AwarenessConfig {
    /// Fixed simulation rate the awareness systems assume.
    pub tick_rate: u32,
    /// Maximum simultaneous tracked targets per source entity.
    pub max_targets_per_entity: u16,
    /// Multiplier applied to [`AwarenessTransition::decay_rate`].
    pub global_decay_rate: f32,
}

/// One row in [`AwarenessState`].
#[derive(Clone, Debug, PartialEq)]
pub struct AwarenessEntry {
    /// Entity being observed.
    pub target: Entity,
    /// Current discrete awareness level.
    pub level: AwarenessLevel,
    /// Latest stimulus score in `[0, 1]`.
    pub score: f32,
    /// Last known world position reported by senses.
    pub last_seen_position: Vec3,
    /// Tick index of the last successful sense hit.
    pub last_seen_tick: u64,
    /// Ticks spent in [`AwarenessLevel::Lost`] without recovery.
    pub lost_ticks: u32,
    /// Ticks since the last stimulus while tracking.
    pub ticks_without_contact: u32,
}

/// ECS-facing awareness component (sans derive macro for now).
#[derive(Clone, Debug, Default, PartialEq)]
pub struct AwarenessState {
    /// Tracked targets for this perceiver.
    pub entries: Vec<AwarenessEntry>,
    /// Sense assets referenced by this entity.
    pub senses: Vec<SenseDefinitionId>,
}

impl AwarenessState {
    /// Builds an empty awareness component.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Returns the entry for `target`, if any.
    #[must_use]
    pub fn get_entry(&self, target: Entity) -> Option<&AwarenessEntry> {
        self.entries.iter().find(|entry| entry.target == target)
    }

    /// Returns the entry with the highest stimulus score.
    #[must_use]
    pub fn highest_threat(&self) -> Option<&AwarenessEntry> {
        self.entries.iter().max_by(|left, right| {
            left.score
                .total_cmp(&right.score)
                .then_with(|| left.target.cmp(&right.target))
        })
    }

    /// Lists every entity currently at `level`.
    #[must_use]
    pub fn entities_at_level(&self, level: AwarenessLevel) -> Vec<Entity> {
        self.entries
            .iter()
            .filter(|entry| entry.level == level)
            .map(|entry| entry.target)
            .collect()
    }

    /// Returns true when `target` is tracked above [`AwarenessLevel::Unaware`].
    #[must_use]
    pub fn is_aware_of(&self, target: Entity) -> bool {
        self.get_entry(target)
            .map(|entry| entry.level != AwarenessLevel::Unaware)
            .unwrap_or(false)
    }
}

/// Event emitted whenever an awareness level changes.
#[derive(Clone, Debug, PartialEq)]
pub struct AwarenessTransitionEvent {
    /// Perceiving entity.
    pub source: Entity,
    /// Observed entity.
    pub target: Entity,
    /// Previous level.
    pub from: AwarenessLevel,
    /// New level.
    pub to: AwarenessLevel,
    /// Score snapshot at transition time.
    pub score: f32,
}

fn emit_transition(
    events: &mut Vec<AwarenessTransitionEvent>,
    source: Entity,
    target: Entity,
    from: AwarenessLevel,
    to: AwarenessLevel,
    score: f32,
) {
    if from == to {
        return;
    }
    events.push(AwarenessTransitionEvent {
        source,
        target,
        from,
        to,
        score,
    });
}

fn level_from_score(score: f32, transition: &AwarenessTransition) -> AwarenessLevel {
    if score >= transition.tracking_threshold {
        AwarenessLevel::Tracking
    } else if score >= transition.alert_threshold {
        AwarenessLevel::Alert
    } else if score >= transition.suspicious_threshold {
        AwarenessLevel::Suspicious
    } else {
        AwarenessLevel::Unaware
    }
}

fn promote_score_level(
    entry: &mut AwarenessEntry,
    transition: &AwarenessTransition,
    source: Entity,
    events: &mut Vec<AwarenessTransitionEvent>,
) {
    if entry.level == AwarenessLevel::Lost {
        return;
    }
    let desired = level_from_score(entry.score, transition);
    if desired != entry.level {
        let from = entry.level;
        entry.level = desired;
        emit_transition(events, source, entry.target, from, desired, entry.score);
    }
}

fn evict_oldest(state: &mut AwarenessState) {
    if state.entries.is_empty() {
        return;
    }
    let index = state
        .entries
        .iter()
        .enumerate()
        .min_by(|(_, left), (_, right)| {
            left.last_seen_tick
                .cmp(&right.last_seen_tick)
                .then_with(|| left.target.cmp(&right.target))
        })
        .map(|pair| pair.0)
        .expect("entry");
    state.entries.remove(index);
}

/// Applies sense results, decay, and transitions for one FixedUpdate tick.
#[allow(clippy::too_many_arguments)]
pub fn update_awareness(
    state: &mut AwarenessState,
    results: &[SenseResult],
    transition: &AwarenessTransition,
    config: &AwarenessConfig,
    current_tick: u64,
    tick_scale: f32,
    source: Entity,
    events: &mut Vec<AwarenessTransitionEvent>,
) {
    if tick_scale <= f32::EPSILON {
        return;
    }

    let hits: HashMap<Entity, &SenseResult> =
        results.iter().map(|result| (result.entity, result)).collect();

    for entry in &mut state.entries {
        let had_hit = hits.contains_key(&entry.target);
        if let Some(hit) = hits.get(&entry.target) {
            entry.score = hit.final_score;
            entry.last_seen_position = hit.target_position;
            entry.last_seen_tick = current_tick;
            entry.ticks_without_contact = 0;

            if entry.level == AwarenessLevel::Lost {
                let from = entry.level;
                entry.level = AwarenessLevel::Suspicious;
                entry.lost_ticks = 0;
                emit_transition(
                    events,
                    source,
                    entry.target,
                    from,
                    AwarenessLevel::Suspicious,
                    entry.score,
                );
            }
        } else {
            let decay = transition.decay_rate * tick_scale * config.global_decay_rate;
            entry.score = (entry.score - decay).clamp(0.0, 1.0);

            if entry.level == AwarenessLevel::Tracking {
                entry.ticks_without_contact = entry.ticks_without_contact.saturating_add(1);
                if entry.ticks_without_contact >= transition.lost_timeout_ticks {
                    let from = entry.level;
                    entry.level = AwarenessLevel::Lost;
                    entry.ticks_without_contact = 0;
                    entry.lost_ticks = 0;
                    emit_transition(
                        events,
                        source,
                        entry.target,
                        from,
                        AwarenessLevel::Lost,
                        entry.score,
                    );
                }
            } else if entry.level == AwarenessLevel::Lost {
                entry.lost_ticks = entry.lost_ticks.saturating_add(1);
            }
        }

        let tracking_without_contact =
            entry.level == AwarenessLevel::Tracking && !had_hit;
        if entry.level == AwarenessLevel::Lost {
            if had_hit {
                promote_score_level(entry, transition, source, events);
            }
        } else if !tracking_without_contact {
            promote_score_level(entry, transition, source, events);
        }
    }

    for (target, hit) in &hits {
        if state.entries.iter().any(|entry| entry.target == *target) {
            continue;
        }
        if hit.final_score < transition.suspicious_threshold {
            continue;
        }

        while state.entries.len() >= config.max_targets_per_entity as usize {
            evict_oldest(state);
        }

        let mut entry = AwarenessEntry {
            target: *target,
            level: AwarenessLevel::Suspicious,
            score: hit.final_score,
            last_seen_position: hit.target_position,
            last_seen_tick: current_tick,
            lost_ticks: 0,
            ticks_without_contact: 0,
        };
        let desired = level_from_score(entry.score, transition);
        entry.level = desired;
        emit_transition(
            events,
            source,
            *target,
            AwarenessLevel::Unaware,
            desired,
            entry.score,
        );
        state.entries.push(entry);
    }

    state.entries.retain(|entry| {
        if entry.level == AwarenessLevel::Lost {
            if entry.lost_ticks >= transition.lost_timeout_ticks {
                emit_transition(
                    events,
                    source,
                    entry.target,
                    AwarenessLevel::Lost,
                    AwarenessLevel::Unaware,
                    entry.score,
                );
                return false;
            }
            return true;
        }

        if entry.score < transition.suspicious_threshold {
            emit_transition(
                events,
                source,
                entry.target,
                entry.level,
                AwarenessLevel::Unaware,
                entry.score,
            );
            return false;
        }

        true
    });
}

/// Builds a synthetic sense hit for an environmental noise source.
#[must_use]
pub fn apply_noise_pulse(
    perceived: Entity,
    noise_position: Vec3,
    listener_position: Vec3,
    radius: f32,
    pulse_score: f32,
) -> Option<SenseResult> {
    let distance = (noise_position - listener_position).length();
    if distance > radius {
        return None;
    }
    Some(SenseResult {
        entity: perceived,
        distance,
        angle: 0.0,
        occlusion: 0.0,
        raw_score: pulse_score,
        final_score: pulse_score,
        target_position: noise_position,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::spatial_awareness::sense::{
        FalloffCurve, ScoringFunction, SenseCandidate, SenseDefinition, SenseDefinitionId,
        SenseShape, StringId, TagSet,
    };
    use crate::spatial_awareness::{query::SenseQueryOptions, query_sense};

    fn default_transition() -> AwarenessTransition {
        AwarenessTransition {
            suspicious_threshold: 0.2,
            alert_threshold: 0.5,
            tracking_threshold: 0.8,
            decay_rate: 0.1,
            lost_timeout_ticks: 3,
        }
    }

    fn default_config() -> AwarenessConfig {
        AwarenessConfig {
            tick_rate: 60,
            max_targets_per_entity: 8,
            global_decay_rate: 1.0,
        }
    }

    fn sense_hit(target: Entity, score: f32) -> SenseResult {
        SenseResult {
            entity: target,
            distance: 1.0,
            angle: 0.0,
            occlusion: 0.0,
            raw_score: score,
            final_score: score,
            target_position: Vec3::Z,
        }
    }

    /// TC-13.18.1.1 — suspicious promotion.
    #[test]
    fn test_awareness_unaware_to_suspicious() {
        let mut state = AwarenessState::new();
        let mut events = Vec::new();
        update_awareness(
            &mut state,
            &[sense_hit(Entity(1), 0.3)],
            &default_transition(),
            &default_config(),
            1,
            1.0,
            Entity(99),
            &mut events,
        );
        assert_eq!(state.entries.len(), 1);
        assert_eq!(state.entries[0].level, AwarenessLevel::Suspicious);
    }

    /// TC-13.18.1.2 — alert promotion.
    #[test]
    fn test_awareness_suspicious_to_alert() {
        let mut state = AwarenessState::new();
        let mut events = Vec::new();
        update_awareness(
            &mut state,
            &[sense_hit(Entity(1), 0.6)],
            &default_transition(),
            &default_config(),
            1,
            1.0,
            Entity(99),
            &mut events,
        );
        assert_eq!(state.entries[0].level, AwarenessLevel::Alert);
    }

    /// TC-13.18.1.3 — tracking promotion.
    #[test]
    fn test_awareness_alert_to_tracking() {
        let mut state = AwarenessState::new();
        let mut events = Vec::new();
        update_awareness(
            &mut state,
            &[sense_hit(Entity(1), 0.9)],
            &default_transition(),
            &default_config(),
            1,
            1.0,
            Entity(99),
            &mut events,
        );
        assert_eq!(state.entries[0].level, AwarenessLevel::Tracking);
    }

    /// TC-13.18.1.4 — tracking falls to lost without stimulus.
    #[test]
    fn test_awareness_tracking_to_lost() {
        let mut state = AwarenessState::new();
        let mut events = Vec::new();
        update_awareness(
            &mut state,
            &[sense_hit(Entity(1), 0.9)],
            &default_transition(),
            &default_config(),
            1,
            1.0,
            Entity(99),
            &mut events,
        );
        for tick in 2..=4 {
            update_awareness(
                &mut state,
                &[],
                &default_transition(),
                &default_config(),
                tick,
                1.0,
                Entity(99),
                &mut events,
            );
        }
        assert_eq!(state.entries[0].level, AwarenessLevel::Lost);
    }

    /// TC-13.18.1.5 — lost expires to removal.
    #[test]
    fn test_awareness_lost_to_unaware() {
        let mut state = AwarenessState::new();
        let mut events = Vec::new();
        update_awareness(
            &mut state,
            &[sense_hit(Entity(1), 0.9)],
            &default_transition(),
            &default_config(),
            1,
            1.0,
            Entity(99),
            &mut events,
        );
        for tick in 2..=4 {
            update_awareness(
                &mut state,
                &[],
                &default_transition(),
                &default_config(),
                tick,
                1.0,
                Entity(99),
                &mut events,
            );
        }
        for tick in 5..=7 {
            update_awareness(
                &mut state,
                &[],
                &default_transition(),
                &default_config(),
                tick,
                1.0,
                Entity(99),
                &mut events,
            );
        }
        assert!(state.entries.is_empty());
    }

    /// TC-13.18.1.6 — lost regains suspicious on new stimulus.
    #[test]
    fn test_awareness_lost_to_suspicious() {
        let mut state = AwarenessState::new();
        let mut events = Vec::new();
        update_awareness(
            &mut state,
            &[sense_hit(Entity(1), 0.9)],
            &default_transition(),
            &default_config(),
            1,
            1.0,
            Entity(99),
            &mut events,
        );
        for tick in 2..=4 {
            update_awareness(
                &mut state,
                &[],
                &default_transition(),
                &default_config(),
                tick,
                1.0,
                Entity(99),
                &mut events,
            );
        }
        update_awareness(
            &mut state,
            &[sense_hit(Entity(1), 0.3)],
            &default_transition(),
            &default_config(),
            5,
            1.0,
            Entity(99),
            &mut events,
        );
        assert_eq!(state.entries[0].level, AwarenessLevel::Suspicious);
    }

    /// TC-7.6.4.1 — decay lowers score without stimulus.
    #[test]
    fn test_awareness_decay_reduces_score() {
        let mut state = AwarenessState::new();
        let mut events = Vec::new();
        update_awareness(
            &mut state,
            &[sense_hit(Entity(1), 0.5)],
            &default_transition(),
            &default_config(),
            1,
            1.0,
            Entity(99),
            &mut events,
        );
        update_awareness(
            &mut state,
            &[],
            &default_transition(),
            &default_config(),
            2,
            1.0,
            Entity(99),
            &mut events,
        );
        assert!((state.entries[0].score - 0.4).abs() < 1e-4);
    }

    /// TC-7.6.4.2 — decay removes entry below suspicious threshold.
    #[test]
    fn test_awareness_decay_below_threshold() {
        let mut state = AwarenessState::new();
        let mut events = Vec::new();
        update_awareness(
            &mut state,
            &[sense_hit(Entity(1), 0.25)],
            &default_transition(),
            &default_config(),
            1,
            1.0,
            Entity(99),
            &mut events,
        );
        for tick in 2..=10 {
            update_awareness(
                &mut state,
                &[],
                &default_transition(),
                &default_config(),
                tick,
                1.0,
                Entity(99),
                &mut events,
            );
        }
        assert!(state.entries.is_empty());
    }

    /// TC-7.6.5.1 — oldest entry evicted when exceeding max targets.
    #[test]
    fn test_awareness_max_targets_eviction() {
        let mut state = AwarenessState::new();
        let mut events = Vec::new();
        let config = AwarenessConfig {
            max_targets_per_entity: 2,
            ..default_config()
        };
        update_awareness(
            &mut state,
            &[sense_hit(Entity(1), 0.4), sense_hit(Entity(2), 0.4)],
            &default_transition(),
            &config,
            1,
            1.0,
            Entity(99),
            &mut events,
        );
        update_awareness(
            &mut state,
            &[sense_hit(Entity(3), 0.4)],
            &default_transition(),
            &config,
            2,
            1.0,
            Entity(99),
            &mut events,
        );
        let targets: Vec<_> = state.entries.iter().map(|entry| entry.target).collect();
        assert_eq!(targets.len(), 2);
        assert!(!targets.contains(&Entity(1)));
    }

    /// TC-13.18.2.1 — highest threat score wins ties by entity id.
    #[test]
    fn test_highest_threat_returns_max() {
        let mut state = AwarenessState::new();
        state.entries.push(AwarenessEntry {
            target: Entity(1),
            level: AwarenessLevel::Suspicious,
            score: 0.3,
            last_seen_position: Vec3::ZERO,
            last_seen_tick: 0,
            lost_ticks: 0,
            ticks_without_contact: 0,
        });
        state.entries.push(AwarenessEntry {
            target: Entity(2),
            level: AwarenessLevel::Suspicious,
            score: 0.8,
            last_seen_position: Vec3::ZERO,
            last_seen_tick: 0,
            lost_ticks: 0,
            ticks_without_contact: 0,
        });
        state.entries.push(AwarenessEntry {
            target: Entity(3),
            level: AwarenessLevel::Suspicious,
            score: 0.5,
            last_seen_position: Vec3::ZERO,
            last_seen_tick: 0,
            lost_ticks: 0,
            ticks_without_contact: 0,
        });
        assert_eq!(state.highest_threat().expect("entry").score, 0.8);
    }

    /// TC-13.18.2.2 — entities_at_level filters correctly.
    #[test]
    fn test_entities_at_level_filter() {
        let mut state = AwarenessState::new();
        state.entries.push(AwarenessEntry {
            target: Entity(1),
            level: AwarenessLevel::Suspicious,
            score: 0.3,
            last_seen_position: Vec3::ZERO,
            last_seen_tick: 0,
            lost_ticks: 0,
            ticks_without_contact: 0,
        });
        state.entries.push(AwarenessEntry {
            target: Entity(2),
            level: AwarenessLevel::Alert,
            score: 0.6,
            last_seen_position: Vec3::ZERO,
            last_seen_tick: 0,
            lost_ticks: 0,
            ticks_without_contact: 0,
        });
        let alerts = state.entities_at_level(AwarenessLevel::Alert);
        assert_eq!(alerts, vec![Entity(2)]);
    }

    /// TC-13.18.2.3 — `is_aware_of` respects entry presence.
    #[test]
    fn test_is_aware_of_above_unaware() {
        let mut state = AwarenessState::new();
        state.entries.push(AwarenessEntry {
            target: Entity(1),
            level: AwarenessLevel::Suspicious,
            score: 0.3,
            last_seen_position: Vec3::ZERO,
            last_seen_tick: 0,
            lost_ticks: 0,
            ticks_without_contact: 0,
        });
        assert!(state.is_aware_of(Entity(1)));
        assert!(!state.is_aware_of(Entity(2)));
    }

    /// TC-17.3.4.1 — transition events record endpoints.
    #[test]
    fn test_transition_event_emitted() {
        let mut state = AwarenessState::new();
        let mut events = Vec::new();
        update_awareness(
            &mut state,
            &[sense_hit(Entity(1), 0.3)],
            &default_transition(),
            &default_config(),
            1,
            1.0,
            Entity(7),
            &mut events,
        );
        update_awareness(
            &mut state,
            &[sense_hit(Entity(1), 0.6)],
            &default_transition(),
            &default_config(),
            2,
            1.0,
            Entity(7),
            &mut events,
        );
        let suspicious_to_alert = events.iter().find(|event| {
            event.from == AwarenessLevel::Suspicious && event.to == AwarenessLevel::Alert
        });
        assert!(suspicious_to_alert.is_some());
        assert_eq!(suspicious_to_alert.expect("event").source, Entity(7));
        assert_eq!(suspicious_to_alert.expect("event").target, Entity(1));
    }

    /// TC-17.3.3.1 — scripted progression across all discrete levels.
    #[test]
    fn test_awareness_state_machine_5() {
        let mut state = AwarenessState::new();
        let mut events = Vec::new();
        let transition = default_transition();
        let config = default_config();
        update_awareness(
            &mut state,
            &[sense_hit(Entity(1), 0.25)],
            &transition,
            &config,
            1,
            1.0,
            Entity(1),
            &mut events,
        );
        assert_eq!(state.entries[0].level, AwarenessLevel::Suspicious);
        update_awareness(
            &mut state,
            &[sense_hit(Entity(1), 0.55)],
            &transition,
            &config,
            2,
            1.0,
            Entity(1),
            &mut events,
        );
        assert_eq!(state.entries[0].level, AwarenessLevel::Alert);
        update_awareness(
            &mut state,
            &[sense_hit(Entity(1), 0.85)],
            &transition,
            &config,
            3,
            1.0,
            Entity(1),
            &mut events,
        );
        assert_eq!(state.entries[0].level, AwarenessLevel::Tracking);
        for tick in 4..=6 {
            update_awareness(
                &mut state,
                &[],
                &transition,
                &config,
                tick,
                1.0,
                Entity(1),
                &mut events,
            );
        }
        assert_eq!(state.entries[0].level, AwarenessLevel::Lost);
    }

    /// TC-7.6.6.2 — identical inputs yield identical outputs on the same tick.
    #[test]
    fn test_fixed_update_tick_determinism() {
        let mut left_state = AwarenessState::new();
        let mut right_state = AwarenessState::new();
        let hits = [sense_hit(Entity(1), 0.45)];
        let transition = default_transition();
        let config = default_config();
        let mut left_events = Vec::new();
        let mut right_events = Vec::new();
        update_awareness(
            &mut left_state,
            &hits,
            &transition,
            &config,
            4,
            1.0,
            Entity(1),
            &mut left_events,
        );
        update_awareness(
            &mut right_state,
            &hits,
            &transition,
            &config,
            4,
            1.0,
            Entity(1),
            &mut right_events,
        );
        assert_eq!(left_state, right_state);
        assert_eq!(left_events, right_events);
    }

    /// TC-13.18.3.1 — noise pulse raises awareness.
    #[test]
    fn test_noise_stimulus_distracts() {
        let mut state = AwarenessState::new();
        let mut events = Vec::new();
        let pulse = apply_noise_pulse(
            Entity(404),
            Vec3::new(10.0, 0.0, 0.0),
            Vec3::ZERO,
            15.0,
            0.4,
        );
        update_awareness(
            &mut state,
            &[pulse.expect("pulse")],
            &default_transition(),
            &default_config(),
            1,
            1.0,
            Entity(1),
            &mut events,
        );
        assert!(state.entries[0].score >= 0.2);
        assert_eq!(state.entries[0].level, AwarenessLevel::Suspicious);
    }

    /// TC-1.9.9.1 — sense output feeds awareness without an extra bridge type.
    #[test]
    fn test_ai_perception_integration() {
        let mut state = AwarenessState::new();
        let mut events = Vec::new();
        let definition = SenseDefinition {
            id: SenseDefinitionId(1),
            name: StringId(1),
            shape: SenseShape::Cone {
                radius: 30.0,
                half_angle: std::f32::consts::FRAC_PI_4,
            },
            range: 30.0,
            falloff: FalloffCurve::Linear,
            filter_tags: TagSet::empty(),
            occlusion_check: false,
            scoring: ScoringFunction::default(),
            update_rate_hz: 30.0,
        };
        let origin = Vec3::ZERO;
        let forward = Vec3::Z;
        let candidates: Vec<_> = (1..=10_u64)
            .map(|idx| SenseCandidate {
                entity: Entity(idx),
                position: Vec3::new(0.0, 0.0, idx as f32 * 2.0),
                tags: TagSet::empty(),
            })
            .collect();
        let sense_hits = query_sense(
            &definition,
            origin,
            forward,
            &candidates,
            &SenseQueryOptions::default(),
        );
        update_awareness(
            &mut state,
            &sense_hits,
            &default_transition(),
            &default_config(),
            1,
            1.0,
            Entity(100),
            &mut events,
        );
        assert!(!state.entries.is_empty());
    }

    /// TC-17.3.10.1 — stimulus visible on the next awareness pass.
    #[test]
    fn test_one_frame_stimulus_latency() {
        let mut state = AwarenessState::new();
        let mut events = Vec::new();
        update_awareness(
            &mut state,
            &[sense_hit(Entity(1), 0.4)],
            &default_transition(),
            &default_config(),
            10,
            1.0,
            Entity(5),
            &mut events,
        );
        assert!(state.is_aware_of(Entity(1)));
        let snapshot = state.clone();
        update_awareness(
            &mut state,
            &[],
            &default_transition(),
            &default_config(),
            11,
            1.0,
            Entity(5),
            &mut events,
        );
        assert_ne!(snapshot.entries[0].score, state.entries[0].score);
    }
}
