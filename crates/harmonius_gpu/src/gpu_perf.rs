//! Timestamp query resolve staging (R-2.1.12).

/// Simulated GPU timestamp query pair.
#[derive(Debug, Default)]
pub struct GpuTimestampQuery {
    begin: Option<u64>,
    end: Option<u64>,
    resolved_ns: Option<u64>,
}

impl GpuTimestampQuery {
    /// Fresh query.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Records begin counter ticks.
    pub fn begin(&mut self, ticks: u64) {
        self.begin = Some(ticks);
    }

    /// Records end counter ticks.
    pub fn end(&mut self, ticks: u64) {
        self.end = Some(ticks);
    }

    /// Resolves delta in nanoseconds when both ends are present.
    pub fn resolve(&mut self) -> Option<u64> {
        let delta = self.end?.checked_sub(self.begin?)?;
        self.resolved_ns = Some(delta);
        Some(delta)
    }

    /// Last resolved duration.
    #[must_use]
    pub fn resolved_ns(&self) -> Option<u64> {
        self.resolved_ns
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// TC-2.1.12.1 — query resolves to elapsed ticks.
    #[test]
    fn test_gpu_perf_query_resolve() {
        let mut q = GpuTimestampQuery::new();
        q.begin(100);
        q.end(250);
        assert_eq!(q.resolve(), Some(150));
        assert_eq!(q.resolved_ns(), Some(150));
    }
}
