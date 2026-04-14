//! Typed event channels, persistent streams, observers, bridges, and command buffers.

mod bridge;
mod channel;
mod command_buffer;
mod observer;
mod reactive;
mod stream;
mod writer_reader;

pub use bridge::{EventBridge, EventBridgeConfig};
pub use channel::{Event, EventChannel};
pub use command_buffer::CommandBuffer;
pub use observer::{
    EventObserverRegistry, ObserverCallback, ObserverDescriptor, ObserverId,
    ObserverRegisterError,
};
pub use reactive::ReactiveQuery;
pub use stream::{PersistentStream, StreamConfig, StreamCursor};
pub use writer_reader::{EventReader, EventWriter};
