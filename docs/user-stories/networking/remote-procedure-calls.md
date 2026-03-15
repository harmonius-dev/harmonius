# User Stories -- 8.3 Remote Procedure Calls

## US-8.3.1 Fire Abilities Without Server Exploits

**As a** game developer (P-15), **I want** every client-to-server RPC to be validated and
rate-limited before execution, **so that** compromised clients cannot bypass cooldowns,
duplicate items, or crash the server with malformed payloads.

## US-8.3.2 Broadcast Zone Events Efficiently

**As a** game developer (P-15), **I want** to multicast an RPC to all clients in an area
with a single call instead of looping over individual client RPCs, **so that** zone-wide
events like world boss phase transitions are delivered efficiently without per-client
overhead.

## US-8.3.3 Trigger Ephemeral Client Effects from the Server

**As a** game developer (P-15), **I want** server-to-client RPCs for one-shot events like
damage number popups, loot roll notifications, and cosmetic effects, **so that** ephemeral
events fire without cluttering persistent replicated state.

## US-8.3.4 See Visual Effects for Zone-Wide Events

**As a** player (P-23), **I want** world boss phase transitions, siege wall breaches, and
weather changes to reach my client reliably through multicast RPCs, **so that** I
experience dramatic game events without missing visual or audio cues.

## US-8.3.5 Choose the Right Reliability Mode for Each RPC

**As a** game developer (P-15), **I want** configurable reliability per RPC (reliable,
unreliable, reliable-latest), **so that** ability confirmations are guaranteed to arrive,
cosmetic effects tolerate loss, and rapidly-updating UI state delivers only the latest
value.

## US-8.3.6 Serialize and Validate RPC Parameters Securely

**As an** engine developer (P-26), **I want** compact binary serialization with server-side
type checking, range clamping, and reference resolution for all RPC parameters, **so that**
malformed or out-of-bounds payloads are rejected without crashing the server and bandwidth
is minimized.

## US-8.3.7 Handle RPC Traffic on Mobile Bandwidth Budgets

**As a** player (P-23), **I want** the server to batch or throttle cosmetic client RPCs
when my mobile connection is under bandwidth pressure, **so that** gameplay-critical RPCs
always arrive while cosmetic effects degrade gracefully rather than causing lag spikes.

## US-8.3.8 Monitor RPC Throughput and Rejection Rates

**As a** server admin (P-22), **I want** per-RPC-type metrics showing invocation rate,
validation rejection rate, and average payload size, **so that** I can detect RPC abuse,
identify overly chatty systems, and tune rate limits for production traffic patterns.

## US-8.3.9 Verify RPC Validation Rejects All Malformed Payloads

**As an** engine tester (P-27), **I want** fuzz tests that send randomized and intentionally
malformed RPC payloads to the server, **so that** I can confirm the validation layer rejects
every invalid payload without crashing, leaking memory, or executing unintended logic.

## US-8.3.10 Verify Multicast Reaches All Intended Recipients

**As an** engine tester (P-27), **I want** automated tests that verify multicast RPCs reach
exactly the intended client set (area, party, raid, battleground), **so that** no client is
missed and no unintended client receives the message.
