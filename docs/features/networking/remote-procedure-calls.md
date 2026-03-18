# 8.3 — Remote Procedure Calls

## RPC Invocation

| ID      | Feature                         | Requirements |
|---------|---------------------------------|--------------|
| F-8.3.1 | Server RPC (Client-to-Server)   | R-8.3.1      |
| F-8.3.2 | Client RPC (Server-to-Client)   | R-8.3.2      |
| F-8.3.3 | Multicast RPC (Server-to-Group) | R-8.3.3      |

1. **F-8.3.1** — Allow clients to invoke validated procedures on the server, such as ability
   activation, item usage, or interaction requests. Server RPCs are the primary mechanism for client
   input in a server-authoritative architecture. Every server RPC must pass through validation and
   rate limiting before execution to prevent cheating and denial-of-service from compromised
   clients.
   - **Deps:** F-8.1.3, F-8.1.4
   - **Platform:** Mobile clients use the same RPC protocol. Rate limits may be tuned per platform
     to account for touch input cadence vs. keyboard/mouse.
2. **F-8.3.2** — Allow the server to invoke procedures on specific clients, such as triggering UI
   events, playing cosmetic effects, or delivering chat messages. Client RPCs complement state
   replication for ephemeral events that do not belong in persistent component state — a damage
   number popup or a loot roll notification only needs to fire once and does not need to be part of
   the replicated world snapshot.
   - **Deps:** F-8.1.3, F-8.1.4
   - **Platform:** Server may batch or throttle cosmetic Client RPCs to mobile clients under
     bandwidth pressure; gameplay-critical RPCs are never dropped.
3. **F-8.3.3** — Allow the server to invoke a procedure on a filtered set of clients — all clients
   in an area, all party members, all participants in a raid, or all players in a battleground
   instance. Multicast RPCs avoid the overhead of issuing individual client RPCs for events visible
   to many players, such as a world boss phase transition, a guild siege wall breach, or a zone-wide
   weather change.
   - **Deps:** F-8.3.2, F-8.2.3
   - **Platform:** Server-side fan-out; all platforms receive the same multicast. Mobile clients
     process multicast payloads identically to desktop.

## Reliability and Serialization

| ID      | Feature                                | Requirements |
|---------|----------------------------------------|--------------|
| F-8.3.4 | RPC Reliability Modes                  | R-8.3.4      |
| F-8.3.5 | Parameter Serialization and Validation | R-8.3.5      |

1. **F-8.3.4** — Support configurable reliability per RPC: reliable (guaranteed delivery, used for
   ability confirmations and trade completions), unreliable (fire-and-forget, used for cosmetic
   effects and hit markers), and reliable-latest (only the most recent invocation is delivered, used
   for rapidly-changing UI state like casting bar progress). Reliability mode is declared at RPC
   registration time and cannot be changed per-call.
   - **Deps:** F-8.3.1, F-8.3.2
   - **Platform:** Protocol-level feature; runs identically on all platforms. Mobile benefits from
     reliable-latest mode to reduce retransmission on lossy networks.
2. **F-8.3.5** — Serialize RPC parameters using the engine's reflection and serialization system
   with compact binary encoding, and validate all parameters server-side before execution.
   Validation includes type checking, range clamping, reference resolution (does the target entity
   exist and is the caller allowed to interact with it), and rate limiting. Malformed or
   out-of-bounds parameters must be rejected without crashing the server.
   - **Deps:** F-8.3.1
   - **Platform:** Compact binary encoding minimizes payload size, benefiting mobile clients with
     constrained bandwidth. Validation is server-side and platform-agnostic.
