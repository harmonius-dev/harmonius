//! Session persistence for atomic undo stacks (see `docs/design/tools/undo-redo.md`).
#![allow(missing_docs)] // rkyv-generated companion types omit doc comments.

use std::path::Path;

use rkyv::{Archive, Deserialize, Serialize};

use crate::command::EditorCommand;
use crate::ids::{CommandId, TxId, UserId};
use crate::record::CommandRecord;
use crate::selection::{EntityRef, SelectionSnapshot};
use crate::stack::UndoError;
use crate::world::TestWorld;

/// Flattened command record for rkyv disk and manifest encoding.
#[derive(Archive, Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[rkyv(compare(PartialEq), derive(Debug, Eq, PartialEq))]
pub struct FlatCommandRecord {
    /// Command identifier.
    pub id: u64,
    /// Optional transaction id.
    pub tx_id: Option<u64>,
    /// Author id.
    pub author: u64,
    /// Payload.
    pub command: EditorCommand,
    /// Selection entities before apply.
    pub before_entities: Vec<EntityRef>,
    /// Selection entities after apply.
    pub after_entities: Vec<EntityRef>,
    /// Commit tick.
    pub committed_at: u64,
}

impl From<&CommandRecord> for FlatCommandRecord {
    fn from(record: &CommandRecord) -> Self {
        Self {
            id: record.id.0,
            tx_id: record.tx_id.map(|t| t.0),
            author: record.author.0,
            command: record.command.clone(),
            before_entities: record.before_selection.entities.iter().copied().collect(),
            after_entities: record.after_selection.entities.iter().copied().collect(),
            committed_at: record.committed_at,
        }
    }
}

impl TryFrom<FlatCommandRecord> for CommandRecord {
    type Error = UndoError;

    fn try_from(flat: FlatCommandRecord) -> Result<Self, Self::Error> {
        let mut before_selection = SelectionSnapshot::empty();
        for e in flat.before_entities {
            before_selection.entities.push(e);
        }
        let mut after_selection = SelectionSnapshot::empty();
        for e in flat.after_entities {
            after_selection.entities.push(e);
        }
        Ok(CommandRecord::new(
            CommandId(flat.id),
            flat.tx_id.map(TxId),
            UserId(flat.author),
            flat.command,
            before_selection,
            after_selection,
            flat.committed_at,
        ))
    }
}

/// Manifest describing an atomic-only undo session.
#[derive(Archive, Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[rkyv(compare(PartialEq), derive(Debug, Eq, PartialEq))]
pub struct SessionManifest {
    /// Redo/undo cursor position.
    pub cursor: u32,
    /// In-memory budget in bytes.
    pub budget_bytes: u64,
    /// Next command id allocator.
    pub next_command_id: u64,
    /// Next transaction id allocator.
    pub next_tx: u64,
    /// Serialized command history.
    pub records: Vec<FlatCommandRecord>,
}

/// Manifest file name under a session directory.
pub const MANIFEST_NAME: &str = "manifest.rkyv";
/// World snapshot file name under a session directory.
pub const WORLD_NAME: &str = "world.rkyv";

/// Minimal world snapshot for session restore tests.
#[derive(Archive, Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[rkyv(compare(PartialEq), derive(Debug, Eq, PartialEq))]
pub struct PersistedWorld {
    /// Counter mirrored from [`TestWorld::counter`].
    pub counter: i32,
}

impl From<&TestWorld> for PersistedWorld {
    fn from(world: &TestWorld) -> Self {
        Self {
            counter: world.counter,
        }
    }
}

/// Writes `manifest` bytes to `path` (used by tests for roundtrip checks).
pub fn write_manifest(path: &Path, manifest: &SessionManifest) -> Result<(), UndoError> {
    let bytes = rkyv::to_bytes::<rkyv::rancor::Error>(manifest)
        .map_err(|e| std::io::Error::other(format!("{e:?}")))?;
    std::fs::write(path, bytes)?;
    Ok(())
}

/// Reads a [`SessionManifest`] from `path`.
pub fn read_manifest(path: &Path) -> Result<SessionManifest, UndoError> {
    let bytes = std::fs::read(path)?;
    rkyv::from_bytes::<SessionManifest, rkyv::rancor::Error>(&bytes)
        .map_err(|e| std::io::Error::other(format!("{e:?}")).into())
}

/// Writes a flat command record to `path`.
pub async fn write_flat_record(path: &Path, flat: &FlatCommandRecord) -> Result<(), UndoError> {
    use tokio::io::AsyncWriteExt;
    let bytes = rkyv::to_bytes::<rkyv::rancor::Error>(flat)
        .map_err(|e| std::io::Error::other(format!("{e:?}")))?;
    if let Some(parent) = path.parent() {
        tokio::fs::create_dir_all(parent).await?;
    }
    let mut file = tokio::fs::File::create(path).await?;
    file.write_all(&bytes).await?;
    file.sync_all().await?;
    Ok(())
}

/// Reads a [`FlatCommandRecord`] from disk.
pub fn read_flat_record(path: &Path) -> Result<FlatCommandRecord, UndoError> {
    let bytes = std::fs::read(path)?;
    rkyv::from_bytes::<FlatCommandRecord, rkyv::rancor::Error>(&bytes)
        .map_err(|e| std::io::Error::other(format!("{e:?}")).into())
}
