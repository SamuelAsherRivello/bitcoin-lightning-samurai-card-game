# Implementation Plan: Meta Game Flow

**Branch**: `019-meta-game-flow` | **Date**: 2026-05-14 | **Spec**: [spec.md](spec.md)  
**Input**: Feature specification from `/specs/019-meta-game-flow/spec.md`

## Summary

Introduce a meta-game app shell that starts on MainMenuScreen, uses reusable top navigation across Play Game, My Decks, Settings, and Debug, adds a placeholder LightningScreen, fake matchmaking before GameScreen, and moves pre-game match/audio settings into SettingsScreen with disk persistence. The implementation extends the existing Bevy ECS runtime rather than replacing the current GameScene, DeckScene, and DebugScene behavior.

## Technical Context

**Language/Version**: Rust with the repository-pinned toolchain in `rust-toolchain.toml`  
**Primary Dependencies**: Bevy, `bevy_persistent`, existing `bevy_aspect_ratio_mask`, existing card/theme assets  
**Storage**: Local JSON persistence under `data/local_storage/` using existing Bevy persistent patterns  
**Testing**: `scripts/other/RunTests.ps1` plus focused `cargo test -p samurai_card_game` when iterating
**Target Platform**: Windows desktop and browser WebGPU  
**Project Type**: Bevy ECS game workspace under `bevy/crates/game`  
**Performance Goals**: Screen reload/navigation should complete in a single update tick except fake matchmaking timers  
**Constraints**: Preserve AppScene parenting, aspect-ratio-safe layout, existing deck persistence, and existing debug/Card UI behavior  
**Scale/Scope**: Five user-facing meta-game screens plus the existing GameScreen, no real Lightning auth, no shop purchasing, no backend matchmaking

## Constitution Check

*GATE: Must pass before Phase 0 research. Re-check after Phase 1 design.*

| Check | Status | Notes |
| ---- | ---- | ---- |
| Active spec, constitution, and repo guidance followed | ✅ | Feature lives under `specs/019-meta-game-flow` and Bevy source stays under `bevy/crates/game`. |
| Source/assets/scripts/docs/tests stay in project locations | ✅ | Runtime source remains in `bevy/crates/game/src/runtime`; persistence stays under `data/local_storage/`. |
| Template crate used as reference | ✅ | Existing runtime folders follow the local template crate structure. |
| Rust naming conventions | ✅ | New modules use lowercase `snake_case`. |
| One primary runtime concept per changed file | ✅ | New screen/settings models and screen components are split by concept. |
| HUMAN/AI comments on primary runtime items | ✅ | Required for new/changed runtime items. |
| System naming follows `[domain]_[schedule]_system` | ✅ | New systems use domain/schedule naming. |
| Scene/Model/View naming | ✅ | User-facing screens map to `ActiveView`; persistent `AppScene` remains unchanged. |
| Theme asset organization | ✅ | Existing theme assets are reused; no new theme asset folders required. |
| Visible feedback for asynchronous work | ✅ | Matchmaking has visible Searching and found phases. |
| Browser storage boundaries | ✅ | Uses local JSON persistence pattern; no browser SQLite/OPFS worker changes. |
| Native database setup | ✅ | No database changes. |
| Served-web verification path | ✅ | Existing `scripts/other/RunAppWeb.ps1` remains the browser verification path. |
| Framework standards documented | ✅ | Bevy ECS resources/components/systems and persistent store patterns are used. |
| Aspect-ratio-safe positions | ✅ | UI is spawned under safe HUD/app content roots with 1280x800 virtual layout. |

## Project Structure

### Documentation (this feature)

```text
specs/019-meta-game-flow/
├── spec.md
├── plan.md
├── research.md
├── data-model.md
├── quickstart.md
├── contracts/
│   └── meta-game-flow-ui-contract.md
└── tasks.md
```

### Source Code (repository root)

```text
bevy/crates/game/src/runtime/
├── components/
│   ├── meta_screen_component.rs
│   └── top_navigation_component.rs
├── resources/
│   ├── meta_game_settings_model.rs
│   ├── matchmaking_model.rs
│   ├── top_navigation_model.rs
│   └── mod.rs
├── scenes/
│   ├── main_menu_scene.rs
│   ├── lightning_scene.rs
│   ├── matchmaking_scene.rs
│   ├── settings_scene.rs
│   ├── game_scene.rs
│   ├── deck_scene.rs
│   └── debug_scene.rs
├── systems/
│   └── mod.rs
└── plugins/
    └── mod.rs

bevy/crates/game/src/tests/runtime/
├── resources/
├── systems/
└── components/
```

**Structure Decision**: Extend the existing Bevy crate in place. The persistent AppScene remains the app-level scene; MainMenu, Lightning, Matchmaking, Settings, Game, Deck, and Debug are active child views/screens.

## Complexity Tracking

No constitution violations are expected.

## Phase 0: Research

Research decisions are captured in [research.md](research.md).

## Phase 1: Design & Contracts

Data model is captured in [data-model.md](data-model.md). The user-facing UI behavior contract is captured in [contracts/meta-game-flow-ui-contract.md](contracts/meta-game-flow-ui-contract.md). Quickstart verification is captured in [quickstart.md](quickstart.md).

## Post-Design Constitution Check

| Check | Status | Notes |
| ---- | ---- | ---- |
| Specification and implementation plan remain aligned | ✅ | Stories map directly to runtime models, navigation, timers, and persistence. |
| No destructive data or Git operations required | ✅ | Persistence writes are non-destructive settings updates. |
| Desktop/browser parity remains planned | ✅ | Browser opening for Learn About Lightning may be target-specific but is non-critical and documented. |
