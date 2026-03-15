# User Stories -- Monetization and Live Operations (13.23)

## US-13.23.1 Battle Pass and Seasonal Progression
**As a** player, **I want** to earn battle pass tiers through gameplay activities across
time-limited seasons with free and premium reward tracks, **so that** I have ongoing
progression goals and exclusive rewards to work toward.

## US-13.23.2 Integrate Purchases Without Platform-Specific Code
**As a** game developer, **I want** a unified purchase API that works identically across
Apple, Google, Steam, and console stores, **so that** I write one purchase flow and it
works on all platforms without platform-specific branching.

## US-13.23.2a Validate Purchases Server-Side
**As a** game developer, **I want** all purchase receipts validated server-side with
duplicate replay rejection, **so that** fraudulent transactions are blocked and players
cannot exploit duplicate receipts.

## US-13.23.2b Buy Premium Currency
**As a** player (P-23), **I want** to buy premium currency with real money through my
platform's store and have it credited to my account immediately after validation, **so
that** I can spend it on cosmetics and convenience items.

## US-13.23.2c View My Purchase History
**As a** player (P-23), **I want** to view a complete history of my purchases including
transaction ID, date, amount, and refund status, **so that** I have a record of all my
spending and can track refunds.

## US-13.23.3 Verify My Subscription Is Active on Login
**As a** player (P-23), **I want** my subscription status verified automatically when I log
in and periodically during play, **so that** I always have the correct benefits without
manual intervention.

## US-13.23.4 Keep My Cosmetics After Subscription Lapses
**As a** player (P-23) whose subscription has lapsed, **I want** to keep all cosmetics and
inventory items I earned while subscribed and only lose tier-exclusive benefits like
priority queue, **so that** I am not punished for unsubscribing and I feel safe investing
time into earning rewards during my subscription period.

## US-13.23.4a Manage My Subscription In-Game
**As a** player (P-23), **I want** to view my current tier, renewal date, and billing
amount in-game and upgrade, downgrade, or cancel through my platform's store, **so that**
I can manage my subscription without leaving the game.

## US-13.23.5 Gift a Subscription to a Friend
**As a** player (P-23), **I want** to purchase a subscription as a gift for another player
by selecting them from my friend list, **so that** I can share the membership benefits
with friends and encourage them to play with me.

## US-13.23.6 Try the Full Game Before Buying
**As a** prospective player (P-23), **I want** to play the full game for a limited number
of hours as a free trial with my progress saved across sessions, **so that** I can evaluate
the game risk-free and continue where I left off if I decide to purchase.

## US-13.23.7 Play During a Free Weekend Event
**As a** non-owner (P-23), **I want** to download and play the full game during a free
weekend event with all my progress carrying over to a purchase, **so that** I can
experience the game without commitment and make an informed buying decision.

## US-13.23.8 Schedule a Free Weekend Promotion
**As a** studio executive (P-1), **I want** to configure free weekend events server-side
with start/end dates that automatically broadcast to the launcher, **so that** I can run
marketing promotions to drive player acquisition without requiring a client update.

## US-13.23.8a Try DLC Content During a Promotional Period
**As a** player (P-23), **I want** to temporarily access DLC content during a promotional
content trial, **so that** I can experience the expansion before deciding to purchase it.

## US-13.23.9 Buy and Download a DLC Expansion
**As a** player (P-23), **I want** to browse available DLC in the in-game store, purchase an
expansion pack, and download it on demand without reinstalling the base game, **so that** I
can access new content quickly with a small incremental download.

## US-13.23.10 Get a Bundle Deal on Multiple DLC
**As a** player (P-23), **I want** to see bundle deals that discount multiple DLC packs when
purchased together, **so that** I save money when buying several expansions at once and feel
I am getting good value.

## US-13.23.11 Browse and Buy Cosmetics in the Shop
**As a** player (P-23), **I want** to browse a rotating cosmetic store with character skins,
emotes, mount skins, and seasonal limited-time items purchasable with earned or premium
currency, **so that** I can personalize my character's appearance without gaining a gameplay
advantage.

## US-13.23.12 Refund a Cosmetic Purchase Within 24 Hours
**As a** player (P-23), **I want** to refund any cosmetic purchase within 24 hours with no
questions asked and receive my currency back immediately, **so that** I can buy with
confidence knowing I have a safety net if I change my mind.

## US-13.23.13 Use Cosmetics Across All My Characters
**As a** player (P-23), **I want** cosmetics I purchase to be available on all characters on
my account, **so that** I do not have to re-buy the same skin for each character I play.

## US-13.23.14 Close Ads Easily Without Being Tricked
**As a** player (P-23), **I want** ad close buttons to be at least 44x44 points, clearly
visible, and functional immediately, **so that** I can always close an ad without being
tricked by fake or hidden close buttons.

## US-13.23.14a Protect My Child from Targeted Ads
**As a** parent (P-23), **I want** the engine to automatically detect that my child is
under 16 via platform parental controls and serve only contextual (non-personalized) ads,
**so that** my child is protected from behaviorally-targeted advertising.

## US-13.23.15 Play Without Any Ads
**As a** player (P-23), **I want** a global "disable all ads" toggle in accessibility
settings that completely removes all ad formats from the game, **so that** I can play
without any advertising interruptions if I prefer.

## US-13.23.16 Trust That Ads Cannot Trick Me
**As a** player (P-23), **I want** the engine to reject ads that auto-play audio, vibrate
my device, launch external apps without consent, or obscure the "Ad" label, **so that** I
am never manipulated by dark pattern advertising.

## US-13.23.16a Not Be Overwhelmed by Ad Frequency
**As a** player (P-23), **I want** ad frequency hard-capped at 1 interstitial per 10
minutes and 3 rewarded videos per hour (engine-enforced, not overridable), **so that** I
am never bombarded with excessive advertising regardless of the game's configuration.

## US-13.23.17 Configure Monetization Strategy for a New Title
**As a** studio executive (P-1), **I want** to configure subscription tiers, DLC pricing,
cosmetic store rotation schedules, and ad frequency caps through the visual editor, **so
that** I can define the monetization strategy for each title without engineering involvement.

## US-13.23.18 Set Advertising Safeguards with Confidence
**As a** studio executive (P-1), **I want** to know that the engine enforces advertising
safeguards (minimum close button size, frequency caps, minor protections) at the binary level
that cannot be overridden by game configuration, **so that** my studio is legally compliant
and our players are protected regardless of how individual teams configure their games.
