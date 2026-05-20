# Apache 2.0 core, no royalties, no commission

## Status

Accepted — 2025-01-15

## Context

Three competitive pressures shape the engine's licensing posture:

1. **Trust erosion in commercial engines.** The Unity Runtime Fee crisis of 2023 demonstrated
   that proprietary engines can change pricing retroactively. Studios that committed to a
   pricing tier could not unwind that commitment quickly.
2. **Open-source competitors are growing.** Godot and O3DE are gaining market share by being
   fully free. Bevy is gaining mindshare in the Rust community.
3. **Indie developer economics.** A 5% royalty above $1M (Unreal) or per-seat fees (Unity Pro)
   are real friction points for indie studios making their first commercial title.

The project must commit to a posture that maximizes adoption and developer trust while still
generating revenue.

## Decision

The engine is Apache 2.0 across all core components: engine, editor, headless game server, codegen
pipeline, all visual editors, all server services, and the deployment chart. There is no royalty at
any revenue band. There is no per-seat fee. There is no marketplace commission on the open-source
asset store. Third-party stores (FAB, Synty, TurboSquid) keep their own fees; the engine does not
insert itself into those transactions.

Revenue comes from optional services and proprietary console SDKs only:

- Console SDK addon licenses (PS5, Xbox, Switch) at $10K/year per platform
- Enterprise support contracts at $10K–$50K/year
- Managed hosting at $29/user/month (operates the same Helm chart customers can self-host)

## Engineering implications

- Every server component must be self-hostable from a Helm chart on any Kubernetes substrate.
  No back-doors that require Harmonius cloud.
- Console SDKs are the only proprietary components, justified by platform-holder NDAs.
- License headers and `LICENSE-APACHE` files appear at every crate root.
- Contributor License Agreement (CLA) policy must accommodate Apache 2.0 contributions
  (typically the ICLA / CCLA pattern).
- No engine code path may charge users for engine-side functionality. Cloud LLM access uses
  customer-owned API keys (PDR-0002), not a Harmonius-billed service.

## Reversibility

Reversing this decision would require:

- Re-licensing every contributed file (impossible without contributor consent for Apache 2.0
  contributions made in good faith).
- Breaking the trust contract with the community.

Effectively, this decision is one-way. New revenue streams must fit *inside* the free-engine
constraint, not relax it.

## References

- [docs/business/monetization-analysis.md](../../business/monetization-analysis.md) §0
- [docs/business/go-to-market.md](../../business/go-to-market.md) §3
- [ADR-0009 OSS Kubernetes stack](../adr/ADR-0009-oss-kubernetes-stack.md)
