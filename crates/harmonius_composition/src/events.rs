//! Cross-primitive ECS-style events (R-16.5.3).

use bevy_ecs::prelude::*;

/// Event log entry appended (design: `EntryAppended`).
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct EntryAppended {
    /// Target entity owning the log consumer.
    pub target: Entity,
    /// Payload label for tests.
    pub label: &'static str,
}

/// Container slot changed.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SlotChanged {
    /// Owning entity.
    pub owner: Entity,
    /// Slot index.
    pub slot: u32,
}

/// Grid cell crossed a threshold.
#[derive(Clone, Debug, PartialEq)]
pub struct CellChanged {
    /// Grid owner entity.
    pub owner: Entity,
    /// Cell value after change.
    pub value: f32,
}

/// Attribute threshold crossed.
#[derive(Clone, Debug, PartialEq)]
pub struct ThresholdCrossed {
    /// Entity whose attribute crossed.
    pub entity: Entity,
    /// Attribute name.
    pub attr: String,
    /// New value.
    pub value: f32,
}

/// Timeline keyframe fired.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct KeyframeFired {
    /// Timeline owner.
    pub owner: Entity,
    /// Keyframe tick.
    pub tick: u64,
}

/// Union of composition-layer events for the in-memory bus (R-16.5.3).
#[derive(Clone, Debug, PartialEq)]
pub enum CompositionEvent {
    /// See [`EntryAppended`].
    EntryAppended(EntryAppended),
    /// See [`SlotChanged`].
    SlotChanged(SlotChanged),
    /// See [`CellChanged`].
    CellChanged(CellChanged),
    /// See [`ThresholdCrossed`].
    ThresholdCrossed(ThresholdCrossed),
    /// See [`KeyframeFired`].
    KeyframeFired(KeyframeFired),
}

/// Resource holding ordered composition events for a frame.
#[derive(Resource, Default, Debug)]
pub struct CompositionEventQueue {
    /// Pending events drained in phase order.
    pub events: std::collections::VecDeque<CompositionEvent>,
}

impl CompositionEventQueue {
    /// Push an event to the back of the queue.
    pub fn send(&mut self, event: CompositionEvent) {
        self.events.push_back(event);
    }
}
