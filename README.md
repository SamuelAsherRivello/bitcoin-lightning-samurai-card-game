# Bevy Card Game

A Bevy ECS card game project built from the Codex Project Template.

## Getting Started

| Task | Command |
| ---- | ------- |
| Install dependencies once | `scripts/main/InstallDependencies.ps1` |
| Run tests | `scripts/other/RunTests.ps1` |
| Run desktop | `scripts/main/RunAppDesktop.ps1` |
| Check desktop | `scripts/main/RunAppDesktop.ps1 -CheckOnly` |
| Run web | `scripts/main/RunAppWeb.ps1` |
| Check web | `scripts/main/RunAppWeb.ps1 -CheckOnly` |
| Stop app | `scripts/other/StopApp.ps1` |

## Requirements

| Requirement | Purpose |
| ----------- | ------- |
| Windows package manager | `InstallDependencies.ps1` can install rustup through `winget` when Rust is not already installed. If `winget` is unavailable, install rustup manually from <https://rustup.rs/>. |
| Rust toolchain | `InstallDependencies.ps1` installs and verifies the `stable` Rust toolchain. |
| Rust target | `InstallDependencies.ps1` installs and verifies `x86_64-pc-windows-msvc`. |
| Cargo | `InstallDependencies.ps1` verifies `cargo` is available after rustup setup. If it is not found, restart the terminal and rerun the script. |
| MSVC linker | Desktop builds may require `link.exe`. Install Build Tools for Visual Studio from <https://visualstudio.microsoft.com/visual-cpp-build-tools/> with the `Desktop development with C++` workload. |
| Fast linker | `rust-lld` is optional. When available, the scripts use it for faster desktop linking; otherwise they fall back to the default Windows linker. |

## Build Speed

| Setting | Behavior |
| ------- | -------- |
| Desktop target | `scripts/main/RunAppDesktop.ps1` uses the host target with a dedicated `target/run-app-desktop` cache so other Cargo tasks do not invalidate warm runs. |
| First checkout | `scripts/main/InstallDependencies.ps1` warms the same desktop cache once, so later `RunAppDesktop.ps1` calls only rebuild changed code. |
| Default run | `scripts/main/RunAppDesktop.ps1` compiles only changed artifacts, then opens the cached desktop executable. |
| Headless compile helper | `scripts/other/CompileApp.ps1` centralizes Cargo build, check, and test setup for the main run scripts. |
| Fast validation | `scripts/main/RunAppDesktop.ps1 -CheckOnly` runs `cargo check -p bevy-card-game --features fast-dev` without launching the app. |
| Fast dev feature | Non-release desktop runs enable `fast-dev`, which turns on Bevy dynamic linking for faster edit-run cycles after the first build. |
| Web target | `scripts/main/RunAppWeb.ps1` builds `wasm32-unknown-unknown` into `target/run-app-web`, runs `wasm-bindgen`, serves the generated page on localhost, and opens the browser. |
| Explicit target | Pass `-TargetTriple x86_64-pc-windows-msvc` only when a separate target cache is required. |

## Structure

| Path | Purpose |
| ---- | ------- |
| `Bevy/Crates/Game` | Main Bevy game crate and executable. |
| `Bevy/Crates/Game/src/runtime/components` | ECS data attached to entities. |
| `Bevy/Crates/Game/src/runtime/resources` | Shared ECS state. |
| `Bevy/Crates/Game/src/runtime/systems` | ECS behavior. |
| `Bevy/Crates/Game/src/runtime/plugins` | Bevy plugin composition. |
| `Bevy/Crates/Game/Assets` | Runtime game assets, including cards, audio, and shaders. |
| `Bevy/Crates/Shared` | Shared Rust logic and constants. |
| `.codex` | Repo-local Codex guidance, skills, memory, and rules. |
| `.specify` | Specify workflow configuration and constitution. |
| `specs` | Active project specs. |
| `scripts` | Repeatable local commands. |
| `documentation/images` | README-visible supporting images. |

## Stack

| Area | Choice |
| ---- | ------ |
| Language | Rust |
| Engine | Bevy |
| Architecture | ECS with explicit components, resources, systems, and plugins |
| Workspace | Cargo workspace rooted at this repository |

## Development Notes

Keep gameplay changes small and spec-driven. Components should hold data, systems should own behavior, resources should hold shared state, and plugins should group related runtime features.

## Credits

Created by Samuel Asher Rivello.

## License

Provided as-is under [MIT License](./LICENSE).
