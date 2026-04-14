//! Plan `PLAN-data-systems-directed-graphs` acceptance tests (TC-* rows).

use std::collections::{HashMap, HashSet};

use harmonius_graph::{
    ConditionContext, ConditionExpr, ConditionRegistry, CycleError, DirectedGraph, GraphError,
    MultiGraph, NodeId, OrderedGraph, WeightedGraph,
};

#[test]
fn test_cycle_detect_self_loop() {
    let mut g = DirectedGraph::<(), ()>::new();
    let a = g.add_node(());
    let err = g.add_edge(a, a, ()).unwrap_err();
    assert_eq!(
        err,
        GraphError::CycleDetected(CycleError {
            cycle_path: vec![a],
        })
    );
}

#[test]
fn test_cycle_detect_two_node() {
    let mut g = DirectedGraph::<(), ()>::new();
    let a = g.add_node(());
    let b = g.add_node(());
    g.add_edge(a, b, ()).unwrap();
    let err = g.add_edge(b, a, ()).unwrap_err();
    assert_eq!(
        err,
        GraphError::CycleDetected(CycleError {
            cycle_path: vec![a, b],
        })
    );
}

#[test]
fn test_cycle_detect_three_node() {
    let mut g = DirectedGraph::<(), ()>::new();
    let a = g.add_node(());
    let b = g.add_node(());
    let c = g.add_node(());
    g.add_edge(a, b, ()).unwrap();
    g.add_edge(b, c, ()).unwrap();
    let err = g.add_edge(c, a, ()).unwrap_err();
    assert_eq!(
        err,
        GraphError::CycleDetected(CycleError {
            cycle_path: vec![a, b, c],
        })
    );
}

#[test]
fn test_dag_acyclic_passes() {
    let mut g = DirectedGraph::<char, ()>::new();
    let ids: Vec<NodeId> = (b'A'..=b'E').map(|ch| g.add_node(ch as char)).collect();
    let e = |i: usize| ids[i];
    let edges = [(0, 1), (0, 2), (1, 3), (2, 3), (1, 4), (3, 4), (2, 4)];
    for (a, b) in edges {
        assert!(g.add_edge(e(a), e(b), ()).is_ok());
    }
    assert!(g.is_acyclic());
}

#[test]
fn test_topo_sort_linear_chain() {
    let mut g = DirectedGraph::<(), ()>::new();
    let a = g.add_node(());
    let b = g.add_node(());
    let c = g.add_node(());
    let d = g.add_node(());
    g.add_edge(a, b, ()).unwrap();
    g.add_edge(b, c, ()).unwrap();
    g.add_edge(c, d, ()).unwrap();
    assert_eq!(g.topological_sort().unwrap(), vec![a, b, c, d]);
}

#[test]
fn test_topo_sort_diamond() {
    let mut g = DirectedGraph::<(), ()>::new();
    let a = g.add_node(());
    let b = g.add_node(());
    let c = g.add_node(());
    let d = g.add_node(());
    g.add_edge(a, b, ()).unwrap();
    g.add_edge(a, c, ()).unwrap();
    g.add_edge(b, d, ()).unwrap();
    g.add_edge(c, d, ()).unwrap();
    let t = g.topological_sort().unwrap();
    assert_eq!(t[0], a);
    assert_eq!(*t.last().unwrap(), d);
    assert!(t.contains(&b) && t.contains(&c));
    let bi = t.iter().position(|&x| x == b).unwrap();
    let ci = t.iter().position(|&x| x == c).unwrap();
    assert!(bi > 0 && ci > 0);
    assert!(bi < 3 && ci < 3);
}

#[test]
fn test_topo_sort_deterministic() {
    fn build(order: &[(usize, usize)]) -> DirectedGraph<(), ()> {
        let mut g = DirectedGraph::new();
        let a = g.add_node(());
        let b = g.add_node(());
        let c = g.add_node(());
        let d = g.add_node(());
        let ids = [a, b, c, d];
        for &(x, y) in order {
            g.add_edge(ids[x], ids[y], ()).unwrap();
        }
        g
    }
    let orders = [
        vec![(0, 1), (0, 2), (1, 3), (2, 3)],
        vec![(0, 2), (0, 1), (2, 3), (1, 3)],
        vec![(1, 3), (0, 1), (0, 2), (2, 3)],
    ];
    let r0 = build(&orders[0]).topological_sort().unwrap();
    assert_eq!(r0, build(&orders[1]).topological_sort().unwrap());
    assert_eq!(r0, build(&orders[2]).topological_sort().unwrap());
}

