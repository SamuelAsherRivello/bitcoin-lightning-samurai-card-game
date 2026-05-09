# Project Memory

## Repository Conventions

| Topic | Decision |
| ----- | -------- |
| Purpose | Bevy ECS card game built from the Codex Project Template. |
| Root docs | Keep Codex and Specify guidance visible while documenting Bevy game conventions. |
| Scripts | Put repeatable project commands in root `scripts`. |
| Assets | Put runtime assets under `Bevy/Crates/Game/Assets`. |
| Specs | Put active feature specs in `specs`. |
| Images | Keep README images in `documentation/images`. |

## Bevy Stack

| Topic | Decision |
| ----- | -------- |
| Workspace | Rust workspace at the repository root. |
| Game crate | `Bevy/Crates/Game` with package name `bevy-card-game`. |
| Shared crate | `Bevy/Crates/Shared` for reusable non-Bevy game constants and logic. |
| ECS layout | Keep components, resources, systems, and plugins under `Bevy/Crates/Game/src/runtime`. |
| Verification | Use `scripts/main/InstallDependencies.ps1` once per machine, then `scripts/main/RunTests.ps1`, `scripts/main/RunAppDesktop.ps1`, and `scripts/other/StopApp.ps1`. |

## Notes

- Do not store secrets, credentials, private keys, tokens, or personal data in memory files.
- Add durable project decisions here only when they help future agents avoid rediscovery.
