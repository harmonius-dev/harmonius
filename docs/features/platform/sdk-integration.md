# 14.7 — Platform SDK Integration

## Steam (Steamworks)

| ID       | Feature                         | Requirements |
|----------|---------------------------------|--------------|
| F-14.7.1 | Steam Achievements and Stats    | R-14.7.1     |
| F-14.7.2 | Steam Leaderboards              | R-14.7.2     |
| F-14.7.3 | Steam In-App Purchases          | R-14.7.3     |
| F-14.7.4 | Steam Workshop                  | R-14.7.4     |
| F-14.7.5 | Steam Matchmaking               | R-14.7.5     |
| F-14.7.6 | Steam Anti-Cheat (VAC)          | R-14.7.6     |
| F-14.7.7 | Steam Friends and Rich Presence | R-14.7.7     |
| F-14.7.8 | Steam Cloud Save                | R-14.7.8     |
| F-14.7.9 | Steam Voice Chat                | R-14.7.9     |

1. **F-14.7.1** — Unlock Steam achievements and update player stats through the Steamworks
   `ISteamUserStats` API. The engine maps internal achievement IDs to Steam-specific string
   identifiers and calls `SetAchievement` followed by `StoreStats`. Stat updates (integers, floats,
   averages) are submitted through the same interface. Initial sync on session start fetches current
   unlock and stat state to prevent redundant calls. Deferred unlocks queue when the Steam overlay
   is unavailable and retry on reconnection.
   - **Deps:** F-14.7.44 (Platform Abstraction Layer)
   - **Platform:** Requires Steamworks SDK. Achievement images are configured in the Steamworks
     partner portal, not in-engine. `StoreStats` batches all pending changes in a single call.
2. **F-14.7.2** — Submit and query ranked scores through Steam leaderboards via
   `ISteamUserStats::FindOrCreateLeaderboard` and `UploadLeaderboardScore`. Supports global,
   friends-only, and around-user queries with configurable score type (numeric, time-based).
   Leaderboard entries include attached UGC data (replay metadata, build info). Query results are
   cached locally to avoid repeated API calls within the Steam rate window.
   - **Deps:** F-14.7.44 (Platform Abstraction Layer)
   - **Platform:** Steam limits leaderboard queries; the engine caches results for 60 seconds.
     Leaderboard creation requires the app to own the leaderboard name in the Steamworks portal.
3. **F-14.7.3** — Process microtransactions through `ISteamMicroTxn` for purchasing virtual
   currency, cosmetic items, and DLC content. The engine initiates transactions via `InitTxn`,
   monitors status through Steam overlay callbacks, and confirms completion with `FinalizeTxn`. All
   receipts are forwarded to the game server for server-side validation. Pending and failed
   transactions are tracked and retried.
   - **Deps:** F-14.7.44 (Platform Abstraction Layer), F-13.23.3a (Platform Purchase Abstraction)
   - **Platform:** Steam overlay must be active for purchase UI. `ISteamMicroTxn` returns results
     asynchronously via callbacks. Currency conversion is handled by Steam.
4. **F-14.7.4** — Upload, download, subscribe, and manage user-generated content through the Steam
   Workshop API (`ISteamUGC`). Mod authors publish content from the editor using `CreateItem` and
   `SubmitItemUpdate` with metadata (title, description, tags, preview images). Players browse,
   subscribe, rate, and report mods through the in-game workshop browser or Steam client. Subscribed
   mods auto-download on launch via `DownloadItem`. Dependency resolution handles mods that require
   other mods.
   - **Deps:** F-14.7.44 (Platform Abstraction Layer), F-15.16.5 (Mod Workshop)
   - **Platform:** Workshop items have a 10 GB size limit per item. Visibility states: public,
     friends-only, private, unlisted. Steam handles CDN distribution.
5. **F-14.7.5** — Create and join lobbies, browse dedicated servers, and manage matchmaking through
   `ISteamMatchmaking` and `ISteamMatchmakingServers`. Lobby creation supports public, friends-only,
   and invite-only visibility with custom metadata filters. Server browser queries filter by game
   mode, map, ping, player count, and custom tags. Favorite and history server lists persist through
   `AddFavoriteGame`. Lobby chat relays messages to lobby members.
   - **Deps:** F-14.7.44 (Platform Abstraction Layer)
   - **Platform:** Steam lobbies are limited to 250 members. Server browser uses the master server
     query interface. Ping estimation uses Steam Datagram Relay when available.
