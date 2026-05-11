# UI Contract: Debugging UI

| Contract Item | Requirement |
| ------------- | ----------- |
| Source ownership | DebugHUD UI, inspector visibility, diagnostic key classification, and DebugHUD input capture live under `bevy/crates/shared`; Card UI and scene-specific debug drawing live under `bevy/crates/game`; the game crate composes shared diagnostics |
| Panel visibility | Exactly one DebugHUD panel is visible by default after startup |
| Panel placement | Panel is anchored near the top-left of the aspect-ratio-safe HUD area with translucent background styling |
| Status text | Panel shows the prototype title and frame/status text |
| Key labels | Panel shows `W`, `A`, `S`, `D`, `F`, and `I` labels |
| `F` behavior | Pressing `F` toggles FPS text visibility and does not toggle inspector visibility |
| `I` behavior | Pressing `I` toggles inspector visibility and does not toggle FPS visibility |
| `WASD` behavior | Pressing `W`, `A`, `S`, or `D` may activate DebugHUD-only hold feedback, but does not move or rotate the card, trigger gameplay, or toggle diagnostics |
| Card UI separation | Card UI is temporary developer/prototype UI, remains separate from DebugHUD, and is not final player-facing UI |
| Card UI placement | Card UI, when present, remains inside the aspect-ratio-safe HUD area and does not merge with DebugHUD |
| Debug drawing | Requested scene areas may be marked with runtime visual annotations that remain until removal or replacement is requested |
| Debug drawing placement | Debug drawings align to requested `GameView` or `CardBrowserView` scene areas using aspect-ratio-safe coordinates, not raw window pixels |
| Debug drawing temporariness | Debug drawings are temporary scaffolding and must not be treated as final player-facing UI or production art |
| Terminal logging | Scoped logs may support developer self-debugging, but must avoid secrets and unrelated noisy output |
| Test workflow | Implementers run `scripts/other/RunTests.ps1` and document any blocked manual or browser checks before handoff |
| Excluded systems | Toasts, minimap, reticle, autopilot, reset, shooting, health, score, and gameplay HUD behavior are not part of this contract |
| Responsive behavior | DebugHUD, Card UI, inspector offsets, and debug drawings update placement when the primary window or browser viewport is resized |
