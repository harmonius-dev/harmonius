//! Unit tests mapped to `docs/design/ai/behavior-test-cases.md` (PLAN-ai-behavior scope).

use harmonius_ai::blackboard::{
    get_self, resolve_with_group, BlackboardKey, BlackboardValue, GroupBlackboardStore, GroupId,
    ObserverRegistry, ScopedBlackboard,
};
use harmonius_ai::bt::{
    tick_tree, AbortMode, BehaviorTreeAsset, BlackboardSchema, BtNode, BtTickContext, BtTickState,
    CompositeData, DecoratorData, DecoratorKind, LeafId, NodeId, NodeStatus, ParallelData,
    ParallelPolicy,
};
use harmonius_ai::debug_trace::TraceLog;
use harmonius_ai::goap::{
    hash_world, plan_dijkstra, GoapAction, GoapActionRegistry, GoapAgent, GoapGoal, Plan,
    PlanCache, PlanCacheKey, WorldState, WorldStateDelta,
};
use harmonius_ai::subtree::{validate_subtree_graph, SubtreeError};
use harmonius_ai::utility::{
    compensate_product, dual_axis_pick, evaluate_builtins, hysteresis_should_switch,
    score_action_with_inputs, score_custom, select_highest, weighted_random_index, Consideration,
    CustomConsideration, InputAxis, ResponseCurve,
};
use rand::rngs::SmallRng;
use rand::SeedableRng;
use std::cell::{Cell, RefCell};
use std::collections::HashMap;
use std::rc::Rc;

const K_HP: BlackboardKey = BlackboardKey(1);

fn leaf_table<'a>(table: &'a [(LeafId, NodeStatus)]) -> impl FnMut(LeafId) -> NodeStatus + 'a {
    move |id: LeafId| {
        table
            .iter()
            .find(|(l, _)| *l == id)
            .map(|(_, s)| *s)
            .unwrap_or(NodeStatus::Failure)
    }
}

fn mk_asset(root: NodeId, nodes: Vec<BtNode>, labels: Vec<String>) -> BehaviorTreeAsset {
    let mut a = BehaviorTreeAsset {
        root,
        nodes,
        labels,
        blackboard_schema: BlackboardSchema::default(),
    };
    a.normalize_labels();
    a
}

// TC-7.3.1.1
#[test]
fn tc_7_3_1_1_sequence_fail_fast() {
    let nodes = vec![
        BtNode::Sequence(CompositeData {
            children: vec![NodeId(1), NodeId(2), NodeId(3)],
            abort_mode: AbortMode::None,
        }),
        BtNode::Leaf(LeafId(0)),
        BtNode::Leaf(LeafId(1)),
        BtNode::Leaf(LeafId(2)),
    ];
    let asset = mk_asset(
        NodeId(0),
        nodes,
        vec!["S".into(), "L0".into(), "L1".into(), "L2".into()],
    );
    let mut st = BtTickState::new(asset.nodes.len());
    let mut tab = [
        (LeafId(0), NodeStatus::Success),
        (LeafId(1), NodeStatus::Failure),
        (LeafId(2), NodeStatus::Success),
    ];
    let mut leaf = leaf_table(&tab);
    let mut ctx = BtTickContext::new(0.0, &mut leaf);
    let r = tick_tree(&asset, &mut st, &mut ctx);
    assert_eq!(r, NodeStatus::Failure);
    let mut tab2 = [
        (LeafId(0), NodeStatus::Success),
        (LeafId(1), NodeStatus::Success),
        (LeafId(2), NodeStatus::Success),
    ];
    let mut leaf2 = leaf_table(&tab2);
    let mut st2 = BtTickState::new(asset.nodes.len());
    let mut ctx2 = BtTickContext::new(0.0, &mut leaf2);
    assert_eq!(tick_tree(&asset, &mut st2, &mut ctx2), NodeStatus::Success);
}

// TC-7.3.1.2
#[test]
fn tc_7_3_1_2_selector_succeed_fast() {
    let nodes = vec![
        BtNode::Selector(CompositeData {
            children: vec![NodeId(1), NodeId(2), NodeId(3)],
            abort_mode: AbortMode::None,
        }),
        BtNode::Leaf(LeafId(0)),
        BtNode::Leaf(LeafId(1)),
        BtNode::Leaf(LeafId(2)),
    ];
    let asset = mk_asset(
        NodeId(0),
        nodes,
        vec!["Sel".into(), "a".into(), "b".into(), "c".into()],
    );
    let mut st = BtTickState::new(asset.nodes.len());
    let mut tab = [
        (LeafId(0), NodeStatus::Failure),
        (LeafId(1), NodeStatus::Success),
        (LeafId(2), NodeStatus::Failure),
    ];
    let mut leaf = leaf_table(&tab);
    let mut ctx = BtTickContext::new(0.0, &mut leaf);
    assert_eq!(tick_tree(&asset, &mut st, &mut ctx), NodeStatus::Success);
    let mut tab2 = [
        (LeafId(0), NodeStatus::Failure),
        (LeafId(1), NodeStatus::Failure),
        (LeafId(2), NodeStatus::Failure),
    ];
    let mut leaf2 = leaf_table(&tab2);
    let mut st2 = BtTickState::new(asset.nodes.len());
    let mut ctx2 = BtTickContext::new(0.0, &mut leaf2);
    assert_eq!(tick_tree(&asset, &mut st2, &mut ctx2), NodeStatus::Failure);
}

