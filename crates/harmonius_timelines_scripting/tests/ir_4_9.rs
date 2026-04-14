//! Integration coverage for `docs/design/integration/timelines-scripting-test-cases.md`.

use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;

use harmonius_timelines_scripting::log;
use harmonius_timelines_scripting::{
    branch_tick_or_default, tick_simulation, variable, BindError, ChoiceMadeEvent, Entity,
    GraphBranchConfig, GraphExecCtx, GraphExecutionSystem, GraphId, GraphInstance, GraphProgram,
    GraphStateMachine, Keyframe, KeyframeId, MultiTrackTimeline, PlaybackState, RuntimeError,
    ScriptTypeId, SlotId, StepOutcome, TickCount, TimelineAdvanceSystem, TimelineEvent,
    TimelineEventKind, Track, TrackId, TrackValue, TypedSlot, VariableStore, World,
    DEFAULT_WAIT_TIMEOUT, GRAPH_STEP_NOT_STARTED,
};

static TIMELINE_ENTRY_HITS: AtomicUsize = AtomicUsize::new(0);

fn on_timeline_event(_ctx: &mut GraphExecCtx) {
    TIMELINE_ENTRY_HITS.fetch_add(1, Ordering::SeqCst);
}

#[test]
fn tc_ir_4_9_1_1_keyframe_fires_graph_entry() {
    TIMELINE_ENTRY_HITS.store(0, Ordering::SeqCst);
    let mut world = World::new();
    let advance = TimelineAdvanceSystem::new();
    world.timeline_seek_tx = Some(advance.seek_tx.clone());

    let tl = Entity(1);
    let graph = Entity(2);
    world.timelines.insert(
        tl,
        MultiTrackTimeline {
            duration: TickCount(10),
            tracks: vec![Track {
                id: TrackId(0),
                keyframes: vec![Keyframe {
                    id: KeyframeId(0),
                    tick: TickCount(1),
                    value: TrackValue::Entity(graph),
                }],
            }],
        },
    );
    world.playback.insert(
        tl,
        PlaybackState {
            current_tick: TickCount(0),
            playing: true,
            looping: false,
        },
    );

    let mut prog = GraphProgram::new(1, GraphId(9));
    prog
        .fn_table
        .insert("on_timeline_event".to_string(), on_timeline_event);
    world.programs.insert(graph, Arc::new(prog));
    world.graphs.insert(graph, GraphInstance::new(1));

    tick_simulation(&mut world, &advance, TickCount(0));
    assert_eq!(TIMELINE_ENTRY_HITS.load(Ordering::SeqCst), 1);
}

#[test]
fn tc_ir_4_9_1_2_multiple_keyframes_in_order() {
    let tl = MultiTrackTimeline {
        duration: TickCount(10),
        tracks: vec![Track {
            id: TrackId(1),
            keyframes: vec![
                Keyframe {
                    id: KeyframeId(10),
                    tick: TickCount(2),
                    value: TrackValue::F32(1.0),
                },
                Keyframe {
                    id: KeyframeId(11),
                    tick: TickCount(3),
                    value: TrackValue::F32(2.0),
                },
                Keyframe {
                    id: KeyframeId(12),
                    tick: TickCount(4),
                    value: TrackValue::F32(3.0),
                },
            ],
        }],
    };
    let crossed = tl.keyframes_crossed(TickCount(0), TickCount(5));
    let ids: Vec<KeyframeId> = crossed.iter().map(|c| c.1).collect();
    assert_eq!(ids, vec![KeyframeId(10), KeyframeId(11), KeyframeId(12)]);
}

