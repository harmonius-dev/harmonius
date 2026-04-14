//! Compile-time guards for schema authoring.

/// Reject [`crate::types::PiiClass::Sensitive`] at compile time (TC-14.5.6.2).
#[macro_export]
macro_rules! field_pii_check {
    (Sensitive) => {
        ::core::compile_error!(
            "telemetry schema fields may not use PiiClass::Sensitive; relocate the field"
        );
    };
    ($class:ident) => {};
}
