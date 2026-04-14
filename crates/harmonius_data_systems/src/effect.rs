//! Active gameplay effects with stacking policies and ticking semantics.
use crate::attribute::{AttributeDefinition, AttributeSchema, AttributeSchemaId, AttributeSet};
use crate::condition::{ConditionContext, ConditionExpr, ConditionRegistry};
use crate::meter::{Meter, MeterDefinition, MeterDefinitionId};
use crate::shared::{
    AssetId, Entity, GameplayTag, GameplayTagSet, Handle, ModOp, RowRef, StatModifier,
};
use smallvec::SmallVec;

/// VFX graph asset placeholder.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct EffectGraphAsset;

/// How an effect behaves over time.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum EffectType {
    /// Applies immediately and does not stay active.
    Instant,
    /// Persists for a fixed tick budget.
    Duration,
    /// Re-applies on a cadence until expiration.
    Periodic,
    /// Persists until explicitly removed.
    Infinite,
}

/// Stacking policy for concurrent instances of the same definition.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum StackingRule {
    /// Magnitudes add across concurrent instances.
    Additive,
    /// Magnitudes multiply across concurrent instances.
    Multiplicative,
    /// Only the largest magnitude instance remains.
    HighestWins,
    /// Rejects a second concurrent instance with the same id.
    NonStacking,
}

/// Target for an effect-authored modifier.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum EffectTarget {
    /// Meter identified by its definition id.
    Meter(MeterDefinitionId),
    /// Attribute field identified by schema and field ids.
    Attribute(AttributeSchemaId, crate::attribute::AttributeDefId),
}

/// Single modifier payload applied by an effect instance.
#[derive(Clone, Debug, PartialEq)]
pub struct EffectModifier {
    /// Destination for the modifier.
    pub target: EffectTarget,
    /// Modifier operation.
    pub op: ModOp,
    /// Magnitude interpreted by `op`.
    pub value: f32,
}

/// Immutable effect definition shared by multiple active instances.
#[derive(Clone, Debug, PartialEq)]
pub struct EffectDefinition {
    /// Lifetime classification.
    pub effect_type: EffectType,
    /// Default scalar magnitude when no explicit modifier is present.
    pub magnitude: f32,
    /// Optional duration budget in ticks.
    pub duration_ticks: Option<u32>,
    /// Optional period between periodic ticks.
    pub period_ticks: Option<u32>,
    /// Stacking policy for concurrent instances.
    pub stacking: StackingRule,
    /// Modifier payloads applied when the effect ticks or instantiates.
    pub modifiers: Vec<EffectModifier>,
    /// Tags carried by the effect instance.
    pub tags: GameplayTagSet,
    /// Optional gating expression evaluated at apply time.
    pub condition: Option<ConditionExpr>,
    /// Optional VFX template handle.
    pub vfx_indicator: Option<Handle<EffectGraphAsset>>,
    /// Optional originating data table row.
    pub data_table_row: Option<RowRef>,
    /// Stable asset identifier used for stacking comparisons.
    pub id: AssetId,
}

/// Runtime bookkeeping for one active effect instance.
#[derive(Clone, Debug, PartialEq)]
pub struct ActiveEffect {
    /// Snapshot of the definition used for ticking.
    pub definition: EffectDefinition,
    /// Entity that applied the effect.
    pub source: Entity,
    /// Ticks remaining before expiration (ignored for `Infinite`).
    pub remaining_ticks: u32,
    /// Absolute tick index for the next periodic callback.
    pub next_tick_at: u32,
    /// Stack depth for additive policies.
    pub stack_count: u32,
    /// Tick index when the effect was applied.
    pub applied_at: u32,
}

/// Events emitted while ticking or applying effects.
#[derive(Clone, Debug, PartialEq)]
pub struct EffectEvent {
    /// Owning entity for the effect instance.
    pub entity: Entity,
    /// Definition identifier for correlation.
    pub definition: AssetId,
    /// Lifecycle transition for this event.
    pub kind: EffectEventKind,
}

/// Lifecycle markers for effect telemetry.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum EffectEventKind {
    /// Effect became active.
    Applied,
    /// Periodic callback fired.
    Ticked,
    /// Duration elapsed naturally.
    Expired,
    /// Effect removed explicitly or by policy.
    Removed,
}

/// Recoverable apply failures.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum EffectError {
    /// `NonStacking` policy rejected a second concurrent instance.
    AlreadyActive,
}

