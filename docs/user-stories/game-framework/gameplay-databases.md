# User Stories — 13.7 Gameplay Databases

## US-13.7.1 Tune Game Balance Without Restarting the Server

**As a** designer, **I want** to edit gameplay data tables (damage values, loot weights, prices)
in a spreadsheet and hot-reload them into a running game session, **so that** I can iterate on
balance in real time without restarting or redeploying.

## US-13.7.2 Define Item Hierarchies With Inherited Properties

**As a** designer, **I want** to create item type hierarchies (Item > Weapon > Sword >
FireSword) where child rows inherit parent values and override only what differs, **so that**
I avoid duplicating data across thousands of similar items.
