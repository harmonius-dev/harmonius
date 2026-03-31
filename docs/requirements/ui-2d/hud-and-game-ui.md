# R-10.3 — HUD and Game UI Requirements

## Player Status

1. **R-10.3.1** — The engine **SHALL** render bar widgets for health, mana, and resources with
   animated fill/drain, predicted-damage overlays, and absorb-shield overlays, at 60 FPS with 40+
   bars simultaneously.
   - **Rationale:** Raid encounters require all party bars visible at once without frame drops.
   - **Verification:** Render 40 bars each animating drain, predicted damage, and absorb. Assert
     frame time under 16.67 ms for 300 frames.

2. **R-10.3.2** — The engine **SHALL** display buff/debuff icon grids with radial cooldown sweeps,
   countdown text, tooltip inspection, and priority filtering for 30+ simultaneous effects.
   - **Rationale:** Players must quickly identify dispellable debuffs and track timers in combat.
   - **Verification:** Apply 35 effects with varying priorities. Assert icons grouped by category,
     each showing correct remaining duration.

3. **R-10.3.3** — The engine **SHALL** render action bar grids with frame-accurate cooldown sweeps
   synchronized to the simulation tick within one frame of display error.
   - **Rationale:** Frame-accurate display prevents players from pressing abilities still on
     cooldown.
   - **Verification:** Start a 5-second cooldown. Assert sweep reaches zero within one frame of the
     simulation tick when cooldown expires.

## World-Anchored UI

4. **R-10.3.4** — The engine **SHALL** render floating nameplates anchored to 3D positions with LOD
   culling, terrain occlusion, and overlap avoidance, scaling to 200+ at 60 FPS.
   - **Rationale:** Cities contain hundreds of characters; without culling, nameplates stack
     illegibly.
   - **Verification:** Spawn 250 entities with nameplates. Assert frame time under 16.67 ms. Assert
     no two overlap by more than 10% area.

5. **R-10.3.5** — The engine **SHALL** spawn floating text at world positions with configurable
   trajectories, damage-type colors, and cumulative merging, handling 50+ simultaneous events.
   - **Rationale:** Burst AoE combat produces dozens of numbers; merging prevents unreadable text.
   - **Verification:** Emit 60 damage events in one frame at the same position. Assert all displayed
     or merged. Assert frame time under 16.67 ms.

## Navigation

6. **R-10.3.6** — The engine **SHALL** render a minimap with real-time icons and a world map with
   zoom, pan, fog-of-war, and waypoint placement, with markers updating each frame.
   - **Rationale:** The minimap is the primary spatial awareness tool; stale markers lose party
     members.
   - **Verification:** Place 20 moving entities. Assert minimap icons match projected positions
     within 1 pixel.

7. **R-10.3.7** — The engine **SHALL** display a quest tracker with progress indicators, waypoint
   markers, compass indicators, and distance, supporting multi-step chains and group-shared
   progress.
   - **Rationale:** Complex quest structures require clear tracking so players know their next
     objective.
   - **Verification:** Create a 3-step chain with branch and group kills. Assert tracker updates
     steps, branch, shared count, and waypoint marker.

8. **R-10.3.8** — The engine **SHALL** provide a chat window with multiple channels, tabs,
   scrollable history, clickable links, emoji, and moderation, sustaining 200+ messages/second.
   - **Rationale:** City hubs generate extreme throughput; dropped messages degrade the social
     experience.
   - **Verification:** Enqueue 250 messages/second across 5 channels for 10 seconds. Assert all
     appear in correct tabs. Assert frame time under 16.67 ms.

9. **R-10.3.9** — The engine **SHALL** render grid-based inventory with drag-and-drop, stack
   splitting, sorting, search, and quality borders, sharing one widget across all container types.
   - **Rationale:** A unified grid reduces implementation surface and ensures consistent behavior.
   - **Verification:** Open bag, bank, vendor. Drag sell, split stack, sort by rarity, filter by
     search. Assert each completes correctly.

## Compass and Map

10. **R-10.3.10** — The engine **SHALL** render a compass strip with cardinal directions and tracked
    markers that rotate with facing, in both 2D and 3D modes. At least 3 styles **SHALL** be
    configurable.
    - **Rationale:** Compass provides constant directional awareness without opening the map.
    - **Verification:** Track 5 objectives. Rotate player. Verify markers move correctly. Switch
      styles.

11. **R-10.3.11** — The engine **SHALL** display edge arrows for off-screen tracked objectives that
    transition smoothly to in-world markers on viewport entry.
    - **Rationale:** Off-screen indicators guide players without map checks.
    - **Verification:** Track objective behind player. Assert edge arrow appears. Turn toward it.
      Assert smooth transition to in-world marker.

12. **R-10.3.12** — The engine **SHALL** auto-generate minimap textures from terrain, biome,
    building, and road data, updating as chunks stream in.
    - **Rationale:** Procedural minimap eliminates manual authoring for infinite worlds.
    - **Verification:** Load procedural world. Assert minimap renders terrain and roads. Stream new
      chunk. Assert minimap extends.

13. **R-10.3.13** — The engine **SHALL** generate a multi-layer world map with continuous zoom,
    panning, waypoints, fast travel, and tiled image pyramid caching.
    - **Rationale:** The world map is the primary navigation tool for open-world games.
    - **Verification:** Open world map. Verify all layers. Zoom from world to local. Place waypoint.
      Assert it appears on minimap and compass.

14. **R-10.3.14** — The engine **SHALL** support artistic overlays on generated maps with at least 5
    post-processing styles. Static hand-painted maps **SHALL** be substitutable while preserving
    dynamic markers.
    - **Rationale:** Artistic map styles reinforce the game's visual identity.
    - **Verification:** Apply parchment style. Verify sepia and ink lines. Replace zone with static
      image. Verify quest markers overlay correctly.

15. **R-10.3.15** — The engine **SHALL** display data-driven markers on minimap and compass
    simultaneously with consistent icons, supporting at least 8 categories, quest-driven visibility,
    and clustering.
    - **Rationale:** Consistent markers across views give reliable spatial awareness.
    - **Verification:** Track quest. Assert markers on minimap, compass, and world map. Untrack.
      Assert markers disappear from all. Zoom out. Assert clustering activates.
