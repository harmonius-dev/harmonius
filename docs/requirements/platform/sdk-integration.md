# R-14.7 — Platform SDK Integration Requirements

## Steam (Steamworks)

1. **R-14.7.1** — The engine **SHALL** unlock Steam achievements and update stats through
   `ISteamUserStats`, mapping internal IDs to Steam identifiers. Deferred unlocks **SHALL** be
   queued when unavailable and retried on reconnection. Initial sync **SHALL** fetch current state
   on session start.
   - **Rationale:** ID mapping decouples gameplay from the Steam SDK. Deferred unlocks prevent loss
     during network outages.
   - **Verification:** Unlock while online; verify on Steam. Disconnect, unlock, reconnect; verify
     deferred unlock completes. Verify batch `StoreStats` call.

2. **R-14.7.2** — The engine **SHALL** submit and query ranked scores through Steam leaderboards,
   supporting global, friends-only, and around-user queries. Results **SHALL** be cached for at
   least 60 seconds. Submissions **SHALL** be retried on transient failures.
   - **Rationale:** Caching prevents rate-limit violations. Retry ensures no score is lost.
   - **Verification:** Submit 10 scores, query leaderboards, verify ranking. Simulate failure,
     verify retry. Query twice within 60 seconds, verify cache hit.

3. **R-14.7.3** — The engine **SHALL** process microtransactions through `ISteamMicroTxn`. All
   receipts **SHALL** be forwarded to the game server for validation. Pending and failed
   transactions **SHALL** be tracked and retried.
   - **Rationale:** Server-side validation prevents fraud. Retry ensures no purchase is lost.
   - **Verification:** Complete a purchase, verify `FinalizeTxn` fires with correct data. Verify
     server-side receipt forwarding. Simulate failure, verify retry.

4. **R-14.7.4** — The engine **SHALL** support uploading, downloading, subscribing, rating, and
   reporting UGC through Steam Workshop (`ISteamUGC`). Subscribed mods **SHALL** auto-download on
   launch. Dependency resolution **SHALL** be handled.
   - **Rationale:** Workshop provides managed UGC distribution with CDN delivery.
   - **Verification:** Upload a mod, verify it appears. Subscribe, verify auto-download. Upload with
     dependency, verify resolution.

5. **R-14.7.5** — The engine **SHALL** create and join lobbies, browse servers, and manage
   matchmaking through `ISteamMatchmaking`. Lobbies **SHALL** support public, friends-only, and
   invite-only visibility with custom metadata filters.
   - **Rationale:** Steam matchmaking provides platform-native lobby and server browsing.
   - **Verification:** Create friends-only lobby, verify non-friends cannot see it. Browse servers
     with filters, verify matches. Join lobby, verify chat works.

6. **R-14.7.6** — The engine **SHALL** integrate VAC on dedicated servers. VAC-banned players
   **SHALL** be rejected at connection. VAC state **SHALL** be exposed to the engine anti-cheat
   system.
   - **Rationale:** VAC supplements engine-side anti-cheat without custom client-side development.
   - **Verification:** Connect banned account, verify rejection. Connect clean account, verify
     acceptance. Verify VAC status reaches anti-cheat pipeline.

7. **R-14.7.7** — The engine **SHALL** access the Steam friend list and update rich presence,
   throttled to at most one update per 15 seconds. Game invites **SHALL** be sent through
   `InviteUserToGame`.
   - **Rationale:** Rich presence drives discovery. Throttling prevents rate-limit violations.
   - **Verification:** Set rich presence, verify on friend's client. Issue 10 rapid updates in 5
     seconds, verify only one API call. Send invite, verify receipt.

8. **R-14.7.8** — The engine **SHALL** persist player data to Steam Cloud through Auto-Cloud or
   `ISteamRemoteStorage`, respecting quota limits. Conflict resolution **SHALL** use Steam's dialog.
   - **Rationale:** Steam Cloud ensures preferences roam across machines.
   - **Verification:** Save on one machine, verify sync on another. Exceed quota, verify graceful
     error. Simulate conflict, verify dialog.

9. **R-14.7.9** — The engine **SHALL** capture and transmit voice through the Steam Voice API with
   push-to-talk and voice activity detection. Voice **SHALL** integrate with the engine audio mixer
   for spatial voice. Quality **SHALL** be configurable.
   - **Rationale:** Steam Voice provides low-latency voice with built-in codec support.
   - **Verification:** Enable push-to-talk, verify reception. Enable voice activity, verify auto
     transmission. Verify spatial attenuation.

