# Monetization and Live Operations Test Cases

Companion test cases for [monetization.md](monetization.md).

## Unit Tests

### TC-13.23.1.1 Pass XP Tier Advance

| # | Requirement |
|---|-------------|
| 1 | R-13.23.1   |
| 2 | R-13.23.1   |

1. **#1** — Award 500 XP; tier requires 400 XP
   - **Expected:** current_tier advances by 1
2. **#2** — Award 300 XP; tier requires 400 XP
   - **Expected:** current_tier unchanged; xp = 300

### TC-13.23.1.2 Pass Premium Gating

| # | Requirement |
|---|-------------|
| 1 | R-13.23.1   |
| 2 | R-13.23.1   |

1. **#1** — Claim tier with premium_reward; premium_purchased=false
   - **Expected:** Only free_reward returned
2. **#2** — Claim tier with premium_reward; premium_purchased=true
   - **Expected:** Both free and premium rewards returned

### TC-13.23.1.3 Pass Catchup Multiplier

| # | Requirement |
|---|-------------|
| 1 | R-13.23.1   |
| 2 | R-13.23.1   |

1. **#1** — Season at 70% elapsed (final third); multiplier=1.5
   - **Expected:** `is_catchup_active` = true
2. **#2** — Season at 50% elapsed
   - **Expected:** `is_catchup_active` = false

### TC-13.23.1.4 Pass Season Reset

| # | Requirement |
|---|-------------|
| 1 | R-13.23.1   |

1. **#1** — Season end_timestamp reached; new season starts
   - **Expected:** Progress resets to tier 0, xp 0

### TC-13.23.2.1 Challenge Counter

| # | Requirement |
|---|-------------|
| 1 | R-13.23.2   |
| 2 | R-13.23.2   |

1. **#1** — Challenge: kill 10; event: kill
   - **Expected:** Counter increments to 1
2. **#2** — 10th kill event
   - **Expected:** Challenge completes; returned in result

### TC-13.23.2.2 Challenge Rotation

| # | Requirement |
|---|-------------|
| 1 | R-13.23.2   |

1. **#1** — Daily refresh timestamp reached
   - **Expected:** Old dailies replaced with new set

### TC-13.23.2.3 Challenge Reroll

| # | Requirement |
|---|-------------|
| 1 | R-13.23.2   |

1. **#1** — Reroll daily index 0; rerolls_remaining=3
   - **Expected:** New challenge at index 0; rerolls=2; no duplicate

### TC-13.23.3a.1 Purchase Flow

| # | Requirement |
|---|-------------|
| 1 | R-13.23.3a  |

1. **#1** — Initiate purchase for product_id=100
   - **Expected:** `PurchaseResult::Success` with receipt

### TC-13.23.3b.1 Receipt Duplicate

| # | Requirement |
|---|-------------|
| 1 | R-13.23.3b  |

1. **#1** — Submit same receipt twice
   - **Expected:** Second validation returns `Duplicate` within 100 ms

### TC-13.23.3b.2 Receipt Retry

| # | Requirement |
|---|-------------|
| 1 | R-13.23.3b  |

1. **#1** — First validation fails (NetworkError); retry
   - **Expected:** Retries with exponential backoff; succeeds on retry

### TC-13.23.3c.1 Premium Currency Server

| # | Requirement |
|---|-------------|
| 1 | R-13.23.3c  |

1. **#1** — Credit 100 gems server-side
   - **Expected:** Balance = previous + 100; client cannot modify

### TC-13.23.3c.2 Premium Cosmetic Only

| # | Requirement |
|---|-------------|
| 1 | R-13.23.3c  |

1. **#1** — Attempt purchase of gameplay stat item
   - **Expected:** Rejected; cosmetic-only enforced

### TC-13.23.3d.1 Purchase History Record

| # | Requirement |
|---|-------------|
| 1 | R-13.23.3d  |

1. **#1** — Complete purchase
   - **Expected:** TransactionRecord stored with all fields

### TC-13.23.3d.2 Refund Webhook

| # | Requirement |
|---|-------------|
| 1 | R-13.23.3d  |

