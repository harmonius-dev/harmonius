//! Integration tests for scripting ↔ ECS (IR-2.8.*). See companion test-case doc.

use std::collections::HashMap;
use std::sync::{Arc, Mutex};

use harmonius_integration_tests::{
    graph_instance_bucket_count, max_concurrency_this_frame, read_armor_var, read_health_var,
    read_pose_var, report_command_buffer_full,
    write_pose_var, write_speed_var, AccessSet, AnimState, Armor, AssetId, CommandBuffer,
    CommandSegment, DebugBridge, Entity, EventWriter, ExecutionContext, FnPtrTable,
    GraphAccessDescriptor, GraphError, GraphExecutionConfig, GraphExecutionSystem,
    GraphInstance, GraphInstanceState, GraphProgram, GraphRegistrationError, GraphSchedulePhase,
    GraphScheduler, Health, HotReloadError, ParallelCommandWriter, ResumeVariant, Speed, StepResult,
    ThreadArena, Transform, World,
};

#[derive(Clone, Debug, PartialEq, Eq)]
struct Damage(pub i32);

#[derive(Clone, Debug, PartialEq, Eq)]
struct Velocity(pub i32);

#[derive(Clone, Debug, PartialEq, Eq)]
struct Vfx;

fn register_world(world: &mut World) {
    world.ensure_registered::<Health>();
    world.ensure_registered::<Armor>();
    world.ensure_registered::<Damage>();
    world.ensure_registered::<Velocity>();
    world.ensure_registered::<Speed>();
    world.ensure_registered::<Transform>();
    world.ensure_registered::<AnimState>();
    world.ensure_registered::<Vfx>();
}

/// TC-IR-2.8.1.1
#[test]
fn tc_ir_2_8_1_1_graph_reads_own_component() {
    let mut world = World::new();
    register_world(&mut world);
    let e = Entity(1);
    world.insert(e, Health(100));
    let arena = ThreadArena::default();
    let events = EventWriter::new();
    let mut seg = CommandSegment::new();
    let ctx = ExecutionContext {
        arena: &arena,
        commands: &mut seg,
        debug: None,
        delta_time: 1.0 / 60.0,
        entity: e,
        events: &events,
        frame: 1,
        instruction_budget: 10_000,
        world: &world,
    };
    assert_eq!(ctx.read::<Health>(), Some(&Health(100)));
}

/// TC-IR-2.8.1.2
#[test]
fn tc_ir_2_8_1_2_graph_reads_other_entity() {
    let mut world = World::new();
    register_world(&mut world);
    let a = Entity(1);
    let b = Entity(2);
    world.insert(b, Armor(50));
    let arena = ThreadArena::default();
    let events = EventWriter::new();
    let mut seg = CommandSegment::new();
    let ctx = ExecutionContext {
        arena: &arena,
        commands: &mut seg,
        debug: None,
        delta_time: 1.0 / 60.0,
        entity: a,
        events: &events,
        frame: 1,
        instruction_budget: 10_000,
        world: &world,
    };
    assert_eq!(ctx.read_entity::<Armor>(b), Some(&Armor(50)));
}

/// TC-IR-2.8.1.3
#[test]
fn tc_ir_2_8_1_3_graph_reads_missing_component() {
    let mut world = World::new();
    register_world(&mut world);
    let e = Entity(1);
    world.insert(e, Health(1));
    let arena = ThreadArena::default();
    let events = EventWriter::new();
    let mut seg = CommandSegment::new();
    let ctx = ExecutionContext {
        arena: &arena,
        commands: &mut seg,
        debug: None,
        delta_time: 1.0 / 60.0,
        entity: e,
        events: &events,
        frame: 1,
        instruction_budget: 10_000,
        world: &world,
    };
    assert_eq!(ctx.read::<Armor>(), None);
}

/// TC-IR-2.8.2.1
#[test]
fn tc_ir_2_8_2_1_graph_writes_component() {
    let mut world = World::new();
    register_world(&mut world);
    let e = Entity(1);
    world.insert(e, Health(100));
    let arena = ThreadArena::default();
    let events = EventWriter::new();
    let mut seg = CommandSegment::new();
    let mut ctx = ExecutionContext {
        arena: &arena,
        commands: &mut seg,
        debug: None,
        delta_time: 1.0 / 60.0,
        entity: e,
        events: &events,
        frame: 1,
        instruction_budget: 10_000,
        world: &world,
    };
    ctx.write(e, Damage(10));
    let mut buf = CommandBuffer::default();
    buf.extend_from_segment(ctx.commands);
    buf.flush(&mut world);
    assert_eq!(world.get::<Damage>(e), Some(&Damage(10)));
}

