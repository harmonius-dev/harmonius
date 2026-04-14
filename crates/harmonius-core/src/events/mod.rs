//! Typed event channels, persistent streams, observers, bridges, and command buffers.

mod bridge;
mod channel;
mod command_buffer;
mod observer;
mod reactive;
mod stream;

pub use bridge::{EventBridge, EventBridgeConfig};
pub use channel::{Event, EventChannel};
pub use command_buffer::CommandBuffer;
pub use observer::{EventObserverRegistry, ObserverCallback, ObserverDescriptor, ObserverId};
pub use reactive::ReactiveQuery;
pub use stream::{PersistentStream, StreamConfig, StreamCursor};