1. **#1** — Platform refund notification for txn_id=42
   - **Expected:** RefundStatus updated to Approved

### TC-13.23.4.1 Login Streak Strict

| # | Requirement |
|---|-------------|
| 1 | R-13.23.4   |

1. **#1** — Claim day 5; skip day 6; claim day 7
   - **Expected:** Streak resets to 1

### TC-13.23.4.2 Login Streak Lenient

| # | Requirement |
|---|-------------|
| 1 | R-13.23.4   |

1. **#1** — Lenient mode (stamps=2); skip 1 day
   - **Expected:** Catchup stamp consumed; streak preserved

### TC-13.23.4.3 Login Clock Tamper

| # | Requirement |
|---|-------------|
| 1 | R-13.23.4   |

1. **#1** — Client clock set 24h ahead of server
   - **Expected:** Claim rejected; server timestamp enforced

### TC-13.23.5a.1 Sub Verify Active

| # | Requirement |
|---|-------------|
| 1 | R-13.23.5a  |

1. **#1** — Active subscription; verify on login
   - **Expected:** active=true; tier set correctly

### TC-13.23.5a.2 Sub Lapse Detect

| # | Requirement |
|---|-------------|
| 1 | R-13.23.5a  |

1. **#1** — Subscription lapsed; verify
   - **Expected:** active=false detected within interval

### TC-13.23.5b.1 Sub Benefit Grant

| # | Requirement |
|---|-------------|
| 1 | R-13.23.5b  |

1. **#1** — Active tier with XpMultiplier(2)
   - **Expected:** `BenefitAction::Grant(XpMultiplier { multiplier: 2 })`

### TC-13.23.5b.2 Sub Benefit Revoke

| # | Requirement |
|---|-------------|
| 1 | R-13.23.5b  |

1. **#1** — Subscription lapses
   - **Expected:** `BenefitAction::Revoke` for all tier benefits

### TC-13.23.5d.1 Sub Gift No Autorenew

| # | Requirement |
|---|-------------|
| 1 | R-13.23.5d  |

1. **#1** — Gift subscription state
   - **Expected:** is_gift=true; no renewal_date set

### TC-13.23.6a.1 Trial Time Persist

| # | Requirement |
|---|-------------|
| 1 | R-13.23.6a  |

1. **#1** — Play 30 min trial; close; reopen
   - **Expected:** elapsed_sec = 1800; remaining = total - 1800

### TC-13.23.6a.2 Trial Progress Carry

| # | Requirement |
|---|-------------|
| 1 | R-13.23.6a  |

1. **#1** — Trial progress; purchase full game
   - **Expected:** All trial progress preserved

### TC-13.23.6b.1 Free Weekend Window

| # | Requirement |
|---|-------------|
| 1 | R-13.23.6b  |
| 2 | R-13.23.6b  |

1. **#1** — During free weekend event
   - **Expected:** Access granted
2. **#2** — After event end_timestamp
   - **Expected:** Access revoked

### TC-13.23.7.1 DLC Entitlement

| # | Requirement |
|---|-------------|
| 1 | R-13.23.7   |

1. **#1** — Signed DLC bundle; check entitlement
   - **Expected:** `is_entitled` = true; activation succeeds

### TC-13.23.7.2 DLC Tampered Reject

| # | Requirement |
|---|-------------|
| 1 | R-13.23.7   |

1. **#1** — Unsigned/tampered DLC bundle
   - **Expected:** Activation rejected

### TC-13.23.8.1 Store Cosmetic No Stat

| # | Requirement |
|---|-------------|
| 1 | R-13.23.8   |

1. **#1** — Purchase cosmetic skin
   - **Expected:** Item has zero stat effect

### TC-13.23.8.2 Store Refund 24h

| # | Requirement |
|---|-------------|
| 1 | R-13.23.8   |
| 2 | R-13.23.8   |

1. **#1** — Refund within 24 hours of purchase
   - **Expected:** Refund succeeds; currency returned
2. **#2** — Refund after 24 hours
   - **Expected:** Refund fails

### TC-13.23.8.3 Store AI Label

| # | Requirement |
|---|-------------|
| 1 | R-13.23.8   |

