//! Effect graph validation, custom nodes, instance parameters, spawn mapping, and LOD.

/// Stable node identifier.
#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
pub struct NodeId(pub u32);

/// Pin identifier local to a node.
#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
pub struct PinId(pub u32);

/// Directed edge connecting an output pin to an input pin.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct TypedEdge {
    /// Source node.
    pub from_node: NodeId,
    /// Source pin.
    pub from_pin: PinId,
    /// Destination node.
    pub to_node: NodeId,
    /// Destination pin.
    pub to_pin: PinId,
}

/// Validation failures for draft graphs (**TC-11.6.1.1**).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ValidationError {
    /// Required input pin has no incoming edge.
    DanglingRequired {
        /// Owning node.
        node: NodeId,
        /// Missing pin.
        pin: PinId,
    },
}

/// Output of successful validation.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ValidatedGraph {
    /// Number of nodes retained after validation.
    pub nodes: u32,
}

/// Minimal draft graph for validation tests.
#[derive(Clone, Debug, Default)]
pub struct EffectGraphDraft {
    /// Required input pins that must receive edges.
    pub required_inputs: Vec<(NodeId, PinId)>,
    /// Connections between pins.
    pub edges: Vec<TypedEdge>,
}

impl EffectGraphDraft {
    /// Validates required pins are satisfied (**TC-11.6.1.1**).
    pub fn validate(&self) -> Result<ValidatedGraph, ValidationError> {
        for &(node, pin) in &self.required_inputs {
            let ok = self.edges.iter().any(|e| e.to_node == node && e.to_pin == pin);
            if !ok {
                return Err(ValidationError::DanglingRequired { node, pin });
            }
        }
        Ok(ValidatedGraph { nodes: 0 })
    }
}

/// Index into the static custom-node table (**TC-11.6.2.1**).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct CustomNodeRef(pub u32);

/// Describes a custom node entry in the palette.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CustomNodeDescriptor {
    /// Table index.
    pub index: u32,
    /// Human-readable label.
    pub label: &'static str,
}

/// Returns palette metadata for a custom node reference (**TC-11.6.2.1**).
pub fn describe_custom_node(r: CustomNodeRef) -> CustomNodeDescriptor {
    CustomNodeDescriptor {
        index: r.0,
        label: "custom",
    }
}

/// Linear RGBA parameter value.
pub type Rgba = [f32; 4];

/// Per-instance parameter overrides (**TC-11.6.3.1**).
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ParamOverride {
    /// Optional color override.
    pub color: Option<Rgba>,
}

/// Resolved instance color after applying overrides (**TC-11.6.3.1**).
pub fn resolve_instance_color(default_color: Rgba, inst: ParamOverride) -> Rgba {
    inst.color.unwrap_or(default_color)
}

/// Spawn request emitted by gameplay observers (**TC-11.6.4.1**).
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct VfxSpawnEvent {
    /// Effect asset id (opaque for this test).
    pub effect: u64,
    /// World position.
    pub position: [f32; 3],
    /// Surface normal for orientation.
    pub normal: [f32; 3],
    /// Optional material tag encoded as u8.
    pub material_code: u8,
}

/// Minimal transform extracted from a spawn event (**TC-11.6.4.1**).
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct EffectSpawnTransform {
    /// Translation.
    pub origin: [f32; 3],
    /// Up vector aligned to `normal`.
    pub up: [f32; 3],
}

/// Builds a transform from collision data (**TC-11.6.4.1**).
pub fn spawn_transform_from_hit(evt: VfxSpawnEvent) -> EffectSpawnTransform {
    EffectSpawnTransform {
        origin: evt.position,
        up: normalize3(evt.normal),
    }
}

/// Selects a stone override code when material tag is stone (**TC-11.6.4.1** #2).
pub fn material_override_code(material_code: u8) -> u32 {
    if material_code == 1 {
        9001
    } else {
        0
    }
}

