//! Interest management backed by the networking grid (not the shared BVH).

use crate::ids::ConnectionId;
use crate::ids::Entity;

/// Networking-owned uniform grid used for AOI queries (never the gameplay BVH).
#[derive(Clone, Debug)]
pub struct NetworkGrid {
    cell_size: f32,
}

impl NetworkGrid {
    /// Creates a grid with the given cell size in world units.
    #[must_use]
    pub const fn new(cell_size: f32) -> Self {
        Self { cell_size }
    }

    /// Exposes cell size for tests asserting grid construction.
    #[must_use]
    pub const fn cell_size(&self) -> f32 {
        self.cell_size
    }
}

/// Result set of entities relevant to a client.
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct RelevantSet {
    entities: Vec<Entity>,
}

impl RelevantSet {
    /// Creates an empty set.
    #[must_use]
    pub fn new() -> Self {
        Self {
            entities: Vec::new(),
        }
    }

    /// Iterator over relevant entities.
    pub fn iter(&self) -> impl Iterator<Item = Entity> + '_ {
        self.entities.iter().copied()
    }

    /// Returns whether `entity` is present.
    #[must_use]
    pub fn contains(&self, entity: Entity) -> bool {
        self.entities.binary_search(&entity).is_ok()
    }

    fn insert_sorted(&mut self, entity: Entity) {
        match self.entities.binary_search(&entity) {
            Ok(_) => {}
            Err(idx) => self.entities.insert(idx, entity),
        }
    }
}

/// Minimal world surface for interest tests (positions in XZ plane, Y optional).
pub trait ReplicationWorld {
    /// Client camera / pawn position, if known.
    fn client_position(&self, client: ConnectionId) -> Option<(f32, f32, f32)>;

    /// Entity world position.
    fn entity_position(&self, entity: Entity) -> Option<(f32, f32, f32)>;

    /// Candidate entities considered for AOI.
    fn candidate_entities(&self) -> &[Entity];
}

/// Spatial interest manager using [`NetworkGrid`] (axis-agnostic stub uses Euclidean distance).
#[derive(Debug)]
pub struct InterestManager {
    aoi_radius: f32,
    grid: NetworkGrid,
}

impl InterestManager {
    /// Creates a manager with AOI radius and an explicit networking grid.
    #[must_use]
    pub fn new(aoi_radius: f32, grid: NetworkGrid) -> Self {
        Self { aoi_radius, grid }
    }

    /// Borrows the networking grid (TC-IR-4.4.3.3 asserts this is not a BVH type).
    #[must_use]
    pub const fn grid(&self) -> &NetworkGrid {
        &self.grid
    }

    /// Evaluates relevancy for `client` against `world`.
    #[must_use]
    pub fn evaluate<W: ReplicationWorld>(&self, client: ConnectionId, world: &W) -> RelevantSet {
        let Some((cx, cy, cz)) = world.client_position(client) else {
            return RelevantSet::new();
        };
        let mut set = RelevantSet::new();
        let r2 = self.aoi_radius * self.aoi_radius;
        for &entity in world.candidate_entities() {
            let Some((ex, ey, ez)) = world.entity_position(entity) else {
                continue;
            };
            let dx = ex - cx;
            let dy = ey - cy;
            let dz = ez - cz;
            if dx * dx + dy * dy + dz * dz <= r2 {
                set.insert_sorted(entity);
            }
        }
        let _ = self.grid.cell_size(); // future: drive neighbor iteration
        set
    }
}

#[cfg(test)]
mod test_support {
    use super::{ConnectionId, Entity, ReplicationWorld};

    /// Test-only world bag for integration tests.
    #[derive(Debug)]
    pub struct TestWorld {
        client_positions: Vec<(ConnectionId, (f32, f32, f32))>,
        entity_positions: Vec<(Entity, (f32, f32, f32))>,
        candidates: Vec<Entity>,
    }

    impl TestWorld {
        /// Empty world.
        #[must_use]
        pub fn new() -> Self {
            Self {
                client_positions: Vec::new(),
                entity_positions: Vec::new(),
                candidates: Vec::new(),
            }
        }

        /// Registers a client position.
        pub fn set_client_pos(&mut self, client: ConnectionId, pos: (f32, f32, f32)) {
            if let Some(row) = self.client_positions.iter_mut().find(|(c, _)| *c == client) {
                row.1 = pos;
                return;
            }
            self.client_positions.push((client, pos));
        }

