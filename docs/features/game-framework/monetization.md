# 13.23 — Monetization and Live Operations

| ID         | Feature                              | Requirements |
|------------|--------------------------------------|--------------|
| F-13.23.1  | Battle Pass and Season System        | R-13.23.1    |
| F-13.23.2  | Daily and Weekly Challenge System    | R-13.23.2    |
| F-13.23.3a | Platform Purchase Abstraction        | R-13.23.3a   |
| F-13.23.3b | Server-Side Receipt Validation       | R-13.23.3b   |
| F-13.23.3c | Premium Currency System              | R-13.23.3c   |
| F-13.23.3d | Purchase History and Refund Tracking | R-13.23.3d   |
| F-13.23.4  | Daily Login Reward Calendar          | R-13.23.4    |
| F-13.23.5a | Subscription State Verification      | R-13.23.5a   |
| F-13.23.5b | Subscription Benefit Application     | R-13.23.5b   |
| F-13.23.5c | Subscription Management UI           | R-13.23.5c   |
| F-13.23.5d | Subscription Gifting                 | R-13.23.5d   |
| F-13.23.6a | Timed Game Trial                     | R-13.23.6a   |
| F-13.23.6b | Free Weekend Events                  | R-13.23.6b   |
| F-13.23.6c | Content Trial                        | R-13.23.6c   |
| F-13.23.7  | DLC and Expansion Purchasing         | R-13.23.7    |
| F-13.23.8  | Cosmetic Store and Virtual Currency  | R-13.23.8    |
| F-13.23.9a | Deceptive UI Prevention              | R-13.23.9a   |
| F-13.23.9b | Minor-Targeted Ad Blocking           | R-13.23.9b   |
| F-13.23.9c | Dark Pattern Prevention              | R-13.23.9c   |
| F-13.23.9d | Frequency Cap Enforcement            | R-13.23.9d   |

1. **F-13.23.1** — Tiered reward track with free and premium tiers, time-limited to seasons
   (configurable duration, typically 60-90 days). Players earn pass XP from gameplay activities
   (matches, quests, daily/weekly challenges) that advance their tier. Each tier awards rewards:
   free-tier items available to all players, premium-tier items requiring pass purchase. Rewards
   include cosmetics, currency, consumables, and exclusive items. The pass UI displays current tier,
   XP progress, all tier rewards (locked and unlocked), and days remaining. Season transitions reset
   progress but allow catch-up mechanics (XP boost late in the season). Pass definitions are data
   assets loaded from the server.
   - **Deps:** F-13.7.1 (Table Schema), F-13.12.6 (Achievements)
2. **F-13.23.2** — Rotating challenge lists that refresh on configurable schedules. Daily challenges
   are simpler tasks (kill 10 enemies, craft 5 items, complete a dungeon). Weekly challenges are
   larger objectives (win 5 PvP matches, reach reputation tier with a faction). Each challenge has:
   description, progress tracking (incremental counter or boolean), reward (XP, currency, items),
   and optional difficulty tiers (easy/medium/hard). Active challenges are displayed in a dedicated
   HUD panel. Challenges are defined server-side for live-ops flexibility — new challenges can be
   deployed without a client update. A subset of challenges can be rerolled per day at a currency
   cost.
   - **Deps:** F-1.1.30 (Observers), F-13.7.6 (Currency)
3. **F-13.23.3a** — Unified purchase API abstracting platform-specific store SDKs (Apple StoreKit 2,
   Google Play Billing 5+, Steam ISteamMicroTxn, console native store APIs). The abstraction exposes
   a single `initiate_purchase` / `on_purchase_complete` flow regardless of platform. Platform
   selection is automatic based on the runtime environment. The abstraction handles
   platform-specific quirks: deferred purchases on iOS, pending transactions on Google Play, and
   overlay callbacks on Steam.
   - **Deps:** F-14.5.1 (Platform Services)
   - **Platform:** Apple requires StoreKit 2 on iOS 15+. Google requires Play Billing 5+. Steam uses
     ISteamMicroTxn. Console store APIs vary per manufacturer.
4. **F-13.23.3b** — All purchase receipts are validated server-side before granting items or
   currency. The validation service contacts the platform's verification endpoint (Apple
   /verifyReceipt, Google androidpublisher API, Steam ISteamMicroTxn/QueryTxn). Duplicate receipt
   replay is detected and rejected. Failed validations trigger a retry queue with exponential
   backoff. Validation results are logged with transaction ID, timestamp, and status for audit.
   - **Deps:** F-13.23.3a (Platform Purchase Abstraction), F-8.7.7 (Cross-Shard Services)
   - **Platform:** Each platform has a distinct receipt format and verification endpoint. Server
     must hold per-platform credentials.
5. **F-13.23.3c** — Premium currency is a special currency type (F-13.7.6) purchasable with real
   money via IAP and spendable on cosmetic shop items, battle pass tiers, and convenience features.
   Currency balances are stored server-side to prevent tampering. Purchases credit currency only
   after successful receipt validation (F-13.23.3b). All monetized items are cosmetic or convenience
   only — no pay-to-win items in the default configuration (enforceable via governance rules).
   - **Deps:** F-13.7.6 (Currency), F-13.23.3b (Server-Side Receipt Validation)
