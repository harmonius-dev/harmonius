# User Stories — 13.13 Social and Guild Systems

## F-13.13.1a Guild CRUD and Membership

## US-13.13.1a.1 Monitor Guild Membership for Server Capacity

**As a** server admin (P-22), **I want to** monitor guild creation rates and membership counts,
**so that** I can track social system load on the server.

## US-13.13.1a.2 Create a Guild

**As a** player (P-23), **I want to** create a guild with a name and emblem, **so that** I can
form a persistent social group for coordinated gameplay.

## US-13.13.1a.3 Invite and Accept Guild Members

**As a** player (P-23), **I want to** invite players by name or proximity and accept
applications, **so that** I can grow my guild's roster.

## US-13.13.1a.4 Leave or Disband a Guild

**As a** player (P-23), **I want to** leave a guild freely or disband it as founder with a
cooldown confirmation, **so that** I can manage my guild membership with safety against
accidental disbandment.

## US-13.13.1a.5 Verify Guild CRUD Operations

**As an** engine tester (P-27), **I want to** create, join, kick, leave, and disband guilds and
verify all operations update membership state correctly, **so that** guild lifecycle
operations are robust.

## F-13.13.1b Guild Rank and Permission System

## US-13.13.1b.1 Manage Guild Ranks and Permissions

**As a** player (P-23), **I want to** the guild founder to create, rename, reorder, and delete
ranks with configurable permissions, **so that** guild management can be delegated
appropriately.

## US-13.13.1b.2 Verify Permission Checks on Privileged Actions

**As an** engine tester (P-27), **I want to** attempt each privileged action (invite, kick,
promote, demote, bank access, start events) with insufficient rank and verify it is rejected,
**so that** permission enforcement is correct.

## US-13.13.1b.3 Verify Rank Assignment Persists Across Sessions

**As an** engine tester (P-27), **I want to** assign ranks, log out, log back in, and verify
rank assignments are preserved, **so that** rank persistence works correctly.

## F-13.13.1c Guild Leveling and Perks

## US-13.13.1c.1 Configure Guild XP Sources and Perk Unlocks

**As a** server admin (P-22), **I want to** view guild level distribution and perk activation
rates across the server, **so that** I can verify the guild economy is healthy.

## US-13.13.1c.2 Contribute to Guild XP Through Activities

**As a** player (P-23), **I want to** my quests, dungeons, PvP wins, and crafting to contribute
XP toward guild leveling, **so that** collective activity advances the guild.

## US-13.13.1c.3 Unlock Guild Perks at Level Thresholds

**As a** player (P-23), **I want to** guild perks (bonus XP, larger roster, guild mounts,
cosmetics) to unlock at level thresholds, **so that** guild progression provides tangible
benefits.

## US-13.13.1c.4 Verify Perk Activation at Level Thresholds

**As an** engine tester (P-27), **I want to** level a guild to each perk threshold and verify
the perk activates for all members, **so that** perk unlocks work correctly.

## F-13.13.1d Guild Roster UI

## US-13.13.1d.1 Browse the Guild Roster

**As a** player (P-23), **I want to** view all guild members with online/offline status, rank,
level, class, and last-login date, **so that** I can coordinate with guildmates.

## US-13.13.1d.2 Sort and Filter the Roster

**As a** player (P-23), **I want to** sort the roster by any column and filter by rank or online
status, **so that** I can find specific members quickly.

## US-13.13.1d.3 Verify Roster Performance With 1000 Members

**As an** engine tester (P-27), **I want to** render a roster with 1000 members and verify
smooth scrolling and responsive sorting, **so that** large guilds do not degrade UI
performance.

## US-13.13.1d.4 Verify Context Menu Actions Respect Permissions

**As an** engine tester (P-27), **I want to** right-click roster entries and verify the context
menu only shows actions permitted by the viewer's rank, **so that** the UI respects
permission rules.

## F-13.13.2 Guild Storage and Bank

## US-13.13.2.1 Monitor Guild Bank Transaction Logs

**As a** server admin (P-22), **I want to** transaction logs to record every deposit and
withdrawal with timestamps and member names, **so that** guild disputes can be audited.

## US-13.13.2.2 Deposit and Withdraw Items From the Guild Bank

**As a** player (P-23), **I want to** deposit and withdraw items from a shared guild bank with
per-rank permissions and daily withdrawal limits, **so that** the guild can pool resources
while preventing abuse.

