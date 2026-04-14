//! ECS scheduler system timing (design: `EcsSystemTracker`).

use std::sync::Mutex;

/// Timing data for a single ECS system.
#[derive(Clone, Debug, PartialEq)]
pub struct SystemTiming {
    /// System name from registration.
    pub name: &'static str,
    /// Wall time in microseconds.
    pub duration_us: f64,
    /// Worker thread id (stub `0` until threading metadata lands).
    pub thread_id: u32,
}

/// Per-system timing tracked by the ECS scheduler.
#[derive(Debug)]
pub struct EcsSystemTracker {
    timings: Mutex<Vec<SystemTiming>>,
}

impl EcsSystemTracker {
    /// Empty tracker.
    #[must_use]
    pub fn new() -> Self {
        Self {
            timings: Mutex::new(Vec::new()),
        }
    }

    /// Records one system execution sample for the active frame.
    pub fn record_system(&self, system_name: &'static str, duration_us: f64) {
        let mut g = self.timings.lock().expect("poisoned lock");
        g.push(SystemTiming {
            name: system_name,
            duration_us,
            thread_id: 0,
        });
    }

    /// Returns all timings recorded since the last [`EcsSystemTracker::clear_frame`].
    #[must_use]
    pub fn system_timings(&self) -> Vec<SystemTiming> {
        self.timings.lock().expect("poisoned lock").clone()
    }

    /// Returns the top `n` systems by [`SystemTiming::duration_us`] descending.
    #[must_use]
    pub fn top_systems(&self, n: u32) -> Vec<SystemTiming> {
        let mut rows = self.system_timings();
        rows.sort_by(|a, b| {
            b.duration_us
                .partial_cmp(&a.duration_us)
                .unwrap_or(std::cmp::Ordering::Equal)
        });
        rows.truncate(usize::try_from(n).unwrap_or(usize::MAX));
        rows
    }

    /// Clears per-frame samples (call at frame boundary).
    pub fn clear_frame(&self) {
        self.timings.lock().expect("poisoned lock").clear();
    }
}

impl Default for EcsSystemTracker {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// TC-15.5.1.7 #1 — record one system, read timings.
    #[test]
    fn tc_15_5_1_7_record_system() {
        let t = EcsSystemTracker::new();
        t.record_system("physics", 150.0);
        let rows = t.system_timings();
        assert_eq!(rows.len(), 1);
        assert_eq!(rows[0].name, "physics");
        assert!((rows[0].duration_us - 150.0).abs() < f64::EPSILON);
    }

    /// TC-15.5.1.7 #2 — top three systems by duration.
    #[test]
    fn tc_15_5_1_7_top_systems() {
        let t = EcsSystemTracker::new();
        for (name, us) in [
            ("a", 10.0),
            ("b", 40.0),
            ("c", 20.0),
            ("d", 35.0),
            ("e", 5.0),
            ("f", 15.0),
            ("g", 25.0),
            ("h", 30.0),
            ("i", 12.0),
            ("j", 8.0),
        ] {
            t.record_system(name, us);
        }
        let top = t.top_systems(3);
        assert_eq!(top.len(), 3);
        assert_eq!(top[0].name, "b");
        assert_eq!(top[1].name, "d");
        assert_eq!(top[2].name, "h");
    }
}
