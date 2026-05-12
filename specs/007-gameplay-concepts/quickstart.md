# Quickstart: Gameplay Concepts

## Prerequisites

| Requirement | Command |
| ----------- | ------- |
| Rust and project dependencies | `scripts/main/InstallDependencies.ps1` |
| Workspace tests | `scripts/other/RunTests.ps1` |

## Verification Flow

1. Run the focused tests while iterating:

```powershell
cargo test -p bevy-card-game
```

2. Run the repository test script before handoff:

```powershell
scripts/other/RunTests.ps1
```

3. Launch the desktop runtime:

```powershell
scripts/main/RunAppDesktop.ps1
```

4. Verify GameView startup:

| Check | Expected Result |
| ----- | --------------- |
| Active view | GameView is active under persistent AppScene |
| Lower right | End Turn remains visible |
| Lower left | Restart appears above Undo |
| Undo label | Shows `Energy 1/1` and `Undo` on separate lines |
| Undo state | Disabled or greyed out before any current-round placement |
| Initial deal | One card from the near player's deck animates from below screen center into the hand |
| Left location | Open with title `Fortress Gate` and body `+2 Energy to each card here` |
| Middle location | Closed with title `Closed Until Round 2` and no body text |
| Right location | Closed with title `Closed Until Round 3` and no body text |

5. Verify round progression:

| Round | Requested Deal | Expected Deal From Initial Deck | Expected Energy | Expected Location State |
| ----- | -------------- | ------------------------------- | --------------- | ----------------------- |
| 1 | 1 | 1 card from remaining deck order | 1/1 | Left open, middle closed until round 2, right closed until round 3 |
| 2 | 2 | 2 cards from remaining deck order | 2/2 | Left and middle open, right closed until round 3 |
| 3 | 3 | 3 cards from remaining deck order | 3/3 | All three locations open |
| 4 | 1 | 1 card from remaining deck order | 4/4 | All three locations open |
| 5 | 1 | 1 card from remaining deck order | 5/5 | All three locations open |
| 6 | 1 | 1 card from remaining deck order | 6/6 | All three locations open |

6. Verify placement and undo:

| Action | Expected Result |
| ------ | --------------- |
| Move affordable card to a location | Energy decreases by the card's energy cost |
| Move a card to open Fortress Gate | Card's effective energy immediately increases by 2 while placed there |
| Move a card to open Bamboo Crossing | Card's effective energy immediately decreases by 2 while placed there |
| Move a card to open Normal location | Card's effective energy is unchanged and body shows `(No Ability)` |
| Press Undo in same round | Only cards placed this round return to hand; energy is restored |
| Press Undo after a location effect | The returned card no longer has that location's effective-energy modifier |
| End Turn after placement | Undo history clears and prior-round placed cards remain placed |
| Try over-cost placement | Card stays in hand and energy is unchanged |

7. Verify Restart:

| Action | Expected Result |
| ------ | --------------- |
| Press Restart at any time | GameView returns to round `1/6`, new 12-card deck, empty current-round history, and clean hand/location state |
| Press Restart during animation | Final state is a clean fresh game |

## Browser WebGPU Verification

Run the project's browser workflow when available, then repeat the startup, round progression, placement, undo, and restart checks in browser WebGPU. If browser verification cannot run, record the exact blocker in the implementation notes before completion.

## AI Runtime Peek

If the app is running with the AI runtime endpoint, capture a screenshot through the Bevy Remote Protocol workflow and inspect:

| Area | Expected Result |
| ---- | --------------- |
| Lower-left controls | Restart above Undo, inside safe area |
| Lower-right control | End Turn inside safe area |
| Hand | Centered group after deals, undo, and restart |
| Locations | Correct open/closed title/body text, current-round placements removed by Undo only, and open-location effective-energy effects applied only while placed |
