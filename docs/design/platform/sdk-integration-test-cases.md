# Platform SDK Integration Test Cases

Companion test cases for [sdk-integration.md](sdk-integration.md).

## Unit Tests

### TC-14.6.1.1 Achievement Unlock Per Platform

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | `unlock(AchievementId(1))` on Steam backend | `ISteamUserStats::SetAchievement` called, `Ok(())` | R-14.5.1 |
| 2 | `unlock(AchievementId(1))` on Apple backend | `GKAchievement.report` called, `Ok(())` | R-14.5.1 |
| 3 | `unlock(AchievementId(1))` on Xbox backend | `XblAchievement::UpdateAchievement` called, `Ok(())` | R-14.5.1 |
| 4 | `unlock(AchievementId(1))` on PSN backend | `NpTrophy::UnlockTrophy` called, `Ok(())` | R-14.5.1 |
| 5 | `unlock(AchievementId(1))` on EOS backend | `EOS_Achievements_UnlockAchievements` called, `Ok(())` | R-14.5.1 |

### TC-14.6.1.2 Achievement Progress Update

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | `set_progress(AchievementId(1), 0.5)` on Steam | `IndicateAchievementProgress` called with 50% | R-14.5.1 |
| 2 | `set_progress(AchievementId(1), 1.0)` | Auto-unlock triggered, state = `Unlocked` | R-14.5.1 |
| 3 | `set_progress(AchievementId(1), -0.1)` | `Err(AchievementError::InvalidProgress)` | R-14.5.1 |
| 4 | `set_progress(AchievementId(1), 1.5)` | `Err(AchievementError::InvalidProgress)` | R-14.5.1 |

### TC-14.6.1.3 Achievement Offline Deferred

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | `unlock(id)` while offline | Enqueued in deferred queue, `Ok(())` | R-14.5.1 |
| 2 | Network restored, `sync()` called | Deferred unlock submitted, platform confirms | R-14.5.1 |
| 3 | 5 deferred unlocks, `sync()` | All 5 submitted in order, all confirmed | R-14.5.1 |

### TC-14.6.1.4 Achievement List Retrieval

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | `list()` with 3 of 10 unlocked | Vec of 10, 3 with `unlocked=true` | R-14.5.1 |
| 2 | `list()` on platform with 0 achievements | Empty Vec, `Ok(vec![])` | R-14.5.1 |

### TC-14.6.2.1 IAP Query Products

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | `query_products([id_1, id_2])` on Steam | Vec of 2 `LocalizedProduct` with prices | R-14.5.6 |
| 2 | `query_products([id_1, id_2])` on Apple | Vec of 2 with StoreKit 2 localized prices | R-14.5.6 |
| 3 | `query_products([invalid_id])` | Empty Vec (unknown product filtered) | R-14.5.6 |

### TC-14.6.2.2 IAP Purchase Success

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | `initiate_purchase(product_1)` on Steam | `PurchaseResult::Success(receipt)` with valid `transaction_id` | R-14.5.6 |
| 2 | `initiate_purchase(product_1)` on Apple | `PurchaseResult::Success(receipt)` with JWS receipt data | R-14.5.6 |
| 3 | `initiate_purchase(product_1)` on Google | `PurchaseResult::Success(receipt)` with purchase token | R-14.5.6 |

### TC-14.6.2.3 IAP Purchase Failure

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | User cancels platform dialog | `PurchaseResult::Cancelled` | R-14.5.6 |
| 2 | Network error during purchase | `PurchaseResult::Failed(NetworkError)` | R-14.5.6 |
| 3 | Product not found in platform store | `PurchaseResult::Failed(ProductNotFound)` | R-14.5.6 |

### TC-14.6.2.4 IAP Purchase Pending

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Google Play pending purchase (parental approval) | `PurchaseResult::Pending` | R-14.5.6 |
| 2 | Apple Ask to Buy (child account) | `PurchaseResult::Deferred` | R-14.5.6 |

### TC-14.6.2.5 IAP Restore Purchases

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | `restore_purchases()` with 2 prior non-consumables | Vec of 2 `PlatformReceipt` | R-14.5.6 |
| 2 | `restore_purchases()` with no prior purchases | Empty Vec | R-14.5.6 |
| 3 | `restore_purchases()` with expired subscription | Vec of 1 with subscription receipt | R-14.5.6 |

### TC-14.6.2.6 IAP Purchase Refund

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Platform issues refund for transaction_1 | Webhook updates `RefundStatus::Approved` | R-14.5.6 |
| 2 | Refunded entitlement check | `is_entitled(product_1)` returns `false` | R-14.5.6 |

