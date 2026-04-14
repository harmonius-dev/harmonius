//! Directional combo evaluation (fighting-game style chains).

use crate::actions::ActionId;
use crate::device::GamepadButton;
use crate::device::Scancode;
use crate::ids::ComboTreeId;
use glam::Vec2;

/// Node id inside a `ComboTree`.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct ComboNodeId(pub u32);

/// Cardinal / diagonal directions used by combo notation.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum CardinalDirection {
    /// Down.
    Down,
    /// Down-forward diagonal.
    DownForward,
    /// Forward toward opponent.
    Forward,
    /// Back.
    Back,
    /// Up.
    Up,
    /// Up-forward.
    UpForward,
    /// Down-back.
    DownBack,
    /// Up-back.
    UpBack,
}

/// Inputs accepted by combo nodes.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum ComboInput {
    /// Directional gate.
    Direction(CardinalDirection),
    /// Gamepad face / shoulder button.
    Button(GamepadButton),
    /// Keyboard key.
    Key(Scancode),
    /// Wildcard attack button.
    AnyAttack,
    /// Wildcard direction.
    AnyDirection,
}

/// One node in a combo graph.
#[derive(Clone, Debug, PartialEq)]
pub struct ComboNode {
    /// Node id.
    pub id: ComboNodeId,
    /// Required input to traverse out of this node.
    pub input: ComboInput,
    /// Max time in seconds allowed before the next input.
    pub window: f32,
    /// Child transitions keyed by destination node id.
    pub children: smallvec::SmallVec<[ComboNodeId; 4]>,
    /// Terminal action when this node completes a chain.
    pub action: Option<ActionId>,
}

/// Full combo graph.
#[derive(Clone, Debug, PartialEq)]
pub struct ComboTree {
    /// Tree id.
    pub id: ComboTreeId,
    /// All nodes.
    pub nodes: Vec<ComboNode>,
    /// Entry node.
    pub root: ComboNodeId,
}

/// Runtime cursor into a `ComboTree`.
#[derive(Clone, Debug, PartialEq)]
pub struct ComboEvaluator {
    /// Tree id being evaluated.
    pub tree_id: ComboTreeId,
    /// Current node id.
    pub current_node: ComboNodeId,
    /// Time since last accepted transition.
    pub timer: f32,
    /// Extra leniency added to authored per-edge windows (seconds).
    pub leniency: f32,
}

impl ComboEvaluator {
    /// Start at the authored root.
    pub fn new(tree: &ComboTree, leniency: f32) -> Self {
        Self {
            tree_id: tree.id,
            current_node: tree.root,
            timer: 0.0,
            leniency,
        }
    }

    /// Feed the next discrete input; returns a resolved action id when a terminal node completes.
    pub fn advance(&mut self, input: ComboInput, dt: f32, tree: &ComboTree) -> Option<ActionId> {
        self.timer += dt;
        let node = tree.nodes.iter().find(|n| n.id == self.current_node)?;
        let max_wait = node.window + self.leniency;
        if self.timer > max_wait && self.current_node != tree.root {
            self.reset(tree);
        }
        let node = tree.nodes.iter().find(|n| n.id == self.current_node)?;
        if input_matches(node.input, input) {
            if let Some(a) = node.action {
                self.reset(tree);
                return Some(a);
            }
            if let Some(child) = node.children.first() {
                self.current_node = *child;
                self.timer = 0.0;
            }
            return None;
        }
        if self.current_node != tree.root {
            self.reset(tree);
        }
        None
    }

    /// Reset cursor to the tree root.
    pub fn reset(&mut self, tree: &ComboTree) {
        self.current_node = tree.root;
        self.timer = 0.0;
    }
}

fn input_matches(expected: ComboInput, got: ComboInput) -> bool {
    match (expected, got) {
        (ComboInput::AnyAttack, ComboInput::Button(_)) => true,
        (ComboInput::AnyDirection, ComboInput::Direction(_)) => true,
        (a, b) => a == b,
    }
}

/// Normalize stick / D-pad / keyboard into a cardinal direction for combo input.
///
/// This is a deliberately small helper for `TC-6.2.18.1`; full stick octant mapping lives in the
/// runtime mapper.
pub fn normalize_cardinal_forward_from_sources(
    stick: Vec2,
    dpad_up: bool,
    key_w: bool,
) -> CardinalDirection {
    if dpad_up || key_w || (stick.y < -0.5 && stick.x.abs() < 0.5) {
        CardinalDirection::Forward
    } else if stick.y > 0.5 && stick.x.abs() < 0.5 {
        CardinalDirection::Back
    } else if stick.x < -0.5 && stick.y.abs() < 0.5 {
        CardinalDirection::DownBack
    } else if stick.x > 0.5 && stick.y.abs() < 0.5 {
        CardinalDirection::UpForward
    } else {
        CardinalDirection::Forward
    }
}

/// Build the canonical QCF test tree: `Down -> DownForward -> Forward -> AnyAttack`.
///
/// Used by `TC-6.2.17.1` to validate `ComboEvaluator` timing and terminal resolution.
pub fn qcf_punch_tree() -> ComboTree {
    let n0 = ComboNodeId(0);
    let n1 = ComboNodeId(1);
    let n2 = ComboNodeId(2);
    let n3 = ComboNodeId(3);
    ComboTree {
        id: ComboTreeId(1),
        nodes: vec![
            ComboNode {
                id: n0,
                input: ComboInput::Direction(CardinalDirection::Down),
                window: 0.2,
                children: smallvec::smallvec![n1],
                action: None,
            },
            ComboNode {
                id: n1,
                input: ComboInput::Direction(CardinalDirection::DownForward),
                window: 0.2,
                children: smallvec::smallvec![n2],
                action: None,
            },
            ComboNode {
                id: n2,
                input: ComboInput::Direction(CardinalDirection::Forward),
                window: 0.2,
                children: smallvec::smallvec![n3],
                action: None,
            },
            ComboNode {
                id: n3,
                input: ComboInput::AnyAttack,
                window: 0.2,
                children: smallvec::SmallVec::new(),
                action: Some(ActionId(9001)),
            },
        ],
        root: n0,
    }
}