        /// Registers an entity position.
        pub fn set_entity_pos(&mut self, entity: Entity, pos: (f32, f32, f32)) {
            if let Some(row) = self.entity_positions.iter_mut().find(|(e, _)| *e == entity) {
                row.1 = pos;
                return;
            }
            self.entity_positions.push((entity, pos));
        }

        /// Registers a candidate entity id.
        pub fn add_candidate(&mut self, entity: Entity) {
            if !self.candidates.contains(&entity) {
                self.candidates.push(entity);
            }
        }
    }

    impl ReplicationWorld for TestWorld {
        fn client_position(&self, client: ConnectionId) -> Option<(f32, f32, f32)> {
            self.client_positions
                .iter()
                .find(|(c, _)| *c == client)
                .map(|(_, p)| *p)
        }

        fn entity_position(&self, entity: Entity) -> Option<(f32, f32, f32)> {
            self.entity_positions
                .iter()
                .find(|(e, _)| *e == entity)
                .map(|(_, p)| *p)
        }

        fn candidate_entities(&self) -> &[Entity] {
            &self.candidates
        }
    }

    pub fn filter_always_true(_world: &TestWorld, _entity: Entity, _client: ConnectionId) -> bool {
        true
    }

    pub fn filter_always_false(_world: &TestWorld, _entity: Entity, _client: ConnectionId) -> bool {
        false
    }
}

#[cfg(test)]
mod tests {
    use super::test_support::{filter_always_false, filter_always_true, TestWorld};
    use super::*;
    use crate::filters::ReplicationFilterTable;
    use crate::wire::ReplicationFilterId;

    #[test]
    fn tc_ir_4_4_3_1_nearby_entity_relevant() {
        let grid = NetworkGrid::new(1.0);
        let mgr = InterestManager::new(10.0, grid);
        let mut world = TestWorld::new();
        let client = ConnectionId(1);
        let entity = Entity(2);
        world.set_client_pos(client, (0.0, 0.0, 0.0));
        world.set_entity_pos(entity, (1.0, 0.0, 0.0));
        world.add_candidate(entity);
        let set = mgr.evaluate(client, &world);
        assert!(set.contains(entity));
    }

    #[test]
    fn tc_ir_4_4_3_2_distant_filtered() {
        let grid = NetworkGrid::new(1.0);
        let mgr = InterestManager::new(0.5, grid);
        let mut world = TestWorld::new();
        let client = ConnectionId(1);
        let entity = Entity(3);
        world.set_client_pos(client, (0.0, 0.0, 0.0));
        world.set_entity_pos(entity, (100.0, 0.0, 0.0));
        world.add_candidate(entity);
        let set = mgr.evaluate(client, &world);
        assert!(!set.contains(entity));
    }

    #[test]
    fn tc_ir_4_4_3_3_grid_is_network_grid() {
        let grid = NetworkGrid::new(2.0);
        let mgr = InterestManager::new(5.0, grid);
        // Compile-time assertion: field type is `NetworkGrid`, not a BVH stub.
        let _: &NetworkGrid = mgr.grid();
    }

    #[test]
    fn tc_ir_4_4_3_n1_unknown_client_position_returns_empty() {
        let grid = NetworkGrid::new(1.0);
        let mgr = InterestManager::new(50.0, grid);
        let world = TestWorld::new();
        let set = mgr.evaluate(ConnectionId(7), &world);
        assert_eq!(set.iter().count(), 0);
    }

    #[test]
    fn tc_ir_4_4_8_4_custom_filter_fn_table() {
        static TABLE: [fn(&TestWorld, Entity, ConnectionId) -> bool; 2] =
            [filter_always_true, filter_always_false];
        let table = ReplicationFilterTable::new(&TABLE);
        let call0 = table.get(ReplicationFilterId(0)).expect("fn0")(
            &TestWorld::new(),
            Entity(0),
            ConnectionId(0),
        );
        let call1 = table.get(ReplicationFilterId(1)).expect("fn1")(
            &TestWorld::new(),
            Entity(0),
            ConnectionId(0),
        );
        assert!(call0);
        assert!(!call1);
        assert_eq!(table.filter_count(), 2);
    }
}