### TC-14.6.3.1 Subscription Check Active

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | `check_status(sub_id)` with active Apple sub | `SubStatus { active: true, state: Active }` | R-14.6.3 |
| 2 | `check_status(sub_id)` with active Google sub | `SubStatus { active: true, state: Active }` | R-14.6.3 |
| 3 | `check_status(sub_id)` with active Steam sub | `SubStatus { active: true, state: Active }` | R-14.6.3 |

### TC-14.6.3.2 Subscription Grace Period

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Apple sub with `gracePeriodExpirationDate` set | `SubStatus { state: GracePeriod, grace_period_end: Some(ts) }` | R-14.6.3 |
| 2 | Google sub in `IN_GRACE_PERIOD` state | `SubStatus { state: GracePeriod }` | R-14.6.3 |
| 3 | `handle_grace_period(sub_id)` | `GracePeriodInfo { in_grace_period: true }` | R-14.6.3 |

### TC-14.6.3.3 Subscription Cancellation

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | User cancels Apple sub | `SubStatus { state: Cancelled, expiry_date: Some(end_of_period) }` | R-14.6.3 |
| 2 | User cancels Google sub | `SubStatus { state: Cancelled }` | R-14.6.3 |
| 3 | Benefits remain until expiry | `active=true` until `expiry_date` reached | R-14.6.3 |

### TC-14.6.3.4 Subscription Expiry

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Check status after expiry date | `SubStatus { active: false, state: Expired }` | R-14.6.3 |
| 2 | Benefits revoked after expiry | Benefit actions contain `Revoke` entries | R-14.6.3 |

### TC-14.6.3.5 Subscription Renewal

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Auto-renew succeeds on Apple | `SubStatus { state: Active, renewal_date: Some(next) }` | R-14.6.3 |
| 2 | Auto-renew succeeds on Google | `SubStatus { state: Active }` | R-14.6.3 |

### TC-14.6.3.6 Subscription Pause (Google)

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Google sub paused by user | `SubStatus { state: Paused }` | R-14.6.3 |
| 2 | User resumes paused sub | `SubStatus { state: Active }` | R-14.6.3 |

### TC-14.6.3.7 Subscription Family Sharing

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Apple family shared sub | `SubStatus { is_family_shared: true, active: true }` | R-14.6.3 |
| 2 | Primary cancels family sub | `SubStatus { active: false }` for family member | R-14.6.3 |

### TC-14.5.5.1 Cloud Save Read Write

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | `write("slot_1", data)` then `read("slot_1")` | Returns `Some(data)` identical to written | R-14.5.5 |
| 2 | `read("nonexistent_key")` | Returns `None` | R-14.5.5 |

### TC-14.5.5.2 Cloud Save Conflict Detection

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Local vector clock `{steam:5}`, remote `{steam:3}` | Local dominates, use local | R-14.5.5 |
| 2 | Local `{steam:3}`, remote `{xbox:5}` | Concurrent, trigger conflict dialog | R-14.5.5 |
| 3 | Local `{steam:5}`, remote `{steam:5}` | No conflict, versions equal | R-14.5.5 |

### TC-14.5.5.3 Cloud Save Quota

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Write data exceeding platform quota | `Err(CloudError::QuotaExceeded)` | R-14.5.5 |

### TC-14.6.5.1 Matchmaking Create Lobby

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | `create_lobby(config)` on Steam | `LobbyHandle` with Steam lobby ID | R-14.6.5 |
| 2 | `create_lobby(config)` on Xbox | `LobbyHandle` with SmartMatch session | R-14.6.5 |
| 3 | `create_lobby(config)` on EOS | `LobbyHandle` with EOS lobby ID | R-14.6.5 |

### TC-14.6.5.2 Matchmaking Find Match

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | `find_match(criteria)` with available lobby | `MatchResult::Found(lobby_id)` | R-14.6.5 |
| 2 | `find_match(criteria)` with no lobbies | `MatchResult::NotFound` after timeout | R-14.6.5 |

### TC-14.6.5.3 Matchmaking Join Lobby

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | `join_lobby(valid_id)` | `Ok(())`, player added to lobby | R-14.6.5 |
| 2 | `join_lobby(full_lobby_id)` | `Err(MatchmakingError::LobbyFull)` | R-14.6.5 |
| 3 | `join_lobby(invalid_id)` | `Err(MatchmakingError::NotFound)` | R-14.6.5 |