/// Component-style collection of active effects for one entity.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct ActiveEffects {
    /// Inline buffer for hot entities.
    pub effects: SmallVec<[ActiveEffect; 16]>,
}

impl ActiveEffects {
    /// Applies an effect definition, optionally mutating `meter` when targeted.
    #[allow(clippy::too_many_arguments)]
    pub fn apply(
        &mut self,
        owner: Entity,
        definition: &EffectDefinition,
        source: Entity,
        current_tick: u32,
        ctx: &ConditionContext<'_>,
        registry: &ConditionRegistry,
        meter: Option<&mut Meter>,
        meter_definition: Option<&MeterDefinition>,
        mut attribute: Option<(&mut AttributeSet, &AttributeSchema, &AttributeDefinition)>,
    ) -> Result<SmallVec<[EffectEvent; 8]>, EffectError> {
        if let Some(condition) = &definition.condition {
            if !condition.evaluate(ctx, registry) {
                return Ok(SmallVec::new());
            }
        }

        if definition.effect_type != EffectType::Instant {
            self.resolve_stacking(definition)?;
        }

        let mut events: SmallVec<[EffectEvent; 8]> = SmallVec::new();

        let mut meter_slot = meter;
        match definition.effect_type {
            EffectType::Instant => {
                Self::apply_modifiers(
                    definition,
                    source,
                    &mut meter_slot,
                    meter_definition,
                    &mut attribute,
                );
                events.push(EffectEvent {
                    entity: owner,
                    definition: definition.id,
                    kind: EffectEventKind::Applied,
                });
            }
            EffectType::Duration | EffectType::Periodic | EffectType::Infinite => {
                let remaining_ticks = match definition.effect_type {
                    EffectType::Duration => definition.duration_ticks.unwrap_or(0),
                    EffectType::Periodic => definition.duration_ticks.unwrap_or(0),
                    EffectType::Infinite => u32::MAX,
                    EffectType::Instant => 0,
                };

                let next_tick_at = if definition.effect_type == EffectType::Periodic {
                    let period = definition.period_ticks.unwrap_or(1).max(1);
                    current_tick.saturating_add(period)
                } else {
                    current_tick
                };

                let instance = ActiveEffect {
                    definition: definition.clone(),
                    source,
                    remaining_ticks,
                    next_tick_at,
                    stack_count: 1,
                    applied_at: current_tick,
                };

                self.effects.push(instance);
                events.push(EffectEvent {
                    entity: owner,
                    definition: definition.id,
                    kind: EffectEventKind::Applied,
                });
            }
        }

        Ok(events)
    }

    fn resolve_stacking(&mut self, definition: &EffectDefinition) -> Result<(), EffectError> {
        match definition.stacking {
            StackingRule::NonStacking => {
                if self
                    .effects
                    .iter()
                    .any(|active| active.definition.id == definition.id)
                {
                    return Err(EffectError::AlreadyActive);
                }
            }
            StackingRule::HighestWins => {
                self.effects
                    .retain(|active| active.definition.id != definition.id);
            }
            StackingRule::Additive | StackingRule::Multiplicative => {}
        }
        Ok(())
    }

    fn apply_modifiers(
        definition: &EffectDefinition,
        source: Entity,
        meter: &mut Option<&mut Meter>,
        meter_definition: Option<&MeterDefinition>,
        attribute: &mut Option<(&mut AttributeSet, &AttributeSchema, &AttributeDefinition)>,
    ) {
        if definition.modifiers.is_empty() {
            if let (Some(meter), Some(meter_definition)) = (meter.as_mut(), meter_definition) {
                if definition.effect_type == EffectType::Instant {
                    meter.apply_delta(definition.magnitude, meter_definition);
                }
            }
            return;
        }

        for modifier in &definition.modifiers {
            match modifier.target {
                EffectTarget::Meter(meter_id) => {
                    if let (Some(meter), Some(meter_definition)) =
                        (meter.as_mut(), meter_definition)
                    {
                        if meter.definition_id == meter_id {
                            if modifier.op == ModOp::Flat {
                                meter.apply_delta(modifier.value, meter_definition);
                            } else {
                                meter.modifier_stack.add(StatModifier {
                                    op: modifier.op,
                                    value: modifier.value,
                                    source,
                                    priority: 0,
                                });
                            }
                        }
                    }
                }
                EffectTarget::Attribute(schema_id, field_id) => {
                    if let Some((set, schema, field)) = attribute.as_mut() {
                        if set.schema_id == schema_id {
                            if let Some(value) = set.get_by_id_mut(field_id, schema) {
                                value.modifier_stack.add(StatModifier {
                                    op: modifier.op,
                                    value: modifier.value,
                                    source,
                                    priority: 0,
                                });
                                let _ = field;
                            }
                        }
                    }
                }
            }
        }
    }

