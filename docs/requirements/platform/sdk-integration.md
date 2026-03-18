# R-14.7 — Platform SDK Integration Requirements

## Steam (Steamworks)

| ID       | Derived From                                           |
|----------|--------------------------------------------------------|
| R-14.7.1 | [F-14.7.1](../../features/platform/sdk-integration.md) |
| R-14.7.2 | [F-14.7.2](../../features/platform/sdk-integration.md) |
| R-14.7.3 | [F-14.7.3](../../features/platform/sdk-integration.md) |
| R-14.7.4 | [F-14.7.4](../../features/platform/sdk-integration.md) |
| R-14.7.5 | [F-14.7.5](../../features/platform/sdk-integration.md) |
| R-14.7.6 | [F-14.7.6](../../features/platform/sdk-integration.md) |
| R-14.7.7 | [F-14.7.7](../../features/platform/sdk-integration.md) |
| R-14.7.8 | [F-14.7.8](../../features/platform/sdk-integration.md) |
| R-14.7.9 | [F-14.7.9](../../features/platform/sdk-integration.md) |

1. **R-14.7.1** — The engine **SHALL** unlock Steam achievements and update player stats through the
   Steamworks `ISteamUserStats` API, mapping internal achievement IDs to Steam-specific string
   identifiers. Deferred unlocks **SHALL** be queued when the Steam service is unavailable and
   retried automatically on reconnection. Initial sync **SHALL** fetch current unlock and stat state
   on session start to prevent redundant API calls.
   - **Rationale:** Mapping internal IDs to Steam identifiers decouples gameplay code from the Steam
     SDK, and deferred unlocks prevent achievement loss during network outages.
   - **Verification:** Unlock an achievement while online and verify it appears in the Steam
     achievement list. Disconnect the network, unlock a second achievement, reconnect, and verify
     the deferred unlock completes. Verify `SetAchievement` + `StoreStats` are called as a batch.
2. **R-14.7.2** — The engine **SHALL** submit and query ranked scores through Steam leaderboards via
   `ISteamUserStats`, supporting global, friends-only, and around-user queries. Query results
   **SHALL** be cached for at least 60 seconds to respect Steam rate limits. Score submissions
   **SHALL** be retried on transient failures.
   - **Rationale:** Caching prevents rate-limit violations that block leaderboard access. Retry
     ensures scores are not lost during transient network issues.
   - **Verification:** Submit 10 scores and query global and friends-only leaderboards; verify
     correct ranking. Simulate a transient failure and verify retry succeeds. Query the same
     leaderboard twice within 60 seconds and verify the second query returns cached results without
     an API call.
3. **R-14.7.3** — The engine **SHALL** process microtransactions through `ISteamMicroTxn`,
   initiating transactions via `InitTxn`, monitoring status through Steam overlay callbacks, and
   confirming with `FinalizeTxn`. All receipts **SHALL** be forwarded to the game server for
   server-side validation. Pending and failed transactions **SHALL** be tracked and retried.
   - **Rationale:** Server-side receipt validation prevents fraudulent transactions. Retry logic
     ensures no purchase is lost during transient failures.
   - **Verification:** Initiate a purchase, complete the overlay flow, and verify `FinalizeTxn`
     fires with correct product data. Verify the receipt is forwarded to the game server. Simulate a
     failed finalization and verify retry.
4. **R-14.7.4** — The engine **SHALL** support uploading, downloading, subscribing, rating, and
   reporting user-generated content through the Steam Workshop API (`ISteamUGC`). Mod authors
   **SHALL** publish via `CreateItem` and `SubmitItemUpdate`. Subscribed mods **SHALL**
   auto-download on launch via `DownloadItem`. Dependency resolution **SHALL** handle mods that
   require other mods.
   - **Rationale:** Workshop integration provides a managed distribution channel for user-generated
     content with built-in moderation and CDN delivery.
   - **Verification:** Upload a mod from the editor and verify it appears in the Workshop. Subscribe
     to a mod and verify it auto-downloads on launch. Upload a mod with a dependency and verify the
     dependency is resolved and downloaded.
5. **R-14.7.5** — The engine **SHALL** create and join lobbies, browse dedicated servers, and manage
   matchmaking through `ISteamMatchmaking` and `ISteamMatchmakingServers`. Lobby creation **SHALL**
   support public, friends-only, and invite-only visibility with custom metadata filters. Server
   browser queries **SHALL** filter by game mode, map, ping, player count, and custom tags.
   - **Rationale:** Steam matchmaking provides platform-native lobby and server browsing that
     players expect on the Steam platform.
   - **Verification:** Create a lobby with friends-only visibility and verify non-friends cannot see
     it. Browse servers filtered by game mode and verify only matching servers appear. Join a lobby
     and verify lobby chat works.
6. **R-14.7.6** — The engine **SHALL** integrate Valve Anti-Cheat on dedicated servers via
   `GSInitGameServer`. Client sessions **SHALL** be authenticated through Steam tickets carrying VAC
   status. VAC-banned players **SHALL** be rejected at connection time. VAC session state **SHALL**
   be exposed to the engine's anti-cheat system (R-8.8.1) for combined enforcement.
   - **Rationale:** VAC provides a baseline anti-cheat layer that supplements the engine's
     server-side validation without requiring custom client-side anti-cheat development.
   - **Verification:** Connect a VAC-banned Steam account and verify rejection. Connect a clean
     account and verify acceptance. Verify VAC status is passed to the engine's anti-cheat severity
     pipeline.