6. **F-14.7.6** — Integrate Valve Anti-Cheat through the Steamworks game server API. The engine
   registers with VAC on dedicated server startup via `GSInitGameServer` and enables VAC
   enforcement. Client sessions are authenticated through Steam tickets that carry VAC status.
   VAC-banned players are rejected at connection time. The engine exposes VAC session state to the
   anti-cheat system (F-8.8.1) for combined enforcement.
   - **Deps:** F-14.7.44 (Platform Abstraction Layer), F-8.8.1 (Server-Side Cheat Detection)
   - **Platform:** VAC integration requires the game server to register as a secure server. VAC bans
     are managed entirely by Valve with no developer API for ban management.
7. **F-14.7.7** — Access the player's Steam friend list through `ISteamFriends` and display
   contextual game status via `SetRichPresence`. Friend data includes persona name, avatar, online
   status, and current game. Rich presence strings use localization tokens configured in the
   Steamworks portal. Game invites are sent through `InviteUserToGame` and received via callback.
   The engine throttles rich presence updates to one per 15 seconds.
   - **Deps:** F-14.7.44 (Platform Abstraction Layer), F-14.5.3 (Rich Presence)
   - **Platform:** Rich presence tokens are defined in the Steamworks partner site. Steam limits
     rich presence string length to 256 bytes UTF-8.
8. **F-14.7.8** — Persist player data to Steam Cloud through `ISteamRemoteStorage` or Steam
   Auto-Cloud. Settings, keybindings, and game configuration files sync across machines
   automatically. The engine writes files to the Auto-Cloud monitored directory and Steam handles
   upload/download. Conflict resolution uses Steam's built-in resolution dialog. Per-user cloud
   quota (configurable in Steamworks, typically 1 GB) is respected.
   - **Deps:** F-14.7.44 (Platform Abstraction Layer), F-14.5.5 (Platform Cloud Storage)
   - **Platform:** Auto-Cloud is preferred over manual `ISteamRemoteStorage` calls. File paths are
     configured in the Steamworks partner portal. Maximum file count and total size limits apply per
     app.
9. **F-14.7.9** — Capture and transmit voice audio through the Steam Voice API
   (`ISteamUser::StartVoiceRecording`, `GetAvailableVoice`, `GetVoice`, `DecompressVoice`). Voice
   data is transmitted through Steam Networking or the engine's networking layer. Supports
   push-to-talk and voice activity detection modes. Audio quality is configurable (sample rate,
   bitrate). Integrates with the engine's audio mixer for spatial voice in-game.
   - **Deps:** F-14.7.44 (Platform Abstraction Layer), F-14.5.4 (Platform Voice and Party
     Integration)
   - **Platform:** Steam Voice uses Opus codec internally. Voice data packets are typically 20ms
     frames. Steam Networking transport is optional; the engine can use its own transport layer.

## Apple (iOS/macOS)

| ID        | Feature                                 | Requirements |
|-----------|-----------------------------------------|--------------|
| F-14.7.10 | StoreKit 2 In-App Purchases             | R-14.7.10    |
| F-14.7.11 | Game Center Achievements                | R-14.7.11    |
| F-14.7.12 | Game Center Leaderboards and Challenges | R-14.7.12    |
| F-14.7.13 | Game Center Matchmaking                 | R-14.7.13    |
| F-14.7.14 | iCloud Key-Value and CloudKit Save      | R-14.7.14    |
| F-14.7.15 | App Tracking Transparency Compliance    | R-14.7.15    |

1. **F-14.7.10** — Process in-app purchases on iOS and macOS through StoreKit 2. Supports consumable
   items (virtual currency), non-consumable items (permanent unlocks), and auto-renewable
   subscriptions. The engine fetches products via `Product.products(for:)`, initiates purchases with
   `Product.purchase()`, and verifies transactions through `Transaction.currentEntitlements`. All
   receipts are forwarded to the game server for server-side validation using the App Store Server
   API. Handles interrupted purchases, pending transactions, and family sharing entitlements.
   - **Deps:** F-14.7.44 (Platform Abstraction Layer), F-13.23.3a (Platform Purchase Abstraction)
   - **Platform:** StoreKit 2 requires iOS 15+ / macOS 12+. JWS-signed transaction data replaces
     legacy receipt bundles. Sandbox testing uses the StoreKit Testing environment.
2. **F-14.7.11** — Unlock achievements and report progress through Game Center on iOS and macOS via
   `GKAchievement.report()`. The engine maps internal achievement IDs to Game Center identifiers
   configured in App Store Connect. Progress is reported as a percentage (0-100) for incremental
   achievements. Completed achievements trigger the Game Center banner. Achievement state is fetched
   on session start via `GKAchievement.loadAchievements()` to sync local state.
   - **Deps:** F-14.7.44 (Platform Abstraction Layer)
   - **Platform:** Game Center achievements are configured in App Store Connect. Hidden achievements
     are supported. Game Center authentication is required before reporting.
