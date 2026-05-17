# Bevy Runtime Structure

Use this rule when adding, moving, reviewing, or documenting Bevy runtime code.

## Structure Rules

| Rule | Requirement |
| ---- | ----------- |
| Crates | Use `bevy/crates/game` for the Bevy app and gameplay; use `bevy/crates/shared` for reusable runtime support. |
| Template reference | Use `bevy/crates/template-crate` as the proper reference for Bevy crate folders, representative files, asset folders, and Rust coding standards before adding or reorganizing runtime code. |
| ECS layout | Keep runtime code split by `components`, `resources`, `systems`, and `plugins`. |
| Runtime domains | Apply the same file-ownership rules to `plugins`, `scenes`, `bundles`, and `shaders` modules. |
| Ownership | Components hold data; systems own behavior; resources hold shared state; plugins group features and app wiring. |
| Primary concept files | Changed runtime files should center on one primary plugin, component, scene, view, model, or system concept. |
| `mod.rs` behavior | Keep domain `mod.rs` files as module declarations and re-exports; do not keep primary runtime concepts in `mod.rs`. |
| Resource files | In `runtime/resources`, keep at most one `Resource` type per file. Use dedicated `*_resource.rs` files (for example `screen_transition_resource.rs` with `ScreenTransitionResource`). |
| Purpose comments | Put a terse `HUMAN:` line and `AI:` line immediately above each changed or new primary runtime item. |
| Scene/Model/View naming | Use `Scene` for the persistent app-level scene, `Model` for data, and `View` for rendering or presentation; `AppScene` is persistent while `GameScene`, `DeckScene`, and `DebugScene` are child scenes; conceptual screens pair `AppScene` with one child scene: `GameScreen` = `AppScene` + `GameScene`, `DeckScreen` = `AppScene` + `DeckScene`, and `DebugScreen` = `AppScene` + `DebugScene`. |
| 2D vs 3D presentation | Keep 2D Bevy UI entities for fixed screen overlays, controls, text panels, HUD, menus, and interaction hints. Use 3D mesh entities for camera-framed world presentation such as cards, locations, and world backgrounds; their apparent bounds must be derived from the same aspect-ratio-safe GameScene coordinates as the matching UI layout. |
| System naming | Name changed runtime system functions as `[domain]_[schedule]_system`, for example `player_update_system`. |
| Assets | Put runtime assets in `bevy/crates/game/assets`; put theme-owned cards, locations, and worlds in `assets/themes/theme_<theme_name>/{cards,locations,worlds}` with category-prefixed folders; keep custom shaders in `bevy/crates/game/assets/shaders`. |
| Feature wiring | Prefer feature plugins over direct app wiring in entrypoints. |
| Hot reload | Mark hot-reloadable update systems with `#[hot]` when the workflow supports it. |
| Tests | Keep tests aligned with feature and plugin behavior, close to the runtime code they validate. |
| Path casing | Keep this repo's lowercase Rust-style paths unless a separate migration updates the constitution, scripts, specs, and docs. |
| AI runtime peek | For a request to "peek" at the running app, use local BRP on `http://localhost:15702` plus `bevy_debugger/screenshot`; save transient screenshots under `target/ai-runtime-screenshots/`. |

## Screen Hierarchy Reports

Use `Target` for the user-facing screen or screen-like concept being described. Treat target names as labels, not ECS ownership claims. Under the target, list concrete runtime scenes and key spawned children; suffix meaningful visual entities with `(2D)` or `(3D)`.

```text
Target: Blah
├─ Scene: Foo
│  ├─ Bar (3D)
│  └─ FooFoo (2D)
```

## Reference Source

Imported from the structure guidance in `https://github.com/SamuelAsherRivello/bevy-jam-1` and adapted to this repository's existing lowercase `bevy/crates` layout.
The local `bevy/crates/template-crate` skeleton is the project-owned reference to use for current folder, file, asset, and Rust coding standards.
