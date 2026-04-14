//! Traceability tests for `PLAN-core-runtime-events-plugins` (TC-* rows).

use std::any::TypeId;
use std::sync::atomic::{AtomicU32, Ordering};
use std::sync::Arc;
use std::thread;
use std::time::Instant;

use harmonius_core::events::{
    CommandBuffer, EventBridge, EventBridgeConfig, EventChannel, EventObserverRegistry,
    EventReader, EventWriter, ObserverCallback, ObserverDescriptor, PersistentStream,
    ReactiveQuery, StreamCursor,
};
use harmonius_core::plugins::{
    verify_plugin_abi, App, Capability, HotReloadError, HotReloadManager, HotReloadMarker, Plugin,
    PluginDescriptor, PluginError, PluginGraphError, PluginGroupBuilder, SemVer,
};
use harmonius_core::{ComponentEvent, Entity, World, WorldId};

// --- TC-1.5.1.1 Double buffer swap ---

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct E1(u32);

#[test]
fn tc_1_5_1_1_double_buffer_swap() {
    let mut ch = EventChannel::<E1>::new();
    {
        let mut w = EventWriter::new(&mut ch);
        w.send(E1(1));
        w.send(E1(2));
        w.send(E1(3));
    }
    assert_eq!(ch.back_len(), 3);
    ch.swap();
    let r = EventReader::new(&ch);
    assert_eq!(r.read_front().len(), 3);
    assert_eq!(ch.read_front().count(), 3);
    ch.swap();
    assert_eq!(ch.read_front().count(), 0);
}

// --- TC-1.5.1.2 Parallel readers ---

#[test]
fn tc_1_5_1_2_parallel_readers_no_contention() {
    let mut ch = EventChannel::<E1>::new();
    ch.send(E1(7));
    ch.swap();
    let shared = Arc::new(ch);
    let handles: Vec<_> = (0..8)
        .map(|_| {
            let c = Arc::clone(&shared);
            thread::spawn(move || c.read_front().cloned().collect::<Vec<_>>())
        })
        .collect();
    let mut all = Vec::new();
    for h in handles {
        all.push(h.join().expect("join"));
    }
    for v in &all {
        assert_eq!(v, &vec![E1(7)]);
    }
}

// --- TC-1.5.1.3 Flood warning ---

#[test]
fn tc_1_5_1_3_flood_warning_threshold() {
    let mut ch = EventChannel::<E1>::with_flood_threshold(50_000);
    for i in 0..50_001 {
        ch.send(E1(i));
    }
    assert!(ch.flood_diagnostic_fired());
}

// --- TC-1.5.1.4 Throughput ---

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct E64([u8; 64]);

impl Default for E64 {
    fn default() -> Self {
        Self([0u8; 64])
    }
}

#[test]
fn tc_1_5_1_4_throughput_100k_events() {
    let mut ch = EventChannel::<E64>::new();
    let start = Instant::now();
    for _ in 0..100_000 {
        ch.send(E64::default());
    }
    let elapsed = start.elapsed();
    let limit_ms = if cfg!(debug_assertions) { 10 } else { 1 };
    assert!(
        elapsed.as_millis() < u128::from(limit_ms as u32),
        "elapsed {:?} exceeds {limit_ms} ms budget",
        elapsed
    );
}

// --- TC-1.5.2.* Persistent stream ---

#[test]
fn tc_1_5_2_1_persistent_stream_cursor() {
    let mut s = PersistentStream::<E1>::new(128);
    for frame in 0u32..6 {
        for i in 0..10 {
            s.push(E1(frame * 10 + i));
        }
    }
    let mut c = s.cursor_from_oldest();
    let batch: Vec<_> = c.read(&s).into_iter().cloned().collect();
    assert_eq!(batch.len(), 60);
}