## Apple (iOS/macOS)

10. **R-14.7.10** — The engine **SHALL** process IAP through StoreKit 2, supporting consumables,
    non-consumables, and auto-renewable subscriptions. Receipts **SHALL** be forwarded for
    server-side validation. Interrupted and pending transactions **SHALL** be handled. Family
    sharing **SHALL** be recognized.
    - **Rationale:** StoreKit 2 is required for iOS 15+ with JWS-signed receipts.
    - **Verification:** Purchase consumable, verify server validation. Purchase non-consumable,
      verify persistent entitlement. Subscribe, verify renewal detection. Simulate interruption,
      verify completion on relaunch.

11. **R-14.7.11** — The engine **SHALL** unlock achievements through Game Center via
    `GKAchievement.report()`, mapping internal IDs to Game Center identifiers. Incremental progress
    **SHALL** be reported as percentage (0-100).
    - **Rationale:** Game Center is the standard Apple achievement system.
    - **Verification:** Unlock achievement, verify banner. Report 50 % progress, verify stored.
      Fetch on session start, verify sync.

12. **R-14.7.12** — The engine **SHALL** submit scores and query Game Center leaderboards supporting
    classic and recurring types. Players **SHALL** be able to issue challenges. Submissions
    **SHALL** be batched.
    - **Rationale:** Game Center leaderboards provide platform-native competitive features.
    - **Verification:** Submit score, verify global ranking. Query friends-only, verify filtering.
      Issue challenge, verify receipt.

13. **R-14.7.13** — The engine **SHALL** create and join multiplayer matches through Game Center
    using `GKMatchmaker`, supporting real-time, turn-based, and auto-match modes. Voice chat
    **SHALL** use `GKVoiceChat`.
    - **Rationale:** Game Center matchmaking provides Apple's native multiplayer infrastructure.
    - **Verification:** Create 4-player real-time match, verify connections. Create turn-based
      match, verify progression. Verify voice chat.

14. **R-14.7.14** — The engine **SHALL** sync data through iCloud Key-Value Storage (1 MB limit) and
    CloudKit. Conflict resolution **SHALL** use last-write-wins for key-value and CloudKit server
    change tokens for structured data.
    - **Rationale:** iCloud provides Apple-native cloud storage that syncs across Apple devices.
    - **Verification:** Write key-value, verify sync to second device. Write CloudKit data, verify
      read-back. Simulate conflict, verify resolution.

15. **R-14.7.15** — The engine **SHALL** request ATT authorization before any IDFA access on iOS
    14.5+. If denied, all IDFA-dependent analytics **SHALL** be disabled. Tracking status **SHALL**
    be cached per session.
    - **Rationale:** ATT compliance is mandatory for App Store submission.
    - **Verification:** Deny tracking, verify no IDFA access. Grant tracking, verify IDFA available.
      Verify prompt is not shown on first launch.

## Google Play

16. **R-14.7.16** — The engine **SHALL** process IAP through Play Billing Library 7. Consumables
    **SHALL** be consumed via `consumeAsync`. Non-consumables **SHALL** be acknowledged within 3
    days. Purchase tokens **SHALL** be validated server-side.
    - **Rationale:** Play Billing 7 is required for Google Play. Unacknowledged purchases are
      automatically refunded.
    - **Verification:** Purchase consumable, verify consume. Purchase non-consumable, verify
      acknowledge. Verify server-side validation.

17. **R-14.7.17** — The engine **SHALL** unlock achievements and submit leaderboard scores through
    GPGS, supporting standard, incremental, and hidden types with daily, weekly, and all-time time
    spans.
    - **Rationale:** GPGS is the standard Android achievement and leaderboard system.
    - **Verification:** Unlock standard, verify in overlay. Increment step-based, verify progress.
      Submit score, verify ranking across time spans.

18. **R-14.7.18** — The engine **SHALL** create and join multiplayer matches through GPGS real-time
    and turn-based APIs, handling connection state changes and player disconnections.
    - **Rationale:** GPGS provides Google's native multiplayer infrastructure.
    - **Verification:** Create 4-player match, verify connections. Create turn-based, verify
      progression. Disconnect player, verify handling.

