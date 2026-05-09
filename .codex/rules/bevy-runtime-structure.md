# Bevy Runtime Structure

Use this rule when adding, moving, reviewing, or documenting Bevy runtime code.

## Structure Rules

| Rule | Requirement |
| ---- | ----------- |
| Crates | Use `bevy/crates/game` for the Bevy app and gameplay; use `bevy/crates/shared` for reusable runtime support. |
| ECS layout | Keep runtime code split by `components`, `resources`, `systems`, and `plugins`. |
| Ownership | Components hold data; systems own behavior; resources hold shared state; plugins group features and app wiring. |
| Assets | Put runtime assets in `bevy/crates/game/assets`; keep custom shaders in `bevy/crates/game/assets/shaders`. |
| Feature wiring | Prefer feature plugins over direct app wiring in entrypoints. |
| Hot reload | Mark hot-reloadable update systems with `#[hot]` when the workflow supports it. |
| Tests | Keep tests aligned with feature and plugin behavior, close to the runtime code they validate. |
| Path casing | Keep this repo's lowercase Rust-style paths unless a separate migration updates the constitution, scripts, specs, and docs. |

## Reference Source

Imported from the structure guidance in `https://github.com/SamuelAsherRivello/bevy-jam-1` and adapted to this repository's existing lowercase `bevy/crates` layout.