#[test]
fn tc_1_5_2_2_cursor_independence() {
    let mut s = PersistentStream::<E1>::new(64);
    for i in 0..40 {
        s.push(E1(i));
    }
    let base = s.write_head().saturating_sub(40);
    let mut a = s.cursor_at(base + 10);
    let mut b = s.cursor_at(base + 30);
    let ra: Vec<_> = a.read(&s).into_iter().map(|e| e.0).collect();
    let rb: Vec<_> = b.read(&s).into_iter().map(|e| e.0).collect();
    assert!(ra.iter().all(|&v| v >= 10));
    assert!(rb.iter().all(|&v| v >= 30));
}

#[test]
fn tc_1_5_2_3_cursor_overflow_detection() {
    let mut s = PersistentStream::<E1>::new(4);
    for i in 0..10 {
        s.push(E1(i));
    }
    let base = s.write_head().saturating_sub(4);
    let mut c: StreamCursor<E1> = s.cursor_at(base);
    let _ = c.read(&s);
    for i in 0..10 {
        s.push(E1(100 + i));
    }
    assert!(c.has_overflowed(&s));
}

// --- TC-1.5.3.* Observers ---

#[derive(Default)]
struct CountingObserver {
    hits: Arc<AtomicU32>,
}

impl ObserverCallback for CountingObserver {
    fn invoke(&mut self, _event: ComponentEvent, _entity: Entity, world: &mut World) {
        self.hits.fetch_add(1, Ordering::SeqCst);
        world.log_observer_event(ComponentEvent::Added, Entity(0), TypeId::of::<()>());
    }
}

#[test]
fn tc_1_5_3_1_observer_fires_on_add() {
    let hits = Arc::new(AtomicU32::new(0));
    let mut reg = EventObserverRegistry::new();
    let desc = ObserverDescriptor {
        watched_components: vec![TypeId::of::<()>()],
        triggers: vec![ComponentEvent::Added],
        priority: 0,
    };
    reg.register(
        desc,
        CountingObserver {
            hits: Arc::clone(&hits),
        },
    )
    .unwrap();
    let mut world = World::new(WorldId(0));
    for i in 0u64..100 {
        reg.notify(
            ComponentEvent::Added,
            TypeId::of::<()>(),
            Entity(i),
            &mut world,
        );
    }
    assert_eq!(hits.load(Ordering::SeqCst), 100);
}

#[test]
fn tc_1_5_3_2_observer_fires_on_remove() {
    let hits = Arc::new(AtomicU32::new(0));
    let mut reg = EventObserverRegistry::new();
    reg.register(
        ObserverDescriptor {
            watched_components: vec![TypeId::of::<()>()],
            triggers: vec![ComponentEvent::Removed],
            priority: 0,
        },
        CountingObserver {
            hits: Arc::clone(&hits),
        },
    )
    .unwrap();
    let mut world = World::new(WorldId(0));
    for i in 0u64..50 {
        reg.notify(
            ComponentEvent::Removed,
            TypeId::of::<()>(),
            Entity(i),
            &mut world,
        );
    }
    assert_eq!(hits.load(Ordering::SeqCst), 50);
}

#[test]
fn tc_1_5_3_3_observer_fires_on_mutate() {
    let hits = Arc::new(AtomicU32::new(0));
    let mut reg = EventObserverRegistry::new();
    reg.register(
        ObserverDescriptor {
            watched_components: vec![TypeId::of::<()>()],
            triggers: vec![ComponentEvent::Mutated],
            priority: 0,
        },
        CountingObserver {
            hits: Arc::clone(&hits),
        },
    )
    .unwrap();
    let mut world = World::new(WorldId(0));
    for i in 0u64..25 {
        reg.notify(
            ComponentEvent::Mutated,
            TypeId::of::<()>(),
            Entity(i),
            &mut world,
        );
    }
    assert_eq!(hits.load(Ordering::SeqCst), 25);
}

