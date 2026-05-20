# K8s + TiKV + Garage + Pingora OSS stack

## Status

Accepted — 2025-02-11 (backfilled 2026-05-20)

## Context

Game backend services (matchmaker, GameDb, control panel, mesh controller, leaderboard, cloud save,
telemetry) traditionally rely on a mix of managed cloud services: AWS RDS (PostgreSQL), ElastiCache
(Redis), DynamoDB, S3, CloudFront, OpenSearch, plus a CI service and a managed Kubernetes substrate.
This works but binds shippable infra to a specific cloud and prevents self-hosting.

The Harmonius project is fully open source and explicitly self-hostable. Console SDK builds must run
on shared build servers anyone can deploy. The marketplace, analytics, audit, and AI routing are
OSS. Customers deploy on EKS / GKE / AKS / k3s / bare metal interchangeably.

## Decision

The canonical server stack is OSS and runs on Kubernetes:

| Layer                  | Service                                          |
|------------------------|--------------------------------------------------|
| Database               | TiKV (replaces RDS, Redis/Valkey, DynamoDB)      |
| Object storage         | Garage (S3-compatible; replaces S3, MinIO Pro)   |
| CDN / reverse proxy    | Pingora (replaces CloudFront, CloudFlare)        |
| Logs                   | Vector → Loki                                    |
| Metrics                | Prometheus + Grafana                             |
| Auth                   | Keycloak                                         |
| Git                    | Forgejo                                          |
| CI                     | Forgejo Actions                                  |
| Orchestration          | Kubernetes + custom kube-rs operator             |
| Secrets / TLS          | Sealed Secrets, cert-manager                     |

The Helm chart `harmonius-server/` deploys this bundle. The custom kube-rs operator handles
game-aware deployment, canary validation, and graceful player drain.

There are no AWS-specific dependencies in the default stack. AWS is a permitted substrate for
managed hosting customers — same Helm chart, just running on EKS.

## Consequences

- Customers can self-host the entire backend on any Kubernetes substrate. No vendor lock-in.
- Database migrations from PostgreSQL to TiKV affect every service that previously assumed
  SQL. The application layer adapts to TiKV's KV + transaction + range-scan model.
- Search functionality previously delegated to OpenSearch is rebuilt on TiKV indexes plus
  app-layer query logic.
- CDN egress costs depend on the substrate, not on a fixed CloudFront contract.
- Observability stack is Vector + Loki + Prometheus + Grafana. CloudWatch is not used.
- Console SDK builds run on shared build servers in customer-controlled clusters.
- Per-user managed-hosting cost models that previously used AWS list pricing must be rebuilt on K8s
  node sizing; tracked under [BL-0044](../../backlog/issues/BL-0044-thin-requirement-files.md) and
  [docs/okrs/2026-q3.md](../../okrs/2026-q3.md).

## Alternatives Considered

- **AWS-only** — rejected by the open-source / self-host requirement.
- **PostgreSQL + Redis + S3 + custom CDN** — works but requires four separate operational
  surfaces. TiKV unifies database and cache into one HA system.
- **Cassandra / ScyllaDB** — fast but lacks native transactions; TiKV's MVCC + transactions
  cover the project's consistency needs.

## References

- [docs/design/constraints.md](../../design/constraints.md) "Infrastructure"
- [docs/design/networking/network-infrastructure.md](../../design/networking/network-infrastructure.md)
- [docs/requirements/networking/mmo-infrastructure.md](../../requirements/networking/mmo-infrastructure.md)
  R-8.7.13–14
- [docs/business/monetization-analysis.md](../../business/monetization-analysis.md) §1