3. **F-14.7.12** — Submit scores and query leaderboards through Game Center via `GKLeaderboard` and
   `GKLeaderboardEntry`. Supports classic and recurring leaderboards with global, friends, and
   around-player scopes. Players can issue challenges to friends from leaderboard entries. The
   engine submits scores via `GKLeaderboard.submitScore()` and fetches entries with configurable
   time scope (today, week, all time).
   - **Deps:** F-14.7.44 (Platform Abstraction Layer)
   - **Platform:** Recurring leaderboards reset on a configured schedule. Game Center rate limits
     leaderboard submissions; the engine batches updates.
4. **F-14.7.13** — Create and join multiplayer matches through Game Center using `GKMatchmaker`.
   Supports real-time matches, turn-based matches, and auto-match with skill-based matchmaking. The
   engine uses `GKMatchRequest` to configure player count, player group, and player attributes.
   Match invites are sent through Game Center and received via `GKMatchmakerViewControllerDelegate`.
   Voice chat within matches uses `GKVoiceChat`.
   - **Deps:** F-14.7.44 (Platform Abstraction Layer)
   - **Platform:** Game Center matchmaking supports 2-16 players for real-time and 2-12 for
     turn-based. Auto-match uses Apple's skill rating algorithm.
5. **F-14.7.14** — Sync lightweight player data through iCloud Key-Value Storage
   (`NSUbiquitousKeyValueStore`) and structured game saves through CloudKit. Key-value storage holds
   preferences, settings, and small state (1 MB limit). CloudKit stores larger save data with
   per-record change tracking. Conflict resolution uses last-write-wins for key-value and CloudKit's
   server change token for structured data. The engine monitors for external changes via
   `NSUbiquitousKeyValueStoreDidChangeExternallyNotification`.
   - **Deps:** F-14.7.44 (Platform Abstraction Layer), F-14.5.5 (Platform Cloud Storage)
   - **Platform:** iCloud Key-Value is limited to 1 MB total and 1024 keys. CloudKit private
     database has 25 MB asset limit per record. Requires iCloud entitlement in the app's
     provisioning profile.
6. **F-14.7.15** — Enforce Apple's App Tracking Transparency (ATT) framework on iOS 14.5+. The
   engine requests tracking authorization via `ATTrackingManager.requestTrackingAuthorization()`
   before any IDFA access or cross-app tracking. If the user denies tracking, the engine disables
   all behavioral advertising, analytics SDKs that use IDFA, and cross-app attribution. Tracking
   status is cached and re-checked on each app launch. The ATT prompt is displayed at a contextually
   appropriate moment (not on first launch).
   - **Deps:** F-14.7.44 (Platform Abstraction Layer)
   - **Platform:** ATT is mandatory for App Store submission on iOS 14.5+. Apps that access IDFA
     without the ATT prompt are rejected during App Review. SKAdNetwork is used for
     privacy-preserving attribution when tracking is denied.

## Google Play

| ID        | Feature                                           | Requirements |
|-----------|---------------------------------------------------|--------------|
| F-14.7.16 | Play Billing Library 7                            | R-14.7.16    |
| F-14.7.17 | Play Games Services Achievements and Leaderboards | R-14.7.17    |
| F-14.7.18 | Play Games Services Matchmaking                   | R-14.7.18    |
| F-14.7.19 | Google Play Integrity API                         | R-14.7.19    |
| F-14.7.20 | Google Play Cloud Save                            | R-14.7.20    |

1. **F-14.7.16** — Process in-app purchases and subscriptions on Android through Google Play Billing
   Library 7. Supports one-time products (consumable, non-consumable) and subscriptions
   (auto-renewing, prepaid). The engine connects via `BillingClient`, queries products with
   `queryProductDetailsAsync`, launches purchase flow with `launchBillingFlow`, and acknowledges
   purchases with `acknowledgePurchase` (non-consumable) or `consumeAsync` (consumable). All
   purchase tokens are forwarded to the game server for validation via the Google Play Developer
   API.
   - **Deps:** F-14.7.44 (Platform Abstraction Layer), F-13.23.3a (Platform Purchase Abstraction)
   - **Platform:** Play Billing Library 7 requires targetSdkVersion 34+. Unacknowledged purchases
     are refunded after 3 days. Subscription base plans support multiple offers (introductory,
     upgrade).
2. **F-14.7.17** — Unlock achievements and submit leaderboard scores through Google Play Games
   Services (GPGS). Achievements support standard (unlock once), incremental (step-based progress),
   and hidden types. The engine calls `achieve()` for standard achievements and `increment()` for
   incremental. Leaderboard scores are submitted via `submitScore()` with support for daily, weekly,
   and all-time time spans. GPGS sign-in uses `GamesSignInClient.signIn()`.
   - **Deps:** F-14.7.44 (Platform Abstraction Layer)
   - **Platform:** GPGS requires Google Play Games Services SDK v2. Achievements and leaderboards
     are configured in the Google Play Console. GPGS supports cross-platform play with PC via Play
     Games for PC.
