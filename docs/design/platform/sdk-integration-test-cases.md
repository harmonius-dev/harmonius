# Platform SDK Integration Test Cases

Companion test cases for [sdk-integration.md](sdk-integration.md).

## Unit Tests

### TC-14.6.1.1 Achievement Unlock Per Platform

| # | Requirement |
|---|-------------|
| 1 | R-14.5.1    |
| 2 | R-14.5.1    |
| 3 | R-14.5.1    |
| 4 | R-14.5.1    |
| 5 | R-14.5.1    |

1. **#1** — `unlock(AchievementId(1))` on Steam backend
   - **Expected:** `ISteamUserStats::SetAchievement` called, `Ok(())`
2. **#2** — `unlock(AchievementId(1))` on Apple backend
   - **Expected:** `GKAchievement.report` called, `Ok(())`
3. **#3** — `unlock(AchievementId(1))` on Xbox backend
   - **Expected:** `XblAchievement::UpdateAchievement` called, `Ok(())`
4. **#4** — `unlock(AchievementId(1))` on PSN backend
   - **Expected:** `NpTrophy::UnlockTrophy` called, `Ok(())`
5. **#5** — `unlock(AchievementId(1))` on EOS backend
   - **Expected:** `EOS_Achievements_UnlockAchievements` called, `Ok(())`

### TC-14.6.1.2 Achievement Progress Update

| # | Requirement |
|---|-------------|
| 1 | R-14.5.1    |
| 2 | R-14.5.1    |
| 3 | R-14.5.1    |
| 4 | R-14.5.1    |

1. **#1** — `set_progress(AchievementId(1), 0.5)` on Steam
   - **Expected:** `IndicateAchievementProgress` called with 50%
2. **#2** — `set_progress(AchievementId(1), 1.0)`
   - **Expected:** Auto-unlock triggered, state = `Unlocked`
3. **#3** — `set_progress(AchievementId(1), -0.1)`
   - **Expected:** `Err(AchievementError::InvalidProgress)`
4. **#4** — `set_progress(AchievementId(1), 1.5)`
   - **Expected:** `Err(AchievementError::InvalidProgress)`

### TC-14.6.1.3 Achievement Offline Deferred

| # | Requirement |
|---|-------------|
| 1 | R-14.5.1    |
| 2 | R-14.5.1    |
| 3 | R-14.5.1    |

1. **#1** — `unlock(id)` while offline
   - **Expected:** Enqueued in deferred queue, `Ok(())`
2. **#2** — Network restored, `sync()` called
   - **Expected:** Deferred unlock submitted, platform confirms
3. **#3** — 5 deferred unlocks, `sync()`
   - **Expected:** All 5 submitted in order, all confirmed

### TC-14.6.1.4 Achievement List Retrieval

| # | Requirement |
|---|-------------|
| 1 | R-14.5.1    |
| 2 | R-14.5.1    |

1. **#1** — `list()` with 3 of 10 unlocked
   - **Expected:** Vec of 10, 3 with `unlocked=true`
2. **#2** — `list()` on platform with 0 achievements
   - **Expected:** Empty Vec, `Ok(vec![])`

### TC-14.6.2.1 IAP Query Products

| # | Requirement |
|---|-------------|
| 1 | R-14.5.6    |
| 2 | R-14.5.6    |
| 3 | R-14.5.6    |

1. **#1** — `query_products([id_1, id_2])` on Steam
   - **Expected:** Vec of 2 `LocalizedProduct` with prices
2. **#2** — `query_products([id_1, id_2])` on Apple
   - **Expected:** Vec of 2 with StoreKit 2 localized prices
3. **#3** — `query_products([invalid_id])`
   - **Expected:** Empty Vec (unknown product filtered)

### TC-14.6.2.2 IAP Purchase Success

| # | Requirement |
|---|-------------|
| 1 | R-14.5.6    |
| 2 | R-14.5.6    |
| 3 | R-14.5.6    |

1. **#1** — `initiate_purchase(product_1)` on Steam
   - **Expected:** `PurchaseResult::Success(receipt)` with valid `transaction_id`
2. **#2** — `initiate_purchase(product_1)` on Apple
   - **Expected:** `PurchaseResult::Success(receipt)` with JWS receipt data
3. **#3** — `initiate_purchase(product_1)` on Google
   - **Expected:** `PurchaseResult::Success(receipt)` with purchase token

### TC-14.6.2.3 IAP Purchase Failure