// TC-7.3.1.3 / 1.4 parallel
#[test]
fn tc_7_3_1_3_parallel_require_all() {
    let nodes = vec![
        BtNode::Parallel(ParallelData {
            children: vec![NodeId(1), NodeId(2), NodeId(3)],
            success_policy: ParallelPolicy::RequireAll,
            failure_policy: ParallelPolicy::RequireAll,
        }),
        BtNode::Leaf(LeafId(0)),
        BtNode::Leaf(LeafId(1)),
        BtNode::Leaf(LeafId(2)),
    ];
    let asset = mk_asset(NodeId(0), nodes, vec!["P".into(); 4]);
    let mut st = BtTickState::new(asset.nodes.len());
    let mut tab = [
        (LeafId(0), NodeStatus::Success),
        (LeafId(1), NodeStatus::Success),
        (LeafId(2), NodeStatus::Success),
    ];
    let mut leaf = leaf_table(&tab);
    let mut ctx = BtTickContext::new(0.0, &mut leaf);
    assert_eq!(tick_tree(&asset, &mut st, &mut ctx), NodeStatus::Success);
    let mut tab2 = [
        (LeafId(0), NodeStatus::Success),
        (LeafId(1), NodeStatus::Failure),
        (LeafId(2), NodeStatus::Success),
    ];
    let mut leaf2 = leaf_table(&tab2);
    let mut st2 = BtTickState::new(asset.nodes.len());
    let mut ctx2 = BtTickContext::new(0.0, &mut leaf2);
    assert_eq!(tick_tree(&asset, &mut st2, &mut ctx2), NodeStatus::Failure);
}

#[test]
fn tc_7_3_1_4_parallel_require_one() {
    let nodes = vec![
        BtNode::Parallel(ParallelData {
            children: vec![NodeId(1), NodeId(2), NodeId(3)],
            success_policy: ParallelPolicy::RequireOne,
            failure_policy: ParallelPolicy::RequireOne,
        }),
        BtNode::Leaf(LeafId(0)),
        BtNode::Leaf(LeafId(1)),
        BtNode::Leaf(LeafId(2)),
    ];
    let asset = mk_asset(NodeId(0), nodes, vec!["P".into(); 4]);
    let mut st = BtTickState::new(asset.nodes.len());
    let mut tab = [
        (LeafId(0), NodeStatus::Failure),
        (LeafId(1), NodeStatus::Success),
        (LeafId(2), NodeStatus::Failure),
    ];
    let mut leaf = leaf_table(&tab);
    let mut ctx = BtTickContext::new(0.0, &mut leaf);
    assert_eq!(tick_tree(&asset, &mut st, &mut ctx), NodeStatus::Success);
    let mut tab2 = [
        (LeafId(0), NodeStatus::Failure),
        (LeafId(1), NodeStatus::Failure),
        (LeafId(2), NodeStatus::Failure),
    ];
    let mut leaf2 = leaf_table(&tab2);
    let mut st2 = BtTickState::new(asset.nodes.len());
    let mut ctx2 = BtTickContext::new(0.0, &mut leaf2);
    assert_eq!(tick_tree(&asset, &mut st2, &mut ctx2), NodeStatus::Failure);
}

// TC-7.3.2.1
#[test]
fn tc_7_3_2_1_inverter() {
    let nodes = vec![
        BtNode::Decorator(DecoratorData {
            child: NodeId(1),
            kind: DecoratorKind::Inverter,
        }),
        BtNode::Leaf(LeafId(0)),
    ];
    let asset = mk_asset(NodeId(0), nodes, vec!["D".into(), "L".into()]);
    let mut st = BtTickState::new(asset.nodes.len());
    let mut tab = [(LeafId(0), NodeStatus::Success)];
    let mut leaf = leaf_table(&tab);
    let mut ctx = BtTickContext::new(0.0, &mut leaf);
    assert_eq!(tick_tree(&asset, &mut st, &mut ctx), NodeStatus::Failure);
    let mut tab2 = [(LeafId(0), NodeStatus::Failure)];
    let mut leaf2 = leaf_table(&tab2);
    let mut st2 = BtTickState::new(asset.nodes.len());
    let mut ctx2 = BtTickContext::new(0.0, &mut leaf2);
    assert_eq!(tick_tree(&asset, &mut st2, &mut ctx2), NodeStatus::Success);
    let mut tab3 = [(LeafId(0), NodeStatus::Running)];
    let mut leaf3 = leaf_table(&tab3);
    let mut st3 = BtTickState::new(asset.nodes.len());
    let mut ctx3 = BtTickContext::new(0.0, &mut leaf3);
    assert_eq!(tick_tree(&asset, &mut st3, &mut ctx3), NodeStatus::Running);
}

