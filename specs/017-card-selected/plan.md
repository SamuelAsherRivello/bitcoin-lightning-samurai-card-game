# Implementation Plan: Card Selected Modal Backdrop

**Branch**: `017-card-selected` | **Date**: 2026-05-13 | **Spec**: [spec.md](./spec.md)  
**Input**: Feature specification from `specs/017-card-selected/spec.md`

## Summary

Extend card selection from local hand/local placed cards to any front-facing stationary rendered card, keep existing drag-threshold behavior, and add a selected-card modal backdrop that darkens and blocks lower interactions. Fix the current point-view text layering issue by making selected inspection suppress or correctly order non-selected card point overlays from the separate point text render layer.

## Technical Context

**Language/Version**: Rust in the existing Bevy workspace  
**Primary Dependencies**: Existing Bevy ECS runtime, Bevy UI/egui integration already present; no new dependency planned  
**Storage**: Transient runtime state only; no persistence or schema change  
**Testing**: `scripts/other/RunTests.ps1`; targeted model/system tests under `bevy/crates/game/src/tests/runtime/`; desktop visual verification through `scripts/main/RunAppDesktop.ps1` or AI runtime screenshot workflow  
**Target Platform**: Windows desktop and browser WebGPU parity  
**Project Type**: Bevy ECS game runtime under `bevy/crates/game`  
**Performance Goals**: Keep selection hit testing linear over currently rendered card roots; avoid per-frame allocations for modal opacity and point overlay visibility  
**Constraints**: Preserve `CARD_GESTURE_DRAG_THRESHOLD`, existing selected inspection transform, aspect-ratio-safe GameScene layout, current drag placement behavior, current CPU reveal rules, and Scene/Model/View naming  
**Scale/Scope**: Rendered front-facing card selection across local/CPU/near/far hand and location cards, selected modal fade/input capture, and selected-card point overlay depth correctness

## Constitution Check

*GATE: Must pass before Phase 0 research. Re-check after Phase 1 design.*

| Check | Status | Notes |
| ----- | ------ | ----- |
| Active spec and repo guidance | ✅ | Plan follows `specs/017-card-selected/spec.md`, constitution 1.6.1, and AGENTS.md. |
| Source/assets/scripts/docs locations | ✅ | Planning artifacts stay under `specs/017-card-selected`; runtime changes belong under `bevy/crates/game/src/runtime`. |
| Bevy template reference | ✅ | Future implementation must inspect `bevy/crates/template-crate` before adding runtime files. |
| Rust naming conventions | ✅ | Proposed names use lowercase module paths and `Model`/`Component`/`System` suffixes where appropriate. |
| One primary runtime concept per file | ✅ | Likely new concepts are selectable card state, selected modal model/component, and modal systems. |
| HUMAN/AI purpose comments | ✅ | Required for every changed or new primary runtime item. |
| Runtime system naming | ✅ | Planned systems use names such as `card_selection_update_system`, `card_selected_modal_update_system`, and `card_point_overlay_selection_update_system`. |
| Scene/Model/View naming | ✅ | Modal state belongs in a model/resource; rendered backdrop and card presentation remain view concerns. |
| Theme asset organization | ✅ | No runtime assets are introduced. |
| Visible feedback | ✅ | Modal darkening is visible feedback for selected inspection. |
| Browser/native storage constraints | ✅ | No storage, localStorage, SQLite, or OPFS change. |
| Browser-visible verification path | ✅ | Plan includes desktop and browser/WebGPU parity; browser verification can use existing served-web flow if available. |
| Aspect-ratio-safe layout | ✅ | Selected transform and modal dimensions must derive from the aspect-ratio-safe GameScene/fullscreen layout as appropriate. |
| Framework constraints documented | ✅ | Separate 3D card and 2D point text render layers are documented as a key depth risk. |

## Project Structure

### Documentation (this feature)

```text
specs/017-card-selected/
├── spec.md
├── plan.md
├── research.md
├── data-model.md
├── quickstart.md
└── contracts/
    └── card-selected-modal-contract.md
```

