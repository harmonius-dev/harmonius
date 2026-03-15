# User Stories — 12.3 Asset Database

## US-12.3.1 Find and Manage Any Asset Instantly in a Million-Entry Library
**As an** artist, **I want** content-addressable storage, full-text and semantic search,
AI-driven categorization, smart recommendations, incremental builds, and full version
history, **so that** I can locate, organize, and iterate on assets efficiently in a
large-scale MMO production.

| Acceptance criteria | Features | Requirements |
|---|---|---|
| BLAKE3 content-addressable storage with dedup | F-12.3.1 | R-12.3.1 |
| Metadata store as single source of truth | F-12.3.2 | R-12.3.2 |
| Hash-based import cache skips unchanged assets | F-12.3.3 | R-12.3.3 |
| Incremental builds via dependency graph invalidation | F-12.3.4 | R-12.3.4 |
| Full-text and tag-based faceted search | F-12.3.5 | R-12.3.5 |
| Auto-generated thumbnails for all asset types | F-12.3.6 | R-12.3.6 |
| LLM-based auto-categorization and quality rating | F-12.3.7 | R-12.3.7 |
| Natural language semantic search via vector index | F-12.3.8 | R-12.3.8 |
| Smart collections and near-duplicate flagging | F-12.3.9 | R-12.3.9 |
| Full revision history with hash-based restore | F-12.3.10 | R-12.3.10 |
| Incremental build 10x faster than full for <1% changes | F-12.3.4 | R-X.14.1 |
