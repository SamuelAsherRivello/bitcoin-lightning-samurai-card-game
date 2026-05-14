# Implementation Plan: DeckScreen Meta Game UI

**Branch**: `018-deck-screen` | **Date**: 2026-05-13 | **Spec**: [spec.md](./spec.md)  
**Input**: Feature specification from `specs/018-deck-screen/spec.md`

## Summary

Implement the DeckScreen mockups as a meta-game screen composed of persistent `AppScene` plus `DeckScene`. Add a reusable top navigation model/view, mounted only on DeckScreen for this feature, plus deck selection, selected deck editor, library/shop tabs, and a DeckScreen-specific fullscreen card modal.

## Technical Context

**Language/Version**: Rust in the existing Bevy workspace  
**Primary Dependencies**: Existing Bevy ECS runtime, Bevy UI/egui integration, card/deck models; no new dependency planned  
**Storage**: Existing persisted `PlayerDeckCollectionModel`; no destructive migration planned  
**Testing**: `scripts/other/RunTests.ps1`; targeted model/system tests under `bevy/crates/game/src/tests/runtime/`; visual desktop check through app run or AI runtime screenshot workflow  
**Target Platform**: Windows desktop and browser WebGPU parity  
**Project Type**: Bevy ECS game runtime under `bevy/crates/game`  
**Performance Goals**: Keep deck/library/shop derivation linear over displayed entries; avoid per-frame allocation-heavy recomputation outside state changes  
**Constraints**: Preserve gameplay state, active match deck/hand behavior, screen terminology, aspect-ratio-safe layout, and existing deck persistence  
**Scale/Scope**: Reusable top navigation concepts mounted on DeckScreen only, deck selection, editor, tabs, modal, local persistence, tests, and visual verification notes

## Constitution Check

| Check | Status | Notes |
| ----- | ------ | ----- |
| Active spec and repo guidance | ✅ | Plan follows `specs/018-deck-screen/spec.md`, constitution 1.6.1, and AGENTS.md. |
| Source locations | ✅ | Runtime changes belong under `bevy/crates/game/src/runtime`; tests under `bevy/crates/game/src/tests/runtime`. |
| Bevy template reference | ✅ | Future implementation must inspect `bevy/crates/template-crate`. |
| Rust naming conventions | ✅ | New modules use lowercase `snake_case`. |
| One primary runtime concept per file | ✅ | Planned concepts split top navigation, DeckScreen model, view/bundles, and systems. |
| HUMAN/AI comments | ✅ | Required for every new or changed primary runtime item. |
| Runtime system naming | ✅ | Planned systems use names such as `top_navigation_update_system` and `deck_screen_update_system`. |
| Scene/Model/View naming | ✅ | `DeckScreen` is conceptual; `DeckScene` is the child scene; models hold data; views render UI. |
| Visible feedback | ✅ | Selected nav, selected tabs, modal dimming, disabled actions, and immediate list updates. |
| Storage constraints | ✅ | Reuses existing local persisted deck collection. |
| Browser-visible verification | ✅ | Visual layout can be checked in existing desktop/browser workflows. |
| Aspect-ratio-safe layout | ✅ | Top nav and DeckScreen UI are safe-area bounded. |

## Project Structure

### Documentation

```text
specs/018-deck-screen/
├── spec.md
├── plan.md
├── research.md
├── data-model.md
├── quickstart.md
├── contracts/
│   └── deck-screen-ui-contract.md
└── tasks.md
```

### Source Code

```text
bevy/crates/game/src/runtime/
├── bundles/
│   ├── screen_bundle.rs
│   ├── top_navigation_view_bundle.rs
│   └── deck_screen_view_bundle.rs
├── components/
│   ├── top_navigation_component.rs
│   ├── deck_screen_component.rs
│   └── mod.rs
├── resources/
│   ├── top_navigation_model.rs
│   ├── deck_screen_model.rs
│   └── mod.rs
├── scenes/
│   ├── deck_scene.rs
│   └── mod.rs
└── systems/
    ├── top_navigation_update_system.rs
    ├── deck_screen_update_system.rs
    └── mod.rs
```

**Structure Decision**: Keep the reusable top navigation in general runtime model/component/bundle/system files, but only mount and activate it from `DeckScene` during this feature. Keep DeckScreen state in DeckScreen-specific resources and systems. Do not mutate active gameplay deck/hand/slot state.

## Phase 0 Research

Research is complete in [research.md](./research.md).

| Decision | Outcome |
| -------- | ------- |
| Top navigation | Reusable model/view, mounted only on DeckScreen now. |
| Screen-local model | Add `DeckScreenModel` for mode/tab/modal state. |
| Persistence | Reuse `PlayerDeckCollectionModel`. |
| Modal | Keep separate from gameplay selected-card modal. |
| Shop | Implement an empty shop state; defer shop contents and purchase execution. |

## Phase 1 Design

| Artifact | Purpose |
| -------- | ------- |
| [data-model.md](./data-model.md) | Defines top navigation and DeckScreen state models. |
| [contracts/deck-screen-ui-contract.md](./contracts/deck-screen-ui-contract.md) | Defines navigation, layout, modal, action, persistence, and QA contracts. |
| [quickstart.md](./quickstart.md) | Captures implementation and verification workflow. |

## Implementation Approach

| Area | Plan |
| ---- | ---- |
| Top navigation | Add reusable destination model, component markers, view bundle, and update system; mount in DeckScene only. |
| State | Add `DeckScreenModel` with mode, selected deck, tab, and modal state. |
| Data derivation | Derive deck summaries and editable card lists from `PlayerDeckCollectionModel` plus `CardModelRegistry`. |
| Layout | Replace current DeckScene single-card preview with top nav, deck selection, and editor layouts. |
| Tabs | Add Library/Shop tab visuals and input. |
| Modal | Add DeckScreen overlay, large card preview, action rail, input blocking, and Back close. |
| Persistence | Move owned entries between deck and library through existing deck collection storage. |
| Tests | Add model and system tests for top nav, derivation, modal, and persistence. |

## Post-Design Constitution Check

| Check | Status | Notes |
| ----- | ------ | ----- |
| Source remains scoped | ✅ | Runtime and test changes stay under Bevy game crate. |
| Desktop/browser parity addressed | ✅ | No platform-specific dependency proposed. |
| Aspect-ratio-safe layout addressed | ✅ | Top nav, grids, and modal are safe-area bounded. |
| Data changes explicit | ✅ | Existing deck collection persistence is reused. |
| Framework constraints recorded | ✅ | Modal input capture, Bevy UI layering, and navigation reuse are recorded. |

## Complexity Tracking

No constitution violations require justification.

## Deck View Constraint

| Requirement | Detail |
| ----------- | ------ |
| DeckViewBundle | Implementation MUST create a DeckViewBundle that renders a deck tile using the existing card back asset and the deck name only. |
