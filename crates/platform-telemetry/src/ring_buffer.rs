//! Bounded in-memory ring with disk spill hooks (R-14.5.3).

use std::cmp::Ordering;
use std::collections::VecDeque;
use std::path::{Path, PathBuf};

use serde::{Deserialize, Serialize};

use crate::error::{BufferError, TelemetryError};
use crate::types::Scope;

/// Serialized telemetry row stored in memory or on disk.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct EventRecord {
    /// Owning schema id.
    pub schema_id: u32,
    /// Wall clock in milliseconds (test harness uses deterministic values).
    pub timestamp_ms: u64,
    /// Consent scope for the row.
    pub scope: Scope,
    /// Archived payload bytes.
    pub payload: Vec<u8>,
}

/// Fixed-capacity FIFO of [`EventRecord`] values with spill metrics.
#[derive(Debug)]
pub struct RingBuffer {
    max: usize,
    spill_threshold: usize,
    queue: VecDeque<EventRecord>,
    spill_count: u32,
}

impl RingBuffer {
    /// Create a buffer that holds at most `max` records and spills at `spill_threshold` occupancy.
    pub fn new(max: usize, spill_threshold: usize) -> Self {
        Self {
            max,
            spill_threshold,
            queue: VecDeque::new(),
            spill_count: 0,
        }
    }

    /// Convenience constructor: spill triggers at 75% of `max`.
    pub fn with_three_quarter_spill(max: usize) -> Self {
        let spill_threshold = (max * 75).div_ceil(100).max(1);
        Self::new(max, spill_threshold)
    }

    /// Current occupancy.
    pub fn len(&self) -> usize {
        self.queue.len()
    }

    /// Returns true when no records are buffered.
    pub fn is_empty(&self) -> bool {
        self.queue.is_empty()
    }

    /// Capacity bound.
    pub fn capacity(&self) -> usize {
        self.max
    }

    /// Number of spill operations executed.
    pub fn spill_count(&self) -> u32 {
        self.spill_count
    }

    /// Push a record, enforcing hard capacity and spill policy.
    pub fn push(&mut self, record: EventRecord) -> Result<(), BufferError> {
        if self.queue.len() >= self.max {
            return Err(BufferError::Full);
        }
        let before = self.queue.len();
        self.queue.push_back(record);
        let after = self.queue.len();
        if before < self.spill_threshold && after >= self.spill_threshold {
            self.spill_count = self.spill_count.saturating_add(1);
        }
        Ok(())
    }

    /// Push to the front (used when restoring a failed upload batch in FIFO order).
    pub fn push_front(&mut self, record: EventRecord) -> Result<(), BufferError> {
        if self.queue.len() >= self.max {
            return Err(BufferError::Full);
        }
        self.queue.push_front(record);
        Ok(())
    }

    /// Drain up to `max` records without partial success semantics (caller slices exact batches).
    pub fn drain_up_to(&mut self, max: usize) -> Vec<EventRecord> {
        let mut out = Vec::new();
        for _ in 0..max {
            if let Some(r) = self.queue.pop_front() {
                out.push(r);
            } else {
                break;
            }
        }
        out
    }

    /// Drain exactly `n` records or leave the buffer untouched.
    pub fn try_drain_exact(&mut self, n: usize) -> Result<Vec<EventRecord>, BufferError> {
        if self.queue.len() < n {
            return Err(BufferError::NotEnoughRecords);
        }
        Ok(self.drain_up_to(n))
    }

    /// Remove every buffered record.
    pub fn clear(&mut self) {
        self.queue.clear();
    }

    /// Keep only rows matching `keep`, preserving FIFO order among kept rows.
    pub fn retain(&mut self, mut keep: impl FnMut(&EventRecord) -> bool) {
        let mut kept = VecDeque::new();
        while let Some(row) = self.queue.pop_front() {
            if keep(&row) {
                kept.push_back(row);
            }
        }
        self.queue = kept;
    }

    /// Snapshot for equality checks in tests.
    pub fn snapshot(&self) -> Vec<EventRecord> {
        self.queue.iter().cloned().collect()
    }

    /// Write all queued records to `path` as JSON lines, then clear memory.
    pub fn flush_to_disk(&mut self, path: &Path) -> Result<(), TelemetryError> {
        if self.queue.is_empty() {
            return Ok(());
        }
        let mut bytes = Vec::new();
        for row in &self.queue {
            let line = serde_json::to_vec(row).map_err(|e| TelemetryError::Io(e.to_string()))?;
            bytes.extend_from_slice(&line);
            bytes.push(b'\n');
        }
        std::fs::write(path, bytes).map_err(|e| TelemetryError::Io(e.to_string()))?;
        self.queue.clear();
        Ok(())
    }
}

