//! Criterion benches for directed-graph ↔ scripting integration (IR-2.7.x).

use criterion::{black_box, criterion_group, criterion_main, Criterion};
use harmonius_scripting::{
    step_graph, traversal_to_step, ConditionExpr, CondEdge, DirectedGraph, EdgePayload,
    GraphCompiler, GraphProgram, GraphTraversalState, NodeId, NodeOpId, NodePayload, ScriptTypeId,
};

fn node() -> NodePayload {
    NodePayload {
        op: NodeOpId(0),
        output_types: vec![ScriptTypeId::UNKNOWN],
        input_types: vec![ScriptTypeId::UNKNOWN],
    }
}

fn bench_topo_sort_1000(c: &mut Criterion) {
    let mut g = DirectedGraph::new();
    let mut ids = Vec::new();
    for _ in 0..1000 {
        ids.push(g.add_node(node()));
    }
    for w in ids.windows(2) {
        g.add_edge(
            w[0],
            w[1],
            EdgePayload {
                source_pin: 0,
                target_pin: 0,
                data_type: ScriptTypeId::UNKNOWN,
            },
        );
    }
    c.bench_function("tc_ir_2_7_2_b1_topo_sort_1000", |b| {
        b.iter(|| black_box(g.topological_sort().unwrap()));
    });
}

fn bench_validate_1000(c: &mut Criterion) {
    let mut g = DirectedGraph::new();
    let mut ids = Vec::new();
    for _ in 0..1000 {
        ids.push(g.add_node(node()));
    }
    for w in ids.windows(2) {
        g.add_edge(
            w[0],
            w[1],
            EdgePayload {
                source_pin: 0,
                target_pin: 0,
                data_type: ScriptTypeId::UNKNOWN,
            },
        );
    }
    c.bench_function("tc_ir_2_7_5_b1_validate_1000", |b| {
        b.iter(|| black_box(g.validate().unwrap()));
    });
}

fn bench_traversal_1000(c: &mut Criterion) {
    let tr = GraphTraversalState {
        current_node: Some(NodeId(7)),
        ..GraphTraversalState::default()
    };
    c.bench_function("tc_ir_2_7_4_b1_traversal_to_step_1000", |b| {
        b.iter(|| {
            for _ in 0..1000 {
                black_box(traversal_to_step(&tr));
            }
        });
    });
}

fn bench_compile_conditional_500(c: &mut Criterion) {
    let mut g = DirectedGraph::new();
    let mut prev = g.add_node(node());
    for _i in 1..501 {
        let n = g.add_node(node());
        g.add_edge(
            prev,
            n,
            CondEdge {
                condition: ConditionExpr::Runtime(0),
                data: EdgePayload {
                    source_pin: 0,
                    target_pin: 0,
                    data_type: ScriptTypeId::UNKNOWN,
                },
            },
        );
        prev = n;
    }
    c.bench_function("tc_ir_2_7_3_b1a_lower_500_conditional_edges", |b| {
        b.iter(|| black_box(GraphCompiler::compile_conditional(&g, None).unwrap()));
    });
}

fn bench_step_graph_chain(c: &mut Criterion) {
    let p = GraphProgram {
        version: 1,
        entry: NodeId(0),
    };
    let tr = GraphTraversalState::default();
    c.bench_function("tc_ir_2_7_4_b1_step_graph_updates", |b| {
        b.iter(|| {
            let mut t = tr.clone();
            for _ in 0..1000 {
                let (nt, _) = step_graph(&p, t, 0);
                t = nt;
            }
            black_box(t);
        });
    });
}

criterion_group!(
    benches,
    bench_topo_sort_1000,
    bench_validate_1000,
    bench_traversal_1000,
    bench_compile_conditional_500,
    bench_step_graph_chain,
);
criterion_main!(benches);