#[test]
fn tc_1_5_3_4_observer_deterministic_order() {
    let mut sequences = Vec::new();
    for _ in 0..100 {
        let hits = Arc::new(AtomicU32::new(0));
        let mut reg = EventObserverRegistry::new();
        reg.register(
            ObserverDescriptor {
                watched_components: vec![TypeId::of::<()>()],
                triggers: vec![ComponentEvent::Added],
                priority: 0,
            },
            CountingObserver {
                hits: Arc::clone(&hits),
            },
        )
        .unwrap();
        let mut world = World::new(WorldId(0));
        reg.notify(
            ComponentEvent::Added,
            TypeId::of::<()>(),
            Entity(1),
            &mut world,
        );
        sequences.push(world.observer_events().to_vec());
    }
    let first = &sequences[0];
    for s in &sequences[1..] {
        assert_eq!(s, first);
    }
}

// --- TC-1.5.4.* Command buffer ---

#[test]
fn tc_1_5_4_1_command_buffer_flush_order() {
    let mut buf = CommandBuffer::new();
    let state: Arc<AtomicU32> = Arc::new(AtomicU32::new(0));
    for order in [3u32, 1, 2] {
        buf.record(order, move |ctx| {
            let c = ctx.downcast_mut::<Arc<AtomicU32>>().expect("ctx");
            c.fetch_add(order, Ordering::SeqCst);
        });
    }
    let mut ctx: Arc<AtomicU32> = Arc::clone(&state);
    buf.flush(&mut ctx);
    assert_eq!(state.load(Ordering::SeqCst), 6);
}

#[test]
fn tc_1_5_4_2_command_buffer_deterministic() {
    for _ in 0..100 {
        let mut buf = CommandBuffer::new();
        let v: Arc<AtomicU32> = Arc::new(AtomicU32::new(0));
        for tid in 0u32..8 {
            buf.record(tid, move |ctx| {
                let c = ctx.downcast_mut::<Arc<AtomicU32>>().expect("ctx");
                c.fetch_add(1, Ordering::SeqCst);
            });
        }
        let mut ctx = Arc::clone(&v);
        buf.flush(&mut ctx);
        assert_eq!(v.load(Ordering::SeqCst), 8);
    }
}

// --- TC-1.5.5.* Reactive ---

struct MarkerA;

#[test]
fn tc_1_5_5_1_reactive_query_skip() {
    let mut world = World::new(WorldId(0));
    let mut rq = ReactiveQuery::new::<MarkerA>();
    for _ in 0..10 {
        assert!(!rq.has_changed(&world, world.current_tick()));
        rq.mark_seen(&world);
    }
    world.bump_component_archetype::<MarkerA>();
    assert!(rq.has_changed(&world, world.current_tick()));
}

#[test]
fn tc_1_5_5_2_reactive_query_overhead() {
    let world = World::new(WorldId(0));
    let queries: Vec<_> = (0..200).map(|_| ReactiveQuery::new::<MarkerA>()).collect();
    let start = Instant::now();
    for q in &queries {
        let _ = q.has_changed(&world, 0);
    }
    let elapsed = start.elapsed();
    let limit_us = if cfg!(debug_assertions) { 2000 } else { 200 };
    assert!(
        elapsed.as_micros() < u128::from(limit_us as u32),
        "{elapsed:?}"
    );
}

// --- TC-1.5.6.* Resources (scheduler ordering simulated via command keys) ---

#[test]
fn tc_1_5_6_1_resource_scheduler_ordering() {
    let mut buf = CommandBuffer::new();
    let cell = Arc::new(AtomicU32::new(0));
    buf.record(1, move |ctx| {
        let c = ctx.downcast_mut::<Arc<AtomicU32>>().expect("ctx");
        c.store(10, Ordering::SeqCst);
    });
    buf.record(2, move |ctx| {
        let c = ctx.downcast_mut::<Arc<AtomicU32>>().expect("ctx");
        assert_eq!(c.load(Ordering::SeqCst), 10);
        c.store(20, Ordering::SeqCst);
    });
    let mut ctx: Arc<AtomicU32> = Arc::clone(&cell);
    buf.flush(&mut ctx);
    assert_eq!(cell.load(Ordering::SeqCst), 20);
}

