//! Headless frame harness: staging → bridge → runtime phases → snapshot publish.

use std::collections::VecDeque;

use crate::bridge::EventBridge;
use crate::mutation::EditorMutation;
use crate::snapshot::{GameStateSnapshot, TripleSnapshotBuffer};
use crate::world::{EditorWorld, EntityId, GameWorld};

/// High-level editor / runtime mode used by play-in-editor tests.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum PlayMode {
    /// Default scene editing.
    Edit,
    /// Active play-in-editor session.
    Playing,
    /// Paused simulation during PIE.
    Paused,
    /// Single-step requests during PIE.
    Stepping,
}

/// Ordered trace of work done in one synthetic frame.
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct FrameReport {
    /// Human-readable major steps (`drain`, `phase-1`, …, `phase-8`, `snapshot`).
    pub log: Vec<&'static str>,
    /// True when at least one mutation was drained before phase 1 began.
    pub drained_before_phase1: bool,
}

/// Deterministic harness wiring editor staging, bridging, runtime ticks, and snapshots.
#[derive(Debug)]
pub struct HeadlessHarness {
    /// Editor-owned shadow world.
    pub editor: EditorWorld,
    /// Runtime world (play-in-editor uses this as the live copy).
    pub game: GameWorld,
    /// One-way mutation bridge (`CH-22`).
    pub bridge: EventBridge,
    /// Triple-buffered snapshots delivered after phase 7.
    pub snapshots: TripleSnapshotBuffer,
    /// Mutations waiting to cross the bridge this frame.
    pub staged: VecDeque<EditorMutation>,
    /// Current play mode (`IR-9.1.4`).
    pub play_mode: PlayMode,
    /// `FM-6` counter: game despawn removed a selected editor entity.
    pub fm6_selection_cleared_events: u64,
}

impl Default for HeadlessHarness {
    fn default() -> Self {
        Self {
            editor: EditorWorld::default(),
            game: GameWorld::default(),
            bridge: EventBridge::new(),
            snapshots: TripleSnapshotBuffer::new(),
            staged: VecDeque::new(),
            play_mode: PlayMode::Edit,
            fm6_selection_cleared_events: 0,
        }
    }
}

impl HeadlessHarness {
    /// Builds a harness with default worlds and counters.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Stages a mutation for the next `run_frame` flush.
    pub fn stage_mutation(&mut self, mutation: EditorMutation) {
        self.staged.push_back(mutation);
    }

    /// Moves staged mutations across the bridge, honoring `FM-1` by draining when full.
    pub fn flush_staged_to_bridge(&mut self) {
        while let Some(mut current) = self.staged.pop_front() {
            loop {
                match self.bridge.try_enqueue(current) {
                    Ok(()) => break,
                    Err(m) => {
                        self.bridge.fm1_backpressure_events =
                            self.bridge.fm1_backpressure_events.saturating_add(1);
                        self.bridge.drain_into_game(&mut self.game);
                        current = m;
                    }
                }
            }
        }
    }

    /// Runs one full edit-mode frame: flush → drain → phases 1..8 → snapshot publish.
    pub fn run_frame(&mut self) -> FrameReport {
        let mut log = Vec::new();
        self.flush_staged_to_bridge();
        let drained = self.bridge.drain_into_game(&mut self.game);
        let drained_before_phase1 = drained > 0;
        if drained > 0 {
            log.push("drain");
        }
        for phase in 1_u8..=8_u8 {
            match phase {
                1 => log.push("phase-1"),
                2 => log.push("phase-2"),
                3 => log.push("phase-3"),
                4 => log.push("phase-4"),
                5 => log.push("phase-5"),
                6 => log.push("phase-6"),
                7 => log.push("phase-7"),
                _ => log.push("phase-8"),
            }
        }
        let snap = GameStateSnapshot::from_world(&self.game.time, &self.game.inner);
        self.snapshots.publish(snap);
        log.push("snapshot");
        self.game.tick_frame();
        FrameReport {
            log,
            drained_before_phase1,
        }
    }

