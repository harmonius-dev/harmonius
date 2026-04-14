//! Concrete [`DefinitionAsset`](super::DefinitionAsset) implementations for unit tests (R-16.5.1).

use std::collections::BTreeSet;

use bevy_ecs::prelude::*;

use crate::asset_id::AssetId;
use crate::bind_error::BindError;
use crate::components::{AttributeSet, Container, ContainerLayout, DirectedGraphInstance, Meter};
use crate::definition_asset::DefinitionAsset;

/// Global ledger tracking handle generations for bind / unbind (R-16.5.1).
#[derive(Resource, Default)]
pub struct BindingLedger {
    next_gen: u64,
    revoked: BTreeSet<u64>,
}

fn ledger(world: &mut World) -> Mut<'_, BindingLedger> {
    if !world.contains_resource::<BindingLedger>() {
        world.insert_resource(BindingLedger::default());
    }
    world.resource_mut::<BindingLedger>()
}

/// Allocate a fresh generation for a new binding handle (also used by snapshots).
pub fn allocate_generation(world: &mut World) -> u64 {
    let mut l = ledger(world);
    l.next_gen = l.next_gen.saturating_add(1);
    l.next_gen
}

fn revoke_generation(world: &mut World, gen: u64) {
    ledger(world).revoked.insert(gen);
}

fn is_revoked(world: &World, gen: u64) -> bool {
    world
        .get_resource::<BindingLedger>()
        .is_some_and(|l| l.revoked.contains(&gen))
}

/// Handle for [`MeterDefinition`] binding.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct MeterHandle {
    /// Asset id.
    pub asset_id: AssetId,
    /// Expected definition version.
    pub version: u32,
    /// Binding generation (must not be revoked).
    pub gen: u64,
}

/// Immutable meter definition (archive-friendly).
#[derive(Clone, Debug, rkyv::Archive, rkyv::Deserialize, rkyv::Serialize)]
#[rkyv(derive(Debug))]
pub struct MeterDefinition {
    id: AssetId,
    version: u32,
    pub min: f32,
    pub max: f32,
    pub default: f32,
}

impl MeterDefinition {
    /// Construct a meter definition with authoring metadata.
    pub fn new(id: AssetId, version: u32, min: f32, max: f32, default: f32) -> Self {
        Self {
            id,
            version,
            min,
            max,
            default,
        }
    }

    /// Mint a handle for binding (uses `allocate_generation`).
    pub fn mint_handle(&self, world: &mut World) -> MeterHandle {
        MeterHandle {
            asset_id: self.id,
            version: self.version,
            gen: allocate_generation(world),
        }
    }
}

impl DefinitionAsset for MeterDefinition {
    type Component = Meter;
    type Handle = MeterHandle;

    fn id(&self) -> AssetId {
        self.id
    }

    fn version(&self) -> u32 {
        self.version
    }

    fn bind(
        &self,
        world: &mut World,
        entity: Entity,
        handle: MeterHandle,
    ) -> Result<(), BindError> {
        if handle.asset_id != self.id {
            return Err(BindError::InvalidHandle);
        }
        if handle.version != self.version {
            return Err(BindError::VersionMismatch {
                expected: self.version,
                actual: handle.version,
            });
        }
        if is_revoked(world, handle.gen) {
            return Err(BindError::InvalidHandle);
        }
        if let Some(existing) = world.entity(entity).get::<Meter>() {
            if existing.binding_gen == handle.gen
                && (existing.value - self.default).abs() < f32::EPSILON
                && (existing.min - self.min).abs() < f32::EPSILON
                && (existing.max - self.max).abs() < f32::EPSILON
            {
                return Ok(());
            }
        }
        world.entity_mut(entity).insert(Meter {
            binding_gen: handle.gen,
            value: self.default,
            min: self.min,
            max: self.max,
        });
        Ok(())
    }

