# Managed-hosting tier policy and pricing

## Status

Accepted — 2025-03-25

## Context

The Apache 2.0 + no-royalty + no-commission posture ([PDR-0001](PDR-0001-apache2-no-royalties.md))
commits to free engine and free self-hosting. The engine still needs revenue. Three sources are
committed: console SDK licenses, enterprise support, and managed hosting.

Managed hosting must satisfy three constraints:

1. **No lock-in.** Customers running managed hosting must be able to migrate to self-hosting
   at any time without code changes. The Helm chart is the same.
2. **Sustainable margin.** Per-user price needs to cover infrastructure plus operational
   overhead plus profit, even at small scale.
3. **Clear value vs self-hosting.** For solo developers, the price must be competitive with
   the substrate cost of self-hosting on a small VM. For larger teams, the value comes from
   reduced operational burden.

## Decision

Managed hosting is offered at **$29/user/month**. It runs the same Helm chart customers can
self-host (TiKV, Garage, Pingora, Vector, Prometheus, Grafana, Loki, Keycloak, Forgejo). The
substrate is operated by Harmonius (or a delegated SRE partner).

Tier inclusions:

| Inclusion                          | Status                              |
|------------------------------------|-------------------------------------|
| All engine cloud services          | Included                            |
| Build cache + collaboration        | Included                            |
| Marketplace + mod hosting          | Included                            |
| AI assistant (customer keys)       | Customer pays provider directly     |
| 99.9% SLA                          | Standard                            |
| US/EU region selection             | Standard                            |
| Migration to self-host             | Supported at any time, no fees      |

The earlier per-user tiering used three separate subscriptions for collaboration, AI, and enterprise
(priced at $10, $20, and
$50 per user per month respectively). That model is superseded by the single $29/user/month tier.
Enterprise support remains separate at $10K–$50K/year for studios with shipping deadlines who need
SLA guarantees beyond the standard 99.9%.

## Engineering implications

- Managed hosting is a deployment of the open-source Helm chart, not a separate codebase.
  Drift between hosted and self-host paths is forbidden.
- Per-user cost model rebases on Kubernetes node sizing plus OSS bundle resource usage.
  Tracked under [BL-0044](../../backlog/issues/BL-0044-thin-requirement-files.md) and the
  [2026-Q3 OKRs](../../okrs/2026-q3.md).
- Customer data export must be one command, exporting to the Helm-chart-compatible format so
  any customer can take their data elsewhere.
- Pricing is reviewed quarterly; changes apply forward-only with 90 days notice.

## Reversibility

Pricing changes are forward-only and contractually limited. Discontinuing managed hosting is
possible because customers can always self-host the same Helm chart — it is the no-lock-in guarantee
that protects them.

## References

- [docs/business/monetization-analysis.md](../../business/monetization-analysis.md) §1, §2
- [docs/business/go-to-market.md](../../business/go-to-market.md) §3
- [ADR-0009 OSS Kubernetes stack](../adr/ADR-0009-oss-kubernetes-stack.md)
- [PDR-0001 Apache 2.0 no royalties](PDR-0001-apache2-no-royalties.md)