#[test]
fn tc_ir_4_9_1_3_missing_entry_point_logs_warning() {
    log::clear_warnings();
    let mut world = World::new();
    let advance = TimelineAdvanceSystem::new();
    world.timeline_seek_tx = Some(advance.seek_tx.clone());

    let tl = Entity(1);
    let graph = Entity(2);
    world.timelines.insert(
        tl,
        MultiTrackTimeline {
            duration: TickCount(10),
            tracks: vec![Track {
                id: TrackId(0),
                keyframes: vec![Keyframe {
                    id: KeyframeId(0),
                    tick: TickCount(1),
                    value: TrackValue::Entity(graph),
                }],
            }],
        },
    );
    world.playback.insert(
        tl,
        PlaybackState {
            current_tick: TickCount(0),
            playing: true,
            looping: false,
        },
    );

    let prog = GraphProgram::new(1, GraphId(77));
    world.programs.insert(graph, Arc::new(prog));
    world.graphs.insert(graph, GraphInstance::new(1));

    tick_simulation(&mut world, &advance, TickCount(0));
    let warns = log::take_warnings();
    assert!(warns.iter().any(|w| w.contains("77")));
}

fn emit_seek_and_pause(ctx: &mut GraphExecCtx) {
    let tl = Entity(1);
    let _ = ctx.emit_timeline_seek(tl, TickCount(300));
    if let Some(pb) = ctx.world.playback.get_mut(&tl) {
        pb.pause();
    }
}

#[test]
fn tc_ir_4_9_6_1_graph_seek_applies_next_tick() {
    let mut world = World::new();
    let advance = TimelineAdvanceSystem::new();
    world.timeline_seek_tx = Some(advance.seek_tx.clone());

    let tl = Entity(1);
    let graph = Entity(2);
    world.timelines.insert(
        tl,
        MultiTrackTimeline {
            duration: TickCount(600),
            tracks: vec![Track {
                id: TrackId(0),
                keyframes: vec![Keyframe {
                    id: KeyframeId(0),
                    tick: TickCount(1),
                    value: TrackValue::Entity(graph),
                }],
            }],
        },
    );
    world.playback.insert(
        tl,
        PlaybackState {
            current_tick: TickCount(0),
            playing: true,
            looping: false,
        },
    );

    let mut prog = GraphProgram::new(1, GraphId(1));
    prog.fn_table.insert("on_timeline_event".to_string(), emit_seek_and_pause);
    world.programs.insert(graph, Arc::new(prog));
    world.graphs.insert(graph, GraphInstance::new(1));

    tick_simulation(&mut world, &advance, TickCount(0));
    tick_simulation(&mut world, &advance, TickCount(1));
    assert_eq!(world.playback.get(&tl).unwrap().current_tick, TickCount(300));
}

#[test]
fn tc_ir_4_9_6_3_timeline_seek_channel_full() {
    log::clear_warnings();
    let advance = TimelineAdvanceSystem::new();
    for i in 0..256 {
        let ev = harmonius_timelines_scripting::TimelineSeekEvent {
            timeline_entity: Entity(1),
            target_tick: TickCount(i),
        };
        advance.try_send(ev).unwrap();
    }
    let ev = harmonius_timelines_scripting::TimelineSeekEvent {
        timeline_entity: Entity(1),
        target_tick: TickCount(999),
    };
    assert!(advance.try_send(ev).is_err());
    let warns = log::take_warnings();
    assert!(warns.iter().any(|w| w.contains("TimelineSeek")));
}

#[test]
fn tc_ir_4_9_4_1_f32_track_drives_slot() {
    let tl = Entity(1);
    let graph = Entity(2);
    let mut world = World::new();
    let advance = TimelineAdvanceSystem::new();
    world.timeline_graph.insert(tl, graph);
    world.bindings.insert(tl, vec![(TrackId(0), SlotId(0))]);
    world.timelines.insert(
        tl,
        MultiTrackTimeline {
            duration: TickCount(10),
            tracks: vec![Track {
                id: TrackId(0),
                keyframes: vec![Keyframe {
                    id: KeyframeId(0),
                    tick: TickCount(0),
                    value: TrackValue::F32(0.5),
                }],
            }],
        },
    );
    world.playback.insert(
        tl,
        PlaybackState {
            current_tick: TickCount(0),
            playing: true,
            looping: false,
        },
    );
    let mut graph_inst = GraphInstance::new(1);
    graph_inst
        .variables
        .insert_typed(SlotId(0), ScriptTypeId::F32, TypedSlot::F32(0.0));
    world.graphs.insert(graph, graph_inst);

    tick_simulation(&mut world, &advance, TickCount(0));
    let sample = world
        .graphs
        .get(&graph)
        .and_then(|g| g.variables.get(SlotId(0)))
        .unwrap();
    assert_eq!(sample, TypedSlot::F32(0.5));
}

