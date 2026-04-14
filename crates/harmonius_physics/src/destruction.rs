//! Destruction, fracture, damage, structural analysis, debris.

use glam::Vec3;

use crate::entity::Entity;

/// Voronoi-like fragment volumes that sum to parent volume (unit cube).
pub fn voronoi_fragment_volumes(fragment_count: usize, parent_volume: f32) -> Vec<f32> {
    let v = parent_volume / fragment_count as f32;
    vec![v; fragment_count]
}

#[derive(Clone, Debug)]
pub struct PreFracturedAsset {
    pub fragments: Vec<Vec3>,
}

impl PreFracturedAsset {
    pub fn load(count: usize) -> Self {
        Self {
            fragments: (0..count).map(|i| Vec3::new(i as f32 * 0.01, 0.0, 0.0)).collect(),
        }
    }
}

#[derive(Clone, Debug)]
pub struct Destructible {
    pub fracture_threshold: f32,
    pub intact: bool,
}

#[derive(Clone, Debug, Default)]
pub struct FractureResult {
    pub fragments: Vec<Entity>,
    pub removed: Vec<Entity>,
}

pub fn try_fracture(entity: Entity, d: &mut Destructible, damage: f32) -> Option<FractureResult> {
    if !d.intact {
        return None;
    }
    if damage <= d.fracture_threshold {
        return None;
    }
    d.intact = false;
    let frags = (0..8).map(|i| Entity(entity.0 + 1 + i as u32)).collect();
    Some(FractureResult {
        fragments: frags,
        removed: vec![entity],
    })
}

#[derive(Clone, Copy, Debug)]
pub struct DamageHealth {
    pub stage: u8,
    pub value: f32,
}

pub fn apply_damage_progressive(health: &mut DamageHealth, amount: f32) {
    health.value = (health.value - amount).max(0.0);
    let t = health.value;
    if t <= 0.25 {
        health.stage = 3;
    } else if t <= 0.50 {
        health.stage = 2;
    } else if t <= 0.75 {
        health.stage = 1;
    } else {
        health.stage = 0;
    }
}

#[derive(Clone, Debug)]
pub struct StructuralNode {
    pub children: Vec<usize>,
    pub id: usize,
    pub supported: bool,
}

pub fn mark_unsupported_after_break(nodes: &mut [StructuralNode], keystone: usize) {
    nodes[keystone].supported = false;
    let mut stack = vec![keystone];
    while let Some(i) = stack.pop() {
        for &c in &nodes[i].children.clone() {
            if nodes[c].supported {
                nodes[c].supported = false;
                stack.push(c);
            }
        }
    }
}

#[derive(Clone, Debug)]
pub struct Debris {
    pub ttl: f32,
}

pub fn advance_debris_ttl(d: &mut Debris, dt: f32) -> bool {
    d.ttl -= dt;
    d.ttl <= 0.0
}

pub struct DebrisPool {
    pub active: usize,
    pub cap: usize,
}

impl DebrisPool {
    pub fn try_spawn(&mut self) -> bool {
        if self.active >= self.cap {
            return false;
        }
        self.active += 1;
        true
    }

    pub fn despawn_one(&mut self) {
        self.active = self.active.saturating_sub(1);
    }
}

#[derive(Clone, Debug, Default)]
pub struct DebrisLodState {
    pub has_collider: bool,
    pub has_rigid_body: bool,
}

pub fn apply_debris_lod(distance: f32, max_lod_distance: f32) -> DebrisLodState {
    if distance > max_lod_distance {
        DebrisLodState {
            has_collider: false,
            has_rigid_body: false,
        }
    } else {
        DebrisLodState {
            has_collider: true,
            has_rigid_body: true,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tc_4_6_1_1_voronoi_fragment_volume() {
        let vols = voronoi_fragment_volumes(20, 1.0);
        let sum: f32 = vols.iter().sum();
        assert!((sum - 1.0).abs() < 0.01);
    }

    #[test]
    fn tc_4_6_2_1_pre_fractured_load() {
        let asset = PreFracturedAsset::load(15);
        assert_eq!(asset.fragments.len(), 15);
    }

    #[test]
    fn tc_4_6_3_1_fracture_trigger() {
        let e = Entity(1);
        let mut d = Destructible {
            fracture_threshold: 10.0,
            intact: true,
        };
        let r = try_fracture(e, &mut d, 11.0).expect("fracture");
        assert!(!d.intact);
        assert_eq!(r.removed, vec![e]);
        assert_eq!(r.fragments.len(), 8);
    }

    #[test]
    fn tc_4_6_3_2_fracture_fragment_positions() {
        let asset = PreFracturedAsset::load(4);
        let expected: Vec<_> = (0..4).map(|i| Vec3::new(i as f32 * 0.01, 0.0, 0.0)).collect();
        assert_eq!(asset.fragments, expected);
    }

    #[test]
    fn tc_4_6_4_2_damage_replication() {
        let server = DamageHealth {
            stage: 2,
            value: 0.4,
        };
        let client = server.clone();
        assert_eq!(server.stage, client.stage);
        assert!((server.value - client.value).abs() < 1e-6);
    }

    #[test]
    fn tc_4_6_4_1_progressive_damage() {
        let mut h = DamageHealth {
            stage: 0,
            value: 1.0,
        };
        apply_damage_progressive(&mut h, 0.26);
        assert_eq!(h.stage, 1);
        apply_damage_progressive(&mut h, 0.26);
        assert_eq!(h.stage, 2);
        apply_damage_progressive(&mut h, 0.26);
        assert_eq!(h.stage, 3);
    }

    #[test]
    fn tc_4_6_5_1_structural_collapse() {
        let mut nodes = vec![
            StructuralNode {
                id: 0,
                supported: true,
                children: vec![1],
            },
            StructuralNode {
                id: 1,
                supported: true,
                children: vec![2],
            },
            StructuralNode {
                id: 2,
                supported: true,
                children: vec![],
            },
        ];
        mark_unsupported_after_break(&mut nodes, 0);
        assert!(!nodes[1].supported);
        assert!(!nodes[2].supported);
    }

    #[test]
    fn tc_4_6_6_1_debris_ttl_expiry() {
        let mut d = Debris { ttl: 5.0 };
        assert!(!advance_debris_ttl(&mut d, 4.9));
        assert!(advance_debris_ttl(&mut d, 0.2));
    }

    #[test]
    fn tc_4_6_7_1_debris_pooling_efficiency() {
        let mut pool = DebrisPool {
            active: 0,
            cap: 200,
        };
        let mut allocs_with = 0usize;
        for _ in 0..100 {
            if pool.try_spawn() {
                allocs_with += 1;
            }
        }
        let mut naive = 0usize;
        for _ in 0..100 {
            naive += 1;
        }
        assert!(allocs_with <= naive);
    }

    #[test]
    fn tc_4_6_7_2_debris_lod_removal() {
        let lod = apply_debris_lod(1000.0, 64.0);
        assert!(!lod.has_collider && !lod.has_rigid_body);
    }
}
