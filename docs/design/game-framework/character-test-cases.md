# Character Customization and Inventory Test Cases

Companion test cases for [character.md](character.md).

## Unit Tests

### TC-13.8.1.1 Facial Morph Additive

| # | Requirement |
|---|-------------|
| 1 | R-13.8.1    |

1. **#1** — Set morph on Eyes and Nose (overlapping verts)
   - **Expected:** Vertex positions reflect additive composition

### TC-13.8.1.2 Facial Symmetry Break

| # | Requirement |
|---|-------------|
| 1 | R-13.8.1    |

1. **#1** — Toggle symmetric=false on left-eye marker
   - **Expected:** Left/right sides diverge independently

### TC-13.8.2.1 Preset Blend 50/50

| # | Requirement |
|---|-------------|
| 1 | R-13.8.2    |

1. **#1** — Blend preset A and B at (0.5, 0.5)
   - **Expected:** Morph vector = arithmetic mean of A and B

### TC-13.8.2.2 Preset Save Load

| # | Requirement |
|---|-------------|
| 1 | R-13.8.2    |

1. **#1** — Save custom preset; reload
   - **Expected:** All weights restored within f32 epsilon

### TC-13.8.3.1 Body Shape Constraints

| # | Requirement |
|---|-------------|
| 1 | R-13.8.3    |

1. **#1** — Set height=2.0, waist=-0.5 (out of range)
   - **Expected:** Values clamped to valid [0.0, 1.0] range

### TC-13.8.4.1 Equipment Morph Propagation

| # | Requirement |
|---|-------------|
| 1 | R-13.8.4    |

1. **#1** — Extreme body shape + cloth equipment
   - **Expected:** No mesh penetration between body and equipment

### TC-13.8.4.2 Rigid Armor No Deform

| # | Requirement |
|---|-------------|
| 1 | R-13.8.4    |

1. **#1** — Equip plate armor; change body shape
   - **Expected:** Rigid pieces follow bone transforms only

### TC-13.8.5.1 Skin Params Deterministic

| # | Requirement |
|---|-------------|
| 1 | R-13.8.5    |

1. **#1** — Two characters with identical SkinParams
   - **Expected:** Same material output generated

### TC-13.8.6.1 Appearance Layer Order

| # | Requirement |
|---|-------------|
| 1 | R-13.8.6    |

1. **#1** — 3 layers: Foundation, Blush, Scar
   - **Expected:** Composited in order; blend modes applied

### TC-13.8.7.1 Eye Heterochromia

| # | Requirement |
|---|-------------|
| 1 | R-13.8.7    |

1. **#1** — Left iris=blue, right iris=green
   - **Expected:** Each eye material instance uses correct color

### TC-13.8.8.1 Hair Swap

| # | Requirement |
|---|-------------|
| 1 | R-13.8.8    |

1. **#1** — Swap hairstyle asset A to B
   - **Expected:** New groom renders with current HairParams

### TC-13.8.8.2 Hair Color Propagation

| # | Requirement |
|---|-------------|
| 1 | R-13.8.8    |

1. **#1** — Set primary_color=red; propagate=true; eyebrow_override=brown
   - **Expected:** Hair=red, eyebrows=brown, eyelashes=red

### TC-13.8.9.1 Mesh Parts Sync

| # | Requirement |
|---|-------------|
| 1 | R-13.8.9    |

1. **#1** — Assemble 6 mesh parts
   - **Expected:** All share skeleton transforms in sync

### TC-13.8.10.1 Socket Rigid Attach

| # | Requirement |
|---|-------------|
| 1 | R-13.8.10   |

1. **#1** — Attach weapon to HandRight socket
   - **Expected:** Weapon transform = hand bone transform + offset

### TC-13.8.10.2 Body Region Hiding

| # | Requirement |
|---|-------------|
| 1 | R-13.8.10   |

1. **#1** — Equip opaque chest armor (hides TorsoUpper)
   - **Expected:** Torso upper body mesh culled

### TC-13.8.11.1 Transmog Stats Visual

| # | Requirement |
|---|-------------|
| 1 | R-13.8.11   |

1. **#1** — Equip stat gear + appearance override
   - **Expected:** Stats from equipped item; visuals from override

### TC-13.8.11.2 Outfit Dye Persist

| # | Requirement |
|---|-------------|
| 1 | R-13.8.11   |

1. **#1** — Apply red dye; change outfit; revert
   - **Expected:** Red dye preserved on original outfit

### TC-13.8.12.1 Race Skeleton

| # | Requirement |
|---|-------------|
| 1 | R-13.8.12   |

1. **#1** — Spawn human and elf characters
   - **Expected:** Each uses own race skeleton asset

### TC-13.8.12.2 Race Equipment Filter