19. **R-14.7.19** — The engine **SHALL** request Play Integrity verdicts on session start and
    periodically. Tokens **SHALL** be forwarded to the game server. Failed checks **SHALL** trigger
    configurable responses. Verdicts **SHALL** be cached for 5 minutes.
    - **Rationale:** Play Integrity verifies device and app authenticity.
    - **Verification:** Request verdict on genuine device, verify pass. Modify APK, verify failure.
      Verify cached reuse within 5 minutes.

20. **R-14.7.20** — The engine **SHALL** persist game state through GPGS Saved Games with metadata.
    Conflict resolution **SHALL** use a merge strategy or choice dialog. The 3 MB data and 800 KB
    image limits **SHALL** be respected.
    - **Rationale:** GPGS Saved Games provides cross-device cloud save.
    - **Verification:** Save state, verify sync to second device. Simulate conflict, verify dialog.
      Exceed 3 MB, verify graceful error.

## Microsoft (Xbox/Windows)

21. **R-14.7.21** — The engine **SHALL** unlock achievements through event-based stat writes via
    `XblEventsWriteInGameEvent`. Xbox Live **SHALL** evaluate rules server-side. Achievement state
    **SHALL** be fetched on session start.
    - **Rationale:** Event-based achievements ensure server-side validation.
    - **Verification:** Write stat event, verify achievement notification. Verify progressive
      achievement updates. Verify state sync on start.

22. **R-14.7.22** — The engine **SHALL** submit stats and query Xbox Live leaderboards supporting
    global, social, and around-player queries.
    - **Rationale:** Xbox leaderboards derive from stats for consistent server-computed rankings.
    - **Verification:** Write stats, verify global ranking. Query social, verify filtering.

23. **R-14.7.23** — The engine **SHALL** process purchases through the Microsoft Store supporting
    consumable add-ons, durables, and subscriptions. Receipts **SHALL** be validated server-side.
    - **Rationale:** Microsoft Store integration is required for Xbox and Windows Store
      distribution.
    - **Verification:** Purchase consumable, verify server validation. Purchase durable, verify
      persistent entitlement. Subscribe, verify lapse detection.

24. **R-14.7.24** — The engine **SHALL** create sessions and find matches through SmartMatch.
    SmartMatch **SHALL** evaluate tickets server-side using QoS, skill ratings, and custom rules.
    - **Rationale:** SmartMatch provides QoS-aware server-side matchmaking.
    - **Verification:** Create match ticket, verify SmartMatch returns matched session. Verify QoS
      influence. Test 1v1 and team types.

25. **R-14.7.25** — The engine **SHALL** detect Game Pass membership and differentiate from
    ownership. Game Pass benefits **SHALL** be configurable. Status **SHALL** be re-verified
    periodically.
    - **Rationale:** Game Pass integration is required for all catalog titles.
    - **Verification:** Verify member gains access. Verify benefits applied. Simulate lapse, verify
      revocation within one verification interval.

26. **R-14.7.26** — The engine **SHALL** persist data to Xbox Connected Storage, syncing across
    consoles and Windows PCs. Storage quotas (256 MB Xbox, 64 MB PC) **SHALL** be respected.
    Conflict resolution **SHALL** use Xbox's built-in sync.
    - **Rationale:** Connected Storage provides Xbox's mandatory cross-device save sync.
    - **Verification:** Save on Xbox, load on PC, verify sync. Exceed quota, verify error. Simulate
      conflict, verify resolution dialog.

27. **R-14.7.27** — The engine **SHALL** integrate voice and text chat through Game Chat 2 with
    push-to-talk, open mic, text-to-speech, and speech-to-text. XR-015 accessibility compliance
    **SHALL** be enforced.
    - **Rationale:** Game Chat 2 handles Xbox certification requirements for communication.
    - **Verification:** Verify push-to-talk and open mic. Verify text-to-speech and speech-to-text.
      Verify mute controls per player.

## PlayStation

28. **R-14.7.28** — The engine **SHALL** unlock trophies through `sceNpTrophyUnlockTrophy`, mapping
    internal IDs to trophy IDs. Trophy packs **SHALL** be embedded per TRC R4060. Platinum **SHALL**
    be awarded automatically when all base-game trophies unlock.
    - **Rationale:** Trophy support is mandatory per TRC R4060. Failure blocks certification.
    - **Verification:** Unlock trophy, verify notification. Unlock all base-game, verify platinum.
      Verify trophy ID mapping.

