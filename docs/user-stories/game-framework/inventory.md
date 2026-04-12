# User Stories — Inventory System (13.9)

## Storage Model

| ID          | Persona              |
|-------------|----------------------|
| US-13.9.1.1 | game designer (P-5)  |
| US-13.9.1.2 | game developer (P-15)|
| US-13.9.2.1 | game designer (P-5)  |
| US-13.9.2.2 | player (P-23)        |
| US-13.9.3.1 | game designer (P-5)  |

1. **US-13.9.1.1** — **As a** game designer (P-5), **I want** each inventory to be an ECS entity
   with configurable slot count, weight capacity, and layout mode, **so that** containers are
   data-driven.
2. **US-13.9.1.2** — **As a** game developer (P-15), **I want** all inventory operations to be ECS
   commands on item child entities, **so that** the inventory integrates with the ECS.
3. **US-13.9.2.1** — **As a** game designer (P-5), **I want** grid-based inventory with rectangular
   item cells and auto-sort, **so that** spatial inventory puzzles are supported.
4. **US-13.9.2.2** — [game-specific] **As a** player (P-23), **I want** auto-sort to rearrange items
   minimizing wasted space, **so that** I fit more items without manual organizing.
5. **US-13.9.3.1** — **As a** game designer (P-5), **I want** item stacking up to a per-type maximum
   with split operations, **so that** stackable items are consolidated.

## Item Instances

| ID          | Persona              |
|-------------|----------------------|
| US-13.9.4.1 | game designer (P-5)  |
| US-13.9.4.2 | game developer (P-15)|
| US-13.9.5.1 | game designer (P-5)  |
| US-13.9.5.2 | player (P-23)        |

1. **US-13.9.4.1** — **As a** game designer (P-5), **I want** individual item instances to carry
   mutable properties (durability, enchantments, sockets, bind status), **so that** items are
   unique.
2. **US-13.9.4.2** — **As a** game developer (P-15), **I want** per-instance properties as ECS
   components on item entities, **so that** systems query items by property.
3. **US-13.9.5.1** — **As a** game designer (P-5), **I want** item socket slots that accept insert
   items filtered by type hierarchy, **so that** gems and runes augment equipment.
4. **US-13.9.5.2** — [game-specific] **As a** player (P-23), **I want** inserting a gem into a
   socket to merge stat modifiers onto the parent item, **so that** my equipment improves.

## Operations

| ID          | Persona                    |
|-------------|----------------------------|
| US-13.9.6.1 | game designer (P-5)        |
| US-13.9.6.2 | player (P-23)              |
| US-13.9.7.1 | game designer (P-5)        |
| US-13.9.7.2 | player (P-23)              |
| US-13.9.8.1 | game designer (P-5)        |
| US-13.9.8.2 | player (P-23)              |

1. **US-13.9.6.1** — **As a** game designer (P-5), **I want** item transfers between containers
   validated for capacity, weight, and restrictions, **so that** invalid moves are rejected.
2. **US-13.9.6.2** — **As a** player (P-23), **I want** drag-and-drop between inventories with
   server-authoritative validation, **so that** transfers feel responsive and are secure.
3. **US-13.9.7.1** — **As a** game designer (P-5), **I want** loot distribution modes (free-for-all,
   round-robin, personal loot), **so that** group loot is configurable.
4. **US-13.9.7.2** — [game-specific] **As a** player (P-23), **I want** need/greed voting on group
   loot, **so that** items go to the player who needs them most.
5. **US-13.9.8.1** — **As a** game designer (P-5), **I want** buy/sell transactions driven by
   currency definitions with optional modifiers, **so that** merchant pricing is data-driven.
6. **US-13.9.8.2** — [game-specific] **As a** player (P-23), **I want** a two-inventory trade window
   with mutual confirmation, **so that** player-to-player trading is safe.

## Equipment and Persistence

| ID           | Persona              |
|--------------|----------------------|
| US-13.9.9.1  | game designer (P-5)  |
| US-13.9.9.2  | player (P-23)        |
| US-13.9.10.1 | game developer (P-15)|
| US-13.9.10.2 | server administrator (P-22)|

