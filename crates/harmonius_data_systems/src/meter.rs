//! Bounded meters with drain/fill and threshold crossings.
use crate::shared::{GameplayTagSet, RowRef, StatAggregator};
use smallvec::SmallVec;
use smol_str::SmolStr;

/// Identifier for a meter definition row.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct MeterDefinitionId(pub u32);

/// Drain and fill configuration expressed in units per second.
#[derive(Clone, Debug, PartialEq)]
pub struct MeterDrainFill {
    /// Units removed per second while draining.
    pub drain_rate: f32,
    /// Units restored per second while filling.
    pub fill_rate: f32,
    /// Whether draining continues at the minimum bound.
    pub drain_when_empty: bool,
    /// Whether filling continues at the maximum bound.
    pub fill_when_full: bool,
}

/// Direction filter for threshold crossings.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ThresholdDirection {
    /// Fires when the value increases through the threshold.
    Rising,
    /// Fires when the value decreases through the threshold.
    Falling,
    /// Fires for either crossing direction.
    Either,
}

/// Action executed when a threshold is crossed.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ThresholdAction {
    /// Emit a crossing event only.
    FireEvent,
    /// Apply an effect referenced by the threshold row.
    ApplyEffect,
    /// Remove an effect referenced by the threshold row.
    RemoveEffect,
}

/// Threshold metadata attached to a meter definition.
#[derive(Clone, Debug, PartialEq)]
pub struct MeterThreshold {
    /// Threshold value in meter units.
    pub value: f32,
    /// Crossing direction filter.
    pub direction: ThresholdDirection,
    /// Action associated with the crossing.
    pub action: ThresholdAction,
    /// Optional gameplay effect row.
    pub effect_ref: Option<RowRef>,
    /// Optional tags granted while the condition holds.
    pub tag_grant: Option<GameplayTagSet>,
}

/// Immutable meter definition used for clamping and tick integration.
#[derive(Clone, Debug, PartialEq)]
pub struct MeterDefinition {
    /// Stable definition identifier.
    pub id: MeterDefinitionId,
    /// Human readable name for tooling.
    pub display_name: SmolStr,
    /// Minimum bound for the meter value.
    pub min_value: f32,
    /// Maximum bound for the meter value.
    pub max_value: f32,
    /// Initial value when constructed from the definition.
    pub default_value: f32,
    /// Drain and fill configuration.
    pub drain_fill: MeterDrainFill,
    /// Thresholds evaluated after each value change.
    pub thresholds: Vec<MeterThreshold>,
    /// Whether the meter should be serialized on save.
    pub save_on_persist: bool,
    /// Whether the meter should replicate over the network.
    pub replicate: bool,
}

/// Recorded threshold crossing for consumers/tests.
#[derive(Clone, Debug, PartialEq)]
pub struct ThresholdCrossing {
    /// Threshold value that was crossed.
    pub threshold_value: f32,
    /// Observed crossing direction.
    pub direction: ThresholdDirection,
    /// Action configured on the threshold.
    pub action: ThresholdAction,
}

/// Runtime meter state for a single definition.
#[derive(Clone, Debug, PartialEq)]
pub struct Meter {
    /// Owning definition identifier.
    pub definition_id: MeterDefinitionId,
    /// Current meter value after the last committed update.
    pub current_value: f32,
    /// Value prior to the last update (used for threshold detection).
    pub previous_value: f32,
    /// Modifier stack layered on top of the raw value.
    pub modifier_stack: StatAggregator,
}

impl Meter {
    /// Builds a meter from an immutable definition.
    pub fn from_definition(definition: &MeterDefinition) -> Self {
        Self {
            definition_id: definition.id,
            current_value: definition.default_value,
            previous_value: definition.default_value,
            modifier_stack: StatAggregator::new(),
        }
    }

    /// Returns the fractional position between min and max.
    #[must_use]
    pub fn fraction(&self, definition: &MeterDefinition) -> f32 {
        let span = definition.max_value - definition.min_value;
        if span.abs() <= f32::EPSILON {
            return 0.0;
        }
        (self.current_value - definition.min_value) / span
    }

    /// Applies an immediate delta with clamping and records `previous_value`.
    pub fn apply_delta(&mut self, delta: f32, definition: &MeterDefinition) {
        self.previous_value = self.current_value;
        let mut next = self.current_value + delta;
        next = clamp_scalar(next, definition.min_value, definition.max_value);
        self.current_value = next;
    }

    /// Sets the current value directly with clamping.
    pub fn set_value(&mut self, value: f32, definition: &MeterDefinition) {
        self.previous_value = self.current_value;
        self.current_value = clamp_scalar(value, definition.min_value, definition.max_value);
    }

    /// Advances drain/fill integration for `dt` seconds.
    pub fn tick_drain_fill(&mut self, dt: f32, definition: &MeterDefinition) {
        if dt <= 0.0 {
            return;
        }

        let mut delta = 0.0f32;
        if definition.drain_fill.fill_rate > 0.0 {
            delta += definition.drain_fill.fill_rate * dt;
        }
        if definition.drain_fill.drain_rate > 0.0 {
            delta -= definition.drain_fill.drain_rate * dt;
        }

        if delta != 0.0 {
            self.apply_delta(delta, definition);
        }
    }

