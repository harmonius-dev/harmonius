# Console SDK server-side build farm strategy

## Status

Accepted — 2025-03-01

## Context

Shipping to PS5, Xbox, and Nintendo Switch requires platform-specific SDKs that are bound by
Non-Disclosure Agreements (NDAs). NDAs prohibit:

- Distributing the SDK source publicly.
- Bundling the SDK in a public open-source release.
- Allowing the SDK headers or libraries to be inspected by non-licensed developers.

A pure open-source engine cannot ship console SDKs in the public binary. Customers building for
consoles must obtain platform-holder licenses themselves.

The engine has two architectural choices:

1. **Per-developer SDK install.** Each customer downloads the platform-holder SDK after
   licensing, integrates it with their local engine build, and ships titles from their own
   workstations.
2. **Server-side build farm.** Harmonius operates a server-side build farm running the
   platform-holder SDKs. Customers submit code; the farm produces signed builds. SDK contents
   never reach customer machines.

## Decision

Console SDKs are the only proprietary components in the Harmonius product
([PDR-0001](PDR-0001-apache2-no-royalties.md)). They are delivered via a server-side build farm. The
engine ships an open-source `console-build` client; the farm holds the SDKs.

Annual license fees (per platform):

| Platform | Fee       |
|----------|-----------|
| PS5      | $10K/year |
| Xbox     | $10K/year |
| Switch   | $10K/year |

The license covers NDA compliance, 24/7 support during certification, and platform updates.

## Engineering implications

- The engine provides a `console-build` CLI that submits a build job to the farm and downloads
  the signed result. The CLI is open source.
- The farm itself is operated by Harmonius (or a partner) and is the only place SDK headers
  and libraries are linked into the build.
- Customers retain full source-code access to their game; the SDK linkage happens server-side.
- Open-source contributors do not need access to console SDKs. The farm is the contractual
  surface.
- Platform holders' NDA terms typically require named individuals to be cleared. The farm
  operator becomes the named entity.
- Console-specific feature flags in the engine source are gated by `cfg(target_os = …)` and
  ship as no-ops in open-source builds.

## Reversibility

Could revert to per-developer SDK install if a platform holder permits broader distribution.
Unlikely in the foreseeable future. The build-farm choice is also a customer-experience positive —
customers do not need to manage SDK environment setup.

## References

- [docs/business/monetization-analysis.md](../../business/monetization-analysis.md) Component 25
- [docs/design/platform/console-integration.md](../../design/platform/console-integration.md)
- [PDR-0001 Apache 2.0 no royalties](PDR-0001-apache2-no-royalties.md)