## US-13.13.2.3 Browse Guild Bank With Tab Navigation

**As a** player (P-23), **I want to** the guild bank UI to have multiple tabs (unlocked by guild
level) with tab navigation and permission indicators, **so that** bank organization is clear.

## US-13.13.2.4 Verify Daily Withdrawal Limits

**As an** engine tester (P-27), **I want to** withdraw up to the daily limit and verify the
next withdrawal is blocked until reset, **so that** daily limits prevent bank draining.

## US-13.13.2.5 Verify Transaction Log Completeness

**As an** engine tester (P-27), **I want to** perform 100 deposits and withdrawals and verify
every operation appears in the transaction log with correct details, **so that** audit logging
is complete.

## F-13.13.3a Territory Claim System

## US-13.13.3a.1 Monitor Territory Ownership Across the Server

**As a** server admin (P-22), **I want to** view territory ownership on the server map with
guild emblems and boundaries, **so that** I can monitor server-wide territorial balance.

## US-13.13.3a.2 Claim Territory for My Guild

**As a** player (P-23), **I want to** claim territory by constructing a guild hall or capturing
a control point, **so that** my guild gains resource bonuses and world map visibility.

## US-13.13.3a.3 Verify Only One Guild Owns a Territory

**As an** engine tester (P-27), **I want to** attempt simultaneous territory claims by two
guilds and verify only one succeeds, **so that** exclusive ownership is enforced.

## F-13.13.3b Guild War Declaration and PvP Rules

## US-13.13.3b.1 Declare War on a Rival Guild

**As a** player (P-23), **I want to** officers to declare war on other guilds, enabling PvP
between warring guild members in contested zones, **so that** guild rivalries can be resolved
through combat.

## US-13.13.3b.2 Verify War Enables PvP Only Between Warring Guilds

**As an** engine tester (P-27), **I want to** verify that PvP is enabled between members of
warring guilds but non-warring players are unaffected, **so that** war PvP rules are scoped
correctly.

## US-13.13.3b.3 Verify War Status Displays on Nameplates

**As an** engine tester (P-27), **I want to** verify war status is visible on guild nameplates
and in the guild UI, **so that** players can identify warring guild members.

## F-13.13.3c Siege Mechanics

## US-13.13.3c.1 Configure War Windows Per Server

**As a** server admin (P-22), **I want to** configure time-limited war windows per server,
**so that** siege engagement times are predictable for the server's player base.

## US-13.13.3c.2 Attack Enemy Structures During War Windows

**As a** player (P-23), **I want to** attack enemy guild structures with siege weapons during
scheduled war windows, **so that** territory can be contested through organized siege
gameplay.

## US-13.13.3c.3 Repair Structures During and After Sieges

**As a** player (P-23), **I want to** defenders to repair structures during and after sieges,
**so that** structures are not permanently lost from a single attack.

## US-13.13.3c.4 Verify Structure Destruction Revokes Territory

**As an** engine tester (P-27), **I want to** destroy a guild's structures and verify territory
ownership is revoked, **so that** siege victory mechanics work correctly.

## F-13.13.3d Guild Rankings and Leaderboards

## US-13.13.3d.1 Configure Seasonal Resets and Archiving

**As a** server admin (P-22), **I want to** seasonal resets to archive historical rankings,
**so that** past accomplishments are preserved.

## US-13.13.3d.2 View Guild Leaderboards

**As a** player (P-23), **I want to** view server-wide leaderboards ranked by conquest points,
war wins, and territory count, **so that** I can see which guilds dominate.

## US-13.13.3d.3 Verify Leaderboard Updates After War Conclusion

**As an** engine tester (P-27), **I want to** conclude a war and verify leaderboards update
with new conquest points and war wins, **so that** ranking updates are timely and accurate.

## F-13.13.4 Friends List and Social Graph

## US-13.13.4.1 Manage My Friends List

**As a** player (P-23), **I want to** send and accept friend requests, see online/offline status
and current zone/activity, and send direct messages, **so that** I can coordinate play with
friends.

## US-13.13.4.2 Block and Ignore Unwanted Players

**As a** player (P-23), **I want to** block players to hide all their communication and see a
"recently played with" list, **so that** I control my social experience.

## US-13.13.4.3 Import Platform Friends

**As a** player (P-23), **I want to** import platform friends (Steam, PlayStation, Xbox) who
also play the game, **so that** my existing social network is available in-game.

