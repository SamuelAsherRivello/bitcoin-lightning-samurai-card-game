# Quickstart: Basic Setup

| Goal | Command |
| ---- | ------- |
| Install dependencies once | `scripts/main/InstallDependencies.ps1` |
| Run automated tests | `scripts/main/RunTests.ps1` |
| Run desktop app | `scripts/main/RunAppDesktop.ps1` |
| Stop desktop app and project build processes | `scripts/other/StopApp.ps1` |
| Run desktop app through VS Code | `Terminal > Run Task... > Bevy Card Game: Run Desktop` |

## Manual Acceptance

| Check | Expected Result |
| ----- | --------------- |
| Delete or ignore local placement file, then launch | Window opens at 800x600 |
| Move and resize window, then close normally | Placement JSON is written under `generated/runtime/` |
| Relaunch with same screen setup | Window reopens at saved x/y and size |
| Relaunch after invalid/off-screen placement data | Window opens centered on the primary screen at 800x600 |
| Run VS Code desktop task | Output appears in a dedicated integrated terminal panel |