3. **F-14.7.18** — Create and join multiplayer matches through GPGS real-time and turn-based
   multiplayer APIs. The engine uses `RealTimeMultiplayerClient` for real-time matches and
   `TurnBasedMultiplayerClient` for turn-based. Match configuration includes player count,
   auto-match criteria, and invitation handling. The engine handles connection state changes, player
   disconnections, and room lifecycle events.
   - **Deps:** F-14.7.44 (Platform Abstraction Layer)
   - **Platform:** GPGS matchmaking supports 2-8 players for real-time matches. Turn-based supports
     up to 8 players with configurable turn timeout.
4. **F-14.7.19** — Verify app and device integrity on Android through the Play Integrity API. The
   engine requests integrity verdicts via `IntegrityManager.requestIntegrityToken()` on session
   start and periodically during play. Verdicts include device integrity, app integrity, and license
   status. Integrity tokens are forwarded to the game server for decryption and validation. Failed
   integrity checks trigger configurable responses (warn, restrict features, disconnect).
   - **Deps:** F-14.7.44 (Platform Abstraction Layer), F-8.8.2 (Client Integrity Verification)
   - **Platform:** Play Integrity replaces SafetyNet Attestation. Classic API requests are limited
     to 10,000/day; standard API is unlimited but requires Play Store distribution. Integrity
     verdicts are cached for 5 minutes.
5. **F-14.7.20** — Persist player game state through Google Play Games Services Saved Games API. The
   engine saves snapshots via `SnapshotsClient.open()` and `writeSnapshot()` with metadata
   (description, play time, progress image). Conflict resolution uses the engine's merge strategy or
   presents a choice dialog. Saved games sync across devices through the player's Google account.
   - **Deps:** F-14.7.44 (Platform Abstraction Layer), F-14.5.5 (Platform Cloud Storage)
   - **Platform:** Saved Games requires GPGS sign-in. Maximum snapshot size is 3 MB data + 800 KB
     cover image. Total quota is 200 saved games per player per app.

## Microsoft (Xbox/Windows)

| ID        | Feature                               | Requirements |
|-----------|---------------------------------------|--------------|
| F-14.7.21 | Xbox Live Achievements (Event-Based)  | R-14.7.21    |
| F-14.7.22 | Xbox Live Leaderboards (Stats-Based)  | R-14.7.22    |
| F-14.7.23 | Microsoft Store IAP and Subscriptions | R-14.7.23    |
| F-14.7.24 | Xbox Live Matchmaking (SmartMatch)    | R-14.7.24    |
| F-14.7.25 | Xbox Game Pass Integration            | R-14.7.25    |
| F-14.7.26 | Xbox Connected Storage (Cloud Save)   | R-14.7.26    |
| F-14.7.27 | Xbox Game Chat 2                      | R-14.7.27    |

1. **F-14.7.21** — Unlock achievements on Xbox and Windows through the Xbox Live event-based
   achievement system. The engine writes stat events via `XblEventsWriteInGameEvent` and Xbox Live
   services evaluate achievement rules server-side. Achievement definitions are configured in
   Partner Center. The engine fetches current achievement state on session start. Supports standard,
   progressive, and challenge achievement types.
   - **Deps:** F-14.7.44 (Platform Abstraction Layer)
   - **Platform:** Xbox achievements are event-based; the client writes events and Xbox Live
     evaluates rules server-side. GamerScore values are configured in Partner Center (1000G base
     game, 1000G DLC max).
2. **F-14.7.22** — Submit and query leaderboard rankings through Xbox Live stats-based leaderboards.
   The engine writes player stats via `XblEventsWriteInGameEvent` and queries leaderboard rankings
   through `XblLeaderboardGetLeaderboardAsync`. Supports global, social (friends), and around-player
   queries. Stats are configured in Partner Center with aggregation rules.
   - **Deps:** F-14.7.44 (Platform Abstraction Layer)
   - **Platform:** Xbox leaderboards are derived from stats; no separate leaderboard submission API.
     Stats are configured in Partner Center with rate limiting rules.
3. **F-14.7.23** — Process purchases through the Microsoft Store using
   `XStoreQueryAddOnLicensesAsync` for entitlement checks and `XStoreShowPurchaseUIAsync` for
   purchase flow. Supports consumable add-ons, durable add-ons, and subscriptions. Purchase receipts
   are validated server-side through the Microsoft Store collections API.
   - **Deps:** F-14.7.44 (Platform Abstraction Layer), F-13.23.3a (Platform Purchase Abstraction)
   - **Platform:** Microsoft Store add-ons are configured in Partner Center. Xbox and PC share the
     same store backend. Cross-device entitlements are automatic for Microsoft account holders.
