# Implementation Plan: Point System

**Branch**: `010-point-system` | **Date**: 2026-05-11 | **Spec**: `specs/010-point-system/spec.md`
**Input**: Feature specification from `specs/010-point-system/spec.md`

## Summary

Define the card and location point model for the Bevy card game so card cost, card power, effective power, shared location totals, location control, and final match outcome all use one consistent gameplay vocabulary. The implementation should add focused Rust/Bevy ECS models and presentation views for cost and power points, keep base card data separate from match-state card instances, calculate location totals from revealed effective power, compare three shared locations from the local player's perspective, and verify the rules with deterministic tests before wiring visible point displays into `GameView`.

## Technical Context

| Item | Decision |
| ---- | -------- |
| Language/Version | Rust 2024 workspace |
| Primary Dependencies | Bevy 0.18.1 and existing workspace crates; no new gameplay dependency is required for point arithmetic |
| Storage | N/A; point values are source/runtime model data, not persisted user data |
| Testing | `scripts/other/RunTests.ps1` for workspace tests; targeted `cargo test` may be used during iteration when scoped to the Bevy game crate |
| Target Platform | Windows desktop and browser WebGPU |
| Project Type | Bevy ECS desktop/browser game prototype |
| Performance Goals | Point totals, control, and match outcome are small deterministic calculations over three locations and up to four cards per player per location; recomputation should be frame-cheap and allocation-light |
| Constraints | Keep Bevy runtime code under `bevy/crates/game/src/runtime/`; keep point data as `Model` concepts and point rendering as `View` concepts; keep top opponent total and bottom local total positioned from the aspect-ratio-safe `GameView`; do not implement full deckbuilding, draw, energy, CPU, complete turn rules, abilities, or production UI |
| Scale/Scope | Three shared locations, two players, up to four cards per player per location, rounds 1 through 6, display contract for values from `-99` through `99`, deterministic control and match-outcome rules |

## Constitution Check

| Gate | Status | Notes |
| ---- | ------ | ----- |
| Active spec and repo guidance followed | ✅ | Plan follows `010-point-system`, `007-gameplay-concepts`, constitution 1.6.0, and repo-local AGENTS guidance |
| Source, assets, scripts, docs, and tests stay in approved locations | ✅ | Runtime code belongs under `bevy/crates/game/src/runtime/`; scripts stay under `scripts/`; no new non-runtime asset path is required |
| Rust naming and path casing | ✅ | New modules should use lowercase `snake_case`; no `Bevy/Crates/...` paths |
| One primary runtime concept per file | ✅ | Planned files separate point models, match scoring model, point view components/bundles, and update systems |
| Required `HUMAN:` and `AI:` comments | ✅ | Any changed or new primary runtime items must include the two-line purpose comment immediately above the item |
| Runtime system naming | ✅ | New systems should use names such as `point_view_update_system` and `location_score_update_system` |
| Scene/Model/View naming | ✅ | Point data uses `CostPointModel`, `PowerPointModel`, `CardInstanceModel`, `LocationScoreModel`, and match outcome models; rendering uses `CostPointView` and `PowerPointView` |
| Theme asset organization | ✅ | No new theme-owned card/location/world assets are required; if point backgrounds become image assets later, they must follow theme or shared asset ownership explicitly |
| Visible user feedback | ✅ | Not an asynchronous workflow; visible feedback is the rendered cost/power/location point display |
| Desktop and browser WebGPU parity | ✅ | Point calculations are platform-neutral; visible `GameView` point placement must be verified on desktop and browser WebGPU or blocked with reason |
| Aspect-ratio-safe layout | ✅ | Location totals and card point displays must derive placement from existing safe `GameView` layout rather than raw window pixels |
| Framework-specific API constraints documented | ✅ | Bevy UI/Text and ECS scheduling should be used through existing project patterns; no external UI framework is planned |

## Project Structure

```text
specs/010-point-system/
├── plan.md
├── research.md
├── data-model.md
├── quickstart.md
├── contracts/
│   └── point-system-contract.md
└── tasks.md              # Created by /speckit-tasks, not by this plan

bevy/crates/game/src/runtime/
├── components/
│   ├── mod.rs            # Expose point view components/bundles if small, or re-export focused files
│   └── point_view_component.rs
├── resources/
│   ├── mod.rs            # Card/location registries may gain point fields or re-export focused files
│   └── point_model.rs
├── systems/
│   ├── mod.rs            # Wire point scoring/view systems into existing GameView flow
│   └── point_view_update_system.rs
└── plugins/
    └── mod.rs            # Wire a feature plugin only if existing app composition benefits from it

scripts/
└── other/
    └── RunTests.ps1
```

**Structure Decision**: Keep point-system implementation in the game crate because card cost, card power, location totals, control, and match outcome are gameplay concepts rather than reusable shared tooling. Prefer focused `point_model` and `point_view_component` modules if the change is broad enough to avoid growing the existing aggregate `mod.rs` files further. Keep scoring calculations deterministic and testable as model/resource functions before using systems to synchronize visible views.

## Complexity Tracking

| Violation | Why Needed | Simpler Alternative Rejected Because |
| --------- | ---------- | ------------------------------------ |
| N/A | N/A | N/A |

## Phase 0 Research

See `specs/010-point-system/research.md`.

## Phase 1 Design

See `specs/010-point-system/data-model.md`, `specs/010-point-system/contracts/point-system-contract.md`, and `specs/010-point-system/quickstart.md`.

## Post-Design Constitution Check

| Gate | Status | Notes |
| ---- | ------ | ----- |
| No unresolved clarifications | ✅ | The spec already resolves the major point rules: cost vs power, reveal contribution, top/bottom totals, ties, tiebreakers, range, and scope exclusions |
| No target-specific code required | ✅ | Scoring is pure Rust model behavior; rendering uses existing Bevy UI/GameView conventions for both desktop and browser |
| Constitution implementation standards reflected | ✅ | Planned runtime concepts use Model/View naming, focused files, required purpose comments, and `[domain]_[schedule]_system` system names |
| Verification documented | ✅ | Unit tests, desktop run, and browser WebGPU verification are documented in quickstart |
