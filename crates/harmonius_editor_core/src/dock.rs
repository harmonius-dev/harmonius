//! Dock tree layout for editor panels (`TC-15.1.1.*`).

use std::fmt;

/// Stable identifier for a docked panel instance.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct PanelId(pub u64);

/// Split orientation between two child dock nodes.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum SplitDirection {
    /// Left/right child panes.
    Horizontal,
    /// Top/bottom child panes.
    Vertical,
}

/// Dock placement hints used when re-docking a floating panel.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum DockZone {
    /// Dock into the left global zone.
    Left,
    /// Dock into the right global zone.
    Right,
    /// Dock into the bottom global zone.
    Bottom,
    /// Dock into the main central stack.
    Center,
    /// Floating window (handled separately).
    Floating,
}

/// Floating window metadata for a panel torn out of the dock tree.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct FloatingPanel {
    /// Stable window id chosen by the windowing layer.
    pub window_id: u64,
    /// Panel hosted in the floating window.
    pub panel: PanelId,
    /// Client-area position in screen coordinates.
    pub position: [i32; 2],
    /// Client-area size in pixels.
    pub size: [u32; 2],
}

/// Hierarchical dock layout node.
#[derive(Debug, Clone, PartialEq)]
pub enum DockNode {
    /// Split layout with a resize ratio in `(0.0, 1.0)`.
    Split {
        /// Split orientation.
        direction: SplitDirection,
        /// Portion assigned to the first child (`0.5` is equal split).
        ratio: f32,
        /// Child subtrees.
        children: [Box<DockNode>; 2],
    },
    /// Tab stack sharing one pane.
    TabGroup {
        /// Open panel tabs.
        panels: Vec<PanelId>,
        /// Index into `panels` for the active tab.
        active_tab: usize,
    },
}

/// Recoverable dock mutation failures.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum DockError {
    /// Requested panel is not present in the dock tree or floating set.
    PanelNotFound,
    /// The last root panel cannot be closed.
    CannotCloseLastRootPanel,
    /// `ratio` was outside the supported open interval.
    InvalidRatio,
    /// Panel is already floating.
    AlreadyFloating,
    /// Panel is not floating when `redock` expects it.
    NotFloating,
}

impl fmt::Display for DockError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            DockError::PanelNotFound => write!(f, "panel not found"),
            DockError::CannotCloseLastRootPanel => write!(f, "cannot close last root panel"),
            DockError::InvalidRatio => write!(f, "invalid split ratio"),
            DockError::AlreadyFloating => write!(f, "panel already floating"),
            DockError::NotFloating => write!(f, "panel is not floating"),
        }
    }
}

impl std::error::Error for DockError {}

/// Dock tree with optional floating windows.
#[derive(Debug, Clone, PartialEq)]
pub struct DockTree {
    root: DockNode,
    floating: Vec<FloatingPanel>,
    next_window_id: u64,
}

impl DockTree {
    /// Starts with a single-tab root containing `initial`.
    pub fn new(initial: PanelId) -> Self {
        Self {
            root: DockNode::TabGroup {
                panels: vec![initial],
                active_tab: 0,
            },
            floating: Vec::new(),
            next_window_id: 1,
        }
    }

    /// Borrows the layout root.
    pub fn root(&self) -> &DockNode {
        &self.root
    }

    /// Borrows floating panels.
    pub fn floating_panels(&self) -> &[FloatingPanel] {
        &self.floating
    }

    /// Splits the tab group containing `target`, placing `new_panel` beside it.
    pub fn split(
        &mut self,
        target: PanelId,
        direction: SplitDirection,
        new_panel: PanelId,
        ratio: f32,
    ) -> Result<(), DockError> {
        if !(ratio > 0.0 && ratio < 1.0) {
            return Err(DockError::InvalidRatio);
        }
        let root = std::mem::replace(
            &mut self.root,
            DockNode::TabGroup {
                panels: Vec::new(),
                active_tab: 0,
            },
        );
        self.root = split_recursive(root, target, direction, new_panel, ratio)?;
        Ok(())
    }

    /// Adds `new_panel` as a tab next to `target` in the same tab group.
    pub fn add_tab(&mut self, target: PanelId, new_panel: PanelId) -> Result<(), DockError> {
        let root = std::mem::replace(
            &mut self.root,
            DockNode::TabGroup {
                panels: Vec::new(),
                active_tab: 0,
            },
        );
        self.root = add_tab_recursive(root, target, new_panel)?;
        Ok(())
    }

    /// Detaches `panel` into a floating window record.
    pub fn float(
        &mut self,
        panel: PanelId,
        position: [i32; 2],
        size: [u32; 2],
    ) -> Result<u64, DockError> {
        if self.floating.iter().any(|f| f.panel == panel) {
            return Err(DockError::AlreadyFloating);
        }
        let root = std::mem::replace(
            &mut self.root,
            DockNode::TabGroup {
                panels: Vec::new(),
                active_tab: 0,
            },
        );
        let (new_root, removed) = extract_panel(root, panel)?;
        if !removed {
            return Err(DockError::PanelNotFound);
        }
        self.root = new_root;
        let window_id = self.next_window_id;
        self.next_window_id = self.next_window_id.saturating_add(1);
        self.floating.push(FloatingPanel {
            window_id,
            panel,
            position,
            size,
        });
        Ok(window_id)
    }

