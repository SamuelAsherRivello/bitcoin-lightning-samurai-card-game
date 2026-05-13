# Implementation Plan: Visual Modifier System

**Branch**: `016-visual-modifiers` | **Date**: 2026-05-13 | **Spec**: [spec.md](./spec.md)  
**Input**: Feature specification from `specs/016-visual-modifiers/spec.md`

## Summary

Create the first Visual Modifier System pass for point views using a reusable visual modification rule model. Each rule has a Condition, Target, and Treatment: the Condition decides whether the rule is active, the Target selects the UI/render element to modify, and the Treatment applies the visual change. The initial rules are `abilityoutline`, where modified card power targets a card power point circle with a gold outline, and `leadingscoreoutline`, where the leading location total targets that location total point circle with a white outline.

## Technical Context

**Language/Version**: Rust in the existing Bevy workspace  
**Primary Dependencies**: Existing Bevy ECS runtime, Bevy UI, Bevy mesh/material rendering; no new dependency planned  
**Storage**: Transient runtime ECS components/resources only; no persistence or schema change  
**Testing**: `scripts/other/RunTests.ps1`; targeted component/system tests under `bevy/crates/game/src/tests/runtime/`  
**Target Platform**: Windows desktop and browser WebGPU parity  
**Project Type**: Bevy ECS game runtime under `bevy/crates/game`  
**Performance Goals**: O(point views + occupied slots) synchronization per update; avoid expensive hierarchy scans beyond point view children  
**Constraints**: Preserve point value text behavior, 015 card face/reveal visibility behavior, aspect-ratio-safe GameView layout, and existing point view ownership; rule Treatments must attach to explicit rule Targets rather than ad hoc child/name matches.
**Scale/Scope**: First pass supports `abilityoutline` for card power point views and second pass supports `leadingscoreoutline` for location total point views

## Constitution Check

*GATE: Must pass before Phase 0 research. Re-check after Phase 1 design.*

| Check | Status | Notes |
| ----- | ------ | ----- |
| Active spec and repo guidance | ✅ | Plan follows `specs/016-visual-modifiers/spec.md`, constitution 1.6.1, and AGENTS.md. |
| Source/assets/scripts/docs locations | ✅ | Planning artifacts stay under `specs/016-visual-modifiers`; future runtime changes stay under `bevy/crates/game/src/runtime`. |
| Bevy template reference | ✅ | Future implementation should inspect `bevy/crates/template-crate` before adding runtime files. |
| Rust naming conventions | ✅ | Proposed modules and items use Rust/Bevy naming conventions and lowercase paths. |
| One primary runtime concept per file | ✅ | Proposed files split rule model/component state from synchronization systems. |
| HUMAN/AI purpose comments | ✅ | Required for future primary runtime items. |
| Runtime system naming | ✅ | Future systems should use names such as `visual_modifier_update_system` and `point_view_visual_modifier_sync_system`. |
| Scene/Model/View naming | ✅ | VMS uses modifier model state, `CardInstanceStateModel`, and point view presentation without renaming app scenes. |
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
│   ├── card_instance_state_model.rs
│   ├── card_slot_model.rs
│   └── game_location_model.rs
└── systems/
    ├── mod.rs
    └── visual_modifier_update_system.rs

bevy/crates/game/src/tests/runtime/
├── components/
│   └── point_view_visual_modifier_component_tests.rs
└── systems/
    └── visual_modifier_update_system_tests.rs
```

**Structure Decision**: Keep VMS inside the game runtime because visual modifications are gameplay-facing presentation, not reusable shared tooling. Add focused visual modifier rule/component/system files if implementation would otherwise grow aggregate `mod.rs` files further. Keep numeric point calculation in existing point systems and treat VMS as Condition evaluation plus Target/Treatment synchronization derived from current ability, score state, and the completed 015 `CardInstanceStateModel`/`CardViewStateModel` surface.

## Phase 0 Research

Research is complete in [research.md](./research.md). Key decisions:

| Decision | Outcome |
| -------- | ------- |
| Use Condition/Target/Treatment rules | Every visual modification is represented as why it activates, what it changes, and how it changes it. |
| Modifier state is separate from values | Point systems continue computing values; VMS evaluates Conditions from the same resolved state. |
| Use named treatments | `AbilityOutline` and `LeadingScoreOutline` are explicit Treatment names rather than implicit colors. |
| Use 015 card-state identity | Card ability outline derivation should key off `CardInstanceId` and `CardZoneModel::Location` when available, not a new VMS-specific placement model. |
| Mark circle children | Add a background/circle marker so systems can update outlines without relying on child names. |
| Support two render surfaces | Location point views can use UI border presentation; card point views need a world-space ring/outline presentation around the mesh circle. |

## Phase 1 Design

| Artifact | Purpose |
| -------- | ------- |
| [data-model.md](./data-model.md) | Defines Condition, Target, Treatment, rule entities, state ownership, validation, and update rules. |
| [contracts/visual-modifier-contract.md](./contracts/visual-modifier-contract.md) | Defines rule inputs, outputs, colors, and acceptance samples. |
| [quickstart.md](./quickstart.md) | Defines implementation and verification workflow. |

## Implementation Approach

| Area | Plan |
| ---- | ---- |
| Rule representation | Add `VisualModificationRule`, `VisualModificationCondition`, `VisualModificationTarget`, and `VisualModificationTreatment` concepts, plus compact active modifier state on point view roots. |
| Target marking | Add marker components to point view circle/background children so rule Targets can be resolved without relying on entity names. |
| Ability outline rule | Condition: the linked card's `CardInstanceStateModel.zone` is `CardZoneModel::Location` with a non-zero active ability delta. Target: that card power point view's circle/background. Treatment: gold outline. Use 015 adapters only where current rendered point views still expose hand-index state. |
| Leading score outline rule | Condition: a location total is strictly higher than the paired local/opponent total. Target: the leading location total point view's circle/background. Treatment: white outline. |
| Presentation sync | Apply each active Treatment to its resolved Target while preserving text color, point value, and 015-derived hidden/revealed view behavior. |
| Testing | Add pure rule tests first, then ECS sync tests for card ability state and location leading/tie transitions. |

## Post-Design Constitution Check

| Check | Status | Notes |
| ----- | ------ | ----- |
| Source remains scoped | ✅ | Proposed future changes stay in focused runtime component/system files and point view bundle updates. |
| Desktop/browser parity addressed | ✅ | No platform-specific dependency is proposed; WebGPU-compatible Bevy primitives are expected. |
| Aspect-ratio-safe layout addressed | ✅ | Outlines are attached to existing point view children and do not introduce independent placement. |
| Data changes explicit | ✅ | Modifier state is transient runtime ECS state only. |
| Framework constraints recorded | ✅ | UI/world-space point view presentation differences and 015 card-state integration are part of the contract. |

## Complexity Tracking

No constitution violations require justification.