    /// Evaluates thresholds after a value change, returning any crossings.
    pub fn poll_thresholds(
        &self,
        definition: &MeterDefinition,
    ) -> SmallVec<[ThresholdCrossing; 4]> {
        let mut events = SmallVec::new();
        let previous = self.previous_value;
        let current = self.current_value;

        for threshold in &definition.thresholds {
            if let Some(direction) = crossing_direction(previous, current, threshold.value) {
                let matches = match threshold.direction {
                    ThresholdDirection::Rising => direction == ThresholdDirection::Rising,
                    ThresholdDirection::Falling => direction == ThresholdDirection::Falling,
                    ThresholdDirection::Either => true,
                };

                if matches {
                    events.push(ThresholdCrossing {
                        threshold_value: threshold.value,
                        direction,
                        action: threshold.action,
                    });
                }
            }
        }

        events
    }
}

fn crossing_direction(previous: f32, current: f32, threshold: f32) -> Option<ThresholdDirection> {
    if previous < threshold && current >= threshold {
        Some(ThresholdDirection::Rising)
    } else if previous > threshold && current <= threshold {
        Some(ThresholdDirection::Falling)
    } else {
        None
    }
}

fn clamp_scalar(value: f32, min_value: f32, max_value: f32) -> f32 {
    value.clamp(min_value, max_value)
}

/// Component-style collection of meters keyed by definition id.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct MeterSet {
    /// Inline meters for hot paths.
    pub meters: SmallVec<[Meter; 4]>,
}

impl MeterSet {
    /// Borrows a meter by id.
    #[must_use]
    pub fn get(&self, id: MeterDefinitionId) -> Option<&Meter> {
        self.meters.iter().find(|meter| meter.definition_id == id)
    }

    /// Mutably borrows a meter by id.
    pub fn get_mut(&mut self, id: MeterDefinitionId) -> Option<&mut Meter> {
        self.meters
            .iter_mut()
            .find(|meter| meter.definition_id == id)
    }

    /// Inserts or replaces a meter entry.
    pub fn insert(&mut self, meter: Meter) {
        if let Some(existing) = self
            .meters
            .iter_mut()
            .find(|candidate| candidate.definition_id == meter.definition_id)
        {
            *existing = meter;
            return;
        }
        self.meters.push(meter);
    }