### TC-14.6.5.4 Matchmaking Cross-Platform

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Steam player and Xbox player search | Custom matchmaker pairs them | R-14.6.5 |
| 2 | Cross-play disabled in settings | Only same-platform matches returned | R-14.6.5 |

### TC-14.6.4.1 Anti-Cheat Initialize VAC

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | `initialize()` on Steam backend | VAC session registered, `Ok(())` | R-14.6.4 |
| 2 | `initialize()` on EOS backend | EAC client module loaded, `Ok(())` | R-14.6.4 |

### TC-14.6.4.2 Anti-Cheat Initialize Custom

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | `initialize()` on platform without native AC | Custom integrity checker started, `Ok(())` | R-14.6.4 |
| 2 | Custom checker issues first challenge | Challenge sent within `challenge_interval_seconds` | R-14.6.4 |

### TC-14.6.4.3 Anti-Cheat Ban Check

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | `check_ban_status(clean_player)` | `BanStatus::Clean` | R-14.6.4 |
| 2 | `check_ban_status(banned_player)` on Steam | `BanStatus::Banned { reason: VacBan }` | R-14.6.4 |
| 3 | `check_ban_status(banned_player)` on EOS | `BanStatus::Banned` via `EOS_Sanctions` | R-14.6.4 |

### TC-14.6.4.4 Anti-Cheat Report Player

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | `report_player(id, CheatReason)` on Steam | `ISteamGameServer` report submitted | R-14.6.4 |
| 2 | `report_player(id, CheatReason)` on EOS | `EOS_Reports_SendPlayerBehaviorReport` called | R-14.6.4 |

### TC-14.6.2.7 Receipt Valid Apple

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Valid Apple JWS receipt | `ValidationResult { valid: true, is_duplicate: false }` | R-14.6.2 |
| 2 | Expired Apple receipt | `Err(ValidationError::ExpiredReceipt)` | R-14.6.2 |

### TC-14.6.2.8 Receipt Valid Google

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Valid Google purchase token | `ValidationResult { valid: true }` | R-14.6.2 |
| 2 | Invalid Google token | `Err(ValidationError::InvalidReceipt)` | R-14.6.2 |

### TC-14.6.2.9 Receipt Valid Steam

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Valid Steam transaction | `ValidationResult { valid: true }` after `FinalizeTxn` | R-14.6.2 |
| 2 | Already finalized Steam txn | `Err(ValidationError::Duplicate)` | R-14.6.2 |

### TC-14.6.2.10 Receipt Replay Attack

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Submit same receipt twice | First: `valid=true`, second: `Err(Duplicate)` | R-14.6.2 |
| 2 | Submit receipt from different account | `Err(ValidationError::InvalidReceipt)` | R-14.6.2 |

### TC-14.6.2.11 Receipt Retry With Backoff

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | First attempt fails (network), retry succeeds | `ValidationResult { valid: true }` on retry | R-14.6.2 |
| 2 | All retries fail (max_retries=3) | `Err(ValidationError::NetworkError)` after 3 attempts | R-14.6.2 |

### TC-14.6.1.5 Offline Achievement Degradation

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | `unlock(id)` while offline | Deferred queue count increments, `Ok(())` | R-14.6.1 |
| 2 | `list()` while offline | Returns cached list from last sync | R-14.6.1 |

### TC-14.6.1.6 Offline Leaderboard Degradation

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | `submit_score(board, 9500)` while offline | Score enqueued, `Ok(())` | R-14.6.1 |
| 2 | `query_global(board, range)` while offline | Returns cached entries or empty | R-14.6.1 |

### TC-14.6.1.7 Offline Purchase Blocked

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | `initiate_purchase(id)` while offline | `PurchaseResult::Failed(NetworkError)` | R-14.6.1 |

### TC-14.6.1.8 Offline Subscription Grace

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | `check_status(sub_id)` while offline | Returns last-known status with warning | R-14.6.1 |
| 2 | Offline for > 24 hours with active sub | Status marked as `needs_verification` | R-14.6.1 |

### TC-14.6.1.9 Offline Cloud Save Local

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | `write("slot_1", data)` while offline | Local write succeeds, sync pending | R-14.6.1 |
| 2 | `read("slot_1")` while offline | Returns locally cached data | R-14.6.1 |

### TC-14.6.1.10 Offline Friends Cache

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | `list_friends()` while offline | Returns cached friends list | R-14.6.1 |

### TC-14.5.7.1 Xbox Certification Achievements

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Xbox build with 0 achievements configured | Certification check fails: "XR-015 requires achievements" | R-14.5.7 |
| 2 | Xbox build with 10 achievements | Certification check passes | R-14.5.7 |

