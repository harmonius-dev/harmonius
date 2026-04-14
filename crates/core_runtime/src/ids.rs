//! Engine ID taxonomy, [`StableId`], and [`NetworkEntityMap`].
//!
//! See `docs/design/core-runtime/ids.md`.

use std::hash::Hasher;

use rkyv::{Archive, Deserialize as RkyvDeserialize, Serialize as RkyvSerialize};
use twox_hash::XxHash3_64;
use uuid::Uuid;

use crate::sorted_vec_map::SortedVecMap;

/// Failure decoding a persisted [`StableId`] payload.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IdError {
    /// Trailing bytes in the 16-byte wire form were not zero where required.
    CorruptedPayload,
    /// Namespace or wire layout did not match the requested type.
    InvalidNamespace,
}

/// Every ID that crosses a persistence boundary implements this trait.
/// Ephemeral IDs ([`Entity`], session-only newtypes) do **not** implement it.
pub trait StableId: Copy + Eq + Ord + core::hash::Hash {
    /// Owning subsystem grouping for collision avoidance.
    fn namespace(&self) -> IdNamespace;

    /// Declared stability contract for this ID.
    fn stability(&self) -> StabilityLevel;

    /// Canonical 16-byte little-endian wire representation.
    fn to_bytes(&self) -> [u8; 16];

    /// Parses [`StableId::to_bytes`] output.
    fn from_bytes(bytes: [u8; 16]) -> Result<Self, IdError>
    where
        Self: Sized;
}

/// Coarse namespace tags for stable ID hashing and tooling.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum IdNamespace {
    /// Baked assets and streaming content hashes.
    Asset,
    /// Authoring-time content definitions (tables, graphs).
    Content,
    /// Editor-only identifiers.
    Editor,
    /// Network transport and replication.
    Network,
    /// Save files and player profiles.
    Save,
    /// Live ECS world indices and handles.
    World,
}

/// Declared lifetime of an identifier across engine boundaries.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum StabilityLevel {
    /// Single frame or immediate callback scope.
    Ephemeral,
    /// Hot-reload of editor/gameplay modules preserves these IDs.
    HotReload,
    /// Matches across all clients in a networked session.
    Network,
    /// Survives save and load cycles.
    Save,
    /// Valid for the running process only.
    Session,
}

/// Untyped generational slot reference; see `docs/design/core-runtime/primitives.md`.
#[derive(
    Archive,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
    RkyvDeserialize,
    RkyvSerialize,
)]
#[rkyv(compare(PartialEq, PartialOrd))]
pub struct GenerationalIndex {
    /// Dense slot index.
    pub index: u32,
    /// Generation counter guarding reuse.
    pub generation: u32,
}

impl GenerationalIndex {
    /// Constructs a generational index from parts.
    pub const fn new(index: u32, generation: u32) -> Self {
        Self { index, generation }
    }
}

/// Session-local ECS entity handle; **not** [`StableId`].
#[derive(
    Archive,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
    RkyvDeserialize,
    RkyvSerialize,
)]
#[rkyv(compare(PartialEq, PartialOrd))]
pub struct Entity(pub GenerationalIndex);

impl Entity {
    /// Slot index for debugging and maps.
    pub fn index(self) -> u32 {
        self.0.index
    }

    /// Generation counter for this slot.
    pub fn generation(self) -> u32 {
        self.0.generation
    }
}

/// Server-assigned, network-stable entity identifier (distinct from [`Entity`]).
#[derive(
    Archive,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
    RkyvDeserialize,
    RkyvSerialize,
)]
#[rkyv(compare(PartialEq, PartialOrd))]
pub struct NetworkEntityId(pub u32);

impl StableId for NetworkEntityId {
    fn namespace(&self) -> IdNamespace {
        IdNamespace::Network
    }

    fn stability(&self) -> StabilityLevel {
        StabilityLevel::Network
    }

    fn to_bytes(&self) -> [u8; 16] {
        let mut out = [0u8; 16];
        out[..4].copy_from_slice(&self.0.to_le_bytes());
        out
    }

