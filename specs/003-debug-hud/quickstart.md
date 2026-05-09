# Quickstart: DebugHUD

| Goal | Command |
| ---- | ------- |
| Run automated tests | `project/scripts/RunTests.ps1` |
| Build the workspace | `project/scripts/build.ps1` |
| Run Windows desktop app | `project/scripts/RunAppDesktop.ps1` |
| Run Windows desktop app in release mode | `project/scripts/RunAppDesktop.ps1 -Release` |

## Manual Acceptance

| Check | Expected Result |
| ----- | --------------- |
| Launch desktop app | A translucent DebugHUD panel appears near the top-left |
| Read HUD contents | The panel shows title/status text and labels for `W`, `A`, `S`, `D`, `F`, and `I` |
| Press `F` repeatedly | FPS text toggles on each press; inspector visibility does not change |
| Press `I` repeatedly | Inspector visibility toggles on each press; FPS visibility does not change |
| Press `W`, `A`, `S`, `D` | No gameplay, camera, card, diagnostic toggle, or key legend feedback changes |
| Resize the window | HUD remains anchored and scales consistently |

## Browser WebGPU Verification

| Status | Notes |
| ------ | ----- |
| Required before final cross-target acceptance | Use the project browser/WebGPU workflow once available in scripts or CI; if unavailable, record the exact missing toolchain or script blocker in the feature verification notes |