#[test]
fn test_conditional_edge_skipped() {
    let mut g = harmonius_graph::ConditionalGraph::<(), ()>::new();
    let a = g.add_node(());
    let b = g.add_node(());
    g.add_conditional_edge(a, b, ConditionExpr::HasFlag("key".into()), ())
        .unwrap();
    let ctx = ConditionContext {
        flags: HashMap::new(),
    };
    let reg = ConditionRegistry::new();
    assert!(g.successors(a, &ctx, &reg).is_empty());
    assert_eq!(g.traverse_from(a, &ctx, &reg), vec![a]);
}

#[test]
fn test_conditional_edge_taken() {
    let mut g = harmonius_graph::ConditionalGraph::<(), ()>::new();
    let a = g.add_node(());
    let b = g.add_node(());
    g.add_conditional_edge(a, b, ConditionExpr::HasFlag("key".into()), ())
        .unwrap();
    let ctx = ConditionContext {
        flags: HashMap::from([("key".into(), true)]),
    };
    let reg = ConditionRegistry::new();
    assert_eq!(g.successors(a, &ctx, &reg), vec![b]);
    assert_eq!(g.traverse_from(a, &ctx, &reg), vec![a, b]);
}

#[test]
fn test_ordered_insert_sequence() {
    let mut g = OrderedGraph::<(), ()>::new();
    let p = g.add_node(());
    let a = g.add_node(());
    let b = g.add_node(());
    let c = g.add_node(());
    g.add_child(p, a, ()).unwrap();
    g.add_child(p, b, ()).unwrap();
    g.add_child(p, c, ()).unwrap();
    assert_eq!(g.children(p), [a, b, c]);
}

#[test]
fn test_ordered_reorder_children() {
    let mut g = OrderedGraph::<(), ()>::new();
    let p = g.add_node(());
    let a = g.add_node(());
    let b = g.add_node(());
    let c = g.add_node(());
    g.add_child(p, a, ()).unwrap();
    g.add_child(p, b, ()).unwrap();
    g.add_child(p, c, ()).unwrap();
    g.reorder_children_list(p, &[c, a, b]);
    assert_eq!(g.children(p), [c, a, b]);
}

#[test]
fn test_dijkstra_shortest_path() {
    let mut g = WeightedGraph::<(), ()>::new();
    let a = g.add_node(());
    let b = g.add_node(());
    let c = g.add_node(());
    let d = g.add_node(());
    g.add_weighted_edge(a, b, (), 1.0).unwrap();
    g.add_weighted_edge(b, d, (), 5.0).unwrap();
    g.add_weighted_edge(a, c, (), 2.0).unwrap();
    g.add_weighted_edge(c, d, (), 1.0).unwrap();
    let (path, cost) = g.shortest_path(a, d).unwrap();
    assert_eq!(path, vec![a, c, d]);
    assert!((cost - 3.0).abs() < 1e-9);
}

#[test]
fn test_dijkstra_unreachable() {
    let mut g = WeightedGraph::<(), ()>::new();
    let a = g.add_node(());
    let b = g.add_node(());
    let c = g.add_node(());
    let d = g.add_node(());
    g.add_weighted_edge(a, b, (), 1.0).unwrap();
    g.add_weighted_edge(c, d, (), 1.0).unwrap();
    assert!(g.shortest_path(a, d).is_none());
}

#[test]
fn test_budget_reachability() {
    let mut g = WeightedGraph::<(), ()>::new();
    let a = g.add_node(());
    let b = g.add_node(());
    let c = g.add_node(());
    let d = g.add_node(());
    let e = g.add_node(());
    g.add_weighted_edge(a, b, (), 2.0).unwrap();
    g.add_weighted_edge(b, c, (), 2.0).unwrap();
    g.add_weighted_edge(c, d, (), 2.0).unwrap();
    g.add_weighted_edge(a, e, (), 6.0).unwrap();
    let hs: HashSet<_> = g.min_cost_reachable(a, 5.0).into_iter().collect();
    assert_eq!(hs, HashSet::from([a, b, c]));
}

#[test]
fn test_bfs_order() {
    let mut g = DirectedGraph::<(), ()>::new();
    let a = g.add_node(());
    let b = g.add_node(());
    let c = g.add_node(());
    let d = g.add_node(());
    let e = g.add_node(());
    g.add_edge(a, b, ()).unwrap();
    g.add_edge(a, c, ()).unwrap();
    g.add_edge(b, d, ()).unwrap();
    g.add_edge(c, e, ()).unwrap();
    assert_eq!(g.bfs(a, |_| true), vec![a, b, c, d, e]);
}

