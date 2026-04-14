//! Integration tests for directed graphs ↔ scripting (IR-2.7.x).

use harmonius_scripting::{
    traversal_to_step, ConditionExpr, CondEdge, DirectedGraph, EdgePayload, GraphCompiler,
    GraphError, GraphExecutionSystem, GraphInstance, GraphProgram, GraphTraversalState, LogicGraph,
    NodeId, NodeOpId, NodePayload, OrderedGraph, ScriptTypeId, StepOutcome, TopologyError,
    GRAPH_STEP_NOT_STARTED,
};
use rkyv::{from_bytes, to_bytes};
use syn::visit::{self, Visit};
use syn::File;

fn node_payload(op: u32, outs: Vec<ScriptTypeId>, ins: Vec<ScriptTypeId>) -> NodePayload {
    NodePayload {
        op: NodeOpId(op),
        output_types: outs,
        input_types: ins,
    }
}

fn count_expr_if(file: &File) -> usize {
    struct Counter(usize);
    impl<'ast> Visit<'ast> for Counter {
        fn visit_expr_if(&mut self, i: &'ast syn::ExprIf) {
            self.0 += 1;
            visit::visit_expr_if(self, i);
        }
    }
    let mut c = Counter(0);
    c.visit_file(file);
    c.0
}

fn parse_generated(src: &str) -> File {
    syn::parse_file(src).expect("generated Rust must parse")
}

#[test]
fn tc_ir_2_7_1_1_logic_graph_five_nodes_rkyv_roundtrip() {
    let mut g = LogicGraph::new();
    for k in 0u32..5 {
        g.add_node(node_payload(k, vec![], vec![]));
    }
    assert_eq!(g.node_count(), 5);
    let sample = g.get_node(NodeId(2)).unwrap();
    let bytes = to_bytes::<rkyv::rancor::Error>(sample).expect("serialize");
    let back: NodePayload =
        from_bytes::<NodePayload, rkyv::rancor::Error>(&bytes).expect("deserialize");
    assert_eq!(back.op, sample.op);
}

#[test]
fn tc_ir_2_7_1_2_edge_payload_carries_f32_type() {
    let mut g = LogicGraph::new();
    let a = g.add_node(node_payload(0, vec![ScriptTypeId::F32], vec![]));
    let b = g.add_node(node_payload(1, vec![], vec![ScriptTypeId::F32]));
    let edge = EdgePayload {
        source_pin: 0,
        target_pin: 0,
        data_type: ScriptTypeId::F32,
    };
    g.add_edge(a, b, edge);
    GraphCompiler::compile(&g, None).expect("compile");
}

#[test]
fn tc_ir_2_7_1_3_edge_type_mismatch_rejected() {
    let mut g = LogicGraph::new();
    let a = g.add_node(node_payload(0, vec![ScriptTypeId::F32], vec![]));
    let b = g.add_node(node_payload(1, vec![], vec![ScriptTypeId::I32]));
    let edge = EdgePayload {
        source_pin: 0,
        target_pin: 0,
        data_type: ScriptTypeId::F32,
    };
    g.add_edge(a, b, edge);
    let err = GraphCompiler::compile(&g, None).unwrap_err();
    assert!(matches!(
        err,
        GraphError::EdgeTypeMismatch {
            source: ScriptTypeId::F32,
            target: ScriptTypeId::I32,
            ..
        }
    ));
}

#[test]
fn tc_ir_2_7_2_1_topo_sort_chain_emits_abc_order() {
    let mut g = LogicGraph::new();
    let a = g.add_node(node_payload(0, vec![], vec![]));
    let b = g.add_node(node_payload(1, vec![], vec![]));
    let c = g.add_node(node_payload(2, vec![], vec![]));
    g.add_edge(a, b, EdgePayload {
        source_pin: 0,
        target_pin: 0,
        data_type: ScriptTypeId::UNKNOWN,
    });
    g.add_edge(b, c, EdgePayload {
        source_pin: 0,
        target_pin: 0,
        data_type: ScriptTypeId::UNKNOWN,
    });
    let order = g.topological_sort().expect("topo");
    assert_eq!(order, vec![a, b, c]);
    let src = GraphCompiler::compile(&g, None).expect("compile");
    let file = parse_generated(&src);
    assert_eq!(count_expr_if(&file), 0);
}

