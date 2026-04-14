//! Graph execution, waits, and program dispatch (IR-4.9.1 / IR-4.9.3 / IR-4.9.7).

use std::collections::HashMap;
use std::sync::mpsc::TrySendError;
use std::sync::Arc;

use crate::advance::TimelineSeekEvent;
use crate::command::ChoiceMadeEvent;
use crate::event::{TimelineEvent, TimelineEventKind};
use crate::ids::{Entity, EventId, GraphId, TickCount, DEFAULT_WAIT_TIMEOUT, GRAPH_STEP_NOT_STARTED};
use crate::log;
use crate::variable::VariableStore;
use crate::world::World;

/// Runtime failures surfaced as `StepOutcome::Error` (FM-3).
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum RuntimeError {
    WaitTimeout,
}

/// Result of stepping a graph continuation.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum StepOutcome {
    Ok,
    Error(RuntimeError),
}

/// Synchronous wait condition evaluated every tick (no cooperative stacks).
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WaitCondition {
    NextTick,
    Ticks {
        remaining: u32,
    },
    Deadline {
        resume_at: TickCount,
    },
    TimelineComplete {
        timeline_entity: Entity,
        armed_at: TickCount,
        timeout: Option<TickCount>,
    },
    Event {
        event_id: EventId,
    },
}

fn effective_timeline_timeout(timeout: Option<TickCount>) -> TickCount {
    timeout.unwrap_or(DEFAULT_WAIT_TIMEOUT)
}

/// Cutscene branch ticks for deterministic tests (IR-4.9.5).
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct GraphBranchConfig {
    pub timeline: Entity,
    pub tick_a: TickCount,
    pub tick_b: TickCount,
}

/// Compiled graph program (middleman stand-in for tests).
#[derive(Clone, Debug)]
pub struct GraphProgram {
    pub version: u32,
    pub graph_id: GraphId,
    pub fn_table: HashMap<String, fn(&mut GraphExecCtx)>,
}

impl GraphProgram {
    /// Creates an empty program table.
    pub fn new(version: u32, graph_id: GraphId) -> Self {
        Self {
            version,
            graph_id,
            fn_table: HashMap::new(),
        }
    }
}

/// Mutable context passed to graph entry functions.
pub struct GraphExecCtx<'a> {
    pub world: &'a mut World,
    pub graph_entity: Entity,
    pub tick: TickCount,
}

impl GraphExecCtx<'_> {
    /// Emits a seek request consumed on the next advance (IR-4.9.6).
    pub fn emit_timeline_seek(
        &self,
        timeline_entity: Entity,
        target_tick: TickCount,
    ) -> Result<(), TimelineSeekEvent> {
        let Some(tx) = &self.world.timeline_seek_tx else {
            return Ok(());
        };
        let ev = TimelineSeekEvent {
            timeline_entity,
            target_tick,
        };
        match tx.try_send(ev) {
            Ok(()) => Ok(()),
            Err(TrySendError::Full(v)) => {
                log::warn_channel_full("TimelineSeek");
                Err(v)
            }
            Err(TrySendError::Disconnected(v)) => Err(v),
        }
    }
}

/// FM-5: when a branch condition errors, take the first ordered tick.
pub fn branch_tick_or_default(condition: Result<bool, ()>, tick_a: TickCount, tick_b: TickCount) -> TickCount {
    match condition {
        Ok(false) => tick_b,
        Ok(true) | Err(()) => tick_a,
    }
}

/// Per-entity graph runtime (scripting internal model).
#[derive(Clone, Debug)]
pub struct GraphInstance {
    pub state_machine: GraphStateMachine,
    pub variables: VariableStore,
    pub branch: Option<GraphBranchConfig>,
}

impl GraphInstance {
    /// Creates a fresh graph instance for `program_version`.
    pub fn new(program_version: u32) -> Self {
        Self {
            state_machine: GraphStateMachine::new(program_version),
            variables: VariableStore::default(),
            branch: None,
        }
    }
}

/// Explicit `u32` state machine (no hidden stacks).
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct GraphStateMachine {
    pub current_step: u32,
    pub wait: Option<WaitCondition>,
    pub program_version: u32,
    pub pending_timeline_resume: bool,
    pub last_outcome: StepOutcome,
}

impl GraphStateMachine {
    /// Constructs a machine aligned to a loaded `GraphProgram` version.
    pub fn new(program_version: u32) -> Self {
        Self {
            current_step: GRAPH_STEP_NOT_STARTED,
            wait: None,
            program_version,
            pending_timeline_resume: false,
            last_outcome: StepOutcome::Ok,
        }
    }

    /// Arms a timeline completion wait (IR-4.9.3).
    pub fn arm_timeline_wait(
        &mut self,
        timeline_entity: Entity,
        armed_at: TickCount,
        timeout: Option<TickCount>,
    ) {
        self.wait = Some(WaitCondition::TimelineComplete {
            timeline_entity,
            armed_at,
            timeout,
        });
        self.pending_timeline_resume = false;
        self.last_outcome = StepOutcome::Ok;
    }

