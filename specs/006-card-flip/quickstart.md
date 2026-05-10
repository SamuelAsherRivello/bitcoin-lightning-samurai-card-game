# Quickstart: Card Flip

## Prerequisites

| Requirement | Command |
| ----------- | ------- |
| Verify project dependencies once per machine | `scripts/main/InstallDependencies.ps1` |
| Stop previous local app runs | `scripts/other/StopApp.ps1` |

## Test

| Goal | Command |
| ---- | ------- |
| Run Rust test suite | `scripts/other/RunTests.ps1` |
| Desktop compile check only | `scripts/other/RunAppDesktop.ps1 -CheckOnly` |
| Browser compile check only | `scripts/other/RunAppWeb.ps1 -CheckOnly` |

## Manual Desktop Verification

| Step | Expected Result |
| ---- | --------------- |
| Run `scripts/other/RunAppDesktop.ps1` | The desktop app opens to the CardBrowser prototype entry point with one centered inspectable card |
| Open the temporary Card UI | A `Flip` button appears in Card UI, separate from DebugHUD |
| Click `Flip` from the front face | The card rotates toward the back and swaps from CardFront to CardBack at the edge-on midpoint |
| Move the pointer during the flip | The card keeps its pointer-driven inspection feel without snapping to neutral |
| Click `Flip` during the animation | The flip reverses direction from current progress |
| Press `T` while CardFront is visible | The visible CardFront changes immediately |
| Press `T` while CardBack is visible | CardBack remains visible; the changed CardFront appears only after flipping face up |
| Inspect the backface content | The backface has no words, readable letters, characters, logos, or clear symbols |
| Compare the backface to existing fronts | The palette and tone feel compatible with the existing superhero card fronts |

## Manual Browser Verification

| Step | Expected Result |
| ---- | --------------- |
| Run `scripts/other/RunAppWeb.ps1 -NoOpen` | The web app is served locally and logs the URL |
| Open the served URL in a WebGPU-capable browser | The same single-card scene appears |
| Repeat desktop flip checks | Card UI flip behavior, midpoint face swap, pointer inspection, and shared backface behavior match desktop |

## Completion Evidence

| Evidence | Requirement |
| -------- | ----------- |
| Test output | `RunTests.ps1` passes or blocker is documented |
| Desktop smoke | Front-to-back, back-to-front, and mid-animation reversal verified |
| Browser smoke | Web behavior verified or exact tooling/browser blocker documented |
| Asset audit | Backface asset exists under `bevy/crates/game/assets/cards/card_structure/` and is not duplicated under front-art folders |
| Art-direction audit | Backface is an abstract superhero-pattern card back, not medieval fantasy, not a character/logo card, and not future box/menu art |
| Scope audit | CardBrowser remains the current prototype entry point and is not treated as final user-facing game UI |
| UI audit | Card UI remains temporary and separate from DebugHUD |

## Implementation Verification

| Check | Result |
| ----- | ------ |
| `scripts/other/RunTests.ps1` | Passed on 2026-05-10 |
| `scripts/other/RunAppDesktop.ps1 -CheckOnly` | Passed on 2026-05-10 |
| `scripts/other/RunAppWeb.ps1 -CheckOnly` | Passed on 2026-05-10 |
| Manual desktop smoke | Not run in this pass |
| Manual browser smoke | Not run in this pass |
