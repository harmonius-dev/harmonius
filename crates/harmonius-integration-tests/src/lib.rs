//! Scripting ↔ ECS integration surface (reference implementation for design IR-2.8.*).
//!
//! Self-contained until core engine crates land. See
//! `docs/design/integration/scripting-ecs.md`.

#![deny(clippy::all)]
#![forbid(unsafe_code)]

mod access;
mod arena;
mod commands;
mod context;
mod events;
mod graph;
mod scheduler;
mod variables;
mod world;

pub use access::{AccessSet, GraphAccessDescriptor};
pub use arena::ThreadArena;
pub use commands::{CommandBuffer, CommandSegment, EntityCommands, ParallelCommandWriter};
pub use context::ExecutionContext;
pub use events::{DebugBridge, DebugMsg, Event, EventWriter};
pub use graph::{
    graph_instance_bucket_count, max_concurrency_this_frame, report_command_buffer_full, AssetId,
    DiagnosticEvent, FnPtrTable,
    GraphError, GraphExecutionConfig, GraphExecutionSystem, GraphFn, GraphInstance,
    GraphInstanceState, GraphProgram, GraphRegistrationError, GraphSchedulePhase, HotReloadError,
    ResumeVariant, StepResult,
};
pub use scheduler::GraphScheduler;
pub use variables::{
    read_armor_var, read_health_var, read_pose_var, write_pose_var, write_speed_var, AnimState,
    Armor, Health, Speed, Transform,
};
pub use world::{Component, Entity, World};
