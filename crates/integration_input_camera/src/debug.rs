//! Debug counters and one-shot warning bookkeeping.

use crate::action::ActionId;
use std::collections::HashSet;

/// Runtime-toggleable debug counters for CIAC integration.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct InputCameraDebug {
    /// Messages dropped from `raw_camera_input` overflow handling.
    pub dropped: u64,
    /// `ActionId`s that already emitted a missing-action warning.
    pub missing_action_warned: HashSet<ActionId>,
    /// `ActionId`s that already emitted a type-mismatch warning.
    pub type_mismatch_warned: HashSet<ActionId>,
    /// True after blend-suppression timeout recovery fires once.
    pub blend_suppress_timeout_warned: bool,
}
