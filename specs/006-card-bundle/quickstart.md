# Quickstart: Card Bundle

## Prerequisites

| Requirement | Command |
| ----------- | ------- |
| Verify project dependencies once per machine | `scripts/main/InstallDependencies.ps1` |
| Stop previous local app runs | `scripts/other/StopApp.ps1` |

## Automated Checks

| Goal | Command |
| ---- | ------- |
| Run Rust test suite | `scripts/other/RunTests.ps1` |
| Desktop compile check only | `scripts/main/RunAppDesktop.ps1 -CheckOnly` |
| Browser compile check only | `scripts/other/RunAppWeb.ps1 -CheckOnly` |

## Manual Desktop Verification

| Step | Expected Result |
| ---- | --------------- |
| Run `scripts/main/RunAppDesktop.ps1` | The desktop app opens to the current prototype with one inspectable card |
| Inspect initial card | Exactly one centered poker-proportion card is visible |
| Move pointer to center, edges, and corners | Card tilts smoothly toward pointer direction, clamps at supported limits, and camera remains fixed |
| Inspect CardFront | Background, frame, foreground, and title remain visually distinct and flat-front contained |
| Tilt CardFront | Parallax and frame shine respond to smoothed tilt without jitter or unreadable layers |
| Press `T` while CardFront is visible | Active front changes when another valid entry exists; otherwise it remains valid |
| Press `R` | Reloadable AppScene card content rebuilds and DebugHUD toggle state remains valid |
| Press `H` | Hot-reload auto-restart toggles independently and persists through local runtime state |
| Open temporary Card UI | A `Flip` button appears in Card UI, separate from DebugHUD |
| Click `Flip` from CardFront | Card rotates toward CardBack and swaps side graphics at the edge-on midpoint |
| Move pointer during flip | Card keeps pointer-driven inspection feel without snapping to neutral |
| Click `Flip` during animation | Flip reverses direction from current progress |
| Press `T` while CardBack is visible | CardBack remains visible; changed CardFront appears only after flipping face up |
| Inspect CardBack | Backface is a shared abstract superhero-pattern design with no words, readable letters, characters, logos, or clear symbols |

## Manual Browser Verification

| Step | Expected Result |
| ---- | --------------- |
| Run `scripts/other/RunAppWeb.ps1 -NoOpen` | The web app is served locally and logs the URL |
| Open the served URL in a WebGPU-capable browser | The same single-card prototype appears |
| Repeat desktop inspection, polish, `T`, and flip checks | Behavior matches desktop or exact browser/WebGPU blocker is documented |

## Completion Evidence

| Evidence | Requirement |
| -------- | ----------- |
| Test output | `RunTests.ps1` passes or blocker is documented |
| Desktop smoke | Inspection, CardFront polish, `T`, `R`, `H`, and `Flip` verified |
| Browser smoke | Web behavior verified or exact tooling/browser blocker documented |
| Asset audit | CardFront assets resolve through CardType/CardDefinition paths and CardBack resolves through shared CardStructure/card-series ownership |
| Art-direction audit | CardBack is abstract superhero-pattern card back art and not words, logos, characters, or future box/menu art |
| Scope audit | No gameplay, tabletop placement, final menus, collection UI, deck browsing, or multi-card layout introduced |
| UI audit | Card UI remains temporary and separate from DebugHUD; Deck remains prototype entry point only |