    /// Applies `TimelineComplete` notifications for the current tick.
    pub fn observe_timeline_events(&mut self, events: &[TimelineEvent], tick: TickCount) {
        let Some(WaitCondition::TimelineComplete {
            timeline_entity,
            armed_at,
            timeout,
        }) = self.wait
        else {
            return;
        };

        for ev in events {
            if ev.entity == timeline_entity && matches!(ev.kind, TimelineEventKind::TimelineComplete) {
                self.pending_timeline_resume = true;
                return;
            }
        }

        let effective = effective_timeline_timeout(timeout);
        if tick.0.saturating_sub(armed_at.0) >= effective.0 {
            self.last_outcome = StepOutcome::Error(RuntimeError::WaitTimeout);
            self.wait = None;
            self.pending_timeline_resume = false;
        }
    }

    /// Advances `current_step` after a deferred resume (next tick semantics).
    pub fn apply_pending_resume(&mut self) {
        if self.pending_timeline_resume {
            if self.current_step == GRAPH_STEP_NOT_STARTED {
                self.current_step = 0;
            } else {
                self.current_step = self.current_step.saturating_add(1);
            }
            self.wait = None;
            self.pending_timeline_resume = false;
            self.last_outcome = StepOutcome::Ok;
        }
    }
}

/// Graph-side execution helpers (IR-4.9.1 / IR-4.9.5).
pub struct GraphExecutionSystem;

impl GraphExecutionSystem {
    /// Invokes `on_timeline_event` when a keyframe targets a graph entity (IR-4.9.1).
    pub fn dispatch_timeline_events(world: &mut World, events: &[TimelineEvent], _tick: TickCount) {
        for ev in events {
            let TimelineEventKind::KeyframeCrossed { .. } = &ev.kind else {
                continue;
            };
            let Some(graph_entity) = ev.graph_entity else {
                continue;
            };
            let Some(program) = world.programs.get(&graph_entity).cloned() else {
                continue;
            };
            let Some(handler) = program.fn_table.get("on_timeline_event").copied() else {
                log::warn_missing_timeline_entry(program.graph_id);
                continue;
            };
            let mut ctx = GraphExecCtx {
                world,
                graph_entity,
                tick: ev.time,
            };
            handler(&mut ctx);
        }
    }

    /// Drains UI choices and applies simple branch seeking (IR-4.9.5.5).
    pub fn drain_choice_events(world: &mut World, _tick: TickCount) {
        while let Ok(ev) = world.choice_rx.try_recv() {
            Self::apply_choice(world, ev);
        }
    }

    fn apply_choice(world: &mut World, ev: ChoiceMadeEvent) {
        for instance in world.graphs.values_mut() {
            let Some(cfg) = instance.branch else {
                continue;
            };
            let target = if ev.branch_id == 0 {
                cfg.tick_a
            } else {
                cfg.tick_b
            };
            if let Some(pb) = world.playback.get_mut(&cfg.timeline) {
                let dur = world
                    .timelines
                    .get(&cfg.timeline)
                    .map_or(TickCount(0), |t| t.duration);
                pb.seek(target, dur);
            }
        }
    }

    /// Steps waits using timeline events (resume applies at next tick start).
    pub fn step_state_machines(world: &mut World, events: &[TimelineEvent], tick: TickCount) {
        for instance in world.graphs.values_mut() {
            instance.state_machine.observe_timeline_events(events, tick);
        }
        let _ = tick;
    }

    /// Applies deferred timeline resumes queued on the prior tick (IR-4.9.3.2).
    pub fn begin_tick(world: &mut World) {
        for instance in world.graphs.values_mut() {
            instance.state_machine.apply_pending_resume();
        }
    }

    /// Hot-reloads a program and reconciles instance state (FM-7).
    ///
    /// Returns slot ids removed from the `VariableStore` layout (for tests and
    /// diagnostics).
    pub fn hot_reload(
        world: &mut World,
        graph_entity: Entity,
        new_program: Arc<GraphProgram>,
        kept_slots: &[crate::ids::SlotId],
    ) -> Vec<crate::ids::SlotId> {
        let Some(inst) = world.graphs.get_mut(&graph_entity) else {
            return Vec::new();
        };
        if inst.state_machine.program_version == new_program.version {
            return Vec::new();
        }
        let before = inst.variables.slot_ids();
        inst.state_machine.current_step = GRAPH_STEP_NOT_STARTED;
        inst.state_machine.wait = None;
        inst.state_machine.pending_timeline_resume = false;
        inst.state_machine.program_version = new_program.version;
        inst.variables.retain_slots(kept_slots);
        let mut dropped = Vec::new();
        for sid in before {
            if !kept_slots.contains(&sid) {
                log::warn_dropped_slot_after_reload(sid);
                dropped.push(sid);
            }
        }
        world.programs.insert(graph_entity, new_program);
        dropped
    }
}
