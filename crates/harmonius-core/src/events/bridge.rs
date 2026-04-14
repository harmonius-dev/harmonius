//! Cross-world routing for a single event type.

use super::channel::{Event, EventChannel};

type EventFilter<T> = dyn Fn(&T) -> bool + Send + Sync;
type EventTransform<T> = dyn Fn(T) -> T + Send + Sync;

/// Configuration for [`EventBridge`].
pub struct EventBridgeConfig<T: Event> {
    /// Optional filter evaluated on source events.
    pub filter: Option<Box<EventFilter<T>>>,
    /// Optional transform applied after filtering.
    pub transform: Option<Box<EventTransform<T>>>,
    _marker: std::marker::PhantomData<T>,
}

impl<T: Event> EventBridgeConfig<T> {
    /// Creates a passthrough configuration.
    pub fn passthrough() -> Self {
        Self {
            filter: None,
            transform: None,
            _marker: std::marker::PhantomData,
        }
    }

    /// Adds a filter predicate.
    pub fn with_filter(mut self, f: impl Fn(&T) -> bool + Send + Sync + 'static) -> Self {
        self.filter = Some(Box::new(f) as Box<EventFilter<T>>);
        self
    }

    /// Adds a transform mapping.
    pub fn with_transform(mut self, f: impl Fn(T) -> T + Send + Sync + 'static) -> Self {
        self.transform = Some(Box::new(f) as Box<EventTransform<T>>);
        self
    }
}

/// Routes events from a source channel into a target channel at a frame boundary.
pub struct EventBridge<T: Event> {
    config: EventBridgeConfig<T>,
}

impl<T: Event> EventBridge<T> {
    /// Wraps a bridge configuration.
    pub fn new(config: EventBridgeConfig<T>) -> Self {
        Self { config }
    }

    /// Copies readable events from `source` into `target`'s back buffer.
    pub fn transfer(&self, source: &EventChannel<T>, target: &mut EventChannel<T>) {
        for event in source.read_front() {
            let pass = self.config.filter.as_ref().is_none_or(|f| f(event));
            if !pass {
                continue;
            }
            let out = self
                .config
                .transform
                .as_ref()
                .map_or_else(|| event.clone(), |t| t(event.clone()));
            target.send(out);
        }
    }
}
