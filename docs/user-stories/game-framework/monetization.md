# User Stories — Monetization and Live Operations (13.23)

## Tiered Progression Track

| ID           | Persona              |
|--------------|----------------------|
| US-13.23.1.1 | game designer (P-5)  |
| US-13.23.1.2 | game developer (P-15)|
| US-13.23.1.3 | player (P-23)        |
| US-13.23.1.4 | player (P-23)        |

1. **US-13.23.1.1** — **As a** game designer (P-5), **I want** to configure season duration, tier
   count, XP per tier, and reward tables, **so that** seasonal content is data-driven.
2. **US-13.23.1.2** — **As a** game developer (P-15), **I want** pass definitions loaded from the
   server without client updates, **so that** new seasons deploy live.
3. **US-13.23.1.3** — [game-specific] **As a** player (P-23), **I want** to earn tiers from gameplay
   XP with free and premium reward tracks, **so that** I have ongoing seasonal goals.
4. **US-13.23.1.4** — [game-specific] **As a** player (P-23), **I want** the pass UI to show my
   tier, XP progress, rewards, and days remaining, **so that** I track season progress.

## Scheduled Task and Tracking

| ID           | Persona              |
|--------------|----------------------|
| US-13.23.2.1 | game designer (P-5)  |
| US-13.23.2.2 | game developer (P-15)|
| US-13.23.2.3 | player (P-23)        |
| US-13.23.2.4 | player (P-23)        |

1. **US-13.23.2.1** — **As a** game designer (P-5), **I want** to configure challenge descriptions,
   progress types, rewards, and refresh schedules, **so that** challenge design is data-driven.
2. **US-13.23.2.2** — **As a** game developer (P-15), **I want** challenges defined server-side for
   live-ops flexibility, **so that** new challenges deploy without client updates.
3. **US-13.23.2.3** — [game-specific] **As a** player (P-23), **I want** daily and weekly challenge
   lists with progress tracking, **so that** I have short-term goals.
4. **US-13.23.2.4** — [game-specific] **As a** player (P-23), **I want** to reroll challenges at a
   currency cost, **so that** I swap challenges I dislike.

## Platform Purchase Abstraction

| ID            | Persona              |
|---------------|----------------------|
| US-13.23.3a.1 | game developer (P-15)|
| US-13.23.3b.1 | game developer (P-15)|
| US-13.23.3b.2 | server administrator (P-22)|
| US-13.23.3c.1 | game designer (P-5)  |
| US-13.23.3d.1 | player (P-23)        |

1. **US-13.23.3a.1** — **As a** game developer (P-15), **I want** a unified purchase API abstracting
   platform-specific store SDKs, **so that** one purchase flow works on all platforms.
2. **US-13.23.3b.1** — **As a** game developer (P-15), **I want** all purchase receipts validated
   server-side before granting items, **so that** fraudulent transactions are blocked.
3. **US-13.23.3b.2** — **As a** server administrator (P-22), **I want** validation results logged
   with transaction ID and status, **so that** I have an audit trail.
4. **US-13.23.3c.1** — **As a** game designer (P-5), **I want** a virtual currency ledger stored
   server-side with governance rules preventing pay-to-win, **so that** the economy is secure.
5. **US-13.23.3d.1** — **As a** player (P-23), **I want** a complete purchase history with
   transaction ID, date, amount, and refund status, **so that** I have a record of spending.

## Login Reward Calendar

| ID           | Persona              |
|--------------|----------------------|
| US-13.23.4.1 | game designer (P-5)  |
| US-13.23.4.2 | player (P-23)        |
| US-13.23.4.3 | server administrator (P-22)|

1. **US-13.23.4.1** — **As a** game designer (P-5), **I want** configurable streak milestones with
   strict or lenient modes, **so that** I tune retention mechanics.
2. **US-13.23.4.2** — [game-specific] **As a** player (P-23), **I want** escalating rewards for
   consecutive daily logins, **so that** consistency is incentivized.
3. **US-13.23.4.3** — **As a** server administrator (P-22), **I want** login detection via
   server-side timestamps, **so that** clock manipulation is prevented.

## Subscription Management

| ID           | Persona              |
|--------------|----------------------|
| US-13.23.5a.1| game developer (P-15)|
| US-13.23.5b.1| game designer (P-5)  |
| US-13.23.5b.2| player (P-23)        |
| US-13.23.5c.1| player (P-23)        |
| US-13.23.5d.1| player (P-23)        |

1. **US-13.23.5a.1** — **As a** game developer (P-15), **I want** subscription state verified
   server-side on login and periodically, **so that** lapsed subscriptions are detected.
2. **US-13.23.5b.1** — **As a** game designer (P-5), **I want** per-tier benefits granted on active
   subscription and revoked on lapse without deleting earned content, **so that** tier value is
   tangible.
3. **US-13.23.5b.2** — [game-specific] **As a** player (P-23), **I want** cosmetics and progression
   earned while subscribed preserved after lapse, **so that** I am not punished for unsubscribing.
4. **US-13.23.5c.1** — **As a** player (P-23), **I want** an in-game UI showing tier, renewal date,
   and billing with management actions, **so that** I manage subscriptions without leaving the game.
5. **US-13.23.5d.1** — [game-specific] **As a** player (P-23), **I want** to gift a subscription to
   a friend that does not auto-renew, **so that** I share benefits socially.

## Game Trial and Free Events

| ID           | Persona              |
|--------------|----------------------|
| US-13.23.6a.1| player (P-23)        |
| US-13.23.6a.2| game developer (P-15)|
| US-13.23.6b.1| player (P-23)        |
| US-13.23.6c.1| player (P-23)        |

1. **US-13.23.6a.1** — **As a** player (P-23), **I want** to play the full game for limited hours as
   a trial with all progress preserved, **so that** I evaluate risk-free.
2. **US-13.23.6a.2** — **As a** game developer (P-15), **I want** trial time tracked server-side
   persisting across sessions, **so that** the trial cannot be exploited.
3. **US-13.23.6b.1** — **As a** player (P-23), **I want** to play during free weekend events with
   progress carrying over, **so that** I try the game without commitment.
4. **US-13.23.6c.1** — **As a** player (P-23), **I want** temporary access to DLC during promotional
   periods, **so that** I try expansions before purchasing.

## DLC and Cosmetic Store

| ID           | Persona              |
|--------------|----------------------|
| US-13.23.7.1 | game developer (P-15)|
| US-13.23.7.2 | player (P-23)        |
| US-13.23.8.1 | game designer (P-5)  |
| US-13.23.8.2 | player (P-23)        |
| US-13.23.8.3 | player (P-23)        |

1. **US-13.23.7.1** — **As a** game developer (P-15), **I want** DLC as signed asset bundles
   downloaded on demand and verified by entitlements, **so that** content is protected.
2. **US-13.23.7.2** — **As a** player (P-23), **I want** to browse available DLC with descriptions,
   prices, and download progress, **so that** I purchase and install new content.
3. **US-13.23.8.1** — **As a** game designer (P-5), **I want** cosmetic categories and currency
   types configurable in data with a rotating store schedule, **so that** the shop is data-driven.
4. **US-13.23.8.2** — [game-specific] **As a** player (P-23), **I want** cosmetic items
   account-bound across all characters with no gameplay advantage, **so that** purchases are fair.
5. **US-13.23.8.3** — **As a** player (P-23), **I want** a 24-hour refund window for cosmetic
   purchases, **so that** I buy with confidence.