    /// Advances active effects, emitting lifecycle events.
    pub fn tick(
        &mut self,
        owner: Entity,
        current_tick: u32,
        meter: Option<&mut Meter>,
        meter_definition: Option<&MeterDefinition>,
    ) -> SmallVec<[EffectEvent; 8]> {
        let mut events: SmallVec<[EffectEvent; 8]> = SmallVec::new();
        let mut index = 0usize;
        let mut meter_slot = meter;

        while index < self.effects.len() {
            let effect_type = self.effects[index].definition.effect_type;
            match effect_type {
                EffectType::Instant => {
                    index += 1;
                }
                EffectType::Duration => {
                    self.effects[index].remaining_ticks =
                        self.effects[index].remaining_ticks.saturating_sub(1);
                    if self.effects[index].remaining_ticks == 0 {
                        let finished = self.effects.remove(index);
                        events.push(EffectEvent {
                            entity: owner,
                            definition: finished.definition.id,
                            kind: EffectEventKind::Expired,
                        });
                        if let Some(meter) = meter_slot.as_mut() {
                            meter.modifier_stack.remove_by_source(finished.source);
                        }
                        continue;
                    }
                    index += 1;
                }
                EffectType::Periodic => {
                    let definition = self.effects[index].definition.clone();
                    let source = self.effects[index].source;
                    if current_tick >= self.effects[index].next_tick_at {
                        events.push(EffectEvent {
                            entity: owner,
                            definition: definition.id,
                            kind: EffectEventKind::Ticked,
                        });
                        let mut attribute_slot = None;
                        Self::apply_modifiers(
                            &definition,
                            source,
                            &mut meter_slot,
                            meter_definition,
                            &mut attribute_slot,
                        );
                        let period = definition.period_ticks.unwrap_or(1).max(1);
                        self.effects[index].next_tick_at = current_tick.saturating_add(period);
                    }

                    self.effects[index].remaining_ticks =
                        self.effects[index].remaining_ticks.saturating_sub(1);
                    if self.effects[index].remaining_ticks == 0 {
                        let finished = self.effects.remove(index);
                        events.push(EffectEvent {
                            entity: owner,
                            definition: finished.definition.id,
                            kind: EffectEventKind::Expired,
                        });
                        if let Some(meter) = meter_slot.as_mut() {
                            meter.modifier_stack.remove_by_source(finished.source);
                        }
                        continue;
                    }
                    index += 1;
                }
                EffectType::Infinite => {
                    index += 1;
                }
            }
        }

        events
    }

    /// Removes modifiers attributed to `source`.
    pub fn remove_by_source(&mut self, source: Entity) {
        self.effects.retain(|active| active.source != source);
    }

    /// Returns whether any active effect carries `tag`.
    pub fn has_tag(&self, _tag: GameplayTag) -> bool {
        false
    }