1. **US-13.9.9.1** — **As a** game designer (P-5), **I want** named equipment slots with slot
   compatibility using item type hierarchies, **so that** equipping is data-driven.
2. **US-13.9.9.2** — **As a** player (P-23), **I want** equipping an item to trigger stat modifiers
   and visual attachment, **so that** equipment changes are immediate.
3. **US-13.9.10.1** — **As a** game developer (P-15), **I want** complete inventory serialization
   with schema versioning for live-service migrations, **so that** patches do not wipe inventories.
4. **US-13.9.10.2** — **As a** server administrator (P-22), **I want** server-authoritative
   inventory storage preventing duplication exploits, **so that** item integrity is maintained.

## Parent Stories

The 3-segment parent stories below are umbrella rollups for the refined 4-segment sub-stories listed
above. Each parent inherits the persona of its first sub-story and describes the umbrella capability
that the sub-stories refine.

| ID | Persona |
|----|---------|
| US-13.9.1 | game designer (P-5) |
| US-13.9.10 | game developer (P-15) |
| US-13.9.2 | game designer (P-5) |
| US-13.9.3 | game designer (P-5) |
| US-13.9.4 | game designer (P-5) |
| US-13.9.5 | game designer (P-5) |
| US-13.9.6 | game designer (P-5) |
| US-13.9.7 | game designer (P-5) |
| US-13.9.8 | game designer (P-5) |
| US-13.9.9 | game designer (P-5) |

1. **US-13.9.1** -- **As a** game designer (P-5), **I want** the capabilities defined in sub-stories
   US-13.9.1.1 through US-13.9.1.2 combined into a single umbrella feature, **so that** I have a
   coherent parent story covering the refined child stories.

2. **US-13.9.10** -- **As a** game developer (P-15), **I want** the capabilities defined in
   sub-stories US-13.9.10.1 through US-13.9.10.2 combined into a single umbrella feature,
   **so that** I have a coherent parent story covering the refined child stories.

3. **US-13.9.2** -- **As a** game designer (P-5), **I want** the capabilities defined in sub-stories
   US-13.9.2.1 through US-13.9.2.2 combined into a single umbrella feature, **so that** I have a
   coherent parent story covering the refined child stories.

4. **US-13.9.3** -- **As a** game designer (P-5), **I want** the capabilities defined in sub-stories
   US-13.9.3.1 through US-13.9.3.1 combined into a single umbrella feature, **so that** I have a
   coherent parent story covering the refined child stories.

5. **US-13.9.4** -- **As a** game designer (P-5), **I want** the capabilities defined in sub-stories
   US-13.9.4.1 through US-13.9.4.2 combined into a single umbrella feature, **so that** I have a
   coherent parent story covering the refined child stories.

6. **US-13.9.5** -- **As a** game designer (P-5), **I want** the capabilities defined in sub-stories
   US-13.9.5.1 through US-13.9.5.2 combined into a single umbrella feature, **so that** I have a
   coherent parent story covering the refined child stories.

7. **US-13.9.6** -- **As a** game designer (P-5), **I want** the capabilities defined in sub-stories
   US-13.9.6.1 through US-13.9.6.2 combined into a single umbrella feature, **so that** I have a
   coherent parent story covering the refined child stories.

8. **US-13.9.7** -- **As a** game designer (P-5), **I want** the capabilities defined in sub-stories
   US-13.9.7.1 through US-13.9.7.2 combined into a single umbrella feature, **so that** I have a
   coherent parent story covering the refined child stories.

9. **US-13.9.8** -- **As a** game designer (P-5), **I want** the capabilities defined in sub-stories
   US-13.9.8.1 through US-13.9.8.2 combined into a single umbrella feature, **so that** I have a
   coherent parent story covering the refined child stories.

10. **US-13.9.9** -- **As a** game designer (P-5), **I want** the capabilities defined in
    sub-stories
US-13.9.9.1 through US-13.9.9.2 combined into a single umbrella feature, **so that** I have a
coherent parent story covering the refined child stories.
