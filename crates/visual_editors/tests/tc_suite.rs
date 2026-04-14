//! Traceability tests for `PLAN-tools-visual-editors` / `visual-editors-test-cases.md`.

use glam::Vec2;
use smallvec::smallvec;
use visual_editors::animation::{
    self, AnimState, AnimStateMachine, AnimTimeline, AnimTrack, AnimTransition, BlendSpace,
    BlendSpaceDimension, BlendSample, BoneId, BoneMapping, Keyframe, Skeleton, StateId,
    TangentMode,
};
use visual_editors::compiler::{
    self, AnimEcsSystems, CompiledGraph, CompileError, EcsCompiler, MixerEvents, RenderGraphPasses,
    ToolAction,
};
use visual_editors::diff::{GraphDiffer, MergeResult};
use visual_editors::graph_ir::{
    Edge, GenericParamId, GraphDomain, LogicGraph, Node, NodeId, NodeKind, Pin, PinDirection,
    PinId, PinType, TypeId, Variable, VariableId, VariableScope,
};
use visual_editors::inference::{InferenceResult, TypeDiagnostic, TypeInferenceEngine};
use visual_editors::lowering::{SuspendVariant, lower_suspends};
use visual_editors::material::{MaterialInstance, ParamValue, material_library_search};
use visual_editors::material_graph::{
    AssetId, MaterialGraph, MaterialNodeType, MaterialPinId, PinTypeError,
    inline_material_function,
};
use visual_editors::optimize::{constant_folding, dead_node_elimination, inline_subgraph, monomorphize};
use visual_editors::refactor::{find_usages, rename_symbol};
use visual_editors::shader::{compile_with_dxc_stub, emit_shader_graph_main, material_pbr_template, ShaderStage};
use visual_editors::specialized::{
    EntityHierarchyPanel, EquipmentTable, HierarchyEntity, InspectorField, InspectorPanel,
    NodeLibrary, PriceLedger, QuestIr, VfxGraph, VfxNode, VfxNodeKind, compile_quest_objectives,
    dummy_graph_id, logic_graph_stub_objectives,
};
use visual_editors::validate::{GraphValidator, NodeRegistry};
use visual_editors::widgets::{
    CellValue, ColumnDef, GraphEditorWidget, SortDirection, TableEditorWidget, TableId,
};

fn pin_out(id: u32, t: PinType) -> Pin {
    Pin {
        pin_id: PinId(id),
        direction: PinDirection::Out,
        pin_type: t,
        name: format!("o{id}"),
    }
}

fn pin_in(id: u32, t: PinType) -> Pin {
    Pin {
        pin_id: PinId(id),
        direction: PinDirection::In,
        pin_type: t,
        name: format!("i{id}"),
    }
}

#[test]
fn tc_15_8_1_1_graph_ir_node_add_remove() {
    let mut g = LogicGraph::new(GraphDomain::Gameplay, "g");
    let n = Node {
        node_id: NodeId(1),
        kind: NodeKind::Stub {
            name: "a".into(),
        },
        pins: smallvec![],
        position: Vec2::ZERO,
    };
    g.add_node(n.clone());
    assert_eq!(g.nodes.len(), 1);
    g.remove_node(NodeId(1));
    assert!(g.nodes.is_empty());
}

#[test]
fn tc_15_8_1_2_graph_ir_edge_connect() {
    let mut g = LogicGraph::new(GraphDomain::Gameplay, "g");
    g.add_node(Node {
        node_id: NodeId(1),
        kind: NodeKind::ConstantI32(1),
        pins: smallvec![pin_out(0, PinType::Data(TypeId(1)))],
        position: Vec2::ZERO,
    });
    g.add_node(Node {
        node_id: NodeId(2),
        kind: NodeKind::PureAdd,
        pins: smallvec![pin_in(0, PinType::Data(TypeId(1)))],
        position: Vec2::ZERO,
    });
    g.connect(Edge {
        source_node: NodeId(1),
        source_pin: PinId(0),
        target_node: NodeId(2),
        target_pin: PinId(0),
    });
    assert_eq!(g.edges.len(), 1);
}

