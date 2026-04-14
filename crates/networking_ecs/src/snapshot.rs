//! Copy-on-write indexed snapshot ring for replication history.

use std::collections::VecDeque;

use crate::ids::SequenceTick;

/// Stable identifier assigned when a snapshot is captured (survives until eviction).
#[derive(
    Clone, Copy, Debug, Eq, Hash, PartialEq, rkyv::Archive, rkyv::Deserialize, rkyv::Serialize,
)]
#[archive(check_bytes)]
#[archive_attr(derive(Debug, Eq, Hash, PartialEq))]
#[repr(transparent)]
pub struct SnapshotIndex(pub u32);

/// Either owned bytes for a chunk range or a reference to an older snapshot id.
#[derive(Clone, Debug, Eq, PartialEq, rkyv::Archive, rkyv::Deserialize, rkyv::Serialize)]
#[archive(check_bytes)]
#[archive_attr(derive(Debug, Eq, PartialEq))]
pub enum ChunkRange {
    /// This tick owns the bytes for `(archetype, chunk)`.
    Owned {
        /// Archetype id.
        archetype: u32,
        /// Chunk index.
        chunk: u32,
        /// Raw bytes.
        bytes: Vec<u8>,
    },
    /// Unchanged from an older snapshot identified by [`SnapshotIndex`].
    Shared {
        /// Archetype id.
        archetype: u32,
        /// Chunk index.
        chunk: u32,
        /// Source snapshot id still resident in the ring.
        source: SnapshotIndex,
    },
}

/// One tick of captured world ranges.
#[derive(Clone, Debug, Eq, PartialEq, rkyv::Archive, rkyv::Deserialize, rkyv::Serialize)]
#[archive(check_bytes)]
#[archive_attr(derive(Debug, Eq, PartialEq))]
pub struct Snapshot {
    /// Simulation tick for this capture.
    pub tick: SequenceTick,
    /// Chunk ranges for this tick.
    pub ranges: Vec<ChunkRange>,
}

#[derive(Debug)]
struct SnapshotEntry {
    id: SnapshotIndex,
    tick: SequenceTick,
    snapshot: Snapshot,
}

/// Optional mmap-backed scratch for large snapshot rings (integration tests use anonymous maps).
#[derive(Debug)]
pub struct MmapArena {
    map: memmap2::MmapMut,
}

impl MmapArena {
    /// Creates an anonymous read/write mapping of `len` bytes.
    pub fn anonymous(len: usize) -> std::io::Result<Self> {
        let map = memmap2::MmapOptions::new().len(len).map_anon()?;
        Ok(Self { map })
    }

    /// Mutable view of mapped bytes.
    #[must_use]
    pub fn as_mut_slice(&mut self) -> &mut [u8] {
        &mut self.map[..]
    }
}

/// Ring buffer of snapshots with optional mmap arena hook.
#[derive(Debug)]
pub struct SnapshotBuffer {
    ring: VecDeque<SnapshotEntry>,
    capacity: u32,
    next_id: u32,
    arena: Option<MmapArena>,
}

impl SnapshotBuffer {
    /// Creates a buffer with max `capacity` snapshots.
    #[must_use]
    pub fn new(capacity: u32) -> Self {
        Self {
            ring: VecDeque::new(),
            capacity,
            next_id: 0,
            arena: None,
        }
    }

    /// Attaches an mmap arena for large rings (optional integration path).
    pub fn set_arena(&mut self, arena: MmapArena) {
        self.arena = Some(arena);
    }

    /// Borrows the arena when configured.
    #[must_use]
    pub fn arena_mut(&mut self) -> Option<&mut MmapArena> {
        self.arena.as_mut()
    }

    /// Pushes `snapshot`, evicting oldest entries when over capacity.
    pub fn push(&mut self, snapshot: Snapshot) {
        let id = SnapshotIndex(self.next_id);
        self.next_id = self.next_id.wrapping_add(1);
        let tick = snapshot.tick;
        self.ring.push_back(SnapshotEntry { id, tick, snapshot });
        while self.ring.len() > self.capacity as usize {
            self.ring.pop_front();
        }
    }

    /// Number of snapshots currently retained.
    #[must_use]
    pub fn len(&self) -> usize {
        self.ring.len()
    }

