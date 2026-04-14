//! Integration tests mapped to `docs/design/tools/undo-redo-test-cases.md`.

use std::sync::atomic::{AtomicU64, Ordering};

use harmonius_undo::{
    CollabError, CollabSession, CommandError, CommandId, CommandRecord, DiskSpill, EditorCommand,
    EntityRef, SelectionSnapshot, TestWorld, UndoConflict, UndoError, UndoStack, UserId,
};

static COMMAND_IDS: AtomicU64 = AtomicU64::new(1);

fn snap(entities: &[EntityRef]) -> SelectionSnapshot {
    let mut s = SelectionSnapshot::empty();
    for e in entities {
        s.entities.push(*e);
    }
    s
}

fn mk_record(
    author: UserId,
    cmd: EditorCommand,
    before: SelectionSnapshot,
    after: SelectionSnapshot,
) -> CommandRecord {
    CommandRecord::new(
        CommandId(COMMAND_IDS.fetch_add(1, Ordering::Relaxed)),
        None,
        author,
        cmd,
        before,
        after,
        0,
    )
}

#[test]
fn tc_15_1_3_1_1_command_apply_revert_roundtrip() {
    let mut world = TestWorld::default();
    let cmd = EditorCommand::BumpCounter;
    cmd.apply(&mut world).unwrap();
    assert_eq!(world.counter, 1);
    cmd.revert(&mut world).unwrap();
    assert_eq!(world.counter, 0);
}

#[test]
fn tc_15_1_3_1_2_component_set_reverts_value() {
    let mut world = TestWorld::default();
    let e = EntityRef(7);
    let cmd = EditorCommand::SetComponentValue {
        entity: e,
        before: None,
        after: 42,
    };
    cmd.apply(&mut world).unwrap();
    assert_eq!(world.get_value(e), Some(42));
    cmd.revert(&mut world).unwrap();
    assert_eq!(world.get_value(e), None);
}

#[test]
fn tc_15_1_3_1_3_component_insert_reverts_remove() {
    let mut world = TestWorld::default();
    let e = EntityRef(3);
    let cmd = EditorCommand::InsertComponent {
        entity: e,
        value: 9,
    };
    cmd.apply(&mut world).unwrap();
    assert_eq!(world.get_value(e), Some(9));
    cmd.revert(&mut world).unwrap();
    assert_eq!(world.get_value(e), None);
}

#[test]
fn tc_15_1_3_1_4_entity_spawn_reverts_despawn() {
    let mut world = TestWorld::default();
    let e = EntityRef(11);
    let cmd = EditorCommand::SpawnEntity { entity: e };
    cmd.apply(&mut world).unwrap();
    assert!(world.entities.contains(&e));
    cmd.revert(&mut world).unwrap();
    assert!(!world.entities.contains(&e));
}

#[test]
fn tc_15_1_3_1_5_entity_despawn_reverts_spawn() {
    let mut world = TestWorld::default();
    let e = EntityRef(5);
    world.entities.insert(e);
    world.values.insert(e, 3);
    let cmd = EditorCommand::DespawnEntity {
        entity: e,
        had_value: Some(3),
    };
    cmd.apply(&mut world).unwrap();
    assert!(!world.entities.contains(&e));
    cmd.revert(&mut world).unwrap();
    assert_eq!(world.get_value(e), Some(3));
}

#[test]
fn tc_15_1_3_1_6_hierarchy_reparent_roundtrip() {
    let mut world = TestWorld::default();
    let child = EntityRef(2);
    let parent = EntityRef(3);
    world.entities.insert(child);
    let cmd = EditorCommand::ReparentEntity {
        entity: child,
        old_parent: None,
        new_parent: Some(parent),
    };
    cmd.apply(&mut world).unwrap();
    assert_eq!(world.parents.get(&child).copied().flatten(), Some(parent));
    cmd.revert(&mut world).unwrap();
    assert_eq!(world.parents.get(&child), None);
}

#[test]
fn tc_15_1_3_1_7_coalesce_continuous_typing() {
    let first = EditorCommand::TypingEdit {
        entity: EntityRef(1),
        before: String::new(),
        after: "h".to_string(),
        ts_ms: 0,
    };
    let second = EditorCommand::TypingEdit {
        entity: EntityRef(1),
        before: String::new(),
        after: "i".to_string(),
        ts_ms: 100,
    };
    let merged = first.coalesce(second).expect("coalesce");
    assert_eq!(
        merged,
        EditorCommand::TypingEdit {
            entity: EntityRef(1),
            before: String::new(),
            after: "hi".to_string(),
            ts_ms: 100,
        }
    );
}