4. **F-14.7.24** — Create multiplayer sessions and find matches through Xbox Live SmartMatch. The
   engine creates match tickets via `XblMatchmakingCreateMatchTicketAsync` with a hopper name,
   session reference, and player attributes. SmartMatch evaluates tickets server-side using QoS
   measurements, skill ratings, and custom rules. Supports 1v1, team, and large-session match types.
   - **Deps:** F-14.7.44 (Platform Abstraction Layer)
   - **Platform:** SmartMatch hoppers are configured in Partner Center. Match evaluation runs
     server-side with no client-side filtering. QoS measurements are collected automatically by the
     Xbox networking stack.
5. **F-14.7.25** — Detect and respond to Xbox Game Pass membership status. The engine queries Game
   Pass entitlements via `XStoreQueryEntitledProductsAsync` to determine if the player has access
   through Game Pass versus purchase. Game Pass members receive configurable benefits. The engine
   differentiates Game Pass access from ownership for analytics and content gating.
   - **Deps:** F-14.7.44 (Platform Abstraction Layer), F-14.5.6 (Entitlements)
   - **Platform:** Game Pass titles must meet Xbox Requirements (XR) for Game Pass-specific
     scenarios. Game Pass members who purchase the game retain access if their subscription lapses.
6. **F-14.7.26** — Persist player game data to Xbox Connected Storage via
   `XGameSaveInitializeProviderAsync`. The engine creates save containers with
   `XGameSaveCreateContainer`, writes blobs with `XGameSaveSubmitBlobWrite`, and reads with
   `XGameSaveReadBlobData`. Connected Storage syncs across Xbox consoles and Windows PCs. Conflict
   resolution uses Xbox's built-in sync resolution.
   - **Deps:** F-14.7.44 (Platform Abstraction Layer), F-14.5.5 (Platform Cloud Storage)
   - **Platform:** Connected Storage requires Xbox Live sign-in. Write operations are batched into
     containers. The per-title quota is 256 MB on Xbox and 64 MB on PC by default.
7. **F-14.7.27** — Integrate voice and text chat through Xbox Game Chat 2. The engine initializes
   the chat manager, adds players to the chat network, and processes audio frames. Supports
   push-to-talk, open mic with noise suppression, text-to-speech, and speech-to-text for
   accessibility. Mute, volume control, and voice activity indicators are surfaced to the game UI.
   - **Deps:** F-14.7.44 (Platform Abstraction Layer), F-14.5.4 (Platform Voice and Party
     Integration)
   - **Platform:** Game Chat 2 handles encoding, transport, and platform compliance (parental
     controls, mute lists). Text-to-speech and speech-to-text are mandatory for Xbox certification
     (XR-015).

## PlayStation

| ID        | Feature                    | Requirements |
|-----------|----------------------------|--------------|
| F-14.7.28 | PSN Trophies               | R-14.7.28    |
| F-14.7.29 | PlayStation Store IAP      | R-14.7.29    |
| F-14.7.30 | PSN Leaderboards           | R-14.7.30    |
| F-14.7.31 | PSN Matchmaking            | R-14.7.31    |
| F-14.7.32 | PSN Cloud Save (Save Data) | R-14.7.32    |
| F-14.7.33 | PlayStation Activities     | R-14.7.33    |

1. **F-14.7.28** — Unlock trophies on PlayStation through the PSN trophy API
   (`sceNpTrophyUnlockTrophy`). The engine maps internal achievement IDs to trophy IDs configured in
   the trophy pack. Trophy packs are embedded in the game package. The engine registers the trophy
   context on startup and fetches current unlock state. Platinum trophy is awarded automatically
   when all base-game trophies are unlocked.
   - **Deps:** F-14.7.44 (Platform Abstraction Layer)
   - **Platform:** Trophy packs are required for all PlayStation titles (TRC R4060). DLC trophies
     are added via separate trophy groups.
2. **F-14.7.29** — Process in-app purchases on PlayStation through the PlayStation Store commerce
   APIs. The engine displays the store overlay for product selection and purchase flow. Entitlements
   are verified through `sceNpEntitlementAccess`. Server-side receipt validation uses the
   PlayStation Store server API. The engine handles wallet funding prompts when the player's wallet
   balance is insufficient.
   - **Deps:** F-14.7.44 (Platform Abstraction Layer), F-13.23.3a (Platform Purchase Abstraction)
   - **Platform:** PlayStation Store products are configured in the PlayStation Partners portal.
     Pricing is set per region. Age rating gates apply per TRC R4082.