6. **F-13.23.3d** — Every completed transaction is recorded in a per-player purchase history with
   transaction ID, platform, item/currency purchased, amount, timestamp, and refund status. The
   refund status field tracks: none, requested, approved, denied. Platform refund notifications
   (Apple Server Notifications v2, Google Real-Time Developer Notifications) update the refund
   status automatically. The purchase history is queryable via the game's account management UI.
   - **Deps:** F-13.23.3b (Server-Side Receipt Validation), F-13.7.1 (Table Schema)
   - **Platform:** Apple and Google push refund notifications via webhooks. Steam and console refund
     flows vary per platform.
7. **F-13.23.4** — Reward calendar that grants escalating rewards for consecutive daily logins. The
   calendar displays 28-30 days of rewards with the current day highlighted. Consecutive login
   streaks grant bonus rewards at milestones (7-day, 14-day, 28-day). Missing a day optionally
   resets the streak (strict mode) or allows catch-up stamps (lenient mode). Monthly calendar resets
   with new reward pools. Login detection uses server-side timestamp validation to prevent clock
   manipulation. Calendar state is replicated per player account and displayed on the launcher or
   game main menu.
   - **Deps:** F-13.7.1 (Table Schema), F-8.7.7 (Cross-Shard Services)
8. **F-13.23.5a** — Server-side verification of subscription state on login and periodically during
   play (default interval 15 minutes, configurable). The verification service queries the platform
   subscription API (StoreKit 2 subscription status, Google Play subscriptions:get, Steam
   ISteamMicroTxn) to confirm active status. Lapsed subscriptions are detected within one
   verification interval. A local cache avoids redundant API calls within the interval.
   - **Deps:** F-14.5.6 (Entitlements), F-13.23.3a (Platform Purchase Abstraction), F-8.7.7
     (Cross-Shard Services)
   - **Platform:** Apple requires App Store subscription status API on iOS. Google requires Play
     Billing subscriptions:get. Steam uses ISteamMicroTxn recurring.
9. **F-13.23.5b** — Configurable per-tier benefit grants and revocations. Benefits include: bonus
   XP/currency multipliers, exclusive cosmetic access, ad-free experience, priority matchmaking
   queue, expanded inventory/storage, and exclusive game modes. When a subscription lapses, benefits
   are revoked within one verification interval (F-13.23.5a) without deleting earned content —
   cosmetics, inventory items, and progression data are preserved. Tier changes (upgrade or
   downgrade) adjust benefits immediately on the next verification.
   - **Deps:** F-13.23.5a (Subscription State Verification), F-14.5.6 (Entitlements)
10. **F-13.23.5c** — In-game UI for managing subscription state: view current tier and benefits,
    upgrade to a higher tier, downgrade to a lower tier, and cancel. All management actions redirect
    to the platform store's native subscription management flow (App Store subscription management,
    Google Play subscription center, Steam account page). The UI displays renewal date, next billing
    amount, and tier comparison table.
    - **Deps:** F-13.23.5a (Subscription State Verification), F-13.23.3a (Platform Purchase
      Abstraction)
    - **Platform:** Apple requires managing subscriptions through the App Store. Google requires
      Play Billing subscription management. Steam manages through the Steam client.
11. **F-13.23.5d** — Players can purchase a subscription as a gift for another player. The gifter
    selects a recipient (by player ID or friend list), chooses a tier and duration, and completes
    the purchase through the platform store. The recipient receives the subscription with a
    notification and the gifter's name. Gift subscriptions do not auto-renew — the recipient must
    explicitly subscribe after the gift period ends.
    - **Deps:** F-13.23.5a (Subscription State Verification), F-13.23.3a (Platform Purchase
      Abstraction), F-13.10.1 (Friend System)
    - **Platform:** Not all platforms support gift subscriptions natively. Where unsupported, the
      engine handles gifting server-side with a one-time purchase.
12. **F-13.23.6a** — New players can play the full game for N hours (configurable per title) as a
    free trial. Trial time tracking persists across sessions — closing and reopening the game does
    not reset the clock. All trial progress (character data, inventory, achievements) carries over
    to purchase without data loss. Trial expiration reverts the player to non-owner state while
    preserving saved data. On session end, trial players see a non-intrusive purchase prompt
    displaying playtime and current savings/sale price.
    - **Deps:** F-14.5.6 (Entitlements), F-8.7.7 (Cross-Shard Services)
    - **Platform:** Platform stores handle trial download permissions. Trial time is tracked
      server-side to prevent clock manipulation.
13. **F-13.23.6b** — Server-scheduled events where non-owners can download and play the full game
    for a calendar weekend. Event start/end timestamps are configured server-side and broadcast to
    the launcher. Non-owners receive a launcher notification when a free weekend is active. Access
    is automatically revoked after the event end timestamp. Progress made during the free weekend
    carries over if the player later purchases the game.
    - **Deps:** F-14.5.6 (Entitlements), F-15.15.1 (Launcher), F-8.7.7 (Cross-Shard Services)
    - **Platform:** Steam Free Weekends use Steam's native free weekend API. Other platforms use the
      engine's entitlement system with server-side scheduling.