    fn from_bytes(bytes: [u8; 16]) -> Result<Self, IdError> {
        if !bytes[4..].iter().all(|b| *b == 0) {
            return Err(IdError::CorruptedPayload);
        }
        Ok(Self(u32::from_le_bytes(bytes[..4].try_into().unwrap())))
    }
}

/// Maps server [`NetworkEntityId`] values to client-local [`Entity`] handles.
#[derive(Debug, Default, Clone)]
pub struct NetworkEntityMap {
    server_to_local: SortedVecMap<NetworkEntityId, Entity>,
    local_to_server: SortedVecMap<Entity, NetworkEntityId>,
}

impl NetworkEntityMap {
    /// Empty map.
    pub fn new() -> Self {
        Self {
            server_to_local: SortedVecMap::new(),
            local_to_server: SortedVecMap::new(),
        }
    }

    /// Records a binding in both directions, replacing any prior mapping for the same keys.
    pub fn bind(&mut self, net: NetworkEntityId, local: Entity) {
        if let Some(prev_local) = self.server_to_local.remove(&net) {
            let _ = self.local_to_server.remove(&prev_local);
        }
        if let Some(prev_net) = self.local_to_server.remove(&local) {
            let _ = self.server_to_local.remove(&prev_net);
        }
        let _ = self.server_to_local.insert(net, local);
        let _ = self.local_to_server.insert(local, net);
    }

    /// Resolves a server id to the local [`Entity`], if known.
    pub fn resolve(&self, net: NetworkEntityId) -> Option<Entity> {
        self.server_to_local.get(&net).copied()
    }

    /// Reverse lookup from local [`Entity`] to server id.
    pub fn reverse(&self, local: Entity) -> Option<NetworkEntityId> {
        self.local_to_server.get(&local).copied()
    }

    /// Drops a server id mapping, if present.
    pub fn unbind(&mut self, net: NetworkEntityId) {
        if let Some(local) = self.server_to_local.remove(&net) {
            let _ = self.local_to_server.remove(&local);
        }
    }

    /// Number of paired mappings.
    pub fn len(&self) -> usize {
        self.server_to_local.len()
    }

    /// Returns `true` when no bindings are stored.
    pub fn is_empty(&self) -> bool {
        self.server_to_local.is_empty()
    }
}

/// Content hash of baked asset bytes; deterministic and save-stable.
#[derive(
    Archive,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
    RkyvDeserialize,
    RkyvSerialize,
)]
#[rkyv(compare(PartialEq, PartialOrd))]
pub struct AssetId(pub u64);

impl StableId for AssetId {
    fn namespace(&self) -> IdNamespace {
        IdNamespace::Asset
    }

    fn stability(&self) -> StabilityLevel {
        StabilityLevel::Save
    }

    fn to_bytes(&self) -> [u8; 16] {
        let mut out = [0u8; 16];
        out[..8].copy_from_slice(&self.0.to_le_bytes());
        out
    }

    fn from_bytes(bytes: [u8; 16]) -> Result<Self, IdError> {
        if !bytes[8..].iter().all(|b| *b == 0) {
            return Err(IdError::CorruptedPayload);
        }
        Ok(Self(u64::from_le_bytes(bytes[..8].try_into().unwrap())))
    }
}

/// Stable hash of a definition keyed by slug; survives benign content edits.
#[derive(
    Archive,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
    RkyvDeserialize,
    RkyvSerialize,
)]
#[rkyv(compare(PartialEq, PartialOrd))]
pub struct DefinitionId(pub u64);

impl StableId for DefinitionId {
    fn namespace(&self) -> IdNamespace {
        IdNamespace::Content
    }

    fn stability(&self) -> StabilityLevel {
        StabilityLevel::Save
    }

    fn to_bytes(&self) -> [u8; 16] {
        let mut out = [0u8; 16];
        out[..8].copy_from_slice(&self.0.to_le_bytes());
        out
    }