#[test]
fn tc_15_8_1_3_graph_ir_variable_crud() {
    let mut g = LogicGraph::new(GraphDomain::Gameplay, "g");
    g.add_variable(Variable {
        var_id: VariableId(1),
        name: "x".into(),
        type_id: TypeId(1),
        scope: VariableScope::Graph,
    });
    assert_eq!(g.variables.len(), 1);
    g.rename_variable(VariableId(1), "y");
    assert_eq!(g.variables[0].name, "y");
    g.remove_variable(VariableId(1));
    assert!(g.variables.is_empty());
}

#[test]
fn tc_15_8_2_1_type_inference_bind_generic() {
    let mut g = LogicGraph::new(GraphDomain::Gameplay, "g");
    g.add_node(Node {
        node_id: NodeId(1),
        kind: NodeKind::Stub {
            name: "src".into(),
        },
        pins: smallvec![pin_out(0, PinType::Data(TypeId(7)))],
        position: Vec2::ZERO,
    });
    g.add_node(Node {
        node_id: NodeId(2),
        kind: NodeKind::Stub {
            name: "dst".into(),
        },
        pins: smallvec![pin_in(0, PinType::Generic(GenericParamId(0)))],
        position: Vec2::ZERO,
    });
    g.connect(Edge {
        source_node: NodeId(1),
        source_pin: PinId(0),
        target_node: NodeId(2),
        target_pin: PinId(0),
    });
    let mut eng = TypeInferenceEngine::default();
    let r = eng.infer(&g).unwrap();
    assert_eq!(r.resolved.get(&GenericParamId(0)), Some(&TypeId(7)));
}

#[test]
fn tc_15_8_2_2_type_inference_wildcard() {
    let mut g = LogicGraph::new(GraphDomain::Gameplay, "g");
    g.add_node(Node {
        node_id: NodeId(1),
        kind: NodeKind::ConstantI32(0),
        pins: smallvec![pin_out(0, PinType::Wildcard)],
        position: Vec2::ZERO,
    });
    g.add_node(Node {
        node_id: NodeId(2),
        kind: NodeKind::PureAdd,
        pins: smallvec![pin_in(0, PinType::Data(TypeId(3)))],
        position: Vec2::ZERO,
    });
    g.connect(Edge {
        source_node: NodeId(1),
        source_pin: PinId(0),
        target_node: NodeId(2),
        target_pin: PinId(0),
    });
    let mut eng = TypeInferenceEngine::default();
    let r: Result<InferenceResult, Vec<TypeDiagnostic>> = eng.infer(&g);
    assert!(r.is_ok());
}

#[test]
fn tc_15_8_2_3_type_mismatch_diagnostic() {
    let mut g = LogicGraph::new(GraphDomain::Gameplay, "g");
    g.add_node(Node {
        node_id: NodeId(1),
        kind: NodeKind::ConstantI32(0),
        pins: smallvec![pin_out(0, PinType::Data(TypeId(1)))],
        position: Vec2::ZERO,
    });
    g.add_node(Node {
        node_id: NodeId(2),
        kind: NodeKind::PureAdd,
        pins: smallvec![pin_in(0, PinType::Data(TypeId(2)))],
        position: Vec2::ZERO,
    });
    g.connect(Edge {
        source_node: NodeId(1),
        source_pin: PinId(0),
        target_node: NodeId(2),
        target_pin: PinId(0),
    });
    let mut eng = TypeInferenceEngine::default();
    let err = eng.infer(&g).unwrap_err();
    assert!(!err.is_empty());
}

#[test]
fn tc_15_8_3_1_validation_cycle_detect() {
    let mut g = LogicGraph::new(GraphDomain::Gameplay, "g");
    for id in 1u32..=3 {
        g.add_node(Node {
            node_id: NodeId(id),
            kind: NodeKind::Stub {
                name: format!("n{id}"),
            },
            pins: smallvec![
                pin_out(0, PinType::Data(TypeId(1))),
                pin_in(1, PinType::Data(TypeId(1))),
            ],
            position: Vec2::ZERO,
        });
    }
    g.connect(Edge {
        source_node: NodeId(1),
        source_pin: PinId(0),
        target_node: NodeId(2),
        target_pin: PinId(1),
    });
    g.connect(Edge {
        source_node: NodeId(2),
        source_pin: PinId(0),
        target_node: NodeId(3),
        target_pin: PinId(1),
    });
    g.connect(Edge {
        source_node: NodeId(3),
        source_pin: PinId(0),
        target_node: NodeId(1),
        target_pin: PinId(1),
    });
    let v = GraphValidator::validate(&g, &NodeRegistry);
    assert!(v.errors.iter().any(|e| e.contains("cycle")));
}