#[test]
fn tc_ir_4_9_4_2_bool_track_drives_slot() {
    let tl = Entity(1);
    let graph = Entity(2);
    let mut world = World::new();
    let advance = TimelineAdvanceSystem::new();
    world.timeline_graph.insert(tl, graph);
    world.bindings.insert(tl, vec![(TrackId(0), SlotId(0))]);
    world.timelines.insert(
        tl,
        MultiTrackTimeline {
            duration: TickCount(10),
            tracks: vec![Track {
                id: TrackId(0),
                keyframes: vec![Keyframe {
                    id: KeyframeId(0),
                    tick: TickCount(0),
                    value: TrackValue::Bool(true),
                }],
            }],
        },
    );
    world.playback.insert(
        tl,
        PlaybackState {
            current_tick: TickCount(0),
            playing: true,
            looping: false,
        },
    );
    let mut graph_inst = GraphInstance::new(1);
    graph_inst
        .variables
        .insert_typed(SlotId(0), ScriptTypeId::Bool, TypedSlot::Bool(false));
    world.graphs.insert(graph, graph_inst);

    tick_simulation(&mut world, &advance, TickCount(0));
    assert_eq!(
        world.graphs.get(&graph).unwrap().variables.get(SlotId(0)),
        Some(TypedSlot::Bool(true))
    );
}

#[test]
fn tc_ir_4_9_4_3_vec3_track_drives_slot() {
    let tl = Entity(1);
    let graph = Entity(2);
    let mut world = World::new();
    let advance = TimelineAdvanceSystem::new();
    world.timeline_graph.insert(tl, graph);
    world.bindings.insert(tl, vec![(TrackId(0), SlotId(0))]);
    world.timelines.insert(
        tl,
        MultiTrackTimeline {
            duration: TickCount(10),
            tracks: vec![Track {
                id: TrackId(0),
                keyframes: vec![Keyframe {
                    id: KeyframeId(0),
                    tick: TickCount(0),
                    value: TrackValue::Vec3([1.0, 2.0, 3.0]),
                }],
            }],
        },
    );
    world.playback.insert(
        tl,
        PlaybackState {
            current_tick: TickCount(0),
            playing: true,
            looping: false,
        },
    );
    let mut graph_inst = GraphInstance::new(1);
    graph_inst.variables.insert_typed(
        SlotId(0),
        ScriptTypeId::Vec3,
        TypedSlot::Vec3([0.0, 0.0, 0.0]),
    );
    world.graphs.insert(graph, graph_inst);

    tick_simulation(&mut world, &advance, TickCount(0));
    assert_eq!(
        world.graphs.get(&graph).unwrap().variables.get(SlotId(0)),
        Some(TypedSlot::Vec3([1.0, 2.0, 3.0]))
    );
}

#[test]
fn tc_ir_4_9_4_4_slot_type_mismatch_rejected() {
    let mut store = VariableStore::default();
    store.insert_typed(SlotId(0), ScriptTypeId::F32, TypedSlot::F32(0.0));
    let track = TrackId(4);
    let tv = TrackValue::Bool(true);
    let err = variable::validate_binding(track, &tv, &store, SlotId(0)).unwrap_err();
    assert_eq!(
        err,
        BindError {
            track,
            slot: SlotId(0),
            expected: ScriptTypeId::F32,
            actual: ScriptTypeId::Bool,
        }
    );
}

#[test]
fn tc_ir_4_9_3_1_wait_freezes_step_until_complete_signal() {
    let mut sm = GraphStateMachine::new(1);
    sm.current_step = 0;
    let tl = Entity(5);
    sm.arm_timeline_wait(tl, TickCount(0), Some(TickCount(50)));
    sm.observe_timeline_events(&[], TickCount(1));
    assert_eq!(sm.current_step, 0);
}