## US-13.13.4.4 Verify Cross-Shard Friend Visibility

**As an** engine tester (P-27), **I want to** add a friend on a different shard and verify
online status and zone display correctly, **so that** cross-shard social features work.

## US-13.13.4.5 Verify Block Hides All Communication

**As an** engine tester (P-27), **I want to** block a player and verify all their messages,
emotes, and social interactions are hidden, **so that** blocking is comprehensive.

## F-13.13.5a Core Mail Send/Receive

## US-13.13.5a.1 Send and Receive Mail

**As a** player (P-23), **I want to** compose mail with recipient, subject, and body text and
receive notification on login and during sessions, **so that** I can communicate
asynchronously with other players.

## US-13.13.5a.2 Manage Mail (Read, Delete, Return)

**As a** player (P-23), **I want to** mark mail as read/unread, delete mail, and have
undeliverable mail returned to sender, **so that** I can manage my mailbox.

## US-13.13.5a.3 Verify Mail Delivery to Offline Recipients

**As an** engine tester (P-27), **I want to** send mail to an offline player, have them log in,
and verify the mail is delivered with notification, **so that** offline mail delivery works.

## US-13.13.5a.4 Verify Mail Expiration and Cleanup

**As an** engine tester (P-27), **I want to** send mail, wait past the expiration period, and
verify it is deleted, **so that** expiration cleanup works correctly.

## F-13.13.5b Mail Attachments

## US-13.13.5b.1 Attach Items and Currency to Mail

**As a** player (P-23), **I want to** attach items and currency to mail, with items held in
escrow until collected, **so that** I can send items to offline players.

## US-13.13.5b.2 Send COD Mail

**As a** player (P-23), **I want to** send cash-on-delivery mail that requires the recipient to
pay before collecting attachments, **so that** I can sell items to specific players remotely.

## US-13.13.5b.3 Verify Item Escrow on Send

**As an** engine tester (P-27), **I want to** attach an item to mail, send it, and verify the
item is removed from the sender's inventory and held in escrow, **so that** items are not
duplicated during mail transit.

## US-13.13.5b.4 Verify COD Payment Before Collection

**As an** engine tester (P-27), **I want to** receive COD mail and verify I cannot collect
attachments without paying the required amount, **so that** COD enforcement works.

## F-13.13.5c System Mail

## US-13.13.5c.1 Monitor System Mail Delivery

**As a** server admin (P-22), **I want to** monitor system mail delivery rates and failures,
**so that** I can ensure automated mail reaches all intended recipients.

## US-13.13.5c.2 Receive Automated System Mail

**As a** player (P-23), **I want to** receive system-generated mail for auction results, guild
invitations, GM notifications, and event rewards, **so that** automated game events deliver
information and rewards reliably.

## US-13.13.5c.3 Verify System Mail Delivers to Offline Players

**As an** engine tester (P-27), **I want to** trigger system mail to an offline player, have
them log in, and verify the mail is present with correct attachments, **so that** system mail
delivery is guaranteed.

## F-13.13.6a Core Chat Infrastructure

## US-13.13.6a.1 Monitor Chat Rate Limiting

**As a** server admin (P-22), **I want to** chat messages to be rate-limited per player to
prevent flooding, **so that** chat remains usable under load.

## US-13.13.6a.2 Chat Across Multiple Channels

**As a** player (P-23), **I want to** send and receive messages across global, zone, trade,
party, guild, whisper, and LFG channels simultaneously, **so that** I can communicate in the
right context.

## US-13.13.6a.3 See Chat History Persist Across Zone Transitions

**As a** player (P-23), **I want to** chat history to be scrollable and persist across zone
transitions within a session, **so that** I do not lose conversation context when moving.

## US-13.13.6a.4 Verify Rate Limiting Blocks Excessive Messages

**As an** engine tester (P-27), **I want to** send messages exceeding the rate limit and verify
excess messages are blocked, **so that** flood protection works.

## US-13.13.6a.5 Verify Chat Survives Zone Transitions

**As an** engine tester (P-27), **I want to** send chat, transition zones, and verify chat
history is preserved, **so that** zone transitions do not clear chat.

## F-13.13.6b Chat Content Features

## US-13.13.6b.1 Review Filtered Messages for Moderation

**As a** server admin (P-22), **I want to** filtered messages to be logged for moderation
review, **so that** flagged content can be investigated.

## US-13.13.6b.2 Link Items in Chat With Hover Tooltips

