//! Named counters shared by profiler, HUD, and telemetry (R-14.4.5).

use std::collections::HashMap;

use crate::error::CounterError;

/// Registry of uniquely named integer counters.
#[derive(Debug, Default, Clone)]
pub struct CounterRegistry {
    // Cold-path editor and diagnostics map; not used on deterministic simulation hot paths.
    counters: HashMap<String, u64>,
}

impl CounterRegistry {
    /// Empty registry.
    pub fn new() -> Self {
        Self {
            counters: HashMap::new(),
        }
    }

    /// Register a counter initialized to zero. Duplicate names return an error.
    pub fn register(&mut self, name: &str) -> Result<(), CounterError> {
        if self.counters.contains_key(name) {
            return Err(CounterError::DuplicateName);
        }
        self.counters.insert(name.to_owned(), 0);
        Ok(())
    }

    /// Set a counter value for a previously registered name.
    pub fn set(&mut self, name: &str, value: u64) -> Result<(), CounterError> {
        let slot = self
            .counters
            .get_mut(name)
            .ok_or(CounterError::UnknownName)?;
        *slot = value;
        Ok(())
    }

    /// Read the latest value for a registered counter.
    pub fn get(&self, name: &str) -> Option<u64> {
        self.counters.get(name).copied()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_counter_registration_unique_names() {
        let mut reg = CounterRegistry::new();
        reg.register("fps").unwrap();
        assert_eq!(reg.register("fps"), Err(CounterError::DuplicateName));
    }

    #[test]
    fn test_counter_read_returns_current_value() {
        let mut reg = CounterRegistry::new();
        reg.register("frames").unwrap();
        reg.set("frames", 42).unwrap();
        assert_eq!(reg.get("frames"), Some(42));
    }
}
