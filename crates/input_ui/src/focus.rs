//! Focus traversal for keyboard and gamepad navigation.

use crate::types::{Entity, Rect};

/// Focus grouping metadata for directional navigation.
#[derive(Clone, Debug)]
pub struct FocusGroup {
    /// Stable group id shared by focusable widgets.
    pub id: u32,
    /// When true, directional navigation wraps at edges.
    pub wrap: bool,
}

/// Focusable widget metadata.
#[derive(Clone, Debug)]
pub struct Focusable {
    /// Widget entity.
    pub entity: Entity,
    /// Tab order key (ascending).
    pub tab_index: i32,
    /// Optional grouping for directional moves.
    pub group: Option<FocusGroup>,
    /// Layout bounds used for directional hit picking.
    pub rect: Rect,
}

/// Maintains the focused widget among an ordered focus ring.
#[derive(Clone, Debug)]
pub struct FocusManager {
    ring: Vec<Focusable>,
    current: Option<usize>,
}

impl FocusManager {
    /// Creates a manager from a focus ring sorted by tab order by the caller.
    pub fn new(ring: Vec<Focusable>) -> Self {
        Self {
            ring,
            current: None,
        }
    }

    /// Returns the currently focused entity, if any.
    pub fn focused_entity(&self) -> Option<Entity> {
        self.current
            .and_then(|idx| self.ring.get(idx).map(|f| f.entity))
    }

    /// Sets focus explicitly (used by tests).
    pub fn set_focus(&mut self, entity: Entity) -> bool {
        if let Some(idx) = self.ring.iter().position(|f| f.entity == entity) {
            self.current = Some(idx);
            true
        } else {
            false
        }
    }

    /// Advances focus in tab order.
    pub fn advance(&mut self) {
        if self.ring.is_empty() {
            return;
        }
        let next = match self.current {
            None => 0,
            Some(idx) => (idx + 1) % self.ring.len(),
        };
        self.current = Some(next);
    }

    /// Reverses focus in tab order.
    pub fn reverse(&mut self) {
        if self.ring.is_empty() {
            return;
        }
        let next = match self.current {
            None => self.ring.len() - 1,
            Some(idx) => (idx + self.ring.len() - 1) % self.ring.len(),
        };
        self.current = Some(next);
    }

    /// Directional navigation within the active focus group along +X (right).
    pub fn directional_right(&mut self) {
        self.directional(|from, candidate| {
            let dc = candidate.center().x - from.center().x;
            if dc <= f32::EPSILON {
                return None;
            }
            let dy = (candidate.center().y - from.center().y).abs();
            Some((dc, dy))
        });
    }

    /// South button activation while a widget is focused.
    pub fn gamepad_south(&self) -> Option<WidgetCommand> {
        self.focused_entity().map(|_| WidgetCommand::Activate)
    }

    /// East button cancel while a widget is focused.
    pub fn gamepad_east(&self) -> Option<WidgetCommand> {
        self.focused_entity().map(|_| WidgetCommand::Close)
    }

    fn directional<F>(&mut self, score: F)
    where
        F: Fn(Rect, Rect) -> Option<(f32, f32)>,
    {
        let Some(idx) = self.current else {
            return;
        };
        let from = self.ring[idx].rect;
        let group_id = self.ring[idx].group.as_ref().map(|g| g.id);
        let wrap = self.ring[idx].group.as_ref().is_some_and(|g| g.wrap);

        let mut best: Option<(f32, f32, usize)> = None;
        for (j, cand) in self.ring.iter().enumerate() {
            if j == idx {
                continue;
            }
            if group_id
                .zip(cand.group.as_ref().map(|g| g.id))
                .is_some_and(|(a, b)| a != b)
            {
                continue;
            }
            if let Some((dx, dy)) = score(from, cand.rect) {
                let tuple = (dx, dy, j);
                if best.is_none_or(|b| tuple.0 < b.0 || (tuple.0 == b.0 && tuple.1 < b.1)) {
                    best = Some(tuple);
                }
            }
        }

        if let Some((_, _, j)) = best {
            self.current = Some(j);
            return;
        }

        if wrap {
            let mut rightmost = None;
            for (j, cand) in self.ring.iter().enumerate() {
                if j == idx {
                    continue;
                }
                if group_id
                    .zip(cand.group.as_ref().map(|g| g.id))
                    .is_some_and(|(a, b)| a != b)
                {
                    continue;
                }
                let cx = cand.rect.center().x;
                rightmost = match rightmost {
                    None => Some((cx, j)),
                    Some((best_x, _best_j)) if cx > best_x => Some((cx, j)),
                    Some((best_x, best_j)) if (cx - best_x).abs() < f32::EPSILON && j < best_j => {
                        Some((cx, j))
                    }
                    other => other,
                };
            }
            if let Some((_, j)) = rightmost {
                self.current = Some(j);
            }
        }
    }
}

/// Widget-level semantic events for gamepad confirmation.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum WidgetCommand {
    /// Primary activation (South).
    Activate,
    /// Cancel / close (East).
    Close,
}