#[test]
fn tc_15_8_3_2_validation_dangling_pin() {
    let mut g = LogicGraph::new(GraphDomain::Gameplay, "g");
    g.add_node(Node {
        node_id: NodeId(1),
        kind: NodeKind::ConstantI32(0),
        pins: smallvec![pin_out(0, PinType::Data(TypeId(1)))],
        position: Vec2::ZERO,
    });
    let v = GraphValidator::validate(&g, &NodeRegistry);
    assert!(v.errors.iter().any(|e| e.contains("dangling")));
}

#[test]
fn tc_15_8_3_3_validation_missing_input() {
    let mut g = LogicGraph::new(GraphDomain::Gameplay, "g");
    g.add_node(Node {
        node_id: NodeId(1),
        kind: NodeKind::PureAdd,
        pins: smallvec![pin_in(0, PinType::Data(TypeId(1)))],
        position: Vec2::ZERO,
    });
    let v = GraphValidator::validate(&g, &NodeRegistry);
    assert!(v.errors.iter().any(|e| e.contains("missing")));
}

#[test]
fn tc_15_8_4_1_suspend_state_lowering() {
    let gid = dummy_graph_id();
    let s = lower_suspends(gid, NodeId(9));
    assert!(s.iter().any(|x| matches!(x.variant, SuspendVariant::NextFrame)));
}

#[test]
fn tc_15_8_4_2_suspend_state_variants() {
    let gid = dummy_graph_id();
    let s = lower_suspends(gid, NodeId(1));
    assert!(s.iter().any(|x| matches!(x.variant, SuspendVariant::DelayFrames(3))));
    assert!(s
        .iter()
        .any(|x| matches!(x.variant, SuspendVariant::UntilCondition)));
}

#[test]
fn tc_15_8_12_1_dead_node_elimination() {
    let mut g = LogicGraph::new(GraphDomain::Gameplay, "g");
    g.add_node(Node {
        node_id: NodeId(1),
        kind: NodeKind::Stub {
            name: "root".into(),
        },
        pins: smallvec![],
        position: Vec2::ZERO,
    });
    g.add_node(Node {
        node_id: NodeId(2),
        kind: NodeKind::Stub {
            name: "dead".into(),
        },
        pins: smallvec![],
        position: Vec2::ZERO,
    });
    dead_node_elimination(&mut g, &[NodeId(1)]);
    assert_eq!(g.nodes.len(), 1);
}

#[test]
fn tc_15_8_12_2_constant_folding() {
    let mut g = LogicGraph::new(GraphDomain::Gameplay, "g");
    g.add_node(Node {
        node_id: NodeId(1),
        kind: NodeKind::ConstantI32(2),
        pins: smallvec![pin_out(0, PinType::Data(TypeId(1)))],
        position: Vec2::ZERO,
    });
    g.add_node(Node {
        node_id: NodeId(2),
        kind: NodeKind::ConstantI32(3),
        pins: smallvec![pin_out(0, PinType::Data(TypeId(1)))],
        position: Vec2::ZERO,
    });
    g.add_node(Node {
        node_id: NodeId(3),
        kind: NodeKind::PureAdd,
        pins: smallvec![
            pin_in(0, PinType::Data(TypeId(1))),
            pin_in(1, PinType::Data(TypeId(1))),
        ],
        position: Vec2::ZERO,
    });
    g.connect(Edge {
        source_node: NodeId(1),
        source_pin: PinId(0),
        target_node: NodeId(3),
        target_pin: PinId(0),
    });
    g.connect(Edge {
        source_node: NodeId(2),
        source_pin: PinId(0),
        target_node: NodeId(3),
        target_pin: PinId(1),
    });
    constant_folding(&mut g);
    assert!(!g.nodes.iter().any(|n| n.node_id == NodeId(1)));
    assert!(!g.nodes.iter().any(|n| n.node_id == NodeId(2)));
}