| # | Requirement |
|---|-------------|
| 1 | R-14.5.6    |
| 2 | R-14.5.6    |
| 3 | R-14.5.6    |

1. **#1** — User cancels platform dialog
   - **Expected:** `PurchaseResult::Cancelled`
2. **#2** — Network error during purchase
   - **Expected:** `PurchaseResult::Failed(NetworkError)`
3. **#3** — Product not found in platform store
   - **Expected:** `PurchaseResult::Failed(ProductNotFound)`

### TC-14.6.2.4 IAP Purchase Pending

| # | Requirement |
|---|-------------|
| 1 | R-14.5.6    |
| 2 | R-14.5.6    |

1. **#1** — Google Play pending purchase (parental approval)
   - **Expected:** `PurchaseResult::Pending`
2. **#2** — Apple Ask to Buy (child account)
   - **Expected:** `PurchaseResult::Deferred`

### TC-14.6.2.5 IAP Restore Purchases

| # | Requirement |
|---|-------------|
| 1 | R-14.5.6    |
| 2 | R-14.5.6    |
| 3 | R-14.5.6    |

1. **#1** — `restore_purchases()` with 2 prior non-consumables
   - **Expected:** Vec of 2 `PlatformReceipt`
2. **#2** — `restore_purchases()` with no prior purchases
   - **Expected:** Empty Vec
3. **#3** — `restore_purchases()` with expired subscription
   - **Expected:** Vec of 1 with subscription receipt

### TC-14.6.2.6 IAP Purchase Refund

| # | Requirement |
|---|-------------|
| 1 | R-14.5.6    |
| 2 | R-14.5.6    |

1. **#1** — Platform issues refund for transaction_1
   - **Expected:** Webhook updates `RefundStatus::Approved`
2. **#2** — Refunded entitlement check
   - **Expected:** `is_entitled(product_1)` returns `false`

### TC-14.6.3.1 Subscription Check Active

| # | Requirement |
|---|-------------|
| 1 | R-14.6.3    |
| 2 | R-14.6.3    |
| 3 | R-14.6.3    |

1. **#1** — `check_status(sub_id)` with active Apple sub
   - **Expected:** `SubStatus { active: true, state: Active }`
2. **#2** — `check_status(sub_id)` with active Google sub
   - **Expected:** `SubStatus { active: true, state: Active }`
3. **#3** — `check_status(sub_id)` with active Steam sub
   - **Expected:** `SubStatus { active: true, state: Active }`

### TC-14.6.3.2 Subscription Grace Period

| # | Requirement |
|---|-------------|
| 1 | R-14.6.3    |
| 2 | R-14.6.3    |
| 3 | R-14.6.3    |

1. **#1** — Apple sub with `gracePeriodExpirationDate` set
   - **Expected:** `SubStatus { state: GracePeriod, grace_period_end: Some(ts) }`
2. **#2** — Google sub in `IN_GRACE_PERIOD` state
   - **Expected:** `SubStatus { state: GracePeriod }`
3. **#3** — `handle_grace_period(sub_id)`
   - **Expected:** `GracePeriodInfo { in_grace_period: true }`

### TC-14.6.3.3 Subscription Cancellation

| # | Requirement |
|---|-------------|
| 1 | R-14.6.3    |
| 2 | R-14.6.3    |
| 3 | R-14.6.3    |

1. **#1** — User cancels Apple sub
   - **Expected:** `SubStatus { state: Cancelled, expiry_date: Some(end_of_period) }`
2. **#2** — User cancels Google sub
   - **Expected:** `SubStatus { state: Cancelled }`
3. **#3** — Benefits remain until expiry
   - **Expected:** `active=true` until `expiry_date` reached

### TC-14.6.3.4 Subscription Expiry

| # | Requirement |
|---|-------------|
| 1 | R-14.6.3    |
| 2 | R-14.6.3    |

1. **#1** — Check status after expiry date
   - **Expected:** `SubStatus { active: false, state: Expired }`
2. **#2** — Benefits revoked after expiry
   - **Expected:** Benefit actions contain `Revoke` entries

### TC-14.6.3.5 Subscription Renewal

| # | Requirement |
|---|-------------|
| 1 | R-14.6.3    |
| 2 | R-14.6.3    |

1. **#1** — Auto-renew succeeds on Apple
   - **Expected:** `SubStatus { state: Active, renewal_date: Some(next) }`
2. **#2** — Auto-renew succeeds on Google
   - **Expected:** `SubStatus { state: Active }`

### TC-14.6.3.6 Subscription Pause (Google)

