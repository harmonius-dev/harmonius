//! Single-threaded integration harness wiring timelines + graphs.

use std::collections::HashMap;
use std::sync::mpsc::{Receiver, SyncSender};

use crate::command::{ChoiceMadeEvent, CommandBuffer};
use crate::ids::{Entity, SlotId, TrackId};
use crate::playback::PlaybackState;
use crate::timeline::MultiTrackTimeline;
use crate::graph::{GraphInstance, GraphProgram};
use std::sync::Arc;

/// Owning world for integration tests and deterministic stepping.
pub struct World {
    pub timelines: HashMap<Entity, MultiTrackTimeline>,
    pub playback: HashMap<Entity, PlaybackState>,
    /// Timeline entity → graph entity that owns `VariableStore` for track bindings.
    pub timeline_graph: HashMap<Entity, Entity>,
    pub graphs: HashMap<Entity, GraphInstance>,
    pub programs: HashMap<Entity, Arc<GraphProgram>>,
    pub bindings: HashMap<Entity, Vec<(TrackId, SlotId)>>,
    pub command_buffer: CommandBuffer,
    pub choice_rx: Receiver<ChoiceMadeEvent>,
    pub choice_tx: SyncSender<ChoiceMadeEvent>,
    /// Producer handle for `TimelineSeek` (capacity 256) wired into graph handlers.
    pub timeline_seek_tx: Option<SyncSender<crate::advance::TimelineSeekEvent>>,
}

impl World {
    /// Builds a world with a bounded `ChoiceMade` channel (capacity 16).
    pub fn new() -> Self {
        let (choice_tx, choice_rx) = std::sync::mpsc::sync_channel(16);
        Self {
            timelines: HashMap::new(),
            playback: HashMap::new(),
            timeline_graph: HashMap::new(),
            graphs: HashMap::new(),
            programs: HashMap::new(),
            bindings: HashMap::new(),
            command_buffer: CommandBuffer::default(),
            choice_rx,
            choice_tx,
            timeline_seek_tx: None,
        }
    }

    /// Sends a choice event from UI input (FM-6 applies if full).
    pub fn try_send_choice(&self, ev: ChoiceMadeEvent) -> Result<(), ChoiceMadeEvent> {
        self.choice_tx.try_send(ev).map_err(|e| match e {
            std::sync::mpsc::TrySendError::Full(v) | std::sync::mpsc::TrySendError::Disconnected(v) => {
                v
            }
        })
    }
}