#[test]
fn test_dfs_preorder() {
    let mut g = DirectedGraph::<(), ()>::new();
    let a = g.add_node(());
    let b = g.add_node(());
    let c = g.add_node(());
    let d = g.add_node(());
    let e = g.add_node(());
    g.add_edge(a, b, ()).unwrap();
    g.add_edge(a, c, ()).unwrap();
    g.add_edge(b, d, ()).unwrap();
    g.add_edge(c, e, ()).unwrap();
    assert_eq!(g.dfs_pre(a, |_| true), vec![a, b, d, c, e]);
}

#[test]
fn test_dfs_postorder() {
    let mut g = DirectedGraph::<(), ()>::new();
    let a = g.add_node(());
    let b = g.add_node(());
    let c = g.add_node(());
    let d = g.add_node(());
    let e = g.add_node(());
    g.add_edge(a, b, ()).unwrap();
    g.add_edge(a, c, ()).unwrap();
    g.add_edge(b, d, ()).unwrap();
    g.add_edge(c, e, ()).unwrap();
    assert_eq!(g.dfs_post(a, |_| true), vec![d, b, e, c, a]);
}

#[test]
fn test_traversal_with_filter() {
    let mut g = DirectedGraph::<(), ()>::new();
    let a = g.add_node(());
    let b = g.add_node(());
    let c = g.add_node(());
    let d = g.add_node(());
    let e = g.add_node(());
    g.add_edge(a, b, ()).unwrap();
    g.add_edge(a, c, ()).unwrap();
    g.add_edge(b, d, ()).unwrap();
    g.add_edge(c, e, ()).unwrap();
    let c_id = c;
    assert_eq!(g.bfs_filtered(a, |_, to, _| to != c_id), vec![a, b, d]);
}

#[test]
fn test_tree_root_and_leaves() {
    let mut g = OrderedGraph::<char, ()>::new();
    let r = g.add_node('R');
    let n1 = g.add_node('N');
    let n2 = g.add_node('M');
    let l1 = g.add_node('1');
    let l2 = g.add_node('2');
    let l3 = g.add_node('3');
    g.add_child(r, n1, ()).unwrap();
    g.add_child(r, n2, ()).unwrap();
    g.add_child(n1, l1, ()).unwrap();
    g.add_child(n1, l2, ()).unwrap();
    g.add_child(n2, l3, ()).unwrap();
    assert_eq!(g.root(), Some(r));
    let leaves: HashSet<_> = g.leaves().into_iter().collect();
    assert_eq!(leaves, HashSet::from([l1, l2, l3]));
}

#[test]
fn test_tree_depth_and_subtree() {
    let mut g = OrderedGraph::<(), ()>::new();
    let r = g.add_node(());
    let n1 = g.add_node(());
    let n2 = g.add_node(());
    let l = g.add_node(());
    g.add_child(r, n1, ()).unwrap();
    g.add_child(n1, n2, ()).unwrap();
    g.add_child(n2, l, ()).unwrap();
    assert_eq!(g.depth(l), Some(3));
    let sub = g.subtree(n1);
    assert_eq!(sub.node_count(), 3);
}

#[test]
fn test_tree_lca() {
    let mut g = OrderedGraph::<(), ()>::new();
    let r = g.add_node(());
    let a = g.add_node(());
    let b = g.add_node(());
    let x = g.add_node(());
    let y = g.add_node(());
    let z = g.add_node(());
    g.add_child(r, a, ()).unwrap();
    g.add_child(r, b, ()).unwrap();
    g.add_child(a, x, ()).unwrap();
    g.add_child(a, y, ()).unwrap();
    g.add_child(b, z, ()).unwrap();
    assert_eq!(g.lca(x, y), Some(a));
    assert_eq!(g.lca(x, z), Some(r));
    assert_eq!(g.lca(x, x), Some(x));
}

#[test]
fn test_dead_node_elimination() {
    let mut g = DirectedGraph::<char, ()>::new();
    let r = g.add_node('R');
    let a = g.add_node('A');
    let b = g.add_node('B');
    let d = g.add_node('D');
    g.add_edge(r, a, ()).unwrap();
    g.add_edge(r, b, ()).unwrap();
    // `live_roots` are sink/output nodes; walk backward to keep feeders (R, A, B).
    g.prune_unreachable(&[a, b]);
    assert_eq!(g.node_count(), 3);
    assert!(g.get_node(d).is_none());
}

#[test]
fn test_transitive_reduction() {
    let mut g = DirectedGraph::<(), ()>::new();
    let a = g.add_node(());
    let b = g.add_node(());
    let c = g.add_node(());
    g.add_edge(a, b, ()).unwrap();
    g.add_edge(b, c, ()).unwrap();
    g.add_edge(a, c, ()).unwrap();
    let r = g.transitive_reduction();
    assert_eq!(r.edge_count(), 2);
    assert!(r.is_acyclic());
}

