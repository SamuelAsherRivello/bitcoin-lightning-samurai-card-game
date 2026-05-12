# Quickstart: Project Setup

| Goal | Command |
| ---- | ------- |
| Install dependencies once | `scripts/main/InstallDependencies.ps1` |
| Check dependencies without installing | `scripts/main/InstallDependencies.ps1 -CheckOnly` |
| Run automated tests | `scripts/other/RunTests.ps1` |
| Reusable headless compile helper | `scripts/other/CompileApp.ps1 -Action check -PackageName bevy-card-game -TargetDir target/run-app-desktop -Features fast-dev` |
| Run desktop app | `scripts/main/RunAppDesktop.ps1` |
| Check desktop compile without launch | `scripts/main/RunAppDesktop.ps1 -CheckOnly` |
| Run release-like desktop app | `scripts/main/RunAppDesktop.ps1 -Release` |
| Check release-like desktop compile without launch | `scripts/main/RunAppDesktop.ps1 -CheckOnly -Release` |
| Run desktop app with explicit Windows target cache | `scripts/main/RunAppDesktop.ps1 -TargetTriple x86_64-pc-windows-msvc` |
| Run desktop app with hot reload | `scripts/main/RunAppDesktopHotReload.ps1` |
| Run desktop hot reload with extra `dx` args | `scripts/main/RunAppDesktopHotReload.ps1 -- --verbose` |
| Run browser WebGPU app | `scripts/main/RunAppWeb.ps1` |
| Check browser WebGPU compile without launch | `scripts/main/RunAppWeb.ps1 -CheckOnly` |
| Build and serve browser app without opening browser | `scripts/main/RunAppWeb.ps1 -NoOpen` |
| Stop desktop app, browser server, and project build processes | `scripts/other/StopApp.ps1` |
| Run desktop app through VS Code | `Terminal > Run Task... > Bevy Card Game: RunAppDesktop` |
| Run desktop hot reload through VS Code | `Terminal > Run Task... > Bevy Card Game: RunAppDesktopHotReload` |

## Manual Acceptance

| Check | Expected Result |
| ----- | --------------- |
| Run dependency setup | Rust, Cargo, installed target list, metadata check, and warm desktop cache output appear |
| Run automated tests | `cargo test --workspace` runs from the repository root with the dedicated desktop target cache |
| Run desktop check-only workflow | Desktop package compiles without launching the app |
| Run desktop hot reload workflow | Dioxus CLI starts the desktop hot-patch workflow, reports its version, and keeps output attached to the terminal |
| Change one line in one crate after warmed cache, then rerun development workflow | Rebuild uses the existing target cache and completes in 2 seconds or less, or the blocker is recorded |
| Run release-like desktop workflow | Build uses release profile semantics and does not include development-only hot reload, asset watching, or Bevy dynamic-linking features by default |
| Delete or ignore local placement file, then launch | Window opens at 1024x768 |
| Move and resize window, then close normally | Placement JSON is written under `data/local_storage/` |
| Relaunch with same screen setup | Window reopens at saved x/y and size |
| Relaunch after invalid/off-screen placement data | Window opens centered on the primary screen at 1024x768 |
| Run browser WebGPU workflow | Wasm target builds, browser bundle is generated under `target/run-app-web/site`, localhost serving starts, and the browser opens |
| Run browser WebGPU workflow with `-NoOpen`, then request localhost page | `http://127.0.0.1:8080/` returns a successful response |
| Run stop workflow after desktop or web launch | Project-local app, build, and web server processes stop cleanly |
| Run stop workflow after desktop hot reload launch | Project-local app, `dx`, Cargo, and Rust compiler processes stop cleanly |
| Run VS Code desktop task | Output appears in a dedicated integrated terminal panel |
| Run VS Code desktop hot reload task | Hot reload output appears in a dedicated integrated terminal panel |
| Inspect Bevy template reference | `bevy/crates/template-crate` contains the proper reference skeleton for Bevy crate folders, representative files, asset folders, and Rust coding standards |