// TC-7.3.2.2 repeater
#[test]
fn tc_7_3_2_2_repeater_count() {
    let nodes = vec![
        BtNode::Decorator(DecoratorData {
            child: NodeId(1),
            kind: DecoratorKind::Repeater { count: 3 },
        }),
        BtNode::Leaf(LeafId(0)),
    ];
    let asset = mk_asset(NodeId(0), nodes, vec!["R".into(), "L".into()]);
    let count = Rc::new(Cell::new(0u32));
    let c = Rc::clone(&count);
    let mut leaf = move |_id: LeafId| {
        c.set(c.get().saturating_add(1));
        NodeStatus::Success
    };
    let mut st = BtTickState::new(asset.nodes.len());
    let mut ctx = BtTickContext::new(0.0, &mut leaf);
    assert_eq!(tick_tree(&asset, &mut st, &mut ctx), NodeStatus::Success);
    assert_eq!(count.get(), 3);

    let nodes5 = vec![
        BtNode::Decorator(DecoratorData {
            child: NodeId(1),
            kind: DecoratorKind::Repeater { count: 5 },
        }),
        BtNode::Leaf(LeafId(0)),
    ];
    let asset5 = mk_asset(NodeId(0), nodes5, vec!["R".into(), "L".into()]);
    let c2 = Rc::new(Cell::new(0u32));
    let c2b = Rc::clone(&c2);
    let mut leaf5 = move |_id: LeafId| {
        c2b.set(c2b.get().saturating_add(1));
        NodeStatus::Success
    };
    let mut st5 = BtTickState::new(asset5.nodes.len());
    let mut ctx5 = BtTickContext::new(0.0, &mut leaf5);
    assert_eq!(tick_tree(&asset5, &mut st5, &mut ctx5), NodeStatus::Success);
    assert_eq!(c2.get(), 5);

    let nodesf = vec![
        BtNode::Decorator(DecoratorData {
            child: NodeId(1),
            kind: DecoratorKind::Repeater { count: 3 },
        }),
        BtNode::Leaf(LeafId(0)),
    ];
    let assetf = mk_asset(NodeId(0), nodesf, vec!["R".into(), "L".into()]);
    let c3 = Rc::new(Cell::new(0u32));
    let c3b = Rc::clone(&c3);
    let mut leaff = move |_id: LeafId| {
        c3b.set(c3b.get().saturating_add(1));
        NodeStatus::Failure
    };
    let mut stf = BtTickState::new(assetf.nodes.len());
    let mut ctxf = BtTickContext::new(0.0, &mut leaff);
    assert_eq!(tick_tree(&assetf, &mut stf, &mut ctxf), NodeStatus::Failure);
    assert_eq!(c3.get(), 1);
}

// TC-7.3.2.3 cooldown
#[test]
fn tc_7_3_2_3_cooldown_blocks_reentry() {
    let nodes = vec![
        BtNode::Decorator(DecoratorData {
            child: NodeId(1),
            kind: DecoratorKind::Cooldown { duration_secs: 2.0 },
        }),
        BtNode::Leaf(LeafId(0)),
    ];
    let asset = mk_asset(NodeId(0), nodes, vec!["C".into(), "L".into()]);
    let mut st = BtTickState::new(asset.nodes.len());
    let mut tab = [(LeafId(0), NodeStatus::Success)];
    let mut leaf = leaf_table(&tab);
    let mut ctx = BtTickContext::new(0.0, &mut leaf);
    assert_eq!(tick_tree(&asset, &mut st, &mut ctx), NodeStatus::Success);
    let mut leaf2 = leaf_table(&tab);
    let mut ctx2 = BtTickContext::new(1.0, &mut leaf2);
    assert_eq!(tick_tree(&asset, &mut st, &mut ctx2), NodeStatus::Failure);
    let mut leaf3 = leaf_table(&tab);
    let mut ctx3 = BtTickContext::new(2.5, &mut leaf3);
    assert_eq!(tick_tree(&asset, &mut st, &mut ctx3), NodeStatus::Success);
}

// TC-7.3.2.4 rate limiter
#[test]
fn tc_7_3_2_4_rate_limiter_frequency() {
    let nodes = vec![
        BtNode::Decorator(DecoratorData {
            child: NodeId(1),
            kind: DecoratorKind::RateLimiter { hz: 2.0 },
        }),
        BtNode::Leaf(LeafId(0)),
    ];
    let asset = mk_asset(NodeId(0), nodes, vec!["RL".into(), "L".into()]);
    let ticks = Rc::new(Cell::new(0u32));
    let tb = Rc::clone(&ticks);
    let mut leaf = move |_id: LeafId| {
        tb.set(tb.get().saturating_add(1));
        NodeStatus::Success
    };
    let mut st = BtTickState::new(asset.nodes.len());
    for _ in 0..10 {
        let mut ctx = BtTickContext::new(0.0, &mut leaf);
        ctx.dt_secs = 0.1;
        let _ = tick_tree(&asset, &mut st, &mut ctx);
    }
    assert_eq!(ticks.get(), 2);

    let nodes10 = vec![
        BtNode::Decorator(DecoratorData {
            child: NodeId(1),
            kind: DecoratorKind::RateLimiter { hz: 10.0 },
        }),
        BtNode::Leaf(LeafId(0)),
    ];
    let asset10 = mk_asset(NodeId(0), nodes10, vec!["RL".into(), "L".into()]);
    let ticks2 = Rc::new(Cell::new(0u32));
    let t2b = Rc::clone(&ticks2);
    let mut leaf2 = move |_id: LeafId| {
        t2b.set(t2b.get().saturating_add(1));
        NodeStatus::Success
    };
    let mut st2 = BtTickState::new(asset10.nodes.len());
    for _ in 0..20 {
        let mut ctx = BtTickContext::new(0.0, &mut leaf2);
        ctx.dt_secs = 0.1;
        let _ = tick_tree(&asset10, &mut st2, &mut ctx);
    }
    assert_eq!(ticks2.get(), 20);
}