#[test]
fn tc_ir_4_9_3_2_wait_resumes_on_following_tick() {
    let mut sm = GraphStateMachine::new(1);
    sm.current_step = GRAPH_STEP_NOT_STARTED;
    let tl = Entity(5);
    sm.arm_timeline_wait(tl, TickCount(0), Some(TickCount(50)));
    let ev = TimelineEvent {
        kind: TimelineEventKind::TimelineComplete,
        time: TickCount(2),
        entity: tl,
        graph_entity: None,
    };
    sm.observe_timeline_events(&[ev], TickCount(2));
    assert!(sm.pending_timeline_resume);
    sm.apply_pending_resume();
    assert_eq!(sm.current_step, 0);
}

#[test]
fn tc_ir_4_9_3_3_wait_timeout_errors() {
    let mut sm = GraphStateMachine::new(1);
    let tl = Entity(5);
    sm.arm_timeline_wait(tl, TickCount(0), Some(TickCount(10)));
    sm.observe_timeline_events(&[], TickCount(9));
    assert_eq!(sm.last_outcome, StepOutcome::Ok);
    sm.observe_timeline_events(&[], TickCount(10));
    assert_eq!(sm.last_outcome, StepOutcome::Error(RuntimeError::WaitTimeout));
}

#[test]
fn tc_ir_4_9_3_4_default_timeout_is_600_ticks() {
    let mut sm = GraphStateMachine::new(1);
    let tl = Entity(5);
    sm.arm_timeline_wait(tl, TickCount(0), None);
    sm.observe_timeline_events(&[], TickCount(599));
    assert_eq!(sm.last_outcome, StepOutcome::Ok);
    sm.observe_timeline_events(&[], TickCount(600));
    assert_eq!(sm.last_outcome, StepOutcome::Error(RuntimeError::WaitTimeout));
    assert_eq!(DEFAULT_WAIT_TIMEOUT, TickCount(600));
}

#[test]
fn tc_ir_4_9_3_5_no_coroutine_keywords_in_sources() {
    let sources = [
        include_str!("../src/lib.rs"),
        include_str!("../src/advance.rs"),
        include_str!("../src/command.rs"),
        include_str!("../src/event.rs"),
        include_str!("../src/graph.rs"),
        include_str!("../src/ids.rs"),
        include_str!("../src/log.rs"),
        include_str!("../src/playback.rs"),
        include_str!("../src/timeline.rs"),
        include_str!("../src/variable.rs"),
        include_str!("../src/world.rs"),
    ];
    for src in sources {
        assert!(!src.contains("async fn"));
        assert!(!src.contains("await"));
        assert!(!src.contains("coroutine"));
        assert!(!src.contains("Coroutine"));
    }
}

#[test]
fn tc_ir_4_9_5_1_branch_choice_a_seeks_tick_a() {
    let mut world = World::new();
    let advance = TimelineAdvanceSystem::new();
    let tl = Entity(1);
    world.timelines.insert(
        tl,
        MultiTrackTimeline {
            duration: TickCount(600),
            tracks: vec![],
        },
    );
    world.playback.insert(
        tl,
        PlaybackState {
            current_tick: TickCount(0),
            playing: true,
            looping: false,
        },
    );
    let mut graph_inst = GraphInstance::new(1);
    graph_inst.branch = Some(GraphBranchConfig {
        timeline: tl,
        tick_a: TickCount(10),
        tick_b: TickCount(20),
    });
    world.graphs.insert(Entity(2), graph_inst);
    world.try_send_choice(ChoiceMadeEvent { branch_id: 0 }).unwrap();
    tick_simulation(&mut world, &advance, TickCount(0));
    assert_eq!(world.playback.get(&tl).unwrap().current_tick, TickCount(10));
}

#[test]
fn tc_ir_4_9_5_2_branch_choice_b_seeks_tick_b() {
    let mut world = World::new();
    let advance = TimelineAdvanceSystem::new();
    let tl = Entity(1);
    world.timelines.insert(
        tl,
        MultiTrackTimeline {
            duration: TickCount(600),
            tracks: vec![],
        },
    );
    world.playback.insert(
        tl,
        PlaybackState {
            current_tick: TickCount(0),
            playing: true,
            looping: false,
        },
    );
    let mut graph_inst = GraphInstance::new(1);
    graph_inst.branch = Some(GraphBranchConfig {
        timeline: tl,
        tick_a: TickCount(10),
        tick_b: TickCount(25),
    });
    world.graphs.insert(Entity(2), graph_inst);
    world.try_send_choice(ChoiceMadeEvent { branch_id: 1 }).unwrap();
    tick_simulation(&mut world, &advance, TickCount(0));
    assert_eq!(world.playback.get(&tl).unwrap().current_tick, TickCount(25));
}