7. **R-14.7.7** — The engine **SHALL** access the Steam friend list through `ISteamFriends` and
   update rich presence via `SetRichPresence`. Rich presence updates **SHALL** be throttled to at
   most one per 15 seconds. Game invites **SHALL** be sent through `InviteUserToGame` and received
   via callback.
   - **Rationale:** Rich presence drives organic discovery. Throttling prevents API rate-limit
     violations.
   - **Verification:** Set rich presence and verify it appears on a friend's Steam client. Issue 10
     rapid updates within 5 seconds and verify only one API call is made. Send a game invite and
     verify the recipient receives it.
8. **R-14.7.8** — The engine **SHALL** persist player data to Steam Cloud through
   `ISteamRemoteStorage` or Steam Auto-Cloud. The engine **SHALL** respect per-user cloud quota
   limits. Conflict resolution **SHALL** use Steam's built-in resolution dialog.
   - **Rationale:** Steam Cloud ensures players retain settings and progress when switching
     machines.
   - **Verification:** Save settings on one machine, log in on another, and verify settings are
     synced. Write files exceeding the quota and verify the engine handles the error gracefully.
     Simulate a conflict and verify Steam's conflict dialog appears.
9. **R-14.7.9** — The engine **SHALL** capture and transmit voice through the Steam Voice API with
   push-to-talk and voice activity detection. Voice data **SHALL** integrate with the engine's audio
   mixer for spatial voice. Audio quality (sample rate, bitrate) **SHALL** be configurable.
   - **Rationale:** Steam Voice provides low-latency voice chat with built-in codec support,
     reducing integration complexity.
   - **Verification:** Enable push-to-talk, speak, and verify audio is received by another player.
     Enable voice activity detection and verify automatic transmission. Verify spatial voice
     attenuates with distance.

## Apple (iOS/macOS)

| ID        | Derived From                                            |
|-----------|---------------------------------------------------------|
| R-14.7.10 | [F-14.7.10](../../features/platform/sdk-integration.md) |
| R-14.7.11 | [F-14.7.11](../../features/platform/sdk-integration.md) |
| R-14.7.12 | [F-14.7.12](../../features/platform/sdk-integration.md) |
| R-14.7.13 | [F-14.7.13](../../features/platform/sdk-integration.md) |
| R-14.7.14 | [F-14.7.14](../../features/platform/sdk-integration.md) |
| R-14.7.15 | [F-14.7.15](../../features/platform/sdk-integration.md) |

1. **R-14.7.10** — The engine **SHALL** process in-app purchases on iOS and macOS through StoreKit
   2, supporting consumable items, non-consumable items, and auto-renewable subscriptions. All
   receipts **SHALL** be forwarded to the game server for validation using the App Store Server API.
   Interrupted purchases and pending transactions **SHALL** be handled gracefully. Family sharing
   entitlements **SHALL** be recognized.
   - **Rationale:** StoreKit 2 is required for iOS 15+ and provides JWS-signed receipts for
     server-side validation.
   - **Verification:** Purchase a consumable item and verify server-side validation succeeds.
     Purchase a non-consumable and verify the entitlement persists. Subscribe to an auto-renewable
     plan and verify renewal detection. Simulate an interrupted purchase and verify it completes on
     relaunch.
2. **R-14.7.11** — The engine **SHALL** unlock achievements and report progress through Game Center
   via `GKAchievement.report()`, mapping internal IDs to Game Center identifiers. Progress **SHALL**
   be reported as a percentage (0-100) for incremental achievements. Current state **SHALL** be
   fetched on session start.
   - **Rationale:** Game Center achievements are the standard achievement system for Apple
     platforms.
   - **Verification:** Unlock an achievement and verify the Game Center banner appears. Report 50%
     progress on an incremental achievement and verify the correct percentage is stored. Fetch
     achievements on session start and verify sync.
3. **R-14.7.12** — The engine **SHALL** submit scores and query leaderboards through Game Center,
   supporting classic and recurring types with global, friends, and around-player scopes. Players
   **SHALL** be able to issue challenges from leaderboard entries. Score submissions **SHALL** be
   batched.
   - **Rationale:** Game Center leaderboards provide platform-native competitive features with
     built-in friend filtering.
   - **Verification:** Submit a score and verify it appears on the global leaderboard. Query
     friends-only and verify correct filtering. Issue a challenge and verify the recipient receives
     it.
4. **R-14.7.13** — The engine **SHALL** create and join multiplayer matches through Game Center
   using `GKMatchmaker`, supporting real-time, turn-based, and auto-match modes. Match invites
   **SHALL** be sent through Game Center. Voice chat within matches **SHALL** use `GKVoiceChat`.
   - **Rationale:** Game Center matchmaking provides Apple's native multiplayer infrastructure with
     skill-based matching.
   - **Verification:** Create a real-time match with 4 players and verify all players connect.
     Create a turn-based match and verify turn progression. Use auto-match and verify skill-based
     pairing. Verify voice chat works within a match.
5. **R-14.7.14** — The engine **SHALL** sync lightweight data through iCloud Key-Value Storage (1 MB
   limit, 1024 keys) and structured saves through CloudKit. Conflict resolution **SHALL** use
   last-write-wins for key-value and CloudKit's server change token for structured data. External
   changes **SHALL** be detected via system notifications.
   - **Rationale:** iCloud provides Apple-native cloud storage that syncs across all Apple devices
     automatically.
   - **Verification:** Write a key-value pair and verify it syncs to a second device. Write
     structured data to CloudKit and verify read-back. Simulate a conflict and verify resolution.
     Verify the 1 MB key-value limit is enforced gracefully.
