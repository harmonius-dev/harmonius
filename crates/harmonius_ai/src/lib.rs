//! AI behavior systems: behavior trees, utility curves, and GOAP planning.
//!
//! Implements the unit-test scope of [PLAN-ai-behavior] against
//! `docs/design/ai/behavior.md` and `behavior-test-cases.md`.
//!
//! [PLAN-ai-behavior]: https://github.com/cjhowe-us/harmonius/blob/main/docs/plans/ai/behavior.md

#![deny(clippy::all)]
#![deny(unsafe_code)]
#![allow(missing_docs)]

pub mod blackboard;
pub mod bt;
pub mod debug_trace;
pub mod goap;
pub mod subtree;
pub mod utility;

pub use blackboard::{
    BlackboardKey, BlackboardValue, BlackboardValueType, GroupBlackboardStore, GroupId,
    ObserverRegistry, ScopedBlackboard,
};
pub use bt::{
    AbortMode, BehaviorTreeAsset, BlackboardSchema, BtNode, BtTickContext, BtTickState,
    CompositeData, DecoratorData, DecoratorKind, DecoratorRuntime, LeafId, NodeId, NodeStatus,
    ParallelData, ParallelPolicy,
};
pub use debug_trace::TraceLog;
pub use goap::{
    hash_world, plan_dijkstra, GoapAction, GoapActionRegistry, GoapAgent, GoapGoal, Plan,
    PlanCache, PlanCacheKey, WorldState, WorldStateDelta,
};
pub use subtree::{validate_subtree_graph, SubtreeError};
pub use utility::{
    compensate_product, dual_axis_pick, evaluate_builtins, hysteresis_should_switch,
    score_action_with_inputs, score_custom, select_highest, weighted_random_index, CategoryDef,
    Consideration, ContextSet, CustomConsideration, InputAxis, ResponseCurve, ScoredAction,
    SelectionStrategy, UtilityAction, UtilityActionSet, UtilityAgentState,
};