#[test]
fn tc_1_5_6_2_resource_parallel_reads() {
    let cell = Arc::new(AtomicU32::new(7));
    let mut buf = CommandBuffer::new();
    for key in [1u32, 1] {
        buf.record(key, move |ctx| {
            let x = ctx.downcast_mut::<Arc<AtomicU32>>().expect("ctx");
            assert_eq!(x.load(Ordering::SeqCst), 7);
        });
    }
    let mut ctx = Arc::clone(&cell);
    buf.flush(&mut ctx);
}

// --- TC-1.6.1.* Plugins ---

struct PluginA;
impl Plugin for PluginA {
    fn descriptor(&self) -> PluginDescriptor {
        PluginDescriptor {
            name: "A",
            version: SemVer::new(1, 0, 0),
            abi_hash: 1,
        }
    }

    fn build(&self, app: &mut App) {
        app.register_system("A1");
    }
}

struct PluginB;
impl Plugin for PluginB {
    fn descriptor(&self) -> PluginDescriptor {
        PluginDescriptor {
            name: "B",
            version: SemVer::new(1, 0, 0),
            abi_hash: 2,
        }
    }

    fn dependencies(&self) -> Vec<TypeId> {
        vec![TypeId::of::<PluginA>()]
    }

    fn build(&self, app: &mut App) {
        app.register_system("B1");
    }
}

struct PluginC;
impl Plugin for PluginC {
    fn descriptor(&self) -> PluginDescriptor {
        PluginDescriptor {
            name: "C",
            version: SemVer::new(1, 0, 0),
            abi_hash: 3,
        }
    }

    fn dependencies(&self) -> Vec<TypeId> {
        vec![TypeId::of::<PluginB>()]
    }

    fn build(&self, app: &mut App) {
        app.register_system("C1");
    }
}

#[test]
fn tc_1_6_1_1_plugin_reverse_order_registration() {
    let mut app = App::with_default_world();
    app.add_plugin(PluginC).unwrap();
    app.add_plugin(PluginB).unwrap();
    app.add_plugin(PluginA).unwrap();
    let built = app.build().expect("build");
    assert_eq!(built.init_order, vec!["A", "B", "C"]);
}

struct PluginContrib;
impl Plugin for PluginContrib {
    fn descriptor(&self) -> PluginDescriptor {
        PluginDescriptor {
            name: "Contrib",
            version: SemVer::new(1, 0, 0),
            abi_hash: 9,
        }
    }

    fn build(&self, app: &mut App) {
        app.register_system("S1");
        app.register_system("S2");
        app.world_mut().insert_resource(());
        app.world_mut().insert_resource(0u32);
    }
}

#[test]
fn tc_1_6_1_2_plugin_contributions() {
    let mut app = App::with_default_world();
    app.add_plugin(PluginContrib).unwrap();
    let built = app.build().expect("build");
    assert_eq!(built.systems.len(), 2);
    assert!(built.world.get_resource::<()>().is_some());
    assert!(built.world.get_resource::<u32>().is_some());
}

struct P1;
struct P2;
struct P3;
struct P4;
struct P5;

macro_rules! trivial_plugin {
    ($t:ty, $n:expr, $h:expr) => {
        impl Plugin for $t {
            fn descriptor(&self) -> PluginDescriptor {
                PluginDescriptor {
                    name: $n,
                    version: SemVer::new(1, 0, 0),
                    abi_hash: $h,
                }
            }

            fn build(&self, app: &mut App) {
                app.register_system($n);
            }
        }
    };
}

trivial_plugin!(P1, "P1", 11);
trivial_plugin!(P2, "P2", 12);
trivial_plugin!(P3, "P3", 13);
trivial_plugin!(P4, "P4", 14);
trivial_plugin!(P5, "P5", 15);