// TC-7.3.3.1 self abort
#[test]
fn tc_7_3_3_1_self_abort() {
    let nodes = vec![
        BtNode::Sequence(CompositeData {
            children: vec![NodeId(1), NodeId(2)],
            abort_mode: AbortMode::SelfAbort,
        }),
        BtNode::Leaf(LeafId(0)),
        BtNode::Leaf(LeafId(1)),
    ];
    let asset = mk_asset(NodeId(0), nodes, vec!["Seq".into(), "G".into(), "B".into()]);
    let guard_ok = Cell::new(true);
    let body_running = Cell::new(true);
    let mut leaf = |id: LeafId| {
        if id == LeafId(0) {
            if guard_ok.get() {
                NodeStatus::Success
            } else {
                NodeStatus::Failure
            }
        } else if id == LeafId(1) {
            if body_running.get() {
                NodeStatus::Running
            } else {
                NodeStatus::Success
            }
        } else {
            NodeStatus::Failure
        }
    };
    let mut st = BtTickState::new(asset.nodes.len());
    let mut ctx = BtTickContext::new(0.0, &mut leaf);
    assert_eq!(tick_tree(&asset, &mut st, &mut ctx), NodeStatus::Running);
    guard_ok.set(false);
    let mut leaf2 = |id: LeafId| {
        if id == LeafId(0) {
            if guard_ok.get() {
                NodeStatus::Success
            } else {
                NodeStatus::Failure
            }
        } else if id == LeafId(1) {
            if body_running.get() {
                NodeStatus::Running
            } else {
                NodeStatus::Success
            }
        } else {
            NodeStatus::Failure
        }
    };
    let mut ctx2 = BtTickContext::new(0.0, &mut leaf2);
    assert_eq!(tick_tree(&asset, &mut st, &mut ctx2), NodeStatus::Failure);

    guard_ok.set(true);
    body_running.set(true);
    let nodes2 = vec![
        BtNode::Sequence(CompositeData {
            children: vec![NodeId(1), NodeId(2)],
            abort_mode: AbortMode::SelfAbort,
        }),
        BtNode::Leaf(LeafId(0)),
        BtNode::Leaf(LeafId(1)),
    ];
    let asset2 = mk_asset(
        NodeId(0),
        nodes2,
        vec!["Seq".into(), "G".into(), "B".into()],
    );
    let mut st2 = BtTickState::new(asset2.nodes.len());
    let guard_ok2 = Cell::new(true);
    let mut leaf3 = |id: LeafId| {
        if id == LeafId(0) {
            if guard_ok2.get() {
                NodeStatus::Success
            } else {
                NodeStatus::Failure
            }
        } else {
            NodeStatus::Running
        }
    };
    let mut ctx3 = BtTickContext::new(0.0, &mut leaf3);
    assert_eq!(tick_tree(&asset2, &mut st2, &mut ctx3), NodeStatus::Running);
}

// TC-7.3.3.2 lower priority abort
#[test]
fn tc_7_3_3_2_lower_priority_abort() {
    let nodes = vec![
        BtNode::Selector(CompositeData {
            children: vec![NodeId(1), NodeId(2)],
            abort_mode: AbortMode::LowerPriority,
        }),
        BtNode::Leaf(LeafId(0)),
        BtNode::Leaf(LeafId(1)),
    ];
    let asset = mk_asset(NodeId(0), nodes, vec!["Sel".into(), "A".into(), "B".into()]);
    let a_succ = Cell::new(false);
    let mut leaf = |id: LeafId| {
        if id == LeafId(0) {
            if a_succ.get() {
                NodeStatus::Success
            } else {
                NodeStatus::Failure
            }
        } else {
            NodeStatus::Running
        }
    };
    let mut st = BtTickState::new(asset.nodes.len());
    let mut ctx = BtTickContext::new(0.0, &mut leaf);
    assert_eq!(tick_tree(&asset, &mut st, &mut ctx), NodeStatus::Running);
    a_succ.set(true);
    let mut leaf2 = |id: LeafId| {
        if id == LeafId(0) {
            if a_succ.get() {
                NodeStatus::Success
            } else {
                NodeStatus::Failure
            }
        } else {
            NodeStatus::Running
        }
    };
    let mut ctx2 = BtTickContext::new(0.0, &mut leaf2);
    assert_eq!(tick_tree(&asset, &mut st, &mut ctx2), NodeStatus::Success);

    let nodes2 = vec![
        BtNode::Selector(CompositeData {
            children: vec![NodeId(1), NodeId(2)],
            abort_mode: AbortMode::LowerPriority,
        }),
        BtNode::Leaf(LeafId(0)),
        BtNode::Leaf(LeafId(1)),
    ];
    let asset2 = mk_asset(
        NodeId(0),
        nodes2,
        vec!["Sel".into(), "A".into(), "B".into()],
    );
    let mut st2 = BtTickState::new(asset2.nodes.len());
    let mut leaf3 = |id: LeafId| {
        if id == LeafId(0) {
            NodeStatus::Failure
        } else {
            NodeStatus::Running
        }
    };
    let mut ctx3 = BtTickContext::new(0.0, &mut leaf3);
    assert_eq!(tick_tree(&asset2, &mut st2, &mut ctx3), NodeStatus::Running);
}

// TC-7.3.3.3 abort cleanup
#[test]
fn tc_7_3_3_3_abort_no_state_leak() {
    let nodes = vec![
        BtNode::Sequence(CompositeData {
            children: vec![NodeId(1), NodeId(2)],
            abort_mode: AbortMode::SelfAbort,
        }),
        BtNode::Leaf(LeafId(0)),
        BtNode::Leaf(LeafId(1)),
    ];
    let asset = mk_asset(NodeId(0), nodes, vec!["Seq".into(), "G".into(), "B".into()]);
    let bb = Rc::new(RefCell::new(ScopedBlackboard::new()));
    let k_temp = BlackboardKey(99);
    let guard_ok = Rc::new(Cell::new(true));
    let bb_l = Rc::clone(&bb);
    let g_ok = Rc::clone(&guard_ok);
    let mut leaf = move |id: LeafId| {
        if id == LeafId(0) {
            if g_ok.get() {
                NodeStatus::Success
            } else {
                NodeStatus::Failure
            }
        } else {
            bb_l.borrow_mut().set(k_temp, BlackboardValue::Int(1));
            NodeStatus::Running
        }
    };
    let mut st = BtTickState::new(asset.nodes.len());
    let mut ctx = BtTickContext::new(0.0, &mut leaf);
    let _ = tick_tree(&asset, &mut st, &mut ctx);
    assert!(bb.borrow().get(k_temp).is_some());
    guard_ok.set(false);
    let hook_calls = Rc::new(Cell::new(0u32));
    let bb_h = Rc::clone(&bb);
    let hc = Rc::clone(&hook_calls);
    let mut hook = move || {
        hc.set(hc.get().saturating_add(1));
        bb_h.borrow_mut().remove(k_temp);
    };
    let bb_l2 = Rc::clone(&bb);
    let mut leaf2 = move |id: LeafId| {
        if id == LeafId(0) {
            NodeStatus::Failure
        } else {
            bb_l2.borrow_mut().set(k_temp, BlackboardValue::Int(1));
            NodeStatus::Running
        }
    };
    let mut ctx2 = BtTickContext::new(0.0, &mut leaf2);
    ctx2.on_abort = Some(&mut hook);
    let _ = tick_tree(&asset, &mut st, &mut ctx2);
    assert!(bb.borrow().get(k_temp).is_none());
    assert_eq!(hook_calls.get(), 1);

    let mut st3 = BtTickState::new(asset.nodes.len());
    let mut leaf3 = |id: LeafId| {
        if id == LeafId(0) {
            NodeStatus::Success
        } else {
            NodeStatus::Running
        }
    };
    let mut ctx3 = BtTickContext::new(0.0, &mut leaf3);
    let _ = tick_tree(&asset, &mut st3, &mut ctx3);
    st3.reset_subtree(&asset, NodeId(2));
    assert_eq!(st3.node_status[2], NodeStatus::Idle);
}

