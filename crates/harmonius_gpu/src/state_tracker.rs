//! Pipeline bind cache and redundant bind filtering (GR-2.x, R-2.1.8).

/// Tracks the last-bound pipeline id to skip redundant binds (GR-2.1).
#[derive(Debug, Default)]
pub struct StateTracker {
    current: Option<u32>,
    set_pipeline_calls: u32,
}

impl StateTracker {
    /// Creates an empty tracker.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Records a pipeline bind; increments backend call counter only on change.
    pub fn bind_pipeline(&mut self, pso: u32) {
        if self.current != Some(pso) {
            self.set_pipeline_calls = self.set_pipeline_calls.saturating_add(1);
            self.current = Some(pso);
        }
    }

    /// Number of backend `set_pipeline` calls observed.
    #[must_use]
    pub fn set_pipeline_calls(&self) -> u32 {
        self.set_pipeline_calls
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// TC-2.1.2.1 / GR-2.1 — identical consecutive binds hit the cache.
    #[test]
    fn test_state_cache_skip_redundant() {
        let mut st = StateTracker::new();
        st.bind_pipeline(1);
        st.bind_pipeline(1);
        assert_eq!(st.set_pipeline_calls(), 1);
    }

    /// TC-2.1.2.2 / GR-2.2 — alternating binds are not falsely deduplicated.
    #[test]
    fn test_pipeline_bind_cache() {
        let mut st = StateTracker::new();
        st.bind_pipeline(1);
        st.bind_pipeline(2);
        st.bind_pipeline(1);
        assert_eq!(st.set_pipeline_calls(), 3);
    }

    /// TC-2.1.8.1 — redundant bind elision for identical PSO ids.
    #[test]
    fn test_state_tracker_redundant_bind() {
        let mut st = StateTracker::new();
        for _ in 0..10 {
            st.bind_pipeline(7);
        }
        assert_eq!(st.set_pipeline_calls(), 1);
    }
}
