# User Stories — 13.12 Character Progression

## F-13.12.1a Race Definition

## US-13.12.1a.1 Design Race Balance and Differentiation

**As a** gameplay director (P-3), **I want to** define race stat modifiers and racial abilities that
provide meaningful differentiation without imbalance, **so that** race choice is interesting but not
mandatory.

## US-13.12.1a.2 Choose a Race for My Character

**As a** player (P-23), **I want to** select a race with unique stat modifiers, racial abilities,
and cosmetic constraints, **so that** my character has a distinct identity from creation.

## US-13.12.1a.3 Verify Race Stat Modifiers Apply on Character Creation

**As an** engine tester (P-27), **I want to** create characters of each race and verify their
starting stats match the configured modifiers, **so that** race definitions apply correctly.

## F-13.12.1b Class Definition

## US-13.12.1b.1 Design Class Roles and Balance

**As a** gameplay director (P-3), **I want to** define class roles (tank, healer, DPS, support) with
distinct ability sets and resources, **so that** each class provides a unique gameplay experience.

## US-13.12.1b.2 Choose a Class for My Character

**As a** player (P-23), **I want to** select a class with a defined ability set, level-up unlocks,
equipment restrictions, and class-specific resource, **so that** my character has a clear gameplay
role from the start.

## US-13.12.1b.3 Verify Class Ability Unlocks at Level Thresholds

**As an** engine tester (P-27), **I want to** level a character and verify abilities unlock at the
configured level thresholds, **so that** class progression works as designed.

## US-13.12.1b.4 Verify Equipment Proficiency Restrictions

**As an** engine tester (P-27), **I want to** attempt to equip weapons and armor outside the class
proficiency and verify they are rejected, **so that** class equipment restrictions are enforced.

## F-13.12.1c Multi-Class and Job Change

## US-13.12.1c.1 Design Hybrid Class Restrictions

**As a** gameplay director (P-3), **I want to** define hybrid class restrictions that prevent
overpowered combinations, **so that** multi-classing adds depth without breaking balance.

## US-13.12.1c.2 Switch Classes at an NPC

**As a** player (P-23), **I want to** switch classes at designated NPCs while preserving my previous
class's progress, **so that** I can adapt my playstyle without restarting.

## US-13.12.1c.3 Unlock Hybrid Classes Meeting Prerequisites

**As a** player (P-23), **I want to** unlock hybrid classes that combine abilities from two parent
classes when I meet prerequisite conditions, **so that** advanced play rewards investment.

## US-13.12.1c.4 Verify Previous Class Progress Is Preserved

**As an** engine tester (P-27), **I want to** switch classes and switch back, verifying the original
class's level, abilities, and progress are intact, **so that** class switching preserves all prior
progress.

## F-13.12.1d Prestige and Rebirth System

## US-13.12.1d.1 Design Prestige Reward Curve

**As a** gameplay director (P-3), **I want to** design escalating prestige bonuses across multiple
rebirth cycles, **so that** repeated prestige remains motivating.

## US-13.12.1d.2 Prestige at Max Level for Permanent Bonuses

**As a** player (P-23), **I want to** restart at level 1 with permanent stat boosts, cosmetic
rewards, and titles after reaching max level, **so that** I have a rewarding end-game progression
loop.

## US-13.12.1d.3 Verify Prestige Bonuses Accumulate Across Cycles

**As an** engine tester (P-27), **I want to** prestige multiple times and verify bonuses accumulate
correctly across cycles, **so that** prestige progression stacks as designed.

## US-13.12.1d.4 Verify Prestige Resets Level but Preserves Cosmetics

**As an** engine tester (P-27), **I want to** prestige and verify level resets to 1 while permanent
bonuses and cosmetic rewards are retained, **so that** prestige does not lose earned rewards.

## F-13.12.2a Talent Tree Data Model

## US-13.12.2a.1 Design Talent Tree Balance and Depth

