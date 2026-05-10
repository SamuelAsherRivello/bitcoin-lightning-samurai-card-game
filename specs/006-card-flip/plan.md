# Implementation Plan: Card Flip

**Branch**: `006-card-flip` | **Date**: 2026-05-10 | **Spec**: [spec.md](./spec.md)
**Input**: Feature specification from `specs/006-card-flip/spec.md`

## Summary

Add a `Flip` button to the temporary Card UI in the current `CardBrowser` prototype entry point. The button toggles the single inspectable card between CardFront and CardBack through a smooth y-axis flip. The implementation will layer flip state onto the existing pointer-driven inspection rotation, reverse direction when clicked mid-animation, swap visible face graphics at the edge-on midpoint, and add one shared superhero-pattern backface asset under `bevy/crates/game/assets/cards/CardStructure/`.

## Technical Context

**Language/Version**: Rust 2024 workspace  
**Primary Dependencies**: Bevy 0.18.1, bevy-inspector-egui 0.36.0, bevy-persistent 0.10.0, existing `bevy_card_game_shared` crate  
**Storage**: Runtime PNG backface asset under `bevy/crates/game/assets/cards/CardStructure/`; no persisted flip state  
**Testing**: `scripts/other/RunTests.ps1`; desktop smoke with `scripts/other/RunAppDesktop.ps1`; browser WebGPU smoke with `scripts/other/RunAppWeb.ps1` when target/tooling is available  
**Target Platform**: Windows desktop and browser WebGPU parity  
**Project Type**: Bevy ECS game runtime in `bevy/crates/game` with shared window support in `bevy/crates/shared`  
**Performance Goals**: Flip input is visible within one frame, animation remains smooth during pointer movement, and scene remains one centered card  
**Constraints**: Preserve existing CardBrowser entry point and CardFront multi-layer setup, keep temporary Card UI separate from DebugHUD, keep CardBack card-series-level and independent of the active card front, make the CardBack palette/style compatible with existing fronts, avoid words/characters/clear symbols on CardBack, swap face graphics at the flip midpoint, keep assets under approved card asset directories, avoid gameplay or final user-facing UI scope expansion  
**Scale/Scope**: One inspectable card in CardBrowser, one shared CardBack design, one Card UI button, active card-front switching with `T`, no new menus, tabletop, game box cover, main menu art, or multi-card systems

## Constitution Check

*GATE: Must pass before Phase 0 research. Re-check after Phase 1 design.*

| Check | Status | Notes |
| ----- | ------ | ----- |
| Active spec, constitution, and repo guidance followed | ✅ | Work is scoped to `006-card-flip` and AGENTS guidance. |
| Source, assets, scripts, docs, and tests stay in defined locations | ✅ | Runtime code remains under `bevy/crates/game`; asset goes under `bevy/crates/game/assets/cards/CardStructure/`. |
| Rust workspace folders and files use Rust naming conventions | ✅ | New Rust modules/assets should use lowercase filenames where created; existing `CardStructure` asset folder is project-approved. |
| Visible loading/toast feedback for async data/cache/database work | ✅ | Not applicable; feature has no async data/cache/database workflow. |
| Browser builds keep localStorage snapshots and avoid browser SQLite/OPFS startup | ✅ | No browser storage or database changes. |
| Native database/schema/seed setup remains isolated | ✅ | No database changes. |
| Browser-visible changes have served-web verification path | ✅ | `scripts/other/RunAppWeb.ps1` is the planned browser WebGPU path. |
| Selected language and framework standards followed | ✅ | Bevy ECS resources/components/systems/plugins remain the implementation shape. |
| Framework-specific API constraints documented | ✅ | Plan records Bevy 0.18 ECS scheduling, egui Card UI, transform composition, and asset loading constraints. |

## Project Structure

### Documentation (this feature)

```text
specs/006-card-flip/
├── plan.md
├── research.md
├── data-model.md
├── quickstart.md
├── contracts/
│   └── card-flip-ui.md
├── checklists/
│   └── requirements.md
└── tasks.md
```

### Source Code (repository root)

```text
bevy/crates/game/
├── assets/
│   └── cards/
│       └── CardStructure/
│           └── card_back_superhero_pattern.png
└── src/
    └── runtime/
        ├── components/
        │   └── mod.rs
        ├── plugins/
        │   └── mod.rs
        ├── resources/
        │   └── mod.rs
        └── systems/
            └── mod.rs

scripts/
├── main/
│   └── RunAppDesktopHotReload.ps1
└── other/
    ├── RunTests.ps1
    ├── RunAppDesktop.ps1
    ├── RunAppWeb.ps1
    └── StopApp.ps1
```

**Structure Decision**: Keep flip behavior inside the existing Bevy game runtime ECS structure and current CardBrowser scene. Add the Flip control to the temporary Card UI rather than DebugHUD. Add state to runtime resources, marker/role data to runtime components only if needed for face visibility, behavior to runtime systems, scheduling to runtime plugins, and the shared superhero-pattern backface under the existing project-approved `CardStructure` asset directory.

## Phase 0: Research

See [research.md](./research.md). All technical unknowns are resolved with project-local decisions.

## Phase 1: Design & Contracts

See [data-model.md](./data-model.md), [contracts/card-flip-ui.md](./contracts/card-flip-ui.md), and [quickstart.md](./quickstart.md).

## Post-Design Constitution Check

| Check | Status | Notes |
| ----- | ------ | ----- |
| Active spec, constitution, and repo guidance followed | ✅ | Design artifacts map directly to spec requirements. |
| Source, assets, scripts, docs, and tests stay in defined locations | ✅ | No additional locations introduced. |
| Rust workspace folders and files use Rust naming conventions | ✅ | Rust code remains in existing lowercase module files. |
| Visible feedback requirement | ✅ | Not applicable to this synchronous UI animation. |
| Browser storage/database constraints | ✅ | No storage/database changes. |
| Browser-visible verification path | ✅ | Quickstart includes desktop and browser verification. |
| Framework standards and API constraints | ✅ | Design preserves Bevy ECS ownership and schedule ordering. |

## Complexity Tracking

No constitution violations require complexity tracking.
