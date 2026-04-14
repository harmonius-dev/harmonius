//! State-driven cameras, clear shot, and simple sequencing.
//!
//! Phase-1 data uses numeric state tokens and compact playlist rows; expand toward full
//! `wait_time` / `min_duration` / per-entry blends once shared types exist (see subsystem design).

use crate::ids::Entity;

/// Maps gameplay states to virtual cameras.
#[derive(Clone, Debug, PartialEq)]
pub struct StateCameraMapping {
    /// Stable state token for tests.
    pub state_name: u32,
    /// Virtual camera entity to activate.
    pub camera: Entity,
}

/// State-driven camera configuration.
#[derive(Clone, Debug, PartialEq)]
pub struct StateDrivenCamera {
    /// Ordered mappings (first match wins).
    pub mappings: Vec<StateCameraMapping>,
}

/// Clear shot container referencing child cameras.
#[derive(Clone, Debug, PartialEq)]
pub struct ClearShot {
    /// Child cameras to evaluate.
    pub children: Vec<Entity>,
}

/// One entry in a camera sequencer playlist.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SequencerEntry {
    /// Virtual camera entity.
    pub camera: Entity,
    /// Hold duration (seconds).
    pub hold_time: f32,
}

/// Ordered playlist of virtual cameras.
#[derive(Clone, Debug, PartialEq)]
pub struct CameraSequencer {
    /// Ordered camera entries.
    pub entries: Vec<SequencerEntry>,
    /// Loop when reaching the end.
    pub loop_mode: bool,
}

/// Resolves the active camera for a discrete state token.
#[must_use]
pub fn evaluate_state_driven_camera(brain: &StateDrivenCamera, state: u32) -> Option<Entity> {
    brain
        .mappings
        .iter()
        .find(|mapping| mapping.state_name == state)
        .map(|mapping| mapping.camera)
}

/// Picks the child camera with the highest quality score.
///
/// Non-finite scores are ignored. Equal scores break ties using the higher [`Entity`] id (field
/// `0`), matching [`crate::brain::select_highest_priority_camera`].
#[must_use]
pub fn evaluate_clear_shot_child(scores: &[(Entity, f32)]) -> Option<Entity> {
    scores
        .iter()
        .filter(|(_, score)| score.is_finite())
        .max_by(|(ea, a), (eb, b)| a.total_cmp(b).then_with(|| ea.0.cmp(&eb.0)))
        .map(|(entity, _)| *entity)
}

/// Returns the active camera entity and local time within the hold for `timeline_time`.
#[must_use]
pub fn evaluate_camera_sequencer(
    sequencer: &CameraSequencer,
    mut timeline_time: f32,
) -> Option<(Entity, f32)> {
    if sequencer.entries.is_empty() {
        return None;
    }
    let total: f32 = sequencer.entries.iter().map(|entry| entry.hold_time).sum();
    if total <= 0.0 {
        return None;
    }
    if sequencer.loop_mode {
        timeline_time = timeline_time.rem_euclid(total);
    } else if timeline_time >= total {
        let last = sequencer.entries.last()?;
        return Some((last.camera, last.hold_time));
    }

    let mut cursor = 0.0;
    for entry in &sequencer.entries {
        if timeline_time < cursor + entry.hold_time {
            return Some((entry.camera, timeline_time - cursor));
        }
        cursor += entry.hold_time;
    }
    let last = sequencer.entries.last()?;
    Some((last.camera, last.hold_time))
}

#[cfg(test)]
mod tests {
    use super::*;

    /// TC-13.25.23.1 — state mapping selects the combat camera.
    #[test]
    fn tc_13_25_23_1_state_driven_mapping() {
        let brain = StateDrivenCamera {
            mappings: vec![
                StateCameraMapping {
                    state_name: 1,
                    camera: Entity(10),
                },
                StateCameraMapping {
                    state_name: 2,
                    camera: Entity(20),
                },
            ],
        };
        assert_eq!(evaluate_state_driven_camera(&brain, 2), Some(Entity(20)));
    }

    /// TC-13.25.24.1 — clear shot selects the highest scoring child.
    #[test]
    fn tc_13_25_24_1_clear_shot_selection() {
        let scores = [(Entity(1), 0.3), (Entity(2), 0.8), (Entity(3), 0.5)];
        assert_eq!(evaluate_clear_shot_child(&scores), Some(Entity(2)));
        let clear = ClearShot {
            children: scores.iter().map(|(e, _)| *e).collect(),
        };
        assert_eq!(clear.children.len(), 3);
    }

    #[test]
    fn clear_shot_ignores_nan_and_prefers_finite() {
        let scores = [(Entity(1), f32::NAN), (Entity(2), 0.5)];
        assert_eq!(evaluate_clear_shot_child(&scores), Some(Entity(2)));
    }

    #[test]
    fn clear_shot_tie_breaks_on_entity_id() {
        let scores = [(Entity(1), 0.5), (Entity(3), 0.5), (Entity(2), 0.5)];
        assert_eq!(evaluate_clear_shot_child(&scores), Some(Entity(3)));
    }

    /// TC-13.25.26.1 — sequencer advances through hold durations in order.
    #[test]
    fn tc_13_25_26_1_sequencer_playlist() {
        let sequencer = CameraSequencer {
            entries: vec![
                SequencerEntry {
                    camera: Entity(1),
                    hold_time: 2.0,
                },
                SequencerEntry {
                    camera: Entity(2),
                    hold_time: 3.0,
                },
                SequencerEntry {
                    camera: Entity(3),
                    hold_time: 1.0,
                },
            ],
            loop_mode: false,
        };
        assert_eq!(
            evaluate_camera_sequencer(&sequencer, 0.5),
            Some((Entity(1), 0.5))
        );
        assert_eq!(
            evaluate_camera_sequencer(&sequencer, 2.5),
            Some((Entity(2), 0.5))
        );
        assert_eq!(
            evaluate_camera_sequencer(&sequencer, 5.5),
            Some((Entity(3), 0.5))
        );
        assert_eq!(
            evaluate_camera_sequencer(&sequencer, 6.5),
            Some((Entity(3), 1.0))
        );
    }
}
