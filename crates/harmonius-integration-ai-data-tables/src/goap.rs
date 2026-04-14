//! GOAP action cost (`f32` only at runtime) and bake-time formula hooks.

/// Opaque formula identifier resolved at bake time.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct FormulaId(pub u32);

/// Runtime GOAP action (no `FormulaId` indirection).
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct GoapAction {
    /// Baked scalar cost.
    pub cost: f32,
}

/// Bake-time failures when resolving `FormulaId`.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum BakeError {
    /// Unknown formula id in the bake tables.
    UnknownFormula(FormulaId),
}

/// Evaluates a formula id against numeric inputs (pure, deterministic).
pub fn bake_goap_action_cost(formula: FormulaId, inputs: &[f32]) -> Result<f32, BakeError> {
    match formula.0 {
        1 => Ok(inputs.iter().copied().sum()),
        2 => Ok(inputs.iter().copied().product()),
        _ => Err(BakeError::UnknownFormula(formula)),
    }
}

/// Bakes a [`GoapAction`] from a `FormulaId` column (content-pipeline hook, IR-2.1.3).
pub fn bake_goap_action_from_formula(
    formula: FormulaId,
    inputs: &[f32],
) -> Result<GoapAction, BakeError> {
    Ok(GoapAction {
        cost: bake_goap_action_cost(formula, inputs)?,
    })
}

/// Runtime GOAP planning read of baked cost (IR-2.1.3 — no formula indirection).
pub fn read_goap_planning_cost(action: &GoapAction) -> f32 {
    action.cost
}
