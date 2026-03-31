# R-13.23 — Monetization and Live Operations Requirements

## Tiered Progression Track

1. **R-13.23.1** — The engine **SHALL** provide a data-driven tiered progression track with
   configurable season duration, per-tier rewards, XP thresholds, and server-loaded definitions
   supporting live-ops updates without client patches.
   - **Rationale:** Server-driven definitions enable live-ops teams to iterate without client
     updates.
   - **Verification:** Load a 50-tier definition. Award XP and verify correct tier advancement and
     reward grants. Deploy a new definition server-side and verify client reflects the change within
     60 seconds.

## Scheduled Task and Tracking

2. **R-13.23.2** — The engine **SHALL** support server-defined scheduled task lists with incremental
   progress tracking, configurable rewards, scheduled rotation, and per-period reroll at currency
   cost.
   - **Rationale:** Server-side task definitions allow live-ops deployment without client updates.
   - **Verification:** Deploy a daily task set. Advance a counter and verify progress updates.
     Verify rotation replaces tasks after the interval. Verify reroll deducts currency.

## Platform Purchase Abstraction

3. **R-13.23.3** — The engine **SHALL** abstract platform store SDKs (StoreKit 2, Play Billing,
   Steam, console) behind a unified purchase API with server-side receipt validation, duplicate
   replay rejection within 100 ms, and a virtual currency ledger with governance rules.
   - **Rationale:** Unified abstraction reduces per-platform cost; server validation prevents fraud.
   - **Verification:** Complete a purchase and verify server validation succeeds. Replay the receipt
     and verify rejection within 100 ms. Credit currency and verify server-side balance. Attempt
     client-side balance modification and verify server rejects it.

4. **R-13.23.4** — The engine **SHALL** record every transaction in a per-player purchase history
   with transaction ID, platform, amount, timestamp, and refund status, automatically updating on
   platform refund notifications.
   - **Rationale:** Purchase history provides audit trails; automated refund tracking reduces
     support burden.
   - **Verification:** Complete a purchase and verify history entry. Trigger a refund notification
     and verify status updates. Query history via the account UI and verify all fields.

## Login Reward Calendar

5. **R-13.23.5** — The engine **SHALL** provide a login reward calendar with server-validated
   timestamps, configurable streak milestones, escalating rewards, and strict (reset on miss) or
   lenient (catch-up) modes.
   - **Rationale:** Server timestamps prevent clock manipulation; mode selection enables tuning.
   - **Verification:** Simulate 7 consecutive logins and verify milestone reward. In strict mode,
     skip a day and verify streak reset. In lenient mode, verify catch-up stamp consumed. Manipulate
     client clock and verify server rejects it.

## Subscription Management

6. **R-13.23.6** — The engine **SHALL** verify subscription state server-side on login and
   periodically (configurable interval, default 15 minutes), granting per-tier benefits on active
   status and revoking within one interval of lapse without deleting earned content.
   - **Rationale:** Server verification prevents spoofing; content retention encourages
     re-subscription.
   - **Verification:** Create a subscription and verify benefits active on login. Expire it and
     verify revocation within one interval. Verify earned cosmetics and inventory remain accessible.

7. **R-13.23.7** — The engine **SHALL** provide in-game subscription management UI (tier, renewal
   date, billing) redirecting to platform-native management, and support gift subscriptions that do
   not auto-renew.
   - **Rationale:** In-game management reduces friction; non-renewing gifts prevent unwanted
     charges.
   - **Verification:** Open management UI and verify current tier and renewal date. Gift a
     subscription and verify recipient activation with notification. Verify gift does not
     auto-renew.

## Trial and Free Events

8. **R-13.23.8** — The engine **SHALL** support configurable game trials (N hours, server-tracked,
   cross-session persistence), free weekend events (server-scheduled), and content trials (temporary
   DLC unlock), all preserving progress on purchase.
   - **Rationale:** Progress preservation removes friction for trial-to-purchase conversion.
   - **Verification:** Start a 2-hour trial, play 30 minutes, restart, verify 90 minutes remain.
     Play to expiration and verify revert. Purchase and verify all progress intact. Configure a free
     weekend and verify access during the window and revocation after.

## DLC and Cosmetic Store

9. **R-13.23.9** — The engine **SHALL** support DLC as signed asset bundles downloaded on demand,
   verified by entitlements, with a store UI showing available/owned DLC, local-currency pricing,
   and bundle deals.
   - **Rationale:** Signed bundles prevent tampered content; on-demand download keeps the base
     install small.
   - **Verification:** Publish a DLC bundle, purchase, download, and verify content accessible.
     Attempt loading a tampered bundle and verify rejection. Verify bundle pricing applies discount.

10. **R-13.23.10** — The engine **SHALL** provide a cosmetic store where all items provide zero
    gameplay advantage, with three currency types (premium, earned, event), a 24-hour no-approval
    refund window, account-bound items, and a configurable rotation schedule.
    - **Rationale:** Cosmetic-only monetization avoids pay-to-win; automatic refunds build trust.
    - **Verification:** Purchase a cosmetic and verify no stat changes. Equip on one character and
      verify account-wide access. Refund within 24 hours and verify instant currency restoration.
      Refund after 24 hours and verify rejection.

## Non-Functional Requirements

11. **R-13.23.NF1** — Purchase flow from platform dialog to item/currency delivery **SHALL**
    complete within 3 seconds. Subscription verification **SHALL** complete within 500 ms on login.
    - **Rationale:** Fast purchase delivery and login checks prevent player frustration.
    - **Verification:** Measure purchase delivery time across 50 transactions and verify p99 under 3
      seconds. Measure subscription verification across 100 logins and verify p99 under 500 ms.

12. **R-13.23.NF2** — The cosmetic store **SHALL** display the initial page within 2 seconds and
    maintain 60 fps scrolling through 500+ items.
    - **Rationale:** A responsive store encourages browsing and purchasing.
    - **Verification:** Open the store with 500 items and verify first page renders within 2
      seconds. Scroll through the full catalog and verify frame rate stays above 58 fps.
