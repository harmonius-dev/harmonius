# QUIC unified transport

## Status

Accepted — 2025-03-12 (backfilled 2026-05-20)

## Context

Game networking historically uses TCP for reliable channels (HTTP, REST, WebSocket) and UDP
plus a custom reliability layer (ENet, RakNet, custom rollback) for the game loop. Voice chat
adds a third transport. Matchmaking uses HTTPS. The engine evaluated this stack and found that
running four transport types compounds operational complexity (firewall holes, NAT traversal
strategies, monitoring, congestion control) without strong technical motivation.

QUIC (IETF RFC 9000) is a modern UDP-based protocol that provides:

- Reliable streams plus unreliable datagrams in a single connection.
- Built-in TLS 1.3 with 0-RTT resumption.
- Multiplexed streams without head-of-line blocking.
- Connection migration across IP changes.

Mature implementations exist on every target platform: `quinn-proto` (sans-io Rust QUIC),
Apple's `Networking.framework`, and Microsoft's MsQuic.

## Decision

All network communication uses QUIC. There is no TCP, no custom UDP, no separate DTLS or
HTTPS connections, no WebSocket layer. Game state replication, asset streaming, HTTP-style
API calls, voice chat, and matchmaking all run over QUIC streams or datagrams on the same
connection.

Platform implementations:

| Platform | QUIC implementation                       |
|----------|-------------------------------------------|
| Linux    | `quinn-proto` (pure Rust)                 |
| Apple    | `Networking.framework` via `objc2`        |
| Windows  | MsQuic via `windows-rs`                   |

The QUIC layer is consumed via a sans-io adapter on the game side (synchronous, no async
runtime per [ADR-0004](ADR-0004-no-async-in-engine.md)). Backend services on Kubernetes can
use Tokio + `quinn` if convenient.

## Consequences

- One protocol, one connection per session. Operational simplicity.
- Custom reliability layer is replaced by QUIC streams. Rollback netcode rides on unreliable
  datagrams plus tick-stamped reliable acks.
- TLS is mandatory. Plaintext development connections require explicit deviation.
- Voice chat shares the QUIC connection with state replication. Codec and spatialization live
  in audio; transport and stream multiplexing live in networking.
  See [BL-0003](../../backlog/issues/BL-0003-voicestream-ownership.md) and
  [BL-0022](../../backlog/issues/BL-0022-quic-stream-multiplexing.md).
- Backwards compatibility with TCP-only firewalls requires QUIC-over-TCP fallback or an HTTPS
  proxy, both tracked under
  [BL-0022](../../backlog/issues/BL-0022-quic-stream-multiplexing.md).

## Alternatives Considered

- **TCP + custom UDP + WebSocket + HTTPS** — operational complexity is the dominant cost.
- **WebTransport** — useful for browser clients; engine targets desktop and console primarily.
  WebTransport rides on QUIC anyway, so adoption later is straightforward.
- **gRPC over HTTP/2** — works for backend RPC but overkill for game state replication; would
  add a second transport and second protocol.

## References

- [docs/design/constraints.md](../../design/constraints.md) "Networking"
- [docs/design/networking/network-transport.md](../../design/networking/network-transport.md)
- [docs/design/networking/network-services.md](../../design/networking/network-services.md)