6. **R-14.7.15** — The engine **SHALL** request ATT authorization via
   `ATTrackingManager.requestTrackingAuthorization()` before any IDFA access on iOS 14.5+. If
   denied, the engine **SHALL** disable all behavioral advertising and cross-app attribution.
   Tracking status **SHALL** be cached per session.
   - **Rationale:** ATT compliance is mandatory for App Store submission. Non-compliance causes App
     Review rejection.
   - **Verification:** Deny tracking and verify no IDFA access occurs. Grant tracking and verify
     IDFA is available. Verify the ATT prompt is not shown on first launch. Verify the prompt
     appears before any ad SDK initialization.

## Google Play

| ID        | Derived From                                            |
|-----------|---------------------------------------------------------|
| R-14.7.16 | [F-14.7.16](../../features/platform/sdk-integration.md) |
| R-14.7.17 | [F-14.7.17](../../features/platform/sdk-integration.md) |
| R-14.7.18 | [F-14.7.18](../../features/platform/sdk-integration.md) |
| R-14.7.19 | [F-14.7.19](../../features/platform/sdk-integration.md) |
| R-14.7.20 | [F-14.7.20](../../features/platform/sdk-integration.md) |

1. **R-14.7.16** — The engine **SHALL** process in-app purchases and subscriptions on Android
   through Play Billing Library 7. Consumable purchases **SHALL** be consumed via `consumeAsync`.
   Non-consumable purchases **SHALL** be acknowledged via `acknowledgePurchase` within 3 days to
   prevent automatic refund. All purchase tokens **SHALL** be validated server-side via the Google
   Play Developer API.
   - **Rationale:** Play Billing Library 7 is required for Google Play distribution. Unacknowledged
     purchases are automatically refunded, making timely acknowledgment critical.
   - **Verification:** Purchase a consumable item and verify `consumeAsync` succeeds. Purchase a
     non-consumable and verify acknowledgment within 3 days. Verify server-side token validation
     succeeds. Simulate a pending purchase and verify it resolves correctly.
2. **R-14.7.17** — The engine **SHALL** unlock achievements (standard, incremental, hidden) and
   submit leaderboard scores through GPGS. Achievement progress **SHALL** be reported via
   `increment()` for incremental types. Leaderboard scores **SHALL** support daily, weekly, and
   all-time time spans.
   - **Rationale:** GPGS provides the standard achievement and leaderboard system for Android games
     on Google Play.
   - **Verification:** Unlock a standard achievement and verify it appears in the GPGS overlay.
     Increment a step-based achievement and verify progress. Submit a leaderboard score and verify
     ranking across all time spans.
3. **R-14.7.18** — The engine **SHALL** create and join multiplayer matches through GPGS real-time
   and turn-based multiplayer APIs. Match configuration **SHALL** include player count, auto-match
   criteria, and invitation handling. The engine **SHALL** handle connection state changes and
   player disconnections.
   - **Rationale:** GPGS matchmaking provides Google's native multiplayer infrastructure for Android
     games.
   - **Verification:** Create a real-time match with 4 players and verify connections. Create a
     turn-based match and verify turn progression. Disconnect a player and verify the engine handles
     the disconnection gracefully.
4. **R-14.7.19** — The engine **SHALL** request integrity verdicts via the Play Integrity API on
   session start and periodically during play. Integrity tokens **SHALL** be forwarded to the game
   server for decryption and validation. Failed integrity checks **SHALL** trigger configurable
   responses (warn, restrict, disconnect). Verdicts **SHALL** be cached for 5 minutes to avoid
   excessive API calls.
   - **Rationale:** Play Integrity verifies device and app authenticity, detecting rooted devices,
     modified APKs, and unlicensed installations.
   - **Verification:** Request an integrity verdict on a genuine device and verify it passes. Modify
     the APK signature and verify a failed app integrity verdict. Verify cached verdicts are reused
     within 5 minutes. Verify configurable responses fire on failure.
5. **R-14.7.20** — The engine **SHALL** persist player game state through the GPGS Saved Games API
   with metadata (description, play time, cover image). Conflict resolution **SHALL** use the
   engine's merge strategy or present a choice dialog. The engine **SHALL** respect the 3 MB data +
   800 KB cover image limits per snapshot.
   - **Rationale:** GPGS Saved Games provides cross-device cloud save through the player's Google
     account.
   - **Verification:** Save a game state and verify it syncs to a second device. Simulate a conflict
     and verify the merge dialog appears. Write a snapshot exceeding 3 MB and verify graceful error
     handling.

## Microsoft (Xbox/Windows)

| ID        | Derived From                                            |
|-----------|---------------------------------------------------------|
| R-14.7.21 | [F-14.7.21](../../features/platform/sdk-integration.md) |
| R-14.7.22 | [F-14.7.22](../../features/platform/sdk-integration.md) |
| R-14.7.23 | [F-14.7.23](../../features/platform/sdk-integration.md) |
| R-14.7.24 | [F-14.7.24](../../features/platform/sdk-integration.md) |
| R-14.7.25 | [F-14.7.25](../../features/platform/sdk-integration.md) |
| R-14.7.26 | [F-14.7.26](../../features/platform/sdk-integration.md) |
| R-14.7.27 | [F-14.7.27](../../features/platform/sdk-integration.md) |

1. **R-14.7.21** — The engine **SHALL** unlock achievements on Xbox and Windows through event-based
   stat writes via `XblEventsWriteInGameEvent`. Achievement rules **SHALL** be evaluated server-side
   by Xbox Live services. The engine **SHALL** fetch current achievement state on session start.
   Standard, progressive, and challenge achievement types **SHALL** be supported.
   - **Rationale:** Xbox's event-based achievement system ensures server-side validation of unlock
     conditions, preventing client-side manipulation.
   - **Verification:** Write a stat event that triggers an achievement and verify the unlock
     notification appears. Verify progressive achievement progress updates correctly. Fetch
     achievements on session start and verify state sync.