    /// Advances the runtime tick without mutating the editor world (`TC-IR-9.1.1.2`).
    pub fn run_game_idle_ticks(&mut self, frames: u64) {
        for _ in 0..frames {
            self.game.tick_frame();
        }
    }

    /// Starts play-in-editor by cloning the editor shadow into the runtime world.
    pub fn start_pie(&mut self) -> Result<(), crate::error::EditorCoreError> {
        self.game = self.editor.clone_to_game()?;
        self.play_mode = PlayMode::Playing;
        Ok(())
    }

    /// Stops PIE by discarding the runtime copy (`TC-IR-9.1.4.3`).
    pub fn stop_pie(&mut self) {
        self.game = GameWorld::default();
        self.play_mode = PlayMode::Edit;
    }

    /// Applies runtime-only entity churn used in `TC-IR-9.1.4.2`.
    pub fn game_only_spawn_extra(&mut self, id: EntityId) {
        self.game.inner.entities.insert(id);
    }

    /// Despawns a runtime entity; if it was selected in the editor, clears selection (`FM-6`).
    pub fn game_despawn_selected_path(&mut self, id: EntityId) {
        let was_selected = self.editor.selection.contains(&id);
        self.game.despawn(id);
        if was_selected {
            self.editor.sync_selection_after_game_despawn(id);
            self.fm6_selection_cleared_events =
                self.fm6_selection_cleared_events.saturating_add(1);
        }
    }
}

