# Marketplace distribution model

## Status

Accepted — 2025-04-22

## Context

Game engines typically ship marketplaces that take a 12–30% commission on every transaction
(Unity Asset Store: 30%, Unreal FAB: 12%). Commissions fund discovery, hosting, and payment
infrastructure. They also create revenue alignment problems with the engine vendor and with
asset sellers.

[PDR-0001](PDR-0001-apache2-no-royalties.md) commits to no marketplace commission. The
engine still needs an asset marketplace because it is a competitive expectation. Two models
are compatible with the no-commission posture:

1. **Open-source community-run repository.** Like crates.io for Rust. Free hosting, no
   commission, no payment processing in the engine. Sellers integrate with external payment
   providers themselves.
2. **External marketplace integration.** The engine integrates with FAB, Synty, TurboSquid,
   etc., from inside the editor. Those marketplaces handle their own pricing and commissions.
   The engine takes nothing.

Both models are compatible. The decision is whether to operate the OSS repo, integrate
external marketplaces, or both.

## Decision

The engine ships **both**:

- An open-source community-run repository (Git-based, free for browse / download / publish)
  modeled on crates.io.
- In-editor integrations with FAB, Synty, TurboSquid, and other third-party stores. Those
  stores handle their own pricing and commissions; the engine takes no cut.

Distribution and signing for the open-source repo:

| Concern                | Approach                                                  |
|------------------------|------------------------------------------------------------|
| Storage                | Garage (S3-compatible, Helm-deployed)                     |
| Index                  | TiKV                                                      |
| Signing                | Per-publisher GPG key; signature checked at install        |
| Dependency resolution  | Semver-aware solver (vendored from `cargo` if practical)   |
| Update path            | Atomic publish with rollback; idempotent re-publish        |

Plugin marketplace mechanics live in
[docs/design/tools/plugin-marketplace.md](../../design/tools/plugin-marketplace.md).

## Engineering implications

- The marketplace browser is part of the editor and ships open source.
- The repository runs on the same Helm chart used by every other server service.
- External marketplaces are accessed via published HTTP / QUIC APIs of those vendors. The
  engine never proxies their transactions.
- Search uses TiKV indexes plus app-layer query logic ([ADR-0009](../adr/ADR-0009-oss-kubernetes-stack.md)).
- Asset versioning ([F-12.6](../../features/content-pipeline/asset-versioning.md)) integrates
  with marketplace versioning so dependency upgrades are first-class.

## Reversibility

Adding a Harmonius-side commission later would violate [PDR-0001](PDR-0001-apache2-no-royalties.md)
and break trust. Treat the no-commission posture as one-way.

Removing the open-source repo and falling back to external-only is feasible but reduces the
out-of-box value proposition.

## References

- [docs/business/monetization-analysis.md](../../business/monetization-analysis.md) §1, §5
- [docs/design/tools/plugin-marketplace.md](../../design/tools/plugin-marketplace.md)
- [docs/features/content-pipeline/asset-versioning.md](../../features/content-pipeline/asset-versioning.md)
- [PDR-0001 Apache 2.0 no royalties](PDR-0001-apache2-no-royalties.md)
