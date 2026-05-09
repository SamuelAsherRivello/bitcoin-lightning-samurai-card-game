# Bevy Card Game

A Bevy ECS card game project built from the Codex Project Template.

![Bevy Card Game overview](documentation/images/Overview01.png)

![Bevy Card Game workflow](documentation/images/Workflow01.png)

## Getting Started

| Task | Command |
| ---- | ------- |
| Install dependencies once | `scripts/main/InstallDependencies.ps1` |
| Run tests | `scripts/other/RunTests.ps1` |
| Run desktop | `scripts/other/RunAppDesktop.ps1` |
| Run desktop hot reload | `scripts/main/RunAppDesktopHotReload.ps1` |
| Check desktop | `scripts/other/RunAppDesktop.ps1 -CheckOnly` |
| Run web | `scripts/other/RunAppWeb.ps1` |
| Check web | `scripts/other/RunAppWeb.ps1 -CheckOnly` |
| Export web | `scripts/other/RunAppWeb.ps1 -Release -NoOpen -ExportOnly` |
| Stop app | `scripts/other/StopApp.ps1` |

## Live Demo

https://samuelasherrivello.github.io/bevy-card-game/latest/

The static web build is exported and hosted when a GitHub Release is published. Versioned releases live under `/releases/<version>/`, and `/latest/` points at the newest release.

## Requirements

| Requirement | Purpose |
| ----------- | ------- |
| Windows package manager | `InstallDependencies.ps1` can install rustup through `winget` when Rust is not already installed. If `winget` is unavailable, install rustup manually from <https://rustup.rs/>. |
| Rust toolchain | `InstallDependencies.ps1` installs and verifies the `stable` Rust toolchain. |
| Rust target | `InstallDependencies.ps1` installs and verifies `x86_64-pc-windows-msvc`. |
| Cargo | `InstallDependencies.ps1` verifies `cargo` is available after rustup setup. If it is not found, restart the terminal and rerun the script. |
| Dioxus CLI | `InstallDependencies.ps1` verifies or installs Dioxus CLI 0.7.x for desktop hot reload. Pass `-SkipHotReloadTools` to skip this optional setup. |
| MSVC linker | Desktop builds may require `link.exe`. Install Build Tools for Visual Studio from <https://visualstudio.microsoft.com/visual-cpp-build-tools/> with the `Desktop development with C++` workload. |
| Fast linker | `rust-lld` is optional. When available, the scripts use it for faster desktop linking; otherwise they fall back to the default Windows linker. |

## Build Speed

| Setting | Behavior |
| ------- | -------- |
| Desktop target | `scripts/other/RunAppDesktop.ps1` uses the host target with a dedicated `target/run-app-desktop` cache so other Cargo tasks do not invalidate warm runs. |
| First checkout | `scripts/main/InstallDependencies.ps1` warms the same desktop cache once, so later `RunAppDesktop.ps1` calls only rebuild changed code. |
| Default run | `scripts/other/RunAppDesktop.ps1` compiles only changed artifacts, then opens the cached desktop executable. |
| Headless compile helper | `scripts/other/CompileApp.ps1` centralizes Cargo build, check, and test setup for the main run scripts. |
| Fast validation | `scripts/other/RunAppDesktop.ps1 -CheckOnly` runs `cargo check -p bevy-card-game --features fast-dev` without launching the app. |
| Fast dev feature | Non-release desktop runs enable `fast-dev`, which turns on Bevy dynamic linking for faster edit-run cycles after the first build. |
| Desktop hot reload | `scripts/main/RunAppDesktopHotReload.ps1` uses Dioxus CLI hot patching with `target/run-app-desktop-hot-reload` and keeps output in the terminal. |
| Web target | `scripts/other/RunAppWeb.ps1` builds `wasm32-unknown-unknown` into `target/run-app-web`, runs `wasm-bindgen`, serves the generated page on localhost, and opens the browser. |
| Explicit target | Pass `-TargetTriple x86_64-pc-windows-msvc` only when a separate target cache is required. |

## Structure

