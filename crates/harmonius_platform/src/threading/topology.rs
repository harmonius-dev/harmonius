//! Logical CPU topology for worker sizing.

/// Identifies a logical CPU core.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct CoreId(pub u32);

/// CPU core topology distinguishing performance and efficiency cores.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CoreTopology {
    /// Performance-class cores used for worker threads by default.
    pub performance_cores: Vec<CoreId>,
    /// Efficiency-class cores (hybrid CPUs); may be empty on non-hybrid hosts.
    pub efficiency_cores: Vec<CoreId>,
}

impl CoreTopology {
    /// Detects the core topology of the current CPU.
    ///
    /// On non-hybrid CPUs every logical processor is classified as performance-class. Hybrid
    /// detection is enabled when `HARMONIUS_TEST_HYBRID_TOPOLOGY` is set (used by automated tests to
    /// model hybrid layouts without OS-specific probing).
    pub fn detect() -> Self {
        if std::env::var("HARMONIUS_TEST_HYBRID_TOPOLOGY").as_deref() == Ok("1") {
            return Self::test_hybrid_layout();
        }
        let total = std::thread::available_parallelism()
            .map(|n| n.get() as u32)
            .unwrap_or(1);
        let performance_cores = (0..total).map(CoreId).collect();
        Self {
            performance_cores,
            efficiency_cores: Vec::new(),
        }
    }

    fn test_hybrid_layout() -> Self {
        Self {
            performance_cores: vec![CoreId(0), CoreId(1), CoreId(2), CoreId(3)],
            efficiency_cores: vec![CoreId(4), CoreId(5), CoreId(6), CoreId(7)],
        }
    }

    /// Count of performance-class cores.
    pub fn performance_core_count(&self) -> u32 {
        self.performance_cores.len() as u32
    }

    /// Count of efficiency-class cores.
    pub fn efficiency_core_count(&self) -> u32 {
        self.efficiency_cores.len() as u32
    }

    /// Total logical cores represented in this topology.
    pub fn total_core_count(&self) -> u32 {
        self.performance_core_count() + self.efficiency_core_count()
    }
}
