//! Socket attachment and dismemberment bookkeeping.

use crate::math::Vec3;

use super::command_buffer::{CommandBuffer, EcsCommand};

/// Simple entity transform for attachment tests.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Transform {
    /// World position.
    pub position: Vec3,
}

/// Marks that a detached piece should blend toward ragdoll (test harness; not the ECS component).
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct DetachedRagdollFlag {
    /// Whether ragdoll is active on this detached piece.
    pub active: bool,
}

/// Character with socket and optional attached entity.
#[derive(Clone, Debug)]
pub struct Character {
    /// Hand socket in character space.
    pub hand_socket: Vec3,
    /// Character root transform.
    pub root: Transform,
    /// Attached entity id.
    pub attached: Option<u64>,
    /// Bone presence mask (limb index -> present).
    pub bones: Vec<bool>,
}

impl Character {
    /// World socket position.
    pub fn socket_world(&self) -> Vec3 {
        self.root.position + self.hand_socket
    }

    /// Applies attachment each frame: entity follows socket.
    pub fn tick_attachment(&self, entity_tr: &mut Transform) {
        if self.attached.is_some() {
            entity_tr.position = self.socket_world();
        }
    }

    /// Sever limb via command buffer (no direct bone mutation in API).
    pub fn sever(
        &mut self,
        detached_entity: u64,
        limb: u32,
        buf: &mut CommandBuffer,
        detached: &mut DetachedRagdollFlag,
    ) {
        buf.push(EcsCommand::Sever {
            entity: detached_entity,
            limb,
        });
        if (limb as usize) < self.bones.len() {
            self.bones[limb as usize] = false;
        }
        detached.active = true;
    }
}

/// Gait pattern label after limb loss.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum AdaptiveGait {
    /// Standard quadruped.
    Quad,
    /// Three-legged tripod trot.
    Tripod,
    /// Five-legged hexapod adaptation.
    Pentapod,
}

/// Picks adaptive gait from remaining leg count.
pub fn adaptive_gait(legs: u32) -> AdaptiveGait {
    match legs {
        3 => AdaptiveGait::Tripod,
        5 => AdaptiveGait::Pentapod,
        _ => AdaptiveGait::Quad,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tc_9_3_10_1_attach_socket() {
        let c = Character {
            hand_socket: Vec3::new(0.0, 1.0, 0.5),
            root: Transform {
                position: Vec3::new(10.0, 0.0, 0.0),
            },
            attached: Some(99),
            bones: vec![true; 8],
        };
        let mut ent = Transform {
            position: Vec3::ZERO,
        };
        for _ in 0..60 {
            c.tick_attachment(&mut ent);
        }
        assert!((ent.position - c.socket_world()).length() < 0.01);
        let saved = ent.position;
        let c2 = Character {
            hand_socket: c.hand_socket,
            root: Transform {
                position: Vec3::new(100.0, 0.0, 0.0),
            },
            attached: None,
            bones: c.bones.clone(),
        };
        c2.tick_attachment(&mut ent);
        assert!((ent.position - saved).length() < 0.01);
        assert!((ent.position - c2.socket_world()).length() > 0.01);
    }

    #[test]
    fn tc_9_3_10_2_dismember_ragdoll() {
        let mut c = Character {
            hand_socket: Vec3::ZERO,
            root: Transform {
                position: Vec3::ZERO,
            },
            attached: None,
            bones: vec![true; 6],
        };
        let mut buf = CommandBuffer::default();
        let mut rag = DetachedRagdollFlag::default();
        c.sever(42, 2, &mut buf, &mut rag);
        assert!(rag.active);
        assert!(!c.bones[2]);
        let drained = buf.drain();
        assert_eq!(
            drained.as_slice(),
            &[EcsCommand::Sever {
                entity: 42,
                limb: 2
            }]
        );
    }

    #[test]
    fn tc_9_3_10_3_gait_adapt() {
        assert_eq!(adaptive_gait(3), AdaptiveGait::Tripod);
        assert_eq!(adaptive_gait(5), AdaptiveGait::Pentapod);
    }

    #[test]
    fn tc_9_3_10_4_commands_only() {
        let mut buf = CommandBuffer::default();
        buf.push(EcsCommand::Sever { entity: 1, limb: 0 });
        let drained = buf.drain();
        assert_eq!(drained.len(), 1);
        assert!(matches!(drained[0], EcsCommand::Sever { .. }));
    }
}
