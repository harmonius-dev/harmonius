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
}
