# R-8.3 -- Remote Procedure Call Requirements

## RPC Invocation

1. **R-8.3.1** — The engine **SHALL** allow clients to invoke validated procedures on the server,
   where every server RPC passes through server-side validation and per-RPC-type rate limiting
   before execution, rejecting invalid or excessive invocations without crashing the server.
   - **Rationale:** Server RPCs are the primary client input mechanism in a server-authoritative
     architecture; all must be validated to prevent cheating and denial-of-service.
   - **Verification:** Integration test: invoke a server RPC with valid parameters and verify
     execution. Invoke with out-of-range parameters and verify rejection. Exceed the configured rate
     limit and verify throttling. Send a malformed payload and verify the server does not crash.
2. **R-8.3.2** — The engine **SHALL** allow the server to invoke procedures on specific clients for
   ephemeral events (UI triggers, cosmetic effects, chat messages) that do not belong in persistent
   replicated component state.
   - **Rationale:** Ephemeral events like damage popups and loot notifications are fire-once and
     should not consume replication bandwidth as persistent state.
   - **Verification:** Integration test: invoke a client RPC from the server targeting a specific
     client. Verify the target receives and executes it. Verify non-target clients do not receive
     the RPC. Verify the payload does not appear in the replicated state snapshot.
3. **R-8.3.3** — The engine **SHALL** allow the server to invoke a procedure on a filtered set of
   clients (area-based, party, raid, battleground) in a single operation, without issuing individual
   client RPCs per recipient.
   - **Rationale:** Events visible to many players must be delivered efficiently without per-client
     RPC overhead.
   - **Verification:** Integration test: multicast an RPC to 100 clients in a spatial area. Verify
     all 100 receive the RPC. Verify clients outside the area do not. Benchmark: compare CPU cost of
     multicast vs. 100 individual client RPCs and verify multicast is at least 5x more efficient.

## Reliability and Serialization

1. **R-8.3.4** — The engine **SHALL** support three reliability modes per RPC declared at
   registration time: reliable (guaranteed delivery), unreliable (fire-and-forget), and
   reliable-latest (only the most recent invocation is delivered), with the mode immutable after
   registration.
   - **Rationale:** Different RPC types have different delivery needs; ability confirmations require
     reliability while cosmetic effects tolerate loss.
   - **Verification:** Unit test per mode: (1) reliable -- send over a lossy link with 10% loss,
     verify delivery within retransmission timeout. (2) unreliable -- send 1,000, verify no
     retransmission. (3) reliable-latest -- send 10 rapid invocations, verify only the last is
     delivered.
2. **R-8.3.5** — The engine **SHALL** serialize RPC parameters using compact binary encoding via the
   engine's reflection system, and validate all parameters server-side (type checking, range
   clamping, reference resolution, rate limiting) before execution, rejecting malformed or
   out-of-bounds parameters without crashing.
   - **Rationale:** Compromised clients can send arbitrary payloads; the server must validate every
     parameter to prevent exploitation and maintain stability.
   - **Verification:** Unit test: serialize and deserialize an RPC with 5 parameter types (integer,
     float, string, entity reference, enum) and verify round-trip fidelity. Send an RPC referencing
     a non-existent entity and verify rejection. Send a truncated payload and verify the server does
     not crash.

## Non-Functional

1. **R-8.3.6** — The RPC system **SHALL** process at least 50,000 server RPCs per second per server
   process without exceeding the networking frame budget (default 0.5 ms at 60 fps).
   - **Rationale:** Large-scale PvP encounters generate bursts of ability activations that must be
     processed within the frame budget.
   - **Verification:** Benchmark: submit 50,000 RPCs per second with 5-parameter payloads. Verify
     all are validated and executed. Verify networking processing time stays within the 0.5 ms
     budget.