### TC-14.5.7.2 PSN Certification Trophies

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | PSN build with 0 trophies | Certification check fails: "TRC R4060 requires trophies" | R-14.5.7 |
| 2 | PSN build with Platinum + required trophies | Certification check passes | R-14.5.7 |

### TC-14.5.7.3 Console Suspend Resume

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Xbox suspend event received | Game state saved, resources released | R-14.5.7 |
| 2 | Xbox resume event received | Game state restored, sub re-verified | R-14.5.7 |
| 3 | PSN suspend event | Same as Xbox: save state, release | R-14.5.7 |
| 4 | Nintendo sleep mode | Save state, pause all systems | R-14.5.7 |

### TC-14.5.7.4 Console Save Data

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Xbox save with `XGameSaveWriteBlobData` | Data persisted, XR-078 compliant | R-14.5.7 |
| 2 | PSN save with `NpSaveData` | Data persisted, TRC R4082 compliant | R-14.5.7 |

### TC-14.5.7.5 Apple Privacy Manifest

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Apple build without privacy manifest | Build validation fails | R-14.5.7 |
| 2 | Apple build with ATT prompt | ATT dialog shown before tracking | R-14.5.7 |

### TC-14.5.7.6 Nintendo Button Mapping

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Handheld mode detected | Button glyphs match Joy-Con layout | R-14.5.7 |
| 2 | Docked mode with Pro Controller | Glyphs switch to Pro Controller layout | R-14.5.7 |

### TC-14.6.6.1 Account Link

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | `link(engine_id, Steam, token)` | `LinkedAccount` created, platform verified | R-14.6.6 |
| 2 | `link(engine_id, Xbox, token)` same engine_id | Second platform linked, 2 total | R-14.6.6 |
| 3 | `link(different_engine_id, Steam, same_token)` | `Err(LinkError::AlreadyLinked)` | R-14.6.6 |

### TC-14.6.6.2 Account Unlink

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | `unlink(engine_id, Steam)` | Steam removed, 1 link remaining | R-14.6.6 |
| 2 | `unlink(engine_id, NonLinkedPlatform)` | `Err(LinkError::NotLinked)` | R-14.6.6 |

### TC-14.6.6.3 Entitlement Merge

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Steam owns DLC A, Xbox owns DLC B | `EntitlementSet` contains both DLC A and B | R-14.6.6 |
| 2 | Consumable purchased on Steam | Not merged (consumables are platform-bound) | R-14.6.6 |

### TC-14.6.6.4 Save Data Cross-Platform Sync

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Save on Steam (clock `{steam:5}`), load on Xbox | Data available, clock merged to `{steam:5, xbox:1}` | R-14.6.6 |
| 2 | Concurrent saves on Steam and Xbox | Conflict dialog presented to player | R-14.6.6 |

## Integration Tests

### TC-14.6.2.I1 Purchase End-to-End

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Full purchase flow: query, buy, validate, grant | Entitlement granted < 3 seconds | R-14.5.6 |
| 2 | Purchase with network failure + retry | Entitlement granted after retry | R-14.5.6 |

### TC-14.6.3.I1 Subscription Lifecycle E2E

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Subscribe, verify, renew, cancel, expire | All state transitions correct | R-14.6.3 |
| 2 | Subscribe, payment fails, grace period, recover | Benefits maintained during grace | R-14.6.3 |

### TC-14.6.5.I1 Matchmaking Cross-Platform E2E

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Steam player creates lobby, Xbox player joins | Both in same session, game starts | R-14.6.5 |
| 2 | Platform lobby timeout (60 s) | Graceful timeout, player notified | R-14.6.5 |

### TC-14.6.4.I1 Anti-Cheat Full Cycle

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Init VAC, play session, disconnect | VAC session clean, no violations | R-14.6.4 |
| 2 | Init EAC, heartbeat 100 times | All heartbeats verified on server | R-14.6.4 |

### TC-14.6.6.I1 Cross-Platform Progression E2E

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Link Steam + Xbox, buy DLC on Steam, play on Xbox | DLC accessible on Xbox | R-14.6.6 |
| 2 | Earn achievement on Steam, check on Xbox | Achievement visible on both | R-14.6.6 |

### TC-14.6.1.I1 Offline to Online Transition

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Unlock 3 achievements offline, go online | All 3 synced within 30 seconds | R-14.6.1 |
| 2 | Submit 5 scores offline, go online | All 5 submitted on reconnect | R-14.6.1 |

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
