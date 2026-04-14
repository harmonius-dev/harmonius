//! Deterministic deferred command recording used by sync-point tests.

use std::any::Any;
use std::cmp::Ordering;

type CommandApply = dyn FnMut(&mut dyn Any) + Send;

/// A deferred command with a stable sort key (system execution order).
pub struct Command {
    pub(crate) order: u32,
    pub(crate) apply: Box<CommandApply>,
}

/// Records commands and flushes them in deterministic `order` sequence.
#[derive(Default)]
pub struct CommandBuffer {
    commands: Vec<Command>,
}

impl CommandBuffer {
    /// Creates an empty buffer.
    pub fn new() -> Self {
        Self::default()
    }

    /// Records a command with the given stable ordering key.
    pub fn record(&mut self, order: u32, apply: impl FnMut(&mut dyn Any) + Send + 'static) {
        self.commands.push(Command {
            order,
            apply: Box::new(apply),
        });
    }

    /// Sorts by `order` and applies each command against `ctx`.
    pub fn flush(&mut self, ctx: &mut dyn Any) {
        self.commands.sort_by(|a, b| a.order.cmp(&b.order));
        for cmd in &mut self.commands {
            (cmd.apply)(ctx);
        }
        self.commands.clear();
    }

    /// Number of pending commands.
    pub fn len(&self) -> usize {
        self.commands.len()
    }

    /// True when no commands are pending.
    pub fn is_empty(&self) -> bool {
        self.commands.is_empty()
    }
}

impl PartialEq for Command {
    fn eq(&self, other: &Self) -> bool {
        self.order == other.order
    }
}

impl Eq for Command {}

impl PartialOrd for Command {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Command {
    fn cmp(&self, other: &Self) -> Ordering {
        self.order.cmp(&other.order)
    }
}