// TC-7.3.4.*
#[test]
fn tc_7_3_4_1_blackboard_self_scope() {
    let mut a = ScopedBlackboard::new();
    let mut b = ScopedBlackboard::new();
    a.set(K_HP, BlackboardValue::Int(50));
    assert!(get_self(&b, K_HP).is_none());
    assert_eq!(get_self(&a, K_HP), Some(&BlackboardValue::Int(50)));
}

#[test]
fn tc_7_3_4_2_blackboard_group_scope() {
    let mut groups = GroupBlackboardStore::new();
    let g1 = GroupId(1);
    groups.set(g1, K_HP, BlackboardValue::Bool(true));
    let mut agent_a = ScopedBlackboard::new();
    let mut agent_b = ScopedBlackboard::new();
    assert_eq!(
        resolve_with_group(&agent_a, Some(g1), &groups, K_HP),
        Some(BlackboardValue::Bool(true))
    );
    assert_eq!(
        resolve_with_group(&agent_b, Some(g1), &groups, K_HP),
        Some(BlackboardValue::Bool(true))
    );
    let g2 = GroupId(2);
    let mut agent_c = ScopedBlackboard::new();
    assert!(resolve_with_group(&agent_c, Some(g2), &groups, K_HP).is_none());
}

#[test]
fn tc_7_3_4_3_blackboard_observer() {
    let mut bb = ScopedBlackboard::new();
    let mut obs = ObserverRegistry::new();
    obs.set_and_notify(&mut bb, K_HP, BlackboardValue::Int(5));
    obs.set_and_notify(&mut bb, K_HP, BlackboardValue::Int(5));
    assert_eq!(obs.fire_count(), 1);
    let mut bb2 = ScopedBlackboard::new();
    let mut obs2 = ObserverRegistry::new();
    obs2.set_and_notify(&mut bb2, K_HP, BlackboardValue::Int(5));
    obs2.set_and_notify(&mut bb2, K_HP, BlackboardValue::Int(10));
    assert_eq!(obs2.fire_count(), 2);
}

// TC-7.3.5.1 serialization
#[test]
fn tc_7_3_5_1_bt_serialization_roundtrip() {
    let mut nodes = Vec::new();
    for i in 0..10 {
        if i < 9 {
            nodes.push(BtNode::Sequence(CompositeData {
                children: vec![NodeId(i + 1)],
                abort_mode: AbortMode::None,
            }));
        } else {
            nodes.push(BtNode::Leaf(LeafId(9)));
        }
    }
    let mut labels: Vec<String> = (0..10).map(|i| format!("n{i}")).collect();
    let asset = mk_asset(NodeId(0), nodes, labels.clone());
    let ron = ron::ser::to_string(&asset).expect("ron");
    let back: BehaviorTreeAsset = ron::de::from_str(&ron).expect("de");
    assert_eq!(asset, back);
    let js = serde_json::to_string(&asset).expect("json");
    let backj: BehaviorTreeAsset = serde_json::from_str(&js).expect("jde");
    assert_eq!(asset, backj);
}

// TC-7.3.6.1 subtree cycles
#[test]
fn tc_7_3_6_1_subtree_circular_reference_detection() {
    let mut m = HashMap::new();
    m.insert(0u8, vec![1u8]);
    m.insert(1u8, vec![0u8]);
    assert_eq!(
        validate_subtree_graph(&[0u8], &m),
        Err(SubtreeError::CircularReference)
    );
    let mut m2 = HashMap::new();
    m2.insert(0u8, vec![1u8]);
    m2.insert(1u8, vec![2u8]);
    m2.insert(2u8, vec![]);
    assert!(validate_subtree_graph(&[0u8], &m2).is_ok());
}