1. **#1** — AI-generated cosmetic (ai_generated=true)
   - **Expected:** Governance label displayed

### TC-13.23.9a.1 Ad Close Button Size

| # | Requirement |
|---|-------------|
| 1 | R-13.23.9a  |

1. **#1** — Creative with 30x30pt close button
   - **Expected:** Rejected; minimum 44x44pt

### TC-13.23.9b.1 Ad Minor Contextual

| # | Requirement |
|---|-------------|
| 1 | R-13.23.9b  |

1. **#1** — Player under 16; request ad
   - **Expected:** Only contextual (non-targeted) ads served

### TC-13.23.9c.1 Ad No Autoplay Audio

| # | Requirement |
|---|-------------|
| 1 | R-13.23.9c  |

1. **#1** — Creative with autoplay audio
   - **Expected:** Suppressed by safeguards

### TC-13.23.9d.1 Ad Freq Interstitial

| # | Requirement |
|---|-------------|
| 1 | R-13.23.9d  |

1. **#1** — Show interstitial; request 2nd within 10 min
   - **Expected:** 2nd request blocked

### TC-13.23.9d.2 Ad Freq Rewarded

| # | Requirement |
|---|-------------|
| 1 | R-13.23.9d  |

1. **#1** — Show 3 rewarded ads; request 4th within 1 hour
   - **Expected:** 4th request blocked

### TC-13.28.1.1 Rewarded Lifecycle

| # | Requirement |
|---|-------------|
| 1 | R-13.28.1   |

1. **#1** — Request; show; complete video
   - **Expected:** `AdResult::RewardedComplete`; reward granted

### TC-13.28.2.1 Interstitial Cooldown

| # | Requirement |
|---|-------------|
| 1 | R-13.28.2   |

1. **#1** — Show interstitial; request again within cooldown
   - **Expected:** Second display blocked

### TC-13.28.3.1 Banner IAB Size

| # | Requirement |
|---|-------------|
| 1 | R-13.28.3   |
| 2 | R-13.28.3   |

1. **#1** — Mobile banner
   - **Expected:** Dimensions = 320x50
2. **#2** — Tablet banner
   - **Expected:** Dimensions = 728x90

### TC-13.28.4.1 Mediation eCPM

| # | Requirement |
|---|-------------|
| 1 | R-13.28.4   |

1. **#1** — Network A eCPM=5.0, Network B eCPM=8.0
   - **Expected:** Network B selected

### TC-13.28.4.2 GDPR No Init

| # | Requirement |
|---|-------------|
| 1 | R-13.28.4   |

1. **#1** — No consent collected
   - **Expected:** No ad network initialized

## Integration Tests

### TC-NFR-13.23.1.I1 Purchase E2E

| # | Requirement |
|---|-------------|
| 1 | NFR-13.23.1 |

1. **#1** — Full purchase flow: dialog to delivery
   - **Expected:** Completes in < 3 seconds

### TC-NFR-13.23.1.I2 Receipt Idempotent

| # | Requirement |
|---|-------------|
| 1 | NFR-13.23.1 |

1. **#1** — Retry after failure
   - **Expected:** No double-credit

### TC-NFR-13.23.2.I1 Pass Hot Reload

| # | Requirement |
|---|-------------|
| 1 | NFR-13.23.2 |

1. **#1** — Deploy new pass definition server-side
   - **Expected:** Visible on client within 60 sec

### TC-NFR-13.23.3.I1 Sub Login Latency

| # | Requirement |
|---|-------------|
| 1 | NFR-13.23.3 |

1. **#1** — Subscription verify on login
   - **Expected:** Completes in < 500 ms

### TC-NFR-13.23.4.I1 DLC Bandwidth

| # | Requirement |
|---|-------------|
| 1 | NFR-13.23.4 |

1. **#1** — Download DLC on 100 Mbps connection
   - **Expected:** >= 80% bandwidth utilization

### TC-NFR-13.23.4.I2 DLC Resume

| # | Requirement |
|---|-------------|
| 1 | NFR-13.23.4 |

1. **#1** — Interrupt download; resume
   - **Expected:** Resumes from last offset