/// Load JSON-lines disk batches back into memory.
pub fn read_disk_spill(path: &Path) -> Result<Vec<EventRecord>, TelemetryError> {
    let data = std::fs::read_to_string(path).map_err(|e| TelemetryError::Io(e.to_string()))?;
    let mut rows = Vec::new();
    for line in data.lines() {
        if line.is_empty() {
            continue;
        }
        let row: EventRecord =
            serde_json::from_str(line).map_err(|e| TelemetryError::Io(e.to_string()))?;
        rows.push(row);
    }
    Ok(rows)
}

fn spill_path_order(a: &Path, b: &Path) -> Ordering {
    fn numeric_stem(path: &Path) -> Option<u64> {
        path.file_stem()?.to_str()?.parse().ok()
    }
    match (numeric_stem(a), numeric_stem(b)) {
        (Some(na), Some(nb)) => na.cmp(&nb).then_with(|| a.as_os_str().cmp(b.as_os_str())),
        (Some(_), None) => Ordering::Less,
        (None, Some(_)) => Ordering::Greater,
        (None, None) => a.as_os_str().cmp(b.as_os_str()),
    }
}

/// List spill files in stable order for deterministic replay (numeric stems first).
pub fn list_spill_files(dir: &Path) -> Result<Vec<PathBuf>, TelemetryError> {
    let mut files = Vec::new();
    for entry in std::fs::read_dir(dir).map_err(|e| TelemetryError::Io(e.to_string()))? {
        let entry = entry.map_err(|e| TelemetryError::Io(e.to_string()))?;
        let path = entry.path();
        if path.is_file() {
            files.push(path);
        }
    }
    files.sort_by(|a, b| spill_path_order(a, b));
    Ok(files)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::Scope;
    use tempfile::tempdir;

    fn record(seed: u8) -> EventRecord {
        EventRecord {
            schema_id: seed as u32,
            timestamp_ms: seed as u64,
            scope: Scope::Engine,
            payload: vec![seed],
        }
    }

    #[test]
    fn test_ringbuffer_rejects_push_when_full() {
        let mut buf = RingBuffer::new(2, 2);
        buf.push(record(1)).unwrap();
        buf.push(record(2)).unwrap();
        assert_eq!(buf.push(record(3)), Err(BufferError::Full));
    }

    #[test]
    fn test_ringbuffer_spills_to_disk_at_75pct() {
        let mut buf = RingBuffer::with_three_quarter_spill(4);
        buf.push(record(1)).unwrap();
        assert_eq!(buf.spill_count(), 0);
        buf.push(record(2)).unwrap();
        buf.push(record(3)).unwrap();
        assert!(buf.spill_count() >= 1);
    }

    #[test]
    fn test_disk_spill_roundtrip_preserves_records() {
        let dir = tempdir().unwrap();
        let path = dir.path().join("batch.jsonl");
        let mut buf = RingBuffer::new(8, 8);
        let a = record(9);
        let b = record(10);
        buf.push(a.clone()).unwrap();
        buf.push(b.clone()).unwrap();
        buf.flush_to_disk(&path).unwrap();
        let loaded = read_disk_spill(&path).unwrap();
        assert_eq!(loaded, vec![a, b]);
    }

    #[test]
    fn test_offline_buffer_survives_restart() {
        let dir = tempdir().unwrap();
        let spill_path = dir.path().join("0.jsonl");
        {
            let mut buf = RingBuffer::new(4, 4);
            buf.push(record(5)).unwrap();
            buf.flush_to_disk(&spill_path).unwrap();
        }
        let files = list_spill_files(dir.path()).unwrap();
        assert_eq!(files.len(), 1);
        let replayed = read_disk_spill(&files[0]).unwrap();
        assert_eq!(replayed.len(), 1);
        assert_eq!(replayed[0].payload, vec![5]);
    }

    #[test]
    fn test_list_spill_files_orders_numeric_stems() {
        let dir = tempdir().unwrap();
        std::fs::write(dir.path().join("10.jsonl"), b"").unwrap();
        std::fs::write(dir.path().join("9.jsonl"), b"").unwrap();
        let files = list_spill_files(dir.path()).unwrap();
        assert_eq!(files.len(), 2);
        assert_eq!(files[0].file_stem().and_then(|s| s.to_str()), Some("9"));
        assert_eq!(files[1].file_stem().and_then(|s| s.to_str()), Some("10"));
    }
}