2. **R-14.7.22** — The engine **SHALL** submit player stats and query leaderboard rankings through
   Xbox Live, supporting global, social, and around-player queries. Stats **SHALL** be written via
   `XblEventsWriteInGameEvent` with aggregation rules configured in Partner Center.
   - **Rationale:** Xbox leaderboards derive from stats, ensuring consistent ranking based on
     server-computed aggregations.
   - **Verification:** Write stats and query the global leaderboard; verify correct ranking. Query
     social (friends-only) leaderboard and verify filtering. Verify stats respect Partner Center
     aggregation rules.
3. **R-14.7.23** — The engine **SHALL** process purchases through the Microsoft Store using
   `XStoreShowPurchaseUIAsync`, supporting consumable add-ons, durable add-ons, and subscriptions.
   Entitlement checks **SHALL** use `XStoreQueryAddOnLicensesAsync`. Receipts **SHALL** be validated
   server-side through the Microsoft Store collections API.
   - **Rationale:** Microsoft Store integration is required for Xbox and Windows Store distribution
     with unified cross-device entitlements.
   - **Verification:** Purchase a consumable add-on and verify server-side validation. Purchase a
     durable add-on and verify the entitlement persists across sessions. Subscribe and verify
     recurring verification detects lapse.
4. **R-14.7.24** — The engine **SHALL** create multiplayer sessions and find matches through
   SmartMatch via `XblMatchmakingCreateMatchTicketAsync`. SmartMatch **SHALL** evaluate tickets
   server-side using QoS, skill ratings, and custom rules. The engine **SHALL** monitor ticket
   status and support 1v1, team, and large-session match types.
   - **Rationale:** SmartMatch provides server-side matchmaking with QoS-aware player grouping,
     ensuring quality matches.
   - **Verification:** Create a match ticket and verify SmartMatch returns a matched session. Verify
     QoS measurements influence match quality. Test 1v1 and team match types.
5. **R-14.7.25** — The engine **SHALL** detect Game Pass membership via
   `XStoreQueryEntitledProductsAsync` and differentiate Game Pass access from ownership. Game Pass
   members **SHALL** receive configurable benefits. Game Pass status **SHALL** be re-verified
   periodically to detect subscription changes.
   - **Rationale:** Game Pass integration is required for all titles in the Game Pass catalog.
     Differentiating access type enables targeted promotions and analytics.
   - **Verification:** Verify a Game Pass member gains access without purchase. Verify Game Pass
     benefits are applied. Simulate subscription lapse and verify access is revoked within one
     verification interval.
6. **R-14.7.26** — The engine **SHALL** persist player data to Xbox Connected Storage via
   `XGameSaveInitializeProviderAsync`. Save data **SHALL** sync across Xbox consoles and Windows
   PCs. The engine **SHALL** respect per-title storage quotas (256 MB Xbox, 64 MB PC default).
   Conflict resolution **SHALL** use Xbox's built-in sync resolution.
   - **Rationale:** Connected Storage provides Xbox's mandatory cross-device save sync for all Xbox
     Live-enabled titles.
   - **Verification:** Save data on Xbox, load on PC, and verify sync. Write data exceeding the
     quota and verify graceful error handling. Simulate a conflict and verify Xbox's resolution
     dialog appears.
7. **R-14.7.27** — The engine **SHALL** integrate voice and text chat through Xbox Game Chat 2 with
   push-to-talk, open mic, text-to-speech, and speech-to-text. Mute, volume control, and voice
   activity indicators **SHALL** be surfaced to the game UI. XAG compliance for communication
   **SHALL** be enforced. Text-to-speech and speech-to-text **SHALL** be available per XR-015.
   - **Rationale:** Game Chat 2 handles Xbox certification requirements for voice communication,
     including mandatory accessibility features.
   - **Verification:** Verify push-to-talk and open mic modes transmit audio. Verify text-to-speech
     reads chat messages aloud. Verify speech-to-text transcribes voice input. Verify mute controls
     work per player.

## PlayStation

| ID        | Derived From                                            |
|-----------|---------------------------------------------------------|
| R-14.7.28 | [F-14.7.28](../../features/platform/sdk-integration.md) |
| R-14.7.29 | [F-14.7.29](../../features/platform/sdk-integration.md) |
| R-14.7.30 | [F-14.7.30](../../features/platform/sdk-integration.md) |
| R-14.7.31 | [F-14.7.31](../../features/platform/sdk-integration.md) |
| R-14.7.32 | [F-14.7.32](../../features/platform/sdk-integration.md) |
| R-14.7.33 | [F-14.7.33](../../features/platform/sdk-integration.md) |

1. **R-14.7.28** — The engine **SHALL** unlock trophies through `sceNpTrophyUnlockTrophy`, mapping
   internal achievement IDs to trophy IDs. Trophy context **SHALL** be registered on startup via
   `sceNpTrophyCreateContext`. Trophy packs **SHALL** be embedded in the game package per TRC R4060.
   Platinum trophy **SHALL** be awarded automatically when all base-game trophies are unlocked.
   - **Rationale:** Trophy support is mandatory for all PlayStation titles per TRC R4060. Failure to
     include trophies blocks certification.
   - **Verification:** Unlock a trophy and verify the notification appears. Unlock all base-game
     trophies and verify the platinum trophy is awarded automatically. Verify trophy IDs match the
     trophy pack configuration.
