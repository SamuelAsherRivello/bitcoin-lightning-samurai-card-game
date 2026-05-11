# Bevy Runtime Structure

Use this rule when adding, moving, reviewing, or documenting Bevy runtime code.

## Structure Rules

| Rule | Requirement |
| ---- | ----------- |
| Crates | Use `bevy/crates/game` for the Bevy app and gameplay; use `bevy/crates/shared` for reusable runtime support. |
| ECS layout | Keep runtime code split by `components`, `resources`, `systems`, and `plugins`. |
| Ownership | Components hold data; systems own behavior; resources hold shared state; plugins group features and app wiring. |
| Primary concept files | Changed runtime files should center on one primary plugin, component, scene, view, model, or system concept. |
| Purpose comments | Put a terse `HUMAN:` line and `AI:` line immediately above each changed or new primary runtime item. |
| Scene/Model/View naming | Use `Scene` for the persistent app-level scene, `Model` for data, and `View` for rendering or presentation; `AppScene` is persistent while `GameView`, `DeckBuilderScene`, and `DebugSettingsScene` are active sub-screen views. |
| System naming | Name changed runtime system functions as `[domain]_[schedule]_system`, for example `player_update_system`. |
| Assets | Put runtime assets in `bevy/crates/game/assets`; put theme-owned cards, locations, and worlds in `assets/themes/theme_<theme_name>/{cards,locations,worlds}` with category-prefixed folders; keep custom shaders in `bevy/crates/game/assets/shaders`. |
| Feature wiring | Prefer feature plugins over direct app wiring in entrypoints. |
| Hot reload | Mark hot-reloadable update systems with `#[hot]` when the workflow supports it. |
| Tests | Keep tests aligned with feature and plugin behavior, close to the runtime code they validate. |
| Path casing | Keep this repo's lowercase Rust-style paths unless a separate migration updates the constitution, scripts, specs, and docs. |
| AI runtime peek | For a request to "peek" at the running app, use local BRP on `http://localhost:15702` plus `bevy_debugger/screenshot`; save transient screenshots under `target/ai-runtime-screenshots/`. |

## Reference Source

Imported from the structure guidance in `https://github.com/SamuelAsherRivello/bevy-jam-1` and adapted to this repository's existing lowercase `bevy/crates` layout.