**As a** player (P-23), **I want to** link items in chat messages with hover-tooltip display
showing stats, rarity, and icon, **so that** trade communication is informative.

## US-13.13.6b.3 See Profanity and Spam Filtered

**As a** player (P-23), **I want to** profanity to be filtered and spam to be detected (repeated
messages, excessive caps), **so that** the chat environment is healthy.

## US-13.13.6b.4 Verify Item Linking Displays Correct Tooltip

**As an** engine tester (P-27), **I want to** link an item in chat and verify the hover tooltip
matches the item's actual stats and appearance, **so that** item links are accurate.

## US-13.13.6b.5 Verify Profanity Filter Replaces Blocked Terms

**As an** engine tester (P-27), **I want to** send a message with blacklisted terms and verify
they are replaced or blocked per configuration, **so that** profanity filtering works.

## F-13.13.6c Custom Player-Created Channels

## US-13.13.6c.1 Create a Custom Chat Channel

**As a** player (P-23), **I want to** create named chat channels with optional password
protection, **so that** my group has a private communication space.

## US-13.13.6c.2 Moderate My Custom Channel

**As a** player (P-23), **I want to** as channel owner, kick, mute, transfer ownership, and
assign moderator roles, **so that** I can manage my channel's membership.

## US-13.13.6c.3 Verify Channel Destruction When Last Member Leaves

**As an** engine tester (P-27), **I want to** have all members leave a custom channel and verify
it is destroyed, **so that** empty channels are cleaned up.

## US-13.13.6c.4 Verify Password Protection Works

**As an** engine tester (P-27), **I want to** create a password-protected channel and verify
players cannot join without the correct password, **so that** channel privacy is enforced.

## F-13.13.7 Emote and Social Action System

## US-13.13.7.1 Perform Character Emotes

**As a** player (P-23), **I want to** trigger emotes (dance, wave, sit, laugh, bow, sleep) via
chat commands or an emote wheel with synchronized animations and audio, **so that** I can
express myself socially.

## US-13.13.7.2 Perform Paired Emotes With Other Players

**As a** player (P-23), **I want to** initiate paired emotes (handshake, high-five) that
synchronize between two characters when the target accepts, **so that** social interactions
feel collaborative.

## US-13.13.7.3 Verify Emote Animations Play Correctly

**As an** engine tester (P-27), **I want to** trigger each emote and verify the correct
animation (looping or one-shot) plays through the animation state machine, **so that** emote
animations are correct.

## US-13.13.7.4 Verify Paired Emote Synchronization

**As an** engine tester (P-27), **I want to** initiate a paired emote and verify both characters
synchronize within 1 frame, **so that** paired emotes are visually synchronized.

## F-13.13.8 Player Inspection

## US-13.13.8.1 Inspect Another Player's Character

**As a** player (P-23), **I want to** inspect another player's equipped gear, stats, talent
build, achievements, guild, and title within visual proximity, **so that** I can evaluate
group composition and admire equipment.

## US-13.13.8.2 Set Inspection Privacy

**As a** player (P-23), **I want to** set inspection privacy (public, friends-only, guild-only,
private), **so that** I control who can view my character details.

## US-13.13.8.3 Verify Privacy Settings Block Unauthorized Inspection

**As an** engine tester (P-27), **I want to** set privacy to friends-only and have a non-friend
attempt inspection, verifying it is blocked, **so that** privacy settings are enforced.

## US-13.13.8.4 Verify Inspection Displays Correct Data

**As an** engine tester (P-27), **I want to** inspect a player and verify all displayed data
(gear, stats, talents) matches the target's actual character state, **so that** inspection
data is accurate.

## F-13.13.9 Dungeon and Group Finder

## US-13.13.9.1 Monitor Cross-Shard Queue Health

**As a** server admin (P-22), **I want to** monitor cross-shard queue population, wait times,
and match rates, **so that** I can identify matchmaking issues.

## US-13.13.9.2 Queue for Dungeons by Role

**As a** player (P-23), **I want to** queue for dungeon content by my role (tank, healer, DPS,
support) and be automatically matched with a balanced group, **so that** I can participate in
group content without manually assembling a party.

## US-13.13.9.3 See Estimated Wait Time Per Role

**As a** player (P-23), **I want to** the group finder to display estimated wait time per role,
**so that** I can choose roles with shorter queues.

## US-13.13.9.4 Receive Bonus Rewards for In-Demand Roles

