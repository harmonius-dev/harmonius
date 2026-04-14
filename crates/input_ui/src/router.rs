//! Hit testing and capture/target/bubble dispatch.

use std::collections::HashMap;

use crate::types::{Entity, EventPhase, InteractionState, Rect, UiPointerEvent, Vec2};

/// Widget layout and interaction metadata used by the router.
#[derive(Clone, Debug)]
pub struct WidgetSpec {
    /// Widget entity id.
    pub id: Entity,
    /// Parent id, `None` for the synthetic root.
    pub parent: Option<Entity>,
    /// Layout rectangle in logical pixels.
    pub rect: Rect,
    /// Draw order; higher wins overlaps.
    pub z_order: i32,
    /// Disabled widgets skip target delivery.
    pub disabled: bool,
}

/// Immutable widget tree snapshot for routing.
#[derive(Clone, Debug)]
pub struct WidgetTree {
    widgets: Vec<WidgetSpec>,
    root: Entity,
}

impl WidgetTree {
    /// Creates a tree with an explicit root id and widget specs.
    pub fn new(root: Entity, widgets: Vec<WidgetSpec>) -> Self {
        Self { widgets, root }
    }

    fn spec(&self, id: Entity) -> Option<&WidgetSpec> {
        self.widgets.iter().find(|w| w.id == id)
    }

    /// Returns the path from `root` down to `target` (inclusive), if linked by parents.
    pub fn path_root_to_target(&self, target: Entity) -> Option<Vec<Entity>> {
        let mut cur = Some(target);
        let mut rev = Vec::new();
        while let Some(id) = cur {
            rev.push(id);
            if id == self.root {
                break;
            }
            cur = self.spec(id).and_then(|w| w.parent);
        }
        if rev.last().copied() != Some(self.root) {
            return None;
        }
        rev.reverse();
        Some(rev)
    }

    /// Picks the top-most widget under `position`, preferring highest `z_order`.
    pub fn hit_test(&self, position: Vec2) -> Option<Entity> {
        let mut best: Option<(i32, Entity)> = None;
        for w in &self.widgets {
            if w.id == self.root {
                continue;
            }
            if !w.rect.contains(position) {
                continue;
            }
            let candidate = (w.z_order, w.id);
            if best.is_none_or(|b| {
                candidate.0 > b.0 || (candidate.0 == b.0 && candidate.1 .0 > b.1 .0)
            }) {
                best = Some(candidate);
            }
        }
        best.map(|b| b.1)
    }
}

/// Per-widget handler policy used by deterministic tests.
#[derive(Clone, Debug, Default)]
pub struct HandlerPolicy {
    /// Stop capture traversal **before** visiting this widget (handler runs without delivery).
    pub capture_stop_before: std::collections::HashSet<Entity>,
    /// Stop bubble traversal after visiting this widget.
    pub bubble_stop_after: std::collections::HashSet<Entity>,
}

/// One routed delivery record.
#[derive(Clone, Debug, PartialEq)]
pub struct DispatchRecord {
    /// Widget that received the routed copy.
    pub target: Entity,
    /// Propagation phase.
    pub phase: EventPhase,
    /// Event payload.
    pub event: UiPointerEvent,
}

/// Ordered log of routed pointer events.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct DispatchLog {
    records: Vec<DispatchRecord>,
}

impl DispatchLog {
    /// Returns recorded deliveries in visitation order.
    pub fn records(&self) -> &[DispatchRecord] {
        &self.records
    }

    fn push(&mut self, target: Entity, phase: EventPhase, event: UiPointerEvent) {
        self.records.push(DispatchRecord {
            target,
            phase,
            event,
        });
    }
}

/// Routes pointer events with capture/target/bubble semantics.
#[derive(Clone, Debug, Default)]
pub struct EventRouter {
    /// Latest interaction snapshot per widget entity.
    pub interaction: HashMap<Entity, InteractionState>,
}

impl EventRouter {
    /// Creates an empty router.
    pub fn new() -> Self {
        Self {
            interaction: HashMap::new(),
        }
    }

    /// Routes `event` to `hit` using `tree` and `policy`, appending to `log`.
    pub fn dispatch_pointer(
        &mut self,
        tree: &WidgetTree,
        hit: Option<Entity>,
        event: UiPointerEvent,
        policy: &HandlerPolicy,
        log: &mut DispatchLog,
    ) {
        let Some(hit) = hit else {
            return;
        };
        let Some(path) = tree.path_root_to_target(hit) else {
            return;
        };
        if path.len() < 2 {
            return;
        }

        let capture_nodes = &path[..path.len() - 1];
        let mut capture_aborted = false;
        for &node in capture_nodes {
            if policy.capture_stop_before.contains(&node) {
                capture_aborted = true;
                break;
            }
            log.push(node, EventPhase::Capture, event);
        }

        if capture_aborted {
            return;
        }

        if let Some(spec) = tree.spec(hit) {
            if !spec.disabled {
                log.push(hit, EventPhase::Target, event);
                let mut bubble_chain: Vec<Entity> = path[..path.len() - 1].to_vec();
                bubble_chain.reverse();
                for node in bubble_chain {
                    log.push(node, EventPhase::Bubble, event);
                    if policy.bubble_stop_after.contains(&node) {
                        break;
                    }
                }
            }
        }

        self.refresh_interaction(tree, hit, event);
    }

    fn refresh_interaction(&mut self, tree: &WidgetTree, hit: Entity, event: UiPointerEvent) {
        match event {
            UiPointerEvent::Enter { .. } | UiPointerEvent::Move { .. } => {
                for w in &tree.widgets {
                    let hovered = w.id == hit && !w.disabled;
                    let entry = self.interaction.entry(w.id).or_insert(InteractionState {
                        disabled: w.disabled,
                        ..InteractionState::default()
                    });
                    entry.disabled = w.disabled;
                    entry.hovered = hovered;
                }
            }
            UiPointerEvent::Down { .. } => {
                if let Some(spec) = tree.spec(hit) {
                    if !spec.disabled {
                        if let Some(entry) = self.interaction.get_mut(&hit) {
                            entry.pressed = true;
                        } else {
                            self.interaction.insert(
                                hit,
                                InteractionState {
                                    hovered: true,
                                    pressed: true,
                                    disabled: spec.disabled,
                                    ..InteractionState::default()
                                },
                            );
                        }
                    }
                }
            }
            UiPointerEvent::Up { .. } => {
                if let Some(entry) = self.interaction.get_mut(&hit) {
                    entry.pressed = false;
                }
            }
            UiPointerEvent::Leave => {
                for w in &tree.widgets {
                    if let Some(entry) = self.interaction.get_mut(&w.id) {
                        entry.hovered = false;
                    }
                }
            }
        }
    }

    /// Clears hover state for widgets no longer under the cursor (used after explicit leave).
    pub fn clear_hover(&mut self, tree: &WidgetTree) {
        for w in &tree.widgets {
            if let Some(entry) = self.interaction.get_mut(&w.id) {
                entry.hovered = false;
            }
        }
    }
}