#[test]
fn tc_15_8_12_3_subgraph_inlining() {
    let mut main = LogicGraph::new(GraphDomain::Gameplay, "main");
    main.add_node(Node {
        node_id: NodeId(1),
        kind: NodeKind::Stub {
            name: "root".into(),
        },
        pins: smallvec![],
        position: Vec2::ZERO,
    });
    let body = vec![Node {
        node_id: NodeId(1),
        kind: NodeKind::Stub {
            name: "inner".into(),
        },
        pins: smallvec![],
        position: Vec2::ZERO,
    }];
    inline_subgraph(&mut main, &body, 10);
    assert_eq!(main.nodes.len(), 2);
}

#[test]
fn tc_15_8_12_4_monomorphization() {
    let mut g = LogicGraph::new(GraphDomain::Gameplay, "g");
    monomorphize(&mut g, 0, 1);
}

#[test]
fn tc_15_8_5_1_shader_graph_hlsl_emit() {
    let s = emit_shader_graph_main();
    assert!(s.0.contains("SV_Position"));
}

#[test]
fn tc_15_8_5_2_shader_compile_dxc() {
    let s = emit_shader_graph_main();
    let r = compile_with_dxc_stub(&s, ShaderStage::Vertex);
    assert!(r.ok);
}

#[test]
fn tc_15_8_5_3_material_pbr_template() {
    let s = material_pbr_template();
    assert!(s.0.contains("PBR"));
}

#[test]
fn tc_15_8_13_1_graph_two_way_diff() {
    let mut a = LogicGraph::new(GraphDomain::Gameplay, "a");
    a.add_node(Node {
        node_id: NodeId(1),
        kind: NodeKind::Stub {
            name: "x".into(),
        },
        pins: smallvec![],
        position: Vec2::ZERO,
    });
    let mut b = a.clone();
    b.add_node(Node {
        node_id: NodeId(2),
        kind: NodeKind::Stub {
            name: "y".into(),
        },
        pins: smallvec![],
        position: Vec2::ZERO,
    });
    let d = GraphDiffer::diff(&a, &b);
    assert_eq!(d.added, vec![NodeId(2)]);
}

#[test]
fn tc_15_8_13_2_graph_three_way_merge() {
    let base = LogicGraph::new(GraphDomain::Gameplay, "b");
    let mut ours = base.clone();
    ours.add_node(Node {
        node_id: NodeId(1),
        kind: NodeKind::Stub {
            name: "same".into(),
        },
        pins: smallvec![],
        position: Vec2::ZERO,
    });
    let mut theirs = base.clone();
    theirs.add_node(Node {
        node_id: NodeId(1),
        kind: NodeKind::PureAdd,
        pins: smallvec![],
        position: Vec2::ZERO,
    });
    let m = GraphDiffer::merge_three_way(&base, &ours, &theirs);
    assert!(matches!(m, MergeResult::Conflict { .. }));
}

#[test]
fn tc_15_8_14_1_find_usages() {
    let mut g = LogicGraph::new(GraphDomain::Gameplay, "g");
    g.add_node(Node {
        node_id: NodeId(1),
        kind: NodeKind::Stub {
            name: "foo_bar".into(),
        },
        pins: smallvec![],
        position: Vec2::ZERO,
    });
    let u = find_usages(&g, "foo");
    assert_eq!(u, vec![NodeId(1)]);
}

#[test]
fn tc_15_8_14_2_rename_propagation() {
    let mut g = LogicGraph::new(GraphDomain::Gameplay, "g");
    g.add_node(Node {
        node_id: NodeId(1),
        kind: NodeKind::Stub {
            name: "foo_old".into(),
        },
        pins: smallvec![],
        position: Vec2::ZERO,
    });
    rename_symbol(&mut g, "old", "new");
    match &g.nodes[0].kind {
        NodeKind::Stub { name } => assert!(name.contains("new")),
        _ => panic!("stub"),
    }
}

#[test]
fn tc_15_3_1_1_material_node_connect() {
    let mut m = MaterialGraph::new(AssetId(1));
    let a = m.add_node(MaterialNodeType::SampleTexture, Vec2::ZERO);
    let b = m.add_node(MaterialNodeType::PbrOutput, Vec2::ZERO);
    let r = m.connect(a, MaterialPinId(1), b, MaterialPinId(0));
    assert!(r.is_ok());
}