#[test]
fn tc_15_1_3_1_8_coalesce_slider_drag() {
    let first = EditorCommand::SliderSet {
        track_id: 1,
        gesture: 9,
        before: 0,
        after: 10,
    };
    let second = EditorCommand::SliderSet {
        track_id: 1,
        gesture: 9,
        before: 0,
        after: 20,
    };
    let merged = first.coalesce(second).expect("coalesce");
    assert_eq!(
        merged,
        EditorCommand::SliderSet {
            track_id: 1,
            gesture: 9,
            before: 0,
            after: 20,
        }
    );
}

#[test]
fn tc_15_1_3_1_9_memory_bytes_accounting() {
    let cmd = EditorCommand::BumpCounter;
    let direct = cmd.memory_bytes();
    let archived = rkyv::to_bytes::<rkyv::rancor::Error>(&cmd)
        .expect("serialize")
        .len() as u32;
    assert_eq!(direct, archived);
}

#[test]
fn tc_15_1_3_2_1_stack_push_and_undo() {
    let mut world = TestWorld::default();
    let mut stack = UndoStack::new(1024 * 1024);
    let e = EntityRef(4);
    let before = snap(&[e]);
    let after = snap(&[e]);
    let record = mk_record(
        UserId(0),
        EditorCommand::SetComponentValue {
            entity: e,
            before: None,
            after: 5,
        },
        before,
        after,
    );
    stack.push_record(&mut world, record).unwrap();
    assert_eq!(world.get_value(e), Some(5));
    stack.undo(&mut world).unwrap();
    assert_eq!(world.get_value(e), None);
}

#[test]
fn tc_15_1_3_2_2_stack_undo_then_redo() {
    let mut world = TestWorld::default();
    let mut stack = UndoStack::new(1024 * 1024);
    let e = EntityRef(8);
    let sel = snap(&[e]);
    let record = mk_record(
        UserId(0),
        EditorCommand::SetComponentValue {
            entity: e,
            before: None,
            after: 2,
        },
        sel.clone(),
        sel.clone(),
    );
    stack.push_record(&mut world, record).unwrap();
    stack.undo(&mut world).unwrap();
    stack.redo(&mut world).unwrap();
    assert_eq!(world.get_value(e), Some(2));
}

#[test]
fn tc_15_1_3_2_3_stack_push_after_undo_clears_redo() {
    let mut world = TestWorld::default();
    let mut stack = UndoStack::new(1024 * 1024);
    let e = EntityRef(6);
    let sel = snap(&[e]);
    stack
        .push_record(
            &mut world,
            mk_record(
                UserId(0),
                EditorCommand::SetComponentValue {
                    entity: e,
                    before: None,
                    after: 1,
                },
                sel.clone(),
                sel.clone(),
            ),
        )
        .unwrap();
    stack.undo(&mut world).unwrap();
    stack
        .push_record(
            &mut world,
            mk_record(
                UserId(0),
                EditorCommand::SetComponentValue {
                    entity: e,
                    before: None,
                    after: 3,
                },
                sel.clone(),
                sel.clone(),
            ),
        )
        .unwrap();
    assert_eq!(world.get_value(e), Some(3));
    assert!(stack.redo(&mut world).is_err());
}

#[test]
fn tc_15_1_3_2_4_stack_clear() {
    let mut world = TestWorld::default();
    let mut stack = UndoStack::new(1024 * 1024);
    let e = EntityRef(1);
    let sel = snap(&[e]);
    stack
        .push_record(
            &mut world,
            mk_record(
                UserId(0),
                EditorCommand::BumpCounter,
                sel.clone(),
                sel.clone(),
            ),
        )
        .unwrap();
    stack.clear();
    assert_eq!(stack.current_bytes(), 0);
    assert!(matches!(
        stack.undo(&mut world),
        Err(UndoError::NothingToUndo)
    ));
}

#[test]
fn tc_15_1_3_3_1_transaction_single_undo_step() {
    let mut world = TestWorld::default();
    let mut stack = UndoStack::new(1024 * 1024);
    let e = EntityRef(9);
    let sel = snap(&[e]);
    {
        let mut tx = stack.begin_tx("multi");
        tx.push(
            &mut world,
            mk_record(
                UserId(0),
                EditorCommand::InsertComponent {
                    entity: e,
                    value: 1,
                },
                sel.clone(),
                sel.clone(),
            ),
        )
        .unwrap();
        tx.push(
            &mut world,
            mk_record(
                UserId(0),
                EditorCommand::SetComponentValue {
                    entity: e,
                    before: Some(1),
                    after: 2,
                },
                sel.clone(),
                sel.clone(),
            ),
        )
        .unwrap();
        tx.commit().unwrap();
    }
    assert_eq!(world.get_value(e), Some(2));
    stack.undo(&mut world).unwrap();
    assert_eq!(world.get_value(e), None);
}