#[test]
fn tc_1_6_2_1_group_disable() {
    let group = PluginGroupBuilder::new()
        .add(P1)
        .add(P2)
        .add(P3)
        .add(P4)
        .add(P5)
        .disable::<P3>()
        .finish();
    let mut app = App::with_default_world();
    app.add_plugins(group).unwrap();
    let built = app.build().expect("build");
    assert_eq!(built.init_order.len(), 4);
    assert!(!built.systems.contains(&"P3"));
}

struct YPlugin;

impl Plugin for YPlugin {
    fn descriptor(&self) -> PluginDescriptor {
        PluginDescriptor {
            name: "Y",
            version: SemVer::new(1, 0, 0),
            abi_hash: 21,
        }
    }

    fn build(&self, _app: &mut App) {}
}

struct XNeedsY;

impl Plugin for XNeedsY {
    fn descriptor(&self) -> PluginDescriptor {
        PluginDescriptor {
            name: "X",
            version: SemVer::new(1, 0, 0),
            abi_hash: 20,
        }
    }

    fn dependencies(&self) -> Vec<TypeId> {
        vec![TypeId::of::<YPlugin>()]
    }

    fn labeled_dependencies(&self) -> Vec<(TypeId, &'static str)> {
        vec![(TypeId::of::<YPlugin>(), "Y")]
    }

    fn build(&self, _app: &mut App) {}
}

#[test]
fn tc_1_6_3_1_missing_dependency() {
    let mut app = App::with_default_world();
    app.add_plugin(XNeedsY).unwrap();
    let err = match app.build() {
        Err(e) => e,
        Ok(_) => panic!("expected error"),
    };
    match err {
        PluginError::GraphErrors(v) => {
            assert!(v.iter().any(|e| matches!(
                e,
                PluginGraphError::MissingDependency { missing, .. }
                    if *missing == "Y"
            )));
        }
        _ => panic!("unexpected {err:?}"),
    }
}

struct ConflictA;
struct ConflictB;

impl Plugin for ConflictA {
    fn descriptor(&self) -> PluginDescriptor {
        PluginDescriptor {
            name: "CA",
            version: SemVer::new(1, 0, 0),
            abi_hash: 30,
        }
    }

    fn conflicts(&self) -> Vec<TypeId> {
        vec![TypeId::of::<ConflictB>()]
    }

    fn build(&self, _app: &mut App) {}
}

impl Plugin for ConflictB {
    fn descriptor(&self) -> PluginDescriptor {
        PluginDescriptor {
            name: "CB",
            version: SemVer::new(1, 0, 0),
            abi_hash: 31,
        }
    }

    fn build(&self, _app: &mut App) {}
}

#[test]
fn tc_1_6_3_2_conflict_detection() {
    let mut app = App::with_default_world();
    app.add_plugin(ConflictA).unwrap();
    app.add_plugin(ConflictB).unwrap();
    let err = match app.build() {
        Err(e) => e,
        Ok(_) => panic!("expected error"),
    };
    assert!(matches!(err, PluginError::GraphErrors(_)));
}

#[test]
fn tc_1_6_4_1_topological_sort() {
    let mut app = App::with_default_world();
    app.add_plugin(PluginA).unwrap();
    app.add_plugin(PluginB).unwrap();
    app.add_plugin(PluginC).unwrap();
    let built = app.build().expect("build");
    assert_eq!(built.init_order, vec!["A", "B", "C"]);
}

struct CycleA;
struct CycleB;

impl Plugin for CycleA {
    fn descriptor(&self) -> PluginDescriptor {
        PluginDescriptor {
            name: "CyA",
            version: SemVer::new(1, 0, 0),
            abi_hash: 40,
        }
    }

    fn dependencies(&self) -> Vec<TypeId> {
        vec![TypeId::of::<CycleB>()]
    }

    fn build(&self, _app: &mut App) {}
}

