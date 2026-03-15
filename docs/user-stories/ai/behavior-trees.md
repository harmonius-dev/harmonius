# User Stories -- 7.3 Behavior Trees

## US-7.3.1 Iterate on NPC Behavior Without Restarting
**As an** AI designer, **I want** to edit a behavior tree data asset and see the change
reflected on live NPCs within 1 second via hot-reload, **so that** I can rapidly iterate on
NPC behavior without restarting the server.

## US-7.3.2 Reuse Common Behavior Patterns
**As an** AI designer, **I want** to define a "patrol" subtree once and reference it from
many NPC archetypes, **so that** updating the shared patrol logic fixes all NPCs at once
without editing each tree individually.
