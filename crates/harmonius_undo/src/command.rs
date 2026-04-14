//! Concrete editor commands used by the undo stack.
#![allow(missing_docs)] // rkyv-generated archived companion types omit doc comments.

use std::io;

use rkyv::{Archive, Deserialize, Serialize};
use smallvec::SmallVec;
use thiserror::Error;

use crate::ids::UserId;
use crate::selection::EntityRef;
use crate::world::TestWorld;

/// Failure modes while applying or reverting editor commands.
#[derive(Debug, Error)]
pub enum CommandError {
    /// The command cannot run with the current editor scope.
    #[error("invalid command scope")]
    InvalidScope,
    /// Another collaborator owns overlapping state.
    #[error("command conflict")]
    Conflict,
    /// The undo stack is over its memory budget.
    #[error("undo memory budget exceeded")]
    Budget,
    /// Disk or transport IO failed while persisting history.
    #[error("io error: {0}")]
    Io(#[from] io::Error),
}

/// Editor-side command with deterministic apply and revert semantics.
#[derive(Archive, Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[rkyv(compare(PartialEq), derive(Debug, Eq, PartialEq))]
pub enum EditorCommand {
    /// Increments the global counter used by generic roundtrip tests.
    BumpCounter,
    /// Sets a component value, retaining the previous value for revert.
    SetComponentValue {
        /// Target entity.
        entity: EntityRef,
        /// Previous value, if any.
        before: Option<i32>,
        /// New value after apply.
        after: i32,
    },
    /// Inserts a component value, removing it on revert.
    InsertComponent {
        /// Target entity.
        entity: EntityRef,
        /// Inserted value.
        value: i32,
    },
    /// Removes a component while capturing the removed payload.
    RemoveComponent {
        /// Target entity.
        entity: EntityRef,
        /// Removed value.
        removed: i32,
    },
    /// Spawns an entity with no attached component value.
    SpawnEntity {
        /// Spawned entity id.
        entity: EntityRef,
    },
    /// Despawns an entity while capturing optional component state.
    DespawnEntity {
        /// Removed entity id.
        entity: EntityRef,
        /// Captured component value prior to removal.
        had_value: Option<i32>,
    },
    /// Changes hierarchy parentage for an entity.
    ReparentEntity {
        /// Moved entity.
        entity: EntityRef,
        /// Previous parent, if any.
        old_parent: Option<EntityRef>,
        /// New parent, if any.
        new_parent: Option<EntityRef>,
    },
    /// Models continuous typing coalescing within a time window.
    TypingEdit {
        /// Owning entity for the edited field.
        entity: EntityRef,
        /// Text prior to the edit.
        before: String,
        /// Text after the edit.
        after: String,
        /// Timestamp in milliseconds for coalescing heuristics.
        ts_ms: u64,
    },
    /// Models slider drags that should coalesce within a gesture id.
    SliderSet {
        /// Stable track identifier.
        track_id: u64,
        /// Gesture identifier for the drag.
        gesture: u64,
        /// Counter value prior to the drag.
        before: i32,
        /// Latest slider value.
        after: i32,
    },
    /// Placeholder for history entries that were spilled to disk.
    SpilledArchive {
        /// File name within the spill directory.
        file_name: String,
        /// Original serialized payload length in bytes.
        byte_len: u32,
    },
}

impl EditorCommand {
    /// Applies the command to `world`.
    pub fn apply(&self, world: &mut TestWorld) -> Result<(), CommandError> {
        match self {
            Self::BumpCounter => {
                world.counter = world.counter.saturating_add(1);
                Ok(())
            }
            Self::SetComponentValue { entity, after, .. } => {
                world.entities.insert(*entity);
                world.values.insert(*entity, *after);
                Ok(())
            }
            Self::InsertComponent { entity, value } => {
                if world.values.contains_key(entity) {
                    return Err(CommandError::Conflict);
                }
                world.entities.insert(*entity);
                world.values.insert(*entity, *value);
                Ok(())
            }
            Self::RemoveComponent { entity, removed } => {
                let existing = world
                    .values
                    .remove(entity)
                    .ok_or(CommandError::InvalidScope)?;
                if existing != *removed {
                    return Err(CommandError::InvalidScope);
                }
                if world.get_value(*entity).is_none() {
                    world.entities.remove(entity);
                }
                Ok(())
            }
            Self::SpawnEntity { entity } => {
                if world.entities.contains(entity) {
                    return Err(CommandError::Conflict);
                }
                world.entities.insert(*entity);
                Ok(())
            }
            Self::DespawnEntity { entity, had_value } => {
                let removed_value = world.values.remove(entity);
                if removed_value != *had_value {
                    return Err(CommandError::InvalidScope);
                }
                world.entities.remove(entity);
                Ok(())
            }
            Self::ReparentEntity {
                entity,
                old_parent: _,
                new_parent,
            } => {
                world.entities.insert(*entity);
                world.parents.insert(*entity, *new_parent);
                Ok(())
            }
            Self::TypingEdit { entity, after, .. } => {
                world.entities.insert(*entity);
                world.text.insert(*entity, after.clone());
                Ok(())
            }
            Self::SliderSet {
                track_id: _,
                gesture: _,
                after,
                ..
            } => {
                world.counter = *after;
                Ok(())
            }
            Self::SpilledArchive { .. } => Ok(()),
        }
    }