/// TC-IR-2.8.2.2 — last write wins in stable worker order (worker-1 after worker-0).
#[test]
fn tc_ir_2_8_2_2_multiple_writes_same_component() {
    let mut world = World::new();
    register_world(&mut world);
    let e = Entity(1);
    world.insert(e, Health(0));
    let mut parallel = ParallelCommandWriter::new(2);
    parallel.writer(0).insert(e, Health(1));
    parallel.writer(1).insert(e, Health(2));
    let mut buf = CommandBuffer::default();
    parallel.merge_into(&mut buf);
    buf.flush(&mut world);
    assert_eq!(world.get::<Health>(e), Some(&Health(2)));
}

/// TC-IR-2.8.2.3 — deferred writes invisible until flush after graph run.
#[test]
fn tc_ir_2_8_2_3_no_mid_system_flush_visible() {
    let mut world = World::new();
    register_world(&mut world);
    let e = Entity(1);
    world.insert(e, Health(5));
    let arena = ThreadArena::default();
    let events = EventWriter::new();
    let mut seg = CommandSegment::new();
    {
        let mut ctx = ExecutionContext {
            arena: &arena,
            commands: &mut seg,
            debug: None,
            delta_time: 1.0 / 60.0,
            entity: e,
            events: &events,
            frame: 1,
            instruction_budget: 10_000,
            world: &world,
        };
        ctx.write(e, Health(99));
    }
    assert_eq!(world.get::<Health>(e), Some(&Health(5)));
    let mut buf = CommandBuffer::default();
    buf.extend_from_segment(&mut seg);
    buf.flush(&mut world);
    assert_eq!(world.get::<Health>(e), Some(&Health(99)));
}

/// TC-IR-2.8.3.1
#[test]
fn tc_ir_2_8_3_1_graph_spawns_entity() {
    let mut world = World::new();
    register_world(&mut world);
    let e = Entity(1);
    world.insert(e, Health(1));
    let arena = ThreadArena::default();
    let events = EventWriter::new();
    let mut seg = CommandSegment::new();
    {
        let mut ctx = ExecutionContext {
            arena: &arena,
            commands: &mut seg,
            debug: None,
            delta_time: 1.0 / 60.0,
            entity: e,
            events: &events,
            frame: 1,
            instruction_budget: 10_000,
            world: &world,
        };
        let _ = ctx.spawn().insert(Vfx);
    }
    let mut buf = CommandBuffer::default();
    buf.extend_from_segment(&mut seg);
    buf.flush(&mut world);
    let mut found = false;
    for alive in world.iter_entities() {
        if world.get::<Vfx>(alive).is_some() {
            found = true;
        }
    }
    assert!(found);
}

/// TC-IR-2.8.3.2
#[test]
fn tc_ir_2_8_3_2_graph_despawns_entity() {
    let mut world = World::new();
    register_world(&mut world);
    let e = Entity(1);
    world.insert(e, Health(1));
    let _arena = ThreadArena::default();
    let _events = EventWriter::new();
    let mut seg = CommandSegment::new();
    seg.despawn(e);
    let mut buf = CommandBuffer::default();
    buf.extend_from_segment(&mut seg);
    buf.flush(&mut world);
    assert!(!world.contains_entity(e));
}

/// TC-IR-2.8.3.3 / TC-IR-2.8.3.4 / N1 / N2 / N3 — overflow path.
#[test]
fn tc_ir_2_8_3_overflow_halts_deterministic_no_early_flush() {
    let mut world = World::new();
    register_world(&mut world);
    let e = Entity(1);
    world.insert(e, Health(1));
    let _arena = ThreadArena::default();
    let _events = EventWriter::new();
    let diag = Arc::new(Mutex::new(Vec::new()));
    let cfg = GraphExecutionConfig {
        delta_time: 1.0 / 60.0,
        diagnostics: Some(Arc::clone(&diag)),
        frame: 7,
        instruction_budget: 10_000,
        parallel_workers: 1,
    };
    let mut seg = CommandSegment::with_limits(1, 1);
    let op_before = seg.op_index;
    assert!(seg.insert(e, Health(2)));
    assert!(!seg.insert(e, Health(3)));
    assert_eq!(world.get::<Health>(e), Some(&Health(1)));
    report_command_buffer_full(&cfg, AssetId(9), 0);
    let logs = diag.lock().expect("poison");
    assert_eq!(logs.len(), 1);
    drop(logs);
    let mut buf = CommandBuffer::default();
    buf.extend_from_segment(&mut seg);
    buf.flush(&mut world);
    assert_eq!(world.get::<Health>(e), Some(&Health(2)));
    assert_eq!(seg.op_index, op_before.wrapping_add(1));
}

