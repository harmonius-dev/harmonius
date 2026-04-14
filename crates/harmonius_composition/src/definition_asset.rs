//! [`DefinitionAsset`] trait from the composition design.

use bevy_ecs::prelude::{Entity, World};

use crate::asset_id::AssetId;
use crate::bind_error::BindError;

/// Unifies immutable definition assets binding to ECS components (R-16.5.1).
pub trait DefinitionAsset: Sized + rkyv::Archive {
    /// ECS component produced by [`DefinitionAsset::bind`](Self::bind).
    type Component: bevy_ecs::prelude::Component;
    /// Copyable handle carrying version and generation for revocation checks.
    type Handle: Copy + Eq;

    /// Asset identifier.
    fn id(&self) -> AssetId;

    /// Authoring version; must match the handle's version field on bind.
    fn version(&self) -> u32;

    /// Attach the corresponding ECS component (idempotent per design).
    fn bind(
        &self,
        world: &mut World,
        entity: Entity,
        handle: Self::Handle,
    ) -> Result<(), BindError>;

    /// Detach the component and revoke the handle generation.
    fn unbind(&self, world: &mut World, entity: Entity) -> Result<(), BindError>;
}