### Source Code (repository root)

```text
bevy/crates/game/src/runtime/
├── components/
│   ├── card_gesture_component.rs
│   ├── [future card_selection_component.rs]
│   └── mod.rs
├── resources/
│   ├── card_gesture_model.rs
│   ├── card_instance_state_model.rs
│   ├── card_slot_model.rs
│   ├── [future selected_card_modal_model.rs]
│   └── mod.rs
└── systems/
    ├── card_gesture_animation_system.rs
    ├── card_gesture_update_system.rs
    ├── mod.rs
    └── [future card_selected_modal_update_system.rs]

bevy/crates/game/src/tests/runtime/
├── components/
├── resources/
└── systems/
```

**Structure Decision**: Keep this as a focused Bevy runtime interaction/rendering feature. Reuse existing gesture logic for local draggable cards, add a render-facing selectable affordance for all card roots, and keep modal fade/input capture as selected-inspection state rather than durable gameplay state.

## Phase 0 Research

Research is complete in [research.md](./research.md). Key decisions:

| Decision | Outcome |
| -------- | ------- |
| Selectable affordance | Add or extend explicit render-facing selectable state instead of relying only on local `hand_index` state. |
| Drag threshold | Preserve `PointerGestureModel` and `CARD_GESTURE_DRAG_THRESHOLD = 8.0`. |
| Modal progress | Add explicit 0.5 second modal fade/progress because selected transform interpolation has no fixed duration today. |
| Input capture | Pair visual dimming with an input-capture layer so lower UI/card interactions cannot fire. |
| Point text depth | Fix via selected-card-aware point text visibility/order because `CARD_POINT_TEXT_RENDER_LAYER` uses a separate 2D camera. |

## Phase 1 Design

| Artifact | Purpose |
| -------- | ------- |
| [data-model.md](./data-model.md) | Defines selectable card state, selected modal model, modal layer, and transitions. |
| [contracts/card-selected-modal-contract.md](./contracts/card-selected-modal-contract.md) | Defines eligibility, pointer, visual, and verification contracts. |
| [quickstart.md](./quickstart.md) | Captures implementation notes and verification workflow. |

## Implementation Approach

| Area | Plan |
| ---- | ---- |
| Eligibility | Add a selectable affordance/component derived from front-face visibility and stationary movement state for local, CPU, near, far, hand, location, and other screen card roots. |
| Click/drag | Keep existing press/move/release threshold path; route release-before-threshold into selected inspection for selectable targets. |
| Selected modal state | Add a resource/model for selected entity, source transform, target transform, fade elapsed, max opacity 0.5, and dismiss intent. |
| Modal rendering | Add fullscreen black modal layer at the correct render/input order; opacity progresses from 0 to 0.5 over 0.5 seconds. |
| Input blocking | Ensure modal capture runs before lower card gesture, hover, drop target, game controls, and egui/UI actions can react. |
| Dismissal | Modal backdrop click returns the card to source; selected card click does not dismiss. |
| Point overlays | Update `update_card_point_text2d_overlay_system` or adjacent point overlay visibility logic so non-selected `CardPointTextView` entities cannot draw over selected inspection. |
| Tests | Add pure model tests for selectable/modal state and system tests for modal click dismissal, selected-card click no-op, and point overlay suppression. |

## Post-Design Constitution Check

| Check | Status | Notes |
| ----- | ------ | ----- |
| Source remains scoped | ✅ | Future changes stay under `bevy/crates/game/src/runtime` and tests under `bevy/crates/game/src/tests/runtime`. |
| Desktop/browser parity addressed | ✅ | No platform-specific dependency is proposed; input and rendering must be checked on Windows desktop and browser WebGPU when practical. |
| Aspect-ratio-safe layout addressed | ✅ | Selected target and modal visual sizing must derive from existing GameScene layout helpers. |
| Data changes explicit | ✅ | Changes are transient runtime model/component state only. |
| Framework constraints recorded | ✅ | Separate card mesh and point text render layers are recorded as the main ordering risk. |

## Complexity Tracking

No constitution violations require justification.
