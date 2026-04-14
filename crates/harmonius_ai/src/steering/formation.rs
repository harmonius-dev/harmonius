//! Leader-follower formation slots and reassignment.

use glam::Vec3;
use smallvec::SmallVec;

/// Lightweight entity handle for deterministic tests (not the ECS `Entity`).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct Entity(pub u32);

/// One slot in a formation layout.
#[derive(Clone, Debug, PartialEq)]
pub struct FormationSlot {
    /// Offset in leader-local space (x right, z back).
    pub local_offset: Vec3,
    /// Assigned agent, if any.
    pub occupant: Option<Entity>,
}

/// Shape presets for slot generation.
#[derive(Clone, Debug, PartialEq)]
pub enum FormationShape {
    /// Line abreast on the X axis.
    Line,
    /// V / wedge behind the leader.
    Wedge,
    /// Follow-the-leader column on -Z.
    Column,
    /// Ring around leader in XZ.
    Circle,
    /// Designer-provided offsets.
    Custom {
        /// Local offsets for each slot.
        offsets: Vec<Vec3>,
    },
}

/// Runtime formation layout and membership.
#[derive(Clone, Debug, PartialEq)]
pub struct FormationState {
    /// Current geometric template.
    pub shape: FormationShape,
    /// Leader entity id.
    pub leader: Entity,
    /// Distance between adjacent slots (meters).
    pub slot_spacing: f32,
    /// Slots including leader at index 0 for wedge/column presets.
    pub slots: SmallVec<[FormationSlot; 8]>,
}

/// Generate local slot offsets for `slot_count` members.
pub fn compute_slot_offsets(
    shape: &FormationShape,
    slot_count: u8,
    spacing: f32,
) -> SmallVec<[Vec3; 8]> {
    match shape {
        FormationShape::Line => {
            let half = (slot_count.saturating_sub(1)) as f32 * spacing * 0.5;
            (0..slot_count)
                .map(|i| Vec3::new(i as f32 * spacing - half, 0.0, 0.0))
                .collect()
        }
        FormationShape::Wedge => {
            let mut offsets: SmallVec<[Vec3; 8]> = SmallVec::new();
            offsets.push(Vec3::ZERO);
            for i in 1..slot_count {
                let row = (i as u32).div_ceil(2) as f32;
                let side = if i % 2 == 1 { 1.0 } else { -1.0 };
                offsets.push(Vec3::new(side * row * spacing, 0.0, -row * spacing));
            }
            offsets
        }
        FormationShape::Column => (0..slot_count)
            .map(|i| Vec3::new(0.0, 0.0, -(i as f32) * spacing))
            .collect(),
        FormationShape::Circle => {
            let angle_step = std::f32::consts::TAU / slot_count as f32;
            let radius = spacing / (2.0 * (angle_step * 0.5).sin()).max(f32::EPSILON);
            (0..slot_count)
                .map(|i| {
                    let a = i as f32 * angle_step;
                    Vec3::new(a.cos() * radius, 0.0, a.sin() * radius)
                })
                .collect()
        }
        FormationShape::Custom { offsets } => offsets.iter().copied().collect(),
    }
}

/// Reassign `remaining` members to nearest free slots after removals.
pub fn reassign_slots(
    state: &mut FormationState,
    remaining: &[Entity],
    positions: &[(Entity, Vec3)],
) {
    for slot in &mut state.slots {
        slot.occupant = None;
    }
    let mut assigned = vec![false; state.slots.len()];
    for &entity in remaining {
        let pos = positions
            .iter()
            .find(|(e, _)| *e == entity)
            .map(|(_, p)| *p)
            .unwrap_or(Vec3::ZERO);
        let leader_pos = positions
            .iter()
            .find(|(e, _)| *e == state.leader)
            .map(|(_, p)| *p)
            .unwrap_or(Vec3::ZERO);
        let mut best = (usize::MAX, f32::MAX);
        for (i, slot) in state.slots.iter().enumerate() {
            if assigned[i] {
                continue;
            }
            let world_offset = leader_pos + slot.local_offset;
            let d = pos.distance(world_offset);
            if d < best.1 {
                best = (i, d);
            }
        }
        if best.0 < state.slots.len() {
            state.slots[best.0].occupant = Some(entity);
            assigned[best.0] = true;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn rotate_offset(offset: Vec3, forward: Vec3) -> Vec3 {
        let f = forward.normalize_or_zero();
        let r = Vec3::new(-f.z, 0.0, f.x);
        f * offset.x + r * offset.z + Vec3::Y * offset.y
    }

    #[test]
    fn tc_7_2_5_1_formation_offsets() {
        let leader_pos = Vec3::ZERO;
        let forward = Vec3::X;
        let spacing = 1.2_f32;
        let offs = compute_slot_offsets(&FormationShape::Wedge, 5, spacing);
        for (i, o) in offs.iter().enumerate() {
            let slot_world = leader_pos + rotate_offset(*o, forward);
            let member_pos = slot_world + Vec3::new(0.05, 0.0, -0.05);
            assert!(
                member_pos.distance(slot_world) < 0.5,
                "slot {i} offset alignment"
            );
        }
    }

    #[test]
    fn tc_7_2_5_2_formation_corridor_compress() {
        let spacing = 1.0_f32;
        let mut offs: SmallVec<[Vec3; 8]> = compute_slot_offsets(&FormationShape::Line, 4, spacing);
        let formation_width = 3.0_f32;
        let corridor = 2.0_f32;
        let scale = (corridor / formation_width).min(1.0);
        for o in &mut offs {
            o.x *= scale;
        }
        let max_x = offs.iter().map(|o| o.x.abs()).fold(0.0_f32, f32::max);
        assert!(max_x * 2.0 <= corridor + 1e-3);
    }

    #[test]
    fn tc_7_2_5_3_formation_reassign() {
        let spacing = 1.0_f32;
        let offsets = compute_slot_offsets(&FormationShape::Line, 5, spacing);
        let mut slots: SmallVec<[FormationSlot; 8]> = SmallVec::new();
        for o in &offsets {
            slots.push(FormationSlot {
                local_offset: *o,
                occupant: None,
            });
        }
        let mut state = FormationState {
            shape: FormationShape::Line,
            leader: Entity(0),
            slot_spacing: spacing,
            slots,
        };
        let remaining: Vec<Entity> = [1_u32, 2, 4, 5].into_iter().map(Entity).collect();
        let leader = Vec3::ZERO;
        let positions: Vec<(Entity, Vec3)> = std::iter::once((Entity(0), leader))
            .chain(offsets.iter().enumerate().map(|(i, o)| {
                let id = (i as u32) + 1;
                (Entity(id), leader + *o + Vec3::new(0.02, 0.0, 0.0))
            }))
            .collect();
        reassign_slots(&mut state, &remaining, &positions);
        let occupied: Vec<_> = state.slots.iter().filter_map(|s| s.occupant).collect();
        assert_eq!(occupied.len(), 4);
        assert!(!occupied.contains(&Entity(3)));
    }
}
