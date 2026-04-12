# Networking ↔ Physics Integration Test Cases

## Integration Tests

| ID | Test | Input | Expected | Req |
|----|------|-------|----------|-----|
| TC-IR-4.5.1.1 | Server physics state replicated | Server moves body | Client new pos | IR-4.5.1 |
| TC-IR-4.5.1.2 | Client cannot modify body | Client writes Velocity | Server ignores | IR-4.5.1 |
| TC-IR-4.5.2.1 | Prediction applies local input | Input at tick T | Body moves now | IR-4.5.2 |
| TC-IR-4.5.2.2 | Prediction uses same substep | Client pipeline | Matches server | IR-4.5.2 |
| TC-IR-4.5.3.1 | Rollback corrects mismatch | Server pos delta 0.5 | Corrected | IR-4.5.3 |
| TC-IR-4.5.3.2 | Rollback replays N ticks | Mismatch T, cur T+5 | 5 resim | IR-4.5.3 |
| TC-IR-4.5.3.3 | Rollback cap prevents spike | Mismatch T-20 | Max 10 replay | IR-4.5.3 |
| TC-IR-4.5.4.1 | Cross-platform determinism | Same input tri-OS | Bit-identical | IR-4.5.4 |
| TC-IR-4.5.4.2 | No fast-math in physics | Compile flags | ffast-math off | IR-4.5.4 |
| TC-IR-4.5.5.1 | Hitbox rewind to past tick | Rewind 100 ms | Past positions | IR-4.5.5 |
| TC-IR-4.5.5.2 | Raycast hits rewound hitbox | Aim at past pos | Hit confirmed | IR-4.5.5 |
| TC-IR-4.5.5.3 | Raycast misses current pos | Aim past, moved | Miss cur, rewind hit | IR-4.5.5 |
| TC-IR-4.5.6.1 | Interpolation smooths remote | Two snaps 50 ms | Smooth between | IR-4.5.6 |
| TC-IR-4.5.6.2 | Extrapolation extends velocity | Snap late 1 tick | Extrapolated | IR-4.5.6 |
| TC-IR-4.5.7.1 | Physics snapshot captures all | Snapshot tick T | pos/rot/vel/ang | IR-4.5.7 |
| TC-IR-4.5.7.2 | Snapshot restore is exact | Restore tick T | State == orig | IR-4.5.7 |

## Negative Tests

| ID | Test | Input | Expected | Req |
|----|------|-------|----------|-----|
| TC-IR-4.5.1.N1 | Malformed body packet | Junk bytes | Packet rejected | IR-4.5.1 |
| TC-IR-4.5.3.N1 | Rollback snapshot evicted | Tick T > buf len | Returns None | IR-4.5.3 |
| TC-IR-4.5.3.N2 | Rollback beyond cap | Mismatch T-50 | Clamped resync | IR-4.5.3 |
| TC-IR-4.5.5.N1 | Hitbox tick out of range | Tick < buf start | Reject rewind | IR-4.5.5 |
| TC-IR-4.5.5.N2 | Hitbox entity unknown | Bogus entity | Returns None | IR-4.5.5 |
| TC-IR-4.5.6.N1 | Interpolate same snapshot | from == to | Returns from | IR-4.5.6 |
| TC-IR-4.5.6.N2 | Extrapolate past max dt | dt > max | Clamped freeze | IR-4.5.6 |
| TC-IR-4.5.7.N1 | Buffer overflow push | 2x cap pushes | Oldest evicted | IR-4.5.7 |

Detail notes:

1. TC-IR-4.5.1.N1 feeds random bytes through the transport layer; decoder must return a decode error
   and leave physics state untouched.
2. TC-IR-4.5.3.N1 verifies `SnapshotBuffer::get` returns `None` when the requested tick is older
   than `head - len`; reconciler falls back to a full-state resync.
3. TC-IR-4.5.3.N2 verifies that when the mismatch tick is older than `max_rollback_ticks`, the
   reconciler requests a full state sync from the server instead of rolling back.
4. TC-IR-4.5.5.N1 and TC-IR-4.5.5.N2 verify that `HitboxBuffer::get` returns `None` rather than
   panicking on missing entries; the hit validator treats this as a miss.
5. TC-IR-4.5.6.N1 verifies interpolator handles the alpha=0 / alpha=1 / identical-snapshot cases.
6. TC-IR-4.5.6.N2 verifies the extrapolator freeze fallback documented in the design.
7. TC-IR-4.5.7.N1 verifies ring buffer wrap-around behavior.

## Cross-Platform Determinism Orchestration

TC-IR-4.5.4.1 runs in three CI-compatible modes so it can be executed even without GitHub Actions:

1. **Local (developer) mode.** `cargo test -p net-physics --test determinism -- --nocapture` runs
   the test on the current OS, captures a SHA-256 hash of the final body positions over 1000 fixed
   ticks, and writes it to `target/determinism/<os>-<arch>.hash`.
2. **Multi-target mode.** A small Rust test runner (`xtask determinism`) cross-compiles the
   deterministic-physics test binary for `x86_64-pc-windows-msvc`, `x86_64-unknown-linux-gnu`,
   `aarch64-apple-darwin`, and invokes each binary through `cargo-cross` (Linux), Wine (Windows on
   Linux CI), and the local macOS host. Each run produces a hash file as in mode 1.
3. **Comparison step.** `xtask determinism verify` reads all hash files under `target/determinism/`
   and fails if any two differ, printing the diverging byte offset.

CI integration is optional -- the same commands run in any shell, locally or under any CI system,
without requiring GitHub Actions. When GitHub Actions is eventually wired up, the matrix simply
invokes `xtask determinism` on each runner and uploads the hash file as an artifact; a final job
downloads artifacts and runs `xtask determinism verify`.

## Benchmarks

| ID | Benchmark | Target | Req |
|----|-----------|--------|-----|
| TC-IR-4.5.3.B1 | Rollback 10 ticks, 100 bodies | < 4 ms | IR-4.5.3 |
| TC-IR-4.5.5.B1 | Hitbox rewind 64 entities | < 0.2 ms | IR-4.5.5 |
| TC-IR-4.5.6.B1 | Interpolation 1000 remote bodies | < 0.3 ms | IR-4.5.6 |
| TC-IR-4.5.7.B1 | Snapshot capture 2000 bodies | < 0.5 ms | IR-4.5.7 |
| TC-IR-4.5.7.B2 | Snapshot restore 2000 bodies | < 0.5 ms | IR-4.5.7 |
| TC-IR-4.5.4.B1 | Island solver sort 10k bodies | < 0.3 ms | IR-4.5.4 |
