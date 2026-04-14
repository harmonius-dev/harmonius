//! Minimal `EditorWorld` / `GameWorld` stand-ins for integration tests.

use std::collections::{BTreeMap, BTreeSet};

use crate::error::UndoError;
use crate::mutation::{EditorMutation, EditorMutationKind};

/// Stable entity identifier shared by editor and game stand-ins.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct EntityId(pub u64);

/// Deterministic world contents used by both editor shadow and game worlds.
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct WorldState {
    /// Entities that currently exist in this world copy.
    pub entities: BTreeSet<EntityId>,
    /// Component payloads keyed by `(entity, component_type_id)`.
    pub components: BTreeMap<(EntityId, u32), Vec<u8>>,
}

impl WorldState {
    /// Serializes world contents into a deterministic byte vector for equality checks.
    #[must_use]
    pub fn fingerprint(&self) -> Vec<u8> {
        let mut out = Vec::new();
        for e in &self.entities {
            out.extend_from_slice(&e.0.to_le_bytes());
        }
        for ((entity, cid), bytes) in &self.components {
            out.extend_from_slice(&entity.0.to_le_bytes());
            out.extend_from_slice(&cid.to_le_bytes());
            let len = u64::try_from(bytes.len()).unwrap_or(u64::MAX);
            out.extend_from_slice(&len.to_le_bytes());
            out.extend_from_slice(bytes);
        }
        out
    }
}

/// Editor-owned shadow world plus selection (never stored on `GameWorld`).
#[derive(Clone, Debug)]
pub struct EditorWorld {
    /// Scene data being edited.
    pub inner: WorldState,
    /// Selection state lives only in the editor world.
    pub selection: BTreeSet<EntityId>,
    /// Undo stack stores inverse operations as forward mutations to replay on undo.
    undo: Vec<EditorUndoOp>,
    redo: Vec<EditorUndoOp>,
    /// Maximum undo depth before oldest entries drop (`FM-7`).
    pub undo_cap: usize,
    /// Count of dropped undo entries when `undo_cap` is exceeded (`FM-7`).
    pub fm7_undo_overflow_events: u64,
    /// When set, the next `clone_to_game` attempt fails (tests `FM-5`).
    pub force_next_clone_failure: bool,
}

/// Single undo record expressed as editor-local mutations to apply on undo/redo.
#[derive(Clone, Debug, Eq, PartialEq)]
enum EditorUndoOp {
    Forward(EditorMutation),
}

impl Default for EditorWorld {
    fn default() -> Self {
        Self {
            inner: WorldState::default(),
            selection: BTreeSet::new(),
            undo: Vec::new(),
            redo: Vec::new(),
            undo_cap: usize::MAX / 4,
            fm7_undo_overflow_events: 0,
            force_next_clone_failure: false,
        }
    }
}

impl EditorWorld {
    /// Places a primitive entity only in the shadow world (no bridge flush).
    pub fn place_entity(&mut self, id: EntityId) {
        self.inner.entities.insert(id);
    }

    /// Records selection for an entity in the editor world only.
    pub fn select(&mut self, id: EntityId) {
        self.selection.insert(id);
    }

    /// Returns whether the shadow world contains `id`.
    #[must_use]
    pub fn contains_entity(&self, id: EntityId) -> bool {
        self.inner.entities.contains(&id)
    }

    /// Returns the number of undo entries currently retained.
    #[must_use]
    pub fn undo_stack_depth(&self) -> usize {
        self.undo.len()
    }

    /// Clears selection entries that are no longer valid in the editor shadow world.
    pub fn sync_selection_after_game_despawn(&mut self, id: EntityId) {
        self.selection.remove(&id);
    }

    /// Spawns an entity in the editor world and records undo metadata.
    pub fn spawn_with_undo(&mut self, mutation: EditorMutation) {
        self.apply_mutation_local(&mutation);
        while self.undo.len() >= self.undo_cap {
            self.undo.remove(0);
            self.fm7_undo_overflow_events = self.fm7_undo_overflow_events.saturating_add(1);
        }
        self.undo.push(EditorUndoOp::Forward(mutation));
        self.redo.clear();
    }

    /// Undoes the last editor-local change.
    pub fn undo_last(&mut self) -> Result<(), UndoError> {
        let Some(EditorUndoOp::Forward(m)) = self.undo.pop() else {
            return Err(UndoError::StackEmpty);
        };
        self.apply_inverse(&m);
        self.redo.push(EditorUndoOp::Forward(m));
        Ok(())
    }