| Path | Purpose |
| ---- | ------- |
| `.github` | GitHub Actions and repository automation, including release and web export workflows. |
| `bevy/crates/game` | Main Bevy game crate and executable for card-specific runtime behavior. |
| `bevy/crates/game/src/runtime/components` | Card-specific ECS data attached to entities. |
| `bevy/crates/game/src/runtime/resources` | Card-specific ECS resources and inspection state. |
| `bevy/crates/game/src/runtime/systems` | Card-specific setup, pointer mapping, smoothing, and DebugHUD composition. |
| `bevy/crates/game/src/runtime/plugins` | Game plugin composition and card POC tests. |
| `bevy/crates/game/assets` | Runtime game assets, including cards, audio, and shaders. |
| `bevy/crates/shared` | Reusable system-level Rust logic for shared runtime behavior. |
| `bevy/crates/shared/src/window.rs` | Project-approved desktop window defaults: 1024x768. |
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
| Architecture | Shared runtime crate plus game-specific ECS components, resources, systems, and plugins |
| Workspace | Cargo workspace rooted at this repository |

## Development Notes

Keep gameplay changes small and spec-driven. Reusable window, camera, DebugHUD, inspector, and diagnostic input behavior belongs in `bevy/crates/shared`; card-specific geometry, pointer mapping, smoothing, and gameplay behavior belongs in `bevy/crates/game`.

## GitHub Features

| Related tech | Link |
| ------------ | ---- |
| GitHub Actions | [GitHub Actions docs](https://docs.github.com/en/actions) |
| GitHub Pages | [Custom GitHub Pages workflows](https://docs.github.com/en/pages/getting-started-with-github-pages/using-custom-workflows-with-github-pages) |

Keep [`.github/workflows/export-web-build-to-github-pages.yml`](./.github/workflows/export-web-build-to-github-pages.yml) as the only GitHub Pages deployment workflow. Do not create a branch-based Pages action; it can fight with this custom export workflow.

Choose one setup option:

| Option | Instructions |
| ------ | ------------ |
| Enable Pages manually | In GitHub, open `Settings > Pages` and set `Source` to `GitHub Actions`. Do not select a branch source. |
| Add `PAGES_ADMIN_TOKEN` | Add a repository secret named `PAGES_ADMIN_TOKEN` with Pages write permission so the workflow can enable or repair Pages setup without creating another action. |

The GitHub Actions display names are:

| Workflow | Purpose |
| -------- | ------- |
| `PerformRelease` | Manually increments `VERSION.txt`, updates Cargo package versions, commits, tags, and creates a GitHub Release. |
| `ReleaseWebBuildToGithubPages` | Builds the release web app and publishes `/releases/<version>/` plus `/latest/` to GitHub Pages. |

## Release Deployment

Use GitHub Releases as the publishing boundary. Normal commits do not publish the project. To publish, run the `PerformRelease` workflow manually.

[`VERSION.txt`](./VERSION.txt) is the release source of truth. It stores the public project version without the tag prefix, such as `0.01`. Release tags add `v`, such as `v0.01`.

The release workflow uses this version style:

| `VERSION.txt` | Git tag | Cargo package version |
| ------------- | ------- | --------------------- |
| `0.01` | `v0.01` | `0.1.0` |
| `0.02` | `v0.02` | `0.2.0` |
| `0.03` | `v0.03` | `0.3.0` |

Each published release builds the public Bevy web target for GitHub Pages.

If the Pages workflow is run manually, leave the release version input blank to use the current `VERSION.txt`. Enter a value like `v0.01` only when redeploying a specific release folder.

## GitHub Pages URLs

| URL | Purpose |
| --- | ------- |
| `/latest/` | Newest published release. |
| `/releases/v0.01/` | Specific immutable release folder. |

The Pages workflow stores release folders on the `pages-releases` branch, then deploys them through GitHub Pages Actions. Keep the repository Pages source set to `GitHub Actions`, not a branch.

## Credits

Created by Samuel Asher Rivello.

## License

Provided as-is under [MIT License](./LICENSE).