    /// Returns true when no snapshots are stored.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.ring.is_empty()
    }

    /// Looks up a snapshot by tick (linear scan; small N in tests).
    #[must_use]
    pub fn get_at(&self, tick: SequenceTick) -> Option<&Snapshot> {
        self.ring
            .iter()
            .find(|entry| entry.tick == tick)
            .map(|entry| &entry.snapshot)
    }

    /// Resolves `(archetype, chunk)` bytes at `tick`, walking `Shared` links.
    #[must_use]
    pub fn resolve_chunk(
        &self,
        tick: SequenceTick,
        archetype: u32,
        chunk: u32,
    ) -> Option<ResolvedChunkBytes<'_>> {
        let snap = self.get_at(tick)?;
        let range = snap.ranges.iter().find(|r| match r {
            ChunkRange::Owned {
                archetype: a,
                chunk: c,
                ..
            } => *a == archetype && *c == chunk,
            ChunkRange::Shared {
                archetype: a,
                chunk: c,
                ..
            } => *a == archetype && *c == chunk,
        })?;
        self.resolve_range(range)
    }

    fn resolve_range<'a>(&'a self, range: &'a ChunkRange) -> Option<ResolvedChunkBytes<'a>> {
        match range {
            ChunkRange::Owned { bytes, .. } => Some(ResolvedChunkBytes {
                bytes: bytes.as_slice(),
            }),
            ChunkRange::Shared {
                source,
                archetype,
                chunk,
                ..
            } => {
                let entry = self.ring.iter().find(|e| e.id == *source)?;
                let inner = entry.snapshot.ranges.iter().find(|r| match r {
                    ChunkRange::Owned {
                        archetype: a,
                        chunk: c,
                        ..
                    } => *a == *archetype && *c == *chunk,
                    ChunkRange::Shared {
                        archetype: a,
                        chunk: c,
                        ..
                    } => *a == *archetype && *c == *chunk,
                })?;
                self.resolve_range(inner)
            }
        }
    }

    /// Returns true when a [`ChunkRange::Shared`] references an id not present in the ring.
    #[must_use]
    pub fn shared_source_missing(&self) -> bool {
        for entry in &self.ring {
            for range in &entry.snapshot.ranges {
                if let ChunkRange::Shared { source, .. } = range {
                    if !self.ring.iter().any(|e| e.id == *source) {
                        return true;
                    }
                }
            }
        }
        false
    }
}

/// Borrowed view of resolved chunk bytes.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ResolvedChunkBytes<'a> {
    /// Resolved bytes for the requested chunk.
    pub bytes: &'a [u8],
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tc_ir_4_4_5_1_retains_capacity_snapshots() {
        let mut buf = SnapshotBuffer::new(128);
        for i in 0..128 {
            buf.push(Snapshot {
                tick: SequenceTick(i),
                ranges: vec![ChunkRange::Owned {
                    archetype: 0,
                    chunk: 0,
                    bytes: vec![i as u8],
                }],
            });
        }
        assert_eq!(buf.len(), 128);
    }

    #[test]
    fn tc_ir_4_4_5_2_restore_matches_recorded_tick() {
        let mut buf = SnapshotBuffer::new(256);
        for i in 0..60 {
            buf.push(Snapshot {
                tick: SequenceTick(i),
                ranges: vec![ChunkRange::Owned {
                    archetype: 0,
                    chunk: 0,
                    bytes: vec![i as u8; 4],
                }],
            });
        }
        let resolved = buf.resolve_chunk(SequenceTick(50), 0, 0).expect("tick 50");
        assert_eq!(resolved.bytes, &[50u8, 50, 50, 50]);
    }

    #[test]
    fn tc_ir_4_4_5_3_shared_range_resolves() {
        let mut buf = SnapshotBuffer::new(128);
        buf.push(Snapshot {
            tick: SequenceTick(1),
            ranges: vec![ChunkRange::Owned {
                archetype: 0,
                chunk: 0,
                bytes: vec![9, 9, 9],
            }],
        });
        let first_id = buf.ring.front().expect("front").id;
        buf.push(Snapshot {
            tick: SequenceTick(2),
            ranges: vec![ChunkRange::Shared {
                archetype: 0,
                chunk: 0,
                source: first_id,
            }],
        });
        let owned = buf
            .resolve_chunk(SequenceTick(1), 0, 0)
            .expect("owned tick");
        let shared = buf
            .resolve_chunk(SequenceTick(2), 0, 0)
            .expect("shared tick");
        assert_eq!(owned.bytes, shared.bytes);
    }

    #[test]
    fn tc_ir_4_4_5_n1_missing_tick_returns_none() {
        let buf = SnapshotBuffer::new(8);
        assert!(buf.get_at(SequenceTick(10)).is_none());
    }

    #[test]
    fn tc_ir_4_4_5_n2_shared_source_evicted_triggers_detection() {
        let mut buf = SnapshotBuffer::new(1);
        buf.push(Snapshot {
            tick: SequenceTick(1),
            ranges: vec![ChunkRange::Owned {
                archetype: 0,
                chunk: 0,
                bytes: vec![1],
            }],
        });
        let stale = buf.ring.front().expect("front").id;
        buf.push(Snapshot {
            tick: SequenceTick(2),
            ranges: vec![ChunkRange::Shared {
                archetype: 0,
                chunk: 0,
                source: stale,
            }],
        });
        // Second push evicts tick 1; shared link should now be dangling.
        assert!(buf.shared_source_missing());
    }

    #[test]
    fn tc_ir_4_4_5_4_mmap_arena_writes_survive() {
        let mut buf = SnapshotBuffer::new(4);
        let mut arena = MmapArena::anonymous(16).expect("mmap");
        arena.as_mut_slice()[0..4].copy_from_slice(&[1, 2, 3, 4]);
        buf.set_arena(arena);
        {
            let a = buf.arena_mut().expect("arena");
            a.as_mut_slice()[4..8].copy_from_slice(&[5, 6, 7, 8]);
        }
        let a = buf.arena_mut().expect("arena");
        assert_eq!(&a.as_mut_slice()[0..8], &[1, 2, 3, 4, 5, 6, 7, 8]);
    }
}