/// TC-IR-2.8.4.1 — disjoint write slots allow parallel batches.
#[test]
fn tc_ir_2_8_4_1_parallel_graph_execution() {
    let workers = 8;
    let mut world = World::new();
    register_world(&mut world);
    let mut programs = HashMap::new();
    let mut instances = Vec::new();
    for i in 0u8..100 {
        let slot_write = i;
        let pid = AssetId(1000 + u64::from(i));
        programs.insert(
            pid,
            GraphProgram {
                access: GraphAccessDescriptor::new(
                    AccessSet::empty(),
                    AccessSet::singleton(slot_write),
                    true,
                ),
                fn_table: FnPtrTable { entry: graph_touch },
                id: pid,
                phase: GraphSchedulePhase::SimulationFixed,
                state_layout_hash: 1,
            },
        );
        let e = Entity(10_000 + u64::from(i));
        world.insert(e, Health(i as i32));
        instances.push((
            e,
            GraphInstance {
                program_id: pid,
                state: GraphInstanceState::default(),
                version: 1,
            },
        ));
    }
    let cfg = GraphExecutionConfig {
        delta_time: 1.0 / 60.0,
        diagnostics: None,
        frame: 1,
        instruction_budget: 10_000,
        parallel_workers: workers,
    };
    let arena = ThreadArena::default();
    let events = EventWriter::new();
    let mut parallel = ParallelCommandWriter::new(workers);
    assert_eq!(
        graph_instance_bucket_count(&instances, &programs, GraphSchedulePhase::SimulationFixed),
        100
    );
    let _ = GraphExecutionSystem::run(
        &cfg,
        &world,
        &programs,
        &mut instances,
        GraphSchedulePhase::SimulationFixed,
        &mut parallel,
        &events,
        &arena,
        None::<&DebugBridge>,
    );
    assert!(max_concurrency_this_frame() >= 1);
}

fn graph_touch(_s: &mut GraphInstanceState, _c: &mut ExecutionContext<'_>) -> StepResult {
    StepResult::Success
}

/// TC-IR-2.8.4.2 — overlapping `Velocity` writes force one serial bucket.
#[test]
fn tc_ir_2_8_4_2_conflicting_graphs_serialize() {
    let workers = 8;
    let mut world = World::new();
    register_world(&mut world);
    let vslot = AccessSet::singleton(40);
    let p0 = AssetId(200);
    let p1 = AssetId(201);
    let mut programs = HashMap::new();
    programs.insert(
        p0,
        GraphProgram {
            access: GraphAccessDescriptor::new(AccessSet::empty(), vslot, true),
            fn_table: FnPtrTable { entry: graph_touch },
            id: p0,
            phase: GraphSchedulePhase::SimulationFixed,
            state_layout_hash: 1,
        },
    );
    programs.insert(
        p1,
        GraphProgram {
            access: GraphAccessDescriptor::new(AccessSet::empty(), vslot, true),
            fn_table: FnPtrTable { entry: graph_touch },
            id: p1,
            phase: GraphSchedulePhase::SimulationFixed,
            state_layout_hash: 1,
        },
    );
    let e0 = Entity(30);
    let e1 = Entity(31);
    world.insert(e0, Velocity(0));
    world.insert(e1, Velocity(0));
    let mut instances = vec![
        (
            e0,
            GraphInstance {
                program_id: p0,
                state: GraphInstanceState::default(),
                version: 1,
            },
        ),
        (
            e1,
            GraphInstance {
                program_id: p1,
                state: GraphInstanceState::default(),
                version: 1,
            },
        ),
    ];
    let cfg = GraphExecutionConfig {
        delta_time: 1.0 / 60.0,
        diagnostics: None,
        frame: 1,
        instruction_budget: 10_000,
        parallel_workers: workers,
    };
    let arena = ThreadArena::default();
    let events = EventWriter::new();
    let mut parallel = ParallelCommandWriter::new(workers);
    let _ = GraphExecutionSystem::run(
        &cfg,
        &world,
        &programs,
        &mut instances,
        GraphSchedulePhase::SimulationFixed,
        &mut parallel,
        &events,
        &arena,
        None::<&DebugBridge>,
    );
    assert_eq!(max_concurrency_this_frame(), 1);
}

