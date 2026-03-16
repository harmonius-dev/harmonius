# Monetization and Live Operations Test Cases

Companion test cases for [monetization.md](monetization.md).

## Unit Tests

### TC-13.23.1.1 Pass XP Tier Advance

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Award 500 XP; tier requires 400 XP | current_tier advances by 1 | R-13.23.1 |
| 2 | Award 300 XP; tier requires 400 XP | current_tier unchanged; xp = 300 | R-13.23.1 |

### TC-13.23.1.2 Pass Premium Gating

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Claim tier with premium_reward; premium_purchased=false | Only free_reward returned | R-13.23.1 |
| 2 | Claim tier with premium_reward; premium_purchased=true | Both free and premium rewards returned | R-13.23.1 |

### TC-13.23.1.3 Pass Catchup Multiplier

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Season at 70% elapsed (final third); multiplier=1.5 | `is_catchup_active` = true | R-13.23.1 |
| 2 | Season at 50% elapsed | `is_catchup_active` = false | R-13.23.1 |

### TC-13.23.1.4 Pass Season Reset

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Season end_timestamp reached; new season starts | Progress resets to tier 0, xp 0 | R-13.23.1 |

### TC-13.23.2.1 Challenge Counter

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Challenge: kill 10; event: kill | Counter increments to 1 | R-13.23.2 |
| 2 | 10th kill event | Challenge completes; returned in result | R-13.23.2 |

### TC-13.23.2.2 Challenge Rotation

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Daily refresh timestamp reached | Old dailies replaced with new set | R-13.23.2 |

### TC-13.23.2.3 Challenge Reroll

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Reroll daily index 0; rerolls_remaining=3 | New challenge at index 0; rerolls=2; no duplicate | R-13.23.2 |

### TC-13.23.3a.1 Purchase Flow

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Initiate purchase for product_id=100 | `PurchaseResult::Success` with receipt | R-13.23.3a |

### TC-13.23.3b.1 Receipt Duplicate

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Submit same receipt twice | Second validation returns `Duplicate` within 100 ms | R-13.23.3b |

### TC-13.23.3b.2 Receipt Retry

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | First validation fails (NetworkError); retry | Retries with exponential backoff; succeeds on retry | R-13.23.3b |

### TC-13.23.3c.1 Premium Currency Server

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Credit 100 gems server-side | Balance = previous + 100; client cannot modify | R-13.23.3c |

### TC-13.23.3c.2 Premium Cosmetic Only

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Attempt purchase of gameplay stat item | Rejected; cosmetic-only enforced | R-13.23.3c |

### TC-13.23.3d.1 Purchase History Record

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Complete purchase | TransactionRecord stored with all fields | R-13.23.3d |

### TC-13.23.3d.2 Refund Webhook

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Platform refund notification for txn_id=42 | RefundStatus updated to Approved | R-13.23.3d |

### TC-13.23.4.1 Login Streak Strict

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Claim day 5; skip day 6; claim day 7 | Streak resets to 1 | R-13.23.4 |

### TC-13.23.4.2 Login Streak Lenient

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Lenient mode (stamps=2); skip 1 day | Catchup stamp consumed; streak preserved | R-13.23.4 |

### TC-13.23.4.3 Login Clock Tamper

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Client clock set 24h ahead of server | Claim rejected; server timestamp enforced | R-13.23.4 |

### TC-13.23.5a.1 Sub Verify Active

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Active subscription; verify on login | active=true; tier set correctly | R-13.23.5a |

### TC-13.23.5a.2 Sub Lapse Detect

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Subscription lapsed; verify | active=false detected within interval | R-13.23.5a |

### TC-13.23.5b.1 Sub Benefit Grant

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Active tier with XpMultiplier(2) | `BenefitAction::Grant(XpMultiplier { multiplier: 2 })` | R-13.23.5b |

### TC-13.23.5b.2 Sub Benefit Revoke

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Subscription lapses | `BenefitAction::Revoke` for all tier benefits | R-13.23.5b |

### TC-13.23.5d.1 Sub Gift No Autorenew

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Gift subscription state | is_gift=true; no renewal_date set | R-13.23.5d |

### TC-13.23.6a.1 Trial Time Persist

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Play 30 min trial; close; reopen | elapsed_sec = 1800; remaining = total - 1800 | R-13.23.6a |

### TC-13.23.6a.2 Trial Progress Carry

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Trial progress; purchase full game | All trial progress preserved | R-13.23.6a |