| # | Requirement |
|---|-------------|
| 1 | R-14.6.3    |
| 2 | R-14.6.3    |

1. **#1** — Google sub paused by user
   - **Expected:** `SubStatus { state: Paused }`
2. **#2** — User resumes paused sub
   - **Expected:** `SubStatus { state: Active }`

### TC-14.6.3.7 Subscription Family Sharing

| # | Requirement |
|---|-------------|
| 1 | R-14.6.3    |
| 2 | R-14.6.3    |

1. **#1** — Apple family shared sub
   - **Expected:** `SubStatus { is_family_shared: true, active: true }`
2. **#2** — Primary cancels family sub
   - **Expected:** `SubStatus { active: false }` for family member

### TC-14.5.5.1 Cloud Save Read Write

| # | Requirement |
|---|-------------|
| 1 | R-14.5.5    |
| 2 | R-14.5.5    |

1. **#1** — `write("slot_1", data)` then `read("slot_1")`
   - **Expected:** Returns `Some(data)` identical to written
2. **#2** — `read("nonexistent_key")`
   - **Expected:** Returns `None`

### TC-14.5.5.2 Cloud Save Conflict Detection

| # | Requirement |
|---|-------------|
| 1 | R-14.5.5    |
| 2 | R-14.5.5    |
| 3 | R-14.5.5    |

1. **#1** — Local vector clock `{steam:5}`, remote `{steam:3}`
   - **Expected:** Local dominates, use local
2. **#2** — Local `{steam:3}`, remote `{xbox:5}`
   - **Expected:** Concurrent, trigger conflict dialog
3. **#3** — Local `{steam:5}`, remote `{steam:5}`
   - **Expected:** No conflict, versions equal

### TC-14.5.5.3 Cloud Save Quota

| # | Requirement |
|---|-------------|
| 1 | R-14.5.5    |

1. **#1** — Write data exceeding platform quota
   - **Expected:** `Err(CloudError::QuotaExceeded)`

### TC-14.6.5.1 Matchmaking Create Lobby

| # | Requirement |
|---|-------------|
| 1 | R-14.6.5    |
| 2 | R-14.6.5    |
| 3 | R-14.6.5    |

1. **#1** — `create_lobby(config)` on Steam
   - **Expected:** `LobbyHandle` with Steam lobby ID
2. **#2** — `create_lobby(config)` on Xbox
   - **Expected:** `LobbyHandle` with SmartMatch session
3. **#3** — `create_lobby(config)` on EOS
   - **Expected:** `LobbyHandle` with EOS lobby ID

### TC-14.6.5.2 Matchmaking Find Match

| # | Requirement |
|---|-------------|
| 1 | R-14.6.5    |
| 2 | R-14.6.5    |

1. **#1** — `find_match(criteria)` with available lobby
   - **Expected:** `MatchResult::Found(lobby_id)`
2. **#2** — `find_match(criteria)` with no lobbies
   - **Expected:** `MatchResult::NotFound` after timeout

### TC-14.6.5.3 Matchmaking Join Lobby

| # | Requirement |
|---|-------------|
| 1 | R-14.6.5    |
| 2 | R-14.6.5    |
| 3 | R-14.6.5    |

1. **#1** — `join_lobby(valid_id)`
   - **Expected:** `Ok(())`, player added to lobby
2. **#2** — `join_lobby(full_lobby_id)`
   - **Expected:** `Err(MatchmakingError::LobbyFull)`
3. **#3** — `join_lobby(invalid_id)`
   - **Expected:** `Err(MatchmakingError::NotFound)`

### TC-14.6.5.4 Matchmaking Cross-Platform

| # | Requirement |
|---|-------------|
| 1 | R-14.6.5    |
| 2 | R-14.6.5    |

1. **#1** — Steam player and Xbox player search
   - **Expected:** Custom matchmaker pairs them
2. **#2** — Cross-play disabled in settings
   - **Expected:** Only same-platform matches returned

### TC-14.6.4.1 Anti-Cheat Initialize VAC

| # | Requirement |
|---|-------------|
| 1 | R-14.6.4    |
| 2 | R-14.6.4    |

1. **#1** — `initialize()` on Steam backend
   - **Expected:** VAC session registered, `Ok(())`
2. **#2** — `initialize()` on EOS backend
   - **Expected:** EAC client module loaded, `Ok(())`

### TC-14.6.4.2 Anti-Cheat Initialize Custom

| # | Requirement |
|---|-------------|
| 1 | R-14.6.4    |
| 2 | R-14.6.4    |