// TC-7.3.7.1 trace
#[test]
fn tc_7_3_7_1_trace_log_accuracy() {
    let nodes = vec![
        BtNode::Sequence(CompositeData {
            children: vec![NodeId(1), NodeId(2)],
            abort_mode: AbortMode::None,
        }),
        BtNode::Leaf(LeafId(0)),
        BtNode::Leaf(LeafId(1)),
    ];
    let labels = vec![
        "Sequence".into(),
        "Leaf(Success)".into(),
        "Leaf(Failure)".into(),
    ];
    let asset = mk_asset(NodeId(0), nodes, labels);
    let mut st = BtTickState::new(asset.nodes.len());
    let mut tab = [
        (LeafId(0), NodeStatus::Success),
        (LeafId(1), NodeStatus::Failure),
    ];
    let mut leaf = leaf_table(&tab);
    let mut trace = TraceLog::new();
    let mut ctx = BtTickContext::new(0.0, &mut leaf);
    ctx.trace = Some(&mut trace);
    let _ = tick_tree(&asset, &mut st, &mut ctx);
    let names: Vec<String> = trace.entries().iter().map(|e| e.label.clone()).collect();
    assert_eq!(
        names,
        vec![
            "Sequence".to_string(),
            "Leaf(Success)".to_string(),
            "Leaf(Failure)".to_string()
        ]
    );
}

// TC-7.4.1.*
#[test]
fn tc_7_4_1_1_linear_curve() {
    let c = ResponseCurve::Linear {
        slope: 1.0,
        intercept: 0.0,
    };
    assert!((c.evaluate(0.5) - 0.5).abs() < 1e-6);
    let c2 = ResponseCurve::Linear {
        slope: 2.0,
        intercept: -0.5,
    };
    assert!((c2.evaluate(0.75) - 1.0).abs() < 1e-5);
}

#[test]
fn tc_7_4_1_2_logistic_curve() {
    let c = ResponseCurve::Logistic {
        k: 10.0,
        midpoint: 0.5,
    };
    assert!((c.evaluate(0.5) - 0.5).abs() < 1e-3);
    assert!(c.evaluate(1.0) > 0.99);
    assert!(c.evaluate(0.0) < 0.01);
}

#[test]
fn tc_7_4_1_3_curve_clamping() {
    let c = ResponseCurve::Linear {
        slope: 1.0,
        intercept: 0.0,
    };
    assert!((c.evaluate(1.5) - 1.0).abs() < 1e-6);
    let c2 = ResponseCurve::Linear {
        slope: 1.0,
        intercept: 0.0,
    };
    assert!((c2.evaluate(-0.3) - 0.0).abs() < 1e-6);
    let q = ResponseCurve::Quadratic {
        a: 0.0,
        b: 0.0,
        c: 2.0,
    };
    assert!((q.evaluate(1.0) - 1.0).abs() < 1e-5);
}

// TC-7.4.2.*
#[test]
fn tc_7_4_2_1_compensation_fairness() {
    let a = compensate_product(0.8 * 0.8, 2);
    let b = compensate_product(0.8f32.powi(5), 5);
    assert!((a - b).abs() < 0.1 * a.max(b));
    let a2 = compensate_product(0.5, 1);
    let b2 = compensate_product(0.5f32.powi(3), 3);
    assert!((a2 - b2).abs() < 0.1);
}

#[test]
fn tc_7_4_2_2_highest_selection_strategy() {
    assert_eq!(select_highest(&[0.3, 0.9, 0.6, 0.1]), Some(1));
    assert_eq!(select_highest(&[0.5, 0.5, 0.8]), Some(2));
}

#[test]
fn tc_7_4_2_3_weighted_random_distribution() {
    let mut rng = SmallRng::seed_from_u64(1);
    let mut c0 = 0u32;
    for _ in 0..10_000 {
        if weighted_random_index(&[0.75, 0.25], &mut rng) == 0 {
            c0 += 1;
        }
    }
    let p = f64::from(c0) / 10_000.0;
    assert!(p > 0.70 && p < 0.80);
    let mut rng2 = SmallRng::seed_from_u64(2);
    let mut c0b = 0u32;
    for _ in 0..10_000 {
        if weighted_random_index(&[0.5, 0.5], &mut rng2) == 0 {
            c0b += 1;
        }
    }
    let p2 = f64::from(c0b) / 10_000.0;
    assert!(p2 > 0.45 && p2 < 0.55);
}

// TC-7.4.3.1
#[test]
fn tc_7_4_3_1_reusable_considerations() {
    let t = [0.2, 0.4, 0.6, 0.1, 0.9, 0.3];
    for ax in [
        InputAxis::HealthRatio,
        InputAxis::DistanceToTarget,
        InputAxis::ThreatLevel,
        InputAxis::TimeSinceAction,
        InputAxis::AmmoCount,
        InputAxis::LineOfSight,
    ] {
        let s = evaluate_builtins(ax, &t);
        assert!((0.0..=1.0).contains(&s));
    }
    struct C;
    impl CustomConsideration for C {
        fn score(&self) -> f32 {
            0.7
        }
    }
    let c = C;
    assert!((score_custom(&c) - 0.7).abs() < 1e-5);
}

// TC-7.4.4.1 dual axis
#[test]
fn tc_7_4_4_1_dual_axis_category_priority() {
    let cats = [(10u32, 100u32), (20u32, 1u32)];
    let pick = dual_axis_pick(&[(1u32, 10u32, 0.3), (2u32, 20u32, 0.9)], &cats);
    assert_eq!(pick, Some(1));
    let pick2 = dual_axis_pick(&[(3u32, 10u32, 0.4), (4u32, 10u32, 0.8)], &cats);
    assert_eq!(pick2, Some(4));
}

// TC-7.4.5.1 hysteresis
#[test]
fn tc_7_4_5_1_context_hysteresis() {
    assert!(!hysteresis_should_switch(0.6, 0.62, 0.1));
    assert!(!hysteresis_should_switch(0.6, 0.58, 0.1));
    assert!(hysteresis_should_switch(0.6, 0.75, 0.1));
}