/// TC-IR-2.8.4.3
#[test]
fn tc_ir_2_8_4_3_fixed_step_phase_binding() {
    let mut sched = GraphScheduler::new();
    let p = GraphProgram {
        access: GraphAccessDescriptor::new(AccessSet::empty(), AccessSet::singleton(1), false),
        fn_table: FnPtrTable { entry: graph_touch },
        id: AssetId(1),
        phase: GraphSchedulePhase::SimulationFixed,
        state_layout_hash: 1,
    };
    let err = sched
        .register(GraphSchedulePhase::AnimVariable, p)
        .unwrap_err();
    assert!(matches!(
        err,
        GraphRegistrationError::PhaseMismatch { .. }
    ));
}

/// TC-IR-2.8.5.1
#[test]
fn tc_ir_2_8_5_1_access_sets_computed() {
    let ra = AccessSet::singleton(1);
    let wb = AccessSet::singleton(2);
    let d = GraphAccessDescriptor::new(ra, wb, true);
    assert_eq!(d.reads, AccessSet::singleton(1));
    assert_eq!(d.writes, AccessSet::singleton(2));
    assert!(d.has_commands);
}

/// TC-IR-2.8.5.2
#[test]
fn tc_ir_2_8_5_2_access_sets_refreshed_on_reload() {
    let mut sched = GraphScheduler::new();
    let pid = AssetId(55);
    let p = GraphProgram {
        access: GraphAccessDescriptor::new(AccessSet::singleton(1), AccessSet::singleton(2), true),
        fn_table: FnPtrTable { entry: graph_touch },
        id: pid,
        phase: GraphSchedulePhase::SimulationFixed,
        state_layout_hash: 1,
    };
    sched
        .register(GraphSchedulePhase::SimulationFixed, p)
        .unwrap();
    let new_acc = GraphAccessDescriptor::new(
        AccessSet::singleton(3),
        AccessSet::singleton(4),
        false,
    );
    sched.stage_access_reload(pid, new_acc.clone());
    sched.advance_phase_boundary();
    assert_eq!(sched.program(pid).unwrap().access, new_acc);
}

/// TC-IR-2.8.5.3
#[test]
fn tc_ir_2_8_5_3_rkyv_roundtrip_descriptor() {
    use rkyv::{from_bytes, rancor::Error, to_bytes};
    let d = GraphAccessDescriptor::new(
        AccessSet::singleton(1),
        AccessSet::singleton(2),
        true,
    );
    let bytes = to_bytes::<Error>(&d).expect("serialize");
    let out: GraphAccessDescriptor = from_bytes::<_, Error>(&bytes).expect("deserialize");
    assert_eq!(d, out);
}

/// TC-IR-2.8.6.1 / 6.2 / 6.3 / 6.4
#[test]
fn tc_ir_2_8_6_entity_variables() {
    let mut world = World::new();
    register_world(&mut world);
    let e = Entity(1);
    world.insert(e, Health(42));
    world.insert(e, Speed(1.0));
    world.insert(e, Transform(1.0, 2.0));
    world.insert(e, AnimState(3));
    let arena = ThreadArena::default();
    let events = EventWriter::new();
    let mut seg = CommandSegment::new();
    let mut ctx = ExecutionContext {
        arena: &arena,
        commands: &mut seg,
        debug: None,
        delta_time: 1.0 / 60.0,
        entity: e,
        events: &events,
        frame: 1,
        instruction_budget: 10_000,
        world: &world,
    };
    assert_eq!(read_health_var(&ctx), Some(42));
    write_speed_var(&mut ctx, 9.0);
    assert_eq!(read_pose_var(&ctx), Some((1.0, 2.0, 3)));
    assert!(write_pose_var(&mut ctx, 4.0, 5.0, 6));
    let mut buf = CommandBuffer::default();
    buf.extend_from_segment(&mut seg);
    buf.flush(&mut world);
    assert_eq!(world.get::<Speed>(e), Some(&Speed(9.0)));
    let t = world.get::<Transform>(e).expect("transform");
    assert!((t.0 - 4.0).abs() < f32::EPSILON);
}