3. **F-14.7.30** — Submit and query ranked scores through PSN leaderboard APIs. Supports global and
   friends-only queries. Score submissions include optional game-specific data attachments. The
   engine caches query results to respect PSN rate limits.
   - **Deps:** F-14.7.44 (Platform Abstraction Layer)
   - **Platform:** PSN leaderboards enforce rate limits on queries. Score attachment data is limited
     in size per the platform specification.
4. **F-14.7.31** — Create and join multiplayer sessions through PSN matchmaking. The engine uses PSN
   session management APIs with skill-based matchmaking, region filtering, and custom attribute
   matching. Session invites are sent through the PSN social overlay. The engine handles session
   lifecycle events.
   - **Deps:** F-14.7.44 (Platform Abstraction Layer)
   - **Platform:** PSN matchmaking configuration is set in the PlayStation Partners portal. Session
     size limits vary per title configuration.
5. **F-14.7.32** — Persist player save data to PSN cloud storage through `sceNpSaveData`. Cloud sync
   is automatic for PlayStation Plus subscribers. Conflict resolution presents the standard
   PlayStation save data conflict dialog. The engine respects per-title save data quotas.
   - **Deps:** F-14.7.44 (Platform Abstraction Layer), F-14.5.5 (Platform Cloud Storage)
   - **Platform:** PSN cloud save requires PlayStation Plus membership. TRC R4073 mandates save data
     icon and description metadata.
6. **F-14.7.33** — Display game activities on the PlayStation home screen through the Activities
   API. The engine publishes activity cards for active quests, multiplayer sessions, and game
   challenges. Activity cards show progress, estimated time to complete, and a deep link to resume
   directly.
   - **Deps:** F-14.7.44 (Platform Abstraction Layer)
   - **Platform:** Activities are required for PS5 titles per TRC R5070. Deep links must resolve
     within 10 seconds of launch.

## Nintendo Switch

| ID        | Feature                            | Requirements |
|-----------|------------------------------------|--------------|
| F-14.7.34 | Nintendo Switch Online Integration | R-14.7.34    |
| F-14.7.35 | Nintendo eShop IAP                 | R-14.7.35    |
| F-14.7.36 | NSO Save Data Cloud                | R-14.7.36    |
| F-14.7.37 | NSO Matchmaking (NEX/NPLN)         | R-14.7.37    |

1. **F-14.7.34** — Integrate Nintendo Switch Online (NSO) services for online multiplayer, voice
   chat (via the NSO app), and cloud backup. The engine authenticates with NSO through the Nintendo
   Account system. Online play requires an active NSO membership, verified at session start.
   - **Deps:** F-14.7.44 (Platform Abstraction Layer)
   - **Platform:** NSO membership check is required before any online feature access per Lotcheck
     guidelines. Voice chat uses the Nintendo Switch Online mobile app.
2. **F-14.7.35** — Process in-app purchases on Nintendo Switch through the Nintendo eShop APIs. The
   engine initiates purchase requests for consumable and non-consumable items, handles the eShop
   overlay purchase flow, and verifies entitlements after purchase. Server-side receipt validation
   uses the Nintendo eShop server API.
   - **Deps:** F-14.7.44 (Platform Abstraction Layer), F-13.23.3a (Platform Purchase Abstraction)
   - **Platform:** eShop product configuration is managed through Nintendo Developer Portal.
     Lotcheck requires specific purchase flow UX compliance.
3. **F-14.7.36** — Persist player save data to Nintendo Switch Online cloud backup. The engine uses
   the Switch save data API to write save files to the system save data area. Cloud backup is
   handled automatically by the system for NSO subscribers.
   - **Deps:** F-14.7.44 (Platform Abstraction Layer), F-14.5.5 (Platform Cloud Storage)
   - **Platform:** NSO Save Data Cloud requires NSO membership. Save data size limits are defined
     per title in the Nintendo Developer Portal.
4. **F-14.7.37** — Create and join multiplayer sessions through Nintendo's online matchmaking
   services (NPLN). The engine creates matchmaking sessions with configurable player count, region
   preferences, and custom attributes. Session invites are sent through the Switch friend system.
   The engine handles host migration and NAT traversal through Nintendo's relay infrastructure.
   - **Deps:** F-14.7.44 (Platform Abstraction Layer)
   - **Platform:** NPLN is Nintendo's current online infrastructure replacing NEX. NAT traversal is
     handled by Nintendo's relay servers.

## Epic Online Services (EOS)

| ID        | Feature                     | Requirements |
|-----------|-----------------------------|--------------|
| F-14.7.38 | EOS Achievements            | R-14.7.38    |
| F-14.7.39 | EOS Leaderboards            | R-14.7.39    |
| F-14.7.40 | EOS Matchmaking and Lobbies | R-14.7.40    |
| F-14.7.41 | EOS Voice and Chat          | R-14.7.41    |
| F-14.7.42 | EOS Anti-Cheat (EAC)        | R-14.7.42    |
| F-14.7.43 | EOS Player Data Storage     | R-14.7.43    |

