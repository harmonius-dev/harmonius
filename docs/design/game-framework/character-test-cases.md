# Character Customization and Inventory Test Cases

Companion test cases for [character.md](character.md).

## Unit Tests

### TC-13.8.1.1 Facial Morph Additive

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Set morph on Eyes and Nose (overlapping verts) | Vertex positions reflect additive composition | R-13.8.1 |

### TC-13.8.1.2 Facial Symmetry Break

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Toggle symmetric=false on left-eye marker | Left/right sides diverge independently | R-13.8.1 |

### TC-13.8.2.1 Preset Blend 50/50

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Blend preset A and B at (0.5, 0.5) | Morph vector = arithmetic mean of A and B | R-13.8.2 |

### TC-13.8.2.2 Preset Save Load

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Save custom preset; reload | All weights restored within f32 epsilon | R-13.8.2 |

### TC-13.8.3.1 Body Shape Constraints

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Set height=2.0, waist=-0.5 (out of range) | Values clamped to valid [0.0, 1.0] range | R-13.8.3 |

### TC-13.8.4.1 Equipment Morph Propagation

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Extreme body shape + cloth equipment | No mesh penetration between body and equipment | R-13.8.4 |

### TC-13.8.4.2 Rigid Armor No Deform

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Equip plate armor; change body shape | Rigid pieces follow bone transforms only | R-13.8.4 |

### TC-13.8.5.1 Skin Params Deterministic

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Two characters with identical SkinParams | Same material output generated | R-13.8.5 |

### TC-13.8.6.1 Appearance Layer Order

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | 3 layers: Foundation, Blush, Scar | Composited in order; blend modes applied | R-13.8.6 |

### TC-13.8.7.1 Eye Heterochromia

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Left iris=blue, right iris=green | Each eye material instance uses correct color | R-13.8.7 |

### TC-13.8.8.1 Hair Swap

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Swap hairstyle asset A to B | New groom renders with current HairParams | R-13.8.8 |

### TC-13.8.8.2 Hair Color Propagation

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Set primary_color=red; propagate=true; eyebrow_override=brown | Hair=red, eyebrows=brown, eyelashes=red | R-13.8.8 |

### TC-13.8.9.1 Mesh Parts Sync

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Assemble 6 mesh parts | All share skeleton transforms in sync | R-13.8.9 |

### TC-13.8.10.1 Socket Rigid Attach

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Attach weapon to HandRight socket | Weapon transform = hand bone transform + offset | R-13.8.10 |

### TC-13.8.10.2 Body Region Hiding

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Equip opaque chest armor (hides TorsoUpper) | Torso upper body mesh culled | R-13.8.10 |

### TC-13.8.11.1 Transmog Stats Visual

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Equip stat gear + appearance override | Stats from equipped item; visuals from override | R-13.8.11 |

### TC-13.8.11.2 Outfit Dye Persist

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Apply red dye; change outfit; revert | Red dye preserved on original outfit | R-13.8.11 |

### TC-13.8.12.1 Race Skeleton

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Spawn human and elf characters | Each uses own race skeleton asset | R-13.8.12 |

### TC-13.8.12.2 Race Equipment Filter

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Equip elf-only helm on human | Rejection; incompatible race | R-13.8.12 |

### TC-13.8.15.1 Appearance Serialize Size

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Fully customized character; serialize | Binary data < 16 KB | R-13.8.15, R-13.8.NF3 |

### TC-13.8.15.2 Appearance Version Migrate

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Serialize v1; deserialize as v2 (new field) | Migration fills default for new field | R-13.8.15 |

### TC-13.9.1.1 Inventory ECS Hierarchy

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Create container entity; spawn 3 item children | Parent-child verified via ECS relationship query | R-13.9.1 |

### TC-13.9.1.2 Weight Capacity Reject

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Add items exceeding weight_capacity | Rejection; item not added | R-13.9.1 |

### TC-13.9.2.1 Grid Placement

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Place 2x3 item in 10x10 grid at (0,0) | 6 cells marked occupied | R-13.9.2 |

### TC-13.9.2.2 Grid Overlap Reject

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Place item at already-occupied cells | Rejection | R-13.9.2 |

### TC-13.9.3.1 Stack Merge

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Add 150 of max-100 item | Two stacks: 100 + 50 | R-13.9.3 |

### TC-13.9.3.2 Stack Split

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Split 100-stack at 60 | Two stacks: 60 + 40 | R-13.9.3 |

### TC-13.9.4.1 Durability Query

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Two items: durability 5% and 50%; filter < 10% | Returns only 5% item | R-13.9.4 |

### TC-13.9.5.1 Socket Insert Merge

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Insert Rune (+10 dmg) into Rune socket | Item stat modifiers include +10 dmg | R-13.9.5 |

### TC-13.9.5.2 Socket Type Reject

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Insert Gem into Rune-only socket | Rejection; incompatible socket type | R-13.9.5 |

### TC-13.9.6.1 Transfer Reparent

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Transfer item from container A to B | Item parent entity changes to B | R-13.9.6 |

### TC-13.9.6.2 Transfer Prediction Rollback

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Client predicts transfer; server rejects | Client rolls back to pre-transfer state | R-13.9.6 |

### TC-13.9.7.1 Loot Round Robin

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Distribute 4 items to 4 players in round-robin | Each player receives 1 item in rotation | R-13.9.7 |

### TC-13.9.8.1 Merchant Buy

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Buy item for 100 gold; balance=500 | Balance=400; item in inventory | R-13.9.8 |

### TC-13.9.9.1 Equip Stat Apply

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Equip weapon (+20 attack) | Attack stat increased by 20 | R-13.9.9 |

### TC-13.9.9.2 Unequip Stat Revert

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Unequip weapon (+20 attack) | Attack stat reverts to previous value | R-13.9.9 |

### TC-13.9.10.1 Inventory Serialize Roundtrip

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Serialize inventory with socketed items; deserialize | All data preserved including sockets | R-13.9.10 |

## Integration Tests

### TC-13.9.9.I1 Equip Full Pipeline

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Equip item | Stat change + visual attachment + animation trigger + body region occlusion | R-13.9.9, R-13.8.10 |

### TC-13.8.NF1.I1 Character Creator Load

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Load character creator | Interactive within 3 seconds | R-13.8.NF1 |

### TC-13.8.NF2.I1 Morph Slider Latency

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Move facial slider continuously | Mesh updates within same frame | R-13.8.NF2 |

### TC-13.8.13.I1 500 Characters Frame Budget

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Render 500 unique characters at 1080p | Frame time < 16 ms | R-13.8.13 |

### TC-13.8.14.I1 Mesh Merge Async

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Merge 6 mesh parts | Async; no main thread block; 1 draw call | R-13.8.14 |

### TC-13.8.14.I2 Merge Cache Hit

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Repeat same part combination | Cache hit; no re-merge | R-13.8.14 |

### TC-13.9.NF1.I1 Inventory 500 Items

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | 500 stacks; 1000 random operations | All operations under latency budget | R-13.9.NF1 |

### TC-13.9.NF2.I1 20 Containers Memory

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | 20 containers x 100 items with properties | Total memory < 2 MB | R-13.9.NF2 |

### TC-13.9.NF3.I1 Inventory Ops Latency

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | 10,000 random operations | p99 latency < 1 ms | R-13.9.NF3 |

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
