# R-13.28 — Advertising Integration Requirements

## Rewarded Video Ads

| ID | Requirement | Derived From | Rationale | Verification |
|----|-------------|-------------|-----------|--------------|
| R-13.28.1 | The engine **SHALL** provide an opt-in rewarded video ad system with a request-show-complete lifecycle, frequency caps per session and per day, pre-availability checks, and reward delivery via the gameplay effect system on video completion. The engine **SHALL NOT** display rewarded ads during active gameplay. | [F-13.28.1](../../features/game-framework/advertising.md) | Rewarded ads generate revenue while respecting player agency; opt-in mechanics and frequency caps prevent ad fatigue and ensure ads are non-disruptive. | Integration test: request a rewarded ad, verify it caches within the load latency budget. Play the video to completion and verify the reward callback fires within 1 frame. Verify a second request within the frequency cap is rejected. Verify the ad option is hidden when no ad is available. Verify ads are blocked during active gameplay states. |

## Interstitial Ads

| ID | Requirement | Derived From | Rationale | Verification |
|----|-------------|-------------|-----------|--------------|
| R-13.28.2 | The engine **SHALL** display full-screen interstitial ads only at natural transition points (level completion, respawn, menu transition), enforce a configurable cooldown interval between displays (default: 5 minutes), pre-fetch ad content for instant display, and exempt players who have made any in-app purchase or hold a premium subscription. | [F-13.28.2](../../features/game-framework/advertising.md) | Interstitials at natural break points monetize free-to-play users without disrupting gameplay; IAP exemption rewards paying customers. | Integration test: trigger a level completion and verify an interstitial displays. Trigger a second transition within the cooldown window and verify no ad displays. Mark the player as having purchased an IAP and verify interstitials are suppressed. Verify pre-fetched ads display with zero loading delay. |

## Banner Ads

| ID | Requirement | Derived From | Rationale | Verification |
|----|-------------|-------------|-----------|--------------|
| R-13.28.3 | The engine **SHALL** render banner ads at screen edges during designated non-gameplay contexts (menus, loading screens), following IAB size standards (320x50 mobile, 728x90 tablet/desktop), with a configurable refresh interval (default: 30 seconds). Banners **SHALL NOT** overlay gameplay content. | [F-13.28.3](../../features/game-framework/advertising.md) | Banners provide passive ad revenue in menu contexts without affecting gameplay experience or screen real estate during active play. | Integration test: enable a banner in a menu screen and verify it renders at the correct IAB size for the current platform. Transition to gameplay and verify the banner is hidden. Wait for the refresh interval and verify the banner content updates. Verify banner placement matches the configured screen edge (top or bottom). |

## Ad Mediation and Network Abstraction

| ID | Requirement | Derived From | Rationale | Verification |
|----|-------------|-------------|-----------|--------------|
| R-13.28.4 | The engine **SHALL** provide an ad mediation layer that manages multiple ad networks through a unified API, selects the highest-eCPM ad via waterfall or bidding, integrates network SDKs as optional plugins (no ad SDK linked when advertising is disabled), collects GDPR/CCPA consent before network initialization, and reports revenue analytics through the telemetry system. All ad configuration **SHALL** be data-driven and editable in the visual editor. | [F-13.28.4](../../features/game-framework/advertising.md) | A mediation abstraction maximizes fill rate and eCPM by competing multiple networks; plugin-based integration keeps the ad SDK optional for non-advertising games. | Integration test: configure two ad networks with different eCPM values and verify the mediator selects the higher-eCPM network. Disable advertising in the project and verify no ad SDK is linked in the build output. Trigger an ad impression and verify analytics (impression count, eCPM) are reported via telemetry. Verify GDPR consent dialog appears before any ad network is initialized. |

## Non-Functional Requirements

| ID | Requirement | Derived From | Rationale | Verification |
|----|-------------|-------------|-----------|--------------|
| NFR-13.28.1 | Ad content (rewarded video, interstitial, banner) **SHALL** load from the mediation layer within 2 seconds of request under standard network conditions (50 Mbps, 50ms RTT). Pre-fetched ads **SHALL** display within 1 frame of the show call. |  | Slow ad loading frustrates players and reduces ad engagement; sub-2-second loads keep the experience responsive. | Measure ad load time across 100 requests on a 50 Mbps connection. Verify 95th percentile load time is under 2 seconds. Measure pre-fetched ad display latency and verify it completes within 1 frame (16.67ms at 60 FPS). |
| NFR-13.28.2 | The rewarded ad completion callback **SHALL** fire within 1 frame (16.67ms at 60 FPS) of video playback ending. Reward delivery to the player's inventory/currency **SHALL** complete within 100ms of callback. |  | Delayed reward delivery undermines player trust in the ad-for-reward exchange; instant feedback reinforces the value proposition. | Play a rewarded video to completion. Measure the time from video end to callback invocation and verify it is under 16.67ms. Measure the time from callback to inventory/currency update and verify it is under 100ms. |
| NFR-13.28.3 | The ad system **SHALL NOT** initialize any ad network SDK or transmit any user data to ad networks until GDPR/CCPA consent has been collected. Consent withdrawal **SHALL** disable all ad networks within 1 frame. |  | Privacy regulations require explicit consent before data collection; immediate withdrawal enforcement prevents post-consent data leakage. | Launch the app without granting consent and verify no ad network traffic is generated (network capture). Grant consent and verify ad networks initialize. Withdraw consent and verify all ad network activity ceases within 1 frame. |
