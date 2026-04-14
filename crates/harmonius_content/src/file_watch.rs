//! Debounced, deduplicated file watch simulation.

/// Raw filesystem events fed into the watcher.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum FileEvent {
    /// Content changed at path.
    Modified(String),
    /// Path rename.
    Renamed(String, String),
}

/// Normalized change after debounce.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum AssetChange {
    /// File modified at path.
    Modified(String),
    /// Rename within debounce window.
    Renamed {
        /// Old path.
        from: String,
        /// New path.
        to: String,
    },
}

#[derive(Clone, Debug)]
struct Pending {
    first_t: u32,
    last_t: u32,
    state: PendingState,
}

#[derive(Clone, Debug)]
enum PendingState {
    Mod(String),
    Rename(String, String),
}

/// Test double with virtual milliseconds clock.
#[derive(Debug)]
pub struct DebouncedWatcher {
    debounce_ms: u32,
    pending: Option<Pending>,
}

impl DebouncedWatcher {
    /// New watcher with debounce window.
    pub fn new(debounce_ms: u32) -> Self {
        Self {
            debounce_ms,
            pending: None,
        }
    }

    /// Feed an event at `t_ms`.
    pub fn ingest(&mut self, t_ms: u32, ev: FileEvent) {
        match ev {
            FileEvent::Modified(p) => match &mut self.pending {
                None => {
                    self.pending = Some(Pending {
                        first_t: t_ms,
                        last_t: t_ms,
                        state: PendingState::Mod(p),
                    });
                }
                Some(pe) if matches!(&pe.state, PendingState::Mod(prev) if prev == &p) => {
                    pe.last_t = t_ms;
                }
                Some(pe) => match &pe.state {
                    PendingState::Mod(prev) if prev == &p => pe.last_t = t_ms,
                    _ => {
                        self.pending = Some(Pending {
                            first_t: t_ms,
                            last_t: t_ms,
                            state: PendingState::Mod(p),
                        });
                    }
                },
            },
            FileEvent::Renamed(from, to) => {
                // If we were tracking `from` as modified, coalesce into a single rename.
                if let Some(pe) = &self.pending {
                    if let PendingState::Mod(m) = &pe.state {
                        if m == &from {
                            self.pending = Some(Pending {
                                first_t: pe.first_t,
                                last_t: t_ms,
                                state: PendingState::Rename(from, to),
                            });
                            return;
                        }
                    }
                }
                self.pending = Some(Pending {
                    first_t: t_ms,
                    last_t: t_ms,
                    state: PendingState::Rename(from, to),
                });
            }
        }
    }

    /// Emit any ready coalesced events as of `now_ms`.
    pub fn poll_changes(&mut self, now_ms: u32) -> Vec<AssetChange> {
        let mut out = Vec::new();
        if let Some(pe) = &self.pending {
            if now_ms.saturating_sub(pe.last_t) >= self.debounce_ms {
                let p = self.pending.take().unwrap();
                match p.state {
                    PendingState::Mod(s) => out.push(AssetChange::Modified(s)),
                    PendingState::Rename(from, to) => out.push(AssetChange::Renamed { from, to }),
                }
            }
        }
        out
    }
}
