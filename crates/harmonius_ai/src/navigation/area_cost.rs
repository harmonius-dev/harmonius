//! Runtime area cost multipliers without rebuilding geometry.

use std::collections::HashMap;

use super::types::AreaType;

/// Global per-area-type traversal multipliers.
#[derive(Clone, Debug, Default)]
pub struct AreaCostTable {
    costs: HashMap<AreaType, f32>,
}

impl AreaCostTable {
    /// Empty table (all areas default to 1.0).
    #[must_use]
    pub fn new() -> Self {
        Self {
            costs: HashMap::new(),
        }
    }

    /// Sets the multiplier for `area`.
    pub fn set_cost(&mut self, area: AreaType, cost: f32) {
        self.costs.insert(area, cost);
    }

    /// Reads the multiplier for `area` (defaults to 1.0).
    #[must_use]
    pub fn get_cost(&self, area: AreaType) -> f32 {
        self.costs.get(&area).copied().unwrap_or(1.0)
    }
}

/// Per-agent overrides (faction routing, buffs, etc.).
#[derive(Clone, Debug, Default)]
pub struct AgentCostOverrides {
    /// Overrides keyed by area type.
    pub overrides: HashMap<AreaType, f32>,
}

/// Resolves traversal cost using optional per-agent overrides.
#[must_use]
pub fn resolve_area_cost(
    table: &AreaCostTable,
    agent_overrides: Option<&AgentCostOverrides>,
    area: AreaType,
) -> f32 {
    if let Some(ovr) = agent_overrides {
        if let Some(&cost) = ovr.overrides.get(&area) {
            return cost;
        }
    }
    table.get_cost(area)
}

#[cfg(test)]
mod tests {
    use super::*;

    /// TC-7.1.13.3 — cost table edits must not bump geometry rebuild counters (checked in tile_map tests).
    #[test]
    fn tc_7_1_13_3_cost_change_no_geometry_touch() {
        let mut t = AreaCostTable::new();
        t.set_cost(AreaType::Road, 2.0);
        assert!((t.get_cost(AreaType::Road) - 2.0).abs() < f32::EPSILON);
    }

    /// TC-7.1.13.2 — faction override beats global table.
    #[test]
    fn tc_7_1_13_2_faction_override() {
        let mut global = AreaCostTable::new();
        global.set_cost(AreaType::Ground, 1.0);
        let mut oa = AgentCostOverrides::default();
        oa.overrides.insert(AreaType::Ground, 100.0);
        let mut ob = AgentCostOverrides::default();
        ob.overrides.insert(AreaType::Ground, 0.5);
        assert!((resolve_area_cost(&global, Some(&oa), AreaType::Ground) - 100.0).abs() < 1e-5);
        assert!((resolve_area_cost(&global, Some(&ob), AreaType::Ground) - 0.5).abs() < 1e-5);
    }
}
