//! Identifier types shared by runtime subsystems.

/// Stable identifier for a symbol referenced by codegen diagnostics.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct SymbolId(pub u32);
