//! Timeline advance + seek drain (IR-4.9.6).

use smallvec::SmallVec;
use std::sync::mpsc::{sync_channel, Receiver, SyncSender, TrySendError};

use crate::event::{TimelineEvent, TimelineEventKind};
use crate::graph::GraphExecutionSystem;
use crate::log;
use crate::ids::{Entity, TickCount};
use crate::timeline::TrackValue;
use crate::variable::{validate_binding, TypedSlot};
use crate::world::World;

/// Seek request emitted by graphs (capacity 256).
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct TimelineSeekEvent {
    pub timeline_entity: Entity,
    pub target_tick: TickCount,
}

/// Timeline advance + seek integration point.
pub struct TimelineAdvanceSystem {
    pub seek_tx: SyncSender<TimelineSeekEvent>,
    seek_rx: Receiver<TimelineSeekEvent>,
}

impl TimelineAdvanceSystem {
    /// Opens a bounded `TimelineSeek` channel (capacity 256).
    pub fn new() -> Self {
        let (seek_tx, seek_rx) = sync_channel(256);
        Self { seek_tx, seek_rx }
    }

    /// Bounded `try_send` for graph producers (FM-6).
    pub fn try_send(&self, ev: TimelineSeekEvent) -> Result<(), TimelineSeekEvent> {
        match self.seek_tx.try_send(ev) {
            Ok(()) => Ok(()),
            Err(TrySendError::Full(v)) => {
                log::warn_channel_full("TimelineSeek");
                Err(v)
            }
            Err(TrySendError::Disconnected(v)) => Err(v),
        }
    }

    /// Drains pending seeks before advancing timelines.
    pub fn drain_seek_into(&self, world: &mut World) {
        while let Ok(ev) = self.seek_rx.try_recv() {
            if let Some(pb) = world.playback.get_mut(&ev.timeline_entity) {
                let dur = world
                    .timelines
                    .get(&ev.timeline_entity)
                    .map_or(TickCount(0), |t| t.duration);
                pb.seek(ev.target_tick, dur);
            }
        }
    }
}

/// Runs one fixed tick of simulation ordering for this integration crate.
pub fn tick_simulation(
    world: &mut World,
    advance: &TimelineAdvanceSystem,
    tick: TickCount,
) -> SmallVec<[TimelineEvent; 8]> {
    GraphExecutionSystem::begin_tick(world);
    advance.drain_seek_into(world);

    let mut events: SmallVec<[TimelineEvent; 8]> = SmallVec::new();
    let timeline_entities: Vec<Entity> = world.timelines.keys().copied().collect();

    for tl_entity in timeline_entities {
        let Some(tl) = world.timelines.get(&tl_entity).cloned() else {
            continue;
        };
        let Some(pb) = world.playback.get_mut(&tl_entity) else {
            continue;
        };
        if !pb.playing {
            continue;
        }
        let from = pb.current_tick;
        pb.advance_ticks(1, tl.duration);
        let to = pb.current_tick;

        for (track, kf_id, value) in tl.keyframes_crossed(from, to) {
            let graph_entity = match value {
                TrackValue::Entity(e) => Some(e),
                _ => None,
            };
            events.push(TimelineEvent {
                kind: TimelineEventKind::KeyframeCrossed {
                    track,
                    keyframe: kf_id,
                },
                time: tick,
                entity: tl_entity,
                graph_entity,
            });
        }

        if let Some(graph_entity) = world.timeline_graph.get(&tl_entity).copied() {
            if let Some(bindings) = world.bindings.get(&tl_entity).cloned() {
                if let Some(graph) = world.graphs.get_mut(&graph_entity) {
                    for (track_id, slot_id) in bindings {
                        let Some(sample) = tl.sample_track(track_id, to) else {
                            continue;
                        };
                        if validate_binding(track_id, &sample, &graph.variables, slot_id).is_err() {
                            continue;
                        }
                        let typed: TypedSlot = sample.to_typed_slot();
                        let _ = graph.variables.set(slot_id, typed);
                    }
                }
            }
        }

        if from < tl.duration && to >= tl.duration {
            events.push(TimelineEvent {
                kind: TimelineEventKind::TimelineComplete,
                time: tick,
                entity: tl_entity,
                graph_entity: None,
            });
        }
    }

    GraphExecutionSystem::dispatch_timeline_events(world, &events, tick);
    GraphExecutionSystem::step_state_machines(world, &events, tick);
    GraphExecutionSystem::drain_choice_events(world, tick);

    events
}