2. **R-14.7.29** — The engine **SHALL** process in-app purchases through PlayStation Store commerce
   APIs. Entitlements **SHALL** be verified through `sceNpEntitlementAccess`. Server-side receipt
   validation **SHALL** use the PlayStation Store server API. Wallet funding prompts **SHALL** be
   displayed when the balance is insufficient. Age rating gates **SHALL** apply per TRC R4082.
   - **Rationale:** PlayStation Store integration is required for all monetized content on
     PlayStation platforms.
   - **Verification:** Purchase a consumable and verify server-side validation. Attempt a purchase
     with insufficient wallet funds and verify the funding prompt. Verify age rating gate blocks
     underage purchases per TRC R4082.
3. **R-14.7.30** — The engine **SHALL** submit and query ranked scores through PSN leaderboard APIs,
   supporting global and friends-only queries. Query results **SHALL** be cached to respect PSN rate
   limits. Score submissions **SHALL** support optional data attachments.
   - **Rationale:** PSN leaderboards provide platform-native competitive rankings visible on the
     PlayStation profile.
   - **Verification:** Submit a score and verify it appears on the global leaderboard. Query
     friends-only and verify filtering. Submit a score with a data attachment and verify the
     attachment is retrievable.
4. **R-14.7.31** — The engine **SHALL** create and join multiplayer sessions through PSN matchmaking
   APIs with skill-based matching, region filtering, and custom attribute matching. Session invites
   **SHALL** be sent through the PSN social overlay. Session lifecycle events **SHALL** be handled
   (join, leave, disconnect, migration).
   - **Rationale:** PSN matchmaking is the standard multiplayer infrastructure for PlayStation
     titles.
   - **Verification:** Create a session and verify another player can join. Send an invite and
     verify the recipient receives it. Disconnect the host and verify session migration completes.
5. **R-14.7.32** — The engine **SHALL** persist save data through `sceNpSaveData` with cloud sync
   for PlayStation Plus subscribers. Per-title save data quotas **SHALL** be respected. Save data
   icon and description metadata **SHALL** be provided per TRC R4073. Conflict resolution **SHALL**
   use the standard PlayStation dialog.
   - **Rationale:** Save data with cloud sync is expected by PlayStation players. TRC R4073 mandates
     metadata for save data management.
   - **Verification:** Save data and verify cloud sync on a second console. Verify save data icon
     and description appear in the system save data manager. Write data exceeding the quota and
     verify graceful handling.
6. **R-14.7.33** — The engine **SHALL** publish activity cards on the PS5 home screen through the
   Activities API, showing in-progress objectives, joinable sessions, and estimated completion
   times. Deep links **SHALL** resolve to the correct game state within 10 seconds. Activities
   **SHALL** be implemented per TRC R5070.
   - **Rationale:** Activities are mandatory for PS5 titles per TRC R5070. They provide direct
     navigation to game content from the home screen.
   - **Verification:** Publish an activity card and verify it appears on the PS5 home screen. Launch
     from an activity deep link and verify the game reaches the correct state within 10 seconds.
     Verify Universal Search returns the activity.

## Nintendo Switch

| ID        | Derived From                                            |
|-----------|---------------------------------------------------------|
| R-14.7.34 | [F-14.7.34](../../features/platform/sdk-integration.md) |
| R-14.7.35 | [F-14.7.35](../../features/platform/sdk-integration.md) |
| R-14.7.36 | [F-14.7.36](../../features/platform/sdk-integration.md) |
| R-14.7.37 | [F-14.7.37](../../features/platform/sdk-integration.md) |

1. **R-14.7.34** — The engine **SHALL** integrate NSO services for online multiplayer, voice chat,
   and cloud backup. NSO membership **SHALL** be verified at session start before any online feature
   access per Lotcheck guidelines. Membership status changes **SHALL** be handled during gameplay
   with UI prompts.
   - **Rationale:** NSO membership verification is mandatory per Lotcheck. Online features must be
     gated on active membership.
   - **Verification:** Verify an NSO subscriber can access online features. Verify a non-subscriber
     is blocked with a membership prompt. Expire NSO membership mid-session and verify the engine
     displays a prompt.
2. **R-14.7.35** — The engine **SHALL** process in-app purchases through Nintendo eShop APIs for
   consumable and non-consumable items. Server-side receipt validation **SHALL** use the eShop
   server API. Insufficient funds **SHALL** redirect to the eShop funds page. Purchase flow UX
   **SHALL** comply with Lotcheck requirements.
   - **Rationale:** eShop integration is required for all monetized content on Nintendo Switch.
   - **Verification:** Purchase a consumable and verify server-side validation. Attempt a purchase
     with insufficient funds and verify redirect to the eShop funds page. Verify the purchase flow
     meets Lotcheck UX requirements.
3. **R-14.7.36** — The engine **SHALL** write save files to the system save data area for automatic
   NSO cloud backup. Per-title save data size limits **SHALL** be respected. Conflict resolution
   **SHALL** use the system-level save data cloud sync dialog.
   - **Rationale:** NSO Save Data Cloud provides cross-device save sync for Nintendo Switch titles.
   - **Verification:** Save data and verify cloud backup on an NSO subscriber account. Verify save
     data size limits are enforced. Simulate a conflict and verify the system dialog appears.
4. **R-14.7.37** — The engine **SHALL** create and join multiplayer sessions through NPLN with
   configurable player count, region preferences, and custom attributes. Session invites **SHALL**
   be sent through the Switch friend system. Host migration **SHALL** be handled on disconnect. NAT
   traversal **SHALL** use Nintendo's relay infrastructure.
   - **Rationale:** NPLN provides Nintendo's official online matchmaking infrastructure for Switch
     titles.
   - **Verification:** Create a session and verify another player can join via invite. Disconnect
     the host and verify host migration completes. Verify NAT traversal succeeds for players behind
     restricted NAT types.