#[test]
fn tc_15_1_3_3_2_transaction_rollback_on_panic() {
    let mut world = TestWorld::default();
    let mut stack = UndoStack::new(1024 * 1024);
    let e = EntityRef(12);
    let sel = snap(&[e]);
    {
        let mut tx = stack.begin_tx("abort");
        tx.push(
            &mut world,
            mk_record(
                UserId(0),
                EditorCommand::InsertComponent {
                    entity: e,
                    value: 1,
                },
                sel.clone(),
                sel.clone(),
            ),
        )
        .unwrap();
        tx.rollback(&mut world);
    }
    assert_eq!(world.get_value(e), None);
    assert_eq!(stack.cursor(), 0);
}

#[test]
fn tc_15_1_3_3_3_transaction_label_preserved() {
    let mut world = TestWorld::default();
    let mut stack = UndoStack::new(1024 * 1024);
    let sel = SelectionSnapshot::empty();
    {
        let mut tx = stack.begin_tx("labelled");
        tx.push(
            &mut world,
            mk_record(
                UserId(0),
                EditorCommand::BumpCounter,
                sel.clone(),
                sel.clone(),
            ),
        )
        .unwrap();
        tx.commit().unwrap();
    }
    assert_eq!(stack.last_transaction_label(), Some("labelled"));
}

#[tokio::test]
async fn tc_15_1_3_4_1_budget_evicts_oldest() {
    let tmp = tempfile::tempdir().unwrap();
    let spill = DiskSpill::new(tmp.path());
    let mut world = TestWorld::default();
    let mut stack = UndoStack::new(1);
    stack.set_disk_spill(spill);
    let sel = SelectionSnapshot::empty();
    for _ in 0..8 {
        stack
            .push_record(
                &mut world,
                mk_record(
                    UserId(0),
                    EditorCommand::BumpCounter,
                    sel.clone(),
                    sel.clone(),
                ),
            )
            .unwrap();
    }
    stack.maintain_budget().await.unwrap();
    assert!(stack.current_bytes() <= 1);
    let mut spilled = false;
    for entry in std::fs::read_dir(tmp.path()).unwrap() {
        let entry = entry.unwrap();
        if entry.path().extension().and_then(|s| s.to_str()) == Some("rkyv") {
            spilled = true;
        }
    }
    assert!(spilled);
}

#[tokio::test]
async fn tc_15_1_3_4_2_budget_spill_to_disk() {
    let tmp = tempfile::tempdir().unwrap();
    let spill = DiskSpill::new(tmp.path());
    let mut world = TestWorld::default();
    let mut stack = UndoStack::new(16);
    stack.set_disk_spill(spill);
    let sel = SelectionSnapshot::empty();
    stack
        .push_record(
            &mut world,
            mk_record(
                UserId(0),
                EditorCommand::BumpCounter,
                sel.clone(),
                sel.clone(),
            ),
        )
        .unwrap();
    stack.maintain_budget().await.unwrap();
    let path = std::fs::read_dir(tmp.path())
        .unwrap()
        .next()
        .unwrap()
        .unwrap()
        .path();
    let loaded = UndoStack::load_spilled_command(&path).await.unwrap();
    assert_eq!(loaded, EditorCommand::BumpCounter);
}

#[test]
fn tc_15_1_3_4_3_budget_tracks_total_bytes() {
    let mut world = TestWorld::default();
    let mut stack = UndoStack::new(1 << 20);
    let sel = SelectionSnapshot::empty();
    let mut sum = 0u64;
    for _ in 0..4 {
        let record = mk_record(
            UserId(0),
            EditorCommand::BumpCounter,
            sel.clone(),
            sel.clone(),
        );
        sum += u64::from(record.memory_bytes);
        stack.push_record(&mut world, record).unwrap();
    }
    assert_eq!(stack.current_bytes(), sum);
}

#[test]
fn tc_15_1_3_7_1_undo_restores_before_selection() {
    let mut world = TestWorld::default();
    let mut stack = UndoStack::new(1024 * 1024);
    let e1 = EntityRef(1);
    let e2 = EntityRef(2);
    let before = snap(&[e1]);
    let after = snap(&[e2]);
    stack
        .push_record(
            &mut world,
            mk_record(
                UserId(0),
                EditorCommand::BumpCounter,
                before.clone(),
                after.clone(),
            ),
        )
        .unwrap();
    stack.undo(&mut world).unwrap();
    assert_eq!(world.selection, before);
}

