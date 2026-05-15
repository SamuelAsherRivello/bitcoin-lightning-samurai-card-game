# Data Model: Debugging

| Entity | Fields | Relationships | Validation Rules |
| ------ | ------ | ------------- | ---------------- |
| Debugging Tooling | DebugHUD, inspector, Card UI separation, debug drawings, hot reload screen reset, terminal logs, test workflow | Supports developer-facing diagnostics across rendered UI, terminal output, hot patch events, and automated checks | Must remain developer-facing and must not become final player-facing gameplay UI |
| Shared Debug Runtime | `bevy/crates/shared` DebugHUD, inspector, diagnostic input capture, and tests | Composed by `bevy/crates/game` and reusable by future app surfaces | Must not contain card-specific interaction, rendering, selection, scoring, Card UI state, debug drawing target data, or gameplay state |
| DebugHUD Panel | `DebugHudText`, Bevy `Text`, `Node`, `BackgroundColor` | Owns child `TextSpan` key labels and optional FPS text span | One panel is spawned by default; it remains anchored near the top-left |
| Key Legend Label | `DebugHudKeyText.key_code`, `DebugHudKeyText.is_toggle`, `UnderlineColor` | Child text span of the HUD panel | Labels exist for approved DebugHUD keys including `W`, `A`, `S`, `D`, `F`, `I`, and `H`; `WASD` labels are non-toggle hold indicators and may reflect pressed state only inside the DebugHUD |
| FPS Toggle State | `DebugHudState.is_fps_visible`, `fps_accumulated_seconds`, `fps_accumulated_frames`, `fps_display_value` | Read by `update_debug_hud`; displayed through `DebugHudFpsText` | `F` toggles visibility only; hidden FPS uses an empty text span |
| Inspector State | `InspectorState.is_visible`, `x`, `y`, `width`, `height` | Read by `inspector_ui`; toggled by `toggle_inspector`; reflected by the `I` key label | `I` toggles inspector visibility only; hidden inspector draws no egui window |
| Aspect-Ratio Debug Layout | Safe-area anchor, virtual game-view bounds, responsive placement inputs | Applies to DebugHUD, Card UI, inspector offsets, and debug drawings | Visible diagnostic surfaces must derive placement from the aspect-ratio-safe game view and update when the viewport changes |
| Card UI | Card-specific temporary controls and labels | Lives with card-specific prototype behavior in `bevy/crates/game` and remains conceptually separate from DebugHUD | Must not be merged into DebugHUD, moved into shared diagnostics, or described as final player-facing UI |
| Debug Drawing | Runtime visual mark geometry, target area metadata, temporary label or marker intent | Lives with game-scene-specific behavior in `bevy/crates/game`; may consume shared diagnostics but is not owned by shared diagnostics | Must remain temporary, scoped to the requested scene area, aspect-ratio-safe, and removable or replaceable with real UI/art later |
| Hot Reload Screen Reset | `is_enabled`, `last_observed_patch_count`, `pending_screen_reset`, `last_rebuilt_screen`, `last_rebuild_reason` | Consumes shared `H` toggle state and app-specific active screen identity; rebuilds the active screen inside `bevy/crates/game` | When enabled, every observed hot-patch event completely rebuilds the active conceptual screen; when disabled, hot patches must not reinitialize screens |
| Conceptual Screen | `active_screen`, screen root entity, screen-owned entity marker, screen-local models | Hosted below persistent `AppScene`; maps to GameScreen, DeckScreen, DebugScreen, and meta screens | Rebuild means despawn/recreate the active screen root and reset screen-local state as if newly navigated to that screen |
| Terminal Self-Logging | Scoped runtime or test log messages | Supports implementer diagnosis during local run, build, and test loops | Must avoid secrets, credentials, and unrelated noisy output |
| Test Workflow | `scripts/other/RunTests.ps1`, manual acceptance checks, blocked-check notes | Verifies DebugHUD, inspector, Card UI separation, and diagnostic input behavior | Must be repeatable from the repository root |
| Game Tick State | `GameTicks.0` | Read by HUD status text | Increments during updates and is displayed as frame/status text |

## State Transitions

| Input | Previous State | New State | Side Effects |
| ----- | -------------- | --------- | ------------ |
| `F` just pressed | FPS hidden | FPS visible | `DebugHudFpsText` shows sampled FPS text |
| `F` just pressed | FPS visible | FPS hidden | `DebugHudFpsText` becomes empty |
| `I` just pressed | Inspector hidden | Inspector visible | Inspector egui window can render |
| `I` just pressed | Inspector visible | Inspector hidden | Inspector egui window does not render |
| `W`, `A`, `S`, or `D` pressed | Any diagnostic state | No toggle state changes | May update DebugHUD-only hold feedback; no gameplay, movement, camera, card, selection, score, or deck behavior |
| Card UI opened | DebugHUD visible or hidden | Card UI visible | Card UI remains separate from DebugHUD and remains temporary developer/prototype UI |
| Debug drawing requested | No mark or stale mark | Runtime scene area visibly marked | Mark remains until removal/replacement is requested or it becomes misleading |
| Viewport resized | DebugHUD, Card UI, inspector, or debug drawing visible | Diagnostic placement recalculated | Positions remain inside or aligned to the aspect-ratio-safe game view |
| Debug drawing removed or replaced | Runtime mark visible | Mark hidden, despawned, or replaced by production UI/art | Temporary scaffold no longer appears as debug drawing |
| `H` just pressed | Hot reload screen reset disabled | Hot reload screen reset enabled | The next observed hot-patch event will rebuild the active conceptual screen |
| `H` just pressed | Hot reload screen reset enabled | Hot reload screen reset disabled | Future hot-patch events may be applied but must not reset or reinitialize the active screen |
| Hot-patch event observed | `H` enabled, active screen stable | Active screen rebuilt from its default entry state | Screen-local state is discarded; persistent AppScene state remains unless explicitly screen-owned |
| Hot-patch event observed | `H` disabled | Current screen remains mounted | No screen-local state reset or scene presentation restart occurs because of the patch |
| Hot-patch event observed during navigation | Active screen identity changing | Rebuild deferred or targeted after navigation settles | Reset applies to the final stable active conceptual screen |
| Tests run | Implementation changed | Verification result known | Terminal output and test status support self-QA before handoff |
