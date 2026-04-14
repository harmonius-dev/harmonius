//! Runtime tracing toggles (never `cfg`-gated).

/// Debug switches for verbose integration tracing.
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct EventLogDebugFlags {
    /// When true, emit one trace line per `EventLog::query` call.
    pub trace_queries: bool,
    /// When true, trace threshold evaluation and firing.
    pub trace_thresholds: bool,
    /// When true, trace gossip propagation receipt.
    pub trace_propagation: bool,
    /// When true, trace AI decision write-back.
    pub trace_decisions: bool,
    /// Count of [`crate::channels::ThresholdChannel::drain_all_traced`] calls that returned
    /// events while [`Self::trace_thresholds`] was enabled (FM-5 visibility hook).
    pub threshold_nonempty_drains: u32,
}
