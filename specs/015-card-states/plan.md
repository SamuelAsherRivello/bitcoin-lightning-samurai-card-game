# Implementation Plan: Card View State Model

**Branch**: `015-card-states` | **Date**: 2026-05-13 | **Spec**: [spec.md](./spec.md)  
**Input**: Feature specification from `specs/015-card-states/spec.md`

## Summary

Analyze the current `CardViewBundle` state surface and propose a clearer model that separates card identity, durable gameplay location, face/reveal policy, transient interaction, and render pose. Current state is spread across `CardViewBundle`, `CardFaceLayer`, `CardFlipState`, `CardStateModel`, `CardGestureModel`, `CardSlotBoardModel`, CPU placement marker components, and `PlacementVisibilityModel`; the proposed model consolidates the durable relationships into a per-card-instance model and derives view state for rendering.

## Technical Context

**Language/Version**: Rust in the existing Bevy workspace  
**Primary Dependencies**: Existing Bevy ECS runtime; no new dependency planned  
**Storage**: Transient runtime state only; no persistence or schema change  
**Testing**: `scripts/other/RunTests.ps1`; targeted model tests under `bevy/crates/game/src/tests/runtime/resources/` if implemented  
**Target Platform**: Windows desktop and browser WebGPU parity  
**Project Type**: Bevy ECS game runtime under `bevy/crates/game`  
**Performance Goals**: Keep state lookup O(1) or linear only over current small hand/slot/card vectors; avoid repeated cross-resource searches for card identity where practical  
**Constraints**: Preserve aspect-ratio-safe GameScene transforms, current card face visibility behavior, local drag semantics, current-turn hidden CPU/opponent placements, and existing Scene/Model/View naming  
**Scale/Scope**: Planning and data model for deck, hand, selected, dragging, local/opponent location slots, current-turn hidden/revealed face policy, and current same-turn/locked placement rules

## Constitution Check

*GATE: Must pass before Phase 0 research. Re-check after Phase 1 design.*

| Check | Status | Notes |
| ----- | ------ | ----- |
| Active spec and repo guidance | ✅ | Plan follows `specs/015-card-states/spec.md`, constitution 1.6.1, and AGENTS.md. |
| Source/assets/scripts/docs locations | ✅ | Planning artifacts stay under `specs/015-card-states`; any later runtime implementation belongs under `bevy/crates/game/src/runtime`. |
| Bevy template reference | ✅ | Future implementation should inspect `bevy/crates/template-crate` before adding runtime files. |
| Rust naming conventions | ✅ | Proposed model names use `Model` and lowercase module paths. |
| One primary runtime concept per file | ✅ | Proposed future files split instance state, view state, and transitions. |
| HUMAN/AI purpose comments | ✅ | Required for future primary runtime items. |
| Runtime system naming | ✅ | Future systems should use names such as `card_state_update_system` and `card_view_state_sync_system`. |
| Scene/Model/View naming | ✅ | The proposal keeps durable card data in models and render concerns in views. |
| Theme asset organization | ✅ | No runtime assets are introduced. |
| Visible feedback | ✅ | Existing selected, dragging, returning, drop hint, and reveal feedback are preserved by contract. |
| Browser/native storage constraints | ✅ | No storage, database, localStorage, SQLite, or OPFS change. |
| Browser-visible verification path | ✅ | If implemented, verify with the same GameScene gesture and opponent reveal workflows. |
| Aspect-ratio-safe layout | ✅ | Proposed view state derives poses from existing safe GameScene layout helpers. |
| Framework constraints documented | ✅ | Bevy ECS ownership and cross-resource derivation risks are captured below. |

## Project Structure

### Documentation (this feature)

```text
specs/015-card-states/
├── spec.md
├── plan.md
├── research.md
├── data-model.md
├── quickstart.md
└── contracts/
    └── card-state-contract.md
```

### Source Code (repository root)

```text
bevy/crates/game/src/runtime/
├── bundles/
│   └── card_view_bundle.rs
├── components/
│   ├── card_gesture_component.rs
│   └── mod.rs
├── resources/
│   ├── card_gesture_model.rs
│   ├── card_slot_model.rs
│   ├── opponent_match_model.rs
│   └── [future card_instance_state_model.rs]
└── systems/
    ├── card_gesture_animation_system.rs
    ├── card_gesture_update_system.rs
    └── mod.rs

bevy/crates/game/src/tests/runtime/
├── resources/
│   └── [future card_instance_state_model_tests.rs]
└── systems/
    └── card_gesture_update_system_tests.rs
```

**Structure Decision**: Keep this as a model cleanup inside the existing Bevy game runtime. `CardViewBundle` should stay a render-root constructor; the new durable state should live in resources/models and sync into render components.

## Phase 0 Research

Research is complete in [research.md](./research.md). Key decisions:

| Decision | Outcome |
| -------- | ------- |
| Treat state as axes | Model face, zone, ownership, interaction, reveal, and pose as separate axes rather than one overloaded enum. |
| Preserve `CardViewBundle` role | Keep the bundle stateless except for initial transform/visibility and `CardView` marker. |
| Use card instance IDs | Use stable instance identity instead of hand indices as the primary key; keep hand order as a relationship. |
| Derive render state | Derive visible face and pose from instance zone plus active interaction rather than storing separate render truth. |

## Phase 1 Design

| Artifact | Purpose |
| -------- | ------- |
| [data-model.md](./data-model.md) | Documents current state hierarchy and proposed model. |
| [contracts/card-state-contract.md](./contracts/card-state-contract.md) | Defines legal combinations and transition contract. |
| [quickstart.md](./quickstart.md) | Defines review and future verification steps. |

## Implementation Approach

| Area | Plan |
| ---- | ---- |
| Identity | Introduce a stable `CardInstanceId` for each card drawn or placed in a match. |
| Durable state | Replace hand-index-centered `CardStateModel` with `CardInstanceStateModel` containing owner, card model ID, zone, placement, movement lock, and reveal policy. |
| Interaction state | Keep only one active `CardInteractionModel` for pressed/selected/dragging/returning/settling state and reference a `CardInstanceId`. |
| View state | Add or derive `CardViewStateModel` for visible face, pose, z band, input affordance, and layer visibility. |
| Migration | Add adapters so existing `GameHandModel`, `CardSlotBoardModel`, and `OpponentMatchModel` can be read into the new shape before removing older duplicated state. |
| Tests | Start with pure model transition tests before changing gesture systems. |

## Post-Design Constitution Check

| Check | Status | Notes |
| ----- | ------ | ----- |
| Source remains scoped | ✅ | Proposed future changes stay in `bevy/crates/game/src/runtime` and tests under `bevy/crates/game/src/tests/runtime`. |
| Desktop/browser parity addressed | ✅ | No platform-specific dependency is proposed. |
| Aspect-ratio-safe layout addressed | ✅ | Pose derivation continues to use existing GameScene helpers and slot rects. |
| Data changes explicit | ✅ | State changes are transient runtime model changes only. |
| Framework constraints recorded | ✅ | Bevy ECS resource/component ownership is part of the proposed hierarchy. |

## Complexity Tracking

No constitution violations require justification.
