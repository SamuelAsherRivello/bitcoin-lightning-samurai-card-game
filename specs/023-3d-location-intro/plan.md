# Implementation Plan: 3D Location Intro

**Branch**: `[023-3d-location-intro]` | **Date**: 2026-05-17 | **Spec**: [spec.md](spec.md)  
**Input**: Feature specification from `specs/023-3d-location-intro/spec.md`

## Summary

Convert the visible GameScene locations into a combined `location_bundle` presentation: a 3D textured rectangle in front of the world background, with the current title/body, colored border, and two location point views overlaid in the safe-area UI. Add a start-of-game intro that fades and scales locations in sequence: location 01, wait 0.5 seconds, location 02, wait 0.5 seconds, location 03.

## Technical Context

**Language/Version**: Rust 2024, Bevy 0.18.1  
**Primary Dependencies**: Bevy, existing Bevy UI, existing Bevy 3D mesh/material rendering, existing theme location textures  
**Storage**: N/A  
**Testing**: Focused `cargo test -p samurai-card-game` tests plus `scripts/other/RunTests.ps1` when broader validation is practical
**Target Platform**: Native desktop and browser-compatible Bevy GameScene  
**Project Type**: Bevy ECS game runtime in `bevy/crates/game`  
**Performance Goals**: Three location bundles update once per frame during the 4-second intro window, then remain stable with negligible runtime cost  
**Constraints**: Existing cards render in the card overlay camera above the world/UI layer; locations must stay in front of the world background and below cards; visible positions derive from the aspect-ratio-safe GameScene layout  
**Scale/Scope**: Three shared GameScene locations only; no scoring, deck, round, or card behavior changes

## Constitution Check

| Gate | Status | Notes |
| ---- | ------ | ----- |
| Active spec and repo-local guidance | ✅ | Spec is `specs/023-3d-location-intro/spec.md`; implementation remains scoped to GameScene location presentation. |
| Source/assets/scripts/docs/tests locations | ✅ | Runtime source stays under `bevy/crates/game/src/runtime`; docs stay under `specs/023-3d-location-intro`. |
| Bevy template and Rust naming conventions | ✅ | New names use lowercase module/file naming and existing bundle/component/system patterns. |
| One primary concept per changed runtime file | ✅ | Existing large `systems/mod.rs` owns current GameScene spawning; new component markers stay with existing GameScene components. |
| HUMAN/AI purpose comments | ✅ | New primary runtime items include terse `HUMAN:` and `AI:` comments. |
| System naming | ✅ | New runtime system is `location_intro_update_system`. |
| Scene/Model/View naming | ✅ | `GameScene` remains the view; `LocationBundle` is presentation state, not a scene. |
| Theme asset placement | ✅ | Reuses existing location textures from the theme registries. |
| Browser-visible verification path | ✅ | Runtime can be checked through the existing desktop/browser GameScene workflows. |
| Safe-area 2D/3D positioning | ✅ | 3D rectangle size and position derive from `CardSlotBoardModel::location_area_rect`. |
| Framework API constraints documented | ✅ | Bevy UI overlay opacity and Bevy 3D material opacity are updated together by component markers. |

## Project Structure

### Documentation

```text
specs/023-3d-location-intro/
├── spec.md
├── plan.md
├── research.md
├── data-model.md
├── quickstart.md
├── contracts/
│   └── location-intro-contract.md
├── checklists/
│   └── requirements.md
└── tasks.md
```

### Source Code

```text
bevy/crates/game/src/runtime/
├── components/mod.rs
├── systems/mod.rs
└── plugins/core_game_plugin.rs

bevy/crates/game/src/tests/runtime/systems/
└── systems_mod_tests.rs
```

**Structure Decision**: Keep the implementation in the current GameScene spawning and update paths because the latest code owns location spawning in `systems/mod.rs`. Add minimal component markers in `components/mod.rs` and wire the update system in `core_game_plugin.rs`.

## Complexity Tracking

| Violation | Why Needed | Simpler Alternative Rejected Because |
| --------- | ---------- | ------------------------------------ |
| None | N/A | N/A |
