# Implementation Plan: Game Theme POC

**Branch**: `[008-game-theme-poc]` | **Date**: 2026-05-10 | **Spec**: [spec.md](./spec.md)  
**Input**: Feature specification from `/specs/008-game-theme-poc/spec.md`

**Note**: This template is filled in by the `/speckit-plan` command. See `.specify/templates/plan-template.md` for the execution workflow.

## Summary

Permanently replace the existing game-scene card lineup and world background with a Japan Realism tactical theme proof-of-concept. The implementation will extend the existing Bevy ECS runtime rather than adding a parallel UI stack: new card/world/location assets live under `bevy/crates/game/assets`, card identities replace the current SkyBolt/Tar registry entries, the game scene displays all four POC cards along the bottom, `T` cycles Bamboo Forest and Coastal Harbor worlds in the game scene, card clicks open the existing Deck focused on the clicked card, and CardUI settings remain global while flip state stays temporary.

## Technical Context

**Language/Version**: Rust 2024  
**Primary Dependencies**: Bevy 0.18.1, `bevy_aspect_ratio_mask`, `bevy-inspector-egui`, `bevy-persistent`, `serde`, `serde_json`  
**Storage**: Existing JSON persistence under `data/local_storage/card-settings.json` for global CardUI settings; no persistent flip state  
**Testing**: `scripts/other/RunTests.ps1`, plus `scripts/other/RunAppDesktop.ps1 -CheckOnly` and `scripts/other/RunAppWeb.ps1 -CheckOnly` for target checks  
**Target Platform**: Windows desktop and browser WebGPU parity  
**Project Type**: Bevy ECS game runtime in the existing Cargo workspace  
**Performance Goals**: Game scene and Deck remain responsive during card tilt, flip animation, world swaps, and card navigation; asset count remains bounded to the four cards, two worlds, and six reusable tactical locations  
**Constraints**: Use lowercase `snake_case` paths under `bevy/`; keep runtime assets under `bevy/crates/game/assets`; preserve the existing scene and ECS role boundaries; create new art for the 008 theme; permanently replace pre-008 card/world visuals; keep CardUI settings separate from world theme; do not persist card flip state  
**Scale/Scope**: Four bottom cards, two world backgrounds, six reusable location identities, three visible locations per world swap, one focused Deck card at a time

## Constitution Check

*GATE: Must pass before Phase 0 research. Re-check after Phase 1 design.*

| Gate | Status | Notes |
| ---- | ------ | ----- |
| Active spec, constitution, and repo-local guidance followed | PASS | Plan uses `specs/008-game-theme-poc/spec.md`, AGENTS guidance, and `.codex/rules/bevy-runtime-structure.md`. |
| Source, assets, scripts, docs, and tests stay in defined locations | PASS | Runtime changes stay under `bevy/crates/game`; assets stay under `bevy/crates/game/assets`; verification uses `scripts/`. |
| Rust workspace paths use lowercase conventions | PASS | New asset directories use lowercase names such as `card_type_kage_ren` and `worlds/bamboo_forest`. |
| Visible feedback preserved | PASS | Feature does not change template loading/database workflows; Deck scene transitions remain visibly immediate. |
| Browser build constraints preserved | PASS | No browser SQLite, OPFS worker, or unrelated storage changes are introduced. |
| Native database setup preserved | PASS | Feature does not touch database setup. |
| Browser-visible verification path exists | PASS | Use `scripts/other/RunAppWeb.ps1 -CheckOnly`; full browser smoke remains a Phase 2 verification task. |
| Language and framework standards followed | PASS | Bevy ECS ownership remains in components/resources/systems/plugins. |
| Framework-specific constraints documented | PASS | The plan documents Bevy 0.18.1, ECS ownership, asset paths, and desktop/browser parity. |

## Project Structure

### Documentation (this feature)

```text
specs/008-game-theme-poc/
├── plan.md
├── research.md
├── data-model.md
├── quickstart.md
├── contracts/
│   └── ui-behavior-contract.md
└── tasks.md
```

### Source Code (repository root)

```text
bevy/crates/game/
├── assets/
│   ├── cards/
│   │   ├── card_structure/
│   │   │   └── card_back_japan_realism.png
│   │   └── card_types/
│   │       ├── card_type_kage_ren/
│   │       ├── card_type_lord_daichi/
│   │       ├── card_type_sister_hotaru/
│   │       └── card_type_yokai_placeholder/
│   ├── locations/
│   │   ├── fortress_gate/
│   │   ├── bamboo_crossing/
│   │   ├── shrine_ruins/
│   │   ├── battlefield/
│   │   ├── spirit_well/
│   │   └── market_square/
│   └── worlds/
│       ├── bamboo_forest/
│       │   └── world_background.png
│       └── coastal_harbor/
│           └── world_background.png
└── src/runtime/
    ├── components/mod.rs
    ├── plugins/mod.rs
    ├── resources/mod.rs
    └── systems/mod.rs

bevy/crates/shared/
└── src/window.rs

scripts/
├── main/
│   └── RunAppDesktopHotReload.ps1
└── other/
    ├── RunTests.ps1
    ├── RunAppDesktop.ps1
    ├── RunAppWeb.ps1
    └── StopApp.ps1
```

**Structure Decision**: Use the existing Bevy ECS runtime in `bevy/crates/game`. Use `bevy/crates/template-crate` as the proper reference for Bevy crate folders, representative files, asset folders, and Rust coding standards. Card and world definitions belong in `resources`; marker/data components remain in `components`; behavior and scene setup remain in `systems`; plugin wiring remains in `plugins`. New bitmap assets replace the current superhero/minotaur/desert visuals under the existing runtime asset root.

## Complexity Tracking

No constitution violations are planned.