1. **#1** — `initialize()` on platform without native AC
   - **Expected:** Custom integrity checker started, `Ok(())`
2. **#2** — Custom checker issues first challenge
   - **Expected:** Challenge sent within `challenge_interval_seconds`

### TC-14.6.4.3 Anti-Cheat Ban Check

| # | Requirement |
|---|-------------|
| 1 | R-14.6.4    |
| 2 | R-14.6.4    |
| 3 | R-14.6.4    |

1. **#1** — `check_ban_status(clean_player)`
   - **Expected:** `BanStatus::Clean`
2. **#2** — `check_ban_status(banned_player)` on Steam
   - **Expected:** `BanStatus::Banned { reason: VacBan }`
3. **#3** — `check_ban_status(banned_player)` on EOS
   - **Expected:** `BanStatus::Banned` via `EOS_Sanctions`

### TC-14.6.4.4 Anti-Cheat Report Player

| # | Requirement |
|---|-------------|
| 1 | R-14.6.4    |
| 2 | R-14.6.4    |

1. **#1** — `report_player(id, CheatReason)` on Steam
   - **Expected:** `ISteamGameServer` report submitted
2. **#2** — `report_player(id, CheatReason)` on EOS
   - **Expected:** `EOS_Reports_SendPlayerBehaviorReport` called

### TC-14.6.2.7 Receipt Valid Apple

| # | Requirement |
|---|-------------|
| 1 | R-14.6.2    |
| 2 | R-14.6.2    |

1. **#1** — Valid Apple JWS receipt
   - **Expected:** `ValidationResult { valid: true, is_duplicate: false }`
2. **#2** — Expired Apple receipt
   - **Expected:** `Err(ValidationError::ExpiredReceipt)`

### TC-14.6.2.8 Receipt Valid Google

| # | Requirement |
|---|-------------|
| 1 | R-14.6.2    |
| 2 | R-14.6.2    |

1. **#1** — Valid Google purchase token
   - **Expected:** `ValidationResult { valid: true }`
2. **#2** — Invalid Google token
   - **Expected:** `Err(ValidationError::InvalidReceipt)`

### TC-14.6.2.9 Receipt Valid Steam

| # | Requirement |
|---|-------------|
| 1 | R-14.6.2    |
| 2 | R-14.6.2    |

1. **#1** — Valid Steam transaction
   - **Expected:** `ValidationResult { valid: true }` after `FinalizeTxn`
2. **#2** — Already finalized Steam txn
   - **Expected:** `Err(ValidationError::Duplicate)`

### TC-14.6.2.10 Receipt Replay Attack

| # | Requirement |
|---|-------------|
| 1 | R-14.6.2    |
| 2 | R-14.6.2    |

1. **#1** — Submit same receipt twice
   - **Expected:** First: `valid=true`, second: `Err(Duplicate)`
2. **#2** — Submit receipt from different account
   - **Expected:** `Err(ValidationError::InvalidReceipt)`

### TC-14.6.2.11 Receipt Retry With Backoff

| # | Requirement |
|---|-------------|
| 1 | R-14.6.2    |
| 2 | R-14.6.2    |

1. **#1** — First attempt fails (network), retry succeeds
   - **Expected:** `ValidationResult { valid: true }` on retry
2. **#2** — All retries fail (max_retries=3)
   - **Expected:** `Err(ValidationError::NetworkError)` after 3 attempts

### TC-14.6.1.5 Offline Achievement Degradation

| # | Requirement |
|---|-------------|
| 1 | R-14.6.1    |
| 2 | R-14.6.1    |

1. **#1** — `unlock(id)` while offline
   - **Expected:** Deferred queue count increments, `Ok(())`
2. **#2** — `list()` while offline
   - **Expected:** Returns cached list from last sync

### TC-14.6.1.6 Offline Leaderboard Degradation

| # | Requirement |
|---|-------------|
| 1 | R-14.6.1    |
| 2 | R-14.6.1    |

1. **#1** — `submit_score(board, 9500)` while offline
   - **Expected:** Score enqueued, `Ok(())`
2. **#2** — `query_global(board, range)` while offline
   - **Expected:** Returns cached entries or empty

### TC-14.6.1.7 Offline Purchase Blocked

| # | Requirement |
|---|-------------|
| 1 | R-14.6.1    |

1. **#1** — `initiate_purchase(id)` while offline
   - **Expected:** `PurchaseResult::Failed(NetworkError)`

### TC-14.6.1.8 Offline Subscription Grace

