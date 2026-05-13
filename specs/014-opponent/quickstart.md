# Quickstart: Opponent Modes and Two-Player Resolution

## Prerequisites

| Need | Command |
| ---- | ------- |
| Rust/project dependencies | `scripts/main/InstallDependencies.ps1` |
| Full test pass | `scripts/other/RunTests.ps1` |
| Desktop runtime | `scripts/main/RunAppDesktop.ps1` |
| Stop runtime | `scripts/other/StopApp.ps1` |

## Verification Flow

1. Run `scripts/other/RunTests.ps1`.
2. Launch the game with `scripts/main/RunAppDesktop.ps1`.
3. In GameScene, confirm the lower-left control stack reserves Status above Mode above Restart above Undo.
4. With no saved mode preference, confirm the mode button displays `Mode:` plus `Human versus CPU`.
5. Activate the mode button and confirm it cycles to `CPU versus CPU`, saves that mode, and starts a fresh game.
6. Restart the game and confirm the mode button loads `CPU versus CPU`.
7. Activate the mode button again and confirm it returns to `Human versus CPU` and saves that mode.
8. Start or restart in `Human versus CPU`.
9. Confirm the near player uses bottom hand and bottom slots, and the far CPU uses top slots.
10. Press the human Next/End Round control before the CPU is ready and confirm the round does not advance until the CPU is also ready.
11. While the round is active, confirm the near player's hand and own current-round placements appear as card fronts to the human.
12. Confirm the far CPU hand and far current-round placements appear face down to the human.
13. End the round and confirm all current-round placed cards flip face up immediately and remain face up in later rounds.
14. Move the mouse cursor over CPU-owned hand and placed cards and confirm those cards do not hover, show drag affordances, or rotate toward the cursor.
15. Observe CPU card moves and readiness decisions: CPU Brain details are not shown, dealt CPU hand cards come to rest before planning, at least 0.5 seconds passes after hand settle before any CPU card moves to a location, card moves are legal and win-oriented, and each CPU move or decision is paced by 0.5 to 1 second.
16. Play through round `6/6` and confirm the result identifies exactly one winner with `Status:` text above Mode, such as `Status: Winner is Player 1 (CPU)`.
17. Switch to `CPU versus CPU`, restart, provide no gameplay input, and confirm both players progress automatically through round `6/6` to final winner status.

## Expected Test Coverage

| Area | Expected Coverage |
| ---- | ----------------- |
| Mode cycling | Unit/model tests for two-mode cycle, default mode, save/load behavior, and reset trigger. |
| CPU Brain Level 1 | Unit tests for legal move selection, win-oriented move preference, seeded random choice among acceptable moves, deterministic same-seed move sequences, hand-settle plus 0.5 second planning gate, 0.5 to 1 second move/decision pacing, no-energy stop, no-legal-move stop, and hidden terminology. |
| CPU Brain Knowledge | Unit tests that Brain can see own hand/open locations/revealed slots but cannot see unrevealed deck order or opposing current-round hidden placements. |
| Card Visibility | Unit and system tests for face-down opposing hand/current-round placements and end-of-round reveal. |
| Card Cursor Interaction | System tests that CPU-owned cards do not hover, show drag affordance, or rotate toward the mouse cursor. |
| Readiness | Unit tests that one ready player does not advance and two ready players do. |
| Slot side legality | Unit tests for near/bottom and far/top placement ownership. |
| Winner resolution | Unit tests proving no final draw and left-to-right location evaluation. |
| GameScene controls | System/UI tests for Status text, Mode label, Restart reset scope, and Next readiness behavior. |
| CPU versus CPU autoplay | System tests that a fresh CPU-vs-CPU game reaches final winner status after round 6 without human gameplay input. |

## Browser/Desktop Notes

| Target | Verification |
| ------ | ------------ |
| Windows desktop | Required after implementation through the normal desktop run script. |
| Browser WebGPU | Required when practical; if blocked, document exact build/runtime blocker in implementation notes. |
