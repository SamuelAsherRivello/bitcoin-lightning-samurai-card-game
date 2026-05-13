# Quickstart: Debugging

| Goal | Command |
| ---- | ------- |
| Install dependencies once | `scripts/main/InstallDependencies.ps1` |
| Run automated tests | `scripts/other/RunTests.ps1` |
| Run Windows desktop app | `scripts/main/RunAppDesktop.ps1` |
| Check Windows desktop compile without launch | `scripts/main/RunAppDesktop.ps1 -CheckOnly` |
| Run Windows desktop app in release mode | `scripts/main/RunAppDesktop.ps1 -Release` |
| Run Windows desktop app with hot reload | `scripts/main/RunAppDesktopHotReload.ps1` |
| Run Windows desktop app with explicit Windows target cache | `scripts/main/RunAppDesktop.ps1 -TargetTriple x86_64-pc-windows-msvc` |
| Check browser WebGPU compile without launch | `scripts/other/RunAppWeb.ps1 -CheckOnly` |
| Stop desktop app and project build processes | `scripts/other/StopApp.ps1` |

## Developer Self-QA

| Check | Expected Result |
| ----- | --------------- |
| Run tests before handoff | `scripts/other/RunTests.ps1` completes, or the exact blocker is documented |
| Review terminal output | Build, run, and test output shows relevant diagnostic information without secrets or unrelated noise |
| Use scoped logs while debugging | Temporary or permanent logs explain DebugHUD, inspector, Card UI, or diagnostic input state clearly enough to diagnose the current issue |
| Use debug drawing for visual discussion | Runtime marks identify requested scene areas, such as the hand area, and remain until removal or replacement is requested |
| Review safe-area placement | DebugHUD, Card UI, inspector offsets, and debug drawings align to the aspect-ratio-safe game view rather than raw window corners |
| Record blocked checks | Any skipped desktop, browser WebGPU, or manual UI check includes the command attempted and blocker |

## Debug Drawing Reference Layout

| Marker | Runtime Target | Quantized Rect In 1280x800 Game View | Layering |
| ------ | -------------- | ------------------------------------ | -------- |
| `game area` | Game area | `left=304 top=0 width=672 height=800` | UI order 1, below 3D card overlay order 2 |
| `location area` | Location area | `left=364 top=224 width=184 height=208` | UI order 1, below 3D card overlay order 2 |
| `location area` | Location area | `left=548 top=224 width=184 height=208` | UI order 1, below 3D card overlay order 2 |
| `location area` | Location area | `left=732 top=224 width=184 height=208` | UI order 1, below 3D card overlay order 2 |
| unlabeled | Top location card slot | `left=364 top=44 width=184 height=180` | UI order 1, below 3D card overlay order 2 |
| unlabeled | Top location card slot | `left=548 top=44 width=184 height=180` | UI order 1, below 3D card overlay order 2 |
| unlabeled | Top location card slot | `left=732 top=44 width=184 height=180` | UI order 1, below 3D card overlay order 2 |
| unlabeled | Bottom location card slot | `left=364 top=432 width=184 height=180` | UI order 1, below 3D card overlay order 2 |
| unlabeled | Bottom location card slot | `left=548 top=432 width=184 height=180` | UI order 1, below 3D card overlay order 2 |
| unlabeled | Bottom location card slot | `left=732 top=432 width=184 height=180` | UI order 1, below 3D card overlay order 2 |
| `hand area` | Hand area | `left=364 top=612 width=552 height=188` | UI order 1, below 3D card overlay order 2 |

The default `DebugDrawingModel` requests this full reference layout, but the lines render only when the persisted DebugHUD `D` toggle is on and the active view is `GameScene`. Use `remove(target)` to clear a mark, `replace(target, label, rect)` to move or relabel one mark, and `request_reference_layout()` to restore the six inspiration-layout annotations.

## Hot Reload

| Control Or Script | Purpose | Expected Behavior |
| ----------------- | ------- | ----------------- |
| `H` DebugHUD toggle | Enables automatic runtime rebuild after hot patches | When `H` is on and any desktop hot-patch event arrives, the app rebuilds the persistent `AppScene`, respawns the DebugHUD, and rebuilds the current sub-screen view such as `GameScene`, `DeckScene`, or `DebugScene` |
| `R` DebugHUD key | Manual runtime rebuild | Rebuilds the `AppScene` and current sub-screen without waiting for a hot-patch event |
| `scripts/main/RunAppDesktopHotReload.ps1` | Desktop Rust hot-patch workflow | Runs `dx serve --hot-patch --windows` with `desktop-hot-reload` and `asset-hot-reload`; edit hot-enabled Rust systems and save |
| `scripts/main/RunAppDesktop.ps1` | Standard desktop workflow | Builds/runs without Rust hot patching, but includes asset hot reload in non-release mode |
| `scripts/other/StopApp.ps1` | Stops running app/build helpers | Use before switching between normal desktop and hot-reload workflows |

Hot reload uses the included third-party `bevy_hotpatching_experiments` integration through the `desktop-hot-reload` feature and Dioxus CLI hot patching. Rust and Bevy hot reload have real limitations: structural ECS changes, new resources, changed component layouts, startup-only work, and some dependency-level changes may still require a fresh process. For this feature, the supported and expected behavior is narrower and realistic: when `H` is enabled, every observed hot-patch event triggers an in-process rebuild of `AppScene` plus the currently active sub-screen so changed debug drawing layout, HUD text, view setup, and scene-level presentation code become visible without manually restarting the app.

## Manual Acceptance

| Check | Expected Result |
| ----- | --------------- |
| Launch desktop app | A translucent DebugHUD panel appears near the top-left |
| Read HUD contents | The panel shows title/status text and labels for active debug actions including `D`, `F`, `P`, `I`, and `H` |
| Press `D` repeatedly | Debug drawing visibility toggles on each press, persists to disk, and only draws in `GameScene` |
| Press `F` repeatedly | Desktop toggles Bevy fullscreen; browser toggles the browser Fullscreen API; inspector visibility does not change |
| Press `I` repeatedly | Inspector visibility toggles on each press; FPS visibility does not change |
| Inspect Card UI if present | Card UI appears only as temporary developer/prototype UI and remains separate from DebugHUD |
| Inspect debug drawing if present | Requested scene areas are visibly marked, clearly temporary, and not mistaken for final art or player-facing UI |
| Resize the window | HUD, Card UI, inspector offsets, and debug drawings remain aligned to the aspect-ratio-safe game view |

## Browser WebGPU Verification

| Status | Notes |
| ------ | ----- |
| Required before final cross-target acceptance | Use `scripts/other/RunAppWeb.ps1 -CheckOnly` and the served browser WebGPU workflow; if unavailable, record the exact missing toolchain or script blocker in the feature verification notes |
