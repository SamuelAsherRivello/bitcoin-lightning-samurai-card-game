# Implementation Plan: Visual Modifier System

**Branch**: `016-visual-modifiers` | **Date**: 2026-05-13 | **Spec**: [spec.md](./spec.md)  
**Input**: Feature specification from `specs/016-visual-modifiers/spec.md`

## Summary

Create the first Visual Modifier System pass for point views. The system will compute named visual modifier state separately from point value calculation, then synchronize outline presentation onto the circle/background child of each affected `PointView`. The initial modifiers are `abilityoutline`, a gold outline for card power point views modified by an active non-zero ability, and `leadingscoreoutline`, a white outline for the higher location total within each location.

## Technical Context

**Language/Version**: Rust in the existing Bevy workspace  
**Primary Dependencies**: Existing Bevy ECS runtime, Bevy UI, Bevy mesh/material rendering; no new dependency planned  
**Storage**: Transient runtime ECS components/resources only; no persistence or schema change  
**Testing**: `scripts/other/RunTests.ps1`; targeted component/system tests under `bevy/crates/game/src/tests/runtime/`  
**Target Platform**: Windows desktop and browser WebGPU parity  
**Project Type**: Bevy ECS game runtime under `bevy/crates/game`  
**Performance Goals**: O(point views + occupied slots) synchronization per update; avoid expensive hierarchy scans beyond point view children  
**Constraints**: Preserve point value text behavior, card face visibility, aspect-ratio-safe GameView layout, and existing point view ownership; outlines must attach to circle/background visuals  
**Scale/Scope**: First pass supports `abilityoutline` for card power point views and second pass supports `leadingscoreoutline` for location total point views

## Constitution Check

*GATE: Must pass before Phase 0 research. Re-check after Phase 1 design.*

| Check | Status | Notes |
| ----- | ------ | ----- |
| Active spec and repo guidance | ✅ | Plan follows `specs/016-visual-modifiers/spec.md`, constitution 1.6.1, and AGENTS.md. |
| Source/assets/scripts/docs locations | ✅ | Planning artifacts stay under `specs/016-visual-modifiers`; future runtime changes stay under `bevy/crates/game/src/runtime`. |
| Bevy template reference | ✅ | Future implementation should inspect `bevy/crates/template-crate` before adding runtime files. |
| Rust naming conventions | ✅ | Proposed modules and items use Rust/Bevy naming conventions and lowercase paths. |
| One primary runtime concept per file | ✅ | Proposed files split modifier model/component state from synchronization systems. |
| HUMAN/AI purpose comments | ✅ | Required for future primary runtime items. |
| Runtime system naming | ✅ | Future systems should use names such as `visual_modifier_update_system` and `point_view_visual_modifier_sync_system`. |
| Scene/Model/View naming | ✅ | VMS uses modifier data/model state and point view presentation without renaming app scenes. |
| Theme asset organization | ✅ | No theme assets are introduced. |
| Visible feedback | ✅ | The feature is visible feedback for ability-modified points and leading scores. |
| Browser/native storage constraints | ✅ | No storage, database, localStorage, SQLite, or OPFS change. |
| Browser-visible verification path | ✅ | Verify through the GameView desktop run and, when practical, browser WebGPU render path. |
| Aspect-ratio-safe layout | ✅ | Outlines are children of existing point views, whose positions already derive from GameView layout. |
| Framework constraints documented | ✅ | Bevy UI borders and world-space mesh outline differences are captured below. |

## Project Structure

### Documentation (this feature)

```text
specs/016-visual-modifiers/
├── spec.md
├── plan.md
├── research.md
├── data-model.md
├── quickstart.md
└── contracts/
    └── visual-modifier-contract.md
```

### Source Code (repository root)

```text
bevy/crates/game/src/runtime/
├── bundles/
│   └── point_view_bundle.rs
├── components/
│   ├── mod.rs
│   └── point_view_visual_modifier_component.rs
├── resources/
│   ├── card_slot_model.rs
│   └── game_location_model.rs
└── systems/
    ├── mod.rs
    ├── point_view_visual_modifier_sync_system.rs
    └── visual_modifier_update_system.rs

bevy/crates/game/src/tests/runtime/
├── components/
│   └── point_view_visual_modifier_component_tests.rs
└── systems/
    └── visual_modifier_update_system_tests.rs
```

**Structure Decision**: Keep VMS inside the game runtime because modifier outlines are gameplay-facing presentation, not reusable shared tooling. Add focused visual modifier component/system files if implementation would otherwise grow aggregate `mod.rs` files further. Keep numeric point calculation in existing point systems and treat VMS as presentation metadata derived from current ability and score state.

## Phase 0 Research

Research is complete in [research.md](./research.md). Key decisions:

| Decision | Outcome |
| -------- | ------- |
| Modifier state is separate from values | Point systems continue computing values; VMS computes named presentation modifiers from the same resolved state. |
| Use named modifiers | `AbilityOutline` and `LeadingScoreOutline` are explicit enum variants rather than implicit colors. |
| Mark circle children | Add a background/circle marker so systems can update outlines without relying on child names. |
| Support two render surfaces | Location point views can use UI border presentation; card point views need a world-space ring/outline presentation around the mesh circle. |

## Phase 1 Design

| Artifact | Purpose |
| -------- | ------- |
| [data-model.md](./data-model.md) | Defines modifier entities, state ownership, validation, and update rules. |
| [contracts/visual-modifier-contract.md](./contracts/visual-modifier-contract.md) | Defines modifier inputs, outputs, colors, and acceptance samples. |
| [quickstart.md](./quickstart.md) | Defines implementation and verification workflow. |

## Implementation Approach

| Area | Plan |
| ---- | ---- |
| Modifier representation | Add a `VisualModifier` enum and a compact `PointViewVisualModifiers` component or equivalent model on point view roots. |
| Circle targeting | Add a marker component to point view circle/background children so sync systems can update the intended visual element directly. |
| Ability outline | During card power point update or a dedicated VMS pass, detect whether the card is in a local/opponent location with a non-zero active ability delta and set `AbilityOutline`. |
| Leading score outline | Compare local and opponent `PointLocationView` values for each location and set `LeadingScoreOutline` only on the higher total. |
| Presentation sync | Render `AbilityOutline` as gold and `LeadingScoreOutline` as white around the circle/background, preserving text color and value. |
| Testing | Add pure modifier-state tests first, then ECS sync tests for card ability state and location leading/tie transitions. |

## Post-Design Constitution Check

| Check | Status | Notes |
| ----- | ------ | ----- |
| Source remains scoped | ✅ | Proposed future changes stay in focused runtime component/system files and point view bundle updates. |
| Desktop/browser parity addressed | ✅ | No platform-specific dependency is proposed; WebGPU-compatible Bevy primitives are expected. |
| Aspect-ratio-safe layout addressed | ✅ | Outlines are attached to existing point view children and do not introduce independent placement. |
| Data changes explicit | ✅ | Modifier state is transient runtime ECS state only. |
| Framework constraints recorded | ✅ | UI and world-space point view presentation differences are part of the contract. |

## Complexity Tracking

No constitution violations require justification.
