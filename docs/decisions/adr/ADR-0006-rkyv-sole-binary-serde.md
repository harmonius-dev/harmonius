# rkyv as sole binary serialization

## Status

Accepted — 2024-11-06 (backfilled 2026-05-20)

## Context

The engine writes save files, baked assets, and network state. Each path needs a binary
serialization library. Common Rust choices include `serde` (with `bincode` or `postcard`),
`rkyv`, `prost` (Protobuf), and `flatbuffers`.

Serde is the default, but its `Deserialize` step copies bytes into Rust objects, which is
expensive for large baked assets (meshes, textures, audio, scene state). Network paths also
prefer zero-copy or minimal-copy decoding.

`rkyv` provides zero-copy archive access via mmap: the binary layout *is* the in-memory
representation. Validation is performed once at archive load; reads after that are pointer
chases.

## Decision

`rkyv` is the sole binary serialization library for the engine. Save files, baked assets, and
network payloads all use rkyv.

Serde is banned engine-wide except in tools that emit JSON for external consumers (rumdl,
package metadata, editor preferences stored as human-readable text).

A custom Harmonius-specific text format covers scene files. It is designed for line-based
diffing and merge conflict resolution (not BSN, not RON). Mixed text + binary serialization is
allowed: text scene files reference companion `.bin` files in the same directory.

## Consequences

- Zero-copy save and asset loading. Mmap the archive, validate once, treat the buffer as the
  live data.
- Every persistent struct must derive rkyv `Archive`, `Serialize`, `Deserialize`. Versioning
  uses rkyv's archive validation plus the migration registry.
- Network protocols that historically used Protobuf or FlatBuffers ride on rkyv archives over
  QUIC streams ([ADR-0010](ADR-0010-quic-unified-transport.md)).
- Tools, editor preferences, and CI metadata that already use JSON via serde keep using
  serde-json with the explicit "tool only" carve-out.

## Alternatives Considered

- **Bincode + serde** — straightforward but pays a deserialization cost for every load.
- **Protobuf via prost** — schema versioning support is nice, but adds a protobuf compiler to
  the build and is overkill for engine-internal data.
- **FlatBuffers** — also zero-copy, but the schema language is C++-flavoured and the Rust
  bindings are less idiomatic than rkyv's derive macros.

## References

- [docs/design/constraints.md](../../design/constraints.md) "Serialization"
- [docs/design/core-runtime/reflection-serialization.md](../../design/core-runtime/reflection-serialization.md)
- [docs/design/integration/save-system-serialization.md](../../design/integration/save-system-serialization.md)