## Epic Online Services (EOS)

| ID        | Derived From                                            |
|-----------|---------------------------------------------------------|
| R-14.7.38 | [F-14.7.38](../../features/platform/sdk-integration.md) |
| R-14.7.39 | [F-14.7.39](../../features/platform/sdk-integration.md) |
| R-14.7.40 | [F-14.7.40](../../features/platform/sdk-integration.md) |
| R-14.7.41 | [F-14.7.41](../../features/platform/sdk-integration.md) |
| R-14.7.42 | [F-14.7.42](../../features/platform/sdk-integration.md) |
| R-14.7.43 | [F-14.7.43](../../features/platform/sdk-integration.md) |

1. **R-14.7.38** — The engine **SHALL** unlock achievements through
   `EOS_Achievements_UnlockAchievements` and update stats through `EOS_Stats_IngestStat`.
   Achievement definitions **SHALL** be configured in the Epic Developer Portal. Current state
   **SHALL** be fetched on session start via `EOS_Achievements_QueryPlayerAchievements`.
   - **Rationale:** EOS achievements provide cross-platform achievement tracking through Epic's
     infrastructure.
   - **Verification:** Unlock an achievement and verify it appears in the EOS overlay. Ingest a stat
     and verify the value updates. Fetch achievements on session start and verify state sync.
2. **R-14.7.39** — The engine **SHALL** submit stats and query leaderboard rankings through EOS
   Leaderboards, supporting global and friends-only scopes. Leaderboard definitions **SHALL** be
   configured in the Epic Developer Portal. Query results **SHALL** be cached to respect API rate
   limits.
   - **Rationale:** EOS leaderboards provide cross-platform competitive rankings through stat-driven
     server-side computation.
   - **Verification:** Ingest stats and query the global leaderboard; verify correct ranking. Query
     friends-only and verify filtering. Verify cached results are served for repeated queries.
3. **R-14.7.40** — The engine **SHALL** create and manage multiplayer sessions through EOS Lobbies
   (up to 64 members) and EOS Sessions. Lobby creation **SHALL** support configurable member limits,
   permissions, and custom attributes. Invitations **SHALL** be sent through `EOS_Lobby_SendInvite`.
   Member join/leave and ownership transfer **SHALL** be handled.
   - **Rationale:** EOS Lobbies and Sessions provide cross-platform multiplayer infrastructure
     usable on all platforms.
   - **Verification:** Create a lobby and verify another player can join via invite. Verify custom
     attribute filtering works. Disconnect the lobby owner and verify ownership transfer.
4. **R-14.7.41** — The engine **SHALL** integrate voice and text chat through EOS RTC. Voice rooms
   **SHALL** support mute, volume control, voice activity indicators, text-to-speech, and
   speech-to-text.
   - **Rationale:** EOS Voice provides cross-platform voice chat without requiring platform-specific
     voice SDKs.
   - **Verification:** Join a voice room and verify two-way audio. Mute a player and verify their
     audio is silenced. Verify text-to-speech reads chat messages. Verify speech-to-text transcribes
     voice input.
5. **R-14.7.42** — The engine **SHALL** integrate Easy Anti-Cheat via
   `EOS_AntiCheatServer_BeginSession` on servers and `EOS_AntiCheatClient_BeginSession` on clients.
   Action notifications from EAC (kick, ban) **SHALL** be processed through the engine's anti-cheat
   severity pipeline. Client integrity messages **SHALL** be relayed between client and server.
   - **Rationale:** EAC provides client-side anti-cheat that supplements the engine's server-side
     validation with kernel-level tamper detection.
   - **Verification:** Connect a client with EAC active and verify integrity messages are processed.
     Trigger an EAC kick event and verify the engine disconnects the client. Verify EAC session
     state is reported to the anti-cheat severity pipeline.
6. **R-14.7.43** — The engine **SHALL** persist per-player data to EOS cloud storage through
   `EOS_PlayerDataStorage_WriteFile` and `EOS_PlayerDataStorage_ReadFile`. Files **SHALL** be
   encrypted at rest with per-user isolation. File operations **SHALL** be asynchronous with
   progress callbacks. The 200 MB per-file and 4 GB per-user limits **SHALL** be respected.
   - **Rationale:** EOS Player Data Storage provides cross-platform encrypted cloud save accessible
     on all EOS-supported platforms.
   - **Verification:** Write a file and read it back; verify data integrity. Verify async progress
     callbacks fire during large file writes. Attempt to write a file exceeding 200 MB and verify
     graceful error handling.

## Cross-Platform

| ID        | Derived From                                            |
|-----------|---------------------------------------------------------|
| R-14.7.44 | [F-14.7.44](../../features/platform/sdk-integration.md) |
| R-14.7.45 | [F-14.7.45](../../features/platform/sdk-integration.md) |
| R-14.7.46 | [F-14.7.46](../../features/platform/sdk-integration.md) |

1. **R-14.7.44** — The engine **SHALL** provide platform-agnostic Rust traits with async methods for
   each service category (achievements, leaderboards, IAP, matchmaking, cloud save, voice,
   anti-cheat, friends). Platform-specific backends **SHALL** implement these traits using the
   corresponding SDK. Backend selection **SHALL** be automatic based on the compile target and
   runtime environment. The abstraction layer **SHALL** handle SDK init, shutdown, error mapping,
   and retry logic. The abstraction **SHALL** add no measurable overhead beyond the underlying SDK
   call.
   - **Rationale:** A unified abstraction prevents gameplay code from branching per platform and
     centralizes retry, error mapping, and initialization logic.
   - **Verification:** Call the achievement trait on each platform backend and verify the correct
     SDK API is invoked. Measure abstraction layer overhead and verify it is within 1% of the raw
     SDK call. Verify compile-time backend selection links only the active platform SDK.
