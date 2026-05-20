# Customer-owned AI API keys

## Status

Accepted — 2025-02-04

## Context

Cloud LLM features (asset generation prompts, scripting assistant, content moderation) require
either:

1. **Engine-side billing.** Harmonius proxies LLM calls, charges customers per call, and
   manages API keys to providers (OpenAI, Anthropic, Google).
2. **Customer-side billing.** Customers configure their own API keys per project; the engine
   is a thin client that calls the provider directly.

Engine-side billing introduces:

- Margin on every LLM call (revenue), but also pricing risk if providers raise costs.
- Privacy questions about prompt content flowing through Harmonius servers.
- Proxy infrastructure to operate (high-availability LLM gateway, rate limiting, abuse
  prevention, billing pipeline).
- Lock-in concerns from customers who may already have provider relationships.

Customer-side billing avoids all of those at the cost of placing the burden of provider setup on the
customer.

## Decision

Cloud AI features use the **customer's own API keys** for the provider of their choice (OpenAI,
Anthropic, others). The engine is a thin client. Harmonius operates no LLM proxy and takes no margin
on LLM calls.

Local inference is shipped with the engine and runs on the customer's hardware at no cost. This
handles the offline use case and the no-cloud use case.

The AI assistant feature ([F-15.22](../../features/tools/ai-assistant.md)) and AI cloud backend
([F-15.23](../../features/tools/ai-cloud-backend.md)) read keys from project configuration. There is
no Harmonius login, no metered billing, no proxy.

## Engineering implications

- Project configuration includes a per-project AI provider section (provider, base URL, key
  source).
- Keys never reach Harmonius servers. They live in the customer's secret store.
- The engine implements a small, narrow LLM client adapter. Provider changes are contained
  to the adapter.
- No engine-side rate limiting on LLM calls — the provider does it.
- Documentation must walk customers through setting up their preferred provider.

## Reversibility

Adding engine-side billing later is feasible but requires building the proxy and billing pipeline.
The current architecture does not preclude it; it simply does not require it.

Reverting customer-key support would require existing customer projects to migrate to a proxy, which
is operationally invasive. Treat this as the long-term default.

## References

- [docs/design/constraints.md](../../design/constraints.md) "Infrastructure"
- [docs/business/monetization-analysis.md](../../business/monetization-analysis.md) Component 22, AI
  assistant
- [docs/features/tools/ai-cloud-backend.md](../../features/tools/ai-cloud-backend.md)
- [docs/requirements/tools/ai-cloud-backend.md](../../requirements/tools/ai-cloud-backend.md)
