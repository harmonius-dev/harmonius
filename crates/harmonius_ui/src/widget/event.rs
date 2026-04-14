//! Pointer hit testing and capture/bubble delivery (`R-10.1.1`, `TC-10.1.1.3`, `TC-10.1.1.4`).

use crate::widget::tree::WidgetTree;
use crate::widget::WidgetId;

/// Axis-aligned rectangle in logical pixels (matches `ComputedLayout` extents in the design).
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Rect {
    /// Origin x.
    pub x: f32,
    /// Origin y.
    pub y: f32,
    /// Width.
    pub w: f32,
    /// Height.
    pub h: f32,
}

impl Rect {
    /// True when `(px, py)` lies inside the half-open `[x, x+w) × [y, y+h)` range.
    #[must_use]
    pub fn contains(self, px: f32, py: f32) -> bool {
        px >= self.x && py >= self.y && px < self.x + self.w && py < self.y + self.h
    }
}

/// Dispatch phase after the target phase (`TC-10.1.1.4`).
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum BubblePhase {
    /// Deepest widget under the pointer.
    Target,
    /// Ancestor chain walking toward the root.
    Bubble,
}

/// Lets a handler cut off further bubbling.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum BubbleDisposition {
    /// Keep visiting ancestors.
    Continue,
    /// Stop before the next ancestor runs.
    Stop,
}

/// Stateless helper for pointer hit testing and bubbling over a [`WidgetTree`].
#[derive(Clone, Copy, Debug, Default)]
pub struct EventRouter;

impl EventRouter {
    /// Returns the topmost widget under `(px, py)` using paint order (later siblings win).
    #[must_use]
    pub fn pointer_hit<F>(tree: &WidgetTree, rect: F, px: f32, py: f32) -> Option<WidgetId>
    where
        F: Fn(WidgetId) -> Option<Rect>,
    {
        Self::hit_walk(tree, tree.root(), &rect, px, py)
    }

    fn hit_walk<F>(tree: &WidgetTree, id: WidgetId, rect: &F, px: f32, py: f32) -> Option<WidgetId>
    where
        F: Fn(WidgetId) -> Option<Rect>,
    {
        let children = tree.children(id)?;
        for &child in children.iter().rev() {
            if let Some(hit) = Self::hit_walk(tree, child, rect, px, py) {
                return Some(hit);
            }
        }
        rect(id).filter(|r| r.contains(px, py)).map(|_| id)
    }

    /// Target phase on `target`, then each ancestor in bubble order.
    pub fn bubble_pointer<F>(tree: &WidgetTree, target: WidgetId, mut visitor: F)
    where
        F: FnMut(WidgetId, BubblePhase) -> BubbleDisposition,
    {
        if visitor(target, BubblePhase::Target) == BubbleDisposition::Stop {
            return;
        }
        let mut cursor = tree.parent(target);
        while let Some(id) = cursor {
            if visitor(id, BubblePhase::Bubble) == BubbleDisposition::Stop {
                break;
            }
            cursor = tree.parent(id);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::widget::node::{WidgetKey, WidgetKind};
    use crate::widget::WidgetTree;

    /// `TC-10.1.1.3` — hit test picks the widget whose `Rect` contains the point.
    #[test]
    fn tc_10_1_1_3_pointer_hit_topmost() {
        let mut tree = WidgetTree::new_with_root_panel();
        let root = tree.root();
        let leaf = tree
            .insert_child(root, WidgetKind::Button, WidgetKey::Index(1))
            .expect("insert");

        let rects = |id: WidgetId| {
            if id == root {
                Some(Rect {
                    x: 0.0,
                    y: 0.0,
                    w: 200.0,
                    h: 200.0,
                })
            } else if id == leaf {
                Some(Rect {
                    x: 50.0,
                    y: 50.0,
                    w: 40.0,
                    h: 40.0,
                })
            } else {
                None
            }
        };

        let hit = EventRouter::pointer_hit(&tree, rects, 60.0, 60.0);
        assert_eq!(hit, Some(leaf));
    }

    /// `TC-10.1.1.4` — target then bubble; `Stop` on parent skips grandparent.
    #[test]
    fn tc_10_1_1_4_bubble_stop_skips_grandparent() {
        let mut tree = WidgetTree::new_with_root_panel();
        let root = tree.root();
        let mid = tree
            .insert_child(root, WidgetKind::Panel, WidgetKey::Index(1))
            .expect("mid");
        let leaf = tree
            .insert_child(mid, WidgetKind::Label, WidgetKey::Index(2))
            .expect("leaf");

        let mut log = Vec::new();
        EventRouter::bubble_pointer(&tree, leaf, |id, phase| {
            log.push((id, phase));
            if id == mid && phase == BubblePhase::Bubble {
                BubbleDisposition::Stop
            } else {
                BubbleDisposition::Continue
            }
        });

        assert_eq!(
            log,
            vec![
                (leaf, BubblePhase::Target),
                (mid, BubblePhase::Bubble),
            ]
        );
        assert!(!log.iter().any(|(id, _)| *id == root));
    }
}