/// TC-IR-2.8.5.N1 / N2
#[test]
fn tc_ir_2_8_5_hot_reload_layout() {
    let mut sched = GraphScheduler::new();
    let pid = AssetId(77);
    let base = GraphProgram {
        access: GraphAccessDescriptor::new(AccessSet::empty(), AccessSet::singleton(1), false),
        fn_table: FnPtrTable { entry: graph_touch },
        id: pid,
        phase: GraphSchedulePhase::SimulationFixed,
        state_layout_hash: 10,
    };
    sched
        .register(GraphSchedulePhase::SimulationFixed, base.clone())
        .unwrap();
    let bad = GraphProgram {
        access: base.access.clone(),
        fn_table: FnPtrTable { entry: graph_touch },
        id: pid,
        phase: GraphSchedulePhase::SimulationFixed,
        state_layout_hash: 99,
    };
    let err = sched.reload_dylib(pid, bad, 10).unwrap_err();
    assert!(matches!(err, HotReloadError::LayoutMismatch { .. }));
    let good = GraphProgram {
        access: base.access.clone(),
        fn_table: FnPtrTable { entry: graph_touch },
        id: pid,
        phase: GraphSchedulePhase::SimulationFixed,
        state_layout_hash: 10,
    };
    sched.reload_dylib(pid, good, 10).unwrap();
}

/// TC-IR-2.8.6.N1 / N2
#[test]
fn tc_ir_2_8_6_negative_vars() {
    let mut world = World::new();
    register_world(&mut world);
    let e = Entity(1);
    world.insert(e, Health(1));
    let arena = ThreadArena::default();
    let events = EventWriter::new();
    let mut seg = CommandSegment::new();
    {
        let ctx = ExecutionContext {
            arena: &arena,
            commands: &mut seg,
            debug: None,
            delta_time: 1.0 / 60.0,
            entity: e,
            events: &events,
            frame: 1,
            instruction_budget: 10_000,
            world: &world,
        };
        assert_eq!(read_armor_var(&ctx), None);
    }
    world.insert(e, Transform(1.0, 1.0));
    let mut ctx = ExecutionContext {
        arena: &arena,
        commands: &mut seg,
        debug: None,
        delta_time: 1.0 / 60.0,
        entity: e,
        events: &events,
        frame: 1,
        instruction_budget: 10_000,
        world: &world,
    };
    assert!(!write_pose_var(&mut ctx, 2.0, 2.0, 2));
}

/// TC-IR-2.8.4.N1
#[test]
fn tc_ir_2_8_4_n1_instruction_budget_exhausted() {
    fn spin(s: &mut GraphInstanceState, c: &mut ExecutionContext<'_>) -> StepResult {
        let mut edges = 0u32;
        loop {
            if edges >= c.instruction_budget {
                return StepResult::Error(GraphError::BudgetExhausted);
            }
            s.scratch += 1;
            edges += 1;
            if s.scratch > 50_000 {
                return StepResult::Success;
            }
        }
    }
    let mut world = World::new();
    register_world(&mut world);
    let e = Entity(1);
    world.insert(e, Health(1));
    let mut programs = HashMap::new();
    let pid = AssetId(400);
    programs.insert(
        pid,
        GraphProgram {
            access: GraphAccessDescriptor::new(AccessSet::empty(), AccessSet::empty(), false),
            fn_table: FnPtrTable { entry: spin },
            id: pid,
            phase: GraphSchedulePhase::SimulationFixed,
            state_layout_hash: 1,
        },
    );
    let mut instances = vec![(
        e,
        GraphInstance {
            program_id: pid,
            state: GraphInstanceState {
                layout_hash: 1,
                scratch: 0,
                suspend: ResumeVariant::Entry,
            },
            version: 1,
        },
    )];
    let cfg = GraphExecutionConfig {
        delta_time: 1.0 / 60.0,
        diagnostics: None,
        frame: 1,
        instruction_budget: 0,
        parallel_workers: 1,
    };
    let arena = ThreadArena::default();
    let events = EventWriter::new();
    let mut parallel = ParallelCommandWriter::new(1);
    let out = GraphExecutionSystem::run(
        &cfg,
        &world,
        &programs,
        &mut instances,
        GraphSchedulePhase::SimulationFixed,
        &mut parallel,
        &events,
        &arena,
        None::<&DebugBridge>,
    );
    assert!(matches!(
        out[0].1,
        StepResult::Error(GraphError::BudgetExhausted)
    ));
}
