//! Undo stack with budgeting and optional disk spill.

use std::path::{Path, PathBuf};

use smallvec::SmallVec;
use thiserror::Error;
use tokio::fs;

use crate::command::{CommandError, EditorCommand};
use crate::ids::{CommandId, TxId};
use crate::persist::{
    self, FlatCommandRecord, PersistedWorld, SessionManifest, MANIFEST_NAME, WORLD_NAME,
};
use crate::record::CommandRecord;
use crate::selection::EntityRef;
use crate::transaction::TransactionGuard;
use crate::world::TestWorld;

/// Disk spill bookkeeping for evicted command archives.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DiskSpill {
    /// Root directory for spilled command bodies.
    pub root: PathBuf,
    /// Monotonic file name counter.
    pub next_file: u64,
}

impl DiskSpill {
    /// Creates a new spill helper rooted at `root`.
    #[must_use]
    pub fn new(root: impl Into<PathBuf>) -> Self {
        Self {
            root: root.into(),
            next_file: 0,
        }
    }

    fn next_path(&mut self) -> PathBuf {
        let name = format!("{:06}.rkyv", self.next_file);
        self.next_file += 1;
        self.root.join(name)
    }
}

/// Errors surfaced by undo stack operations.
#[derive(Debug, Error)]
pub enum UndoError {
    /// Nothing left to undo.
    #[error("nothing to undo")]
    NothingToUndo,
    /// Nothing left to redo.
    #[error("nothing to redo")]
    NothingToRedo,
    /// Command application failed while mutating the editor world.
    #[error(transparent)]
    Command(#[from] CommandError),
    /// Disk spill failed while persisting evicted history.
    #[error(transparent)]
    Io(#[from] std::io::Error),
}

#[allow(clippy::large_enum_variant)] // `Transaction` is cold compared to `Atomic` records.
#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) enum StackItem {
    Atomic(CommandRecord),
    Transaction {
        label: &'static str,
        records: Vec<CommandRecord>,
    },
}

/// Per-editor-world undo history with redo support.
#[derive(Debug)]
pub struct UndoStack {
    items: Vec<StackItem>,
    cursor: usize,
    budget_bytes: u64,
    current_bytes: u64,
    next_command_id: u64,
    next_tx: u64,
    spill: Option<DiskSpill>,
}

impl UndoStack {
    /// Creates a stack with the provided in-memory budget.
    #[must_use]
    pub fn new(budget_bytes: u64) -> Self {
        Self {
            items: Vec::new(),
            cursor: 0,
            budget_bytes,
            current_bytes: 0,
            next_command_id: 0,
            next_tx: 1,
            spill: None,
        }
    }

    /// Starts a grouped transaction that lands as a single undo step on commit.
    pub fn begin_tx<'a>(&'a mut self, label: &'static str) -> TransactionGuard<'a> {
        let tx_id = self.alloc_tx();
        TransactionGuard::new(self, label, tx_id)
    }

    fn alloc_tx(&mut self) -> TxId {
        let id = TxId(self.next_tx);
        self.next_tx += 1;
        id
    }

    /// Returns the label of the last transaction item, if any.
    #[must_use]
    pub fn last_transaction_label(&self) -> Option<&'static str> {
        self.items.iter().rev().find_map(|item| {
            if let StackItem::Transaction { label, .. } = item {
                Some(*label)
            } else {
                None
            }
        })
    }

    /// Returns the entity touches for the next undo operation, if any.
    #[must_use]
    pub fn peek_next_undo_touches(&self) -> Option<SmallVec<[EntityRef; 4]>> {
        if self.cursor == 0 {
            return None;
        }
        match &self.items[self.cursor - 1] {
            StackItem::Atomic(record) => Some(record.command.touches()),
            StackItem::Transaction { records, .. } => {
                let mut out = SmallVec::new();
                for record in records {
                    for touch in record.command.touches() {
                        if !out.contains(&touch) {
                            out.push(touch);
                        }
                    }
                }
                Some(out)
            }
        }
    }

    /// Returns `true` when any touched entity overlaps between two touch lists.
    #[must_use]
    pub fn touches_overlap(a: &[EntityRef], b: &[EntityRef]) -> bool {
        a.iter().any(|entity| b.contains(entity))
    }

    /// Attaches disk spill metadata for budget overflows.
    pub fn set_disk_spill(&mut self, spill: DiskSpill) {
        self.spill = Some(spill);
    }

    /// Total bytes currently attributed to in-memory history.
    #[must_use]
    pub const fn current_bytes(&self) -> u64 {
        self.current_bytes
    }

    /// Number of stack items currently considered applied for undo/redo.
    #[must_use]
    pub const fn cursor(&self) -> usize {
        self.cursor
    }