#[test]
fn tc_ir_2_7_2_2_diamond_topo_d_last() {
    let mut g = LogicGraph::new();
    let a = g.add_node(node_payload(0, vec![], vec![]));
    let b = g.add_node(node_payload(1, vec![], vec![]));
    let c = g.add_node(node_payload(2, vec![], vec![]));
    let d = g.add_node(node_payload(3, vec![], vec![]));
    g.add_edge(a, b, EdgePayload {
        source_pin: 0,
        target_pin: 0,
        data_type: ScriptTypeId::UNKNOWN,
    });
    g.add_edge(a, c, EdgePayload {
        source_pin: 0,
        target_pin: 0,
        data_type: ScriptTypeId::UNKNOWN,
    });
    g.add_edge(b, d, EdgePayload {
        source_pin: 0,
        target_pin: 0,
        data_type: ScriptTypeId::UNKNOWN,
    });
    g.add_edge(c, d, EdgePayload {
        source_pin: 0,
        target_pin: 0,
        data_type: ScriptTypeId::UNKNOWN,
    });
    let order = g.topological_sort().expect("topo");
    assert_eq!(order.last(), Some(&d));
    let pos_b = order.iter().position(|&x| x == b).unwrap();
    let pos_c = order.iter().position(|&x| x == c).unwrap();
    let pos_d = order.iter().position(|&x| x == d).unwrap();
    assert!(pos_b < pos_d && pos_c < pos_d);
}

#[test]
fn tc_ir_2_7_3_1_conditional_edge_emits_if() {
    let mut g = DirectedGraph::new();
    let u = g.add_node(node_payload(0, vec![], vec![]));
    let v = g.add_node(node_payload(1, vec![], vec![]));
    g.add_edge(
        u,
        v,
        CondEdge {
            condition: ConditionExpr::Runtime(0),
            data: EdgePayload {
                source_pin: 0,
                target_pin: 0,
                data_type: ScriptTypeId::UNKNOWN,
            },
        },
    );
    let src = GraphCompiler::compile_conditional(&g, None).expect("compile");
    let file = parse_generated(&src);
    assert_eq!(count_expr_if(&file), 1);
}

#[test]
fn tc_ir_2_7_3_2_static_true_elides_if() {
    let mut g = DirectedGraph::new();
    let u = g.add_node(node_payload(0, vec![], vec![]));
    let v = g.add_node(node_payload(1, vec![], vec![]));
    g.add_edge(
        u,
        v,
        CondEdge {
            condition: ConditionExpr::Const(true),
            data: EdgePayload {
                source_pin: 0,
                target_pin: 0,
                data_type: ScriptTypeId::UNKNOWN,
            },
        },
    );
    let src = GraphCompiler::compile_conditional(&g, None).expect("compile");
    let file = parse_generated(&src);
    assert_eq!(count_expr_if(&file), 0);
}

#[test]
fn tc_ir_2_7_4_1_traversal_maps_to_step() {
    let tr = GraphTraversalState {
        current_node: Some(NodeId(3)),
        ..GraphTraversalState::default()
    };
    assert_eq!(traversal_to_step(&tr), 3);
}

#[test]
fn tc_ir_2_7_4_2_not_started_sentinel() {
    let tr = GraphTraversalState {
        current_node: None,
        ..GraphTraversalState::default()
    };
    assert_eq!(traversal_to_step(&tr), GRAPH_STEP_NOT_STARTED);
}

#[test]
fn tc_ir_2_7_4_3_hot_reload_stale_then_reset() {
    let p1 = std::sync::Arc::new(GraphProgram {
        version: 1,
        entry: NodeId(0),
    });
    let mut inst = GraphInstance::new(p1.clone());
    assert!(matches!(inst.advance(0), StepOutcome::Continue));
    assert!(matches!(inst.advance(1), StepOutcome::Complete));
    let p2 = std::sync::Arc::new(GraphProgram {
        version: 2,
        entry: NodeId(0),
    });
    inst.swap_program(p2);
    let out = inst.advance(2);
    assert_eq!(
        out,
        StepOutcome::Error(GraphError::StaleProgram {
            expected: 1,
            found: 2,
        })
    );
    GraphExecutionSystem::reset_after_stale(&mut inst, 10);
    assert!(matches!(inst.advance(11), StepOutcome::Continue));
}