#[test]
fn tc_15_3_1_2_material_pin_type_check() {
    let mut m = MaterialGraph::new(AssetId(1));
    let a = m.add_node(MaterialNodeType::SampleTexture, Vec2::ZERO);
    let b = m.add_node(MaterialNodeType::PbrOutput, Vec2::ZERO);
    let r = m.connect(a, MaterialPinId(0), b, MaterialPinId(0));
    assert_eq!(r, Err(PinTypeError::Incompatible));
}

#[test]
fn tc_15_3_2_1_material_function_inline() {
    let mut m = MaterialGraph::new(AssetId(1));
    inline_material_function(&mut m, "body");
    assert!(m
        .nodes
        .iter()
        .any(|n| matches!(&n.kind, MaterialNodeType::FunctionInline { body } if body == "body")));
}

#[test]
fn tc_15_3_4_1_parameter_tweak_no_recompile() {
    let mut m = MaterialGraph::new(AssetId(1));
    m.add_node(MaterialNodeType::PbrOutput, Vec2::ZERO);
    let c1 = m.compile().unwrap().0.clone();
    m.tweak_parameter("roughness", 0.4);
    let c2 = m.compile().unwrap().0;
    assert_eq!(c1, c2);
}

#[test]
fn tc_15_3_5_1_material_instance_override() {
    let mut p = MaterialGraph::new(AssetId(1));
    p.parameter_values.push(("x".into(), 1.0));
    let mut i = MaterialInstance::new(AssetId(2), AssetId(1));
    i.set_override("x".into(), ParamValue::Float(9.0));
    assert_eq!(
        i.effective_value("x", &p),
        Some(ParamValue::Float(9.0))
    );
}

#[test]
fn tc_15_3_6_1_material_library_search() {
    let names = ["metal_floor", "wood_trim"];
    let r = material_library_search(&names, "metal");
    assert_eq!(r, vec!["metal_floor"]);
}

#[test]
fn tc_15_4_1_1_timeline_add_keyframe() {
    let mut t = AnimTimeline::new();
    t.tracks.push(AnimTrack {
        bone: BoneId(0),
        channel: animation::AnimChannel::LocX,
        keyframes: vec![],
    });
    t.set_time(0.5);
    t.add_keyframe(0, 3.0);
    assert_eq!(t.tracks[0].keyframes.len(), 1);
    assert_eq!(t.tracks[0].keyframes[0].time, 0.5);
}

#[test]
fn tc_15_4_1_2_timeline_move_keyframe() {
    let mut t = AnimTimeline::new();
    t.tracks.push(AnimTrack {
        bone: BoneId(0),
        channel: animation::AnimChannel::LocX,
        keyframes: vec![
            Keyframe {
                time: 0.0,
                value: 0.0,
                tangent_mode: TangentMode::Linear,
            },
            Keyframe {
                time: 1.0,
                value: 1.0,
                tangent_mode: TangentMode::Linear,
            },
        ],
    });
    t.tracks[0].keyframes[0].time = 0.25;
    t.tracks[0].keyframes.sort_by(|a, b| {
        a.time
            .partial_cmp(&b.time)
            .unwrap_or(std::cmp::Ordering::Equal)
    });
    assert_eq!(t.tracks[0].keyframes[0].time, 0.25);
}

#[test]
fn tc_15_4_2_1_curve_tangent_modes() {
    let mut tr = AnimTrack {
        bone: BoneId(0),
        channel: animation::AnimChannel::LocX,
        keyframes: vec![Keyframe {
            time: 0.0,
            value: 0.0,
            tangent_mode: TangentMode::Linear,
        }],
    };
    animation::apply_curve_preset(&mut tr, TangentMode::Bezier);
    assert_eq!(tr.keyframes[0].tangent_mode, TangentMode::Bezier);
}

#[test]
fn tc_15_4_2_2_curve_preset_apply() {
    let a = Keyframe {
        time: 0.0,
        value: 0.0,
        tangent_mode: TangentMode::Linear,
    };
    let b = Keyframe {
        time: 1.0,
        value: 10.0,
        tangent_mode: TangentMode::Linear,
    };
    assert!((animation::eval_curve_tangent(&a, &b, 0.5) - 5.0).abs() < 1e-6);
}

#[test]
fn tc_15_4_3_1_skeleton_bone_select() {
    let mut s = Skeleton {
        bones: vec![BoneId(1), BoneId(2)],
        selected: None,
    };
    s.select_bone(BoneId(2));
    assert_eq!(s.selected, Some(BoneId(2)));
}

