# User Stories -- Monetization and Live Operations (13.23)

## Battle Pass and Season (F-13.23.1)

## US-13.23.1.1
**As a** player (P-23), **I want** to earn battle pass tiers by gaining XP from gameplay
activities, **so that** I have ongoing progression goals each season.

## US-13.23.1.2
**As a** player (P-23), **I want** free-tier rewards available without purchase and premium-tier
rewards requiring pass purchase, **so that** free players still progress.

## US-13.23.1.3
**As a** player (P-23), **I want** the pass UI to show my current tier, XP progress, all
rewards, and days remaining, **so that** I can track my season progress.

## US-13.23.1.4
**As a** executive (P-1), **I want** pass definitions to be data assets loaded from the server,
**so that** new seasons deploy without a client update.

## US-13.23.1.5
**As a** designer (P-5), **I want** to configure season duration, tier count, XP per tier, and
reward tables, **so that** I can design seasonal content without code.

## US-13.23.1.6
**As a** tester (P-27), **I want** to verify that season transitions reset progress and apply
catch-up XP boosts, **so that** late-season players can still complete the pass.

## Daily and Weekly Challenges (F-13.23.2)

## US-13.23.2.1
**As a** player (P-23), **I want** daily and weekly challenge lists that refresh on schedule
with progress tracking, **so that** I have short-term goals alongside the battle pass.

## US-13.23.2.2
**As a** player (P-23), **I want** to reroll a subset of challenges per day at a currency cost,
**so that** I can swap challenges I dislike.

## US-13.23.2.3
**As a** executive (P-1), **I want** challenges defined server-side for live-ops flexibility,
**so that** new challenges deploy without a client update.

## US-13.23.2.4
**As a** designer (P-5), **I want** to configure challenge descriptions, progress types, reward
tiers, and refresh schedules, **so that** challenge design is data-driven.

## US-13.23.2.5
**As a** tester (P-27), **I want** to verify that daily challenges refresh at the configured
time and weekly challenges at the configured day, **so that** rotation schedules are correct.

## Platform Purchase Abstraction (F-13.23.3a)

## US-13.23.3a.1
**As a** executive (P-1), **I want** a unified purchase API that abstracts Apple, Google,
Steam, and console stores, **so that** one purchase flow works on all platforms.

## US-13.23.3a.2
**As a** designer (P-5), **I want** platform selection to be automatic based on runtime
environment, **so that** no platform-specific branching is needed.

## US-13.23.3a.3
**As a** tester (P-27), **I want** to verify that deferred purchases on iOS complete correctly
when the user approves, **so that** platform-specific edge cases are handled.

## Server-Side Receipt Validation (F-13.23.3b)

## US-13.23.3b.1
**As a** executive (P-1), **I want** all purchase receipts validated server-side before
granting items or currency, **so that** fraudulent transactions are blocked.

## US-13.23.3b.2
**As a** player (P-23), **I want** duplicate receipt replay detected and rejected, **so that**
exploits cannot grant items twice.

## US-13.23.3b.3
**As a** tester (P-27), **I want** to verify that a replayed receipt is rejected, **so that**
duplicate detection works correctly.

## US-13.23.3b.4
**As a** tester (P-27), **I want** to verify that failed validations retry with exponential
backoff, **so that** transient failures are recoverable.

## Premium Currency (F-13.23.3c)

## US-13.23.3c.1
**As a** player (P-23), **I want** to buy premium currency with real money and have it credited
after validation, **so that** I can spend it on cosmetics and convenience items.

## US-13.23.3c.2
**As a** executive (P-1), **I want** currency balances stored server-side to prevent tampering,
**so that** the economy is secure.

## US-13.23.3c.3
**As a** designer (P-5), **I want** monetized items restricted to cosmetic or convenience only
via governance rules, **so that** pay-to-win is prevented by default.

## US-13.23.3c.4
**As a** tester (P-27), **I want** to verify that premium currency credits only after
successful receipt validation, **so that** currency grants are secure.

## Purchase History and Refund Tracking (F-13.23.3d)

## US-13.23.3d.1
**As a** player (P-23), **I want** a complete purchase history with transaction ID, date,
amount, and refund status, **so that** I have a record of all spending.

## US-13.23.3d.2
**As a** executive (P-1), **I want** platform refund notifications to update refund status
automatically, **so that** refund tracking is current.

## US-13.23.3d.3
**As a** tester (P-27), **I want** to verify that a refund notification from Apple updates the
transaction's refund status, **so that** webhook integration works correctly.

## Daily Login Rewards (F-13.23.4)

## US-13.23.4.1
**As a** player (P-23), **I want** a reward calendar granting escalating rewards for
consecutive daily logins, **so that** I am incentivized to log in each day.

## US-13.23.4.2
**As a** player (P-23), **I want** streak milestones at 7, 14, and 28 days to grant bonus
rewards, **so that** consistency is extra rewarding.