    fn alloc_id(&mut self) -> CommandId {
        let id = CommandId(self.next_command_id);
        self.next_command_id += 1;
        id
    }

    /// Allocates a fresh [`CommandId`] for constructing [`CommandRecord`] values.
    #[must_use]
    pub fn allocate_command_id(&mut self) -> CommandId {
        self.alloc_id()
    }

    fn item_bytes(item: &StackItem) -> u64 {
        match item {
            StackItem::Atomic(record) => u64::from(record.memory_bytes),
            StackItem::Transaction { records, .. } => records
                .iter()
                .map(|record| u64::from(record.memory_bytes))
                .sum(),
        }
    }

    fn account_push(&mut self, item: &StackItem) {
        self.current_bytes += Self::item_bytes(item);
    }

    /// Pushes a command record, applying it to `world` and truncating redo history when needed.
    pub fn push_record(
        &mut self,
        world: &mut TestWorld,
        record: CommandRecord,
    ) -> Result<(), UndoError> {
        self.items.truncate(self.cursor);
        self.recompute_bytes_after_truncate();

        record.command.apply(world)?;
        world.selection = record.after_selection.clone();

        let item = StackItem::Atomic(record);
        self.account_push(&item);
        self.items.push(item);
        self.cursor = self.items.len();
        Ok(())
    }

    fn recompute_bytes_after_truncate(&mut self) {
        self.current_bytes = self.items.iter().map(Self::item_bytes).sum();
    }

    fn clone_atomic_only(&self) -> Result<Vec<CommandRecord>, UndoError> {
        let mut out = Vec::new();
        for item in &self.items {
            match item {
                StackItem::Atomic(record) => out.push(record.clone()),
                StackItem::Transaction { .. } => {
                    return Err(std::io::Error::other("session save requires atomic-only stack").into());
                }
            }
        }
        Ok(out)
    }

    /// Materializes the executable command, loading from disk for spilled archives.
    pub(crate) fn materialize_command(&self, record: &CommandRecord) -> Result<EditorCommand, UndoError> {
        match &record.command {
            EditorCommand::SpilledArchive { file_name, .. } => {
                let root = self
                    .spill
                    .as_ref()
                    .map(|s| s.root.as_path())
                    .ok_or_else(|| std::io::Error::other("missing spill root for spilled command"))?;
                let flat = persist::read_flat_record(&root.join(file_name))?;
                Ok(flat.command)
            }
            other => Ok(other.clone()),
        }
    }

    /// Saves an atomic-only stack plus a minimal world snapshot into `root`.
    pub fn save_atomic_session(&self, world: &TestWorld, root: &Path) -> Result<(), UndoError> {
        let records: Vec<FlatCommandRecord> = self
            .clone_atomic_only()?
            .iter()
            .map(FlatCommandRecord::from)
            .collect();
        let manifest = SessionManifest {
            cursor: u32::try_from(self.cursor).map_err(|_| {
                std::io::Error::other("cursor does not fit manifest representation")
            })?,
            budget_bytes: self.budget_bytes,
            next_command_id: self.next_command_id,
            next_tx: self.next_tx,
            records,
        };
        std::fs::create_dir_all(root)?;
        persist::write_manifest(&root.join(MANIFEST_NAME), &manifest)?;
        let world_blob = rkyv::to_bytes::<rkyv::rancor::Error>(&PersistedWorld::from(world))
            .map_err(|e| std::io::Error::other(format!("{e:?}")))?;
        std::fs::write(root.join(WORLD_NAME), world_blob)?;
        Ok(())
    }

    /// Loads stack and world written by [`Self::save_atomic_session`].
    pub fn load_atomic_session(root: &Path) -> Result<(Self, TestWorld), UndoError> {
        let manifest = persist::read_manifest(&root.join(MANIFEST_NAME))?;
        let world_bytes = std::fs::read(root.join(WORLD_NAME))?;
        let persisted = rkyv::from_bytes::<PersistedWorld, rkyv::rancor::Error>(&world_bytes)
            .map_err(|e| std::io::Error::other(format!("{e:?}")))?;
        let world = TestWorld {
            counter: persisted.counter,
            ..Default::default()
        };

        let mut stack = UndoStack::new(manifest.budget_bytes);
        stack.next_command_id = manifest.next_command_id;
        stack.next_tx = manifest.next_tx;
        stack.cursor = usize::try_from(manifest.cursor)
            .map_err(|_| std::io::Error::other("invalid cursor in manifest"))?;
        for flat in manifest.records {
            let record = CommandRecord::try_from(flat)?;
            stack.items.push(StackItem::Atomic(record));
        }
        stack.recompute_bytes_after_truncate();
        Ok((stack, world))
    }