    /// Re-docks a floating `panel` into the tab group containing `target`.
    ///
    /// `zone` is accepted for API compatibility; tab insertion uses the target tab group.
    pub fn redock(
        &mut self,
        panel: PanelId,
        target: PanelId,
        zone: DockZone,
    ) -> Result<(), DockError> {
        let _ = zone;
        let idx = self
            .floating
            .iter()
            .position(|f| f.panel == panel)
            .ok_or(DockError::NotFloating)?;
        self.floating.remove(idx);
        let root = std::mem::replace(
            &mut self.root,
            DockNode::TabGroup {
                panels: Vec::new(),
                active_tab: 0,
            },
        );
        self.root = add_tab_recursive(root, target, panel)?;
        Ok(())
    }

    /// Removes `panel` from the layout (not floating). Collapses empty splits.
    pub fn close(&mut self, panel: PanelId) -> Result<(), DockError> {
        let root = std::mem::replace(
            &mut self.root,
            DockNode::TabGroup {
                panels: Vec::new(),
                active_tab: 0,
            },
        );
        let (new_root, _) = close_panel(root, panel)?;
        if tab_count(&new_root) == 0 {
            return Err(DockError::CannotCloseLastRootPanel);
        }
        self.root = new_root;
        Ok(())
    }
}

fn panel_in_tree(node: &DockNode, panel: PanelId) -> bool {
    match node {
        DockNode::TabGroup { panels, .. } => panels.contains(&panel),
        DockNode::Split { children, .. } => {
            panel_in_tree(&children[0], panel) || panel_in_tree(&children[1], panel)
        }
    }
}

fn tab_count(node: &DockNode) -> usize {
    match node {
        DockNode::TabGroup { panels, .. } => panels.len(),
        DockNode::Split { children, .. } => tab_count(&children[0]) + tab_count(&children[1]),
    }
}

fn split_recursive(
    node: DockNode,
    target: PanelId,
    direction: SplitDirection,
    new_panel: PanelId,
    ratio: f32,
) -> Result<DockNode, DockError> {
    match node {
        DockNode::TabGroup { panels, active_tab } => {
            if panels.contains(&target) {
                let left = DockNode::TabGroup {
                    panels: panels.clone(),
                    active_tab,
                };
                let right = DockNode::TabGroup {
                    panels: vec![new_panel],
                    active_tab: 0,
                };
                Ok(DockNode::Split {
                    direction,
                    ratio,
                    children: [Box::new(left), Box::new(right)],
                })
            } else {
                Err(DockError::PanelNotFound)
            }
        }
        DockNode::Split {
            direction: d,
            ratio: r,
            children,
        } => {
            let [c0, c1] = children;
            let c0 = *c0;
            let c1 = *c1;
            if panel_in_tree(&c0, target) {
                let nc0 = split_recursive(c0, target, direction, new_panel, ratio)?;
                Ok(DockNode::Split {
                    direction: d,
                    ratio: r,
                    children: [Box::new(nc0), Box::new(c1)],
                })
            } else if panel_in_tree(&c1, target) {
                let nc1 = split_recursive(c1, target, direction, new_panel, ratio)?;
                Ok(DockNode::Split {
                    direction: d,
                    ratio: r,
                    children: [Box::new(c0), Box::new(nc1)],
                })
            } else {
                Err(DockError::PanelNotFound)
            }
        }
    }
}

fn add_tab_recursive(
    node: DockNode,
    target: PanelId,
    new_panel: PanelId,
) -> Result<DockNode, DockError> {
    match node {
        DockNode::TabGroup {
            mut panels,
            active_tab: _,
        } => {
            if let Some(idx) = panels.iter().position(|p| *p == target) {
                panels.push(new_panel);
                Ok(DockNode::TabGroup {
                    panels,
                    active_tab: idx,
                })
            } else {
                Err(DockError::PanelNotFound)
            }
        }
        DockNode::Split {
            direction,
            ratio,
            children,
        } => {
            let [c0, c1] = children;
            let c0 = *c0;
            let c1 = *c1;
            if panel_in_tree(&c0, target) {
                let nc0 = add_tab_recursive(c0, target, new_panel)?;
                Ok(DockNode::Split {
                    direction,
                    ratio,
                    children: [Box::new(nc0), Box::new(c1)],
                })
            } else if panel_in_tree(&c1, target) {
                let nc1 = add_tab_recursive(c1, target, new_panel)?;
                Ok(DockNode::Split {
                    direction,
                    ratio,
                    children: [Box::new(c0), Box::new(nc1)],
                })
            } else {
                Err(DockError::PanelNotFound)
            }
        }
    }
}

