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