29. **R-14.7.29** — The engine **SHALL** process IAP through PlayStation Store APIs. Entitlements
    **SHALL** be verified via `sceNpEntitlementAccess`. Server-side receipt validation **SHALL** be
    used. Age rating gates **SHALL** apply per TRC R4082.
    - **Rationale:** PlayStation Store integration is required for all monetized content.
    - **Verification:** Purchase consumable, verify server validation. Verify wallet funding prompt
      on insufficient funds. Verify age gate.

30. **R-14.7.30** — The engine **SHALL** submit and query PSN leaderboard scores, supporting global
    and friends-only queries. Results **SHALL** be cached to respect rate limits.
    - **Rationale:** PSN leaderboards provide platform-native competitive rankings.
    - **Verification:** Submit score, verify global ranking. Query friends-only, verify filtering.

31. **R-14.7.31** — The engine **SHALL** create and join multiplayer sessions through PSN
    matchmaking with skill-based matching, region filtering, and session lifecycle handling.
    - **Rationale:** PSN matchmaking is the standard PlayStation multiplayer infrastructure.
    - **Verification:** Create session, verify join. Send invite, verify receipt. Disconnect host,
      verify migration.

32. **R-14.7.32** — The engine **SHALL** persist save data through `sceNpSaveData` with cloud sync
    for PS Plus subscribers. Save metadata **SHALL** meet TRC R4073. Quotas **SHALL** be respected.
    - **Rationale:** Cloud save is expected. TRC R4073 mandates save metadata.
    - **Verification:** Save, verify cloud sync. Verify save icon and description. Exceed quota,
      verify handling.

33. **R-14.7.33** — The engine **SHALL** publish activity cards on the PS5 home screen through the
    Activities API per TRC R5070. Deep links **SHALL** resolve within 10 seconds.
    - **Rationale:** Activities are mandatory for PS5 per TRC R5070.
    - **Verification:** Publish activity, verify on home screen. Launch from deep link, verify game
      reaches correct state within 10 seconds.

## Nintendo Switch

34. **R-14.7.34** — The engine **SHALL** integrate NSO services. Membership **SHALL** be verified at
    session start before online feature access per Lotcheck. Membership changes **SHALL** be handled
    with UI prompts.
    - **Rationale:** NSO verification is mandatory per Lotcheck.
    - **Verification:** Verify subscriber accesses online features. Verify non-subscriber is
      blocked. Expire mid-session, verify prompt.

35. **R-14.7.35** — The engine **SHALL** process IAP through eShop APIs. Server-side receipt
    validation **SHALL** be used. Lotcheck UX requirements **SHALL** be met.
    - **Rationale:** eShop integration is required for monetized Switch content.
    - **Verification:** Purchase consumable, verify validation. Verify insufficient funds redirect.
      Verify Lotcheck UX compliance.

36. **R-14.7.36** — The engine **SHALL** write save files for automatic NSO cloud backup. Per-title
    size limits **SHALL** be respected. Conflict resolution **SHALL** use the system dialog.
    - **Rationale:** NSO Save Data Cloud provides cross-device save sync.
    - **Verification:** Save, verify cloud backup. Verify size limits. Simulate conflict, verify
      dialog.

37. **R-14.7.37** — The engine **SHALL** create and join sessions through NPLN with configurable
    player count, region preferences, and host migration. NAT traversal **SHALL** use Nintendo's
    relay infrastructure.
    - **Rationale:** NPLN is Nintendo's official online matchmaking for Switch.
    - **Verification:** Create session, verify join via invite. Disconnect host, verify migration.
      Verify NAT traversal behind restricted NAT.

## Epic Online Services (EOS)

38. **R-14.7.38** — The engine **SHALL** unlock achievements and update stats through EOS. Current
    state **SHALL** be fetched on session start.
    - **Rationale:** EOS achievements provide cross-platform tracking.
    - **Verification:** Unlock, verify in EOS overlay. Ingest stat, verify update. Fetch on start,
      verify sync.

39. **R-14.7.39** — The engine **SHALL** submit stats and query EOS leaderboard rankings, supporting
    global and friends-only scopes. Results **SHALL** be cached to respect rate limits.
    - **Rationale:** EOS leaderboards provide cross-platform rankings.
    - **Verification:** Ingest stats, query global, verify ranking. Query friends-only, verify
      filtering. Verify cache.

40. **R-14.7.40** — The engine **SHALL** create and manage multiplayer sessions through EOS Lobbies
    (up to 64 members) and EOS Sessions. Invitations, member join/leave, and ownership transfer
    **SHALL** be handled.
    - **Rationale:** EOS provides cross-platform multiplayer infrastructure.
    - **Verification:** Create lobby, verify join via invite. Verify attribute filtering. Disconnect
      owner, verify transfer.