**As a** gameplay director (P-3), **I want to** design talent trees with meaningful choices between
competing specializations, **so that** talent allocation creates distinct build identities.

## US-13.12.2a.2 Browse the Talent Tree

**As a** player (P-23), **I want to** view a visual node-graph talent tree with clear prerequisites,
tier gates, and node types (passive, active, ability modification), **so that** I can plan my build
before spending points.

## US-13.12.2a.3 Verify Prerequisite Edges Enforce Unlock Order

**As an** engine tester (P-27), **I want to** attempt to allocate a talent node without its
prerequisites met and verify it is rejected, **so that** prerequisite enforcement is correct.

## US-13.12.2a.4 Verify Tier Gating Requires N Total Points

**As an** engine tester (P-27), **I want to** attempt to allocate a node in a gated tier without
enough total points spent and verify it is rejected, **so that** tier gating works correctly.

## F-13.12.2b Talent Allocation and Respec

## US-13.12.2b.1 Design Respec Economy

**As a** gameplay director (P-3), **I want to** design respec costs that balance build
experimentation against economic friction, **so that** players can adapt builds without trivializing
choices.

## US-13.12.2b.2 Spend Talent Points on Unlocked Nodes

**As a** player (P-23), **I want to** spend talent points earned per level on unlocked nodes
respecting prerequisite and tier constraints, **so that** my build choices feel meaningful.

## US-13.12.2b.3 Respec Talents for Currency

**As a** player (P-23), **I want to** respec all talent allocations for a currency cost and receive
all spent points back, **so that** my build choices are not permanent.

## US-13.12.2b.4 Verify Respec Refunds All Points

**As an** engine tester (P-27), **I want to** allocate all talent points, respec, and verify all
points are refunded and all talent effects are removed, **so that** respec is a complete reset.

## F-13.12.2c Talent Tree Visual Editor

## US-13.12.2c.1 Edit Talent Trees as Graph Assets

**As a** gameplay director (P-3), **I want to** create and edit talent trees in the visual editor
with drag-and-drop node creation, edge drawing, and tier lane assignment, **so that** talent tree
authoring is visual and intuitive.

## US-13.12.2c.2 Preview Prerequisite Chains in the Editor

**As a** player (P-23), **I want to** the talent tree UI to show clear prerequisite chains and
available vs locked nodes, **so that** build planning is straightforward.

## US-13.12.2c.3 Verify Editor Graph Matches Runtime Behavior

**As an** engine tester (P-27), **I want to** create a talent tree in the editor and verify runtime
allocation behavior matches the editor's prerequisite and tier gating preview, **so that** editor
and runtime are in sync.

## F-13.12.3a Profession Data Model

## US-13.12.3a.1 Design Profession Limits and Synergies

**As a** gameplay director (P-3), **I want to** design profession limits (max simultaneous
professions) and cross-profession synergies, **so that** profession choice creates meaningful
specialization.

## US-13.12.3a.2 Level Up a Profession

**As a** player (P-23), **I want to** level a profession by earning XP from successful crafts,
gathers, or quest turn-ins, with recipes unlocking at skill thresholds, **so that** trade skill
progression is its own rewarding path.

## US-13.12.3a.3 Verify Recipe Unlocks at Skill Thresholds

**As an** engine tester (P-27), **I want to** level a profession and verify recipes unlock exactly
at the configured skill thresholds, **so that** profession gating works correctly.

## F-13.12.3b Gathering Profession Integration

## US-13.12.3b.1 Design Gathering Yield Curves

**As a** gameplay director (P-3), **I want to** design yield scaling curves that reward profession
investment with better materials, **so that** gathering progression feels meaningful.

## US-13.12.3b.2 Gather Resources With Skill-Based Yield

**As a** player (P-23), **I want to** gather resources from world nodes with yield quantity and rare
proc rate scaling with my profession level, **so that** investing in gathering feels worthwhile.