#[test]
fn tc_ir_2_7_5_1_cycle_detected_path() {
    let mut g = LogicGraph::new();
    let a = g.add_node(node_payload(0, vec![], vec![]));
    let b = g.add_node(node_payload(1, vec![], vec![]));
    let c = g.add_node(node_payload(2, vec![], vec![]));
    g.add_edge(a, b, EdgePayload {
        source_pin: 0,
        target_pin: 0,
        data_type: ScriptTypeId::UNKNOWN,
    });
    g.add_edge(b, c, EdgePayload {
        source_pin: 0,
        target_pin: 0,
        data_type: ScriptTypeId::UNKNOWN,
    });
    g.add_edge(c, a, EdgePayload {
        source_pin: 0,
        target_pin: 0,
        data_type: ScriptTypeId::UNKNOWN,
    });
    let err = g.validate().unwrap_err();
    match err {
        TopologyError::CycleDetected(ce) => {
            assert_eq!(ce.path.first(), ce.path.last());
            assert!(ce.path.len() >= 3);
        }
        _ => panic!("expected cycle"),
    }
}

#[test]
fn tc_ir_2_7_5_2_self_loop() {
    let mut g = LogicGraph::new();
    let a = g.add_node(node_payload(0, vec![], vec![]));
    g.add_edge(a, a, EdgePayload {
        source_pin: 0,
        target_pin: 0,
        data_type: ScriptTypeId::UNKNOWN,
    });
    let err = g.validate().unwrap_err();
    assert!(matches!(err, TopologyError::SelfLoop(n) if n == a));
}

#[test]
fn tc_ir_2_7_5_3_missing_node_rejected() {
    let mut g = LogicGraph::new();
    for _ in 0..5 {
        g.add_node(node_payload(0, vec![], vec![]));
    }
    let last = NodeId(4);
    g.add_edge(last, NodeId(99), EdgePayload {
        source_pin: 0,
        target_pin: 0,
        data_type: ScriptTypeId::UNKNOWN,
    });
    let err = GraphCompiler::compile(&g, None).unwrap_err();
    assert_eq!(err, GraphError::NodeNotFound(NodeId(99)));
}

#[test]
fn tc_ir_2_7_6_1_ordered_children_preserved() {
    let mut o = OrderedGraph::new();
    let s = o.add_node(node_payload(0, vec![], vec![]));
    let c = o.add_node(node_payload(1, vec![], vec![]));
    let a = o.add_node(node_payload(2, vec![], vec![]));
    let b = o.add_node(node_payload(3, vec![], vec![]));
    o.add_edge(s, c, EdgePayload {
        source_pin: 0,
        target_pin: 0,
        data_type: ScriptTypeId::UNKNOWN,
    });
    o.add_edge(s, a, EdgePayload {
        source_pin: 0,
        target_pin: 0,
        data_type: ScriptTypeId::UNKNOWN,
    });
    o.add_edge(s, b, EdgePayload {
        source_pin: 0,
        target_pin: 0,
        data_type: ScriptTypeId::UNKNOWN,
    });
    o.set_order(s, vec![c, a, b]);
    let src = GraphCompiler::compile(o.inner(), Some(&o)).expect("compile");
    let pos = |n: u32| src.find(&format!("__node_{n}();")).unwrap();
    assert!(pos(1) < pos(2));
    assert!(pos(2) < pos(3));
}

#[test]
fn tc_ir_2_7_6_2_reorder_updates_codegen() {
    let mut o = OrderedGraph::new();
    let s = o.add_node(node_payload(0, vec![], vec![]));
    let c = o.add_node(node_payload(1, vec![], vec![]));
    let a = o.add_node(node_payload(2, vec![], vec![]));
    let b = o.add_node(node_payload(3, vec![], vec![]));
    for &(from, to) in &[(s, c), (s, a), (s, b)] {
        o.add_edge(
            from,
            to,
            EdgePayload {
                source_pin: 0,
                target_pin: 0,
                data_type: ScriptTypeId::UNKNOWN,
            },
        );
    }
    o.set_order(s, vec![c, a, b]);
    let src1 = GraphCompiler::compile(o.inner(), Some(&o)).expect("compile");
    o.set_order(s, vec![a, b, c]);
    let src2 = GraphCompiler::compile(o.inner(), Some(&o)).expect("compile");
    assert_ne!(src1, src2);
}
