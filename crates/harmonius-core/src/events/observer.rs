//! Event-dispatch observer registry (distinct from ECS component observers).

use std::any::TypeId;
use std::cmp::Ordering;
use std::collections::BTreeMap;

use crate::world::{ComponentEvent, Entity, World};

/// Identifier returned when registering an observer.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct ObserverId(pub u64);

/// Describes which components and triggers an observer cares about.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ObserverDescriptor {
    /// Component types watched by this observer.
    pub watched_components: Vec<TypeId>,
    /// Which lifecycle events invoke the callback.
    pub triggers: Vec<ComponentEvent>,
    /// Lower priority values run first among matches.
    pub priority: i32,
}

struct ObserverEntry {
    id: ObserverId,
    descriptor: ObserverDescriptor,
    callback: Box<dyn ObserverCallback>,
}

/// Callback invoked when an observed component event occurs.
pub trait ObserverCallback: Send + 'static {
    /// Invokes the observer with exclusive world access.
    fn invoke(&mut self, event: ComponentEvent, entity: Entity, world: &mut World);
}

/// Registry of event-dispatch observers sorted for deterministic iteration.
#[derive(Default)]
pub struct EventObserverRegistry {
    next_id: u64,
    observers: BTreeMap<TypeId, Vec<ObserverEntry>>,
}

impl EventObserverRegistry {
    /// Creates an empty registry.
    pub fn new() -> Self {
        Self::default()
    }

    /// Registers an observer for the first watched component bucket.
    pub fn register(
        &mut self,
        descriptor: ObserverDescriptor,
        callback: impl ObserverCallback + 'static,
    ) -> ObserverId {
        let id = ObserverId(self.next_id);
        self.next_id = self.next_id.saturating_add(1);
        let key = descriptor
            .watched_components
            .first()
            .copied()
            .unwrap_or(TypeId::of::<()>());
        let entry = ObserverEntry {
            id,
            descriptor,
            callback: Box::new(callback),
        };
        self.observers.entry(key).or_default().push(entry);
        for vec in self.observers.values_mut() {
            vec.sort_by(
                |a, b| match a.descriptor.priority.cmp(&b.descriptor.priority) {
                    Ordering::Equal => a.id.0.cmp(&b.id.0),
                    other => other,
                },
            );
        }
        id
    }

    /// Removes an observer by id (linear scan).
    pub fn unregister(&mut self, id: ObserverId) -> bool {
        for vec in self.observers.values_mut() {
            let before = vec.len();
            vec.retain(|e| e.id != id);
            if vec.len() != before {
                return true;
            }
        }
        false
    }

    /// Dispatches all observers matching `component_type` and `event`.
    pub fn notify(
        &mut self,
        event: ComponentEvent,
        component_type: TypeId,
        entity: Entity,
        world: &mut World,
    ) {
        let Some(entries) = self.observers.get_mut(&component_type) else {
            return;
        };
        for entry in entries.iter_mut() {
            if entry.descriptor.triggers.contains(&event)
                && entry
                    .descriptor
                    .watched_components
                    .contains(&component_type)
            {
                entry.callback.invoke(event, entity, world);
            }
        }
    }
}