| # | Requirement |
|---|-------------|
| 1 | R-13.8.12   |

1. **#1** — Equip elf-only helm on human
   - **Expected:** Rejection; incompatible race

### TC-13.8.15.1 Appearance Serialize Size

| # | Requirement           |
|---|-----------------------|
| 1 | R-13.8.15, R-13.8.NF3 |

1. **#1** — Fully customized character; serialize
   - **Expected:** Binary data < 16 KB

### TC-13.8.15.2 Appearance Version Migrate

| # | Requirement |
|---|-------------|
| 1 | R-13.8.15   |

1. **#1** — Serialize v1; deserialize as v2 (new field)
   - **Expected:** Migration fills default for new field

### TC-13.9.1.1 Inventory ECS Hierarchy

| # | Requirement |
|---|-------------|
| 1 | R-13.9.1    |

1. **#1** — Create container entity; spawn 3 item children
   - **Expected:** Parent-child verified via ECS relationship query

### TC-13.9.1.2 Weight Capacity Reject

| # | Requirement |
|---|-------------|
| 1 | R-13.9.1    |

1. **#1** — Add items exceeding weight_capacity
   - **Expected:** Rejection; item not added

### TC-13.9.2.1 Grid Placement

| # | Requirement |
|---|-------------|
| 1 | R-13.9.2    |

1. **#1** — Place 2x3 item in 10x10 grid at (0,0)
   - **Expected:** 6 cells marked occupied

### TC-13.9.2.2 Grid Overlap Reject

| # | Requirement |
|---|-------------|
| 1 | R-13.9.2    |

1. **#1** — Place item at already-occupied cells
   - **Expected:** Rejection

### TC-13.9.3.1 Stack Merge

| # | Requirement |
|---|-------------|
| 1 | R-13.9.3    |

1. **#1** — Add 150 of max-100 item
   - **Expected:** Two stacks: 100 + 50

### TC-13.9.3.2 Stack Split

| # | Requirement |
|---|-------------|
| 1 | R-13.9.3    |

1. **#1** — Split 100-stack at 60
   - **Expected:** Two stacks: 60 + 40

### TC-13.9.4.1 Durability Query

| # | Requirement |
|---|-------------|
| 1 | R-13.9.4    |

1. **#1** — Two items: durability 5% and 50%; filter < 10%
   - **Expected:** Returns only 5% item

### TC-13.9.5.1 Socket Insert Merge

| # | Requirement |
|---|-------------|
| 1 | R-13.9.5    |

1. **#1** — Insert Rune (+10 dmg) into Rune socket
   - **Expected:** Item stat modifiers include +10 dmg

### TC-13.9.5.2 Socket Type Reject

| # | Requirement |
|---|-------------|
| 1 | R-13.9.5    |

1. **#1** — Insert Gem into Rune-only socket
   - **Expected:** Rejection; incompatible socket type

### TC-13.9.6.1 Transfer Reparent

| # | Requirement |
|---|-------------|
| 1 | R-13.9.6    |

1. **#1** — Transfer item from container A to B
   - **Expected:** Item parent entity changes to B

### TC-13.9.6.2 Transfer Prediction Rollback

| # | Requirement |
|---|-------------|
| 1 | R-13.9.6    |

1. **#1** — Client predicts transfer; server rejects
   - **Expected:** Client rolls back to pre-transfer state

### TC-13.9.7.1 Loot Round Robin

| # | Requirement |
|---|-------------|
| 1 | R-13.9.7    |

1. **#1** — Distribute 4 items to 4 players in round-robin
   - **Expected:** Each player receives 1 item in rotation

### TC-13.9.8.1 Merchant Buy

| # | Requirement |
|---|-------------|
| 1 | R-13.9.8    |

1. **#1** — Buy item for 100 gold; balance=500
   - **Expected:** Balance=400; item in inventory

### TC-13.9.9.1 Equip Stat Apply

| # | Requirement |
|---|-------------|
| 1 | R-13.9.9    |

1. **#1** — Equip weapon (+20 attack)
   - **Expected:** Attack stat increased by 20

### TC-13.9.9.2 Unequip Stat Revert

| # | Requirement |
|---|-------------|
| 1 | R-13.9.9    |

1. **#1** — Unequip weapon (+20 attack)
   - **Expected:** Attack stat reverts to previous value

### TC-13.9.10.1 Inventory Serialize Roundtrip

| # | Requirement |
|---|-------------|
| 1 | R-13.9.10   |

1. **#1** — Serialize inventory with socketed items; deserialize
   - **Expected:** All data preserved including sockets

## Integration Tests

### TC-13.9.9.I1 Equip Full Pipeline

| # | Requirement         |
|---|---------------------|
| 1 | R-13.9.9, R-13.8.10 |

