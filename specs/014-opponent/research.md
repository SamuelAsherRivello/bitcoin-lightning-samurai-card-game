# Research: Opponent Modes and Two-Player Resolution

## Decision: Use CPU Brain terminology for CPU player controller logic

**Rationale**: The project already has an AI runtime concept for external inspection/debug workflows, so CPU gameplay logic should not be called AI. `CPU Brain` clearly means authored Rust game logic that controls a CPU player and does not use a runtime generative AI model.

**Alternatives considered**:

| Alternative | Reason Rejected |
| ----------- | --------------- |
| AI player | Ambiguous with generative AI and existing AI runtime naming. |
| Bot | Less aligned with the user's requested vocabulary. |
| CPU behavior | Too generic and does not create a durable concept for future levels. |

## Decision: Keep CPU Brain hidden from user-facing mode labels

**Rationale**: The user-facing modes are exactly `Human versus CPU` and `CPU versus CPU`. CPU Brain is an implementation detail of each CPU player and should not appear as a mode, difficulty selector, or result label in this feature.

**Alternatives considered**:

| Alternative | Reason Rejected |
| ----------- | --------------- |
| Show `Human versus CPU Brain` | Violates the requested visible labels. |
| Add a Brain level selector | Out of scope; only level 1 exists now. |

## Decision: Support only `CpuBrainLevel = 1`

**Rationale**: Level 1 gives the model a future extension point without implying advanced strategy now. Its goal is to win the game. It should choose legal affordable drag-equivalent moves that appear likely to improve victory chances when it can evaluate them, use seeded randomness when multiple acceptable moves are available, wait 0.5 to 1 second between moves or readiness decisions, and stop when no energy remains or no legal affordable move is available.

**Alternatives considered**:

| Alternative | Reason Rejected |
| ----------- | --------------- |
| No level model | Would make future smarter brain variants harder to introduce cleanly. |
| Multiple levels now | Not requested and increases scope without gameplay value. |

## Decision: Seed CPU Brain randomness for deterministic tests

**Rationale**: CPU Brain should include some randomness to make play less predictable and more interesting for the human, but automated tests need repeatable results. Supplying a seed lets tests assert exact move sequences while runtime play can still vary.

**Alternatives considered**:

| Alternative | Reason Rejected |
| ----------- | --------------- |
| Pure randomness without seed | Makes tests flaky and hard to reproduce. |
| Fully deterministic Brain | Reduces variety in human play. |
| Runtime generative model | Out of scope and conflicts with authored game-code Brain. |

## Decision: Route player choices through controllers

**Rationale**: Each player has one controller. `PlayerController` dispatches mouse, keyboard, and tap choices; `CpuController` uses CPU Brain to dispatch choices. Both controllers should call into shared game logic so legal move, energy, slot, reveal, and readiness rules stay consistent.

**Alternatives considered**:

| Alternative | Reason Rejected |
| ----------- | --------------- |
| Separate CPU-only rules | Risks divergent behavior from human placement rules. |
| Treat Brain as player state | Blurs controller responsibility and makes future controller types harder to add. |

## Decision: Limit CPU Brain knowledge to visible game information plus own hand

**Rationale**: CPU Brain should behave like a player controller with imperfect information. It can inspect its own hand, open locations and abilities, and revealed slots on both sides. It cannot see upcoming deck cards or opposing current-turn hidden placements.

**Alternatives considered**:

| Alternative | Reason Rejected |
| ----------- | --------------- |
| Perfect-information CPU | Conflicts with hidden card gameplay and would make CPU choices unfair. |
| No location knowledge | Too weak for even simple legal/random choices once location abilities matter. |

## Decision: Current-turn placements reveal at end of turn

**Rationale**: Cards placed during the current turn are private to their owner and face down to the opponent until both players mark Next. At turn end, all current-turn placements reveal immediately and stay face up permanently, creating information for later turns.

**Alternatives considered**:

| Alternative | Reason Rejected |
| ----------- | --------------- |
| Cards reveal immediately on placement | Removes the hidden-information behavior requested for two-player play. |
| Cards stay face down across turns | Deferred to future specs; current feature reveals all current-turn placements at turn end. |

## Decision: CPU never uses Undo

**Rationale**: Undo represents a human reconsideration action. CPU Brain dispatches legal choices through `CpuController` and treats those choices as final for the turn. This keeps CPU logic simpler and avoids modeling indecision.

**Alternatives considered**:

| Alternative | Reason Rejected |
| ----------- | --------------- |
| Allow CPU undo for stronger play | Not requested and conflicts with the current CPU Brain Level 1 simplicity. |
| Share the human Undo affordance with CPU | Undo is a human-facing reconsideration tool, not part of CPU choice dispatch. |

## Decision: Readiness gates round advancement

**Rationale**: Both human and CPU players mark Next. A round ends only when both readiness flags are set. The human player has no timer and may take as long as desired.

**Alternatives considered**:

| Alternative | Reason Rejected |
| ----------- | --------------- |
| Timer-based rounds | Explicitly not wanted for the human player now. |
| CPU instantly ends turn | Fails the desired believable opponent pacing. |
| Existing End Turn immediately advances round | No longer valid once two players exist. |

## Decision: Reuse existing top/bottom slot side model

**Rationale**: `CardSlotSide::LocalPlayer` and `CardSlotSide::Opponent` already represent bottom and top slot sides. Extending placement legality to both sides keeps this feature aligned with current board layout and tests.

**Alternatives considered**:

| Alternative | Reason Rejected |
| ----------- | --------------- |
| Create separate far-player board model | Duplicates current slot-side model. |
| Treat CPU cards as visual-only | Would prevent real scoring and winner resolution. |

## Decision: Resolve final draws deterministically

**Rationale**: The spec requires no tied match result. Existing scoring can produce `Draw`, so implementation must add a deterministic final tiebreaker after location wins and total power if needed.

**Alternatives considered**:

| Alternative | Reason Rejected |
| ----------- | --------------- |
| Preserve draw outcome | Contradicts the feature requirement. |
| Ask user every tie | Not practical for automated CPU and finished-match flow. |

## Decision: Persist only selected match mode

**Rationale**: Active match state remains transient, but the last selected match mode is a user preference and should survive restart. When no saved mode exists, the default is `Human versus CPU`. The game already has a player deck concept; for this feature that 12-card deck acts as the master deck and each player receives an independent copy at fresh game start. Different decks per player are deferred.

**Alternatives considered**:

| Alternative | Reason Rejected |
| ----------- | --------------- |
| Persist current match state | Not requested and adds recovery/versioning scope. |
| Persist CPU Brain level | Only one level exists, so persistence adds no user value. |
| Add different decks per player now | Future goal, but out of scope for the first opponent implementation. |
| Do not persist match mode | Contradicts the requested last-selected-mode startup behavior. |

## Decision: `CPU versus CPU` autoplays to final status

**Rationale**: `CPU versus CPU` exists to validate that the game is modeled as two real players with controllers. Once selected, both CPU controllers should keep making paced choices and readiness decisions until round 6 resolves, without requiring hidden human input to advance.

**Alternatives considered**:

| Alternative | Reason Rejected |
| ----------- | --------------- |
| Require the human to press Next between CPU turns | Makes CPU-vs-CPU a manual test harness instead of an autoplay mode. |
| Resolve CPU-vs-CPU instantly | Conflicts with the requirement that CPU players feel human-like through visible delays. |

## Decision: Show final winner through Status text above Mode

**Rationale**: The lower-left control stack already owns mode and restart controls. A `Status:` text directly above Mode gives a stable place to report final outcome in either mode without exposing CPU Brain implementation details.

**Alternatives considered**:

| Alternative | Reason Rejected |
| ----------- | --------------- |
| Show winner only in transient animation | Harder to inspect and test after autoplay completes. |
| Include CPU Brain level in result text | CPU Brain is a hidden implementation detail. |
