# Questions for User Review

## Phase 2 Questions — Feature Confirmation

These questions would be asked after presenting the proposed features and before writing final
artifacts.

### Which features to keep?

1. The proposed features are F-3.6.65 through F-3.6.74. Which of these should be kept, and which
   should be dropped?

2. F-3.6.65 (Biome Region Painting Tool) assumes an editor brush tool. Should this also support
   programmatic biome assignment via the PCG graph (F-3.6.1), or is the brush tool sufficient?

3. F-3.6.72 (Biome Seasonal Variation) adds a global season clock. Does the game framework already
   have a time/calendar system, or should this be a standalone feature? Should seasons be tied to
   real-world time or purely in-game time?

### Expansion suggestions

4. I suggested F-3.6.73 (Biome-Driven Ambient Audio Zones) and F-3.6.74 (Biome Debug Visualization)
   as additional features beyond the original idea. Should these be included?

5. Should we add a biome-driven lighting feature? For example, different sky colors, sun intensity,
   or ambient light levels per biome (jungle canopy darkening, desert harsh sunlight). This would
   extend the existing lighting system rather than creating a new one.

6. Should biome regions affect NPC and creature spawning (F-3.6.40)? For example, wolves in forest
   biomes, scorpions in desert biomes. This is partially covered by the existing creature placement
   feature but could be made more explicit.

### Missing features?

7. The existing features F-3.6.11 (Biome Distribution System) and F-3.6.44 (Biome Classification and
   Distribution) handle procedural biome assignment. The new features focus on designer-painted
   override and biome-driven generation. Is there overlap that should be resolved, or should painted
   and procedural biomes coexist as currently proposed?

8. Should biome painting support a "biome influence" mode where the designer paints a soft
   probability gradient rather than a hard biome assignment? This would let designers nudge
   procedural generation rather than fully overriding it.

9. Is there a need for biome-specific physics properties? For example, sand terrain having different
   friction than ice terrain, or swamp terrain slowing movement. This would connect biome data to
   the physics system (F-4.1.x).

## Phase 4 Questions — Requirements and Stories Review

These questions would be asked after presenting the generated requirements and user stories.

### Requirements review

10. R-3.6.65a specifies a 500ms preview update time. Is this target realistic and sufficient? Should
    it be faster (200ms) for a more responsive editing experience?

11. R-3.6.68a specifies 5ms per tile for heightmap generation. Is this budget appropriate given the
    existing terrain tile generation costs? Should it be tighter or more relaxed?

12. R-3.6.70a specifies 50 species per biome and 100K instances per tile. Are these targets
    appropriate for the art direction? Some biomes (like desert) may need far fewer species.

13. R-3.6.71a specifies a 10-second weather transition time. Is this the right duration? Should it
    be configurable per biome pair (fast for oasis-to-desert, slow for forest-to-tundra)?

### User story review

14. Are the right personas represented? The stories cover game designer (P-5), level designer (P-6),
    environment artist (P-8), technical artist (P-13), audio designer (P-14), player (P-23), and
    engine tester (P-27). Are any personas missing?

15. Should there be user stories from the creative director (P-2) perspective, since biome painting
    is a high-level world-building activity that shapes the game's visual identity?

16. Should there be user stories from the server administrator (P-22) perspective for biome data
    replication and server-side generation?

17. Are the acceptance criteria realistic and measurable? Any criteria that are too vague or need
    specific numbers?
