# Quickstart: Debugging

| Goal | Command |
| ---- | ------- |
| Install dependencies once | `scripts/main/InstallDependencies.ps1` |
| Run automated tests | `scripts/other/RunTests.ps1` |
| Run Windows desktop app | `scripts/main/RunAppDesktop.ps1` |
| Check Windows desktop compile without launch | `scripts/main/RunAppDesktop.ps1 -CheckOnly` |
| Run Windows desktop app in release mode | `scripts/main/RunAppDesktop.ps1 -Release` |
| Run Windows desktop app with explicit Windows target cache | `scripts/main/RunAppDesktop.ps1 -TargetTriple x86_64-pc-windows-msvc` |
| Stop desktop app and project build processes | `scripts/other/StopApp.ps1` |

## Developer Self-QA

| Check | Expected Result |
| ----- | --------------- |
| Run tests before handoff | `scripts/other/RunTests.ps1` completes, or the exact blocker is documented |
| Review terminal output | Build, run, and test output shows relevant diagnostic information without secrets or unrelated noise |
| Use scoped logs while debugging | Temporary or permanent logs explain DebugHUD, inspector, Card UI, or diagnostic input state clearly enough to diagnose the current issue |
| Use debug drawing for visual discussion | Runtime marks identify requested scene areas, such as the hand area, and remain until removal or replacement is requested |
| Record blocked checks | Any skipped desktop, browser WebGPU, or manual UI check includes the command attempted and blocker |

## Manual Acceptance

| Check | Expected Result |
| ----- | --------------- |
| Launch desktop app | A translucent DebugHUD panel appears near the top-left |
| Read HUD contents | The panel shows title/status text and labels for `W`, `A`, `S`, `D`, `F`, and `I` |
| Press `F` repeatedly | FPS text toggles on each press; inspector visibility does not change |
| Press `I` repeatedly | Inspector visibility toggles on each press; FPS visibility does not change |
| Press `W`, `A`, `S`, `D` | DebugHUD-only hold feedback may update; no gameplay, camera, card, or diagnostic toggle changes |
| Inspect Card UI if present | Card UI appears only as temporary developer/prototype UI and remains separate from DebugHUD |
| Inspect debug drawing if present | Requested scene areas are visibly marked, clearly temporary, and not mistaken for final art or player-facing UI |
| Resize the window | HUD remains anchored and scales consistently |

## Browser WebGPU Verification

| Status | Notes |
| ------ | ----- |
| Required before final cross-target acceptance | Use the project browser/WebGPU workflow once available in scripts or CI; if unavailable, record the exact missing toolchain or script blocker in the feature verification notes |