    /// Replays one undone change.
    pub fn redo_last(&mut self) -> Result<(), UndoError> {
        let Some(EditorUndoOp::Forward(m)) = self.redo.pop() else {
            return Err(UndoError::StackEmpty);
        };
        self.apply_mutation_local(&m);
        self.undo.push(EditorUndoOp::Forward(m));
        Ok(())
    }

    fn apply_mutation_local(&mut self, m: &EditorMutation) {
        match &m.kind {
            EditorMutationKind::SpawnEntity { id } => {
                self.inner.entities.insert(*id);
            }
            EditorMutationKind::DespawnEntity { id } => {
                self.inner.entities.remove(id);
                self.inner.components.retain(|k, _| k.0 != *id);
                self.selection.remove(id);
            }
            EditorMutationKind::InsertComponent { entity, component_id, bytes } => {
                self.inner
                    .components
                    .insert((*entity, *component_id), bytes.clone());
            }
            EditorMutationKind::UpdateComponent { entity, component_id, bytes } => {
                self.inner
                    .components
                    .insert((*entity, *component_id), bytes.clone());
            }
            EditorMutationKind::SetResource { .. } | EditorMutationKind::PushScene { .. } => {}
            EditorMutationKind::PopScene => {}
        }
    }

    fn apply_inverse(&mut self, m: &EditorMutation) {
        match &m.kind {
            EditorMutationKind::SpawnEntity { id } => {
                self.inner.entities.remove(id);
                self.selection.remove(id);
                self.inner.components.retain(|k, _| k.0 != *id);
            }
            EditorMutationKind::DespawnEntity { .. } => {}
            EditorMutationKind::InsertComponent { entity, component_id, .. } => {
                self.inner.components.remove(&(*entity, *component_id));
            }
            EditorMutationKind::UpdateComponent { .. } => {}
            EditorMutationKind::SetResource { .. } | EditorMutationKind::PushScene { .. } => {}
            EditorMutationKind::PopScene => {}
        }
    }

    /// Clones the shadow scene into a fresh `GameWorld` for play-in-editor.
    pub fn clone_to_game(&mut self) -> Result<GameWorld, crate::error::EditorCoreError> {
        if self.force_next_clone_failure {
            self.force_next_clone_failure = false;
            return Err(crate::error::EditorCoreError::PieCloneFailed);
        }
        Ok(GameWorld {
            inner: self.inner.clone(),
            time: GameTime::default(),
        })
    }
}

/// Monotonic tick counter carried by the runtime world.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct GameTime {
    /// Current simulation tick index.
    pub tick: u64,
}

/// Runtime-owned world; never stores editor selection.
#[derive(Clone, Debug, Default)]
pub struct GameWorld {
    /// Live simulation state.
    pub inner: WorldState,
    /// Frame/time bookkeeping for snapshot alignment tests.
    pub time: GameTime,
}

impl GameWorld {
    /// Returns whether an entity exists in the runtime world.
    #[must_use]
    pub fn has_entity(&self, id: EntityId) -> bool {
        self.inner.entities.contains(&id)
    }

    /// Despawns an entity in the runtime world (used for `FM-6` tests).
    pub fn despawn(&mut self, id: EntityId) {
        self.inner.entities.remove(&id);
        self.inner.components.retain(|k, _| k.0 != id);
    }

    /// Advances the tick counter without touching any `EditorWorld`.
    pub fn tick_frame(&mut self) {
        self.time.tick = self.time.tick.saturating_add(1);
    }

    /// Applies a bridged editor mutation to the runtime world (post-drain).
    pub fn apply_editor_mutation(&mut self, m: &EditorMutation) {
        match &m.kind {
            EditorMutationKind::SpawnEntity { id } => {
                self.inner.entities.insert(*id);
            }
            EditorMutationKind::DespawnEntity { id } => {
                self.despawn(*id);
            }
            EditorMutationKind::InsertComponent {
                entity,
                component_id,
                bytes,
            } => {
                self.inner
                    .components
                    .insert((*entity, *component_id), bytes.clone());
            }
            EditorMutationKind::UpdateComponent {
                entity,
                component_id,
                bytes,
            } => {
                self.inner
                    .components
                    .insert((*entity, *component_id), bytes.clone());
            }
            EditorMutationKind::SetResource { .. }
            | EditorMutationKind::PushScene { .. }
            | EditorMutationKind::PopScene => {}
        }
    }
}
