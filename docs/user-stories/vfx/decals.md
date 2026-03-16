# User Stories -- 11.2 Decals

## Projection and Rendering

| ID | Persona | Story | Acceptance Criteria | Features | Requirements |
|---|---|---|---|---|---|
| US-11.2.1.1 | Environment artist (P-8) | I want deferred projected decals that modify albedo, normal, roughness, and metallic channels independently across multiple meshes, so that battle damage, graffiti, and signage conform to complex fortress walls and cave surfaces. | Decals modify each G-buffer channel independently; decals blend across mesh boundaries seamlessly | F-11.2.1 | R-11.2.1 |
| US-11.2.1.2 | Environment artist (P-8) | I want triplanar decal projection for surfaces where standard box projection would stretch, so that decals on angled walls and intersecting geometry look correct. | Triplanar projection handles complex geometry without stretching or seams | F-11.2.1 | R-11.2.1 |
| US-11.2.1.3 | Engine tester (P-27) | I want to enable forward rendering and verify that deferred decals are replaced by mesh decals without visual errors, so that decals work on platforms where deferred rendering is disabled. | Forward path falls back to mesh decals; no visual errors on fallback path | F-11.2.1, F-11.2.2 | R-11.2.1, R-11.2.2 |
| US-11.2.2.1 | Environment artist (P-8) | I want geometry-based mesh decals with correct tangent-space normals for permanent markings like murals, graffiti, and signage, so that persistent decals have sub-pixel accuracy on curved surfaces. | Mesh decals conform to surface with correct tangent-space normals; sub-pixel accuracy on curved surfaces | F-11.2.2 | R-11.2.2 |
| US-11.2.2.2 | Engine tester (P-27) | I want to verify that on-demand mesh decal generation is disabled on mobile due to CPU cost, falling back to deferred decals, so that mobile does not incur mesh decal generation overhead. | On-demand mesh decal generation disabled on mobile; deferred decal fallback active on mobile | F-11.2.2 | R-11.2.2 |

## Management and Performance

| ID | Persona | Story | Acceptance Criteria | Features | Requirements |
|---|---|---|---|---|---|
| US-11.2.3.1 | Engine developer (P-26) | I want a runtime decal texture atlas that packs decal textures into shared pages to minimize texture binds and draw calls, so that hundreds of active decals render with minimal submission overhead. | Decals sharing atlas page batched into single draw; draw call count proportional to atlas pages, not individual decals | F-11.2.3 | R-11.2.3 |
| US-11.2.3.2 | Engine tester (P-27) | I want to spawn 500 decals during a siege battle and verify that LRU eviction reclaims atlas entries when memory pressure rises, so that atlas memory stays bounded without allocation failures. | LRU eviction reclaims least-recently-used entries; no allocation failures under memory pressure | F-11.2.3 | R-11.2.3 |
| US-11.2.4.1 | Environment artist (P-8) | I want priority-based decal stacking with per-channel blend modes (alpha, multiply, additive), so that overlapping blood splatter, scorch marks, and footprints resolve cleanly without z-fighting. | Higher-priority decals overwrite lower per-channel; all blend modes composite correctly; no z-fighting between overlapping decals | F-11.2.4 | R-11.2.4 |
| US-11.2.4.2 | Effects artist (P-12) | I want decal lifecycle with configurable fade-in, sustain, and dissolve-out using noise-threshold breakup, so that decals disappear naturally over time rather than popping off abruptly. | Fade-in, sustain, and dissolve-out timing matches configuration; noise-threshold breakup produces natural dissolve | F-11.2.4 | R-11.2.4 |
| US-11.2.4.3 | Engine tester (P-27) | I want to verify that mobile uses a global pool of 64 decals with shorter lifecycle durations and desktop uses 256, so that decal memory is recycled fast enough on each platform. | Mobile pool capped at 64; desktop pool capped at 256; oldest low-priority decals reclaimed first when pool exhausted | F-11.2.4 | R-11.2.4 |

## Gameplay Decals

| ID | Persona | Story | Acceptance Criteria | Features | Requirements |
|---|---|---|---|---|---|
| US-11.2.5.1 | Effects artist (P-12) | I want procedural blood decals spawned from hit events with variation driven by weapon type, impact angle, and surface material, so that melee combat produces unique, non-repetitive blood spatter patterns. | Distinct decal variations per weapon type; projection direction responds to impact angle; atlas selection avoids repeated patterns | F-11.2.5 | R-11.2.5 |
| US-11.2.5.2 | Engine tester (P-27) | I want to verify that blood decals are disabled when platform content rating requires it (region-specific), so that the game passes certification in restricted regions. | Blood decals disabled when content rating requires; no blood decal rendering in restricted regions | F-11.2.5 | R-11.2.5 |
| US-11.2.6.1 | Player (P-23) | I want surface-aware footprint decals where mud displaces, snow compresses, and sand scatters based on the terrain material layer, so that walking through different surfaces feels tactile and grounded. | Footprint shape and material response differ per surface type; terrain material layer drives footprint behavior | F-11.2.6 | R-11.2.6 |
| US-11.2.6.2 | Engine tester (P-27) | I want to verify that mobile spawns footprints every 4th step and desktop spawns every step, so that footprint generation cost scales per platform. | Mobile spawns every 4th step; desktop spawns every step; simplified textures on mobile | F-11.2.6 | R-11.2.6 |
| US-11.2.6.3 | Effects artist (P-12) | I want ribbon-style tire track decals projected along the vehicle path with width matching the wheel contact patch, so that vehicles leave convincing track marks on soft terrain. | Tire track width matches wheel contact patch; ribbon UV projection produces continuous tracks without gaps or distortion | F-11.2.6 | R-11.2.6 |