## US-13.12.3b.3 Verify Yield Scales With Profession Level

**As an** engine tester (P-27), **I want to** gather at each profession level and verify yield
matches the configured scaling curve, **so that** skill-based yield scaling is accurate.

## F-13.12.3c Crafting Profession Integration

## US-13.12.3c.1 Design Crafting XP Progression

**As a** gameplay director (P-3), **I want to** design crafting XP awards that scale with recipe
difficulty, **so that** crafting profession leveling is well-paced.

## US-13.12.3c.2 Craft Items at Profession-Gated Recipes

**As a** player (P-23), **I want to** craft items using the recipe system with availability gated by
my profession level, earning profession XP on success, **so that** crafting advances my trade skill.

## US-13.12.3c.3 Verify Crafting Respects Profession Level Gates

**As an** engine tester (P-27), **I want to** attempt to craft a recipe above my profession level
and verify it is blocked, **so that** profession level gating is enforced.

## F-13.12.4 Crafting Station Interaction

## US-13.12.4.1 Design Station Placement in Housing and World

**As a** gameplay director (P-3), **I want to** stations to be placeable in player housing and at
fixed world locations, **so that** crafting integrates with both housing and world exploration.

## US-13.12.4.2 Craft at Physical In-World Stations

**As a** player (P-23), **I want to** interact with in-world crafting stations (forge, alchemy
table, workbench) to access profession-specific recipes filtered by my level, **so that** crafting
is a spatial, immersive activity.

## US-13.12.4.3 See Crafting UI With Recipe Details

**As a** player (P-23), **I want to** the station UI to display available recipes with required
materials, inventory counts, success probability, and crafting progress bar, **so that** I have all
information needed to craft.

## US-13.12.4.4 Verify Station Type Gates Correct Recipes

**As an** engine tester (P-27), **I want to** interact with each station type and verify only the
correct profession's recipes appear, **so that** station-recipe gating is enforced.

## F-13.12.5 Reputation and Faction Standing

## US-13.12.5.1 Design Rival Faction Dynamics

**As a** gameplay director (P-3), **I want to** gaining reputation with one faction to optionally
reduce reputation with a rival, **so that** faction choice creates meaningful tension and
consequence.

## US-13.12.5.2 Earn Reputation With Factions

**As a** player (P-23), **I want to** earn reputation with factions through kills, quests, item
turn-ins, and world events, progressing through standing tiers (hostile to exalted), **so that**
faction relationships reflect my gameplay choices.

## US-13.12.5.3 Access Tier-Gated Content

**As a** player (P-23), **I want to** reputation tiers to gate faction vendors, quest lines, zone
access, and cosmetic rewards, **so that** reputation progression unlocks meaningful content.

## US-13.12.5.4 Verify Asymmetric Faction Relationships

**As an** engine tester (P-27), **I want to** gain reputation with a faction that has a rival and
verify the rival's reputation decreases by the configured amount, **so that** asymmetric faction
relationships work correctly.

## US-13.12.5.5 Verify Tier-Gated Content Unlocks

**As an** engine tester (P-27), **I want to** reach each standing tier and verify the correct
vendors, quests, and rewards unlock, **so that** tier gating is enforced at every threshold.

## F-13.12.6a Achievement Definition and Tracking

## US-13.12.6a.1 Design Achievement Categories and Visibility

**As a** gameplay director (P-3), **I want to** design achievement categories with configurable
visibility (tracked, hidden, secret), **so that** some achievements reward exploration and surprise.

## US-13.12.6a.2 Track Achievement Progress

**As a** player (P-23), **I want to** see incremental progress toward achievements as I play (kill
47/100 enemies), **so that** I know how close I am to completing each goal.

## US-13.12.6a.3 Verify Achievement Progress Fires Through Observers

**As an** engine tester (P-27), **I want to** complete an achievement condition and verify the
observer system fires the completion event with correct timestamp, **so that** achievement tracking
is event-driven and accurate.