2. **R-14.7.45** — The engine **SHALL** synchronize player progression across platforms for players
   with linked accounts. A canonical progression record **SHALL** be maintained on the game server,
   merging achievement state, purchase entitlements, and save data. Account linking **SHALL** map
   platform identities to a single game account. Cross-buy entitlements **SHALL** grant access on
   all linked platforms where the content is available.
   - **Rationale:** Cross-platform progression retains players who switch platforms, reducing churn
     from lost progress.
   - **Verification:** Link a Steam and Xbox account; unlock an achievement on Steam and verify it
     syncs to the game server. Log in on Xbox and verify the achievement is reflected. Purchase
     content on one platform and verify cross-buy grants access on the linked platform.
3. **R-14.7.46** — The engine **SHALL** detect the active platform at runtime on PC by checking
   launcher-injected environment variables, overlay DLLs, and process parent information. The
   correct SDK backend **SHALL** be activated without requiring separate builds per storefront.
   Fallback behavior **SHALL** be defined for standalone/DRM-free builds where no platform SDK is
   detected.
   - **Rationale:** Hot-switching enables a single PC binary to ship on multiple storefronts,
     reducing build complexity and distribution overhead.
   - **Verification:** Launch from Steam and verify the Steam backend activates. Launch from Epic
     and verify the EOS backend activates. Launch standalone without any storefront and verify
     fallback behavior. Verify no separate builds are required.

## Non-Functional Requirements

| ID         | Derived From |
|------------|--------------|
| NFR-14.7.1 |              |
| NFR-14.7.2 |              |
| NFR-14.7.3 |              |
| NFR-14.7.4 |              |

1. **NFR-14.7.1** — All platform SDK calls **SHALL** complete within 100ms for local operations
   (achievement unlock, stat write, rich presence update). Network-bound SDK calls (leaderboard
   query, cloud save sync, receipt validation) **SHALL** complete within 2 seconds under normal
   network conditions. SDK calls **SHALL NOT** block the main game thread.
   - **Rationale:** SDK call latency must not impact frame rate or gameplay responsiveness.
   - **Verification:** Measure achievement unlock latency across 100 calls and verify p99 is under
     100ms. Measure leaderboard query latency and verify p99 is under 2 seconds. Verify all SDK
     calls execute on background threads.
2. **NFR-14.7.2** — When a platform service is unavailable (network disconnection, service outage,
   maintenance window), the engine **SHALL** queue all pending operations (achievement unlocks,
   score submissions, cloud save writes) and retry automatically on reconnection. Queued operations
   **SHALL** persist across session restarts. The game **SHALL** remain fully playable in offline
   mode with degraded platform features.
   - **Rationale:** Players must not lose progress or be blocked from playing due to transient
     platform service outages.
   - **Verification:** Disconnect the network mid-session; unlock achievements and submit scores.
     Reconnect and verify all queued operations complete. Kill the process while offline; relaunch
     and verify the queue persists and drains on reconnection.
3. **NFR-14.7.3** — All in-app purchase receipts **SHALL** be validated server-side before granting
   items or currency. Duplicate receipt replay **SHALL** be detected and rejected within 100ms. The
   validation pipeline **SHALL** support all platform receipt formats (Apple JWS, Google purchase
   token, Steam ISteamMicroTxn, Microsoft Store collections, PlayStation Store, Nintendo eShop,
   EOS).
   - **Rationale:** Server-side validation prevents fraudulent purchases and receipt replay attacks
     across all platforms.
   - **Verification:** Complete a purchase on each platform and verify server-side validation
     succeeds. Replay the same receipt and verify rejection within 100ms. Verify the server handles
     all platform receipt formats.
4. **NFR-14.7.4** — The engine **SHALL** meet all mandatory certification requirements for each
   target platform:
   - **Rationale:** Certification failures block store submission. Engine-level compliance
     eliminates per-title certification work.
   - **Verification:** Run each platform's automated certification test suite and verify all
     mandatory checks pass. Submit a test build to each platform's certification review and verify
     no blockers are raised.

| Platform | Certification | Key requirements |
|----------|--------------|------------------|
| PlayStation | TRC | R4060 (trophies), R4073 (save data), R4082 (age rating), R5070 (activities) |
| Xbox | XR | XR-015 (accessibility), XR-022 (multiplayer), XR-074 (achievements) |
| Nintendo | Lotcheck | NSO gating, purchase flow UX, save data compliance |
| Apple | App Review | ATT compliance, StoreKit 2, App Store guidelines |
| Google | Play Policy | Play Billing compliance, Play Integrity, content rating |

## Server-Side Proprietary SDK Isolation (R-14.8)

| ID | Derived From |
|-----|--------------|
| R-14.8.1 | [F-14.8.1](../../features/platform/sdk-integration.md), [F-14.8.2](../../features/platform/sdk-integration.md) |
| R-14.8.2 | [F-14.8.1](../../features/platform/sdk-integration.md) |
| R-14.8.3 | [F-14.8.2](../../features/platform/sdk-integration.md) |
| R-14.8.4 | [F-14.8.2](../../features/platform/sdk-integration.md) |
| R-14.8.5 | [F-14.8.3](../../features/platform/sdk-integration.md) |
| R-14.8.6 | [F-14.8.3](../../features/platform/sdk-integration.md) |
| R-14.8.7 | [F-14.8.4](../../features/platform/sdk-integration.md) |
| R-14.8.8 | [F-14.8.4](../../features/platform/sdk-integration.md) |
| R-14.8.9 | [F-14.8.5](../../features/platform/sdk-integration.md) |
| R-14.8.10 | [F-14.8.5](../../features/platform/sdk-integration.md) |

