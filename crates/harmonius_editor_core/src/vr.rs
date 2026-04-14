//! VR editor mode helpers (`TC-15.1.9.1`, `TC-15.16.*`).

use glam::{Quat, Vec3};
use smallvec::SmallVec;
use std::fmt;

/// Left or right tracked side.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Hand {
    /// Non-dominant side.
    Left,
    /// Dominant side.
    Right,
}

/// Simplified OpenXR-style controller digital/analog snapshot.
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct ControllerState {
    /// Primary button pressed.
    pub primary: bool,
    /// Secondary button pressed.
    pub secondary: bool,
    /// Analog stick X axis.
    pub stick_x: f32,
    /// Analog stick Y axis.
    pub stick_y: f32,
}

/// Recognized hand poses.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub enum HandGesture {
    /// No confident pose.
    #[default]
    None,
    /// Thumb and index pinch.
    Pinch,
    /// Power grasp.
    Grab,
    /// Index extended pointing pose.
    Point,
    /// Open palm facing outward.
    OpenPalm,
    /// Closed fist.
    Fist,
}

/// Per-hand tracking snapshot.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct HandState {
    /// Recognized pose for this hand.
    pub gesture: HandGesture,
}

impl Default for HandState {
    fn default() -> Self {
        Self {
            gesture: HandGesture::None,
        }
    }
}

/// High-level editor actions emitted from VR adapters.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum EditorAction {
    /// Menu button on either controller.
    MenuPressed,
    /// Primary select / click.
    Select,
    /// System-level recenter request.
    Recenter,
}

/// VR-specific failures.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum VrError {
    /// HMD or runtime not ready.
    NotReady,
}

impl fmt::Display for VrError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            VrError::NotReady => write!(f, "vr runtime not ready"),
        }
    }
}

impl std::error::Error for VrError {}

/// Desktop vs VR transitions.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum VrModeState {
    /// Standard desktop editing.
    Desktop,
    /// Entering VR session.
    TransitionToVr,
    /// Active stereo submission.
    VrActive,
    /// Returning to desktop.
    TransitionToDesktop,
}

/// Tracks VR session state for the editor shell.
#[derive(Debug)]
pub struct VrModeManager {
    /// Current high-level mode.
    pub state: VrModeState,
    ready: bool,
}

impl VrModeManager {
    /// Starts on the desktop path.
    pub fn new() -> Self {
        Self {
            state: VrModeState::Desktop,
            ready: true,
        }
    }

    /// Marks whether the OpenXR session is ready (test hook).
    pub fn set_runtime_ready(&mut self, ready: bool) {
        self.ready = ready;
    }

    /// Requests a VR session.
    pub fn enter_vr(&mut self) -> Result<(), VrError> {
        if !self.ready {
            return Err(VrError::NotReady);
        }
        self.state = VrModeState::TransitionToVr;
        self.state = VrModeState::VrActive;
        Ok(())
    }

    /// Leaves VR and restores desktop mode.
    pub fn exit_vr(&mut self) -> Result<(), VrError> {
        self.state = VrModeState::TransitionToDesktop;
        self.state = VrModeState::Desktop;
        Ok(())
    }

    /// `true` while stereo submission is active.
    pub fn is_vr_active(&self) -> bool {
        matches!(self.state, VrModeState::VrActive)
    }

    /// Per-frame VR bookkeeping (placeholder for motion-to-photon tuning).
    pub fn frame_update(&mut self, _delta_time: f32) {}
}

impl Default for VrModeManager {
    fn default() -> Self {
        Self::new()
    }
}

/// Controller-first vs hand-tracked input.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum VrInputMode {
    /// Motion controller poses and buttons.
    MotionControllers,
    /// Hand mesh / skeleton gestures.
    HandTracking,
}

/// Maps raw XR state into editor actions.
#[derive(Debug)]
pub struct VrInputAdapter {
    /// Active mapping mode.
    pub mode: VrInputMode,
}

impl VrInputAdapter {
    /// Builds an adapter in `mode`.
    pub fn new(mode: VrInputMode) -> Self {
        Self { mode }
    }

    /// Maps controller digital state to actions.
    pub fn process_input(
        &mut self,
        left: &ControllerState,
        right: &ControllerState,
    ) -> SmallVec<[EditorAction; 4]> {
        let mut out = SmallVec::new();
        if self.mode != VrInputMode::MotionControllers {
            return out;
        }
        if left.primary {
            out.push(EditorAction::MenuPressed);
        }
        if right.primary {
            out.push(EditorAction::Select);
        }
        if left.secondary && right.secondary {
            out.push(EditorAction::Recenter);
        }
        out
    }

