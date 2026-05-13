# Contract: GameScene Opponent UI

## Status Text

| Property | Contract |
| -------- | -------- |
| Position | Lower-left control stack, above Mode. |
| Default | Hidden or neutral before a final winner exists. |
| Final result | Displays `Status:` plus the winning player number and controller type, such as `Status: Winner is Player 1 (CPU)`. |
| Mode coverage | Required in both `Human versus CPU` and `CPU versus CPU`. |
| Hidden details | Must not display `CPU Brain` or `CpuBrainLevel`. |

## Mode Button

| Property | Contract |
| -------- | -------- |
| Position | Lower-left control stack, above Restart. |
| Style | Same visual family as Restart. |
| Line 1 | `Mode:` |
| Line 2 values | `Human versus CPU` or `CPU versus CPU` only. |
| Activation | Cycles to the other mode, saves the selected mode to disk, and starts a fresh game at round `1/6`. |
| Default | If no saved selected mode exists, starts in `Human versus CPU`. |
| Startup load | If a saved selected mode exists, starts with that saved mode. |
| Hidden details | Must not display `CPU Brain` or `CpuBrainLevel`. |

## Restart Button

| Property | Contract |
| -------- | -------- |
| Availability | Visible in `Human versus CPU`; may remain visible in `CPU versus CPU` unless implementation intentionally disables human controls there. |
| Activation | Starts a fresh game in the current mode at round `1/6`. |
| Reset scope | Clears both players' transient decks, hands, placements, readiness, CPU Brain pending actions, round state, and winner state. |

## Next / End Round Control

| Property | Contract |
| -------- | -------- |
| Human control | Lets the near human player mark readiness. |
| Timing | Human player has no timer. |
| Round advancement | Does not advance the round until both players are ready. |
| Label | May retain existing End Round wording or use Next, but behavior must be clear. |

## CPU Brain Presentation

| Property | Contract |
| -------- | -------- |
| Visibility | CPU Brain internals are hidden. |
| Card moves | Visible CPU card placements occur one card at a time. |
| Hand settle gate | CPU cards dealt into a rendered hand must come to rest before the CPU can plan or mark readiness. |
| Post-settle pause | After rendered CPU hand cards settle, at least 0.5 seconds pass before a CPU card can move from hand to location. |
| Timing | Every visible CPU move or non-move readiness decision is paced by a 0.5 to 1 second delay after the hand settle gate. |
| Ready signal | CPU readiness is not exposed as a clickable human button. |
| Knowledge | CPU Brain cannot inspect opposing current-round hidden card identities or unrevealed deck order. |

## CPU Versus CPU Autoplay

| Property | Contract |
| -------- | -------- |
| Human input | No gameplay input is required after a fresh `CPU versus CPU` game starts. |
| Progression | Both CPU players continue paced moves or readiness decisions through rounds `1/6` to `6/6`. |
| Completion | The mode reaches final winner status automatically after round 6. |

## Board Slots

| Slot Side | Contract |
| --------- | -------- |
| Near player | Uses bottom slots at each location. |
| Far player | Uses top slots at each location. |
| Human interaction | Near human can drag only near-player cards into bottom slots. |
| CPU interaction | CPU Brain can place only that CPU player's cards into that player's side slots. |

## Card Visibility

| Card State | Human Player View In Human versus CPU |
| ---------- | -------------------------------------- |
| Near player hand | Card fronts. |
| Far CPU hand | Face-down cards. |
| Near current-round placed card | Card front for the near human, hidden from CPU Brain. |
| Far current-round placed card | Face down until round end. |
| Any prior-round placed card | Face up permanently. |
| End-of-round reveal | All current-round placed cards flip face up immediately after both players mark Next. |

## Card Cursor Interaction

| Card Owner / State | Contract |
| ------------------ | -------- |
| CPU-owned cards | Do not respond to mouse hover, do not show drag affordance, and do not rotate toward the mouse cursor. |
| Local human-owned interactable cards | May keep existing cursor hover, drag, and rotation behavior. |
| Human-owned non-interactable cards | Must not become draggable merely because the cursor is nearby. |

## Winner Result

| Property | Contract |
| -------- | -------- |
| Trigger | After both players are ready on round `6/6`. |
| Evaluation order | Left location, middle location, right location. |
| Result | Identifies near or far player as winner. |
| Status text | Updates the lower-left status text above Mode with the winning player number and controller type. |
| Tie display | No tied match result is shown. |
