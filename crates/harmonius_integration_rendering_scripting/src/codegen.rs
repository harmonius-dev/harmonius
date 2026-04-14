//! `codegen_hlsl` — pure graph → HLSL lowering (IR-3.5.1).

use std::cmp::Reverse;
use std::collections::BinaryHeap;

use crate::graph::ShaderGraphIr;
use crate::hlsl::{MaterialShaderCache, MaterialShaderOutput};
use crate::shader_types::{CompileTarget, PermutationKey};

/// Emits deterministic HLSL for `graph` into `arena` (IR-3.5.1).
///
/// The function is a pure transform of `graph` for [`CompileTarget::Hlsl`]:
/// repeated calls with equal inputs produce byte-identical arena payloads.
#[must_use]
pub fn codegen_hlsl(
    graph: &ShaderGraphIr,
    target: CompileTarget,
    arena: &mut MaterialShaderCache,
) -> MaterialShaderOutput {
    let order = topological_order(graph);
    let mut lines: Vec<String> = Vec::new();
    lines.push("// harmonius:material\n".to_string());
    lines.push(format!("// shading:{}\n", graph.shading_model as u8));
    match target {
        CompileTarget::Hlsl => {}
        CompileTarget::NativeDylib => {
            lines.push("// compile_target:NativeDylib\n".to_string());
        }
    }
    for id in &order {
        let kind = graph
            .nodes
            .iter()
            .find(|(n, _)| n == id)
            .map(|(_, k)| k)
            .expect("node id from topo must exist");
        lines.push(format!("// node:{}:{:?}\n", id.0, kind));
    }
    lines.push("// end\n".to_string());
    let joined = lines.concat();
    let hlsl = arena.insert_hlsl(joined.as_bytes());
    let key = PermutationKey {
        shading_model: graph.shading_model,
        features: graph.features,
        render_path: graph.render_path,
    };
    let span = arena.append_permutation_keys(core::slice::from_ref(&key));
    let content_hash = mix_content_hash(
        joined.as_bytes(),
        graph.shading_model as u8,
        graph.features.0,
        span.offset,
        span.length,
    );
    MaterialShaderOutput {
        hlsl,
        shading_model: graph.shading_model,
        features: graph.features,
        permutations: span,
        content_hash,
    }
}

fn mix_content_hash(
    hlsl: &[u8],
    shading_tag: u8,
    features: u32,
    perm_off: u32,
    perm_len: u32,
) -> u64 {
    let mut h = 0xCBF29CE484222325u64;
    for &b in hlsl {
        h ^= u64::from(b);
        h = h.wrapping_mul(0x100000001B3);
    }
    h ^= u64::from(shading_tag);
    h = h.wrapping_mul(0x100000001B3);
    h ^= u64::from(features);
    h ^= (u64::from(perm_off) << 32) | u64::from(perm_len);
    h
}

fn topological_order(graph: &ShaderGraphIr) -> Vec<crate::graph::NodeId> {
    use crate::graph::NodeId;
    let mut indegree: std::collections::BTreeMap<u32, u32> = std::collections::BTreeMap::new();
    for (id, _) in &graph.nodes {
        indegree.entry(id.0).or_insert(0);
    }
    for (p, c) in &graph.edges {
        *indegree.entry(c.0).or_insert(0) += 1;
        indegree.entry(p.0).or_insert(0);
    }
    let mut heap: BinaryHeap<Reverse<NodeId>> = BinaryHeap::new();
    for (id, _) in &graph.nodes {
        if *indegree.get(&id.0).unwrap_or(&1) == 0 {
            heap.push(Reverse(*id));
        }
    }
    let mut out: Vec<NodeId> = Vec::new();
    while let Some(Reverse(n)) = heap.pop() {
        out.push(n);
        for (p, c) in &graph.edges {
            if p.0 != n.0 {
                continue;
            }
            let e = indegree.get_mut(&c.0).expect("consumer in graph");
            *e = e.saturating_sub(1);
            if *e == 0 {
                heap.push(Reverse(*c));
            }
        }
    }
    debug_assert_eq!(out.len(), graph.nodes.len());
    out
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::graph::{GraphNodeKind, ShaderGraphIr};
    use crate::shader_types::{RenderPath, ShaderFeatures, ShadingModel};

    #[test]
    fn tc_ir_3_5_1_u1_codegen_hlsl_is_pure() {
        let g = ShaderGraphIr::pbr_three_node_graph();
        let mut arena_a = MaterialShaderCache::new();
        let mut arena_b = MaterialShaderCache::new();
        let first = codegen_hlsl(&g, CompileTarget::Hlsl, &mut arena_a);
        for _ in 0..1000 {
            let next = codegen_hlsl(&g, CompileTarget::Hlsl, &mut arena_b);
            assert_eq!(
                arena_a.resolve_hlsl(first.hlsl).unwrap(),
                arena_b.resolve_hlsl(next.hlsl).unwrap()
            );
            assert_eq!(first.content_hash, next.content_hash);
            assert_eq!(first.shading_model, next.shading_model);
            assert_eq!(first.features, next.features);
            arena_b = MaterialShaderCache::new();
        }
    }

    #[test]
    fn tc_ir_3_5_1_u3_pbr_three_node_topological_order() {
        let g = ShaderGraphIr::pbr_three_node_graph();
        let mut arena = MaterialShaderCache::new();
        let out = codegen_hlsl(&g, CompileTarget::Hlsl, &mut arena);
        let bytes = arena.resolve_hlsl(out.hlsl).expect("hlsl");
        let text = core::str::from_utf8(bytes).expect("utf8");
        let pos_albedo = text.find("node:0:").expect("albedo");
        let pos_metal = text.find("node:1:").expect("metallic");
        let pos_rough = text.find("node:2:").expect("roughness");
        let pos_surface = text.find("node:3:").expect("surface");
        assert!(pos_albedo < pos_metal);
        assert!(pos_metal < pos_rough);
        assert!(pos_rough < pos_surface);
    }

    #[test]
    fn unlit_graph_emits_expected_tag() {
        let g = ShaderGraphIr {
            shading_model: ShadingModel::Unlit,
            render_path: RenderPath::Deferred,
            features: ShaderFeatures::EMISSIVE,
            nodes: vec![(crate::graph::NodeId(0), GraphNodeKind::Albedo)],
            edges: vec![],
        };
        let mut arena = MaterialShaderCache::new();
        let out = codegen_hlsl(&g, CompileTarget::Hlsl, &mut arena);
        let text = core::str::from_utf8(arena.resolve_hlsl(out.hlsl).unwrap()).unwrap();
        assert!(text.contains("shading:8"));
    }
}
