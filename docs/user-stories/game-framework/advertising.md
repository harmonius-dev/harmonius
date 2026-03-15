# User Stories -- Advertising Integration (13.28)

## Rewarded Video Ads (F-13.28.1)

## US-13.28.1.1
**As a** player (P-23), **I want** to opt in to watch a video ad and receive a configured
reward on completion, **so that** I can earn items or currency without spending money.

## US-13.28.1.2
**As a** player (P-23), **I want** ads never shown during active gameplay, only during natural
breaks, **so that** ad viewing is voluntary and non-disruptive.

## US-13.28.1.3
**As a** player (P-23), **I want** ad availability checked before the option is shown, **so
that** I never see a dead button for an unavailable ad.

## US-13.28.1.4
**As a** executive (P-1), **I want** the ad mediation layer to abstract multiple ad networks
behind a unified API, **so that** ad sourcing is automatic.

## US-13.28.1.5
**As a** executive (P-1), **I want** frequency caps limiting rewarded ads per session and per
day, **so that** ad volume is controlled.

## US-13.28.1.6
**As a** tester (P-27), **I want** to verify that the reward callback grants items only on
video completion, **so that** partial views do not award rewards.

## US-13.28.1.7
**As a** tester (P-27), **I want** to verify that the frequency cap blocks additional ads when
the limit is reached, **so that** caps are enforced.

## Interstitial Ads (F-13.28.2)

## US-13.28.2.1
**As a** player (P-23), **I want** interstitial ads shown only at natural transition points
like level completion or menu transitions, **so that** gameplay is never interrupted.

## US-13.28.2.2
**As a** player (P-23), **I want** purchasing any IAP or subscribing to automatically exempt me
from interstitial ads, **so that** paying players are not shown ads.

## US-13.28.2.3
**As a** executive (P-1), **I want** a configurable cooldown timer preventing interstitials
more frequently than the minimum interval, **so that** ad frequency is bounded.

## US-13.28.2.4
**As a** executive (P-1), **I want** interstitial ad loading to be pre-fetched for instant
display, **so that** no loading spinner appears.

## US-13.28.2.5
**As a** tester (P-27), **I want** to verify that an interstitial requested within the
cooldown is suppressed, **so that** the cooldown timer works.

## US-13.28.2.6
**As a** tester (P-27), **I want** to verify that a player with any IAP purchase never sees
interstitials, **so that** the IAP exemption is enforced.

## Banner Ads (F-13.28.3)

## US-13.28.3.1
**As a** player (P-23), **I want** banner ads shown only during menu and loading screens,
never during gameplay, **so that** banners do not obstruct game content.

## US-13.28.3.2
**As a** player (P-23), **I want** banners sized to IAB standards and positioned at screen
edges, **so that** ads are non-intrusive.

## US-13.28.3.3
**As a** executive (P-1), **I want** banner refresh rate configurable (default 30 seconds),
**so that** impression frequency is controlled.

## US-13.28.3.4
**As a** tester (P-27), **I want** to verify that banners are hidden during gameplay and shown
only in designated contexts, **so that** context gating works.

## US-13.28.3.5
**As a** tester (P-27), **I want** to verify that banner size follows platform guidelines
(320x50 mobile, 728x90 tablet/desktop), **so that** sizing is correct.

## Ad Mediation and Network Abstraction (F-13.28.4)

## US-13.28.4.1
**As a** executive (P-1), **I want** the mediator to select the highest-eCPM ad from available
networks using waterfall or bidding selection, **so that** ad revenue is maximized.

## US-13.28.4.2
**As a** executive (P-1), **I want** GDPR/CCPA consent collected before initializing ad
networks, **so that** the game complies with privacy regulations.

## US-13.28.4.3
**As a** executive (P-1), **I want** ad revenue analytics (impressions, eCPM, fill rate)
reported through the telemetry system, **so that** I can monitor monetization performance.

## US-13.28.4.4
**As a** executive (P-1), **I want** ad network SDKs integrated as optional platform plugins
so games without ads do not link any SDK, **so that** non-ad games have no ad overhead.

## US-13.28.4.5
**As a** player (P-23), **I want** ad tracking disabled immediately when I withdraw GDPR
consent, **so that** my privacy preference is respected instantly.

## US-13.28.4.6
**As a** tester (P-27), **I want** to verify that withdrawing consent disables all ad network
tracking, **so that** the consent withdrawal flow is complete.

## US-13.28.4.7
**As a** tester (P-27), **I want** to verify that all ad configuration (unit IDs, frequency
caps, network priority) is data-driven and not in code, **so that** ad settings are editable
without recompilation.
