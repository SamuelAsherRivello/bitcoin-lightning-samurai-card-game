# Project Memory

## Repository Conventions

| Topic | Decision |
| ----- | -------- |
| Purpose | Bevy ECS card game built from the Codex Project Template. |
| Root docs | Keep Codex and Specify guidance visible while documenting Bevy game conventions. |
| Scripts | Put repeatable project commands in `project/scripts`. |
| Assets | Put game runtime assets under `Bevy/Crates/Game/Assets`; keep non-game seed assets in `project/assets`. |
| Specs | Put active feature specs in `specs`. |
| Images | Keep README images in `documentation/images`. |

## Bevy Stack

| Topic | Decision |
| ----- | -------- |
| Workspace | Rust workspace at the repository root. |
| Game crate | `Bevy/Crates/Game` with package name `bevy-card-game`. |
| Shared crate | `Bevy/Crates/Shared` for reusable non-Bevy game constants and logic. |
| ECS layout | Keep components, resources, systems, and plugins under `Bevy/Crates/Game/src/runtime`. |
| Verification | Use `project/scripts/build.ps1`, `project/scripts/test.ps1`, and `project/scripts/run.ps1`. |

## Notes

- Do not store secrets, credentials, private keys, tokens, or personal data in memory files.
- Add durable project decisions here only when they help future agents avoid rediscovery.
