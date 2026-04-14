//! Hindley-Milner style binding for editor cold paths.

use std::collections::HashMap;

use crate::graph_ir::{
    Edge, GenericParamId, LogicGraph, PinDirection, PinId, PinType, TypeId,
};

/// Editor-only type table slice (cold path).
pub struct StaticTypeTable;

/// Successful inference output.
#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct InferenceResult {
    pub resolved: HashMap<GenericParamId, TypeId>,
}

/// Single type diagnostic for a failed edge or pin.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TypeDiagnostic {
    pub message: String,
}

/// Incremental inference engine with generic bindings.
#[derive(Debug, Default)]
pub struct TypeInferenceEngine {
    bindings: HashMap<GenericParamId, TypeId>,
}

impl TypeInferenceEngine {
    /// Recomputes bindings for the entire graph.
    pub fn infer(&mut self, graph: &LogicGraph) -> Result<InferenceResult, Vec<TypeDiagnostic>> {
        self.bindings.clear();
        let mut errors = Vec::new();
        for edge in &graph.edges {
            self.unify_edge(graph, edge, &mut errors);
        }
        if errors.is_empty() {
            Ok(InferenceResult {
                resolved: self.bindings.clone(),
            })
        } else {
            Err(errors)
        }
    }

    /// Updates bindings after a single edge add or removal.
    pub fn update_edge(
        &mut self,
        graph: &LogicGraph,
        edge: &Edge,
        added: bool,
    ) -> Result<InferenceResult, Vec<TypeDiagnostic>> {
        let mut errors = Vec::new();
        if added {
            self.unify_edge(graph, edge, &mut errors);
        } else {
            self.infer(graph)?;
        }
        if errors.is_empty() {
            Ok(InferenceResult {
                resolved: self.bindings.clone(),
            })
        } else {
            Err(errors)
        }
    }

    fn unify_edge(&mut self, graph: &LogicGraph, edge: &Edge, errors: &mut Vec<TypeDiagnostic>) {
        let Some(src) = find_pin(graph, edge.source_node, edge.source_pin) else {
            return;
        };
        let Some(dst) = find_pin(graph, edge.target_node, edge.target_pin) else {
            return;
        };
        if src.direction == PinDirection::In || dst.direction == PinDirection::Out {
            return;
        }
        self.unify_types(&src.pin_type, &dst.pin_type, errors);
    }

    fn unify_types(&mut self, a: &PinType, b: &PinType, errors: &mut Vec<TypeDiagnostic>) {
        use PinType::*;
        match (a, b) {
            (Data(ta), Data(tb)) if ta == tb => {}
            (Data(t), Generic(g)) | (Generic(g), Data(t)) => {
                self.bind_generic(*g, *t, errors);
            }
            (Wildcard, Data(t)) | (Data(t), Wildcard) => {
                let _ = t;
            }
            (Generic(ga), Generic(gb)) if ga == gb => {}
            (Data(ta), Data(tb)) => {
                errors.push(TypeDiagnostic {
                    message: format!("type mismatch {:?} vs {:?}", ta, tb),
                });
            }
            (Execution, Execution) => {}
            _ => errors.push(TypeDiagnostic {
                message: "incompatible pin kinds".to_string(),
            }),
        }
    }

    fn bind_generic(&mut self, g: GenericParamId, t: TypeId, errors: &mut Vec<TypeDiagnostic>) {
        if let Some(existing) = self.bindings.get(&g) {
            if existing != &t {
                errors.push(TypeDiagnostic {
                    message: "conflicting generic binding".to_string(),
                });
            }
        } else {
            self.bindings.insert(g, t);
        }
    }
}

fn find_pin(graph: &LogicGraph, node: crate::graph_ir::NodeId, pin: PinId) -> Option<crate::graph_ir::Pin> {
    let n = graph.nodes.iter().find(|n| n.node_id == node)?;
    n.pins.iter().find(|p| p.pin_id == pin).cloned()
}
