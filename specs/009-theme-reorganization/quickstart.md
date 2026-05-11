# Quickstart: Theme Reorganization

## Prerequisites

| Requirement | Command Or Location |
| ----------- | ------------------- |
| Rust and project dependencies | `scripts/main/InstallDependencies.ps1` |
| Existing tests | `scripts/other/RunTests.ps1` |
| Desktop check | `scripts/other/RunAppDesktop.ps1 -CheckOnly` |
| Browser check | `scripts/other/RunAppWeb.ps1 -CheckOnly` |

## Implementation Checkpoints

| Checkpoint | Expected Result |
| ---------- | --------------- |
| Inspect `bevy/crates/game/assets/themes/theme_japan` | Contains `cards`, `locations`, and `worlds`. |
| Inspect card folders | Four card folders start with `card_` and none includes `japan`. |
| Inspect location folders | Six location folders start with `location_` and none includes `japan`. |
| Inspect world folders | Two world folders start with `world_` and none includes `japan`. |
| Search docs for scene terminology | Persistent app container is described as `AppScene`; active sub-screen presentations are described as `GameView` and `CardBrowserView`. |
| Search docs for card terminology | Card data is described as `CardModel`, rendered card presentation as `CardView`, and the visual bundle as `CardViewBundle`. |
| Search runtime asset paths | Current card, location, and world paths point below `themes/theme_japan`. |

## Behavior Smoke

| Workflow | Expected Result |
| -------- | --------------- |
| Launch `GameView` | Existing world background, three tactical locations, and four bottom-row cards appear on top of `AppScene`. |
| Press `T` in `GameView` | World alternates between Bamboo Forest and Coastal Harbor; visible tactical locations remain present. |
| Click each bottom-row card | `CardBrowserView` opens focused on the clicked card. |
| Flip card in `CardBrowserView` | Front/back flip behavior remains visible and temporary. |
| Press `T` in `CardBrowserView` | CardUI settings still change globally without changing `GameView` world state. |

## Verification Commands

```powershell
scripts/other/RunTests.ps1
scripts/other/RunAppDesktop.ps1 -CheckOnly
scripts/other/RunAppWeb.ps1 -CheckOnly
```