#[test]
fn tc_ir_4_9_5_3_default_branch_on_error() {
    let tick_a = TickCount(10);
    let tick_b = TickCount(20);
    assert_eq!(branch_tick_or_default(Err(()), tick_a, tick_b), tick_a);
    assert_eq!(branch_tick_or_default(Ok(true), tick_a, tick_b), tick_a);
    assert_eq!(branch_tick_or_default(Ok(false), tick_a, tick_b), tick_b);
}

#[test]
fn tc_ir_4_9_5_4_prompt_visible_after_apply() {
    let mut buf = harmonius_timelines_scripting::CommandBuffer::default();
    buf.spawn_prompt(Entity(99));
    buf.apply();
    assert_eq!(buf.visible_prompts(), &[Entity(99)]);
}

#[test]
fn tc_ir_4_9_5_5_choice_mpsc_resumes_seek() {
    let mut world = World::new();
    let advance = TimelineAdvanceSystem::new();
    let tl = Entity(1);
    world.timelines.insert(
        tl,
        MultiTrackTimeline {
            duration: TickCount(600),
            tracks: vec![],
        },
    );
    world.playback.insert(
        tl,
        PlaybackState {
            current_tick: TickCount(0),
            playing: true,
            looping: false,
        },
    );
    let mut graph_inst = GraphInstance::new(1);
    graph_inst.branch = Some(GraphBranchConfig {
        timeline: tl,
        tick_a: TickCount(10),
        tick_b: TickCount(40),
    });
    world.graphs.insert(Entity(2), graph_inst);
    world.try_send_choice(ChoiceMadeEvent { branch_id: 1 }).unwrap();
    tick_simulation(&mut world, &advance, TickCount(0));
    assert_eq!(world.playback.get(&tl).unwrap().current_tick, TickCount(40));
}

#[test]
fn tc_ir_4_9_7_1_hot_reload_preserves_compatible_slots() {
    let mut world = World::new();
    let graph = Entity(2);
    let mut inst = GraphInstance::new(1);
    inst
        .variables
        .insert_typed(SlotId(1), ScriptTypeId::F32, TypedSlot::F32(3.5));
    world.graphs.insert(graph, inst);

    let old = GraphProgram::new(1, GraphId(1));
    world.programs.insert(graph, Arc::new(old));

    let new = GraphProgram::new(2, GraphId(1));
    let _dropped = GraphExecutionSystem::hot_reload(&mut world, graph, Arc::new(new), &[SlotId(1)]);
    let stored = world.graphs.get(&graph).unwrap().variables.get(SlotId(1));
    assert_eq!(stored, Some(TypedSlot::F32(3.5)));
    assert_eq!(
        world.graphs.get(&graph).unwrap().state_machine.current_step,
        GRAPH_STEP_NOT_STARTED
    );
}

#[test]
fn tc_ir_4_9_7_2_hot_reload_drops_removed_slots_with_warning() {
    let mut world = World::new();
    let graph = Entity(2);
    let mut inst = GraphInstance::new(1);
    inst
        .variables
        .insert_typed(SlotId(1), ScriptTypeId::F32, TypedSlot::F32(1.0));
    inst
        .variables
        .insert_typed(SlotId(2), ScriptTypeId::F32, TypedSlot::F32(2.0));
    world.graphs.insert(graph, inst);
    world.programs.insert(graph, Arc::new(GraphProgram::new(1, GraphId(1))));

    let new = GraphProgram::new(2, GraphId(1));
    let dropped = GraphExecutionSystem::hot_reload(&mut world, graph, Arc::new(new), &[SlotId(1)]);
    assert_eq!(world.graphs.get(&graph).unwrap().variables.get(SlotId(2)), None);
    assert!(dropped.contains(&SlotId(2)));
}