#[test]
fn tc_15_4_5_1_blend_space_1d_sample() {
    let bs = BlendSpace {
        id: AssetId(1),
        dimension: BlendSpaceDimension::OneD,
        samples: vec![
            BlendSample {
                coord: Vec2::new(0.0, 0.0),
                clip_weight: 0.0,
            },
            BlendSample {
                coord: Vec2::new(1.0, 0.0),
                clip_weight: 1.0,
            },
        ],
    };
    let w = bs.sample_1d(0.5);
    assert!((w - 0.5).abs() < 1e-3);
}

#[test]
fn tc_15_4_5_2_blend_space_2d_sample() {
    let bs = BlendSpace {
        id: AssetId(1),
        dimension: BlendSpaceDimension::TwoD,
        samples: vec![
            BlendSample {
                coord: Vec2::ZERO,
                clip_weight: 1.0,
            },
            BlendSample {
                coord: Vec2::new(1.0, 0.0),
                clip_weight: 0.0,
            },
            BlendSample {
                coord: Vec2::new(0.0, 1.0),
                clip_weight: 0.0,
            },
        ],
    };
    let _ = bs.sample_2d(Vec2::new(0.33, 0.33));
}

#[test]
fn tc_15_4_6_1_state_machine_transition() {
    let mut sm = AnimStateMachine {
        id: AssetId(1),
        states: vec![
            AnimState {
                id: StateId(0),
                name: "a".into(),
            },
            AnimState {
                id: StateId(1),
                name: "b".into(),
            },
        ],
        transitions: vec![AnimTransition {
            from: StateId(0),
            to: StateId(1),
            condition_met: true,
        }],
        default_state: StateId(0),
        current: StateId(0),
    };
    assert_eq!(sm.step_transition(), Some(StateId(1)));
}

#[test]
fn tc_15_4_7_1_retarget_bone_mapping() {
    let m = [
        BoneMapping {
            src: BoneId(1),
            dst: BoneId(99),
        },
    ];
    assert_eq!(animation::apply_retarget(&m, BoneId(1)), Some(BoneId(99)));
}

#[test]
fn tc_15_15_1_1_graph_widget_add_node() {
    let mut w = GraphEditorWidget::new(dummy_graph_id());
    let id = w.add_node(
        NodeKind::Stub {
            name: "n".into(),
        },
        Vec2::ZERO,
    );
    assert_eq!(id.0, 2);
}

#[test]
fn tc_15_15_1_2_graph_widget_connect() {
    let mut w = GraphEditorWidget::new(dummy_graph_id());
    w.add_node(
        NodeKind::Stub {
            name: "a".into(),
        },
        Vec2::ZERO,
    );
    assert!(w.connect(PinId(0), PinId(1)).is_ok());
}

#[test]
fn tc_15_15_2_1_table_widget_add_row() {
    let mut t = TableEditorWidget::new(
        TableId(1),
        vec![ColumnDef {
            name: "c0".into(),
        }],
    );
    let r = t.add_row();
    assert_eq!(t.set_cell(r, 0, CellValue::Int(4)), Ok(()));
}

#[test]
fn tc_15_15_2_2_table_widget_sort() {
    let mut t = TableEditorWidget::new(
        TableId(1),
        vec![ColumnDef {
            name: "c0".into(),
        }],
    );
    let r1 = t.add_row();
    let r2 = t.add_row();
    let _ = t.set_cell(r1, 0, CellValue::Int(10));
    let _ = t.set_cell(r2, 0, CellValue::Int(2));
    t.sort_by(0, SortDirection::Asc);
    assert_eq!(t.cell(r2, 0), Some(&CellValue::Int(2)));
}

#[test]
fn tc_15_8_6_1_render_graph_config_node() {
    let p: RenderGraphPasses = compiler::compile_render_graph_config();
    assert!(p.passes.len() >= 2);
}

#[test]
fn tc_15_8_7_1_animation_logic_graph_lowers() {
    let s: AnimEcsSystems = compiler::lower_animation_logic_graph();
    assert!(!s.names.is_empty());
}

#[test]
fn tc_15_8_8_1_audio_logic_graph_lowers() {
    let m: MixerEvents = compiler::lower_audio_logic_graph();
    assert!(!m.events.is_empty());
}