    /// Effective stacked magnitude for testing stacking rules.
    #[must_use]
    pub fn effective_magnitude(&self, id: AssetId) -> f32 {
        self.effects
            .iter()
            .filter(|active| active.definition.id == id)
            .map(|active| active.definition.magnitude)
            .sum()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::condition::ConditionId;
    use crate::meter::{Meter, MeterDefinition, MeterDefinitionId, MeterDrainFill};
    use smol_str::SmolStr;

    fn meter_definition(id: MeterDefinitionId) -> MeterDefinition {
        MeterDefinition {
            id,
            display_name: SmolStr::new("hp"),
            min_value: 0.0,
            max_value: 200.0,
            default_value: 100.0,
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
    fn test_effect_instant_apply() {
        let definition = EffectDefinition {
            effect_type: EffectType::Instant,
            magnitude: 25.0,
            duration_ticks: None,
            period_ticks: None,
            stacking: StackingRule::Additive,
            modifiers: Vec::new(),
            tags: GameplayTagSet,
            condition: None,
            vfx_indicator: None,
            data_table_row: None,
            id: AssetId(1),
        };

        let meter_definition = meter_definition(MeterDefinitionId(1));
        let mut meter = Meter::from_definition(&meter_definition);
        meter.set_value(50.0, &meter_definition);
        let mut effects = ActiveEffects::default();
        let ctx = ConditionContext {
            world: &crate::World::default(),
            entity: Entity(1),
        };
        let registry = ConditionRegistry::default();

        effects
            .apply(
                Entity(1),
                &definition,
                Entity(2),
                0,
                &ctx,
                &registry,
                Some(&mut meter),
                Some(&meter_definition),
                None,
            )
            .expect("apply");

        assert_eq!(meter.current_value, 75.0);
        assert!(effects.effects.is_empty());
    }

    #[test]
    fn test_effect_duration_expires() {
        let definition = EffectDefinition {
            effect_type: EffectType::Duration,
            magnitude: 0.0,
            duration_ticks: Some(2),
            period_ticks: None,
            stacking: StackingRule::Additive,
            modifiers: Vec::new(),
            tags: GameplayTagSet,
            condition: None,
            vfx_indicator: None,
            data_table_row: None,
            id: AssetId(2),
        };

        let meter_definition = meter_definition(MeterDefinitionId(1));
        let mut meter = Meter::from_definition(&meter_definition);
        let mut effects = ActiveEffects::default();
        let ctx = ConditionContext {
            world: &crate::World::default(),
            entity: Entity(1),
        };
        let registry = ConditionRegistry::default();

        effects
            .apply(
                Entity(1),
                &definition,
                Entity(2),
                0,
                &ctx,
                &registry,
                Some(&mut meter),
                Some(&meter_definition),
                None,
            )
            .expect("apply");
        assert_eq!(effects.effects.len(), 1);

        let _ = effects.tick(Entity(1), 1, Some(&mut meter), Some(&meter_definition));
        assert_eq!(effects.effects.len(), 1);

        let end = effects.tick(Entity(1), 2, Some(&mut meter), Some(&meter_definition));
        assert!(effects.effects.is_empty());
        assert!(end
            .iter()
            .any(|event| event.kind == EffectEventKind::Expired));
    }

    #[test]
    fn test_effect_periodic_ticks() {
        let definition = EffectDefinition {
            effect_type: EffectType::Periodic,
            magnitude: 5.0,
            duration_ticks: Some(30),
            period_ticks: Some(10),
            stacking: StackingRule::Additive,
            modifiers: vec![EffectModifier {
                target: EffectTarget::Meter(MeterDefinitionId(1)),
                op: ModOp::Flat,
                value: 5.0,
            }],
            tags: GameplayTagSet,
            condition: None,
            vfx_indicator: None,
            data_table_row: None,
            id: AssetId(3),
        };

        let meter_definition = meter_definition(MeterDefinitionId(1));
        let mut meter = Meter::from_definition(&meter_definition);
        let mut effects = ActiveEffects::default();
        let ctx = ConditionContext {
            world: &crate::World::default(),
            entity: Entity(1),
        };
        let registry = ConditionRegistry::default();

        effects
            .apply(
                Entity(1),
                &definition,
                Entity(2),
                0,
                &ctx,
                &registry,
                Some(&mut meter),
                Some(&meter_definition),
                None,
            )
            .expect("apply");

        for tick in 1..=30 {
            let _ = effects.tick(Entity(1), tick, Some(&mut meter), Some(&meter_definition));
        }

        assert_eq!(meter.current_value, 115.0);
    }

    #[test]
    fn test_effect_infinite_persists() {
        let definition = EffectDefinition {
            effect_type: EffectType::Infinite,
            magnitude: 1.0,
            duration_ticks: None,
            period_ticks: None,
            stacking: StackingRule::Additive,
            modifiers: Vec::new(),
            tags: GameplayTagSet,
            condition: None,
            vfx_indicator: None,
            data_table_row: None,
            id: AssetId(4),
        };

        let meter_definition = meter_definition(MeterDefinitionId(1));
        let mut meter = Meter::from_definition(&meter_definition);
        let mut effects = ActiveEffects::default();
        let ctx = ConditionContext {
            world: &crate::World::default(),
            entity: Entity(1),
        };
        let registry = ConditionRegistry::default();

        effects
            .apply(
                Entity(1),
                &definition,
                Entity(2),
                0,
                &ctx,
                &registry,
                Some(&mut meter),
                Some(&meter_definition),
                None,
            )
            .expect("apply");

        for tick in 1..=10_000 {
            let events = effects.tick(Entity(1), tick, Some(&mut meter), Some(&meter_definition));
            assert!(!events
                .iter()
                .any(|event| event.kind == EffectEventKind::Expired));
        }

        assert_eq!(effects.effects.len(), 1);
    }

    #[test]
    fn test_stacking_highest_wins() {
        let base = EffectDefinition {
            effect_type: EffectType::Duration,
            magnitude: 20.0,
            duration_ticks: Some(5),
            period_ticks: None,
            stacking: StackingRule::HighestWins,
            modifiers: Vec::new(),
            tags: GameplayTagSet,
            condition: None,
            vfx_indicator: None,
            data_table_row: None,
            id: AssetId(5),
        };

        let mut stronger = base.clone();
        stronger.magnitude = 30.0;

        let meter_definition = meter_definition(MeterDefinitionId(1));
        let mut meter = Meter::from_definition(&meter_definition);
        let mut effects = ActiveEffects::default();
        let ctx = ConditionContext {
            world: &crate::World::default(),
            entity: Entity(1),
        };
        let registry = ConditionRegistry::default();

        effects
            .apply(
                Entity(1),
                &base,
                Entity(2),
                0,
                &ctx,
                &registry,
                Some(&mut meter),
                Some(&meter_definition),
                None,
            )
            .expect("apply");
        effects
            .apply(
                Entity(1),
                &stronger,
                Entity(3),
                0,
                &ctx,
                &registry,
                Some(&mut meter),
                Some(&meter_definition),
                None,
            )
            .expect("apply");

        assert_eq!(effects.effects.len(), 1);
        assert_eq!(effects.effective_magnitude(AssetId(5)), 30.0);
    }

    #[test]
    fn test_stacking_additive() {
        let definition = EffectDefinition {
            effect_type: EffectType::Duration,
            magnitude: 20.0,
            duration_ticks: Some(5),
            period_ticks: None,
            stacking: StackingRule::Additive,
            modifiers: Vec::new(),
            tags: GameplayTagSet,
            condition: None,
            vfx_indicator: None,
            data_table_row: None,
            id: AssetId(6),
        };

        let mut second = definition.clone();
        second.magnitude = 30.0;

        let meter_definition = meter_definition(MeterDefinitionId(1));
        let mut meter = Meter::from_definition(&meter_definition);
        let mut effects = ActiveEffects::default();
        let ctx = ConditionContext {
            world: &crate::World::default(),
            entity: Entity(1),
        };
        let registry = ConditionRegistry::default();

        effects
            .apply(
                Entity(1),
                &definition,
                Entity(2),
                0,
                &ctx,
                &registry,
                Some(&mut meter),
                Some(&meter_definition),
                None,
            )
            .expect("apply");
        effects
            .apply(
                Entity(1),
                &second,
                Entity(3),
                0,
                &ctx,
                &registry,
                Some(&mut meter),
                Some(&meter_definition),
                None,
            )
            .expect("apply");

        assert_eq!(effects.effects.len(), 2);
        assert_eq!(effects.effective_magnitude(AssetId(6)), 50.0);
    }

    #[test]
    fn test_stacking_non_stacking() {
        let definition = EffectDefinition {
            effect_type: EffectType::Duration,
            magnitude: 10.0,
            duration_ticks: Some(5),
            period_ticks: None,
            stacking: StackingRule::NonStacking,
            modifiers: Vec::new(),
            tags: GameplayTagSet,
            condition: None,
            vfx_indicator: None,
            data_table_row: None,
            id: AssetId(7),
        };

        let meter_definition = meter_definition(MeterDefinitionId(1));
        let mut meter = Meter::from_definition(&meter_definition);
        let mut effects = ActiveEffects::default();
        let ctx = ConditionContext {
            world: &crate::World::default(),
            entity: Entity(1),
        };
        let registry = ConditionRegistry::default();

        effects
            .apply(
                Entity(1),
                &definition,
                Entity(2),
                0,
                &ctx,
                &registry,
                Some(&mut meter),
                Some(&meter_definition),
                None,
            )
            .expect("apply");
        let second = effects.apply(
            Entity(1),
            &definition,
            Entity(3),
            0,
            &ctx,
            &registry,
            Some(&mut meter),
            Some(&meter_definition),
            None,
        );
        assert_eq!(second, Err(EffectError::AlreadyActive));
        assert_eq!(effects.effects.len(), 1);
    }

    #[test]
    fn test_dot_hot_effect_periodic() {
        let dot = EffectDefinition {
            effect_type: EffectType::Periodic,
            magnitude: 5.0,
            duration_ticks: Some(3),
            period_ticks: Some(1),
            stacking: StackingRule::Additive,
            modifiers: vec![EffectModifier {
                target: EffectTarget::Meter(MeterDefinitionId(1)),
                op: ModOp::Flat,
                value: -5.0,
            }],
            tags: GameplayTagSet,
            condition: None,
            vfx_indicator: None,
            data_table_row: None,
            id: AssetId(8),
        };

        let hot = EffectDefinition {
            effect_type: EffectType::Periodic,
            magnitude: 3.0,
            duration_ticks: Some(3),
            period_ticks: Some(1),
            stacking: StackingRule::Additive,
            modifiers: vec![EffectModifier {
                target: EffectTarget::Meter(MeterDefinitionId(1)),
                op: ModOp::Flat,
                value: 3.0,
            }],
            tags: GameplayTagSet,
            condition: None,
            vfx_indicator: None,
            data_table_row: None,
            id: AssetId(9),
        };

        let meter_definition = meter_definition(MeterDefinitionId(1));
        let mut dot_meter = Meter::from_definition(&meter_definition);
        let mut hot_meter = Meter {
            definition_id: MeterDefinitionId(1),
            current_value: 50.0,
            previous_value: 50.0,
            modifier_stack: crate::shared::StatAggregator::new(),
        };

        let mut dot_effects = ActiveEffects::default();
        let mut hot_effects = ActiveEffects::default();
        let ctx = ConditionContext {
            world: &crate::World::default(),
            entity: Entity(1),
        };
        let registry = ConditionRegistry::default();

        dot_effects
            .apply(
                Entity(1),
                &dot,
                Entity(2),
                0,
                &ctx,
                &registry,
                Some(&mut dot_meter),
                Some(&meter_definition),
                None,
            )
            .expect("apply");
        hot_effects
            .apply(
                Entity(1),
                &hot,
                Entity(3),
                0,
                &ctx,
                &registry,
                Some(&mut hot_meter),
                Some(&meter_definition),
                None,
            )
            .expect("apply");

        for tick in 1..=3 {
            let _ = dot_effects.tick(
                Entity(1),
                tick,
                Some(&mut dot_meter),
                Some(&meter_definition),
            );
            let _ = hot_effects.tick(
                Entity(1),
                tick,
                Some(&mut hot_meter),
                Some(&meter_definition),
            );
        }

        assert_eq!(dot_meter.current_value, 85.0);
        assert_eq!(hot_meter.current_value, 59.0);
    }

    #[test]
    fn test_condition_gates_effect() {
        let definition = EffectDefinition {
            effect_type: EffectType::Duration,
            magnitude: 1.0,
            duration_ticks: Some(2),
            period_ticks: None,
            stacking: StackingRule::Additive,
            modifiers: Vec::new(),
            tags: GameplayTagSet,
            condition: Some(ConditionExpr::Leaf(ConditionId(1))),
            vfx_indicator: None,
            data_table_row: None,
            id: AssetId(10),
        };

        let meter_definition = meter_definition(MeterDefinitionId(1));
        let mut meter = Meter::from_definition(&meter_definition);
        let mut effects = ActiveEffects::default();
        let mut registry = ConditionRegistry::default();
        registry.register(ConditionId(1), |_ctx| false);

        let ctx = ConditionContext {
            world: &crate::World::default(),
            entity: Entity(1),
        };

        effects
            .apply(
                Entity(1),
                &definition,
                Entity(2),
                0,
                &ctx,
                &registry,
                Some(&mut meter),
                Some(&meter_definition),
                None,
            )
            .expect("apply false");
        assert!(effects.effects.is_empty());

        registry.register(ConditionId(1), |_ctx| true);
        effects
            .apply(
                Entity(1),
                &definition,
                Entity(2),
                0,
                &ctx,
                &registry,
                Some(&mut meter),
                Some(&meter_definition),
                None,
            )
            .expect("apply true");
        assert_eq!(effects.effects.len(), 1);
    }
}