    /// Pushes a grouped transaction as a single undo step.
    pub fn push_transaction(
        &mut self,
        label: &'static str,
        records: Vec<CommandRecord>,
    ) -> Result<(), UndoError> {
        self.items.truncate(self.cursor);
        self.recompute_bytes_after_truncate();
        let item = StackItem::Transaction { label, records };
        self.account_push(&item);
        self.items.push(item);
        self.cursor = self.items.len();
        Ok(())
    }

    /// Undoes the most recent command or transaction.
    pub fn undo(&mut self, world: &mut TestWorld) -> Result<(), UndoError> {
        if self.cursor == 0 {
            return Err(UndoError::NothingToUndo);
        }
        self.cursor -= 1;
        let item = &self.items[self.cursor];
        match item {
            StackItem::Atomic(record) => {
                world.selection = record.before_selection.clone();
                let cmd = self.materialize_command(record)?;
                cmd.revert(world)?;
            }
            StackItem::Transaction { records, .. } => {
                for record in records.iter().rev() {
                    let cmd = self.materialize_command(record)?;
                    cmd.revert(world)?;
                }
                if let Some(first) = records.first() {
                    world.selection = first.before_selection.clone();
                }
            }
        }
        Ok(())
    }

    /// Redoes the next command or transaction.
    pub fn redo(&mut self, world: &mut TestWorld) -> Result<(), UndoError> {
        if self.cursor >= self.items.len() {
            return Err(UndoError::NothingToRedo);
        }
        let item = &self.items[self.cursor];
        match item {
            StackItem::Atomic(record) => {
                let cmd = self.materialize_command(record)?;
                cmd.apply(world)?;
                world.selection = record.after_selection.clone();
            }
            StackItem::Transaction { records, .. } => {
                for record in records {
                    let cmd = self.materialize_command(record)?;
                    cmd.apply(world)?;
                }
                if let Some(last) = records.last() {
                    world.selection = last.after_selection.clone();
                }
            }
        }
        self.cursor += 1;
        Ok(())
    }

    /// Clears all undo history and disables redo.
    pub fn clear(&mut self) {
        self.items.clear();
        self.cursor = 0;
        self.current_bytes = 0;
    }

    /// Evicts the oldest in-memory history to disk until within the configured budget.
    pub async fn maintain_budget(&mut self) -> Result<(), UndoError> {
        while self.current_bytes > self.budget_bytes && !self.items.is_empty() {
            let spill = self
                .spill
                .as_mut()
                .ok_or_else(|| std::io::Error::other("missing spill root"))?;
            let Some(idx) = self.items.iter().position(|item| match item {
                StackItem::Atomic(record) => {
                    !matches!(record.command, EditorCommand::SpilledArchive { .. })
                }
                StackItem::Transaction { .. } => true,
            }) else {
                break;
            };
            let spill_record = match &self.items[idx] {
                StackItem::Atomic(record) => record.clone(),
                StackItem::Transaction { .. } => {
                    return Err(std::io::Error::other(
                        "transaction spill unsupported in this build",
                    )
                    .into());
                }
            };
            let path = spill.next_path();
            if let Some(parent) = path.parent() {
                fs::create_dir_all(parent).await?;
            }
            let flat = FlatCommandRecord::from(&spill_record);
            persist::write_flat_record(&path, &flat).await?;
            let file_name = path
                .file_name()
                .and_then(|n| n.to_str())
                .ok_or_else(|| std::io::Error::other("spill path missing file name"))?
                .to_string();
            let serialized_len = u32::try_from(
                std::fs::metadata(&path).map(|m| m.len()).unwrap_or(0),
            )
            .unwrap_or(u32::MAX);
            let stub_cmd = EditorCommand::SpilledArchive {
                file_name,
                byte_len: serialized_len,
            };
            let stub_record = CommandRecord::new(
                spill_record.id,
                spill_record.tx_id,
                spill_record.author,
                stub_cmd,
                spill_record.before_selection.clone(),
                spill_record.after_selection.clone(),
                spill_record.committed_at,
            );
            let old_bytes = u64::from(spill_record.memory_bytes);
            let new_bytes = u64::from(stub_record.memory_bytes);
            self.current_bytes = self.current_bytes.saturating_sub(old_bytes) + new_bytes;
            self.items[idx] = StackItem::Atomic(stub_record);
        }
        Ok(())
    }

    /// Reloads a spilled command from disk for inspection or tests.
    pub async fn load_spilled_command(path: &Path) -> Result<EditorCommand, UndoError> {
        let bytes = fs::read(path).await?;
        if let Ok(flat) =
            rkyv::from_bytes::<FlatCommandRecord, rkyv::rancor::Error>(&bytes)
        {
            return Ok(flat.command);
        }
        let command = rkyv::from_bytes::<EditorCommand, rkyv::rancor::Error>(&bytes)
            .map_err(|e| std::io::Error::other(format!("{e:?}")))?;
        Ok(command)
    }
}
