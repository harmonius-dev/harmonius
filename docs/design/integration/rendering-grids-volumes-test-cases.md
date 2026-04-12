# Rendering ↔ Grids/Volumes Integration Test Cases

## Integration Tests

| ID | Test | Input | Expected | Req |
|----|------|-------|----------|-----|
| TC-IR-3.3.1.1 | Fog texture uploads from grid | 256x256 UniformGrid, 10 dirty cells | GPU texture updated at dirty coords | IR-3.3.1 |
| TC-IR-3.3.1.2 | Fog three-state values correct | Cells: hidden=0, explored=128, visible=255 | R8 texture values match | IR-3.3.1 |
| TC-IR-3.3.1.3 | Fog shader darkens hidden area | Camera over hidden region | Rendered pixels darkened | IR-3.3.1 |
| TC-IR-3.3.2.1 | Voxel GI volume uploads | 64^3 VoxelVolume with GI data | 3D texture matches volume | IR-3.3.2 |
| TC-IR-3.3.2.2 | Voxel GI lights scene | Emissive voxel in volume | Nearby surfaces receive indirect light | IR-3.3.2 |
| TC-IR-3.3.3.1 | SDF shadow ray march | SDF volume with sphere at center | Soft shadow behind sphere | IR-3.3.3 |
| TC-IR-3.3.3.2 | SDF shadow no NaN output | SDF with edge-case zero distance | No NaN in shadow buffer | IR-3.3.3 |
| TC-IR-3.3.4.1 | Dirty region partial upload | 256x256 grid, 4x4 dirty region | Only 16 cells uploaded | IR-3.3.4 |
| TC-IR-3.3.4.2 | No upload when clean | Grid unchanged for 10 frames | Zero texture uploads | IR-3.3.4 |
| TC-IR-3.3.5.1 | Tactical overlay renders | Grid with cover values | Color-coded decal on terrain | IR-3.3.5 |
| TC-IR-3.3.5.2 | Overlay updates with grid | Change cell from open to cover | Decal color updates next frame | IR-3.3.5 |

## Benchmarks

| ID | Benchmark | Target | Req |
|----|-----------|--------|-----|
| TC-IR-3.3.1.B1 | Fog upload 256x256 full | < 1 ms (NFR-SIM.GV5) | IR-3.3.1 |
| TC-IR-3.3.4.B1 | Dirty upload 100 regions | < 0.5 ms | IR-3.3.4 |
| TC-IR-3.3.2.B1 | Voxel GI pass 128^3 | < 2 ms GPU | IR-3.3.2 |
| TC-IR-3.3.3.B1 | SDF shadow 1080p | < 1 ms GPU | IR-3.3.3 |
