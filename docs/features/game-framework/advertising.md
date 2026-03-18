# 13.28 — Advertising Integration

| ID        | Feature                              | Requirements |
|-----------|--------------------------------------|--------------|
| F-13.28.1 | Rewarded Video Ads                   | R-13.28.1    |
| F-13.28.2 | Interstitial Ads                     | R-13.28.2    |
| F-13.28.3 | Banner Ads                           | R-13.28.3    |
| F-13.28.4 | Ad Mediation and Network Abstraction | R-13.28.4    |

1. **F-13.28.1** — Opt-in video advertisements that grant in-game rewards on completion. The ad
   system provides a request-show-complete lifecycle: gameplay requests an ad, the SDK fetches and
   caches a video from the ad mediation layer, the player opts in (never forced), the video plays in
   a fullscreen overlay, and on completion the reward callback grants configured items/currency via
   the gameplay effect system (F-13.10.3). Frequency caps limit how many rewarded ads a player can
   watch per session/day. Ad availability is checked before showing the option to prevent dead
   buttons. The ad mediation layer abstracts multiple ad networks behind a unified API. Ads are
   never shown during active gameplay -- only during natural breaks (between rounds, in menus, at
   rest points).
   - **Deps:** F-13.10.3 (Gameplay Effects), F-13.7.6 (Currency)
   - **Platform:** Uses platform ad SDKs: AdMob on iOS/Android, Steam Inventory for PC reward items.
     Console platforms generally prohibit in-game advertising.
2. **F-13.28.2** — Full-screen ads shown at natural transition points (level completion, respawn,
   menu transition). Interstitials are never shown mid-gameplay or during time-sensitive moments. A
   cooldown timer prevents showing interstitials more frequently than a configurable interval
   (default: 5 minutes). The ad type (static image, video, playable) is determined by the mediation
   layer. Loading is pre-fetched so display is instant with no loading spinner. Players who have
   purchased any IAP or are on a premium subscription tier are automatically exempt from
   interstitial ads.
   - **Deps:** F-13.28.1, F-13.23.3 (IAP)
   - **Platform:** Same platform SDK abstraction as rewarded ads.
3. **F-13.28.3** — Non-intrusive banner advertisements displayed at screen edges (top or bottom)
   during menu screens, loading screens, or designated UI panels. Banners never overlay gameplay.
   Banner size follows IAB standards (320x50 mobile, 728x90 tablet/desktop). Refresh rate is
   configurable (default: 30 seconds). Banner position, size, and visibility are controlled through
   the UI widget system (F-10.1.1) as a special widget type. Banners are hidden during gameplay and
   shown only in designated contexts.
   - **Deps:** F-10.1.1 (Widget Tree)
   - **Platform:** Banner sizes follow platform guidelines. Not available on console.
4. **F-13.28.4** — A mediation layer that manages multiple ad networks (AdMob, Unity Ads, AppLovin,
   IronSource, Meta Audience Network) through a unified API. The mediator selects the highest-eCPM
   ad from available networks using waterfall or bidding-based selection. Network SDKs are
   integrated as optional platform plugins -- games without advertising do not link any ad SDK.
   GDPR/CCPA consent is collected before initializing ad networks; the consent dialog is
   configurable. Ad revenue analytics (impressions, eCPM, fill rate) are reported through the
   telemetry system. All ad configuration is data-driven -- ad unit IDs, frequency caps, cooldowns,
   and network priority are set in a visual editor panel, not code.
   - **Deps:** F-13.28.1, F-13.28.2, F-13.28.3, F-14.5.6 (Entitlements)
   - **Platform:** Each ad network requires platform-specific SDK integration. GDPR consent uses
     platform-native consent frameworks where available (ATT on iOS).
