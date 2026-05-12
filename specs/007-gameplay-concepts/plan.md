# Implementation Plan: Gameplay Concepts

**Branch**: `007-gameplay-concepts` | **Date**: 2026-05-12 | **Spec**: `specs/007-gameplay-concepts/spec.md`
**Input**: Feature specification from `specs/007-gameplay-concepts/spec.md`

## Summary

Extend the current `GameView` from a static gameplay-facing scene into a local six-round play loop. The implementation should preserve the persistent `AppScene` and current `GameView` layout, keep the lower-right End Turn button, add lower-left Restart and energy-aware Undo controls, introduce runtime models for the near player's 12-card deck, energy-sorted deal selection, hand, round energy, current-round move history, restart state, same-round placed-card mobility, locked prior-round placed cards, and three open/closed locations with text and effective-energy abilities, and integrate those models with existing card gesture placement so energy, undo, manual return-to-hand, location effects, and hand recentering stay deterministic.

## Technical Context

| Item | Decision |
| ---- | -------- |
| Language/Version | Rust 2024 workspace |
| Primary Dependencies | Bevy 0.18.1, existing workspace crates, `bevy_aspect_ratio_mask`, existing `bevy_tweening` usage for card transform animation |
| Storage | N/A; deck order, hand, round, energy, undo, and placed-card state are runtime match state and reset on restart |
| Testing | `scripts/other/RunTests.ps1`; targeted `cargo test -p bevy-card-game` may be used during iteration |
| Target Platform | Windows desktop and browser WebGPU |
| Project Type | Bevy ECS desktop/browser game prototype |
| Performance Goals | Frame-cheap over a fixed local loop: 12 deck cards, energy-sorted deal selection over remaining cards, up to 6 dealt cards with the initial deck, 3 locations, and 12 local placement slots |
| Constraints | Keep runtime work under `bevy/crates/game/src/runtime/`; use `bevy/crates/template-crate` as the local reference; derive visible 2D and 3D positions from the aspect-ratio-safe `GameView`; keep `AppScene` persistent; preserve `DeckBuilderScene`; do not implement CPU turns, scoring, card abilities, full location ownership, or additional location ability types beyond the defined effective-energy modifiers |
| Scale/Scope | One near human player, one randomized 12-card deck, six rounds, fixed requested deal and energy schedules, exact energy-match deal eligibility, current-round undo, manual return-to-hand for cards placed this round, prior-round placed-card lock, restart, three open/closed locations, Fortress Gate and Bamboo Crossing effective-energy modifiers, and existing local hand-to-location gesture path |

## Constitution Check

| Gate | Status | Notes |
| ---- | ------ | ----- |
| Active spec and repo guidance followed | ✅ | Plan follows `007-gameplay-concepts`, constitution 1.6.1, and repo-local `AGENTS.md` guidance |
| Source, assets, scripts, docs, and tests stay in approved locations | ✅ | Runtime code belongs under `bevy/crates/game/src/runtime/`; no new scripts or non-runtime docs are required |
| Template crate reference | ✅ | `bevy/crates/template-crate` remains the reference for Bevy folders, representative files, asset folders, and Rust coding standards |
| Rust naming and path casing | ✅ | New modules should use lowercase `snake_case`; no uppercase `Bevy/Crates/...` paths |
| One primary runtime concept per file | ✅ | Planned files separate round/deck/hand models, UI controls, and round update systems |
| Required `HUMAN:` and `AI:` comments | ✅ | Any changed or new primary runtime item must include the two-line purpose comment immediately above the item |
| Runtime system naming | ✅ | New systems should use names such as `game_round_update_system`, `game_deck_deal_system`, `game_undo_update_system`, and `game_restart_update_system` |
| Scene/Model/View naming | ✅ | Runtime state uses `Model`; presentation and control surfaces use `View`; `GameView` remains the active gameplay presentation under `AppScene` |
| Theme asset organization | ✅ | No new theme-owned assets are required; existing world/location/card assets stay where already defined |
| Visible user feedback | ✅ | Card deal animation when eligible, skipped deal when no matching-energy card remains, energy text, disabled Undo state, hand recentering, hand insertion gap shifts, location open/closed text, location ability text, locked placed-card behavior, and restart state provide visible feedback |
| Desktop and browser WebGPU parity | ✅ | Quickstart includes desktop and browser verification paths; unsupported target blockers must be documented if encountered |
| Aspect-ratio-safe layout | ✅ | Lower-left controls, lower-right End Turn, hand cards, and deal origins derive from the aspect-ratio-safe `GameView` |
| Framework-specific constraints documented | ✅ | Bevy UI, pointer gestures, resource-driven runtime state, and animation ownership are documented in research |

## Project Structure

```text
specs/007-gameplay-concepts/
├── plan.md
├── research.md
├── data-model.md
├── quickstart.md
├── contracts/
│   └── gameview-round-flow-contract.md
└── tasks.md              # Created by /speckit-tasks, not by this plan

bevy/crates/game/src/runtime/
├── components/
│   ├── mod.rs
│   └── game_control_component.rs
├── resources/
│   ├── mod.rs
│   ├── game_deck_model.rs
│   ├── game_hand_model.rs
│   ├── game_location_model.rs
│   └── game_round_model.rs
├── scenes/
│   ├── mod.rs
│   └── game_view_scene.rs
├── systems/
│   ├── mod.rs
│   ├── game_deck_deal_system.rs
│   ├── game_location_effect_system.rs
│   ├── game_round_update_system.rs
│   ├── game_undo_update_system.rs
│   └── game_restart_update_system.rs
└── bundles/
    ├── mod.rs
    ├── card_view_bundle.rs
    └── location_view_bundle.rs

scripts/
└── other/
    └── RunTests.ps1
```

**Structure Decision**: Keep the work inside the existing game crate because the feature depends on `GameView`, runtime resources, Bevy UI controls, hand card transforms, location presentation, and existing gesture placement. Add focused runtime models for deck/hand/round/location state and focused systems for energy-sorted deal selection, round advancement, open-location effective-energy effects, undo, manual current-round return-to-hand, placed-card locking, and restart. Extend the existing `game_view_scene.rs` setup path instead of creating a new scene.

## Complexity Tracking

| Violation | Why Needed | Simpler Alternative Rejected Because |
| --------- | ---------- | ------------------------------------ |
| N/A | N/A | N/A |

## Phase 0 Research

See `specs/007-gameplay-concepts/research.md`.

## Phase 1 Design

See `specs/007-gameplay-concepts/data-model.md`, `specs/007-gameplay-concepts/contracts/gameview-round-flow-contract.md`, and `specs/007-gameplay-concepts/quickstart.md`.

## Post-Design Constitution Check

| Gate | Status | Notes |
| ---- | ------ | ----- |
| No unresolved clarifications | ✅ | The spec resolves button placement, round schedule, energy schedule, deck composition, energy-matching deal selection, undo scope, same-round placed-card mobility, prior-round placed-card locking, restart behavior, location open rounds, location text, and the initial location effective-energy abilities |
| Target parity documented | ✅ | Quickstart covers desktop and browser WebGPU verification, plus visual checks for aspect-ratio-safe layout |
| Constitution implementation standards reflected | ✅ | Planned runtime concepts use Model/View naming, focused files, required purpose comments, and `[domain]_[schedule]_system` system names |
| Verification documented | ✅ | Unit tests, targeted game crate tests, desktop run, browser run, and optional AI runtime screenshots are documented |
