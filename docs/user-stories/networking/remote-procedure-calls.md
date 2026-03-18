# User Stories -- 8.3 Remote Procedure Calls

| ID        | Persona                 | Features | Requirements |
|-----------|-------------------------|----------|--------------|
| US-8.3.1  | game developer (P-15)   | —        | —            |
| US-8.3.2  | game developer (P-15)   | —        | —            |
| US-8.3.3  | game developer (P-15)   | —        | —            |
| US-8.3.4  | player (P-23)           | —        | —            |
| US-8.3.5  | game developer (P-15)   | —        | —            |
| US-8.3.6  | engine developer (P-26) | —        | —            |
| US-8.3.7  | player (P-23)           | —        | —            |
| US-8.3.8  | server admin (P-22)     | —        | —            |
| US-8.3.9  | engine tester (P-27)    | —        | —            |
| US-8.3.10 | engine tester (P-27)    | —        | —            |

1. **US-8.3.1** — As a game developer (P-15), I want every client-to-server RPC to be validated and
   rate-limited before execution, so that compromised clients cannot bypass cooldowns, duplicate
   items, or crash the server with malformed payloads.
   - **Acceptance:** —
2. **US-8.3.2** — As a game developer (P-15), I want to multicast an RPC to all clients in an area
   with a single call instead of looping over individual client RPCs, so that zone-wide events like
   world boss phase transitions are delivered efficiently without per-client overhead.
   - **Acceptance:** —
3. **US-8.3.3** — As a game developer (P-15), I want server-to-client RPCs for one-shot events like
   damage number popups, loot roll notifications, and cosmetic effects, so that ephemeral events
   fire without cluttering persistent replicated state.
   - **Acceptance:** —
4. **US-8.3.4** — As a player (P-23), I want world boss phase transitions, siege wall breaches, and
   weather changes to reach my client reliably through multicast RPCs, so that I experience dramatic
   game events without missing visual or audio cues.
   - **Acceptance:** —
5. **US-8.3.5** — As a game developer (P-15), I want configurable reliability per RPC (reliable,
   unreliable, reliable-latest), so that ability confirmations are guaranteed to arrive, cosmetic
   effects tolerate loss, and rapidly-updating UI state delivers only the latest value.
   - **Acceptance:** —
6. **US-8.3.6** — As an engine developer (P-26), I want compact binary serialization with
   server-side type checking, range clamping, and reference resolution for all RPC parameters, so
   that malformed or out-of-bounds payloads are rejected without crashing the server and bandwidth
   is minimized.
   - **Acceptance:** —
7. **US-8.3.7** — As a player (P-23), I want the server to batch or throttle cosmetic client RPCs
   when my mobile connection is under bandwidth pressure, so that gameplay-critical RPCs always
   arrive while cosmetic effects degrade gracefully rather than causing lag spikes.
   - **Acceptance:** —
8. **US-8.3.8** — As a server admin (P-22), I want per-RPC-type metrics showing invocation rate,
   validation rejection rate, and average payload size, so that I can detect RPC abuse, identify
   overly chatty systems, and tune rate limits for production traffic patterns.
   - **Acceptance:** —
9. **US-8.3.9** — As an engine tester (P-27), I want fuzz tests that send randomized and
   intentionally malformed RPC payloads to the server, so that I can confirm the validation layer
   rejects every invalid payload without crashing, leaking memory, or executing unintended logic.
   - **Acceptance:** —
10. **US-8.3.10** — As an engine tester (P-27), I want automated tests that verify multicast RPCs
    reach exactly the intended client set (area, party, raid, battleground), so that no client is
    missed and no unintended client receives the message.
    - **Acceptance:** —
