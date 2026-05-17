# Quickstart: DeckScreen Meta Game UI

## Goal

Implement and verify the DeckScreen states defined by this feature only.

| State | Expected Result |
| ----- | --------------- |
| Top navigation | `Play Game`, `My Decks`, `Settings`, and `Debug`; `My Decks` selected. |
| Deck selection | `New Deck` and deck tiles visible. |
| Selected deck editor | `Deck 01` and `Not In Deck` columns visible with Library selected. |
| Shop editor | Shop tab selected with an empty-state presentation. |
| Fullscreen card overlay | Lower editor dimmed, large card preview visible, action rail shown, lower input blocked. |

## Implementation Notes

| Area | Guidance |
| ---- | -------- |
| Top navigation | Build reusable model/view/component concepts; mount only on DeckScreen in this feature. |
| Screen naming | Use user-facing `DeckScreen`; keep runtime child scene as `DeckScene`. |
| State | Keep DeckScreen UI state in a dedicated model/resource. |
| Persistence | Use existing player deck collection persistence; do not mutate active gameplay copies. |
| Modal | Use a DeckScreen-specific modal model and input capture path. |
| Layout | Keep all controls inside the aspect-ratio-safe 16:10 area. |
| Shop | Show an empty shop state; shop cards and purchase settlement come later. |

## Verification Workflow

| Step | Command or Action | Expected Result |
| ---- | ----------------- | --------------- |
| Targeted tests | `scripts/other/RunTests.ps1` | DeckScreen and top-nav model/system tests pass with existing runtime tests. |
| Desktop visual | `scripts/main/RunAppDesktop.ps1` | DeckScreen states match mockup structure and stay inside safe area. |
| AI runtime peek | BRP screenshot workflow when app is started with AI runtime | Runtime reports `DeckScreen` active and screenshot shows expected state. |
| Browser visual | Existing served-web workflow when available | Layout remains safe-area bounded and behavior matches desktop. |

## Deck View Constraint

| Requirement | Detail |
| ----------- | ------ |
| DeckViewBundle | Implementation MUST create a DeckViewBundle that renders a deck tile using the existing card back asset and the deck name only. |

## Final Verification Notes

| Check | Result |
| ----- | ------ |
| Workspace tests | `scripts/other/RunTests.ps1` passed with `cargo test --workspace --features fast-dev`. |
| DeckScreen model tests | Covered full-deck empty Library, move-to-library/move-to-deck partitioning, and disabled Transfer Out. |
| DeckScreen system tests | Covered `Deck 01` tile rendering, editor entry, modal action enablement, and deck/library movement. |
| Scene cycling | Existing S-key test passes for `GameScene -> DeckScene -> DebugScene -> GameScene`. |
| Runtime visual screenshot | Not captured in this pass; behavior is covered by automated ECS/UI tests. |