#[test]
fn test_multi_graph_parallel_edges() {
    #[derive(Debug, Clone, PartialEq, Eq, Hash)]
    enum EdgeData {
        Yes,
        No,
    }
    let mut g = MultiGraph::<(), EdgeData>::new();
    let a = g.add_node(());
    let b = g.add_node(());
    g.add_edge(a, b, EdgeData::Yes).unwrap();
    g.add_edge(a, b, EdgeData::No).unwrap();
    let outs: Vec<_> = g.out_edges_with_handles(a).collect();
    assert_eq!(outs.len(), 2);
    assert_ne!(outs[0].0, outs[1].0);
    let hs: HashSet<_> = outs.iter().map(|(_, _, p)| (*p).clone()).collect();
    assert_eq!(hs, HashSet::from([EdgeData::Yes, EdgeData::No]));
}

#[test]
fn test_quest_dag_typed_objectives() {
    #[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
    enum QuestObjective {
        KillCount(u32),
        ReachLocation(&'static str),
        CollectItem(&'static str),
    }
    let mut g = DirectedGraph::<QuestObjective, ()>::new();
    let k = g.add_node(QuestObjective::KillCount(3));
    let r = g.add_node(QuestObjective::ReachLocation("cave"));
    let c = g.add_node(QuestObjective::CollectItem("orb"));
    g.add_edge(k, r, ()).unwrap();
    g.add_edge(r, c, ()).unwrap();
    let order = g.topological_sort().unwrap();
    let objs: Vec<_> = order
        .iter()
        .map(|id| g.get_node(*id).unwrap().clone())
        .collect();
    assert_eq!(
        objs,
        vec![
            QuestObjective::KillCount(3),
            QuestObjective::ReachLocation("cave"),
            QuestObjective::CollectItem("orb"),
        ]
    );
}

#[test]
fn test_dialogue_tree_conditions() {
    let mut g = harmonius_graph::ConditionalGraph::<&'static str, ()>::new();
    let root = g.add_node("root");
    let branch_a = g.add_node("line_a");
    let branch_b = g.add_node("line_b");
    g.add_conditional_edge(
        root,
        branch_a,
        ConditionExpr::HasFlag("met_wizard".into()),
        (),
    )
    .unwrap();
    g.add_conditional_edge(
        root,
        branch_b,
        ConditionExpr::Not(Box::new(ConditionExpr::HasFlag("met_wizard".into()))),
        (),
    )
    .unwrap();
    let ctx_true = ConditionContext {
        flags: HashMap::from([("met_wizard".into(), true)]),
    };
    let ctx_false = ConditionContext {
        flags: HashMap::from([("met_wizard".into(), false)]),
    };
    let reg = ConditionRegistry::new();
    let t = g.traverse_from(root, &ctx_true, &reg);
    let f = g.traverse_from(root, &ctx_false, &reg);
    assert_ne!(t.last(), f.last());
}

#[test]
fn test_ability_composition_graph() {
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    enum Effect {
        Cast,
        ApplyBurn,
        Explode,
    }
    let mut g = DirectedGraph::<Effect, ()>::new();
    let c = g.add_node(Effect::Cast);
    let b = g.add_node(Effect::ApplyBurn);
    let e = g.add_node(Effect::Explode);
    g.add_edge(c, b, ()).unwrap();
    g.add_edge(b, e, ()).unwrap();
    let order = g.dfs_pre(c, |_| true);
    let effects: Vec<_> = order.iter().map(|id| *g.get_node(*id).unwrap()).collect();
    assert_eq!(
        effects,
        vec![Effect::Cast, Effect::ApplyBurn, Effect::Explode]
    );
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum UnlockError {
    PrereqMissing,
}

fn try_unlock_talent(
    graph: &DirectedGraph<(), ()>,
    unlocked: &HashSet<NodeId>,
    target: NodeId,
) -> Result<(), UnlockError> {
    for (pred, _) in graph.in_edges(target) {
        if !unlocked.contains(&pred) {
            return Err(UnlockError::PrereqMissing);
        }
    }
    Ok(())
}

#[test]
fn test_talent_tree_prereq() {
    let mut g = DirectedGraph::<(), ()>::new();
    let t1 = g.add_node(());
    let t2 = g.add_node(());
    g.add_edge(t1, t2, ()).unwrap();
    let unlocked = HashSet::new();
    assert_eq!(
        try_unlock_talent(&g, &unlocked, t2),
        Err(UnlockError::PrereqMissing)
    );
    let unlocked = HashSet::from([t1]);
    assert_eq!(try_unlock_talent(&g, &unlocked, t2), Ok(()));
}