41. **R-14.7.41** — The engine **SHALL** integrate voice and text chat through EOS RTC with mute,
    volume control, voice activity, text-to-speech, and speech-to-text.
    - **Rationale:** EOS Voice provides cross-platform voice without platform-specific SDKs.
    - **Verification:** Join voice room, verify two-way audio. Mute player, verify silence. Verify
      TTS and STT.

42. **R-14.7.42** — The engine **SHALL** integrate EAC on servers and clients. Action notifications
    (kick, ban) **SHALL** be processed through the engine anti-cheat pipeline.
    - **Rationale:** EAC supplements engine-side validation with kernel-level tamper detection.
    - **Verification:** Connect client with EAC, verify integrity messages. Trigger EAC kick, verify
      disconnect. Verify state in anti-cheat pipeline.

43. **R-14.7.43** — The engine **SHALL** persist per-player data to EOS cloud storage with per-user
    isolation and encryption at rest. File operations **SHALL** be asynchronous. The 200 MB per-file
    and 4 GB per-user limits **SHALL** be respected.
    - **Rationale:** EOS storage provides cross-platform encrypted cloud save.
    - **Verification:** Write and read back, verify integrity. Verify async progress callbacks.
      Exceed 200 MB, verify error.

## Cross-Platform

44. **R-14.7.44** — The engine **SHALL** provide platform-agnostic Rust traits with async methods
    for each service category. Platform backends **SHALL** implement these traits using the
    corresponding SDK. Backend selection **SHALL** be automatic. The abstraction **SHALL** add no
    measurable overhead.
    - **Rationale:** Unified abstraction prevents gameplay from branching per platform and
      centralizes retry and error mapping.
    - **Verification:** Call achievement trait on each backend, verify correct SDK call. Measure
      overhead, verify within 1 % of raw SDK. Verify compile-time backend selection.

45. **R-14.7.45** — The engine **SHALL** synchronize progression across platforms for linked
    accounts. A canonical record **SHALL** be maintained on the game server. Account linking
    **SHALL** map platform identities to a single game account. Cross-buy **SHALL** grant access on
    all linked platforms.
    - **Rationale:** Cross-platform progression retains players who switch platforms.
    - **Verification:** Link Steam and Xbox, unlock on Steam, verify sync to server. Log in on Xbox,
      verify reflected. Verify cross-buy.

46. **R-14.7.46** — The engine **SHALL** detect the active platform at runtime on PC by checking
    launcher environment variables, overlay DLLs, and process parent info. Fallback **SHALL** be
    defined for standalone/DRM-free builds.
    - **Rationale:** Hot-switching enables a single PC binary for multiple storefronts.
    - **Verification:** Launch from Steam, verify Steam backend. Launch from Epic, verify EOS.
      Launch standalone, verify fallback.

## Non-Functional Requirements

47. **R-14.7.47** — Local SDK calls **SHALL** complete within 100 ms (p99). Network-bound SDK calls
    **SHALL** complete within 2 seconds under normal conditions. SDK calls **SHALL NOT** block the
    main game thread.
    - **Rationale:** SDK latency must not impact frame rate or responsiveness.
    - **Verification:** Measure achievement unlock latency across 100 calls, verify p99 under 100
      ms. Measure leaderboard query, verify p99 under 2 seconds. Verify background execution.

48. **R-14.7.48** — When a platform service is unavailable, the engine **SHALL** queue pending
    operations and retry on reconnection. Queued operations **SHALL** persist across session
    restarts. The game **SHALL** remain playable offline with degraded platform features.
    - **Rationale:** Players must not lose progress due to transient outages.
    - **Verification:** Disconnect mid-session, unlock achievements and submit scores. Reconnect,
      verify all queued operations complete. Kill process offline, relaunch, verify queue persists.

49. **R-14.7.49** — All IAP receipts **SHALL** be validated server-side before granting items.
    Duplicate receipt replay **SHALL** be detected within 100 ms. The validation pipeline **SHALL**
    support all platform receipt formats.
    - **Rationale:** Server-side validation prevents fraud and receipt replay attacks.
    - **Verification:** Complete purchase on each platform, verify validation. Replay receipt,
      verify rejection within 100 ms.

