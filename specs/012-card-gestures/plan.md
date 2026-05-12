# Implementation Plan: Card Gestures

**Branch**: `012-card-gestures` | **Date**: 2026-05-11 | **Spec**: `specs/012-card-gestures/spec.md`
**Input**: Feature specification from `specs/012-card-gestures/spec.md`

## Summary

Replace the current hand-card click-to-DeckBuilderScene behavior with in-game pointer gestures for card inspection and placement. The implementation should keep `GameView` active, leave the existing Deck Builder view implementation unchanged, add a focused card gesture model for click, selected-inspection, drag, and placement states, animate card movement with Bevy Tweening, derive all visible positions from the aspect-ratio-safe `GameView`, and support local-player placement into the twelve bottom-side location slots while rejecting opponent, populated, and off-board targets.

## Technical Context

| Item | Decision |
| ---- | -------- |
| Language/Version | Rust 2024 workspace |
| Primary Dependencies | Bevy 0.18.1, existing workspace crates, and `bevy_tweening = "0.15"` for card transform/scale animation |
| Storage | N/A; gesture and slot state are runtime match/view state, not persisted user data |
| Testing | `scripts/other/RunTests.ps1` for workspace tests; targeted `cargo test -p bevy-card-game` may be used during iteration |
| Target Platform | Windows desktop and browser WebGPU |
| Project Type | Bevy ECS desktop/browser game prototype |
| Performance Goals | Pointer gesture updates and slot hit-testing should be frame-cheap over a small fixed board: one active gesture, three locations, and twenty-four slots |
| Constraints | Keep runtime work under `bevy/crates/game/src/runtime/`; use `bevy/crates/template-crate` as the proper reference for Bevy crate folders, representative files, asset folders, and Rust coding standards; keep card gesture state as `Model` concepts and card/slot presentation as `View` concepts; remove the GameView hand-card path that opens `DeckBuilderScene`; do not modify the existing DeckBuilderScene implementation; accept that users will no longer have a path to reach it after this feature; keep all visible positions derived from the aspect-ratio-safe game view; do not implement full turn, energy, reveal, CPU, or scoring resolution |
| Scale/Scope | Local player's hand cards, one active selected/dragged card at a time, three location areas, four local slots and four opponent slots per location, twelve valid direct-placement slots total |

## Constitution Check

| Gate | Status | Notes |
| ---- | ------ | ----- |
| Active spec and repo guidance followed | ✅ | Plan follows `012-card-gestures`, constitution 1.6.0, and repo-local AGENTS guidance |
| Source, assets, scripts, docs, and tests stay in approved locations | ✅ | Runtime code belongs under `bevy/crates/game/src/runtime/`; scripts stay under `scripts/`; no new runtime assets are required |
| Rust naming and path casing | ✅ | New modules should use lowercase `snake_case`; no `Bevy/Crates/...` paths |
| One primary runtime concept per file | ✅ | Planned files separate gesture model, slot model, gesture components, tween/lens logic, and update systems |
| Template crate reference | ✅ | `bevy/crates/template-crate` is the proper local reference for Bevy folders, representative files, asset folders, and Rust coding standards |
| Required `HUMAN:` and `AI:` comments | ✅ | Any changed or new primary runtime items must include the two-line purpose comment immediately above the item |
| Runtime system naming | ✅ | New systems should use names such as `card_gesture_update_system`, `card_gesture_animation_system`, and `card_slot_update_system` |
| Scene/Model/View naming | ✅ | Gesture and slot data use `Model`; visible placement/inspection concepts use `View`; `GameView` remains a view under persistent `AppScene` |
| Theme asset organization | ✅ | No new theme-owned card, location, or world assets are required |
| Visible user feedback | ✅ | Gesture feedback is visible through card movement, enlargement, drag preview, snapping, and invalid-drop return |
| Desktop and browser WebGPU parity | ✅ | Pointer and animation behavior must be verified on desktop and browser WebGPU, or blocked with a documented reason |
| Aspect-ratio-safe layout | ✅ | Hand, selected inspection, drag preview, and slot positions derive from the existing safe `GameView` layout rather than raw window pixels |
| Framework-specific API constraints documented | ✅ | Bevy pointer observers, picking events, transform/UI interaction boundaries, and Bevy Tweening animation ownership are documented in research |

## Project Structure

```text
specs/012-card-gestures/
├── plan.md
├── research.md
├── data-model.md
├── quickstart.md
├── contracts/
│   └── card-gestures-contract.md
└── tasks.md              # Created by /speckit-tasks, not by this plan

bevy/crates/game/src/runtime/
├── components/
│   ├── mod.rs
│   └── card_gesture_component.rs
├── resources/
│   ├── mod.rs
│   ├── card_gesture_model.rs
│   └── card_slot_model.rs
├── systems/
│   ├── mod.rs
│   ├── card_gesture_update_system.rs
│   └── card_gesture_animation_system.rs
└── plugins/
    └── mod.rs            # Wire tweening/plugin setup through existing app composition if needed

bevy/crates/game/Cargo.toml
scripts/
└── other/
    └── RunTests.ps1
```

**Structure Decision**: Keep card gesture implementation in the game crate because it depends on `GameView`, hand cards, Deck Builder inspection pose parity, and gameplay slot semantics. Use `bevy/crates/template-crate` as the proper reference for Bevy crate folders, representative files, asset folders, and Rust coding standards. Prefer focused model/component/system files instead of growing the already-large aggregate runtime system module. Add `bevy_tweening` only to the game crate and keep gesture hit-testing deterministic in model code so slot legality can be tested without rendering. Do not edit the Deck Builder view implementation; only remove GameView user navigation into it.

## Complexity Tracking

| Violation | Why Needed | Simpler Alternative Rejected Because |
| --------- | ---------- | ------------------------------------ |
| N/A | N/A | N/A |

## Phase 0 Research

See `specs/012-card-gestures/research.md`.

## Phase 1 Design

See `specs/012-card-gestures/data-model.md`, `specs/012-card-gestures/contracts/card-gestures-contract.md`, and `specs/012-card-gestures/quickstart.md`.

## Post-Design Constitution Check

| Gate | Status | Notes |
| ---- | ------ | ----- |
| No unresolved clarifications | ✅ | The spec resolves click behavior, drag threshold semantics, local-only slots, invalid drops, and scope exclusions |
| Target parity documented | ✅ | Quickstart includes desktop and browser WebGPU verification, including pointer/touch-compatible gesture checks |
| Constitution implementation standards reflected | ✅ | Planned runtime concepts use Model/View naming, focused files, required purpose comments, and `[domain]_[schedule]_system` system names |
| Verification documented | ✅ | Unit tests, targeted app checks, desktop run, and browser WebGPU verification are documented in quickstart |
