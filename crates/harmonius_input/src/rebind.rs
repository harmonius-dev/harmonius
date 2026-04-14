//! In-memory rebinding and collision policy (conflict / reserved keys).
//!
//! Design-time `save_bindings` / `load_bindings` and `PlatformIoBridge` live in a later
//! integration milestone; this module covers deterministic resolution only.

use crate::actions::{ActionId, ContextId, InputSource, MappingContext};
use crate::device::GamepadButton;

/// How to resolve a binding collision.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum RebindResolution {
    /// Swap sources with the incumbent binding.
    Swap,
    /// Fail if occupied.
    Fail,
}

/// Request to move one action to a new physical source.
#[derive(Clone, Debug, PartialEq)]
pub struct RebindRequest {
    /// Owning context.
    pub context_id: ContextId,
    /// Target action.
    pub action_id: ActionId,
    /// Desired new source.
    pub new_source: InputSource,
    /// Collision policy.
    pub resolution: RebindResolution,
}

/// Outcome of a rebind attempt.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum RebindResult {
    /// Applied with no collisions.
    Success,
    /// Applied and swapped sources with another action in the same context.
    Swapped,
    /// Rejected by policy.
    Rejected(RebindError),
}

/// Stable rejection reasons.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum RebindError {
    /// Source is reserved for platform / shell.
    ReservedKey,
    /// Types incompatible (defensive; bindings loader should catch earlier).
    TypeMismatch,
    /// `context_id` did not match any entry in `contexts`.
    UnknownContext,
    /// `RebindResolution::Fail` and `new_source` already bound to another action.
    SourceConflict,
}

/// Stateless rebind policy evaluator and applier.
#[derive(Clone, Debug, Default)]
pub struct RebindManager;

impl RebindManager {
    /// Apply a rebind to the first matching context in `contexts`.
    pub fn apply_request(
        contexts: &mut [MappingContext],
        req: &RebindRequest,
    ) -> RebindResult {
        if is_reserved(&req.new_source) {
            return RebindResult::Rejected(RebindError::ReservedKey);
        }
        let Some(ctx) = contexts.iter_mut().find(|c| c.id == req.context_id) else {
            return RebindResult::Rejected(RebindError::UnknownContext);
        };
        if let RebindResolution::Fail = req.resolution
            && ctx.bindings.iter().any(|b| {
                b.action_id != req.action_id && sources_equal(&b.source, &req.new_source)
            })
        {
            return RebindResult::Rejected(RebindError::SourceConflict);
        }
        if let RebindResolution::Swap = req.resolution
            && let Some((i_other, _)) = ctx
                .bindings
                .iter()
                .enumerate()
                .find(|(_, b)| sources_equal(&b.source, &req.new_source))
        {
            let Some((i_self, _)) = ctx
                .bindings
                .iter()
                .enumerate()
                .find(|(_, b)| b.action_id == req.action_id)
            else {
                return RebindResult::Rejected(RebindError::TypeMismatch);
            };
            if i_self == i_other {
                return RebindResult::Success;
            }
            let tmp = ctx.bindings[i_self].source;
            ctx.bindings[i_self].source = req.new_source;
            ctx.bindings[i_other].source = tmp;
            return RebindResult::Swapped;
        }
        if let Some(b) = ctx
            .bindings
            .iter_mut()
            .find(|b| b.action_id == req.action_id)
        {
            b.source = req.new_source;
            RebindResult::Success
        } else {
            RebindResult::Rejected(RebindError::TypeMismatch)
        }
    }
}

fn is_reserved(src: &InputSource) -> bool {
    matches!(
        src,
        InputSource::GamepadButton(GamepadButton::Guide)
            | InputSource::GamepadButton(GamepadButton::Misc)
    )
}

fn sources_equal(a: &InputSource, b: &InputSource) -> bool {
    a == b
}