**As a** player (P-23), **I want to** in-demand roles to receive bonus rewards (extra currency,
loot chance), **so that** role shortages are incentivized.

## US-13.13.9.5 Verify Deserter Penalty Blocks Re-Queue

**As an** engine tester (P-27), **I want to** leave an instance early and verify the deserter
penalty blocks re-queuing for the configured duration, **so that** deserter penalties are
enforced.

## US-13.13.9.6 Verify Cross-Shard Matchmaking

**As an** engine tester (P-27), **I want to** queue players from different shards and verify
they are matched together and teleported to the instance, **so that** cross-shard pooling
works.

## F-13.13.10a Arena System

## US-13.13.10a.1 Monitor Arena Matchmaking Quality

**As a** server admin (P-22), **I want to** monitor arena matchmaking quality (rating spread,
wait times, queue population per bracket), **so that** competitive matchmaking is healthy.

## US-13.13.10a.2 Queue for Rated Arena Matches

**As a** player (P-23), **I want to** queue for rated 2v2, 3v3, or 5v5 arena matches from
anywhere in the world, **so that** I can engage in competitive deathmatch PvP.

## US-13.13.10a.3 See My Arena Rating Per Bracket

**As a** player (P-23), **I want to** my arena rating to be tracked per bracket, **so that** my
competitive standing is visible and determines matchmaking.

## US-13.13.10a.4 Verify Arena Rating Updates After Match

**As an** engine tester (P-27), **I want to** complete an arena match and verify both teams'
ratings update correctly based on the match result, **so that** the rating system is accurate.

## F-13.13.10b Battleground System

## US-13.13.10b.1 Play Objective-Based Battlegrounds

**As a** player (P-23), **I want to** queue for battlegrounds with varied objectives (capture
the flag, point control, payload) with balanced teams, **so that** I can enjoy large-scale
objective PvP.

## US-13.13.10b.2 See Individual Performance Metrics

**As a** player (P-23), **I want to** individual performance metrics (kills, objective
contributions, healing) tracked on the scoreboard, **so that** I can measure my impact.

## US-13.13.10b.3 Verify Team Composition Balance

**As an** engine tester (P-27), **I want to** verify matchmaking produces balanced team sizes
across 100 matches, **so that** team composition is fair.

## US-13.13.10b.4 Verify Objective Scoring Tracks Correctly

**As an** engine tester (P-27), **I want to** capture flags, hold points, and push payloads and
verify score updates correctly per the configured rules, **so that** objective scoring is
accurate.

## F-13.13.10c PvP Rating and Seasonal Rewards

## US-13.13.10c.1 Execute Seasonal Resets

**As a** server admin (P-22), **I want to** seasonal resets to archive ratings and distribute
rewards correctly, **so that** season transitions are smooth.

## US-13.13.10c.2 Track My Seasonal PvP Rating

**As a** player (P-23), **I want to** my PvP rating tracked on seasonal leaderboards with
placement matches determining initial rating, **so that** I have recurring competitive goals.

## US-13.13.10c.3 Earn Exclusive Season-End Rewards

**As a** player (P-23), **I want to** top-ranked players at season end to receive exclusive
titles, cosmetics, and mounts, **so that** competitive achievement has visible prestige.

## US-13.13.10c.4 Verify Placement Matches Set Initial Rating

**As an** engine tester (P-27), **I want to** complete placement matches and verify the initial
rating is set according to match results, **so that** placement calibration works.

## US-13.13.10c.5 Verify Season-End Reward Distribution

**As an** engine tester (P-27), **I want to** trigger a season end and verify rewards are
distributed to the correct players based on final rankings, **so that** reward distribution
is accurate.

## F-13.13.10d PvP Stat Normalization

## US-13.13.10d.1 Compete With Normalized Stats

**As a** player (P-23), **I want to** optional stat normalization in rated matches to override
gear stats with bracket-appropriate templates, **so that** skill determines outcomes rather
than gear.

## US-13.13.10d.2 Verify Normalization Overrides Gear Stats

**As an** engine tester (P-27), **I want to** enable normalization and verify player stats match
the bracket template regardless of equipped gear, **so that** stat normalization is applied
correctly.

## US-13.13.10d.3 Verify Cosmetic Appearance Is Unaffected

**As an** engine tester (P-27), **I want to** enable normalization and verify equipped gear
appearance is unchanged (only stats are overridden), **so that** players keep their visual
identity in normalized matches.