#[test]
fn tc_15_1_3_7_2_redo_restores_after_selection() {
    let mut world = TestWorld::default();
    let mut stack = UndoStack::new(1024 * 1024);
    let e1 = EntityRef(1);
    let e2 = EntityRef(2);
    let before = snap(&[e1]);
    let after = snap(&[e2]);
    stack
        .push_record(
            &mut world,
            mk_record(UserId(0), EditorCommand::BumpCounter, before, after.clone()),
        )
        .unwrap();
    stack.undo(&mut world).unwrap();
    stack.redo(&mut world).unwrap();
    assert_eq!(world.selection, after);
}

#[test]
fn tc_15_1_3_8_1_collab_two_user_independent_undo() {
    let mut session = CollabSession::new();
    session.insert_stack(UserId(1), UndoStack::new(1024 * 1024));
    session.insert_stack(UserId(2), UndoStack::new(1024 * 1024));
    let e1 = EntityRef(1);
    let e2 = EntityRef(2);
    let sel1 = snap(&[e1]);
    let sel2 = snap(&[e2]);
    let r1 = mk_record(
        UserId(1),
        EditorCommand::SetComponentValue {
            entity: e1,
            before: None,
            after: 10,
        },
        sel1.clone(),
        sel1.clone(),
    );
    session.push_for_user(UserId(1), r1).unwrap();
    let r2 = mk_record(
        UserId(2),
        EditorCommand::SetComponentValue {
            entity: e2,
            before: None,
            after: 20,
        },
        sel2.clone(),
        sel2.clone(),
    );
    session.push_for_user(UserId(2), r2).unwrap();
    session.undo_for(UserId(1)).unwrap();
    assert_eq!(session.world.get_value(e1), None);
    assert_eq!(session.world.get_value(e2), Some(20));
}

#[test]
fn tc_15_1_3_8_2_collab_conflict_detection_touches() {
    let mut session = CollabSession::new();
    session.insert_stack(UserId(1), UndoStack::new(1024 * 1024));
    session.insert_stack(UserId(2), UndoStack::new(1024 * 1024));
    let e = EntityRef(5);
    let sel = snap(&[e]);
    let r1 = mk_record(
        UserId(1),
        EditorCommand::SetComponentValue {
            entity: e,
            before: None,
            after: 1,
        },
        sel.clone(),
        sel.clone(),
    );
    session.push_for_user(UserId(1), r1).unwrap();
    let r2 = mk_record(
        UserId(2),
        EditorCommand::SetComponentValue {
            entity: e,
            before: Some(1),
            after: 2,
        },
        sel.clone(),
        sel.clone(),
    );
    session.push_for_user(UserId(2), r2).unwrap();
    let err = session.undo_for(UserId(2)).unwrap_err();
    assert!(matches!(
        err,
        CollabError::Conflict(UndoConflict::TouchesOverlap)
    ));
}

#[test]
fn tc_15_1_3_8_3_collab_per_user_cursor() {
    let mut session = CollabSession::new();
    session.insert_stack(UserId(1), UndoStack::new(1024 * 1024));
    session.insert_stack(UserId(2), UndoStack::new(1024 * 1024));
    let sel = SelectionSnapshot::empty();
    let r1 = mk_record(
        UserId(1),
        EditorCommand::BumpCounter,
        sel.clone(),
        sel.clone(),
    );
    session.push_for_user(UserId(1), r1).unwrap();
    let r2 = mk_record(
        UserId(2),
        EditorCommand::BumpCounter,
        sel.clone(),
        sel.clone(),
    );
    session.push_for_user(UserId(2), r2).unwrap();
    assert_eq!(session.stacks.get(&UserId(1)).unwrap().cursor(), 1);
    assert_eq!(session.stacks.get(&UserId(2)).unwrap().cursor(), 1);
}

#[test]
fn tc_15_1_3_3_2_transaction_insert_conflict_surfaces() {
    let mut world = TestWorld::default();
    let mut stack = UndoStack::new(1024 * 1024);
    let e = EntityRef(99);
    let sel = snap(&[e]);
    world.entities.insert(e);
    world.values.insert(e, 1);
    let mut tx = stack.begin_tx("conflict");
    let err = tx
        .push(
            &mut world,
            mk_record(
                UserId(0),
                EditorCommand::InsertComponent {
                    entity: e,
                    value: 2,
                },
                sel.clone(),
                sel.clone(),
            ),
        )
        .unwrap_err();
    assert!(matches!(err, UndoError::Command(CommandError::Conflict)));
}
