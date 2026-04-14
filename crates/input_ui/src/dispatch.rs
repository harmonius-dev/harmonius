//! Bounded pointer queue and portable worker-thread dispatch.
//!
//! The integration design sketches `ui_input_dispatch_system` with engine resource handles; this
//! module holds the synchronous drain + route logic those wrappers invoke.

use std::collections::VecDeque;

use crate::context::ContextStack;
use crate::focus::FocusManager;
use crate::router::{DispatchLog, EventRouter, HandlerPolicy, WidgetTree};
use crate::types::{Entity, UiPointerEvent, Vec2};

/// Channel capacity from `docs/design/integration/input-ui.md` (CH-1).
pub const POINTER_CHANNEL_CAPACITY: usize = 256;

/// Receiver side of the bounded pointer queue described in the integration design.
#[derive(Clone, Debug, Default)]
pub struct PointerEventReceiver {
    queue: VecDeque<UiPointerEvent>,
}

/// Returned when [`PointerEventReceiver::try_enqueue`] would exceed
/// [`POINTER_CHANNEL_CAPACITY`].
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct PointerChannelFull;

impl PointerEventReceiver {
    /// Creates an empty queue.
    pub const fn new() -> Self {
        Self {
            queue: VecDeque::new(),
        }
    }

    /// Bounded enqueue; on failure returns the event so callers can drop or retry.
    pub fn try_enqueue(
        &mut self,
        event: UiPointerEvent,
    ) -> Result<(), (PointerChannelFull, UiPointerEvent)> {
        if self.queue.len() >= POINTER_CHANNEL_CAPACITY {
            return Err((PointerChannelFull, event));
        }
        self.queue.push_back(event);
        Ok(())
    }

    fn pop_front(&mut self) -> Option<UiPointerEvent> {
        self.queue.pop_front()
    }
}

/// Immutable inputs for [`ui_input_dispatch_system_body`].
pub struct PointerDispatchEnv<'a> {
    /// Active mapping contexts controlling consumption.
    pub stack: &'a ContextStack,
    /// Widget geometry for hit testing.
    pub tree: &'a WidgetTree,
    /// Capture/bubble stop hooks.
    pub policy: &'a HandlerPolicy,
}

/// Tracks the last hit-tested widget for events without an explicit position (e.g. `Leave`).
#[derive(Clone, Debug, Default)]
pub struct PointerDispatchCursor {
    last_under_pointer: Option<Entity>,
}

impl PointerDispatchCursor {
    /// Clears remembered hover target (e.g. after a synthetic reset).
    pub fn reset(&mut self) {
        self.last_under_pointer = None;
    }

    fn dispatch_one(
        &mut self,
        env: &PointerDispatchEnv<'_>,
        router: &mut EventRouter,
        event: UiPointerEvent,
        log: &mut DispatchLog,
    ) {
        if !env.stack.pointer_consumed() {
            return;
        }

        let hit = match pointer_position(event) {
            Some(pos) => {
                let h = env.tree.hit_test(pos);
                self.last_under_pointer = h;
                h
            }
            None => self.last_under_pointer,
        };

        router.dispatch_pointer(env.tree, hit, event, env.policy, log);

        if matches!(event, UiPointerEvent::Leave) {
            self.last_under_pointer = None;
        }
    }
}

fn pointer_position(event: UiPointerEvent) -> Option<Vec2> {
    match event {
        UiPointerEvent::Enter { position }
        | UiPointerEvent::Down { position, .. }
        | UiPointerEvent::Up { position, .. } => Some(position),
        UiPointerEvent::Move { position, .. } => Some(position),
        UiPointerEvent::Leave => None,
    }
}

/// Portable worker-thread step matching the integration design's `ui_input_dispatch_system` body.
///
/// Engine layers wrap this with scheduler `ResMut` / `Res` types. `focus` is reserved for keyboard
/// routing in the same design section; pointer-only frames ignore it for now.
pub fn ui_input_dispatch_system_body(
    rx: &mut PointerEventReceiver,
    cursor: &mut PointerDispatchCursor,
    router: &mut EventRouter,
    focus: &mut FocusManager,
    env: &PointerDispatchEnv<'_>,
    log: &mut DispatchLog,
) {
    let _ = focus;
    while let Some(event) = rx.pop_front() {
        cursor.dispatch_one(env, router, event, log);
    }
}