    fn from_bytes(bytes: [u8; 16]) -> Result<Self, IdError> {
        if !bytes[8..].iter().all(|b| *b == 0) {
            return Err(IdError::CorruptedPayload);
        }
        Ok(Self(u64::from_le_bytes(bytes[..8].try_into().unwrap())))
    }
}

/// Random save slot identifier (UUID v4 wire bytes).
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct SaveSlotId(pub Uuid);

impl StableId for SaveSlotId {
    fn namespace(&self) -> IdNamespace {
        IdNamespace::Save
    }

    fn stability(&self) -> StabilityLevel {
        StabilityLevel::Save
    }

    fn to_bytes(&self) -> [u8; 16] {
        *self.0.as_bytes()
    }

    fn from_bytes(bytes: [u8; 16]) -> Result<Self, IdError> {
        Ok(Self(Uuid::from_bytes(bytes)))
    }
}

/// Computes [`AssetId`] as XXH3-64 over `bytes` (fixed algorithm, LE output).
pub fn asset_id_from_bytes(bytes: &[u8]) -> AssetId {
    AssetId(XxHash3_64::oneshot(bytes))
}

/// Computes [`DefinitionId`] from a namespace-prefixed slug (slug-only; ignores other content).
pub fn definition_id_from_slug(slug: &str) -> DefinitionId {
    const PREFIX: &[u8] = b"harmonius:definition:v1:";
    let mut hasher = XxHash3_64::default();
    hasher.write(PREFIX);
    hasher.write(slug.as_bytes());
    DefinitionId(hasher.finish())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tc_1_10_1_1_stable_id_round_trip_asset() {
        let id = AssetId(0xCAFE_BABE);
        assert_eq!(AssetId::from_bytes(id.to_bytes()).unwrap(), id);
    }

    #[test]
    fn tc_1_10_1_1_stable_id_round_trip_definition() {
        let id = DefinitionId(0xDEAD_BEEF);
        assert_eq!(DefinitionId::from_bytes(id.to_bytes()).unwrap(), id);
    }

    #[test]
    fn tc_1_10_1_2_namespace_discrimination() {
        let asset = AssetId(1);
        let net = NetworkEntityId(2);
        assert_eq!(asset.namespace(), IdNamespace::Asset);
        assert_eq!(net.namespace(), IdNamespace::Network);
    }

    #[test]
    fn tc_1_10_3_1_network_entity_id_distinct_eq() {
        assert_eq!(NetworkEntityId(5), NetworkEntityId(5));
    }

    #[test]
    fn tc_1_10_3_2_network_entity_map_bind_resolve() {
        let local = Entity(GenerationalIndex::new(7, 1));
        let mut map = NetworkEntityMap::new();
        map.bind(NetworkEntityId(42), local);
        assert_eq!(map.resolve(NetworkEntityId(42)), Some(local));
        assert_eq!(map.reverse(local), Some(NetworkEntityId(42)));
    }

    #[test]
    fn tc_1_10_3_3_unknown_returns_none() {
        let map = NetworkEntityMap::new();
        assert_eq!(map.resolve(NetworkEntityId(999)), None);
    }

    #[test]
    fn tc_1_10_4_1_asset_id_content_hash_deterministic() {
        let bytes = [0x5Au8; 1024];
        assert_eq!(asset_id_from_bytes(&bytes), asset_id_from_bytes(&bytes));
    }

    #[test]
    fn tc_1_10_4_2_asset_id_rkyv_round_trip() {
        let id = AssetId(0x1234);
        let bytes = rkyv::to_bytes::<rkyv::rancor::Error>(&id).expect("serialize");
        let back = rkyv::from_bytes::<AssetId, rkyv::rancor::Error>(&bytes).expect("deserialize");
        assert_eq!(id, back);
    }

    #[test]
    fn tc_1_10_5_1_definition_id_stable_across_content_edit() {
        let slug = "quest.intro";
        assert_eq!(definition_id_from_slug(slug), definition_id_from_slug(slug));
    }

    #[test]
    fn tc_1_10_5_2_definition_id_changes_on_rename() {
        assert_ne!(
            definition_id_from_slug("quest.intro"),
            definition_id_from_slug("quest.prologue")
        );
    }

    #[test]
    fn tc_1_10_3_4_network_spawn_populates_map() {
        let mut map = NetworkEntityMap::new();
        for i in 0u32..100 {
            map.bind(NetworkEntityId(i), Entity(GenerationalIndex::new(i, 0)));
        }
        assert_eq!(map.len(), 100);
        for i in 0u32..100 {
            let e = map.resolve(NetworkEntityId(i)).expect("bound entity");
            assert_eq!(e.index(), i);
        }
    }

    #[derive(Archive, rkyv::Serialize, rkyv::Deserialize)]
    struct StableIdBundle {
        assets: Vec<AssetId>,
        defs: Vec<DefinitionId>,
    }

    #[test]
    fn tc_1_10_4_4_save_round_trip_preserves_stable_ids() {
        let assets: Vec<AssetId> = (0..1000u64)
            .map(|i| AssetId(i ^ 0xA5A5_A5A5_A5A5_A5A5))
            .collect();
        let defs: Vec<DefinitionId> = (0..1000u64)
            .map(|i| DefinitionId(i.rotate_left(7)))
            .collect();
        let bundle = StableIdBundle {
            assets: assets.clone(),
            defs: defs.clone(),
        };
        let bytes = rkyv::to_bytes::<rkyv::rancor::Error>(&bundle).expect("serialize bundle");
        let back: StableIdBundle =
            rkyv::from_bytes::<StableIdBundle, rkyv::rancor::Error>(&bytes).expect("deserialize");
        assert_eq!(back.assets, assets);
        assert_eq!(back.defs, defs);
    }

    #[test]
    fn tc_1_10_4_3_save_slot_uuid_v4() {
        let a = SaveSlotId(Uuid::new_v4());
        let b = SaveSlotId(Uuid::new_v4());
        assert_ne!(a, b);
        let bytes_a = a.0.as_bytes();
        assert_eq!(bytes_a[6] & 0xF0, 0x40, "RFC 4122 version nibble");
        assert_eq!(bytes_a[8] & 0xC0, 0x80, "RFC 4122 variant bits");
    }

    #[test]
    fn tc_1_10_3_5_network_despawn_unbind() {
        let local = Entity(GenerationalIndex::new(1, 0));
        let mut map = NetworkEntityMap::new();
        map.bind(NetworkEntityId(42), local);
        map.unbind(NetworkEntityId(42));
        assert_eq!(map.resolve(NetworkEntityId(42)), None);
        assert_eq!(map.reverse(local), None);
    }

    #[test]
    #[cfg(not(debug_assertions))]
    fn tc_1_10_3_6_network_entity_map_10k_lookups_under_1ms() {
        let mut map = NetworkEntityMap::new();
        for i in 0u32..10_000 {
            map.bind(NetworkEntityId(i), Entity(GenerationalIndex::new(i, 0)));
        }
        let start = std::time::Instant::now();
        for i in 0u32..10_000 {
            std::hint::black_box(map.resolve(NetworkEntityId(i)));
        }
        assert!(start.elapsed().as_secs_f64() < 0.001);
    }

    #[test]
    #[cfg(not(debug_assertions))]
    fn tc_1_10_4_5_asset_id_hash_1mib_under_5ms() {
        let buf = vec![0xABu8; 1024 * 1024];
        let start = std::time::Instant::now();
        std::hint::black_box(asset_id_from_bytes(&buf));
        assert!(start.elapsed().as_secs_f64() < 0.005);
    }

    #[test]
    #[cfg(not(debug_assertions))]
    fn tc_1_10_5_3_definition_id_10k_slugs_under_2ms() {
        let slugs: Vec<String> = (0..10_000).map(|i| format!("quest.line.{i}")).collect();
        let start = std::time::Instant::now();
        for s in &slugs {
            std::hint::black_box(definition_id_from_slug(s));
        }
        assert!(start.elapsed().as_secs_f64() < 0.002);
    }
}
