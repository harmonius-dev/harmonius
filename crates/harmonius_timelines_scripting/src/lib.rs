//! Timelines ↔ scripting integration boundary (IR-4.9.x).
//!
//! Deterministic tick-based contracts between timeline playback and graph
//! execution. Synchronous only; hidden stack switching is forbidden by design.

#![deny(clippy::all)]
#![allow(clippy::module_name_repetitions, clippy::new_without_default, clippy::must_use_candidate)]

pub mod advance;
pub mod command;
pub mod event;
pub mod graph;
pub mod ids;
pub mod log;
pub mod playback;
pub mod timeline;
pub mod variable;
pub mod world;

pub use advance::{tick_simulation, TimelineAdvanceSystem, TimelineSeekEvent};
pub use command::{ChoiceMadeEvent, CommandBuffer};
pub use event::{TimelineEvent, TimelineEventKind};
pub use graph::{
    branch_tick_or_default, GraphBranchConfig, GraphExecCtx, GraphExecutionSystem, GraphInstance,
    GraphProgram, GraphStateMachine, RuntimeError, StepOutcome, WaitCondition,
};
pub use ids::{
    Entity, EventId, GraphId, KeyframeId, SlotId, TickCount, TrackId, DEFAULT_WAIT_TIMEOUT,
    GRAPH_STEP_NOT_STARTED,
};
pub use playback::PlaybackState;
pub use timeline::{Keyframe, MultiTrackTimeline, Track, TrackValue};
pub use variable::{BindError, ScriptTypeId, TypedSlot, VariableStore};
pub use world::World;