    fn unbind(&self, world: &mut World, entity: Entity) -> Result<(), BindError> {
        let Some(m) = world.entity(entity).get::<Meter>().map(|m| m.binding_gen) else {
            return Err(BindError::MissingComponent("Meter"));
        };
        revoke_generation(world, m);
        world.entity_mut(entity).remove::<Meter>();
        Ok(())
    }
}

/// Handle for [`AttributeSetDefinition`].
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct AttributeSetHandle {
    pub asset_id: AssetId,
    pub version: u32,
    pub gen: u64,
}

/// Attribute set definition.
#[derive(Clone, Debug, rkyv::Archive, rkyv::Deserialize, rkyv::Serialize)]
#[rkyv(derive(Debug))]
pub struct AttributeSetDefinition {
    id: AssetId,
    version: u32,
    pub defaults: Vec<(String, f32)>,
}

impl AttributeSetDefinition {
    /// New attribute set definition.
    pub fn new(id: AssetId, version: u32, defaults: Vec<(String, f32)>) -> Self {
        Self {
            id,
            version,
            defaults,
        }
    }

    /// Mint binding handle.
    pub fn mint_handle(&self, world: &mut World) -> AttributeSetHandle {
        AttributeSetHandle {
            asset_id: self.id,
            version: self.version,
            gen: allocate_generation(world),
        }
    }
}

impl DefinitionAsset for AttributeSetDefinition {
    type Component = AttributeSet;
    type Handle = AttributeSetHandle;

    fn id(&self) -> AssetId {
        self.id
    }

    fn version(&self) -> u32 {
        self.version
    }

    fn bind(
        &self,
        world: &mut World,
        entity: Entity,
        handle: AttributeSetHandle,
    ) -> Result<(), BindError> {
        if handle.asset_id != self.id {
            return Err(BindError::InvalidHandle);
        }
        if handle.version != self.version {
            return Err(BindError::VersionMismatch {
                expected: self.version,
                actual: handle.version,
            });
        }
        if is_revoked(world, handle.gen) {
            return Err(BindError::InvalidHandle);
        }
        let mut map = std::collections::BTreeMap::new();
        for (k, v) in &self.defaults {
            map.insert(k.clone(), *v);
        }
        if let Some(existing) = world.entity(entity).get::<AttributeSet>() {
            if existing.binding_gen == handle.gen && existing.values == map {
                return Ok(());
            }
        }
        world.entity_mut(entity).insert(AttributeSet {
            binding_gen: handle.gen,
            values: map,
        });
        Ok(())
    }

    fn unbind(&self, world: &mut World, entity: Entity) -> Result<(), BindError> {
        let Some(g) = world
            .entity(entity)
            .get::<AttributeSet>()
            .map(|a| a.binding_gen)
        else {
            return Err(BindError::MissingComponent("AttributeSet"));
        };
        revoke_generation(world, g);
        world.entity_mut(entity).remove::<AttributeSet>();
        Ok(())
    }
}

/// Handle for [`ContainerDefinition`].
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ContainerHandle {
    pub asset_id: AssetId,
    pub version: u32,
    pub gen: u64,
}

/// Container definition.
#[derive(Clone, Debug, rkyv::Archive, rkyv::Deserialize, rkyv::Serialize)]
#[rkyv(derive(Debug))]
pub struct ContainerDefinition {
    id: AssetId,
    version: u32,
    pub rows: u32,
    pub cols: u32,
    pub capacity: u32,
}

impl ContainerDefinition {
    /// New container definition (grid layout).
    pub fn new(id: AssetId, version: u32, rows: u32, cols: u32, capacity: u32) -> Self {
        Self {
            id,
            version,
            rows,
            cols,
            capacity,
        }
    }

    /// Mint binding handle.
    pub fn mint_handle(&self, world: &mut World) -> ContainerHandle {
        ContainerHandle {
            asset_id: self.id,
            version: self.version,
            gen: allocate_generation(world),
        }
    }
}

impl DefinitionAsset for ContainerDefinition {
    type Component = Container;
    type Handle = ContainerHandle;

    fn id(&self) -> AssetId {
        self.id
    }

