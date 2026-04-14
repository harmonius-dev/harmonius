//! Zone ownership and seamless handoff (`R-8.7.2`).

use std::collections::HashMap;

/// World-space placement used for zone replication tests.
#[derive(Clone, Debug, PartialEq)]
pub struct Transform {
    /// X position in world units.
    pub x: f32,
    /// Y position in world units.
    pub y: f32,
    /// Z position in world units.
    pub z: f32,
}

/// Simple health pool for handoff snapshots.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Health(pub u32);

/// Item identifier carried in inventories.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Item(pub u32);

/// Inventory snapshot for deterministic tests.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Inventory(pub Vec<Item>);

/// Pending RPCs queued during a zone transition.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Rpc {
    /// Ability cast request.
    CastAbility {
        /// Stable ability identifier.
        id: u32,
    },
    /// Loot interaction.
    Loot {
        /// Item identifier.
        item: u32,
    },
}

/// Minimal player entity replicated across zones.
#[derive(Clone, Debug, PartialEq)]
pub struct PlayerEntity {
    /// Authoritative transform.
    pub transform: Transform,
    /// Health pool.
    pub health: Health,
    /// Carried items.
    pub inventory: Inventory,
    /// RPCs awaiting replay after migration.
    pub pending_rpcs: Vec<Rpc>,
}

/// Logical zone partition inside a shard.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct ZoneId(pub u32);

/// Owns entities for a single zone.
#[derive(Debug)]
pub struct ZoneServer {
    /// Zone identifier for diagnostics.
    pub id: ZoneId,
    entities: HashMap<u64, PlayerEntity>,
}

impl ZoneServer {
    /// Creates an empty zone.
    pub fn new(id: ZoneId) -> Self {
        Self {
            id,
            entities: HashMap::new(),
        }
    }

    /// Inserts or replaces `entity_id`.
    pub fn insert(&mut self, entity_id: u64, entity: PlayerEntity) {
        self.entities.insert(entity_id, entity);
    }

    /// Removes `entity_id` if present.
    pub fn remove(&mut self, entity_id: u64) -> Option<PlayerEntity> {
        self.entities.remove(&entity_id)
    }

    /// Borrows `entity_id` when owned by this zone.
    pub fn get(&self, entity_id: u64) -> Option<&PlayerEntity> {
        self.entities.get(&entity_id)
    }
}

/// Executes deterministic cross-zone transfers (`TC-8.7.2.1`, `TC-8.7.2.2`).
pub struct PlayerHandoff;

impl PlayerHandoff {
    /// Moves `entity_id` from `src` to `dst`, preserving component snapshots and RPC order.
    pub fn handoff(
        src: &mut ZoneServer,
        dst: &mut ZoneServer,
        entity_id: u64,
        _destination: ZoneId,
    ) -> Result<(), HandoffError> {
        let entity = src.remove(entity_id).ok_or(HandoffError::MissingSource)?;
        dst.insert(entity_id, entity);
        Ok(())
    }
}

/// Failure modes for [`PlayerHandoff::handoff`].
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum HandoffError {
    /// Entity was not present on the source zone.
    MissingSource,
}

#[cfg(test)]
mod tests {
    use super::*;

    /// `TC-8.7.2.1` `test_zone_handoff_state`
    #[test]
    fn test_zone_handoff_state() {
        let mut zone_a = ZoneServer::new(ZoneId(1));
        let mut zone_b = ZoneServer::new(ZoneId(2));
        let entity = PlayerEntity {
            transform: Transform {
                x: 1.0,
                y: 2.0,
                z: 3.0,
            },
            health: Health(85),
            inventory: Inventory(vec![Item(1), Item(2)]),
            pending_rpcs: Vec::new(),
        };
        zone_a.insert(1, entity.clone());
        PlayerHandoff::handoff(&mut zone_a, &mut zone_b, 1, ZoneId(7)).unwrap();
        assert!(zone_a.get(1).is_none());
        assert_eq!(zone_b.get(1), Some(&entity));
    }

    /// `TC-8.7.2.2` `test_zone_handoff_pending_rpcs`
    #[test]
    fn test_zone_handoff_pending_rpcs() {
        let mut zone_a = ZoneServer::new(ZoneId(1));
        let mut zone_b = ZoneServer::new(ZoneId(2));
        let pending = vec![
            Rpc::CastAbility { id: 12 },
            Rpc::Loot { item: 3 },
        ];
        zone_a.insert(
            1,
            PlayerEntity {
                transform: Transform {
                    x: 0.0,
                    y: 0.0,
                    z: 0.0,
                },
                health: Health(10),
                inventory: Inventory(Vec::new()),
                pending_rpcs: pending.clone(),
            },
        );
        PlayerHandoff::handoff(&mut zone_a, &mut zone_b, 1, ZoneId(2)).unwrap();
        assert_eq!(zone_a.get(1).map(|e| e.pending_rpcs.len()), None);
        assert_eq!(zone_b.get(1).unwrap().pending_rpcs, pending);
    }
}