1. **R-14.8.1** — The engine client **SHALL** contain zero proprietary SDK code, headers, or
   libraries. The open-source engine binary **SHALL** compile and link without any console SDK
   installed on the developer's machine.
   - **Rationale:** Enables the engine to be fully open source. Developers contribute to and build
     the engine without NDA-encumbered dependencies.
   - **Verification:** Build the engine from source on a clean machine with no console SDKs
     installed and verify it compiles and links without errors. Scan all source files and verify
     zero references to proprietary SDK headers.
2. **R-14.8.2** — Console builds **SHALL** be triggered via an authenticated REST API from the
   editor to the build server. The API **SHALL** accept a project identifier, target platform (PS5,
   Xbox Series, Switch), build profile, and source revision. The API **SHALL** return a job ID for
   status polling.
   - **Rationale:** REST API decouples the open-source client from proprietary server-side build
     logic. Authentication prevents unauthorized access to licensed SDK infrastructure.
   - **Verification:** Submit a console build via the REST API with valid credentials and verify a
     job ID is returned. Submit with invalid credentials and verify rejection with HTTP 401.
3. **R-14.8.3** — All proprietary SDK headers, libraries, and tools (PlayStation SDK, Xbox GDK,
   Nintendo SDK) **SHALL** reside exclusively on the build server. The build server **SHALL** load
   console SDKs from a secure, access-controlled directory. SDK files **SHALL NOT** be downloadable
   via any API endpoint.
   - **Rationale:** Proprietary SDKs are NDA-protected. Restricting them to the server prevents
     accidental distribution and license violations.
   - **Verification:** Attempt to access SDK files via all server API endpoints and verify all
     return HTTP 403. Verify SDK directories have restrictive filesystem permissions.
4. **R-14.8.4** — Individual developers **SHALL NOT** require console SDK licenses to build, test,
   or iterate on gameplay code using the engine. Only the build server operator **SHALL** hold
   console SDK licenses.
   - **Rationale:** Reduces licensing cost from per-developer to per-server. Enables open-source
     contributors to work on the engine without console manufacturer approval.
   - **Verification:** Set up a developer workstation with no console SDK licenses. Verify all
     engine features except console packaging are fully functional.
5. **R-14.8.5** — The build server **SHALL** support concurrent builds from multiple projects with
   per-project resource isolation. One project's source code, intermediate files, and artifacts
   **SHALL NOT** be accessible to another project's build jobs.
   - **Rationale:** Studios sharing a build server must not risk source code leakage between
     projects.
   - **Verification:** Submit builds from two different projects concurrently. Verify each build
     uses isolated working directories. Verify project A's job cannot read project B's files.
6. **R-14.8.6** — The build server **SHALL** provide a priority-based job queue supporting
   configurable priority per project and FIFO ordering within the same priority level. Queue depth
   and wait times **SHALL** be exposed via a monitoring API.
   - **Rationale:** Priority scheduling ensures urgent builds (certification submissions) are
     processed before routine builds. Monitoring enables capacity planning.
   - **Verification:** Submit a low-priority and high-priority job simultaneously. Verify the
     high-priority job is dequeued first. Query the monitoring API and verify queue depth is
     accurate.
7. **R-14.8.7** — The build server **SHALL** deploy signed console packages to network-accessible
   dev kits upon request from the editor, using the console manufacturer's deployment tools
   installed on the server.
   - **Rationale:** Developers test on console hardware without installing proprietary deployment
     tools locally.
   - **Verification:** Trigger a deploy from the editor to a connected PS5 dev kit via the build
     server. Verify the package installs and launches on the dev kit.
8. **R-14.8.8** — Console output from dev kits **SHALL** stream back to the editor through the build
   server relay with latency under 500 ms. The relay **SHALL** support multiple concurrent streaming
   sessions.
   - **Rationale:** Real-time console output is essential for debugging. The relay avoids requiring
     proprietary console tools on the developer's machine.
   - **Verification:** Launch a game on a dev kit via the build server. Verify console output
     appears in the editor within 500 ms. Open two concurrent sessions and verify both stream
     independently.
9. **R-14.8.9** — The build server **SHALL** produce code-signed console packages and store them in
   S3-compatible object storage keyed by content hash. Artifacts **SHALL** include a build manifest
   recording source revision, engine version, SDK version, and build timestamp.
   - **Rationale:** Content-hash keys enable deduplication and cache hits across identical builds.
     Build manifests provide full traceability for certification.
   - **Verification:** Build the same source twice and verify the content hash is identical
     (deterministic builds). Retrieve the build manifest and verify all metadata fields are
     populated.
10. **R-14.8.10** — Console build artifacts **SHALL** be downloadable by authenticated users from
    S3-compatible storage. Artifact retention policies **SHALL** automatically delete builds older
    than a configurable threshold (default 90 days). Active builds pinned for certification
    **SHALL** be exempt from retention deletion.
    - **Rationale:** Automated retention prevents unbounded storage growth. Certification pins
      ensure submission builds are preserved.
    - **Verification:** Download an artifact via the editor and verify integrity. Set retention to 1
      day, wait, and verify expired unpinned builds are deleted. Pin a build and verify it survives
      retention.