// TC-7.4.2 score_action_with_inputs (sanity)
#[test]
fn tc_score_action_with_inputs() {
    let cons = [
        Consideration {
            input_axis: InputAxis::HealthRatio,
            curve: ResponseCurve::Linear {
                slope: 1.0,
                intercept: 0.0,
            },
        },
        Consideration {
            input_axis: InputAxis::AmmoCount,
            curve: ResponseCurve::Linear {
                slope: 1.0,
                intercept: 0.0,
            },
        },
    ];
    let action = harmonius_ai::UtilityAction {
        id: 1,
        considerations: vec![0, 1],
        context_id: 0,
        category_id: 0,
    };
    let s = score_action_with_inputs(&action, &cons, |ax| match ax {
        InputAxis::HealthRatio => 0.8,
        InputAxis::AmmoCount => 0.5,
        _ => 0.0,
    });
    assert!(s > 0.0 && s <= 1.0);
}

// GOAP TC-7.5.1.*
#[test]
fn tc_7_5_1_1_world_state_satisfies() {
    const W: u128 = 1;
    let s = WorldState::new(W | 2, [0; 8]);
    assert!(s.satisfies(W, W, &[None; 8]));
    let s2 = WorldState::new(0, [0; 8]);
    assert!(!s2.satisfies(W, W, &[None; 8]));
    let mut mins = [None; 8];
    mins[0] = Some(50);
    let s3 = WorldState::new(0, [80, 0, 0, 0, 0, 0, 0, 0]);
    assert!(s3.satisfies(0, 0, &mins));
}

#[test]
fn tc_7_5_1_2_world_state_apply() {
    let s = WorldState::new(0, [10, 0, 0, 0, 0, 0, 0, 0]);
    let d = WorldStateDelta {
        pre_mask: 0,
        pre_values: 0,
        set_bits: 1,
        clear_bits: 0,
        int_add: [5, 0, 0, 0, 0, 0, 0, 0],
        int_touched: 1,
    };
    let s2 = d.apply(s);
    assert_eq!(s2.bits & 1, 1);
    assert_eq!(s2.ints[0], 15);
}

#[test]
fn tc_7_5_1_3_world_state_heuristic() {
    let s = WorldState::new(0b010, [0; 8]);
    let h = s.goal_distance_heuristic(0b111, 0b111, &[None; 8]);
    assert_eq!(h, 2);
    let s2 = WorldState::new(0b111, [0; 8]);
    assert_eq!(s2.goal_distance_heuristic(0b111, 0b111, &[None; 8]), 0);
}

// TC-7.5.2.*
#[test]
fn tc_7_5_2_1_planner_finds_optimal_path() {
    const BIT_W: u128 = 1;
    let reg = GoapActionRegistry::new(vec![
        GoapAction {
            id: 0,
            name: "FindWeapon",
            delta: WorldStateDelta {
                pre_mask: 0,
                pre_values: 0,
                set_bits: BIT_W,
                clear_bits: 0,
                int_add: [0; 8],
                int_touched: 0,
            },
            cost: 2.0,
        },
        GoapAction {
            id: 1,
            name: "StealWeapon",
            delta: WorldStateDelta {
                pre_mask: 0,
                pre_values: 0,
                set_bits: BIT_W,
                clear_bits: 0,
                int_add: [0; 8],
                int_touched: 0,
            },
            cost: 5.0,
        },
    ]);
    let start = WorldState::new(0, [0; 8]);
    let p = plan_dijkstra(start, BIT_W, BIT_W, &[None; 8], &reg).expect("plan");
    assert_eq!(p.steps, vec![0]);
    assert!((p.total_cost - 2.0).abs() < 1e-5);

    const DEAD: u128 = 1;
    const WEAPON: u128 = 2;
    let reg2 = GoapActionRegistry::new(vec![
        GoapAction {
            id: 0,
            name: "GetWeapon",
            delta: WorldStateDelta {
                pre_mask: 0,
                pre_values: 0,
                set_bits: WEAPON,
                clear_bits: 0,
                int_add: [0; 8],
                int_touched: 0,
            },
            cost: 1.0,
        },
        GoapAction {
            id: 1,
            name: "Attack",
            delta: WorldStateDelta {
                pre_mask: WEAPON,
                pre_values: WEAPON,
                set_bits: DEAD,
                clear_bits: 0,
                int_add: [0; 8],
                int_touched: 0,
            },
            cost: 2.0,
        },
    ]);
    let start2 = WorldState::new(0, [0; 8]);
    let p2 = plan_dijkstra(start2, DEAD, DEAD, &[None; 8], &reg2).expect("plan2");
    assert_eq!(p2.steps, vec![0, 1]);
    assert!((p2.total_cost - 3.0).abs() < 1e-5);
}

#[test]
fn tc_7_5_2_2_planner_unsolvable_goal() {
    let reg = GoapActionRegistry::new(vec![
        GoapAction {
            id: 0,
            name: "Walk",
            delta: WorldStateDelta::noop(),
            cost: 1.0,
        },
        GoapAction {
            id: 1,
            name: "Run",
            delta: WorldStateDelta::noop(),
            cost: 1.0,
        },
    ]);
    let start = WorldState::new(0, [0; 8]);
    assert!(plan_dijkstra(start, 1, 1, &[None; 8], &reg).is_none());

    const KEY: u128 = 1;
    let reg2 = GoapActionRegistry::new(vec![GoapAction {
        id: 0,
        name: "OpenDoor",
        delta: WorldStateDelta {
            pre_mask: KEY,
            pre_values: KEY,
            set_bits: 2,
            clear_bits: 0,
            int_add: [0; 8],
            int_touched: 0,
        },
        cost: 1.0,
    }]);
    let start2 = WorldState::new(0, [0; 8]);
    assert!(plan_dijkstra(start2, 2, 2, &[None; 8], &reg2).is_none());
}

