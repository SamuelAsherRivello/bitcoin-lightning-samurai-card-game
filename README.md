# Bevy Card Game

A Bevy ECS card game project built from the Codex Project Template.

## Getting Started

| Task | Command |
| ---- | ------- |
| Build | `project/scripts/build.ps1` |
| Test | `project/scripts/test.ps1` |
| Run | `project/scripts/run.ps1` |

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
| `project/scripts` | Repeatable local commands. |
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
