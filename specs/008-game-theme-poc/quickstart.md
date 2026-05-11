# Quickstart: Game Theme POC

## Prerequisites

| Step | Command |
| ---- | ------- |
| Verify dependencies once per machine | `scripts/main/InstallDependencies.ps1` |
| Stop existing local app processes if needed | `scripts/other/StopApp.ps1` |

## Verification Flow

| Goal | Command |
| ---- | ------- |
| Run unit/system tests | `scripts/other/RunTests.ps1` |
| Check desktop build | `scripts/other/RunAppDesktop.ps1 -CheckOnly` |
| Run desktop app | `scripts/other/RunAppDesktop.ps1` |
| Check web build | `scripts/other/RunAppWeb.ps1 -CheckOnly` |
| Run web app | `scripts/other/RunAppWeb.ps1` |

## Manual Smoke Scenario

| Step | Expected Result |
| ---- | --------------- |
| Open the game scene | The pre-008 desert background and old card lineup are gone; a Japan Realism world and four bottom cards are visible. |
| Press `T` in GameScene | The world alternates between Bamboo Forest and Coastal Harbor and three locations re-render. |
| Move cursor over/around cards | Cards tilt subtly while names and silhouettes remain readable. |
| Click Kage Ren, Lord Daichi, Sister Hotaru, and Yokai placeholder | Card Browser opens focused on the clicked card. |
| Press `T` in Card Browser | Global CardUI settings change without changing the GameScene world. |
| Flip the browser card | Only the currently viewed card flips; leaving/opening another card does not preserve that flip state. |

## Completion Checks

| Check | Expected Result |
| ----- | --------------- |
| Asset paths | New assets live under lowercase paths in `bevy/crates/game/assets`. |
| Card registry | Four JW/Japan Realism card identities replace SkyBolt/Tar in normal runtime presentation. |
| World registry | Bamboo Forest and Coastal Harbor replace the pre-008 world background path. |
| Persistence | CardUI settings persist globally; flip state does not persist. |
| Desktop/web parity | Relevant desktop and browser checks pass or any blocker is documented. |