// TC-7.5.3.*
#[test]
fn tc_7_5_3_1_preconditions_gate() {
    const WEAPON: u128 = 1;
    const DEAD: u128 = 2;
    let reg = GoapActionRegistry::new(vec![GoapAction {
        id: 0,
        name: "Attack",
        delta: WorldStateDelta {
            pre_mask: WEAPON,
            pre_values: WEAPON,
            set_bits: DEAD,
            clear_bits: 0,
            int_add: [0; 8],
            int_touched: 0,
        },
        cost: 1.0,
    }]);
    let start = WorldState::new(0, [0; 8]);
    assert!(plan_dijkstra(start, DEAD, DEAD, &[None; 8], &reg).is_none());

    const POTION: u128 = 1;
    const HEALED: u128 = 2;
    let reg2 = GoapActionRegistry::new(vec![GoapAction {
        id: 0,
        name: "Heal",
        delta: WorldStateDelta {
            pre_mask: POTION,
            pre_values: POTION,
            set_bits: HEALED,
            clear_bits: 0,
            int_add: [0; 8],
            int_touched: 0,
        },
        cost: 1.0,
    }]);
    let start2 = WorldState::new(POTION, [0; 8]);
    let p = plan_dijkstra(start2, HEALED, HEALED, &[None; 8], &reg2).expect("heal plan");
    assert_eq!(p.steps, vec![0]);
}

#[test]
fn tc_7_5_3_2_plan_cost_sum() {
    let p = Plan {
        steps: vec![0, 1, 2],
        total_cost: 6.0,
        current_step: 0,
    };
    assert!((p.total_cost - 6.0).abs() < 1e-6);
    let p2 = Plan {
        steps: vec![0],
        total_cost: 5.0,
        current_step: 0,
    };
    assert!((p2.total_cost - 5.0).abs() < 1e-6);
}

// TC-7.5.4.*
#[test]
fn tc_7_5_4_1_cache_hit_identical() {
    let mut cache = PlanCache::new();
    let reg = GoapActionRegistry::new(vec![]);
    cache.sync_version(&reg);
    let p = Plan {
        steps: vec![0],
        total_cost: 1.0,
        current_step: 0,
    };
    let k = PlanCacheKey {
        goal_id: 1,
        state_hash: 42,
    };
    cache.entries.insert(k, p.clone());
    assert_eq!(cache.entries.get(&k), Some(&p));
    let k2 = PlanCacheKey {
        goal_id: 1,
        state_hash: 43,
    };
    assert!(cache.entries.get(&k2).is_none());
}

#[test]
fn tc_7_5_4_2_cache_invalidation() {
    let mut cache = PlanCache::new();
    let mut reg = GoapActionRegistry::new(vec![]);
    cache.sync_version(&reg);
    let k = PlanCacheKey {
        goal_id: 1,
        state_hash: 1,
    };
    cache.entries.insert(
        k,
        Plan {
            steps: vec![],
            total_cost: 0.0,
            current_step: 0,
        },
    );
    reg.bump_version();
    cache.sync_version(&reg);
    assert!(cache.entries.is_empty());
}

// TC-7.5.5.*
#[test]
fn tc_7_5_5_1_replan_on_precondition_failure() {
    const AMMO: u128 = 1;
    let reg = GoapActionRegistry::new(vec![GoapAction {
        id: 0,
        name: "Shoot",
        delta: WorldStateDelta {
            pre_mask: AMMO,
            pre_values: AMMO,
            set_bits: 0,
            clear_bits: AMMO,
            int_add: [0; 8],
            int_touched: 0,
        },
        cost: 1.0,
    }]);
    let mut plan = Plan {
        steps: vec![0],
        total_cost: 1.0,
        current_step: 0,
    };
    let ok = WorldState::new(AMMO, [0; 8]);
    assert!(plan.current_step_pre_hold(&reg, ok));
    let bad = WorldState::new(0, [0; 8]);
    assert!(!plan.current_step_pre_hold(&reg, bad));
}

#[test]
fn tc_7_5_5_2_replan_cooldown() {
    let mut agent = GoapAgent::new(WorldState::new(0, [0; 8]));
    agent.arm_replan_cooldown(1.0);
    agent.tick_cooldown(0.5);
    assert!(!agent.replan_allowed());
    agent.tick_cooldown(0.6);
    assert!(agent.replan_allowed());
}

// TC-7.5.6.*
#[test]
fn tc_7_5_6_1_goal_priority_ordering() {
    let mut agent = GoapAgent::new(WorldState::new(0, [0; 8]));
    agent.goals.push(GoapGoal {
        id: 1,
        bit_mask: 1,
        bit_values: 1,
        int_mins: [None; 8],
        priority: 3.0,
        is_satisfied: false,
    });
    agent.goals.push(GoapGoal {
        id: 2,
        bit_mask: 2,
        bit_values: 2,
        int_mins: [None; 8],
        priority: 10.0,
        is_satisfied: false,
    });
    assert_eq!(agent.pick_goal().expect("g").id, 2);
    agent.current_state = WorldState::new(2, [0; 8]);
    assert_eq!(agent.pick_goal().expect("g2").id, 1);
}

#[test]
fn tc_7_5_6_2_goal_satisfaction_stops_replan() {
    let mut g = GoapGoal {
        id: 1,
        bit_mask: 1,
        bit_values: 1,
        int_mins: [None; 8],
        priority: 1.0,
        is_satisfied: false,
    };
    g.refresh(WorldState::new(1, [0; 8]));
    assert!(g.is_satisfied);
    g.refresh(WorldState::new(0, [0; 8]));
    assert!(!g.is_satisfied);
}

#[test]
fn tc_hash_world_stable() {
    let s = WorldState::new(0x55, [1, 2, 0, 0, 0, 0, 0, 0]);
    let h = hash_world(s);
    assert_ne!(h, 0);
}