    /// Reverts the command on `world`.
    pub fn revert(&self, world: &mut TestWorld) -> Result<(), CommandError> {
        match self {
            Self::BumpCounter => {
                world.counter = world.counter.saturating_sub(1);
                Ok(())
            }
            Self::SetComponentValue { entity, before, .. } => match before {
                Some(prev) => {
                    world.values.insert(*entity, *prev);
                    world.entities.insert(*entity);
                    Ok(())
                }
                None => {
                    world.values.remove(entity);
                    world.entities.remove(entity);
                    Ok(())
                }
            },
            Self::InsertComponent { entity, .. } => {
                world.values.remove(entity);
                world.entities.remove(entity);
                Ok(())
            }
            Self::RemoveComponent { entity, removed } => {
                world.entities.insert(*entity);
                world.values.insert(*entity, *removed);
                Ok(())
            }
            Self::SpawnEntity { entity } => {
                world.entities.remove(entity);
                world.values.remove(entity);
                Ok(())
            }
            Self::DespawnEntity {
                entity,
                had_value: Some(value),
            } => {
                world.entities.insert(*entity);
                world.values.insert(*entity, *value);
                Ok(())
            }
            Self::DespawnEntity {
                entity,
                had_value: None,
            } => {
                world.entities.insert(*entity);
                Ok(())
            }
            Self::ReparentEntity {
                entity, old_parent, ..
            } => {
                match old_parent {
                    Some(parent) => {
                        world.parents.insert(*entity, Some(*parent));
                    }
                    None => {
                        world.parents.remove(entity);
                    }
                }
                Ok(())
            }
            Self::TypingEdit { entity, before, .. } => {
                if before.is_empty() {
                    world.text.remove(entity);
                } else {
                    world.text.insert(*entity, before.clone());
                }
                Ok(())
            }
            Self::SliderSet { before, .. } => {
                world.counter = *before;
                Ok(())
            }
            Self::SpilledArchive { .. } => Ok(()),
        }
    }

    /// Attempts to merge `other` into `self`, returning the merged command when possible.
    #[must_use]
    pub fn coalesce(self, other: Self) -> Option<Self> {
        match (self, other) {
            (
                Self::TypingEdit {
                    entity: e1,
                    before: b1,
                    after: a1,
                    ts_ms: t1,
                },
                Self::TypingEdit {
                    entity: e2,
                    before: b2,
                    after: a2,
                    ts_ms: t2,
                },
            ) if e1 == e2 && b1 == b2 && t2.saturating_sub(t1) <= 500 => {
                let mut merged_after = a1;
                merged_after.push_str(&a2);
                Some(Self::TypingEdit {
                    entity: e1,
                    before: b2,
                    after: merged_after,
                    ts_ms: t2,
                })
            }
            (
                Self::SliderSet {
                    track_id: tid1,
                    gesture: g1,
                    before: b1,
                    after: _,
                },
                Self::SliderSet {
                    track_id: tid2,
                    gesture: g2,
                    before: _,
                    after: a2,
                },
            ) if tid1 == tid2 && g1 == g2 => Some(Self::SliderSet {
                track_id: tid1,
                gesture: g1,
                before: b1,
                after: a2,
            }),
            _ => None,
        }
    }

    /// Returns an estimate of retained bytes for budgeting.
    #[must_use]
    pub fn memory_bytes(&self) -> u32 {
        if matches!(self, Self::SpilledArchive { .. }) {
            return 1;
        }
        let bytes = rkyv::to_bytes::<rkyv::rancor::Error>(self)
            .map(|b| b.len())
            .unwrap_or(0);
        u32::try_from(bytes).unwrap_or(u32::MAX)
    }

    /// Entities touched by this command for collaborative conflict checks.
    #[must_use]
    pub fn touches(&self) -> SmallVec<[EntityRef; 4]> {
        match self {
            Self::BumpCounter => SmallVec::new(),
            Self::SetComponentValue { entity, .. }
            | Self::InsertComponent { entity, .. }
            | Self::RemoveComponent { entity, .. }
            | Self::SpawnEntity { entity, .. }
            | Self::DespawnEntity { entity, .. }
            | Self::ReparentEntity { entity, .. }
            | Self::TypingEdit { entity, .. } => {
                let mut out = SmallVec::new();
                out.push(*entity);
                out
            }
            Self::SliderSet { .. } => SmallVec::new(),
            Self::SpilledArchive { .. } => SmallVec::new(),
        }
    }

    /// Human-readable label for editor UI surfaces.
    #[must_use]
    pub const fn label(&self) -> &'static str {
        match self {
            Self::BumpCounter => "BumpCounter",
            Self::SetComponentValue { .. } => "SetComponentValue",
            Self::InsertComponent { .. } => "InsertComponent",
            Self::RemoveComponent { .. } => "RemoveComponent",
            Self::SpawnEntity { .. } => "SpawnEntity",
            Self::DespawnEntity { .. } => "DespawnEntity",
            Self::ReparentEntity { .. } => "ReparentEntity",
            Self::TypingEdit { .. } => "TypingEdit",
            Self::SliderSet { .. } => "SliderSet",
            Self::SpilledArchive { .. } => "SpilledArchive",
        }
    }

    /// Returns the authoring user for collaborative commands.
    #[must_use]
    pub const fn author(&self) -> UserId {
        // Default author for local edits until networking supplies identifiers.
        UserId(0)
    }
}
