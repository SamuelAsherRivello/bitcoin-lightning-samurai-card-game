# Project Memory

## Repository Conventions

| Topic | Decision |
| ----- | -------- |
| Purpose | Bevy ECS card game built from the Codex Project Template. |
| Root docs | Keep Codex and Specify guidance visible while documenting Bevy game conventions. |
| Scripts | Put repeatable project commands in root `scripts`. |
| Assets | Put runtime assets under `bevy/crates/game/assets`. |
| Template crate | Use `bevy/crates/template-crate` as the proper reference for Bevy crate folders, representative files, asset folders, and Rust coding standards. |
| Theme assets | Put theme-owned cards, locations, and worlds under `bevy/crates/game/assets/themes/theme_<theme_name>/{cards,locations,worlds}/` with `card_`, `location_`, and `world_` folders. |
| Shaders | Put custom runtime shaders under `bevy/crates/game/assets/shaders/`. |
| Specs | Put active feature specs in `specs`. |
| Images | Keep README images in `documentation/images`. |

## Bevy Stack

| Topic | Decision |
| ----- | -------- |
| Workspace | Rust workspace at the repository root. |
| Path casing | Bevy workspace paths are lowercase `bevy/crates/...`; do not use `Bevy/Crates/...` in code, scripts, workflows, or docs. |
| Game crate | `bevy/crates/game` with package name `samurai-card-game`. |
| Shared crate | `bevy/crates/shared` for reusable non-Bevy game constants and logic. |
| Template crate | `bevy/crates/template-crate` is a non-workspace reference skeleton for Bevy crate layout, runtime role folders, asset folders, and representative Rust files. |
| ECS layout | Keep components, resources, systems, and plugins under `bevy/crates/game/src/runtime`. |
| Runtime file shape | Changed runtime files center on one primary plugin, component, scene, view, model, or system concept. |
| Runtime naming | Use `Scene` for the persistent app-level scene, `Model` for data, and `View` for rendering/presentation; `AppScene` is persistent and active sub-screens are views such as `GameView`, `DeckBuilderScene`, and `DebugSettingsScene`. |
| Card naming | Use `CardModel` for card data, `CardView` for rendered card presentation, and `CardViewBundle` for the visual bundle. |
| System naming | Changed runtime system functions use `[domain]_[schedule]_system`, for example `player_update_system`. |
| Purpose comments | Changed or new primary runtime items have a terse `HUMAN:` line and `AI:` line immediately above the item. |
| AI runtime peek | When the user asks to "peek" at the app/running app/game, query local BRP at `http://localhost:15702`, call `bevy_debugger/screenshot` into `target/ai-runtime-screenshots/`, inspect the screenshot, and report runtime plus visual observations. |
| Scene vocabulary | `AppScene` is always-present; report the active view such as `GameView`, `DeckBuilderScene`, or `DebugSettingsScene` as the current scene/view state. |
| Verification | Use `scripts/main/InstallDependencies.ps1` once per machine, then `scripts/main/RunTests.ps1`, `scripts/main/RunAppDesktop.ps1`, and `scripts/other/StopApp.ps1`. |
| Desktop warm builds | `RunAppDesktop.ps1` uses a dedicated `target/run-app-desktop` cache and enables the `fast-dev` feature for Bevy dynamic linking on non-release runs. |
| Dependency install | `InstallDependencies.ps1` warms the `target/run-app-desktop` cache with `cargo build -p samurai-card-game --features fast-dev`. |

## Notes

- Do not store secrets, credentials, private keys, tokens, or personal data in memory files.
- Add durable project decisions here only when they help future agents avoid rediscovery.