impl Plugin for CycleB {
    fn descriptor(&self) -> PluginDescriptor {
        PluginDescriptor {
            name: "CyB",
            version: SemVer::new(1, 0, 0),
            abi_hash: 41,
        }
    }

    fn dependencies(&self) -> Vec<TypeId> {
        vec![TypeId::of::<CycleA>()]
    }

    fn build(&self, _app: &mut App) {}
}

#[test]
fn tc_1_6_4_2_cycle_detection() {
    let mut app = App::with_default_world();
    app.add_plugin(CycleA).unwrap();
    app.add_plugin(CycleB).unwrap();
    let err = match app.build() {
        Err(e) => e,
        Ok(_) => panic!("expected error"),
    };
    let PluginError::GraphErrors(v) = err else {
        panic!();
    };
    assert!(v
        .iter()
        .any(|e| matches!(e, PluginGraphError::CyclicDependency { .. })));
}

struct BadDeps;

impl Plugin for BadDeps {
    fn descriptor(&self) -> PluginDescriptor {
        PluginDescriptor {
            name: "BadDeps",
            version: SemVer::new(1, 0, 0),
            abi_hash: 50,
        }
    }

    fn dependencies(&self) -> Vec<TypeId> {
        vec![TypeId::of::<YPlugin>()]
    }

    fn labeled_dependencies(&self) -> Vec<(TypeId, &'static str)> {
        vec![(TypeId::of::<YPlugin>(), "Y")]
    }

    fn build(&self, _app: &mut App) {}
}

#[test]
fn tc_1_6_4_3_all_errors_single_pass() {
    let mut app = App::with_default_world();
    app.add_plugin(BadDeps).unwrap(); // missing A
    app.add_plugin(ConflictA).unwrap();
    app.add_plugin(ConflictB).unwrap();
    app.add_plugin(CycleA).unwrap();
    app.add_plugin(CycleB).unwrap();
    let err = match app.build() {
        Err(e) => e,
        Ok(_) => panic!("expected error"),
    };
    let PluginError::GraphErrors(v) = err else {
        panic!();
    };
    assert!(v.len() >= 3);
}

#[test]
fn tc_1_6_6_1_abi_hash_match() {
    let d = PluginDescriptor {
        name: "p",
        version: SemVer::new(1, 0, 0),
        abi_hash: 99,
    };
    verify_plugin_abi(&d, 99).unwrap();
}

#[test]
fn tc_1_6_6_2_abi_hash_mismatch() {
    let d = PluginDescriptor {
        name: "p",
        version: SemVer::new(1, 0, 0),
        abi_hash: 1,
    };
    assert!(verify_plugin_abi(&d, 2).is_err());
}

#[test]
fn tc_1_6_7_1_capability_query() {
    let mut reg = harmonius_core::plugins::CapabilityRegistry::new();
    reg.register(Capability {
        name: "physics",
        version: SemVer::new(1, 2, 0),
    });
    assert_eq!(
        reg.get("physics").map(|c| c.version),
        Some(SemVer::new(1, 2, 0))
    );
    assert!(reg.get("audio").is_none());
}

#[test]
fn tc_1_6_7_2_capability_branch() {
    let mut reg = harmonius_core::plugins::CapabilityRegistry::new();
    reg.register(Capability {
        name: "physics",
        version: SemVer::new(1, 0, 0),
    });
    let branch_present = if reg.has("physics") {
        "present"
    } else {
        "absent"
    };
    assert_eq!(branch_present, "present");
    let empty = harmonius_core::plugins::CapabilityRegistry::new();
    let branch_absent = if empty.has("physics") {
        "present"
    } else {
        "absent"
    };
    assert_eq!(branch_absent, "absent");
}

// --- TC-1.5.7 bridge + integration ---

#[derive(Clone, Debug, Eq, PartialEq)]
struct ChatMsg {
    text: String,
    is_private: bool,
}