1. **#1** — Equip item
   - **Expected:** Stat change + visual attachment + animation trigger + body region occlusion

### TC-13.8.NF1.I1 Character Creator Load

| # | Requirement |
|---|-------------|
| 1 | R-13.8.NF1  |

1. **#1** — Load character creator
   - **Expected:** Interactive within 3 seconds

### TC-13.8.NF2.I1 Morph Slider Latency

| # | Requirement |
|---|-------------|
| 1 | R-13.8.NF2  |

1. **#1** — Move facial slider continuously
   - **Expected:** Mesh updates within same frame

### TC-13.8.13.I1 500 Characters Frame Budget

| # | Requirement |
|---|-------------|
| 1 | R-13.8.13   |

1. **#1** — Render 500 unique characters at 1080p
   - **Expected:** Frame time < 16 ms

### TC-13.8.14.I1 Mesh Merge Async

| # | Requirement |
|---|-------------|
| 1 | R-13.8.14   |

1. **#1** — Merge 6 mesh parts
   - **Expected:** Async; no main thread block; 1 draw call

### TC-13.8.14.I2 Merge Cache Hit

| # | Requirement |
|---|-------------|
| 1 | R-13.8.14   |

1. **#1** — Repeat same part combination
   - **Expected:** Cache hit; no re-merge

### TC-13.9.NF1.I1 Inventory 500 Items

| # | Requirement |
|---|-------------|
| 1 | R-13.9.NF1  |

1. **#1** — 500 stacks; 1000 random operations
   - **Expected:** All operations under latency budget

### TC-13.9.NF2.I1 20 Containers Memory

| # | Requirement |
|---|-------------|
| 1 | R-13.9.NF2  |

1. **#1** — 20 containers x 100 items with properties
   - **Expected:** Total memory < 2 MB

### TC-13.9.NF3.I1 Inventory Ops Latency

| # | Requirement |
|---|-------------|
| 1 | R-13.9.NF3  |

1. **#1** — 10,000 random operations
   - **Expected:** p99 latency < 1 ms

## Benchmarks

### TC-13.8.NF2.B1 Morph Evaluation

| # | Scenario | Metric | Target | Requirement |
|---|----------|--------|--------|-------------|
| 1 | Full face morph evaluation | Wall-clock time | < 1 ms | R-13.8.NF2 |

### TC-13.8.NF2.B2 Body Shape Blend

| # | Scenario | Metric | Target | Requirement |
|---|----------|--------|--------|-------------|
| 1 | Body shape blend (9 params) | Wall-clock time | < 0.5 ms | R-13.8.NF2 |

### TC-13.8.14.B1 Mesh Merge

| # | Scenario | Metric | Target | Requirement |
|---|----------|--------|--------|-------------|
| 1 | Merge 6 mesh parts | Wall-clock time (async) | < 5 ms | R-13.8.14 |

### TC-13.8.NF3.B1 Appearance Serialization

| # | Scenario | Metric | Target | Requirement |
|---|----------|--------|--------|-------------|
| 1 | Serialize full appearance | Time and size | < 0.5 ms, < 16 KB | R-13.8.NF3 |

### TC-13.9.NF3.B1 Grid Auto Sort

| # | Scenario | Metric | Target | Requirement |
|---|----------|--------|--------|-------------|
| 1 | Auto-sort 200 items in grid | Wall-clock time | < 2 ms | R-13.9.NF3 |

### TC-13.9.NF3.B2 Stack Consolidate

| # | Scenario | Metric | Target | Requirement |
|---|----------|--------|--------|-------------|
| 1 | Consolidate 500 items | Wall-clock time | < 1 ms | R-13.9.NF3 |

### TC-13.9.NF3.B3 Equip Unequip Cycle

| # | Scenario | Metric | Target | Requirement |
|---|----------|--------|--------|-------------|
| 1 | Equip/unequip cycle (server) | Wall-clock time | < 0.5 ms | R-13.9.NF3 |

### TC-13.9.NF3.B4 Transfer Validation

| # | Scenario | Metric | Target | Requirement |
|---|----------|--------|--------|-------------|
| 1 | Validate transfer operation | Wall-clock time | < 0.2 ms | R-13.9.NF3 |

### TC-13.9.10.B1 Inventory Serialize

| # | Scenario | Metric | Target | Requirement |
|---|----------|--------|--------|-------------|
| 1 | Serialize 500-item inventory | Wall-clock time | < 5 ms | R-13.9.10 |

### TC-13.8.13.B1 LOD Evaluation

| # | Scenario | Metric | Target | Requirement |
|---|----------|--------|--------|-------------|
| 1 | LOD evaluation for 500 characters | Wall-clock time | < 1 ms | R-13.8.13 |
