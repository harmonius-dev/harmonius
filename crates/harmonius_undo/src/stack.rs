//! Undo stack with budgeting and optional disk spill.

use std::path::{Path, PathBuf};

use smallvec::SmallVec;
use thiserror::Error;
use tokio::fs;
use tokio::io::AsyncWriteExt;

use crate::command::{CommandError, EditorCommand};
use crate::ids::{CommandId, TxId};
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
enum StackItem {
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

    fn account_pop(&mut self, item: &StackItem) {
        self.current_bytes = self.current_bytes.saturating_sub(Self::item_bytes(item));
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
                record.command.revert(world)?;
            }
            StackItem::Transaction { records, .. } => {
                for record in records.iter().rev() {
                    record.command.revert(world)?;
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
                record.command.apply(world)?;
                world.selection = record.after_selection.clone();
            }
            StackItem::Transaction { records, .. } => {
                for record in records {
                    record.command.apply(world)?;
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
            let path = {
                let spill = self
                    .spill
                    .as_mut()
                    .ok_or_else(|| std::io::Error::other("missing spill root"))?;
                spill.next_path()
            };
            let removed = self.items.remove(0);
            self.account_pop(&removed);
            self.cursor = self.cursor.saturating_sub(1);
            if let Some(parent) = path.parent() {
                fs::create_dir_all(parent).await?;
            }

            let command = match &removed {
                StackItem::Atomic(record) => &record.command,
                StackItem::Transaction { .. } => {
                    return Err(std::io::Error::other(
                        "transaction spill unsupported in this build",
                    )
                    .into());
                }
            };

            let bytes = rkyv::to_bytes::<rkyv::rancor::Error>(command)
                .map_err(|e| std::io::Error::other(format!("{e:?}")))?;
            let mut file = fs::File::create(&path).await?;
            file.write_all(&bytes).await?;
            file.sync_all().await?;
        }
        Ok(())
    }

    /// Reloads a spilled command from disk for inspection or tests.
    pub async fn load_spilled_command(path: &Path) -> Result<EditorCommand, UndoError> {
        let bytes = fs::read(path).await?;
        let command = rkyv::from_bytes::<EditorCommand, rkyv::rancor::Error>(&bytes)
            .map_err(|e| std::io::Error::other(format!("{e:?}")))?;
        Ok(command)
    }
}