## US-13.23.4.3
**As a** executive (P-1), **I want** login detection using server-side timestamps to prevent
clock manipulation, **so that** rewards cannot be exploited.

## US-13.23.4.4
**As a** designer (P-5), **I want** to configure strict or lenient streak modes and monthly
reward pools, **so that** I can tune retention mechanics.

## US-13.23.4.5
**As a** tester (P-27), **I want** to verify that missing a day in strict mode resets the
streak, **so that** the streak policy is enforced.

## Subscription State Verification (F-13.23.5a)

## US-13.23.5a.1
**As a** player (P-23), **I want** my subscription status verified on login and periodically,
**so that** I always have the correct benefits.

## US-13.23.5a.2
**As a** executive (P-1), **I want** lapsed subscriptions detected within one verification
interval, **so that** benefits are revoked promptly.

## US-13.23.5a.3
**As a** tester (P-27), **I want** to verify that the local cache avoids redundant API calls
within the verification interval, **so that** server load is minimized.

## Subscription Benefit Application (F-13.23.5b)

## US-13.23.5b.1
**As a** player (P-23), **I want** subscription benefits like bonus XP, exclusive cosmetics,
and expanded storage to activate while subscribed, **so that** my subscription has tangible
value.

## US-13.23.5b.2
**As a** player (P-23), **I want** cosmetics and progression earned while subscribed to be
preserved after my subscription lapses, **so that** I am not punished for unsubscribing.

## US-13.23.5b.3
**As a** designer (P-5), **I want** to configure per-tier benefits and revocation rules, **so
that** I can differentiate subscription tiers.

## US-13.23.5b.4
**As a** tester (P-27), **I want** to verify that tier upgrade adjusts benefits immediately on
the next verification, **so that** upgrade takes effect promptly.

## Subscription Management UI (F-13.23.5c)

## US-13.23.5c.1
**As a** player (P-23), **I want** to view my current tier, renewal date, and billing amount
in-game, **so that** I can manage my subscription without leaving the game.

## US-13.23.5c.2
**As a** player (P-23), **I want** upgrade, downgrade, and cancel actions to redirect to the
platform's native subscription management, **so that** billing changes use trusted flows.

## US-13.23.5c.3
**As a** tester (P-27), **I want** to verify that the tier comparison table displays correct
benefit differences, **so that** the UI is accurate.

## Subscription Gifting (F-13.23.5d)

## US-13.23.5d.1
**As a** player (P-23), **I want** to purchase a subscription as a gift for a friend by
selecting them from my friend list, **so that** I can share membership benefits.

## US-13.23.5d.2
**As a** player (P-23), **I want** gift subscriptions to not auto-renew, **so that** the
recipient decides whether to subscribe after the gift expires.

## US-13.23.5d.3
**As a** executive (P-1), **I want** gift subscriptions handled server-side where platforms
lack native support, **so that** gifting works on all platforms.

## US-13.23.5d.4
**As a** tester (P-27), **I want** to verify that the recipient receives the subscription with
a notification and the gifter's name, **so that** gift delivery is correct.

## Timed Game Trial (F-13.23.6a)

## US-13.23.6a.1
**As a** player (P-23), **I want** to play the full game for a limited number of hours as a
free trial with all progress preserved, **so that** I can evaluate risk-free.

## US-13.23.6a.2
**As a** player (P-23), **I want** trial time to persist across sessions so closing the game
does not reset the clock, **so that** the trial is fairly metered.

## US-13.23.6a.3
**As a** executive (P-1), **I want** trial time tracked server-side to prevent clock
manipulation, **so that** the trial cannot be exploited.

## US-13.23.6a.4
**As a** tester (P-27), **I want** to verify that trial progress carries over to a purchase
without data loss, **so that** the transition is seamless.

## Free Weekend Events (F-13.23.6b)

## US-13.23.6b.1
**As a** player (P-23), **I want** to download and play the full game during a free weekend
with all progress carrying over, **so that** I can try the game without commitment.

## US-13.23.6b.2
**As a** executive (P-1), **I want** to configure free weekend start/end timestamps
server-side, **so that** I can run promotions without a client update.

## US-13.23.6b.3
**As a** tester (P-27), **I want** to verify that access is revoked after the event end
timestamp, **so that** free weekend expiration is enforced.

## Content Trial (F-13.23.6c)

## US-13.23.6c.1
**As a** player (P-23), **I want** to temporarily access DLC content during a promotional
period, **so that** I can try expansions before purchasing.

## US-13.23.6c.2
**As a** player (P-23), **I want** a purchase prompt after the content trial ends, **so that**
I can buy the full DLC if I enjoyed it.

## US-13.23.6c.3
**As a** executive (P-1), **I want** content trial start/end timestamps set server-side, **so
that** promotional periods are managed centrally.