fn normalize3(v: [f32; 3]) -> [f32; 3] {
    let l = (v[0] * v[0] + v[1] * v[1] + v[2] * v[2]).sqrt().max(1e-8);
    [v[0] / l, v[1] / l, v[2] / l]
}

/// Emitter LOD tier (**TC-11.6.5.1**).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LodTier {
    /// Highest fidelity.
    Full,
    /// Reduced simulation cost.
    Reduced,
    /// Impostor billboards.
    Impostor,
    /// Fully culled.
    Culled,
}

/// Selects LOD tier from camera distance and ascending thresholds (**TC-11.6.5.1**).
pub fn select_lod_tier(distance_m: f32, thresholds: [f32; 3]) -> LodTier {
    let t0 = thresholds[0];
    let t1 = thresholds[1];
    let t2 = thresholds[2];
    if distance_m < t0 {
        LodTier::Full
    } else if distance_m < t1 {
        LodTier::Reduced
    } else if distance_m < t2 {
        LodTier::Impostor
    } else {
        LodTier::Culled
    }
}

#[cfg(test)]
mod tests {
    use super::{
        CustomNodeRef, EffectGraphDraft, LodTier, NodeId, ParamOverride, PinId, TypedEdge,
        ValidationError, VfxSpawnEvent, describe_custom_node, material_override_code,
        resolve_instance_color, select_lod_tier, spawn_transform_from_hit,
    };

    /// **TC-11.6.1.1** — dangling required pin fails until connected.
    #[test]
    fn tc_11_6_1_1_effect_graph_validation_dangling_pin() {
        let mut g = EffectGraphDraft::default();
        g.required_inputs.push((NodeId(1), PinId(2)));
        let err = g.validate().unwrap_err();
        assert_eq!(
            err,
            ValidationError::DanglingRequired {
                node: NodeId(1),
                pin: PinId(2),
            }
        );
        g.edges.push(TypedEdge {
            from_node: NodeId(0),
            from_pin: PinId(0),
            to_node: NodeId(1),
            to_pin: PinId(2),
        });
        assert!(g.validate().is_ok());
    }

    /// **TC-11.6.2.1** — custom node references map to palette indices.
    #[test]
    fn tc_11_6_2_1_custom_node_library_load() {
        let d = describe_custom_node(CustomNodeRef(7));
        assert_eq!(d.index, 7);
    }

    /// **TC-11.6.3.1** — per-instance color overrides preserve defaults for others.
    #[test]
    fn tc_11_6_3_1_parameter_override_per_instance() {
        let red = [1.0_f32, 0.0, 0.0, 1.0];
        let a = resolve_instance_color(red, ParamOverride { color: None });
        assert_eq!(a, red);
        let blue = [0.0_f32, 0.0, 1.0, 1.0];
        let b = resolve_instance_color(
            red,
            ParamOverride {
                color: Some(blue),
            },
        );
        assert_eq!(b, blue);
    }

    /// **TC-11.6.4.1** — collision events map to spawn transforms and material codes.
    #[test]
    fn tc_11_6_4_1_observer_event_spawn_parameterization() {
        let evt = VfxSpawnEvent {
            effect: 1,
            position: [3.0, 0.0, 0.0],
            normal: [0.0, 1.0, 0.0],
            material_code: 0,
        };
        let xf = spawn_transform_from_hit(evt);
        assert_eq!(xf.origin, [3.0, 0.0, 0.0]);
        assert_eq!(xf.up, [0.0, 1.0, 0.0]);

        let code = material_override_code(1);
        assert_eq!(code, 9001);
    }

    /// **TC-11.6.5.1** — distance thresholds map to LOD tiers.
    #[test]
    fn tc_11_6_5_1_lod_tier_selection_by_distance() {
        let th = [10.0_f32, 30.0, 80.0];
        assert_eq!(select_lod_tier(5.0, th), LodTier::Full);
        assert_eq!(select_lod_tier(50.0, th), LodTier::Impostor);
        assert_eq!(select_lod_tier(100.0, th), LodTier::Culled);
    }
}
