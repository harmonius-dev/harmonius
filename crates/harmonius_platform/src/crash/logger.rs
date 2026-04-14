//! Structured logger with ring buffer and filters (`R-14.4.4`).

use std::collections::VecDeque;
use std::fs::OpenOptions;
use std::io::Write;
use std::sync::Mutex;

use crate::filesystem::CanonicalPath;

/// Severity ordering for [`LogRecord`].
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Severity {
    /// Debug verbosity.
    Debug,
    /// Informational.
    Info,
    /// Warnings.
    Warn,
    /// Errors.
    Error,
}

/// One log line passed to [`Logger::log`].
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct LogRecord {
    /// Logical subsystem channel name.
    pub channel: &'static str,
    /// Severity of the record.
    pub severity: Severity,
    /// UTF-8 payload.
    pub message: String,
}

/// Channel-specific minimum severity map plus default.
#[derive(Clone, Debug)]
pub struct LogFilter {
    /// Per-channel minimum severities.
    pub channel_levels: Vec<(&'static str, Severity)>,
    /// Default when a channel is not listed.
    pub default_level: Severity,
}

/// Enum-dispatched sinks without trait objects.
#[derive(Clone, Debug)]
pub enum LogSink {
    /// Append UTF-8 lines to a file path.
    File {
        /// Destination path.
        path: CanonicalPath,
    },
    /// Host OS log (test harness records hits in-memory).
    Platform,
    /// Fixed-capacity ring buffer in memory.
    RingBuffer {
        /// Maximum retained records.
        capacity: usize,
    },
}

struct LoggerInner {
    filter: LogFilter,
    ring_cap: usize,
    ring: VecDeque<LogRecord>,
    sinks: Vec<LogSink>,
    platform: Vec<LogRecord>,
}

/// Multi-sink logger with channel filters.
pub struct Logger {
    inner: Mutex<LoggerInner>,
}

impl Logger {
    /// Builds a logger with the given sinks and shared ring capacity hint.
    pub fn new(filter: LogFilter, sinks: Vec<LogSink>, ring_buffer_capacity: usize) -> Self {
        let ring_cap = sinks
            .iter()
            .filter_map(|s| match s {
                LogSink::RingBuffer { capacity } => Some(*capacity),
                _ => None,
            })
            .max()
            .unwrap_or(ring_buffer_capacity)
            .max(1);
        Self {
            inner: Mutex::new(LoggerInner {
                filter,
                ring_cap,
                ring: VecDeque::new(),
                sinks,
                platform: Vec::new(),
            }),
        }
    }

    /// Records a log line if it passes the active filter.
    pub fn log(&self, record: &LogRecord) {
        let mut g = self.inner.lock().expect("logger mutex poisoned");
        if !passes(&g.filter, record) {
            return;
        }
        let sinks = g.sinks.clone();
        for sink in sinks {
            match sink {
                LogSink::RingBuffer { .. } => {
                    if g.ring.len() >= g.ring_cap {
                        g.ring.pop_front();
                    }
                    g.ring.push_back(record.clone());
                }
                LogSink::File { path } => {
                    if let Ok(mut f) = OpenOptions::new().create(true).append(true).open(
                        path.to_path_buf(),
                    ) {
                        let _ = writeln!(
                            f,
                            "{} {:?} {}",
                            record.channel, record.severity, record.message
                        );
                    }
                }
                LogSink::Platform => {
                    g.platform.push(record.clone());
                }
            }
        }
    }

    /// Flushes file sinks (best effort).
    pub fn flush(&self) {
        drop(self.inner.lock());
    }

    /// Replaces the filter at runtime.
    pub fn set_filter(&self, filter: LogFilter) {
        let mut g = self.inner.lock().expect("logger mutex poisoned");
        g.filter = filter;
    }

    /// Returns a snapshot of the ring buffer (oldest first).
    pub fn ring_snapshot(&self) -> Vec<LogRecord> {
        self.inner
            .lock()
            .expect("logger mutex poisoned")
            .ring
            .iter()
            .cloned()
            .collect()
    }

    /// Returns records routed to the platform sink.
    pub fn platform_snapshot(&self) -> Vec<LogRecord> {
        self.inner
            .lock()
            .expect("logger mutex poisoned")
            .platform
            .clone()
    }
}

fn passes(filter: &LogFilter, record: &LogRecord) -> bool {
    let min = filter
        .channel_levels
        .iter()
        .find(|(c, _)| *c == record.channel)
        .map(|(_, s)| *s)
        .unwrap_or(filter.default_level);
    record.severity >= min
}