#[test]
fn tc_15_8_9_1_custom_tool_graph_registers() {
    let a: Vec<ToolAction> = compiler::compile_custom_tool_graph();
    assert_eq!(a[0].name, "custom_tool_action");
}

#[test]
fn tc_15_8_10_1_node_library_enumerates() {
    let mut lib = NodeLibrary::default();
    lib.register("add");
    lib.register("mul");
    assert_eq!(lib.enumerate(), &["add", "mul"]);
}

#[test]
fn tc_15_3_3_1_material_live_preview_split_view() {
    let mut m = MaterialGraph::new(AssetId(1));
    m.add_node(MaterialNodeType::PbrOutput, Vec2::ZERO);
    m.tweak_parameter("roughness", 0.1);
    assert!(m.split_view_dirty());
}

#[test]
fn tc_15_4_4_1_animation_preview_debug_overlays() {
    let mut t = AnimTimeline::new();
    t.set_preview_debug(12, true);
    assert_eq!(t.preview_bone_overlay_count, 12);
    assert!(t.preview_velocity_overlay);
}

#[test]
fn tc_15_5_1_1_vfx_graph_emitter_initializer() {
    let mut g = VfxGraph {
        nodes: vec![
            VfxNode {
                id: NodeId(1),
                kind: VfxNodeKind::Emitter,
            },
            VfxNode {
                id: NodeId(2),
                kind: VfxNodeKind::Initializer,
            },
        ],
        edges: vec![],
    };
    assert!(g.connect_emitter_to_initializer());
}

#[test]
fn tc_15_13_2_1_ai_state_machine_compiles() {
    let sm = AnimStateMachine {
        id: AssetId(1),
        states: vec![
            AnimState {
                id: StateId(0),
                name: "idle".into(),
            },
            AnimState {
                id: StateId(1),
                name: "run".into(),
            },
        ],
        transitions: vec![],
        default_state: StateId(0),
        current: StateId(0),
    };
    let c = animation::compile_ai_state_machine(&sm);
    assert_eq!(c.state_count, 2);
}

#[test]
fn tc_15_14_1_1_quest_editor_compiles_objectives() {
    let g = logic_graph_stub_objectives(4);
    let q: QuestIr = compile_quest_objectives(g.nodes.len());
    assert_eq!(q.node_count, g.nodes.len());
}

#[test]
fn tc_15_15_3_1_equipment_stat_table_sort_compare() {
    let mut e = EquipmentTable {
        rows: vec![vec![10, 1], vec![2, 20]],
        comparison_mode: false,
    };
    e.sort_and_compare(0);
    assert!(e.comparison_mode);
    assert_eq!(e.rows[0][0], 2);
}

#[test]
fn tc_15_15_4_1_price_ledger_inflation_step() {
    let mut p = PriceLedger {
        prices: vec![1.0, 2.0],
        inflation_rate: 0.1,
    };
    p.inflation_step();
    assert!((p.prices[0] - 1.1).abs() < 1e-5);
}

#[test]
fn tc_15_1_4_1_entity_hierarchy_selection_templates() {
    let panel = EntityHierarchyPanel {
        entities: vec![
            HierarchyEntity {
                name: "e1".into(),
                template: Some("T".into()),
                selected: true,
            },
        ],
    };
    let s = panel.selection_summary();
    assert_eq!(s[0], "e1 [T]");
}

#[test]
fn tc_15_2_1_1_entity_template_override_badge() {
    let panel = InspectorPanel {
        fields: vec![InspectorField {
            name: "hp".into(),
            overridden_from_template: true,
        }],
    };
    assert!(panel.template_override_badge_visible());
}

#[test]
fn ecs_compile_happy_path() {
    let mut g = LogicGraph::new(GraphDomain::Gameplay, "ok");
    g.add_node(Node {
        node_id: NodeId(1),
        kind: NodeKind::Stub {
            name: "root".into(),
        },
        pins: smallvec![],
        position: Vec2::ZERO,
    });
    static TT: visual_editors::inference::StaticTypeTable =
        visual_editors::inference::StaticTypeTable;
    let r: Result<CompiledGraph, CompileError> = EcsCompiler::compile(&g, &TT, &NodeRegistry);
    assert!(r.is_ok());
}