## F-13.12.6b Achievement Rewards and Display

## US-13.12.6b.1 Design Achievement Reward Tiers

**As a** gameplay director (P-3), **I want to** design per-achievement rewards (items, titles,
currency, cosmetics) and point values, **so that** achievements have appropriate incentives.

## US-13.12.6b.2 Receive Achievement Rewards

**As a** player (P-23), **I want to** receive items, titles, currency, or cosmetics when I complete
an achievement, **so that** achievements feel rewarding beyond the notification.

## US-13.12.6b.3 See Achievement Score Accumulate

**As a** player (P-23), **I want to** achievement points to accumulate into a player score displayed
in the achievement UI, **so that** my overall completion effort is tracked.

## US-13.12.6b.4 Verify Reward Distribution on Completion

**As an** engine tester (P-27), **I want to** complete an achievement and verify all configured
rewards are granted correctly, **so that** achievement reward distribution is reliable.

## F-13.12.6c Platform Achievement Sync

## US-13.12.6c.1 Design Platform Achievement Mappings

**As a** gameplay director (P-3), **I want to** map each engine achievement to platform achievement
IDs (Steam, PlayStation Trophies, Xbox), **so that** unlock synchronization is configured per
platform.

## US-13.12.6c.2 See Achievements on My Platform Profile

**As a** player (P-23), **I want to** engine achievements to sync to Steam, PlayStation, or Xbox,
**so that** my accomplishments appear on my platform profile.

## US-13.12.6c.3 Verify Platform Achievement Sync

**As an** engine tester (P-27), **I want to** complete an engine achievement and verify the
corresponding platform achievement unlocks, **so that** cross-platform sync works correctly.

## F-13.12.7 Item Enhancement and Enchanting

## US-13.12.7.1 Design Enhancement Risk/Reward Curve

**As a** gameplay director (P-3), **I want to** decreasing success probability at higher levels with
escalating rewards, **so that** enhancement creates exciting risk/reward decisions.

## US-13.12.7.2 Enhance My Equipment

**As a** player (P-23), **I want to** enhance equipment through levels (0 to +15) with visible
success rates, material costs, and protection items, **so that** I have a rewarding long-term goal
of improving my gear.

## US-13.12.7.3 See Enhancement Preview in UI

**As a** player (P-23), **I want to** the enchanting UI to show current level, next level preview,
materials required, and success rate, **so that** I can make informed enhancement decisions.

## US-13.12.7.4 Verify Enhancement Success/Failure Outcomes

**As an** engine tester (P-27), **I want to** verify each failure consequence (no change, level
decrease, item destruction) triggers at the correct probability, **so that** enhancement outcomes
match configured odds.

## US-13.12.7.5 Verify Protection Items Prevent Destruction

**As an** engine tester (P-27), **I want to** use a protection item during enhancement, trigger a
failure that would destroy the item, and verify the item survives, **so that** protection items work
correctly.

## F-13.12.8a Item Rarity Tier System

## US-13.12.8a.1 Design Rarity Distribution in Loot

**As a** gameplay director (P-3), **I want to** rarity to be assigned when items drop from loot
tables with controlled distribution rates, **so that** rarity feels meaningful and rare items are
genuinely rare.

## US-13.12.8a.2 Compare Items by Rarity

**As a** player (P-23), **I want to** items to have color-coded rarity tiers (common to mythic) with
stat ranges corresponding to rarity, **so that** I can quickly judge item quality at a glance.

## US-13.12.8a.3 Verify Stat Rolls Fall Within Rarity Ranges

**As an** engine tester (P-27), **I want to** generate 1000 items at each rarity tier and verify all
stat rolls fall within the configured ranges, **so that** rarity-based stat generation is correct.

## F-13.12.8b Affix System

## US-13.12.8b.1 Design Affix Pools and Rarity Scaling