    fn version(&self) -> u32 {
        self.version
    }

    fn bind(
        &self,
        world: &mut World,
        entity: Entity,
        handle: ContainerHandle,
    ) -> Result<(), BindError> {
        if handle.asset_id != self.id {
            return Err(BindError::InvalidHandle);
        }
        if handle.version != self.version {
            return Err(BindError::VersionMismatch {
                expected: self.version,
                actual: handle.version,
            });
        }
        if is_revoked(world, handle.gen) {
            return Err(BindError::InvalidHandle);
        }
        let layout = ContainerLayout::Grid {
            rows: self.rows,
            cols: self.cols,
        };
        let slots = vec![None; self.capacity as usize];
        let c = Container {
            binding_gen: handle.gen,
            layout,
            capacity: self.capacity,
            slots,
        };
        if let Some(existing) = world.entity(entity).get::<Container>() {
            if existing.binding_gen == handle.gen && *existing == c {
                return Ok(());
            }
        }
        world.entity_mut(entity).insert(c);
        Ok(())
    }

    fn unbind(&self, world: &mut World, entity: Entity) -> Result<(), BindError> {
        let Some(g) = world
            .entity(entity)
            .get::<Container>()
            .map(|c| c.binding_gen)
        else {
            return Err(BindError::MissingComponent("Container"));
        };
        revoke_generation(world, g);
        world.entity_mut(entity).remove::<Container>();
        Ok(())
    }
}

/// Handle for [`DirectedGraphDefinition`].
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct GraphHandle {
    pub asset_id: AssetId,
    pub version: u32,
    pub gen: u64,
}

/// Directed graph definition (topology counts only for this layer).
#[derive(Clone, Debug, rkyv::Archive, rkyv::Deserialize, rkyv::Serialize)]
#[rkyv(derive(Debug))]
pub struct DirectedGraphDefinition {
    id: AssetId,
    version: u32,
    pub nodes: u32,
    pub edges: u32,
}

impl DirectedGraphDefinition {
    /// New graph definition.
    pub fn new(id: AssetId, version: u32, nodes: u32, edges: u32) -> Self {
        Self {
            id,
            version,
            nodes,
            edges,
        }
    }

    /// Mint binding handle.
    pub fn mint_handle(&self, world: &mut World) -> GraphHandle {
        GraphHandle {
            asset_id: self.id,
            version: self.version,
            gen: allocate_generation(world),
        }
    }
}

impl DefinitionAsset for DirectedGraphDefinition {
    type Component = DirectedGraphInstance;
    type Handle = GraphHandle;

    fn id(&self) -> AssetId {
        self.id
    }

    fn version(&self) -> u32 {
        self.version
    }

    fn bind(
        &self,
        world: &mut World,
        entity: Entity,
        handle: GraphHandle,
    ) -> Result<(), BindError> {
        if handle.asset_id != self.id {
            return Err(BindError::InvalidHandle);
        }
        if handle.version != self.version {
            return Err(BindError::VersionMismatch {
                expected: self.version,
                actual: handle.version,
            });
        }
        if is_revoked(world, handle.gen) {
            return Err(BindError::InvalidHandle);
        }
        let g = DirectedGraphInstance {
            binding_gen: handle.gen,
            nodes: self.nodes,
            edges: self.edges,
            current_node: 0,
        };
        if let Some(existing) = world.entity(entity).get::<DirectedGraphInstance>() {
            if existing.binding_gen == handle.gen && *existing == g {
                return Ok(());
            }
        }
        world.entity_mut(entity).insert(g);
        Ok(())
    }

    fn unbind(&self, world: &mut World, entity: Entity) -> Result<(), BindError> {
        let Some(gen) = world
            .entity(entity)
            .get::<DirectedGraphInstance>()
            .map(|g| g.binding_gen)
        else {
            return Err(BindError::MissingComponent("DirectedGraphInstance"));
        };
        revoke_generation(world, gen);
        world.entity_mut(entity).remove::<DirectedGraphInstance>();
        Ok(())
    }
}