/// Runs a hot-reload migration across both worlds, bumping `dylib` on success (`IR-9.1.5`).
///
/// On panic, `dylib` is restored to `prev` and `fm4` increments (`FM-4`).
pub fn run_hot_reload_barrier(
    editor: &mut EditorWorld,
    game: &mut GameWorld,
    dylib: &mut u32,
    fm4: &mut u64,
    migrate: impl FnOnce(&mut EditorWorld, &mut GameWorld),
) {
    let prev = *dylib;
    let mut slot = Some(migrate);
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let f = slot.take().expect("migrate closure");
        f(editor, game);
    }));
    match result {
        Ok(()) => {
            *dylib = prev.saturating_add(1);
        }
        Err(_) => {
            *dylib = prev;
            *fm4 = fm4.saturating_add(1);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::mutation::EditorMutationKind;
    use crate::world::EntityId;

    #[test]
    fn tc_ir_9_1_1_1_editor_edit_without_flush_does_not_touch_game() {
        let mut h = HeadlessHarness::new();
        h.editor.place_entity(EntityId(7));
        assert!(!h.game.has_entity(EntityId(7)));
    }

    #[test]
    fn tc_ir_9_1_1_2_game_idle_ticks_leave_editor_identical() {
        let mut h = HeadlessHarness::new();
        h.editor.place_entity(EntityId(1));
        let fp_before = h.editor.inner.fingerprint();
        h.run_game_idle_ticks(60);
        let fp_after = h.editor.inner.fingerprint();
        assert_eq!(fp_before, fp_after);
    }

    #[test]
    fn tc_ir_9_1_1_3_selection_editor_only() {
        let mut h = HeadlessHarness::new();
        h.editor.place_entity(EntityId(9));
        h.editor.select(EntityId(9));
        assert!(h.editor.selection.contains(&EntityId(9)));
        assert!(h.game.inner.entities.is_empty());
    }

    #[test]
    fn tc_ir_9_1_2_1_spawn_entity_via_bridge() {
        let mut h = HeadlessHarness::new();
        h.stage_mutation(EditorMutation {
            mutation_id: 1,
            kind: EditorMutationKind::SpawnEntity { id: EntityId(3) },
        });
        let rep = h.run_frame();
        assert!(rep.drained_before_phase1);
        assert!(h.game.has_entity(EntityId(3)));
    }

    #[test]
    fn tc_ir_9_1_2_2_despawn_entity() {
        let mut h = HeadlessHarness::new();
        h.game.apply_editor_mutation(&EditorMutation {
            mutation_id: 1,
            kind: EditorMutationKind::SpawnEntity { id: EntityId(42) },
        });
        h.stage_mutation(EditorMutation {
            mutation_id: 2,
            kind: EditorMutationKind::DespawnEntity { id: EntityId(42) },
        });
        h.run_frame();
        assert!(!h.game.has_entity(EntityId(42)));
    }

    #[test]
    fn tc_ir_9_1_2_3_insert_component() {
        let mut h = HeadlessHarness::new();
        h.stage_mutation(EditorMutation {
            mutation_id: 1,
            kind: EditorMutationKind::SpawnEntity { id: EntityId(5) },
        });
        h.stage_mutation(EditorMutation {
            mutation_id: 2,
            kind: EditorMutationKind::InsertComponent {
                entity: EntityId(5),
                component_id: 9,
                bytes: vec![1, 2, 3],
            },
        });
        h.run_frame();
        assert_eq!(
            h.game
                .inner
                .components
                .get(&(EntityId(5), 9))
                .map(Vec::as_slice),
            Some(&[1_u8, 2, 3][..])
        );
    }

    #[test]
    fn tc_ir_9_1_2_4_update_component() {
        let mut h = HeadlessHarness::new();
        h.stage_mutation(EditorMutation {
            mutation_id: 1,
            kind: EditorMutationKind::SpawnEntity { id: EntityId(5) },
        });
        h.stage_mutation(EditorMutation {
            mutation_id: 2,
            kind: EditorMutationKind::InsertComponent {
                entity: EntityId(5),
                component_id: 9,
                bytes: vec![1],
            },
        });
        h.run_frame();
        h.stage_mutation(EditorMutation {
            mutation_id: 3,
            kind: EditorMutationKind::UpdateComponent {
                entity: EntityId(5),
                component_id: 9,
                bytes: vec![2],
            },
        });
        h.run_frame();
        assert_eq!(
            h.game.inner.components.get(&(EntityId(5), 9)),
            Some(&vec![2])
        );
    }

    #[test]
    fn tc_ir_9_1_3_1_drain_before_phase_1() {
        let mut h = HeadlessHarness::new();
        h.stage_mutation(EditorMutation {
            mutation_id: 1,
            kind: EditorMutationKind::SpawnEntity { id: EntityId(1) },
        });
        let rep = h.run_frame();
        assert!(rep.drained_before_phase1);
        let drain_pos = rep.log.iter().position(|l| *l == "drain").expect("drain");
        let p1_pos = rep
            .log
            .iter()
            .position(|l| *l == "phase-1")
            .expect("phase-1");
        assert!(drain_pos < p1_pos);
    }

    #[test]
    fn tc_ir_9_1_3_2_phases_run_after_drain() {
        let mut h = HeadlessHarness::new();
        h.stage_mutation(EditorMutation {
            mutation_id: 1,
            kind: EditorMutationKind::SpawnEntity { id: EntityId(1) },
        });
        let rep = h.run_frame();
        let drain_pos = rep.log.iter().position(|l| *l == "drain").unwrap();
        let p8_pos = rep.log.iter().position(|l| *l == "phase-8").unwrap();
        assert!(drain_pos < p8_pos);
    }

    #[test]
    fn tc_ir_9_1_3_3_snapshot_frame_matches_tick_after_frame() {
        let mut h = HeadlessHarness::new();
        h.run_frame();
        let snap = h
            .snapshots
            .read_expecting_tick(h.game.time.tick.saturating_sub(1));
        assert_eq!(snap.frame_index, h.game.time.tick.saturating_sub(1));
    }

    #[test]
    fn tc_ir_9_1_4_1_pie_clone_counts_entities() {
        let mut h = HeadlessHarness::new();
        for i in 0..100 {
            h.editor.place_entity(EntityId(i));
        }
        h.start_pie().expect("pie");
        assert_eq!(h.game.inner.entities.len(), 100);
    }

    #[test]
    fn tc_ir_9_1_4_2_pie_does_not_mutate_editor() {
        let mut h = HeadlessHarness::new();
        h.editor.place_entity(EntityId(1));
        let fp0 = h.editor.inner.fingerprint();
        h.start_pie().expect("pie");
        for _ in 0..60 {
            h.game_only_spawn_extra(EntityId(999));
            h.game.tick_frame();
        }
        assert_eq!(h.editor.inner.fingerprint(), fp0);
    }

    #[test]
    fn tc_ir_9_1_4_3_pie_stop_drops_game() {
        let mut h = HeadlessHarness::new();
        h.editor.place_entity(EntityId(1));
        h.start_pie().expect("pie");
        assert!(h.game.has_entity(EntityId(1)));
        h.stop_pie();
        assert!(!h.game.has_entity(EntityId(1)));
        assert!(h.editor.contains_entity(EntityId(1)));
    }

    #[test]
    fn tc_ir_9_1_2_n1_burst_300_mutation_backpressure() {
        let mut h = HeadlessHarness::new();
        for i in 0_u32..300 {
            let i_u64 = u64::from(i);
            h.stage_mutation(EditorMutation {
                mutation_id: i_u64 + 1,
                kind: EditorMutationKind::SpawnEntity {
                    id: EntityId(i_u64 + 1),
                },
            });
        }
        h.flush_staged_to_bridge();
        assert!(h.bridge.fm1_backpressure_events > 0);
        let _tail = h.bridge.drain_into_game(&mut h.game);
        assert_eq!(h.game.inner.entities.len(), 300);
    }

    #[test]
    fn tc_ir_9_1_2_n2_mutation_id_collision_last_wins() {
        let mut b = EventBridge::new();
        let m1 = EditorMutation {
            mutation_id: 99,
            kind: EditorMutationKind::SpawnEntity { id: EntityId(1) },
        };
        let m2 = EditorMutation {
            mutation_id: 99,
            kind: EditorMutationKind::SpawnEntity { id: EntityId(2) },
        };
        b.try_enqueue(m1).expect("first");
        b.try_enqueue(m2).expect("second");
        assert_eq!(b.fm2_id_collision_events, 1);
        let mut g = GameWorld::default();
        b.drain_into_game(&mut g);
        assert!(!g.has_entity(EntityId(1)));
        assert!(g.has_entity(EntityId(2)));
    }

    #[test]
    fn tc_ir_9_1_3_n1_snapshot_stale_read_counter() {
        let mut buf = TripleSnapshotBuffer::new();
        buf.publish(GameStateSnapshot {
            frame_index: 1,
            entities: vec![EntityId(1)],
            render_token: 0,
        });
        buf.block_next_publish = true;
        buf.publish(GameStateSnapshot {
            frame_index: 2,
            entities: vec![],
            render_token: 0,
        });
        let _ = buf.read_expecting_tick(2);
        assert!(buf.fm3_stale_read_events >= 1);
    }

    #[test]
    fn tc_ir_9_1_5_n1_hot_reload_migration_panic_reverts() {
        let mut ed = EditorWorld::default();
        let mut gw = GameWorld::default();
        let mut dylib = 5_u32;
        let mut fm4 = 0_u64;
        run_hot_reload_barrier(&mut ed, &mut gw, &mut dylib, &mut fm4, |_e, _g| {
            panic!("migration failure");
        });
        assert_eq!(dylib, 5);
        assert_eq!(fm4, 1);
    }

    #[test]
    fn tc_ir_9_1_4_n1_pie_clone_failure() {
        let mut ed = EditorWorld::default();
        ed.force_next_clone_failure = true;
        let r = ed.clone_to_game();
        assert!(r.is_err());
    }

    #[test]
    fn tc_ir_9_1_1_n1_game_despawn_clears_selection_fm6() {
        let mut h = HeadlessHarness::new();
        h.editor.place_entity(EntityId(7));
        h.editor.select(EntityId(7));
        h.start_pie().expect("pie");
        h.game_despawn_selected_path(EntityId(7));
        assert!(!h.editor.selection.contains(&EntityId(7)));
        assert_eq!(h.fm6_selection_cleared_events, 1);
    }

    #[test]
    fn tc_ir_9_1_6_n1_undo_stack_overflow_drops_oldest_fm7() {
        let mut ed = EditorWorld::default();
        ed.undo_cap = 1024;
        for i in 0..10_000 {
            ed.spawn_with_undo(EditorMutation {
                mutation_id: u64::try_from(i).unwrap_or(u64::MAX),
                kind: EditorMutationKind::SpawnEntity { id: EntityId(i) },
            });
        }
        assert_eq!(ed.fm7_undo_overflow_events, 8976);
        assert_eq!(ed.undo_stack_depth(), 1024);
    }

    #[test]
    fn tc_ir_9_1_5_3_migration_runs_on_both_worlds() {
        let mut ed = EditorWorld::default();
        let mut gw = GameWorld::default();
        let mut dylib = 0_u32;
        let mut fm4 = 0_u64;
        let mut hits = 0_u32;
        run_hot_reload_barrier(&mut ed, &mut gw, &mut dylib, &mut fm4, |e, g| {
            e.place_entity(EntityId(10));
            g.apply_editor_mutation(&EditorMutation {
                mutation_id: 1,
                kind: EditorMutationKind::SpawnEntity { id: EntityId(20) },
            });
            hits += 2;
        });
        assert_eq!(hits, 2);
        assert_eq!(fm4, 0);
        assert_eq!(dylib, 1);
        assert!(ed.contains_entity(EntityId(10)));
        assert!(gw.has_entity(EntityId(20)));
    }

    #[test]
    fn tc_ir_9_1_6_1_undo_three_spawns_then_two_undos() {
        let mut ed = EditorWorld::default();
        for i in 1..=3 {
            ed.spawn_with_undo(EditorMutation {
                mutation_id: u64::from(i),
                kind: EditorMutationKind::SpawnEntity { id: EntityId(i) },
            });
        }
        ed.undo_last().expect("undo");
        ed.undo_last().expect("undo");
        assert_eq!(ed.inner.entities.len(), 1);
        assert!(ed.contains_entity(EntityId(1)));
    }

    #[test]
    fn tc_ir_9_1_6_2_redo_replays_one_spawn() {
        let mut ed = EditorWorld::default();
        for i in 1..=3 {
            ed.spawn_with_undo(EditorMutation {
                mutation_id: u64::from(i),
                kind: EditorMutationKind::SpawnEntity { id: EntityId(i) },
            });
        }
        ed.undo_last().expect("u1");
        ed.undo_last().expect("u2");
        ed.redo_last().expect("r1");
        assert_eq!(ed.inner.entities.len(), 2);
    }

    #[test]
    fn tc_ir_9_1_6_3_undo_does_not_revert_flushed_game_world() {
        let mut h = HeadlessHarness::new();
        h.stage_mutation(EditorMutation {
            mutation_id: 1,
            kind: EditorMutationKind::SpawnEntity { id: EntityId(5) },
        });
        h.run_frame();
        assert!(h.game.has_entity(EntityId(5)));
        h.editor.spawn_with_undo(EditorMutation {
            mutation_id: 2,
            kind: EditorMutationKind::SpawnEntity { id: EntityId(5) },
        });
        h.editor.undo_last().expect("undo");
        assert!(!h.editor.contains_entity(EntityId(5)));
        assert!(h.game.has_entity(EntityId(5)));
    }
}