### TC-NFR-13.23.5.I1 Store Load

| # | Requirement |
|---|-------------|
| 1 | NFR-13.23.5 |

1. **#1** — Initial store page load
   - **Expected:** Renders in < 2 sec

### TC-NFR-13.23.5.I2 Store Scroll

| # | Requirement |
|---|-------------|
| 1 | NFR-13.23.5 |

1. **#1** — Scroll 500-item catalog
   - **Expected:** Maintains 60 fps

### TC-NFR-13.28.1.I1 Ad Load Latency

| # | Requirement |
|---|-------------|
| 1 | NFR-13.28.1 |

1. **#1** — Request and load ad
   - **Expected:** Loads in < 2 sec (p95)

### TC-NFR-13.28.2.I1 Rewarded Callback

| # | Requirement |
|---|-------------|
| 1 | NFR-13.28.2 |

1. **#1** — Video completes; callback fires
   - **Expected:** Reward callback < 16.67 ms after video

### TC-NFR-13.28.3.I1 GDPR Consent Withdraw

| # | Requirement |
|---|-------------|
| 1 | NFR-13.28.3 |

1. **#1** — Withdraw GDPR consent
   - **Expected:** All ad networks disabled within 1 frame

## Benchmarks

### TC-NFR-13.23.1.B1 Purchase Flow

| # | Scenario | Metric | Target | Requirement |
|---|----------|--------|--------|-------------|
| 1 | Dialog to delivery | Wall-clock time | < 3 sec | NFR-13.23.1 |

### TC-NFR-13.23.1.B2 Duplicate Receipt Rejection

| # | Scenario | Metric | Target | Requirement |
|---|----------|--------|--------|-------------|
| 1 | Replay same receipt | Rejection time | < 100 ms | NFR-13.23.1 |

### TC-NFR-13.23.2.B1 Live-Ops Refresh

| # | Scenario | Metric | Target | Requirement |
|---|----------|--------|--------|-------------|
| 1 | Server deploy to client visibility | Latency | < 60 sec | NFR-13.23.2 |

### TC-NFR-13.23.3.B1 Subscription Verify

| # | Scenario | Metric | Target | Requirement |
|---|----------|--------|--------|-------------|
| 1 | Subscription verify on login | Latency (p99) | < 500 ms | NFR-13.23.3 |

### TC-NFR-13.23.4.B1 DLC Download

| # | Scenario | Metric | Target | Requirement |
|---|----------|--------|--------|-------------|
| 1 | DLC download on 100 Mbps | Bandwidth utilization | >= 80% | NFR-13.23.4 |

### TC-NFR-13.23.5.B1 Store Page Load

| # | Scenario | Metric | Target | Requirement |
|---|----------|--------|--------|-------------|
| 1 | Initial store page | Load time | < 2 sec | NFR-13.23.5 |

### TC-NFR-13.23.5.B2 Store Scroll

| # | Scenario | Metric | Target | Requirement |
|---|----------|--------|--------|-------------|
| 1 | 500-item catalog scroll | Frame rate | 60 fps | NFR-13.23.5 |

### TC-NFR-13.28.1.B1 Ad Load

| # | Scenario | Metric | Target | Requirement |
|---|----------|--------|--------|-------------|
| 1 | Ad load request | Latency (p95) | < 2 sec | NFR-13.28.1 |

### TC-NFR-13.28.2.B1 Rewarded Callback

| # | Scenario | Metric | Target | Requirement |
|---|----------|--------|--------|-------------|
| 1 | Video complete to callback | Latency | < 16.67 ms | NFR-13.28.2 |

### TC-NFR-13.28.2.B2 Reward Delivery

| # | Scenario | Metric | Target | Requirement |
|---|----------|--------|--------|-------------|
| 1 | Reward delivery to inventory | Latency | < 100 ms | NFR-13.28.2 |

### TC-NFR-13.28.3.B1 GDPR Consent Withdrawal

| # | Scenario | Metric | Target | Requirement |
|---|----------|--------|--------|-------------|
| 1 | Consent withdrawal to full disable | Latency | < 1 frame | NFR-13.28.3 |
