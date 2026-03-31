# R-13.27 -- Block-Based Voxel Requirements

## Block Data and Interaction

1. **R-13.27.1** -- The engine **SHALL** provide a data-driven block type registry with O(1) lookup
   by ID supporting properties including visual mesh, collision mode, hardness, tool requirement,
   drop table, light emission, and logic graph behavior references.
   - **Rationale:** A registry with constant-time lookup is essential for large voxel worlds with
     hundreds of block types.
   - **Verification:** Register 256 block types. Verify O(1) lookup by ID. Verify a block with a
     pickaxe requirement rejects an axe.

2. **R-13.27.2** -- The engine **SHALL** validate block placement against collision, slope,
   adjacency, and restriction zone rules, and block modifications in multiplayer **SHALL** be
   server-authoritative.
   - **Rationale:** Server authority prevents cheating; validation rules ensure placement
     consistency.
   - **Verification:** Attempt placement inside a player volume and verify rejection. Modify a block
     from a client and verify the server authorizes or rejects the change.

## Chunk Storage and Meshing

1. **R-13.27.3** -- The engine **SHALL** store block data in fixed-size chunks with palette
   compression for diverse chunks and single-value storage for uniform chunks, with configurable
   render distance per platform.
   - **Rationale:** Palette compression reduces memory; per-platform render distance ensures
     scalability.
   - **Verification:** Create a chunk with 4 block types and verify 4-bit palette storage. Create a
     uniform chunk and verify single- value storage. Verify chunks beyond render distance are
     unloaded.

2. **R-13.27.4** -- The engine **SHALL** convert chunk data into meshes using greedy meshing with
   internal face culling, per-vertex ambient occlusion, separate transparent passes, and incremental
   re-meshing on worker threads.
   - **Rationale:** Greedy meshing minimizes polygon count; incremental updates avoid full-chunk
     rebuilds.
   - **Verification:** Modify one block and verify only the affected chunk and neighbors re-mesh.
     Verify transparent blocks render with correct draw ordering.

## Block Lighting

1. **R-13.27.5** -- The engine **SHALL** propagate light via BFS flood fill across two channels
   (sunlight and block light) with 4-bit per-channel values and incremental updates when blocks
   change.
   - **Rationale:** Incremental BFS bounds update cost to affected blocks rather than recalculating
     entire chunks.
   - **Verification:** Place a light source and verify correct propagation. Remove a block between a
     light and a target and verify the target's light value updates incrementally.

## Block Physics

1. **R-13.27.6** -- The engine **SHALL** simulate gravity-affected blocks and fluid flow at a
   configurable tick rate with deterministic results, and fluid-block interaction rules **SHALL** be
   data-driven per pair.
   - **Rationale:** Deterministic physics ensures multiplayer consistency; data-driven rules allow
     designer customization.
   - **Verification:** Run gravity and fluid simulation on two clients and verify identical block
     state. Configure a custom fluid-block rule and verify it fires on contact.

## Block Circuits

1. **R-13.27.7** -- The engine **SHALL** evaluate block circuit networks deterministically with
   incremental signal propagation, distance attenuation, and per-chunk complexity budgets.
   - **Rationale:** Deterministic evaluation ensures identical behavior across clients; budgets
     prevent CPU spikes.
   - **Verification:** Build a circuit spanning two chunks and verify deterministic output. Exceed
     the per-chunk budget and verify excess components are depowered with a warning.

## Block World Generation

1. **R-13.27.8** -- The engine **SHALL** generate block terrain from a deterministic seed using
   heightmap noise, 3D cave noise, biome selection, ore placement, and structure templates on worker
   threads prioritized by player distance.
   - **Rationale:** Deterministic seeding enables seed sharing; worker thread prioritization ensures
     nearby terrain generates first.
   - **Verification:** Generate terrain from the same seed on two platforms and verify identical
     block data. Verify structures generate after terrain to avoid floating placements.
