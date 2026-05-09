# UI Contract: DebugHUD

| Contract Item | Requirement |
| ------------- | ----------- |
| Panel visibility | Exactly one DebugHUD panel is visible by default after startup |
| Panel placement | Panel is anchored near the top-left with translucent background styling |
| Status text | Panel shows the prototype title and frame/status text |
| Key labels | Panel shows `W`, `A`, `S`, `D`, `F`, and `I` labels |
| `F` behavior | Pressing `F` toggles FPS text visibility and does not toggle inspector visibility |
| `I` behavior | Pressing `I` toggles inspector visibility and does not toggle FPS visibility |
| `WASD` behavior | Pressing `W`, `A`, `S`, or `D` does not move or rotate the card, trigger gameplay, toggle diagnostics, or activate legend feedback |
| Excluded systems | Toasts, minimap, reticle, autopilot, reset, shooting, health, score, and gameplay HUD behavior are not part of this contract |
| Responsive behavior | HUD scale updates when the primary window is resized |
