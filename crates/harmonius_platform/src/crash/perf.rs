//! Per-thread style performance counters (`R-14.4.5`).

use std::collections::HashMap;
use std::sync::Mutex;

/// Static counter name identifier.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct CounterName(pub &'static str);

/// Point-in-time counter snapshot.
#[derive(Clone, Debug, PartialEq)]
pub struct Snapshot {
    /// Engine tick timestamp in nanoseconds (monotonic when available).
    pub timestamp: u64,
    /// Counter name to value pairs.
    pub values: Vec<(CounterName, f64)>,
}

/// Mutable counter bucket guarded for tests and single-threaded hosts.
pub struct PerfCounters {
    inner: Mutex<HashMap<&'static str, f64>>,
}

impl PerfCounters {
    /// Creates an empty counter set.
    pub fn new() -> Self {
        Self {
            inner: Mutex::new(HashMap::new()),
        }
    }

    /// Adds `1.0` to `name`.
    pub fn increment(&self, name: &CounterName) {
        let mut g = self.inner.lock().expect("perf mutex poisoned");
        *g.entry(name.0).or_insert(0.0) += 1.0;
    }

    /// Adds `amount` to `name`.
    pub fn increment_by(&self, name: &CounterName, amount: f64) {
        let mut g = self.inner.lock().expect("perf mutex poisoned");
        *g.entry(name.0).or_insert(0.0) += amount;
    }

    /// Sets a gauge value (last write wins).
    pub fn gauge(&self, name: &CounterName, value: f64) {
        let mut g = self.inner.lock().expect("perf mutex poisoned");
        g.insert(name.0, value);
    }

    /// Drains values into a deterministic snapshot.
    pub fn flush(&self) -> Snapshot {
        let g = self.inner.lock().expect("perf mutex poisoned");
        let mut pairs: Vec<_> = g
            .iter()
            .map(|(&k, &v)| (CounterName(k), v))
            .collect();
        pairs.sort_by(|a, b| a.0.0.cmp(b.0.0));
        Snapshot {
            timestamp: std::time::Instant::now().elapsed().as_nanos() as u64,
            values: pairs,
        }
    }
}

impl Default for PerfCounters {
    fn default() -> Self {
        Self::new()
    }
}