    /// Maps hand gestures to editor actions.
    pub fn process_hands(
        &mut self,
        left: &HandState,
        right: &HandState,
    ) -> SmallVec<[EditorAction; 4]> {
        let mut out = SmallVec::new();
        if self.mode != VrInputMode::HandTracking {
            return out;
        }
        if matches!(left.gesture, HandGesture::Pinch) {
            out.push(EditorAction::Select);
        }
        if matches!(right.gesture, HandGesture::Grab) {
            out.push(EditorAction::MenuPressed);
        }
        out
    }

    /// Aim ray for laser-pointer style UI hits.
    pub fn pointing_ray(&self) -> (Vec3, Vec3) {
        (Vec3::ZERO, -Vec3::Z)
    }
}

/// Spatial anchor modes for floating VR panels.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum VrPanelAnchor {
    /// Fixed transform in the tracked playspace.
    WorldFixed(Vec3, Quat),
    /// Attached to a tracked hand with local offset.
    HandAttached(Hand, Vec3),
    /// Follows the HMD with a head-locked offset.
    HeadLocked(Vec3),
    /// Detached spatial panel with independent drag.
    Floating,
}

/// Identifier for a spawned VR panel.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct VrPanelId(pub u64);

/// Floating UI panel in VR.
#[derive(Clone, Debug, PartialEq)]
pub struct VrPanel {
    /// Stable id.
    pub id: VrPanelId,
    /// Anchor mode for this frame.
    pub anchor: VrPanelAnchor,
}

/// Tracks spawned VR UI panels.
#[derive(Debug, Default)]
pub struct VrPanelSystem {
    panels: Vec<VrPanel>,
    next: u64,
}

impl VrPanelSystem {
    /// Spawns a panel with `anchor`.
    pub fn spawn_panel(&mut self, anchor: VrPanelAnchor) -> VrPanelId {
        let id = VrPanelId(self.next);
        self.next = self.next.saturating_add(1);
        self.panels.push(VrPanel { id, anchor });
        id
    }

    /// Removes a panel by id.
    pub fn dismiss_panel(&mut self, id: VrPanelId) {
        self.panels.retain(|p| p.id != id);
    }

    /// Borrows live panels.
    pub fn panels(&self) -> &[VrPanel] {
        &self.panels
    }
}

/// Remote collaborator avatar record.
#[derive(Clone, Debug, PartialEq)]
pub struct VrAvatar {
    /// Session user name.
    pub name: String,
    /// Hidden avatars are not submitted to the renderer.
    pub visible: bool,
}

/// Toggles avatar visibility for co-presence sessions.
#[derive(Debug, Default)]
pub struct VrAvatarSystem {
    avatars: Vec<VrAvatar>,
}

impl VrAvatarSystem {
    /// Adds or updates an avatar by `name`.
    pub fn upsert(&mut self, name: &str, visible: bool) {
        if let Some(a) = self.avatars.iter_mut().find(|a| a.name == name) {
            a.visible = visible;
        } else {
            self.avatars.push(VrAvatar {
                name: name.to_string(),
                visible,
            });
        }
    }

    /// Sets visibility for `name`.
    pub fn set_visible(&mut self, name: &str, visible: bool) {
        self.upsert(name, visible);
    }

    /// Borrows avatar rows.
    pub fn avatars(&self) -> &[VrAvatar] {
        &self.avatars
    }
}

/// Follow-mode subject.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub enum FollowTarget {
    /// Follow disabled.
    #[default]
    None,
    /// Follow another user session id.
    User(u64),
    /// Follow an AI agent id.
    AiAgent(u64),
}

/// Invalid follow requests.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FollowError {
    /// Cannot attach follow to an empty target.
    InvalidTarget,
}

impl fmt::Display for FollowError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            FollowError::InvalidTarget => write!(f, "invalid follow target"),
        }
    }
}

impl std::error::Error for FollowError {}

/// Simple follow attachment state machine.
#[derive(Debug)]
pub struct VrFollowMode {
    /// Active follow target.
    pub target: FollowTarget,
    attached: bool,
}

impl Default for VrFollowMode {
    fn default() -> Self {
        Self {
            target: FollowTarget::None,
            attached: false,
        }
    }
}

