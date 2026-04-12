# Containers and Slots User Stories

## Containers

| ID        | Persona               |
|-----------|-----------------------|
| US-16.2.1 | game developer (P-15) |
| US-16.2.2 | game designer (P-5)   |
| US-16.2.3 | gamer (P-23)          |
| US-16.2.4 | game designer (P-5)   |
| US-16.2.5 | gamer (P-23)          |

1. **US-16.2.1** — **As a** game developer (P-15), **I want** a generic Container component with
   configurable capacity, weight limit, and slot count, **so that** I can model inventories, chests,
   and loot bags with one primitive.
2. **US-16.2.2** — **As a** game designer (P-5), **I want** Tetris-style grid-layout containers with
   rectangular items and automatic bin-packing, **so that** I can author Diablo and Tarkov-style
   inventories without custom code.
3. **US-16.2.3** — **As a** gamer (P-23), **I want** stackable resources like potions and arrows to
   automatically merge into stacks in my inventory, **so that** my slots do not fill up with
   duplicates.
4. **US-16.2.4** — **As a** game designer (P-5), **I want** to configure a maximum container nesting
   depth so bag-in-bag chains cannot exceed it, **so that** serialization cost stays bounded and
   circular nesting cannot occur.
5. **US-16.2.5** — **As a** gamer (P-23), **I want** to sort my inventory by weight, rarity, or type
   with one click, **so that** I can find items quickly after picking up loot.

## Sockets

| ID        | Persona               |
|-----------|-----------------------|
| US-16.2.6 | game designer (P-5)   |
| US-16.2.7 | gamer (P-23)          |
| US-16.2.8 | character artist (P-9)|

1. **US-16.2.6** — **As a** game designer (P-5), **I want** typed attachment sockets with tag
   compatibility rules on entities, **so that** I can define equipment slots, weapon attachments,
   and gem sockets from data.
2. **US-16.2.7** — **As a** gamer (P-23), **I want** gems socketed into my weapon to increase its
   stats automatically, **so that** equipment customization directly affects combat performance.
3. **US-16.2.8** — **As a** character artist (P-9), **I want** items attached to sockets to snap
   into place with the correct transform offset and visual override, **so that** weapons in hand and
   helmets on head render at authored positions.

## Transfers

| ID         | Persona               |
|------------|-----------------------|
| US-16.2.9  | gamer (P-23)          |
| US-16.2.10 | game designer (P-5)   |

1. **US-16.2.9** — **As a** gamer (P-23), **I want** drag-and-drop item transfers to validate and
   complete in under 0.1 ms, **so that** inventory manipulation feels instant even on crowded
   pickups.
2. **US-16.2.10** — **As a** game designer (P-5), **I want** crafting recipes to consume ingredients
   atomically from containers on craft completion, **so that** failed crafts never partially consume
   materials.
