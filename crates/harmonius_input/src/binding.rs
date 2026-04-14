//! Compile-time / load-time binding validation.

use crate::actions::{ActionBinding, ActionDefinition, ActionId, InputSource};
use crate::value::ActionValueType;

/// Errors returned while validating authored bindings.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum BindingError {
    /// Source value shape cannot feed the action type.
    TypeMismatch {
        /// Expected runtime type.
        action: ActionValueType,
        /// Inferred source type.
        source: ActionValueType,
    },
    /// No definition row exists for the binding's action id.
    UnknownAction(ActionId),
}

/// Validates definitions against concrete bindings.
pub struct BindingLoader;

impl BindingLoader {
    /// Validate every binding against its action definition.
    pub fn load(
        definitions: &[ActionDefinition],
        bindings: &[ActionBinding],
    ) -> Result<(), BindingError> {
        for b in bindings {
            let def = definitions
                .iter()
                .find(|d| d.id == b.action_id)
                .ok_or(BindingError::UnknownAction(b.action_id))?;
            let src_ty = source_value_type(&b.source);
            if !types_compatible(def.value_type, src_ty) {
                return Err(BindingError::TypeMismatch {
                    action: def.value_type,
                    source: src_ty,
                });
            }
        }
        Ok(())
    }
}

fn source_value_type(src: &InputSource) -> ActionValueType {
    match src {
        InputSource::Key(_) => ActionValueType::Bool,
        InputSource::MouseButton(_) => ActionValueType::Bool,
        InputSource::GamepadButton(_) => ActionValueType::Bool,
        InputSource::GamepadAxis(_) => ActionValueType::Axis1D,
        InputSource::MouseAxis(_) => ActionValueType::Axis1D,
        InputSource::GamepadStick(_) => ActionValueType::Axis2D,
        InputSource::TouchRegion(_) => ActionValueType::Axis2D,
    }
}

fn types_compatible(action: ActionValueType, source: ActionValueType) -> bool {
    match (action, source) {
        (a, s) if a == s => true,
        // Digital keys may also drive Axis1D "pressed strength" in some engines — not in this
        // plan's mismatch test.
        _ => false,
    }
}