fn extract_panel(node: DockNode, panel: PanelId) -> Result<(DockNode, bool), DockError> {
    match node {
        DockNode::TabGroup {
            mut panels,
            active_tab,
        } => {
            if let Some(idx) = panels.iter().position(|p| *p == panel) {
                panels.remove(idx);
                let active_tab = active_tab.min(panels.len().saturating_sub(1));
                if panels.is_empty() {
                    return Err(DockError::CannotCloseLastRootPanel);
                }
                Ok((DockNode::TabGroup { panels, active_tab }, true))
            } else {
                Ok((DockNode::TabGroup { panels, active_tab }, false))
            }
        }
        DockNode::Split {
            direction,
            ratio,
            children,
        } => {
            let [c0, c1] = children;
            let c0 = *c0;
            let c1 = *c1;
            if panel_in_tree(&c0, panel) {
                let (n0, got) = extract_panel(c0, panel)?;
                if got {
                    return Ok((collapse_if_needed(direction, ratio, n0, c1), true));
                }
                return Ok((
                    DockNode::Split {
                        direction,
                        ratio,
                        children: [Box::new(n0), Box::new(c1)],
                    },
                    false,
                ));
            }
            if panel_in_tree(&c1, panel) {
                let (n1, got) = extract_panel(c1, panel)?;
                if got {
                    return Ok((collapse_if_needed(direction, ratio, c0, n1), true));
                }
                return Ok((
                    DockNode::Split {
                        direction,
                        ratio,
                        children: [Box::new(c0), Box::new(n1)],
                    },
                    false,
                ));
            }
            Ok((
                DockNode::Split {
                    direction,
                    ratio,
                    children: [Box::new(c0), Box::new(c1)],
                },
                false,
            ))
        }
    }
}

fn collapse_if_needed(
    direction: SplitDirection,
    ratio: f32,
    left: DockNode,
    right: DockNode,
) -> DockNode {
    if tab_count(&left) == 0 {
        return right;
    }
    if tab_count(&right) == 0 {
        return left;
    }
    DockNode::Split {
        direction,
        ratio,
        children: [Box::new(left), Box::new(right)],
    }
}

fn close_panel(node: DockNode, panel: PanelId) -> Result<(DockNode, bool), DockError> {
    extract_panel(node, panel)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tc_15_1_1_1_dock_split_horizontal() {
        let a = PanelId(1);
        let b = PanelId(2);
        let mut tree = DockTree::new(a);
        tree.split(a, SplitDirection::Horizontal, b, 0.5)
            .expect("split");
        let DockNode::Split {
            direction,
            ratio,
            children,
        } = tree.root()
        else {
            panic!("expected split root");
        };
        assert_eq!(*direction, SplitDirection::Horizontal);
        assert!((*ratio - 0.5).abs() < f32::EPSILON);
        let t0 = &*children[0];
        let t1 = &*children[1];
        assert!(matches!(
            t0,
            DockNode::TabGroup { panels, .. } if panels == &vec![a]
        ));
        assert!(matches!(
            t1,
            DockNode::TabGroup { panels, .. } if panels == &vec![b]
        ));
    }

    #[test]
    fn tc_15_1_1_2_dock_split_vertical() {
        let a = PanelId(1);
        let b = PanelId(2);
        let mut tree = DockTree::new(a);
        tree.split(a, SplitDirection::Vertical, b, 0.4).unwrap();
        let DockNode::Split { direction, .. } = tree.root() else {
            panic!();
        };
        assert_eq!(*direction, SplitDirection::Vertical);
    }

    #[test]
    fn tc_15_1_1_3_dock_add_tab() {
        let a = PanelId(1);
        let b = PanelId(2);
        let mut tree = DockTree::new(a);
        tree.add_tab(a, b).unwrap();
        let DockNode::TabGroup { panels, .. } = tree.root() else {
            panic!();
        };
        assert_eq!(panels, &vec![a, b]);
    }

    #[test]
    fn tc_15_1_1_4_dock_float_and_redock() {
        let a = PanelId(1);
        let b = PanelId(2);
        let mut tree = DockTree::new(a);
        tree.add_tab(a, b).unwrap();
        let wid = tree.float(b, [10, 20], [400, 300]).expect("float");
        assert!(wid > 0);
        assert_eq!(tree.floating_panels().len(), 1);
        tree.redock(b, a, DockZone::Center).expect("redock");
        assert!(tree.floating_panels().is_empty());
        let DockNode::TabGroup { panels, .. } = tree.root() else {
            panic!();
        };
        assert!(panels.contains(&a));
        assert!(panels.contains(&b));
    }

    #[test]
    fn tc_15_1_1_5_dock_close_panel() {
        let a = PanelId(1);
        let b = PanelId(2);
        let mut tree = DockTree::new(a);
        tree.add_tab(a, b).unwrap();
        tree.close(b).unwrap();
        let DockNode::TabGroup { panels, .. } = tree.root() else {
            panic!();
        };
        assert_eq!(panels, &vec![a]);
    }
}