14. **F-13.23.6c** — Specific DLC or expansion content is temporarily unlocked for all players
    (owners and non-owners of that DLC) during server-configured promotional periods. Content trial
    start/end timestamps are set server-side. The entitlement system (F-14.5.6) grants temporary
    access that automatically reverts on expiration. Players who enjoy the content trial see a
    targeted purchase prompt for the full DLC.
    - **Deps:** F-14.5.6 (Entitlements), F-13.23.7 (DLC Purchasing), F-8.7.7 (Cross-Shard Services)
    - **Platform:** Content trials require the DLC assets to be temporarily downloadable. Platform
      store integration varies.
15. **F-13.23.7** — Purchase and download additional game content (new zones, story chapters,
    character classes, game modes) through the in-game store or platform storefront. Each DLC is a
    signed asset bundle (F-15.14.6) gated by entitlement verification (F-14.5.6). The DLC store UI
    displays: available DLC with descriptions/screenshots/trailers, owned DLC, prices in local
    currency, bundle deals (multiple DLC at discount), and seasonal sales. DLC content is downloaded
    on demand — not included in the base install. Download progress is visible in the launcher and
    in-game. DLC that adds gameplay systems (new class, new crafting) integrates through the modular
    system (F-13.1.9) and activates on next login.
    - **Deps:** F-15.14.6 (Asset Bundles/DLC), F-14.5.6 (Entitlements), F-13.1.9 (Modular Systems)
    - **Platform:** Each platform store handles DLC purchasing natively. The engine verifies
      entitlements.
16. **F-13.23.8** — An in-game cosmetic shop where players spend premium or earned currency on
    visual items that provide no gameplay advantage. Cosmetic categories: character skins, weapon
    skins, mount skins, emotes, UI themes, profile frames, titles, and seasonal limited-time items.
    Virtual currency types: premium (purchased with real money via IAP F-13.23.3c), earned (from
    gameplay: quests, achievements, daily rewards), and event (from seasonal events, expires after
    event). The store rotates featured items on a configurable schedule (daily/weekly). Purchase
    history and refund window (24 hours, no questions asked) are tracked. Items are bound to the
    account, not per-character, for cross-character access. The cosmetic store respects the AI
    governance toggle (F-15.7.3) — AI-generated cosmetics are clearly labeled.
    - **Deps:** F-13.23.3a (Platform Purchase Abstraction), F-13.7.6 (Currency), F-13.8.9 (Modular
      Mesh Parts), F-15.7.3 (AI Toggle)
    - **Platform:** Console certification requires that purchased cosmetics are never lost due to
      server issues.
17. **F-13.23.9a** — Engine-enforced rules preventing deceptive ad UI. Ad close buttons SHALL be
    minimum 44x44 points, clearly visible, and functional immediately upon display — no delayed
    activation, no fake close buttons, no invisible tap regions. The ad mediation layer (F-13.28.4)
    rejects ad creatives that mimic game UI elements, fake system notifications, or simulate
    interactive game content. These rules are compiled into the engine binary and cannot be
    overridden by game configuration.
    - **Deps:** F-13.28.1 (Rewarded Ads), F-13.28.2 (Interstitial), F-13.28.3 (Banner)
    - **Platform:** Complies with platform certification requirements for ad display.
18. **F-13.23.9b** — If the player's age is under 16 (as reported by platform parental controls), no
    personalized or behaviorally-targeted advertising is served. Only contextual ads are permitted
    for minors. Age detection integrates with platform parental control APIs (Apple Screen Time,
    Google Family Link, console parental settings). The age check is performed on session start and
    cached for the session duration.
    - **Deps:** F-14.5.1 (Platform Services), F-10.6.1 (Accessibility)
    - **Platform:** Complies with COPPA (US), GDPR Article 8 (EU), and Apple's SKAdNetwork privacy
      requirements.
19. **F-13.23.9c** — Engine-enforced rules preventing dark pattern advertising. Ads SHALL NOT:
    auto-play audio without user initiation, vibrate the device to attract attention, launch
    external apps without explicit consent, or obscure the "Ad" label. The ad mediation layer
    validates each ad creative against these rules before display. Violations are logged and the
    offending ad is suppressed. A global "disable all ads" setting in accessibility preferences
    disables all ad formats entirely. These rules are compiled into the engine binary.
    - **Deps:** F-13.28.1 (Rewarded Ads), F-13.28.2 (Interstitial), F-13.28.3 (Banner), F-10.6.1
      (Accessibility)
20. **F-13.23.9d** — Engine-enforced hard caps on ad frequency: maximum 1 interstitial per 10
    minutes and 3 rewarded videos per hour. Caps are enforced engine-side regardless of game
    developer configuration or ad mediation settings. Cap timers reset on a rolling window (not
    calendar boundaries). Attempts to display ads exceeding the cap are silently blocked and logged.
    The cap values are compiled into the engine binary and cannot be changed at runtime.
    - **Deps:** F-13.28.1 (Rewarded Ads), F-13.28.2 (Interstitial)