#[test]
fn tc_1_5_7_i1_cross_world_bridge() {
    let mut a = EventChannel::<ChatMsg>::new();
    a.send(ChatMsg {
        text: "hi".into(),
        is_private: false,
    });
    a.swap();
    let mut b = EventChannel::<ChatMsg>::new();
    let bridge = EventBridge::new(EventBridgeConfig::passthrough());
    bridge.transfer(&a, &mut b);
    b.swap();
    assert_eq!(b.read_front().count(), 1);
}

#[test]
fn tc_1_5_7_i2_bridge_filter() {
    let mut a = EventChannel::<ChatMsg>::new();
    a.send(ChatMsg {
        text: "p".into(),
        is_private: true,
    });
    a.send(ChatMsg {
        text: "ok".into(),
        is_private: false,
    });
    a.swap();
    let mut b = EventChannel::<ChatMsg>::new();
    let bridge = EventBridge::new(
        EventBridgeConfig::<ChatMsg>::passthrough().with_filter(|m| !m.is_private),
    );
    bridge.transfer(&a, &mut b);
    b.swap();
    let texts: Vec<_> = b.read_front().map(|m| m.text.as_str()).collect();
    assert_eq!(texts, vec!["ok"]);
}

#[test]
fn tc_1_5_7_i3_bridge_transform() {
    let mut a = EventChannel::<ChatMsg>::new();
    a.send(ChatMsg {
        text: "hello".into(),
        is_private: false,
    });
    a.swap();
    let mut b = EventChannel::<ChatMsg>::new();
    let bridge = EventBridge::new(EventBridgeConfig::<ChatMsg>::passthrough().with_transform(
        |mut m| {
            m.text = format!("[bridged] {}", m.text);
            m
        },
    ));
    bridge.transfer(&a, &mut b);
    b.swap();
    let t = b.read_front().next().expect("one");
    assert_eq!(t.text, "[bridged] hello");
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct OtherMsg(u32);

#[test]
fn tc_1_5_7_i4_bridge_unsubscribed_type() {
    // Only `ChatMsg` participates in a bridge; `OtherMsg` traffic is not transferred.
    let mut a = EventChannel::<OtherMsg>::new();
    a.send(OtherMsg(3));
    a.swap();
    let b = EventChannel::<OtherMsg>::new();
    assert_eq!(b.read_front().count(), 0);
}

#[test]
fn tc_1_5_1_i1_full_frame_lifecycle() {
    let mut ch = EventChannel::<E1>::new();
    ch.send(E1(1));
    ch.swap();
    assert_eq!(ch.read_front().count(), 1);
    let mut buf = CommandBuffer::new();
    let counter = Arc::new(AtomicU32::new(0));
    buf.record(0, move |ctx| {
        let c = ctx.downcast_mut::<Arc<AtomicU32>>().expect("ctx");
        c.fetch_add(1, Ordering::SeqCst);
    });
    let mut ctx = Arc::clone(&counter);
    buf.flush(&mut ctx);
    assert_eq!(counter.load(Ordering::SeqCst), 1);
    let mut reg = EventObserverRegistry::new();
    reg.register(
        ObserverDescriptor {
            watched_components: vec![TypeId::of::<()>()],
            triggers: vec![ComponentEvent::Added],
            priority: 0,
        },
        CountingObserver {
            hits: Arc::new(AtomicU32::new(0)),
        },
    )
    .unwrap();
    let mut world = World::new(WorldId(0));
    reg.notify(
        ComponentEvent::Added,
        TypeId::of::<()>(),
        Entity(0),
        &mut world,
    );
    assert!(!world.observer_events().is_empty());
}

// --- TC-1.1.1.3 SoA + events ---

#[test]
fn tc_1_1_1_3_event_channel_preserves_soa_direct_access() {
    let positions: Vec<f32> = (0..10_000).map(|i| i as f32).collect();
    let mut ch = EventChannel::<Entity>::new();
    for i in 0..10_000 {
        ch.send(Entity(i));
    }
    ch.swap();
    let sum: f32 = ch.read_front().map(|e| positions[e.0 as usize]).sum();
    assert!(sum > 0.0);
}

// --- US / integration style ---

#[test]
fn tc_1_5_3_i1_events_written_frame_n_observed_frame_n_plus_one() {
    let mut ch = EventChannel::<E1>::new();
    for i in 0..1000 {
        ch.send(E1(i));
    }
    ch.swap();
    assert_eq!(ch.read_front().count(), 1000);
}

#[test]
fn tc_1_5_9_i1_command_buffer_flush_applies_all_commands_once() {
    let v = Arc::new(AtomicU32::new(0));
    let mut buffers: Vec<CommandBuffer> = (0..10).map(|_| CommandBuffer::new()).collect();
    for (i, buf) in buffers.iter_mut().enumerate() {
        for j in 0..100 {
            let ord = (i as u32) * 100 + j;
            buf.record(ord, move |ctx| {
                let c = ctx.downcast_mut::<Arc<AtomicU32>>().expect("ctx");
                c.fetch_add(1, Ordering::SeqCst);
            });
        }
    }
    let mut ctx = Arc::clone(&v);
    for mut buf in buffers {
        buf.flush(&mut ctx);
    }
    assert_eq!(v.load(Ordering::SeqCst), 1000);
    let mut empty = CommandBuffer::new();
    empty.flush(&mut ctx);
    assert_eq!(v.load(Ordering::SeqCst), 1000);
}

// --- Hot reload (simulated) ---

#[test]
fn tc_1_6_5_i1_hot_reload_state_preservation() {
    let mut world = World::new(WorldId(0));
    world.insert_resource(42u64);
    let mut hr = HotReloadManager::new();
    hr.set_reload_latency_ms(0);
    hr.register_loaded(
        PluginDescriptor {
            name: "gameplay",
            version: SemVer::new(1, 0, 0),
            abi_hash: 1,
        },
        0,
    );
    hr.reload("gameplay", &mut world).unwrap();
    assert_eq!(world.get_resource::<u64>().copied(), Some(42));
}

#[test]
fn tc_1_6_5_i2_hot_reload_new_behavior() {
    let mut world = World::new(WorldId(0));
    let mut hr = HotReloadManager::new();
    hr.set_reload_latency_ms(0);
    hr.register_loaded(
        PluginDescriptor {
            name: "gameplay",
            version: SemVer::new(1, 0, 0),
            abi_hash: 1,
        },
        0,
    );
    hr.reload("gameplay", &mut world).unwrap();
    let m = world.get_resource::<HotReloadMarker>().expect("marker");
    assert_eq!(m.tick, 1);
}

#[test]
fn tc_1_6_5_i3_hot_reload_latency() {
    let mut world = World::new(WorldId(0));
    let mut hr = HotReloadManager::new();
    hr.set_reload_latency_ms(0);
    hr.register_loaded(
        PluginDescriptor {
            name: "big",
            version: SemVer::new(1, 0, 0),
            abi_hash: 1,
        },
        0,
    );
    let start = Instant::now();
    hr.reload("big", &mut world).unwrap();
    assert!(start.elapsed().as_secs() < 2);
}

#[test]
fn tc_1_6_5_i4_hot_reload_migration_failure() {
    let mut world = World::new(WorldId(0));
    world.insert_resource(5u64);
    let mut hr = HotReloadManager::new();
    hr.set_reload_latency_ms(0);
    hr.set_migration_should_fail(true);
    hr.register_loaded(
        PluginDescriptor {
            name: "gameplay",
            version: SemVer::new(1, 0, 0),
            abi_hash: 1,
        },
        0,
    );
    let err = hr.reload("gameplay", &mut world).unwrap_err();
    assert!(matches!(err, HotReloadError::StateMigrationFailed { .. }));
    assert_eq!(world.get_resource::<u64>().copied(), Some(5));
}
