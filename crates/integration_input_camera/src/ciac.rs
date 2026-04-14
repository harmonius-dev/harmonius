//! `CameraInputAxisController` tick: action lookup, modifiers, aim, blend suppression.

use crate::action::{ActionValue, InputActionState};
use crate::aim::{aim_deflect_look_delta, AimAssistConfig, AimAssistState};
use crate::blend::BlendState;
use crate::components::{CameraInputAxisController, OrbitalFollow, PanTilt};
use crate::debug::InputCameraDebug;
use crate::modifiers::{ModifierChain, ModifierState};
use glam::{Vec2, Vec3};
use tracing::warn;

/// Advance one CIAC frame: read actions, apply modifiers, aim assist, sensitivity, write rotation.
#[allow(clippy::too_many_arguments)]
pub fn ciac_tick(
    dt: f32,
    actions: &InputActionState,
    blend: &BlendState,
    debug: &mut InputCameraDebug,
    chain: &ModifierChain,
    chain_state: &mut ModifierState,
    ciac: &mut CameraInputAxisController,
    aim_config: Option<&AimAssistConfig>,
    aim_state: Option<&mut AimAssistState>,
    camera_position: Vec3,
    camera_forward: Vec3,
    targets: &[(crate::components::Entity, Vec3)],
    pan_tilt: Option<&mut PanTilt>,
    orbital: Option<&mut OrbitalFollow>,
) {
    if blend.is_blending && ciac.suppress_during_blend {
        ciac.suppression_elapsed += dt;
        if ciac.suppression_elapsed > ciac.suppression_timeout {
            if !debug.blend_suppress_timeout_warned {
                warn!(
                    "ciac blend suppression exceeded {:.1}s; clearing suppress_during_blend",
                    ciac.suppression_timeout
                );
                debug.blend_suppress_timeout_warned = true;
            }
            ciac.suppress_during_blend = false;
            ciac.suppression_elapsed = 0.0;
        }
        return;
    }

    if !blend.is_blending {
        ciac.suppression_elapsed = 0.0;
    }

    let Some(state) = actions.get(ciac.look_action) else {
        if debug.missing_action_warned.insert(ciac.look_action) {
            warn!("ciac missing ActionState for ActionId({})", ciac.look_action.0);
        }
        return;
    };

    let axis = match state.value {
        ActionValue::Axis2D(v) => v,
        _ => {
            if debug.type_mismatch_warned.insert(ciac.look_action) {
                warn!(
                    "ciac ActionId({}) expected Axis2D, got mismatched value type",
                    ciac.look_action.0
                );
            }
            return;
        }
    };

    let shaped = chain.apply(ActionValue::Axis2D(axis), chain_state);
    let mut delta = match shaped {
        ActionValue::Axis2D(v) => v,
        _ => Vec2::ZERO,
    };

    if let (Some(cfg), Some(st)) = (aim_config, aim_state) {
        delta = aim_deflect_look_delta(cfg, st, camera_position, camera_forward, targets, delta);
    }

    let sens = ciac.sensitivity;
    let yaw_delta = delta.x * sens.x;
    let mut pitch_delta = delta.y * sens.y;
    if ciac.invert_y {
        pitch_delta = -pitch_delta;
    }

    if let Some(pt) = pan_tilt {
        pt.yaw += yaw_delta;
        pt.pitch += pitch_delta;
    } else if let Some(orb) = orbital {
        orb.horizontal += yaw_delta;
        orb.vertical += pitch_delta;
    }
}
