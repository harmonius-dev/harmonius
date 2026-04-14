//! Ragdoll asset types and generational `Handle` storage.

use core::marker::PhantomData;

use rkyv_derive::{Archive, Deserialize, Serialize};

/// Stable bone index inside a skeleton palette.
#[derive(Archive, Clone, Copy, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
#[archive(compare(PartialEq))]
#[archive_attr(derive(Debug, Eq, PartialEq))]
pub struct BoneIndex(pub u16);

/// Authoring-time collider shape paired with a ragdoll bone.
#[derive(Archive, Clone, Debug, Deserialize, PartialEq, Serialize)]
#[archive_attr(derive(Debug, PartialEq))]
pub enum ColliderShape {
    /// Sphere primitive.
    Sphere {
        /// Radius in world units.
        radius: f32,
    },
    /// Capsule aligned to the local Y axis.
    Capsule {
        /// Cylinder half-height excluding hemispheres.
        half_height: f32,
        /// Hemispherical radius.
        radius: f32,
    },
}

/// One rigid body entry inside a ragdoll definition.
#[derive(Archive, Clone, Debug, Deserialize, PartialEq, Serialize)]
#[archive_attr(derive(Debug, PartialEq))]
pub struct RagdollBone {
    /// Skeleton bone index.
    pub bone_index: BoneIndex,
    /// Collision shape for this body.
    pub shape: ColliderShape,
    /// Mass in kilograms.
    pub mass: f32,
    /// Coulomb friction coefficient.
    pub friction: f32,
    /// Restitution coefficient.
    pub restitution: f32,
}

/// Ragdoll joint limits between parent and child bones.
#[derive(Archive, Clone, Debug, Deserialize, PartialEq, Serialize)]
#[archive_attr(derive(Debug, PartialEq))]
pub struct RagdollConstraint {
    /// Parent bone index.
    pub parent_bone: BoneIndex,
    /// Child bone index.
    pub child_bone: BoneIndex,
    /// Twist limit in radians.
    pub twist_limit: f32,
    /// Swing limit in radians (cone half-angle).
    pub swing_limit: f32,
}

/// Serialized ragdoll asset mapping bones to rigid bodies and constraints.
#[derive(Archive, Clone, Debug, Deserialize, PartialEq, Serialize)]
#[archive_attr(derive(Debug, PartialEq))]
pub struct RagdollDef {
    /// Rigid bodies for mapped bones.
    pub bone_bodies: Vec<RagdollBone>,
    /// Joint constraints between bones.
    pub constraints: Vec<RagdollConstraint>,
}

/// Generational index handle into `RagdollDefStore`.
#[derive(Debug, Eq, PartialEq)]
pub struct Handle<T> {
    /// Slot index.
    pub index: u32,
    /// Generation counter for invalidation.
    pub generation: u32,
    _marker: PhantomData<fn() -> T>,
}

impl<T> Handle<T> {
    /// Builds a typed handle.
    pub const fn new(index: u32, generation: u32) -> Self {
        Self {
            index,
            generation,
            _marker: PhantomData,
        }
    }
}

impl<T> Clone for Handle<T> {
    fn clone(&self) -> Self {
        *self
    }
}

impl<T> Copy for Handle<T> {}

/// In-memory store for `RagdollDef` values addressed by `Handle`.
#[derive(Debug, Default)]
pub struct RagdollDefStore {
    slots: Vec<Option<(u32, RagdollDef)>>,
}

impl RagdollDefStore {
    /// Inserts a definition, returning a generational handle.
    pub fn insert(&mut self, def: RagdollDef) -> Handle<RagdollDef> {
        let generation = 1u32;
        for (i, slot) in self.slots.iter_mut().enumerate() {
            if slot.is_none() {
                *slot = Some((generation, def));
                return Handle::new(i as u32, generation);
            }
        }
        let i = self.slots.len() as u32;
        self.slots.push(Some((generation, def)));
        Handle::new(i, generation)
    }

    /// Returns the definition if the handle matches the stored generation.
    pub fn get(&self, handle: Handle<RagdollDef>) -> Option<&RagdollDef> {
        let slot = self.slots.get(handle.index as usize)?.as_ref()?;
        (slot.0 == handle.generation).then_some(&slot.1)
    }

    /// Looks up a bone entry by `BoneIndex`.
    pub fn bone(&self, handle: Handle<RagdollDef>, bone_index: BoneIndex) -> Option<&RagdollBone> {
        let def = self.get(handle)?;
        def.bone_bodies.iter().find(|b| b.bone_index == bone_index)
    }

    /// Removes a slot, bumping generation so stale handles fail lookups.
    pub fn remove(&mut self, handle: Handle<RagdollDef>) {
        if let Some(slot) = self.slots.get_mut(handle.index as usize) {
            if let Some((gen, _)) = slot {
                if *gen == handle.generation {
                    *gen = gen.saturating_add(1);
                    *slot = None;
                }
            }
        }
    }
}