50. **R-14.7.50** — The engine **SHALL** meet all mandatory certification requirements for each
    target platform (TRC for PlayStation, XR for Xbox, Lotcheck for Nintendo, App Review for Apple,
    Play Policy for Google).
    - **Rationale:** Certification failures block store submission. Engine-level compliance
      eliminates per-title certification work.
    - **Verification:** Run each platform's automated certification test suite and verify all
      mandatory checks pass.

## Server-Side Proprietary SDK Isolation (R-14.8)

51. **R-14.8.1** — The engine client **SHALL** contain zero proprietary SDK code, headers, or
    libraries. The open-source binary **SHALL** compile without any console SDK installed.
    - **Rationale:** Enables fully open-source engine. Developers contribute without NDA
      dependencies.
    - **Verification:** Build from source on a clean machine without console SDKs. Scan source for
      proprietary headers, find none.

52. **R-14.8.2** — Console builds **SHALL** be triggered via an authenticated REST API accepting
    project ID, target platform, build profile, and source revision, returning a job ID for status
    polling.
    - **Rationale:** REST API decouples client from server-side build logic. Authentication prevents
      unauthorized SDK access.
    - **Verification:** Submit build with valid credentials, verify job ID returned. Submit with
      invalid credentials, verify HTTP 401.

53. **R-14.8.3** — All proprietary SDK files **SHALL** reside exclusively on the build server in
    access-controlled directories. SDK files **SHALL NOT** be downloadable via any API endpoint.
    - **Rationale:** Proprietary SDKs are NDA-protected. Server-only storage prevents accidental
      distribution.
    - **Verification:** Attempt SDK file access via all API endpoints, verify HTTP 403. Verify
      restrictive filesystem permissions.

54. **R-14.8.4** — Individual developers **SHALL NOT** require console SDK licenses. Only the build
    server operator **SHALL** hold licenses.
    - **Rationale:** Reduces licensing cost from per-developer to per-server. Enables open-source
      contribution without manufacturer approval.
    - **Verification:** Set up workstation without console SDKs. Verify all features except console
      packaging are functional.

55. **R-14.8.5** — The build server **SHALL** support concurrent builds from multiple projects with
    per-project isolation. One project's files **SHALL NOT** be accessible to another.
    - **Rationale:** Studios sharing a server must not risk source code leakage.
    - **Verification:** Submit builds from two projects concurrently. Verify isolated directories.
      Verify project A cannot read project B files.

56. **R-14.8.6** — The build server **SHALL** provide a priority-based job queue with configurable
    priority per project and FIFO within the same level. Queue metrics **SHALL** be exposed via
    monitoring API.
    - **Rationale:** Priority scheduling ensures urgent builds are processed first. Monitoring
      enables capacity planning.
    - **Verification:** Submit low and high priority simultaneously. Verify high dequeued first.
      Query monitoring API, verify accurate depth.

57. **R-14.8.7** — The build server **SHALL** deploy signed console packages to network-accessible
    dev kits using manufacturer deployment tools.
    - **Rationale:** Developers test on hardware without local proprietary tools.
    - **Verification:** Trigger deploy from editor to dev kit via build server. Verify package
      installs and launches.

58. **R-14.8.8** — Console output from dev kits **SHALL** stream to the editor through the build
    server relay with latency under 500 ms. Multiple concurrent sessions **SHALL** be supported.
    - **Rationale:** Real-time output is essential for debugging. The relay avoids requiring
      proprietary tools locally.
    - **Verification:** Launch game on dev kit. Verify output in editor within 500 ms. Open two
      sessions, verify independent streaming.

59. **R-14.8.9** — The build server **SHALL** produce code-signed console packages and store them in
    S3-compatible object storage keyed by content hash. Build manifests **SHALL** record source
    revision, engine version, SDK version, and timestamp.
    - **Rationale:** Content-hash keys enable deduplication. Manifests provide traceability.
    - **Verification:** Build same source twice, verify identical hash. Retrieve manifest, verify
      all fields populated.

60. **R-14.8.10** — Console artifacts **SHALL** be downloadable by authenticated users. Retention
    **SHALL** automatically delete builds older than a configurable threshold (default 90 days).
    Pinned certification builds **SHALL** be exempt.
    - **Rationale:** Automated retention prevents unbounded storage. Pins preserve submission
      builds.
    - **Verification:** Download artifact, verify integrity. Set retention to 1 day, verify expired
      unpinned builds deleted. Pin a build, verify it survives retention.