## US-13.23.6c.4
**As a** tester (P-27), **I want** to verify that temporary access reverts on expiration, **so
that** content trial entitlements expire correctly.

## DLC and Expansion Purchasing (F-13.23.7)

## US-13.23.7.1
**As a** player (P-23), **I want** to browse available DLC with descriptions, screenshots,
trailers, and local-currency pricing, **so that** I can evaluate content before buying.

## US-13.23.7.2
**As a** player (P-23), **I want** DLC downloaded on demand without reinstalling the base game,
**so that** new content is a small incremental download.

## US-13.23.7.3
**As a** player (P-23), **I want** bundle deals that discount multiple DLC packs, **so that**
I save money when buying several expansions together.

## US-13.23.7.4
**As a** executive (P-1), **I want** DLC gated by entitlement verification with signed asset
bundles, **so that** content is protected from unauthorized access.

## US-13.23.7.5
**As a** designer (P-5), **I want** DLC that adds gameplay systems to integrate through the
modular system and activate on next login, **so that** new features plug in cleanly.

## US-13.23.7.6
**As a** tester (P-27), **I want** to verify that downloading and activating DLC does not
require a full client restart, **so that** the activation flow is seamless.

## Cosmetic Store (F-13.23.8)

## US-13.23.8.1
**As a** player (P-23), **I want** to browse a rotating cosmetic shop with skins, emotes,
mount skins, and seasonal items, **so that** I can personalize my appearance.

## US-13.23.8.2
**As a** player (P-23), **I want** to refund any cosmetic purchase within 24 hours with no
questions asked, **so that** I can buy with confidence.

## US-13.23.8.3
**As a** player (P-23), **I want** purchased cosmetics to be account-bound and available on
all characters, **so that** I do not re-buy per character.

## US-13.23.8.4
**As a** executive (P-1), **I want** the store rotation to be configurable on a daily/weekly
schedule, **so that** featured items refresh regularly.

## US-13.23.8.5
**As a** designer (P-5), **I want** cosmetic categories and currency types (premium, earned,
event) configurable in data, **so that** the shop economy is data-driven.

## US-13.23.8.6
**As a** tester (P-27), **I want** to verify that a refund within 24 hours returns the
currency immediately, **so that** the refund window is enforced.

## Deceptive UI Prevention (F-13.23.9a)

## US-13.23.9a.1
**As a** player (P-23), **I want** ad close buttons to be at least 44x44 points, clearly
visible, and functional immediately, **so that** I can close any ad without being tricked.

## US-13.23.9a.2
**As a** executive (P-1), **I want** deceptive UI rules compiled into the engine binary and not
overridable by game configuration, **so that** compliance is guaranteed.

## US-13.23.9a.3
**As a** tester (P-27), **I want** to verify that ad creatives mimicking game UI are rejected
by the mediation layer, **so that** fake-button ads are blocked.

## Minor-Targeted Ad Blocking (F-13.23.9b)

## US-13.23.9b.1
**As a** player (P-23), **I want** minors under 16 to receive only contextual ads with no
behavioral targeting, **so that** children are protected.

## US-13.23.9b.2
**As a** executive (P-1), **I want** age detection integrated with platform parental control
APIs, **so that** the engine complies with COPPA and GDPR Article 8.

## US-13.23.9b.3
**As a** tester (P-27), **I want** to verify that a session with parental controls reporting
age under 16 blocks personalized ads, **so that** minor protection is enforced.

## Dark Pattern Prevention (F-13.23.9c)

## US-13.23.9c.1
**As a** player (P-23), **I want** ads to never auto-play audio, vibrate my device, launch
external apps without consent, or obscure the "Ad" label, **so that** I am never manipulated.

## US-13.23.9c.2
**As a** player (P-23), **I want** a global "disable all ads" toggle in accessibility
preferences, **so that** I can play without any advertising.

## US-13.23.9c.3
**As a** executive (P-1), **I want** dark pattern prevention rules compiled into the engine
binary, **so that** they cannot be bypassed.

## US-13.23.9c.4
**As a** tester (P-27), **I want** to verify that an ad creative that auto-plays audio is
suppressed and logged, **so that** rule enforcement works correctly.

## Frequency Cap Enforcement (F-13.23.9d)

## US-13.23.9d.1
**As a** player (P-23), **I want** interstitial ads hard-capped at 1 per 10 minutes and
rewarded videos at 3 per hour, **so that** I am never bombarded with ads.

## US-13.23.9d.2
**As a** executive (P-1), **I want** frequency caps compiled into the engine binary and not
changeable at runtime, **so that** caps are guaranteed regardless of game configuration.

## US-13.23.9d.3
**As a** tester (P-27), **I want** to verify that a second interstitial within 10 minutes is
silently blocked and logged, **so that** the rolling window cap is enforced.
