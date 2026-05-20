# No async/await in engine, editor, or headless game server

## Status

Accepted — 2024-10-02 (backfilled 2026-05-20)

## Context

Tokio and other async runtimes are the standard Rust pattern for I/O-bound concurrency. They
introduce a runtime, a poll loop, and a `Future`-typed surface over every API. For a game
engine driving a tight frame loop, async runtimes add three problems:

1. **Thread topology conflict.** Tokio's multi-threaded runtime spawns its own workers, which
   compete with the engine's job system for cores. Pinning, QoS, and core-affinity rules
   collide.
2. **Latency unpredictability.** Async wake-up latency depends on the runtime's poll frequency
   and on which worker thread the task ends up on. Frame-time budgets cannot tolerate this.
3. **API contagion.** Every `await` propagates `async` upward. Eventually most APIs are
   `async fn`, which infects user-facing system code that should be plain synchronous Rust.

Backend services on Kubernetes (matchmaker, GameDb, control panel, K8s operator) are
different — they are standard server workloads that benefit from async I/O.

## Decision

There is no `async fn`, `.await`, `Future<>`, `AsyncMutex`, `AsyncRwLock`, or async runtime in
the engine, editor, or headless game server. All engine code is synchronous Rust.

Async I/O is performed via platform-native completion-based APIs (io_uring on Linux, IOCP on
Windows, GCD `dispatch_io` on Apple). Completions arrive at the main thread, which posts jobs
to the worker pool via `crossbeam-channel`.

Backend services on Kubernetes are exempt: they may use Tokio, hyper, axum, etc. The boundary
between engine (sync) and backend (async) is the QUIC transport.

## Consequences

- The engine has no Tokio dependency. Tokio ecosystem crates (`reqwest`, `sqlx`, `quinn`,
  `tokio-tungstenite`, `hyper`, `axum`) are not permitted in engine, editor, or headless
  server crates.
- A custom `CancelToken` (atomic + waker list) replaces `tokio_util::CancellationToken`.
- All user-facing engine APIs are synchronous. Long operations expose a request/handle pattern
  instead of `await`.
- Backend services use Tokio normally; the constraint applies to game-side code only.
- Earlier reviews recommended replacing Tokio with `compio` and were superseded; `compio` is
  also rejected because it spawns one thread per core, doubling the thread count when combined
  with the job system.

## Alternatives Considered

- **Tokio engine-wide** — incompatible with frame-time budgets and the custom job system.
- **`compio` engine-wide** — rejected for the per-core thread proliferation issue noted
  above.
- **Custom async runtime** — would solve the thread proliferation but reintroduces
  `await`-based contagion and adds a complex maintenance burden for negligible benefit on
  platform-native completion-based APIs.

## References

- [docs/design/constraints.md](../../design/constraints.md) "CPU Parallelism"
- [docs/design/core-runtime/io.md](../../design/core-runtime/io.md)
- [docs/design/core-runtime/memory-async-io.md](../../design/core-runtime/memory-async-io.md)