1. **F-14.7.38** — Unlock achievements and track stats through Epic Online Services (EOS). The
   engine maps internal achievement IDs to EOS achievement definition IDs and unlocks via
   `EOS_Achievements_UnlockAchievements`. Stats are updated through `EOS_Stats_IngestStat`. Current
   state is fetched on session start.
   - **Deps:** F-14.7.44 (Platform Abstraction Layer)
   - **Platform:** EOS achievements work across all platforms through the EOS overlay or headless
     mode. Achievement definitions are managed in the Epic Developer Portal.
2. **F-14.7.39** — Submit and query ranked scores through EOS Leaderboards. The engine ingests stats
   and queries leaderboard rankings. Supports global and friends-only scopes. Leaderboard
   definitions are configured in the Epic Developer Portal. Query results are cached to respect API
   rate limits.
   - **Deps:** F-14.7.44 (Platform Abstraction Layer)
   - **Platform:** EOS leaderboards are stat-driven; rankings are computed server-side from ingested
     stats. Rate limits apply to query operations.
3. **F-14.7.40** — Create, join, and manage multiplayer sessions through EOS Lobbies and EOS
   Sessions interfaces. The engine creates lobbies with configurable member limit, permissions, and
   custom attributes. Invitations are sent through `EOS_Lobby_SendInvite`. The engine handles member
   join/leave, attribute updates, and lobby ownership transfer.
   - **Deps:** F-14.7.44 (Platform Abstraction Layer)
   - **Platform:** EOS Lobbies support up to 64 members. EOS Sessions support dedicated server
     registration. Matchmaking attributes allow custom filtering logic.
4. **F-14.7.41** — Integrate voice and text chat through EOS Voice and Messaging interfaces. The
   engine joins voice rooms via EOS RTC layer. Supports mute, volume control, voice activity
   indicators, and text-to-speech/speech-to-text for accessibility.
   - **Deps:** F-14.7.44 (Platform Abstraction Layer), F-14.5.4 (Platform Voice and Party
     Integration)
   - **Platform:** EOS Voice uses the EOS RTC subsystem. Voice rooms are associated with lobbies or
     sessions.
5. **F-14.7.42** — Integrate Easy Anti-Cheat through the EOS Anti-Cheat interfaces. The server
   registers with EAC and processes client integrity messages. Clients register and send integrity
   data. Action notifications from EAC (kick, ban) are processed through the engine's anti-cheat
   severity pipeline.
   - **Deps:** F-14.7.44 (Platform Abstraction Layer), F-8.8.1 (Server-Side Cheat Detection)
   - **Platform:** EAC supports Windows, macOS, and Linux. The EAC client-side module runs as a
     separate process.
6. **F-14.7.43** — Persist per-player data to EOS cloud storage. Supports encrypted file storage
   with per-user isolation. File operations are asynchronous with progress callbacks for large
   files. Conflict resolution uses last-write-wins with EOS-managed versioning.
   - **Deps:** F-14.7.44 (Platform Abstraction Layer), F-14.5.5 (Platform Cloud Storage)
   - **Platform:** EOS Player Data Storage has a per-file limit of 200 MB and per-user limit of 4
     GB. Files are encrypted at rest.

## Cross-Platform

| ID        | Feature                    | Requirements |
|-----------|----------------------------|--------------|
| F-14.7.44 | Platform Abstraction Layer | R-14.7.44    |
| F-14.7.45 | Cross-Platform Progression | R-14.7.45    |
| F-14.7.46 | Platform SDK Hot-Switching | R-14.7.46    |

1. **F-14.7.44** — Unified API that abstracts all platform SDK functionality behind
   platform-agnostic interfaces. Each service category is defined as a Rust trait with async
   methods. Platform-specific backends implement these traits using the corresponding SDK. Backend
   selection is automatic based on the target platform at compile time and the detected runtime
   environment. The abstraction layer handles SDK initialization, shutdown, error mapping, and retry
   logic uniformly.
   - **Deps:** F-14.5.1 (Cross-Platform Achievement System), F-14.5.2 (Leaderboards), F-14.5.5
     (Platform Cloud Storage)
   - **Platform:** Each platform backend is a separate compilation unit behind feature flags. Only
     the active platform's SDK is linked. The abstraction layer adds no measurable overhead.