    /// Removes a meter by id.
    pub fn remove(&mut self, id: MeterDefinitionId) -> Option<Meter> {
        let index = self
            .meters
            .iter()
            .position(|meter| meter.definition_id == id)?;
        Some(self.meters.remove(index))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn basic_definition() -> MeterDefinition {
        MeterDefinition {
            id: MeterDefinitionId(1),
            display_name: SmolStr::new("test"),
            min_value: 0.0,
            max_value: 100.0,
            default_value: 50.0,
            drain_fill: MeterDrainFill {
                drain_rate: 0.0,
                fill_rate: 0.0,
                drain_when_empty: false,
                fill_when_full: false,
            },
            thresholds: Vec::new(),
            save_on_persist: true,
            replicate: false,
        }
    }

    #[test]
    fn test_meter_construct_defaults() {
        let definition = basic_definition();
        let meter = Meter::from_definition(&definition);
        assert_eq!(meter.current_value, 50.0);
        assert_eq!(definition.min_value, 0.0);
        assert_eq!(definition.max_value, 100.0);
    }

    #[test]
    fn test_meter_clamp_min_max() {
        let definition = basic_definition();
        let mut meter = Meter::from_definition(&definition);
        meter.apply_delta(-200.0, &definition);
        assert_eq!(meter.current_value, 0.0);
        meter.apply_delta(500.0, &definition);
        assert_eq!(meter.current_value, 100.0);
    }

    #[test]
    fn test_meter_drain_rate() {
        let mut definition = basic_definition();
        definition.default_value = 50.0;
        definition.drain_fill.drain_rate = 5.0;
        let mut meter = Meter::from_definition(&definition);
        meter.tick_drain_fill(2.0, &definition);
        assert_eq!(meter.current_value, 40.0);
    }

    #[test]
    fn test_meter_fill_rate() {
        let mut definition = basic_definition();
        definition.default_value = 20.0;
        definition.drain_fill.drain_rate = 0.0;
        definition.drain_fill.fill_rate = 10.0;
        let mut meter = Meter::from_definition(&definition);
        meter.tick_drain_fill(0.5, &definition);
        assert_eq!(meter.current_value, 25.0);
    }

    #[test]
    fn test_threshold_rising_fires_once() {
        let mut definition = basic_definition();
        definition.thresholds.push(MeterThreshold {
            value: 80.0,
            direction: ThresholdDirection::Rising,
            action: ThresholdAction::FireEvent,
            effect_ref: None,
            tag_grant: None,
        });
        let mut meter = Meter::from_definition(&definition);
        meter.set_value(70.0, &definition);
        let _ = meter.poll_thresholds(&definition);
        meter.set_value(90.0, &definition);
        let events = meter.poll_thresholds(&definition);
        assert_eq!(events.len(), 1);
        assert_eq!(events[0].direction, ThresholdDirection::Rising);
        assert_eq!(events[0].threshold_value, 80.0);
    }

    #[test]
    fn test_threshold_falling_fires() {
        let mut definition = basic_definition();
        definition.default_value = 30.0;
        definition.thresholds.push(MeterThreshold {
            value: 20.0,
            direction: ThresholdDirection::Falling,
            action: ThresholdAction::FireEvent,
            effect_ref: None,
            tag_grant: None,
        });
        let mut meter = Meter::from_definition(&definition);
        meter.set_value(10.0, &definition);
        let events = meter.poll_thresholds(&definition);
        assert_eq!(events.len(), 1);
        assert_eq!(events[0].direction, ThresholdDirection::Falling);
    }

    #[test]
    fn test_threshold_either_direction() {
        let mut definition = basic_definition();
        definition.default_value = 40.0;
        definition.thresholds.push(MeterThreshold {
            value: 50.0,
            direction: ThresholdDirection::Either,
            action: ThresholdAction::FireEvent,
            effect_ref: None,
            tag_grant: None,
        });
        let mut meter = Meter::from_definition(&definition);
        meter.set_value(60.0, &definition);
        let first = meter.poll_thresholds(&definition);
        assert_eq!(first.len(), 1);
        meter.set_value(30.0, &definition);
        let second = meter.poll_thresholds(&definition);
        assert_eq!(second.len(), 1);
    }

    #[test]
    fn test_threshold_no_double_fire() {
        let mut definition = basic_definition();
        definition.default_value = 40.0;
        definition.thresholds.push(MeterThreshold {
            value: 50.0,
            direction: ThresholdDirection::Rising,
            action: ThresholdAction::FireEvent,
            effect_ref: None,
            tag_grant: None,
        });
        let mut meter = Meter::from_definition(&definition);
        meter.set_value(60.0, &definition);
        let first = meter.poll_thresholds(&definition);
        assert_eq!(first.len(), 1);
        meter.set_value(70.0, &definition);
        let second = meter.poll_thresholds(&definition);
        assert_eq!(second.len(), 0);
        meter.set_value(80.0, &definition);
        let third = meter.poll_thresholds(&definition);
        assert_eq!(third.len(), 0);
    }

    #[test]
    fn test_hp_mp_stamina_meters_bound() {
        let mut hp_def = basic_definition();
        hp_def.id = MeterDefinitionId(1);
        hp_def.min_value = 0.0;
        hp_def.max_value = 100.0;
        hp_def.default_value = 100.0;

        let mut mp_def = basic_definition();
        mp_def.id = MeterDefinitionId(2);
        mp_def.min_value = 0.0;
        mp_def.max_value = 50.0;
        mp_def.default_value = 50.0;

        let mut st_def = basic_definition();
        st_def.id = MeterDefinitionId(3);
        st_def.min_value = 0.0;
        st_def.max_value = 80.0;
        st_def.default_value = 80.0;

        let mut set = MeterSet::default();
        set.insert(Meter::from_definition(&hp_def));
        set.insert(Meter::from_definition(&mp_def));
        set.insert(Meter::from_definition(&st_def));

        {
            let hp = set.get_mut(MeterDefinitionId(1)).expect("hp");
            hp.apply_delta(-20.0, &hp_def);
        }

        assert_eq!(
            set.get(MeterDefinitionId(1)).expect("hp").current_value,
            80.0
        );
        assert_eq!(
            set.get(MeterDefinitionId(2)).expect("mp").current_value,
            50.0
        );
        assert_eq!(
            set.get(MeterDefinitionId(3)).expect("st").current_value,
            80.0
        );
    }

    #[test]
    fn test_faction_rep_tier_thresholds() {
        let mut definition = basic_definition();
        definition.min_value = -100.0;
        definition.max_value = 100.0;
        definition.default_value = 0.0;
        for value in [-50.0f32, 0.0, 50.0, 100.0] {
            definition.thresholds.push(MeterThreshold {
                value,
                direction: ThresholdDirection::Rising,
                action: ThresholdAction::FireEvent,
                effect_ref: None,
                tag_grant: None,
            });
        }

        let mut meter = Meter::from_definition(&definition);
        meter.set_value(10.0, &definition);
        let _ = meter.poll_thresholds(&definition);
        meter.set_value(60.0, &definition);
        let mid = meter.poll_thresholds(&definition);
        assert_eq!(mid.len(), 1);
        assert_eq!(mid[0].threshold_value, 50.0);
        meter.set_value(110.0, &definition);
        let end = meter.poll_thresholds(&definition);
        assert_eq!(end.len(), 1);
        assert_eq!(end[0].threshold_value, 100.0);
        assert_eq!(meter.current_value, 100.0);
    }
}