| # | Requirement |
|---|-------------|
| 1 | R-14.6.1    |
| 2 | R-14.6.1    |

1. **#1** — `check_status(sub_id)` while offline
   - **Expected:** Returns last-known status with warning
2. **#2** — Offline for > 24 hours with active sub
   - **Expected:** Status marked as `needs_verification`

### TC-14.6.1.9 Offline Cloud Save Local

| # | Requirement |
|---|-------------|
| 1 | R-14.6.1    |
| 2 | R-14.6.1    |

1. **#1** — `write("slot_1", data)` while offline
   - **Expected:** Local write succeeds, sync pending
2. **#2** — `read("slot_1")` while offline
   - **Expected:** Returns locally cached data

### TC-14.6.1.10 Offline Friends Cache

| # | Requirement |
|---|-------------|
| 1 | R-14.6.1    |

1. **#1** — `list_friends()` while offline
   - **Expected:** Returns cached friends list

### TC-14.5.7.1 Xbox Certification Achievements

| # | Requirement |
|---|-------------|
| 1 | R-14.5.7    |
| 2 | R-14.5.7    |

1. **#1** — Xbox build with 0 achievements configured
   - **Expected:** Certification check fails: "XR-015 requires achievements"
2. **#2** — Xbox build with 10 achievements
   - **Expected:** Certification check passes

### TC-14.5.7.2 PSN Certification Trophies

| # | Requirement |
|---|-------------|
| 1 | R-14.5.7    |
| 2 | R-14.5.7    |

1. **#1** — PSN build with 0 trophies
   - **Expected:** Certification check fails: "TRC R4060 requires trophies"
2. **#2** — PSN build with Platinum + required trophies
   - **Expected:** Certification check passes

### TC-14.5.7.3 Console Suspend Resume

| # | Requirement |
|---|-------------|
| 1 | R-14.5.7    |
| 2 | R-14.5.7    |
| 3 | R-14.5.7    |
| 4 | R-14.5.7    |

1. **#1** — Xbox suspend event received
   - **Expected:** Game state saved, resources released
2. **#2** — Xbox resume event received
   - **Expected:** Game state restored, sub re-verified
3. **#3** — PSN suspend event
   - **Expected:** Same as Xbox: save state, release
4. **#4** — Nintendo sleep mode
   - **Expected:** Save state, pause all systems

### TC-14.5.7.4 Console Save Data

| # | Requirement |
|---|-------------|
| 1 | R-14.5.7    |
| 2 | R-14.5.7    |

1. **#1** — Xbox save with `XGameSaveWriteBlobData`
   - **Expected:** Data persisted, XR-078 compliant
2. **#2** — PSN save with `NpSaveData`
   - **Expected:** Data persisted, TRC R4082 compliant

### TC-14.5.7.5 Apple Privacy Manifest

| # | Requirement |
|---|-------------|
| 1 | R-14.5.7    |
| 2 | R-14.5.7    |

1. **#1** — Apple build without privacy manifest
   - **Expected:** Build validation fails
2. **#2** — Apple build with ATT prompt
   - **Expected:** ATT dialog shown before tracking

### TC-14.5.7.6 Nintendo Button Mapping

| # | Requirement |
|---|-------------|
| 1 | R-14.5.7    |
| 2 | R-14.5.7    |

1. **#1** — Handheld mode detected
   - **Expected:** Button glyphs match Joy-Con layout
2. **#2** — Docked mode with Pro Controller
   - **Expected:** Glyphs switch to Pro Controller layout

### TC-14.6.6.1 Account Link

| # | Requirement |
|---|-------------|
| 1 | R-14.6.6    |
| 2 | R-14.6.6    |
| 3 | R-14.6.6    |

1. **#1** — `link(engine_id, Steam, token)`
   - **Expected:** `LinkedAccount` created, platform verified
2. **#2** — `link(engine_id, Xbox, token)` same engine_id
   - **Expected:** Second platform linked, 2 total
3. **#3** — `link(different_engine_id, Steam, same_token)`
   - **Expected:** `Err(LinkError::AlreadyLinked)`

### TC-14.6.6.2 Account Unlink

| # | Requirement |
|---|-------------|
| 1 | R-14.6.6    |
| 2 | R-14.6.6    |

1. **#1** — `unlink(engine_id, Steam)`
   - **Expected:** Steam removed, 1 link remaining
2. **#2** — `unlink(engine_id, NonLinkedPlatform)`
   - **Expected:** `Err(LinkError::NotLinked)`