2. **F-14.7.45** — Synchronize player progression across platforms for players with linked accounts.
   The engine maintains a canonical progression record on the game server, merging platform-specific
   achievement state, purchase entitlements, and save data. Account linking maps platform identities
   to a single game account. Cross-buy entitlements grant access on all linked platforms.
   - **Deps:** F-14.7.44 (Platform Abstraction Layer), F-14.5.5 (Platform Cloud Storage), F-14.5.6
     (Entitlements)
   - **Platform:** Cross-platform progression requires platform holder approval. Some platforms
     restrict cross-buy. Account linking must comply with each platform's identity policies.
3. **F-14.7.46** — Detect the active platform at runtime and dynamically load the correct SDK
   backend. On PC, the engine detects the active storefront by checking launcher-injected
   environment variables, overlay DLLs, and process parent information. The correct backend is
   activated without requiring separate builds per storefront.
   - **Deps:** F-14.7.44 (Platform Abstraction Layer)
   - **Platform:** Console platforms do not require hot-switching. PC is the primary use case for
     runtime SDK detection.

## Server-Side Proprietary SDK Isolation (F-14.8)

The engine is 100% open source for every developer. All proprietary console SDK integrations
(PlayStation SDK, Xbox GDK, Nintendo SDK) run exclusively on a shared build/deploy server.
Individual developers never need console SDK licenses.

| ID       | Feature                           | Requirements        |
|----------|-----------------------------------|---------------------|
| F-14.8.1 | Server-Side Console Build Service | R-14.8.1, R-14.8.2  |
| F-14.8.2 | Proprietary SDK Isolation         | R-14.8.3, R-14.8.4  |
| F-14.8.3 | Shared Build Server               | R-14.8.5, R-14.8.6  |
| F-14.8.4 | Remote Console Deployment         | R-14.8.7, R-14.8.8  |
| F-14.8.5 | Console Build Artifacts           | R-14.8.9, R-14.8.10 |

1. **F-14.8.1** — Console SDK compilation runs exclusively on a licensed build server. The editor
   triggers PS5, Xbox, and Switch builds via an authenticated REST API. The build server compiles
   the game against proprietary SDKs, produces signed console packages, and returns artifact URLs.
   No proprietary code, headers, or libraries exist on the client. The engine binary that ships to
   every developer contains zero console SDK references.
   - **Deps:** F-14.7.44 (Platform Abstraction Layer), F-15.18.4 (Build Farm)
   - **Platform:** Build server requires licensed SDKs from each console manufacturer. Client
     communicates only through abstract REST endpoints.
2. **F-14.8.2** — All proprietary SDKs (PlayStation SDK, Xbox GDK, Nintendo SDK) are loaded only on
   the build server. The open-source engine client communicates via abstract platform traits
   (F-14.7.44) with no SDK headers, libraries, or NDA-covered code. Console platform trait
   implementations exist solely in the server-side build environment as closed-source modules. The
   client-side engine compiles without any console feature flags.
   - **Deps:** F-14.7.44 (Platform Abstraction Layer)
   - **Platform:** Console trait implementations are never distributed. The open-source repository
     contains only the abstract trait definitions.
3. **F-14.8.3** — Multiple teams and game projects share a single licensed console build server. A
   priority-based job queue manages concurrent build requests from different projects. Per-project
   isolation ensures one team's builds cannot access another team's source or artifacts. Artifact
   caching keyed by content hash avoids redundant compilation across projects sharing the same
   engine version.
   - **Deps:** F-14.8.1 (Console Build Service), F-15.18.4 (Build Farm), F-15.18.8 (Shared Cache)
   - **Platform:** One console SDK license per server covers all teams using that server. Caching is
     per-engine-version to prevent ABI mismatches.
4. **F-14.8.4** — Deploy built console packages to development kits directly from the editor via the
   build server. The editor sends a deploy request to the build server REST API specifying the
   target dev kit. The build server transfers the signed package to the dev kit over the studio
   network. Developers test on console hardware without installing proprietary SDKs locally. Console
   output streams back to the editor via the build server relay.
   - **Deps:** F-14.8.1 (Console Build Service), F-15.14.2 (Deploy-to-Device)
   - **Platform:** Dev kits must be network-accessible from the build server. Dev kit communication
     uses manufacturer-provided deployment tools on the server.
5. **F-14.8.5** — The build server produces code-signed console packages (.pkg for PS5, .xvc for
   Xbox, .nsp for Switch) and stores them in S3-compatible object storage with content-hash keys.
   The editor downloads finished artifacts for local testing on connected dev kits. Artifact
   retention policies automatically expire old builds. Build manifests track which source revision,
   engine version, and SDK version produced each artifact.
   - **Deps:** F-14.8.1 (Console Build Service), F-15.18.5 (Signing Server), F-15.18.8 (Shared
     Cache)
   - **Platform:** Console package formats are NDA-covered. Only the build server handles
     format-specific packaging.
