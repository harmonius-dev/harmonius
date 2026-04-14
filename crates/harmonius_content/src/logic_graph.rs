//! Logic graph hot reload with variable layout checks.

use std::collections::HashMap;

/// Serialized variable layout signature.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct LogicGraphSpec {
    /// Variable names in stable order.
    pub vars: Vec<String>,
}

/// Runtime instance values.
#[derive(Clone, Debug, Default)]
pub struct LogicGraphInstance {
    /// Variable name -> value.
    pub vars: HashMap<String, i32>,
}

/// Outcome of hot reload.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum LogicReloadEvent {
    /// Layout matched; state preserved.
    StatePreserved,
    /// Layout changed; reset to defaults.
    LogicGraphRestart,
}

/// Apply `new_spec` to `inst`, preserving compatible vars.
pub fn hot_reload_logic_graph(
    inst: &mut LogicGraphInstance,
    old: &LogicGraphSpec,
    new: &LogicGraphSpec,
    defaults: &HashMap<String, i32>,
) -> LogicReloadEvent {
    if old.vars == new.vars {
        return LogicReloadEvent::StatePreserved;
    }
    *inst = LogicGraphInstance {
        vars: defaults.clone(),
    };
    LogicReloadEvent::LogicGraphRestart
}

impl LogicGraphInstance {
    /// Read variable.
    pub fn read_var(&self, name: &str) -> Option<i32> {
        self.vars.get(name).copied()
    }
}