### TC-14.6.6.3 Entitlement Merge

| # | Requirement |
|---|-------------|
| 1 | R-14.6.6    |
| 2 | R-14.6.6    |

1. **#1** — Steam owns DLC A, Xbox owns DLC B
   - **Expected:** `EntitlementSet` contains both DLC A and B
2. **#2** — Consumable purchased on Steam
   - **Expected:** Not merged (consumables are platform-bound)

### TC-14.6.6.4 Save Data Cross-Platform Sync

| # | Requirement |
|---|-------------|
| 1 | R-14.6.6    |
| 2 | R-14.6.6    |

1. **#1** — Save on Steam (clock `{steam:5}`), load on Xbox
   - **Expected:** Data available, clock merged to `{steam:5, xbox:1}`
2. **#2** — Concurrent saves on Steam and Xbox
   - **Expected:** Conflict dialog presented to player

## Integration Tests

### TC-14.6.2.I1 Purchase End-to-End

| # | Requirement |
|---|-------------|
| 1 | R-14.5.6    |
| 2 | R-14.5.6    |

1. **#1** — Full purchase flow: query, buy, validate, grant
   - **Expected:** Entitlement granted < 3 seconds
2. **#2** — Purchase with network failure + retry
   - **Expected:** Entitlement granted after retry

### TC-14.6.3.I1 Subscription Lifecycle E2E

| # | Requirement |
|---|-------------|
| 1 | R-14.6.3    |
| 2 | R-14.6.3    |

1. **#1** — Subscribe, verify, renew, cancel, expire
   - **Expected:** All state transitions correct
2. **#2** — Subscribe, payment fails, grace period, recover
   - **Expected:** Benefits maintained during grace

### TC-14.6.5.I1 Matchmaking Cross-Platform E2E

| # | Requirement |
|---|-------------|
| 1 | R-14.6.5    |
| 2 | R-14.6.5    |

1. **#1** — Steam player creates lobby, Xbox player joins
   - **Expected:** Both in same session, game starts
2. **#2** — Platform lobby timeout (60 s)
   - **Expected:** Graceful timeout, player notified

### TC-14.6.4.I1 Anti-Cheat Full Cycle

| # | Requirement |
|---|-------------|
| 1 | R-14.6.4    |
| 2 | R-14.6.4    |

1. **#1** — Init VAC, play session, disconnect
   - **Expected:** VAC session clean, no violations
2. **#2** — Init EAC, heartbeat 100 times
   - **Expected:** All heartbeats verified on server

### TC-14.6.6.I1 Cross-Platform Progression E2E

| # | Requirement |
|---|-------------|
| 1 | R-14.6.6    |
| 2 | R-14.6.6    |

1. **#1** — Link Steam + Xbox, buy DLC on Steam, play on Xbox
   - **Expected:** DLC accessible on Xbox
2. **#2** — Earn achievement on Steam, check on Xbox
   - **Expected:** Achievement visible on both

### TC-14.6.1.I1 Offline to Online Transition

| # | Requirement |
|---|-------------|
| 1 | R-14.6.1    |
| 2 | R-14.6.1    |

1. **#1** — Unlock 3 achievements offline, go online
   - **Expected:** All 3 synced within 30 seconds
2. **#2** — Submit 5 scores offline, go online
   - **Expected:** All 5 submitted on reconnect

## Benchmarks

| Benchmark | Target | Source |
|-----------|--------|--------|
| SDK initialization time | < 2 s | R-14.6.1 |
| Achievement unlock latency | < 500 ms (p99) | R-14.5.1 |
| Purchase flow (dialog to delivery) | < 3 s | R-14.5.6 |
| Receipt validation round-trip | < 1 s (p99) | R-14.6.2 |
| Duplicate receipt rejection | < 100 ms | R-14.6.2 |
| Subscription status check | < 500 ms (p99) | R-14.6.3 |
| Cloud save read (1 MB) | < 2 s | R-14.5.5 |
| Cloud save write (1 MB) | < 3 s | R-14.5.5 |
| Matchmaking lobby creation | < 1 s | R-14.6.5 |
| Anti-cheat init (VAC/EAC) | < 500 ms | R-14.6.4 |
| Account link operation | < 2 s | R-14.6.6 |
| Entitlement merge query | < 1 s | R-14.6.6 |
| Deferred queue drain (100 items) | < 5 s | R-14.6.1 |
| Offline-to-online sync | < 30 s for 50 pending ops | R-14.6.1 |