impl VrFollowMode {
    /// Attaches follow mode to `target`.
    pub fn follow(&mut self, target: FollowTarget) -> Result<(), FollowError> {
        if matches!(target, FollowTarget::None) {
            return Err(FollowError::InvalidTarget);
        }
        self.target = target;
        self.attached = true;
        Ok(())
    }

    /// Breaks follow attachment.
    pub fn break_follow(&mut self) {
        self.target = FollowTarget::None;
        self.attached = false;
    }

    /// Whether follow is actively latched.
    pub fn is_attached(&self) -> bool {
        self.attached
    }
}

/// Adaptive quality controller for VR frame budgets.
#[derive(Debug, Clone)]
pub struct VrPerfBudget {
    /// Target frame time in milliseconds.
    pub frame_budget_ms: f32,
    /// Requested frames per second.
    pub target_fps: u32,
    /// Internal quality scalar (0 = min, 1 = max).
    pub quality: f32,
}

impl VrPerfBudget {
    /// Starts at 90 Hz with a matching millisecond budget.
    pub fn for_90_fps() -> Self {
        Self {
            frame_budget_ms: 1000.0 / 90.0,
            target_fps: 90,
            quality: 1.0,
        }
    }

    /// Lowers quality when frames exceed the budget, raises when headroom exists.
    pub fn adjust_quality(&mut self, frame_time_ms: f32) {
        if frame_time_ms > self.frame_budget_ms * 1.1 {
            self.quality = (self.quality - 0.05).max(0.0);
        } else if frame_time_ms < self.frame_budget_ms * 0.85 {
            self.quality = (self.quality + 0.02).min(1.0);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tc_15_1_9_1_vr_mode_enter_exit() {
        let mut m = VrModeManager::new();
        m.enter_vr().unwrap();
        assert!(m.is_vr_active());
        m.exit_vr().unwrap();
        assert!(!m.is_vr_active());
    }

    #[test]
    fn tc_15_16_1_1_controller_mapping() {
        let mut a = VrInputAdapter::new(VrInputMode::MotionControllers);
        let left = ControllerState {
            primary: true,
            ..Default::default()
        };
        let right = ControllerState {
            primary: true,
            secondary: true,
            ..Default::default()
        };
        let out = a.process_input(&left, &right);
        assert!(out.contains(&EditorAction::MenuPressed));
        assert!(out.contains(&EditorAction::Select));
    }

    #[test]
    fn tc_15_16_1_2_hand_gesture_recognition() {
        let mut a = VrInputAdapter::new(VrInputMode::HandTracking);
        let left = HandState {
            gesture: HandGesture::Pinch,
        };
        let right = HandState {
            gesture: HandGesture::Grab,
        };
        let out = a.process_hands(&left, &right);
        assert!(out.contains(&EditorAction::Select));
        assert!(out.contains(&EditorAction::MenuPressed));
    }

    #[test]
    fn tc_15_16_2_1_vr_panel_spawn_dismiss() {
        let mut s = VrPanelSystem::default();
        let id = s.spawn_panel(VrPanelAnchor::Floating);
        assert_eq!(s.panels().len(), 1);
        s.dismiss_panel(id);
        assert!(s.panels().is_empty());
    }

    #[test]
    fn tc_15_16_2_2_vr_panel_anchor_modes() {
        let mut s = VrPanelSystem::default();
        let _ = s.spawn_panel(VrPanelAnchor::WorldFixed(Vec3::ONE, Quat::IDENTITY));
        let _ = s.spawn_panel(VrPanelAnchor::HandAttached(Hand::Right, Vec3::ZERO));
        assert_eq!(s.panels().len(), 2);
    }

    #[test]
    fn tc_15_16_3_1_vr_avatar_visibility() {
        let mut sys = VrAvatarSystem::default();
        sys.set_visible("ada", true);
        sys.set_visible("ada", false);
        assert!(!sys.avatars()[0].visible);
    }

    #[test]
    fn tc_15_16_4_1_follow_mode_attach() {
        let mut f = VrFollowMode::default();
        f.follow(FollowTarget::User(7)).unwrap();
        assert!(f.is_attached());
    }

    #[test]
    fn tc_15_16_4_2_follow_mode_break() {
        let mut f = VrFollowMode::default();
        f.follow(FollowTarget::AiAgent(3)).unwrap();
        f.break_follow();
        assert!(!f.is_attached());
    }

    #[test]
    fn tc_15_16_5_1_vr_perf_budget_adjust() {
        let mut b = VrPerfBudget::for_90_fps();
        b.adjust_quality(b.frame_budget_ms * 2.0);
        assert!(b.quality < 1.0);
    }
}
