# Transport and Replication

Network transmission, object synchronization, and bandwidth optimization.

## What it covers

- Protocol: UDP or TCP transport layer selection.
- Packet structure: headers, message framing, checksum validation.
- Serialization: encoding entities and components for transmission.
- Bandwidth optimization: delta compression, quantization, lossy coding.
- Entity replication: synchronizing entity state across clients.
- Property replication: selectively replicating changed properties.
- Relevance culling: only sending entities relevant to each player.
- Interpolation on client: smoothing received positions between updates.
- Extrapolation: predicting future position based on velocity.
- Connection management: handshake, heartbeats, timeout detection.

## Concepts

### Transport Protocol

Most games use UDP for low latency; TCP adds reliability overhead. UDP sends packets without
delivery guarantees; game code adds custom reliability for critical data. Packet structure includes
sequence number (for ordering), timestamp (for latency measurement), and message payload.

### Serialization and Bandwidth

Entities serialize to binary: position (3 floats) quantizes to bytes; rotation to 2 bytes (assume
sphere normal). Delta compression sends only changed properties: if position hasn't changed, omit it.
This reduces bandwidth from every-property-every-frame to changed-properties-only.

### Replication and Relevance

Server maintains authoritative entity state. Each client receives a subset of entities (relevance
set): nearby entities, team members, important entities (bosses). Entities outside relevance range
don't transmit, reducing bandwidth. When an entity enters relevance range, server sends full state
(spawn); when it exits, server sends destroy.

### Interpolation and Prediction

Client receives new entity position approximately 100ms apart (10 updates/sec). Linear interpolation
blends between received positions over time, producing smooth motion. Extrapolation predicts future
position if no new data arrives: predicted = current + velocity × time_since_update. This reduces
visible latency.

### Connection Lifecycle

Handshake establishes connection: client sends connect request; server acknowledges. Heartbeats
(empty packets) detect dead connections: if no packets for N seconds, connection times out. Client
reconnects automatically. Graceful disconnect sends disconnect message before closing.

## How it fits

- See [session-and-prediction.md](./session-and-prediction.md) for session state and prediction
  systems.
- See [communication-and-replay.md](./communication-and-replay.md) for RPC and replay systems.
- See [../core-runtime/data-and-io.md](../core-runtime/data-and-io.md) for serialization.