**As a** gameplay director (P-3), **I want to** design affix pools and per-rarity affix count
ranges, **so that** higher-rarity items have more and better affixes.

## US-13.12.8b.2 Find Items With Random Affixes

**As a** player (P-23), **I want to** dropped items to have random prefixes and suffixes ("Blazing
Sword of the Bear") granting additional stat bonuses, **so that** each drop feels unique.

## US-13.12.8b.3 Verify Affix Count Scales With Rarity

**As an** engine tester (P-27), **I want to** generate items at each rarity and verify affix counts
fall within the configured per-rarity ranges, **so that** rarity-affix scaling is correct.

## F-13.12.8c Stat Re-Rolling Mechanics

## US-13.12.8c.1 Design Re-Roll Cost Progression

**As a** gameplay director (P-3), **I want to** re-roll cost to scale with item level and rarity,
**so that** re-rolling high-end items is a meaningful investment.

## US-13.12.8c.2 Re-Roll Item Affixes for Currency

**As a** player (P-23), **I want to** spend currency to re-randomize an item's affixes while
preserving the base item and rarity, **so that** I have a path to improve a good base item.

## US-13.12.8c.3 Verify Re-Roll Preserves Base Item and Rarity

**As an** engine tester (P-27), **I want to** re-roll an item and verify the base item type and
rarity tier are unchanged while affixes are re-randomized, **so that** re-rolling only changes what
it should.

## F-13.12.9 Item Set Bonuses

## US-13.12.9.1 Design Set Bonus Tiers and Effects

**As a** gameplay director (P-3), **I want to** define equipment sets with escalating bonus tiers
(2-piece, 4-piece, 6-piece) including stat bonuses, proc effects, and ability unlocks, **so that**
set completion rewards are compelling.

## US-13.12.9.2 Earn Set Bonuses by Equipping Multiple Pieces

**As a** player (P-23), **I want to** wearing multiple pieces from an equipment set to grant
escalating bonuses, **so that** completing sets is a rewarding long-term goal.

## US-13.12.9.3 See Set Progress in Equipment Panel

**As a** player (P-23), **I want to** the equipment panel to show collected vs. missing set pieces
with active and upcoming bonus tiers, **so that** I can track my set completion.

## US-13.12.9.4 Verify Set Bonuses Activate at Correct Thresholds

**As an** engine tester (P-27), **I want to** equip set pieces one by one and verify bonuses
activate exactly at the configured piece count thresholds, **so that** set bonus triggers are
correct.

## US-13.12.9.5 Verify Set Bonuses Deactivate on Unequip

**As an** engine tester (P-27), **I want to** unequip a set piece below a bonus threshold and verify
the bonus is removed, **so that** set bonuses do not persist when thresholds are no longer met.

## F-13.12.10 Item Durability and Repair

## US-13.12.10.1 Design Durability as a Gold Sink

**As a** gameplay director (P-3), **I want to** durability repair costs to serve as a gold sink that
scales with gear quality, **so that** the economy has a consistent currency drain.

## US-13.12.10.2 See Durability on My Equipment

**As a** player (P-23), **I want to** durability displayed in item tooltips and equipment UI with
warnings at 25% and 10%, **so that** I know when to repair.

## US-13.12.10.3 Repair Equipment at Vendors or With Kits

**As a** player (P-23), **I want to** repair at NPC repair vendors (cost scales with item level and
rarity) or with consumable repair kits, **so that** I have options for maintaining my gear.

## US-13.12.10.4 Verify Equipment Becomes Non-Functional at 0% Durability

**As an** engine tester (P-27), **I want to** reduce durability to 0% and verify the item becomes
non-functional (no damage, no defense) but is not destroyed, **so that** zero durability behavior is
correct.

## US-13.12.10.5 Verify Repair Restores Full Functionality

**As an** engine tester (P-27), **I want to** repair an item from 0% to 100% and verify full stat
restoration, **so that** repair fully reverses durability loss.