### TC-13.23.6b.1 Free Weekend Window

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | During free weekend event | Access granted | R-13.23.6b |
| 2 | After event end_timestamp | Access revoked | R-13.23.6b |

### TC-13.23.7.1 DLC Entitlement

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Signed DLC bundle; check entitlement | `is_entitled` = true; activation succeeds | R-13.23.7 |

### TC-13.23.7.2 DLC Tampered Reject

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Unsigned/tampered DLC bundle | Activation rejected | R-13.23.7 |

### TC-13.23.8.1 Store Cosmetic No Stat

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Purchase cosmetic skin | Item has zero stat effect | R-13.23.8 |

### TC-13.23.8.2 Store Refund 24h

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Refund within 24 hours of purchase | Refund succeeds; currency returned | R-13.23.8 |
| 2 | Refund after 24 hours | Refund fails | R-13.23.8 |

### TC-13.23.8.3 Store AI Label

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | AI-generated cosmetic (ai_generated=true) | Governance label displayed | R-13.23.8 |

### TC-13.23.9a.1 Ad Close Button Size

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Creative with 30x30pt close button | Rejected; minimum 44x44pt | R-13.23.9a |

### TC-13.23.9b.1 Ad Minor Contextual

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Player under 16; request ad | Only contextual (non-targeted) ads served | R-13.23.9b |

### TC-13.23.9c.1 Ad No Autoplay Audio

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Creative with autoplay audio | Suppressed by safeguards | R-13.23.9c |

### TC-13.23.9d.1 Ad Freq Interstitial

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Show interstitial; request 2nd within 10 min | 2nd request blocked | R-13.23.9d |

### TC-13.23.9d.2 Ad Freq Rewarded

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Show 3 rewarded ads; request 4th within 1 hour | 4th request blocked | R-13.23.9d |

### TC-13.28.1.1 Rewarded Lifecycle

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Request; show; complete video | `AdResult::RewardedComplete`; reward granted | R-13.28.1 |

### TC-13.28.2.1 Interstitial Cooldown

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Show interstitial; request again within cooldown | Second display blocked | R-13.28.2 |

### TC-13.28.3.1 Banner IAB Size

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Mobile banner | Dimensions = 320x50 | R-13.28.3 |
| 2 | Tablet banner | Dimensions = 728x90 | R-13.28.3 |

### TC-13.28.4.1 Mediation eCPM

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Network A eCPM=5.0, Network B eCPM=8.0 | Network B selected | R-13.28.4 |

### TC-13.28.4.2 GDPR No Init

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | No consent collected | No ad network initialized | R-13.28.4 |

## Integration Tests

### TC-NFR-13.23.1.I1 Purchase E2E

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Full purchase flow: dialog to delivery | Completes in < 3 seconds | NFR-13.23.1 |

### TC-NFR-13.23.1.I2 Receipt Idempotent

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Retry after failure | No double-credit | NFR-13.23.1 |

### TC-NFR-13.23.2.I1 Pass Hot Reload

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Deploy new pass definition server-side | Visible on client within 60 sec | NFR-13.23.2 |

### TC-NFR-13.23.3.I1 Sub Login Latency

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Subscription verify on login | Completes in < 500 ms | NFR-13.23.3 |

### TC-NFR-13.23.4.I1 DLC Bandwidth

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Download DLC on 100 Mbps connection | >= 80% bandwidth utilization | NFR-13.23.4 |

### TC-NFR-13.23.4.I2 DLC Resume

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Interrupt download; resume | Resumes from last offset | NFR-13.23.4 |

### TC-NFR-13.23.5.I1 Store Load

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Initial store page load | Renders in < 2 sec | NFR-13.23.5 |

### TC-NFR-13.23.5.I2 Store Scroll

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Scroll 500-item catalog | Maintains 60 fps | NFR-13.23.5 |

### TC-NFR-13.28.1.I1 Ad Load Latency

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Request and load ad | Loads in < 2 sec (p95) | NFR-13.28.1 |

### TC-NFR-13.28.2.I1 Rewarded Callback

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Video completes; callback fires | Reward callback < 16.67 ms after video | NFR-13.28.2 |

### TC-NFR-13.28.3.I1 GDPR Consent Withdraw

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Withdraw GDPR consent | All ad networks disabled within 1 frame | NFR-13.28.3 |

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
