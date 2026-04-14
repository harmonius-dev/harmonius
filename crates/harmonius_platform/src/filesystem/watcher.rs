//! Debounced directory watching (`R-14.6.5`).

use std::collections::HashMap;
use std::collections::HashSet;
use std::fs;
use std::time::{Duration, Instant};

use super::error::FsError;
use super::path::CanonicalPath;

/// Kind of filesystem change observed by [`FileWatcher`].
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum FileEventKind {
    /// File was created.
    Created,
    /// File contents or metadata changed.
    Modified,
    /// File was deleted.
    Deleted,
    /// File was renamed.
    Renamed {
        /// Previous path.
        from: CanonicalPath,
    },
}

/// One watcher event with a concrete path.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct FileEvent {
    /// Affected path.
    pub path: CanonicalPath,
    /// Event classification.
    pub kind: FileEventKind,
}

/// Opaque watch registration handle.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct WatchId(pub(crate) u32);

/// Pull-based stream of [`FileEvent`] values.
#[derive(Debug)]
pub struct FileEventStream {
    id: WatchId,
}

impl FileEventStream {
    /// Drains any ready events for this watch id from `watcher`.
    pub fn poll(&mut self, watcher: &mut FileWatcher) -> Vec<FileEvent> {
        watcher.poll_watch(self.id)
    }
}

/// Snapshot of a single path for diffing.
#[derive(Clone, Debug, Eq, PartialEq)]
struct Snap {
    exists: bool,
    mtime: u128,
    size: u64,
}

/// Debounced watcher using directory scans (host-neutral test surface).
#[derive(Debug)]
pub struct FileWatcher {
    debounce: Duration,
    next_id: u32,
    watches: HashMap<WatchId, (CanonicalPath, bool)>,
    state: HashMap<WatchId, HashMap<String, Snap>>,
    pending_modified: HashMap<(WatchId, String), Instant>,
    outbox: HashMap<WatchId, Vec<FileEvent>>,
}

impl FileWatcher {
    /// Creates a watcher with the given debounce interval in milliseconds.
    pub fn new(debounce_ms: u32) -> Result<Self, FsError> {
        Ok(Self {
            debounce: Duration::from_millis(u64::from(debounce_ms)),
            next_id: 1,
            watches: HashMap::new(),
            state: HashMap::new(),
            pending_modified: HashMap::new(),
            outbox: HashMap::new(),
        })
    }

    /// Registers a path to observe; `recursive` controls subdirectory scanning.
    pub fn watch(
        &mut self,
        path: &CanonicalPath,
        recursive: bool,
    ) -> Result<(WatchId, FileEventStream), FsError> {
        let id = WatchId(self.next_id);
        self.next_id = self.next_id.saturating_add(1);
        self.watches.insert(id, (path.clone(), recursive));
        self.state.insert(id, scan_tree(path, recursive)?);
        Ok((id, FileEventStream { id }))
    }

    /// Stops delivering events for `id`.
    pub fn unwatch(&mut self, id: WatchId) -> Result<(), FsError> {
        self.watches.remove(&id);
        self.state.remove(&id);
        self.outbox.remove(&id);
        self
            .pending_modified
            .retain(|(wid, _), _| *wid != id);
        Ok(())
    }

    fn poll_watch(&mut self, id: WatchId) -> Vec<FileEvent> {
        if !self.watches.contains_key(&id) {
            return Vec::new();
        }
        // Expire debounced `Modified` events from prior scans before taking a new snapshot.
        self.flush_debounced();
        self.rescan_all();
        self.flush_debounced();
        self.outbox.remove(&id).unwrap_or_default()
    }

    fn rescan_all(&mut self) {
        let ids: Vec<WatchId> = self.watches.keys().copied().collect();
        for id in ids {
            let Some((root, recursive)) = self.watches.get(&id).cloned() else {
                continue;
            };
            let Ok(new_map) = scan_tree(&root, recursive) else {
                continue;
            };
            let old = self
                .state
                .get(&id)
                .cloned()
                .unwrap_or_default();
            let mut events = Vec::new();
            let keys: HashSet<String> = old.keys().chain(new_map.keys()).cloned().collect();
            for k in keys {
                let o = old.get(&k).cloned();
                let n = new_map.get(&k).cloned();
                let Ok(path) = CanonicalPath::resolve(&k) else {
                    continue;
                };
                match (o, n) {
                    (None, Some(s)) if s.exists => events.push(FileEvent {
                        path: path.clone(),
                        kind: FileEventKind::Created,
                    }),
                    (Some(o), None) if o.exists => events.push(FileEvent {
                        path: path.clone(),
                        kind: FileEventKind::Deleted,
                    }),
                    (Some(o), Some(n))
                        if o.exists && n.exists && (o.mtime, o.size) != (n.mtime, n.size) =>
                    {
                        self.pending_modified
                            .insert((id, k.clone()), Instant::now());
                    }
                    _ => {}
                }
            }
            self.state.insert(id, new_map);
            self.outbox.entry(id).or_default().extend(events);
        }
    }

    fn flush_debounced(&mut self) {
        let now = Instant::now();
        let mut ready: Vec<(WatchId, String)> = Vec::new();
        for ((wid, k), t) in &self.pending_modified {
            if now.duration_since(*t) >= self.debounce {
                ready.push((*wid, k.clone()));
            }
        }
        for (wid, k) in ready {
            self.pending_modified.remove(&(wid, k.clone()));
            if let Ok(path) = CanonicalPath::resolve(&k) {
                self.outbox.entry(wid).or_default().push(FileEvent {
                    path,
                    kind: FileEventKind::Modified,
                });
            }
        }
    }
}

fn mtime_of(path: &std::path::Path) -> u128 {
    fs::metadata(path)
        .and_then(|m| m.modified())
        .ok()
        .and_then(|t| t.duration_since(std::time::UNIX_EPOCH).ok())
        .map(|d| d.as_nanos())
        .unwrap_or(0)
}

fn scan_tree(root: &CanonicalPath, recursive: bool) -> Result<HashMap<String, Snap>, FsError> {
    let mut map = HashMap::new();
    let mut stack = vec![root.clone()];
    while let Some(cur) = stack.pop() {
        let rd = fs::read_dir(cur.to_path_buf()).map_err(|e| FsError::Platform {
            code: e.raw_os_error().unwrap_or(-1),
            message: e.to_string(),
        })?;
        for ent in rd {
            let ent = ent.map_err(|e| FsError::Platform {
                code: e.raw_os_error().unwrap_or(-1),
                message: e.to_string(),
            })?;
            let name = ent.file_name().to_string_lossy().into_owned();
            let child = cur.join(&name)?;
            let pb = child.to_path_buf();
            let meta = ent.metadata().map_err(|e| FsError::Platform {
                code: e.raw_os_error().unwrap_or(-1),
                message: e.to_string(),
            })?;
            let key = pb.to_string_lossy().replace('\\', "/");
            if meta.is_file() {
                map.insert(
                    key,
                    Snap {
                        exists: true,
                        mtime: mtime_of(&pb),
                        size: meta.len(),
                    },
                );
            } else if meta.is_dir() && recursive {
                stack.push(child);
            }
        }
    }
    Ok(map)
}
