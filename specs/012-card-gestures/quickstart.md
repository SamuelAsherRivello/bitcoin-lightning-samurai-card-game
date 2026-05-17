# Quickstart: Card Gestures

## Prerequisites

| Step | Command |
| ---- | ------- |
| Verify local dependencies once per machine | `scripts/main/InstallDependencies.ps1` |
| Run automated tests | `scripts/other/RunTests.ps1` |
| Run desktop app | `scripts/main/RunAppDesktop.ps1` |
| Stop app/build processes | `scripts/other/StopApp.ps1` |

## Implementation Checkpoints

| Checkpoint | Expected Result |
| ---------- | --------------- |
| Gesture model tests | Click/tap, threshold-crossing drag, selected-card dismissal, concurrent drag guards, and invalid transitions resolve deterministically |
| Slot model tests | Three locations expose four opponent slots and four local-player slots each; runtime slot rectangles match debug-drawn reference lines; only locations with an empty local slot accept direct placement and assign slots in upper-left, upper-right, lower-left, lower-right order |
| Card state tests | Cards move through `Hand`, `Dragging`, `LocationCurrentRound`, and `LocationLocked`; only hand cards and current-round placed cards can start allowed drags |
| Location power tests | Location-side PowerPointViews start at 0 and update to the sum of card powers assigned to that side's slots |
| Navigation regression tests | Clicking a hand card in `GameScene` no longer opens `DeckScene`; the existing Deck view implementation is not modified |
| Animation integration | Hand-to-selected, selected-to-hand, drag-to-slot, and invalid-drop return movements animate smoothly |
| Aspect-ratio checks | Hand, selected inspection, drag preview, and placed slots derive from the safe game view and reflow on viewport changes |

## Desktop Verification

1. Run `scripts/other/RunTests.ps1`.
2. Run `scripts/main/RunAppDesktop.ps1`.
3. In `GameScene`, click or tap a local hand card and confirm the Deck view does not open.
4. Confirm the selected card animates to the center inspection position at approximately 90% safe visible height.
5. Click or tap the selected card and confirm it returns to its hand source.
6. Press a hand card, move beyond the drag threshold, release over each empty local-player slot, and confirm the card snaps into the destination location card slot.
7. Confirm that drag begin grows the card to 150% of its hand size over roughly 0.25 seconds and that the pointer keeps the same offset from the card center while dragging.
8. Confirm light blue DropTargetHint rectangles appear over available local-player slot areas while dragging and disappear for full locations.
9. While one card is pressed or dragged, press another hand card and confirm the original card remains the only active draggable card.
10. Try to drag a card already in `Dragging` or `Location` state and confirm it cannot start another drag.
11. During the same round as placement, drag a placed card back to the hand area and confirm existing hand cards shift on the x axis to show a candidate insertion gap.
12. Release the current-round placed card over a hand insertion gap and confirm it returns to hand at the chosen order and the hand group recenters.
13. End the round, then try to drag a prior-round placed card and confirm it cannot start a drag.
14. Toggle the debug drawing lines and confirm the slot rectangles used by placement match the debug-drawn slot instances.
15. Place one or more cards into a location and confirm the local side's visible power points move from 0 to the sum of the placed cards' power values.
16. Drag over an opponent slot, a full local location, and empty board space; confirm each invalid drop returns the card to its original hand card slot.
17. Resize the window and confirm selected, dragged, placed cards, DropTargetHint rectangles, insertion gaps, and location power points keep their aspect-ratio-safe layout.
18. Confirm there is no user-facing GameScene gesture path to reach the Deck view.
19. Run `scripts/other/StopApp.ps1` when finished.

## Browser WebGPU Verification

1. Run the existing web workflow documented by the repository, typically `scripts/other/RunAppWeb.ps1` if present.
2. Open the served local browser target.
3. Repeat the click/tap inspection, drag threshold, valid placement, invalid placement, and resize checks from desktop verification.
4. If browser WebGPU or touch-style pointer verification cannot be completed, record the exact blocker before considering the feature complete.

## Notes For Task Generation

| Topic | Guidance |
| ----- | -------- |
| First tasks | Add model tests for gesture and slot rules before visible animation work where practical |
| Dependency task | Add `bevy_tweening = "0.15"` to `bevy/crates/game/Cargo.toml` and wire the plugin once |
| Template reference | Use `bevy/crates/template-crate` as the proper reference for Bevy crate folders, representative files, asset folders, and Rust coding standards |
| File scope | Prefer focused gesture model/component/system files over expanding the aggregate runtime system module |
| Naming | Use `Model` for gesture/slot state and `View` for visual card presentation states |
| Purpose comments | Add required `HUMAN:` and `AI:` lines above new or changed primary runtime items |
| Exclusions | Do not add CPU placement, reveal rules, scoring resolution, or production mobile packaging; round-aware placed-card mobility is governed by the gameplay round spec |
| Deck boundary | Do not modify the existing Deck view implementation; only remove user reachability from GameScene gestures |

## Implementation Verification Notes

| Check | Result |
| ----- | ------ |
| `cargo test -p samurai-card-game --no-default-features` | Passed: 124 game lib tests, 1 game binary test, and doc tests after location power point iteration. |
| `scripts/other/RunTests.ps1 -NoFastDevFeature` | Passed: game workspace tests, shared workspace tests, and doc tests. |
| `scripts/other/RunAppDesktop.ps1 -CheckOnly -NoFastDevFeature` | Passed desktop `cargo check` with `asset-hot-reload`. |
| `scripts/other/RunAppWeb.ps1 -CheckOnly` | Passed browser WebGPU target `cargo check` for `wasm32-unknown-unknown`. |
| `scripts/other/RunTests.ps1` | Blocked by rustc process exit `0xffffffff` during the `fast-dev` workspace build without a Rust diagnostic; rerunning without `fast-dev` passed. |
| Manual desktop gesture walkthrough | Not performed in this pass; covered by model/system regression tests and desktop check-only build. |
| Manual browser gesture walkthrough | Not performed in this pass; covered by model/system regression tests and browser check-only build. |
| Deck boundary | Confirmed in tests: GameScene hand-card click remains in GameScene and does not spawn DeckScene entities. |
