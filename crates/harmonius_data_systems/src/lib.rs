//! Meters, attribute sets, active effects, and condition expressions.
#![deny(clippy::all)]

mod attribute;
mod condition;
mod effect;
mod meter;
mod shared;

pub use attribute::{
    AttributeDefinition, AttributeSchema, AttributeSchemaId, AttributeSet, AttributeValue,
    AttributeValueType,
};
pub use condition::{
    ConditionCheckFn, ConditionContext, ConditionExpr, ConditionId, ConditionRegistry,
};
pub use effect::{
    ActiveEffect, ActiveEffects, EffectDefinition, EffectError, EffectEvent, EffectEventKind,
    EffectModifier, EffectTarget, EffectType, StackingRule,
};
pub use meter::{
    Meter, MeterDefinition, MeterDefinitionId, MeterDrainFill, MeterSet, MeterThreshold,
    ThresholdAction, ThresholdCrossing, ThresholdDirection,
};
pub use shared::{
    AssetId, Entity, GameplayTag, GameplayTagSet, ModOp, RowRef, StatAggregator, StatModifier,
    World,
};
