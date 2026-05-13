# Implementation Plan: Opponent Modes and Two-Player Resolution

**Branch**: `014-opponent` | **Date**: 2026-05-12 | **Spec**: [spec.md](./spec.md)  
**Input**: Feature specification from `specs/014-opponent/spec.md`

## Summary

Introduce two-player GameView state with a user-facing mode button, final `Status:` text above that mode button, per-player controllers, independent 12-card copies of the same master deck, per-player hands/readiness, hidden CPU Brain controllers for CPU players, hidden current-turn placements, end-of-turn reveal, top and bottom location slots, CPU-vs-CPU autoplay from turn `1/6` through final winner status, and final three-location winner resolution after turn `6/6`. The implementation should extend the current Bevy ECS runtime models (`GameRoundModel`, `GameDeckModel`, `GameHandModel`, `CardSlotBoardModel`, `PointModel`) rather than create a parallel gameplay stack.

## Technical Context

**Language/Version**: Rust, using the repository Bevy workspace and existing Rust toolchain  
**Primary Dependencies**: Existing Bevy runtime, `bevy_aspect_ratio_mask`, `bevy_persistent`, `fastrand`; no runtime generative AI dependency  
**Storage**: Active match state is transient; the existing player deck concept supplies the current 12-card master deck that is copied independently for each player; the last selected match mode is saved to disk as a user preference  
**Testing**: `scripts/other/RunTests.ps1`; targeted Rust tests under `bevy/crates/game/src/tests/runtime/`  
**Target Platform**: Windows desktop and browser WebGPU parity  
**Project Type**: Bevy ECS game runtime within `bevy/crates/game`  
**Performance Goals**: CPU Brain moves and readiness decisions are paced by 0.5 to 1 second delays; seeded CPU Brain tests are deterministic; human turns have no timer  
**Constraints**: Keep CPU Brain hidden from user-facing labels; use existing labels `Human versus CPU` and `CPU versus CPU`; keep GameView controls and card slots inside the aspect-ratio-safe HUD/game view; keep CPU-owned cards passive to mouse hover, drag affordance, and cursor-facing rotation effects  
**Scale/Scope**: Exactly two players, exactly two modes, exactly one controller per player (`PlayerController` or `CpuController`), exactly one CPU Brain level (`CpuBrainLevel = 1`), three shared locations, six-turn match flow, current-turn placement reveal at end of turn

## Constitution Check

*GATE: Must pass before Phase 0 research. Re-check after Phase 1 design.*

| Check | Status | Notes |
| ----- | ------ | ----- |
| Active spec, constitution, and repo-local guidance | ✅ | Plan follows `specs/014-opponent/spec.md`, constitution 1.6.1, and AGENTS.md. |
| Project source/assets/scripts/docs locations | ✅ | Runtime code remains under `bevy/crates/game/src/runtime`; tests remain under `bevy/crates/game/src/tests/runtime`; no new runtime assets required. |
| Bevy template reference | ✅ | `bevy/crates/template-crate` was inspected for folder/test conventions. |
| Rust naming conventions | ✅ | Planned modules use lowercase `snake_case`; primary items use `Model`, `System`, and controller naming where appropriate. |
| One primary runtime concept per changed file | ✅ | New model files should separate match mode, player match state, CPU Brain, and winner state if added. |
| HUMAN/AI purpose comments | ✅ | Required for all new or changed primary runtime items. |
| Runtime system function names | ✅ | Planned systems use names such as `cpu_brain_update_system`, `game_control_update_system`, and `match_resolution_update_system`. |
| Scene/Model/View naming | ✅ | This feature extends GameView and runtime models; no new Scene is planned. |
| Theme asset organization | ✅ | No theme-owned assets are introduced. |
| Visible feedback | ✅ | Mode, readiness, and winner states are visible through GameView controls/result presentation, including final `Status:` text above Mode; CPU Brain internals remain hidden. |
| Browser localStorage / native DB constraints | ✅ | No schema, database, browser SQLite, or OPFS change. |
| Browser-visible verification path | ✅ | Use project scripts after implementation; document any browser verification blocker. |
| Aspect-ratio-safe layout | ✅ | Mode button, Restart, Undo, and top/bottom slots must derive from the safe GameView/HUD layout. |
| Framework constraints documented | ✅ | Bevy ECS state and system ordering risks are captured below. |

## Project Structure

### Documentation (this feature)

```text
specs/014-opponent/
├── plan.md
├── research.md
├── data-model.md
├── quickstart.md
├── contracts/
│   └── gameview-opponent-ui.md
└── tasks.md
```

### Source Code (repository root)

```text
bevy/crates/game/src/runtime/
├── components/
│   ├── game_control_component.rs
│   └── mod.rs
├── resources/
│   ├── card_slot_model.rs
│   ├── game_round_model.rs
│   ├── mod.rs
│   ├── point_model.rs
│   └── [new opponent/match/brain model files as needed]
├── systems/
│   └── mod.rs
└── plugins/
    └── mod.rs

bevy/crates/game/src/tests/runtime/
├── resources/
│   ├── card_slot_model_tests.rs
│   ├── game_round_model_tests.rs
│   ├── point_model_tests.rs
│   └── [new opponent/match/brain model tests as needed]
└── systems/
    ├── card_gesture_update_system_tests.rs
    └── systems_tests.rs
```

**Structure Decision**: Implement within the existing `bevy/crates/game` runtime. Prefer model-level tests for match mode, CPU Brain move selection, readiness, and winner resolution, with system tests only where Bevy interactions, buttons, or UI labels need coverage.

## Phase 0 Research

Research is complete in [research.md](./research.md). Key decisions:

| Decision | Outcome |
| -------- | ------- |
| CPU terminology | Use hidden `CPU Brain` for authored game-code controller logic; do not expose Brain labels in the UI. |
| Controller split | Each player has one controller; `PlayerController` dispatches human input choices and `CpuController` dispatches CPU Brain choices to shared game logic. |
| CPU Brain level | Support only `CpuBrainLevel = 1`; future levels can extend the model without changing visible mode labels. |
| CPU move policy | Level 1 attempts to win, prefers evaluated moves that improve victory chances, uses seeded randomness when multiple acceptable legal moves are available, and paces every move or readiness decision by 0.5 to 1 second. |
| CPU Brain knowledge | Brain may know its hand, open locations and abilities, and revealed slots on both sides; it cannot inspect unrevealed deck cards or opposing current-turn hidden placements. |
| Readiness | A turn advances only after both players mark Next; human has no timer. |
| Hidden placements | Current-turn placed cards are hidden to the opposing controller/player until both players mark Next; then they reveal and stay face up. |
| Winner ties | Final match result cannot be a draw; use deterministic tiebreaking where existing scoring can draw. |

## Phase 1 Design

Design artifacts are complete:

| Artifact | Purpose |
| -------- | ------- |
| [data-model.md](./data-model.md) | Defines match mode, player controllers, CPU Brain knowledge, placement visibility, readiness, slots, scoring, and transitions. |
| [contracts/gameview-opponent-ui.md](./contracts/gameview-opponent-ui.md) | Defines user-facing GameView mode/readiness/winner UI contract. |
| [quickstart.md](./quickstart.md) | Defines verification workflow and expected behavior. |

## Implementation Approach

| Area | Plan |
| ---- | ---- |
| Mode control | Add `GameControlAction::Mode` and update GameView control spawning so Mode appears above Restart with `Mode:` plus the active label, with final `Status:` text above Mode. |
| Mode persistence | Load the saved selected mode at startup, default to `Human versus CPU` when absent, and save the selected mode to disk whenever the player changes it. |
| Player state | Replace or wrap single near-player deck/hand/readiness state with a two-player transient match model where each player starts from an independent copy of the same 12-card master deck. |
| Controllers | Route human input and CPU choices through `PlayerController` and `CpuController` into shared game logic rather than separate rule paths. |
| Slots | Reuse `CardSlotSide::LocalPlayer` and `CardSlotSide::Opponent`; generalize placement helpers so CPU Brain can place into opponent/top slots and CPU-versus-CPU can place for both sides. |
| CPU Brain | Add hidden CPU Brain model/controller state with `CpuBrainLevel::Level1` and injectable random seed; schedule a system that chooses legal affordable moves from the permitted knowledge view with the goal of winning, uses seeded randomness among acceptable moves, paces each move or readiness decision by 0.5 to 1 second, never dispatches Undo, and marks readiness when exhausted. |
| Visibility | Add placement visibility state so current-turn placements are private/face down to opponents, reveal at turn end, and remain revealed afterward. CPU-owned rendered cards remain passive and do not respond to mouse hover, drag affordance, or cursor-facing rotation. |
| Turn flow | Change End Turn/Next handling from immediate round advance to near-player readiness; advance only when both player readiness flags are set, then reveal current-turn placements before the next turn. In `CPU versus CPU`, both controllers must continue paced choices and readiness automatically until final winner status without human input. |
| Restart/mode change | Reset both players' transient decks, hands, slots, readiness, pending CPU Brain actions, round, and winner state. |
| Winner resolution | Extend existing `point_model` scoring or a match result model so round-six resolution always returns near or far winner, never draw. |
| UI feedback | Show visible mode, turn/readiness, and final `Status:` winner text above the mode button while keeping CPU Brain details hidden. |

## Post-Design Constitution Check

| Check | Status | Notes |
| ----- | ------ | ----- |
| Source remains scoped | ✅ | Planned files are all under `bevy/crates/game/src/runtime` and tests under `bevy/crates/game/src/tests/runtime`. |
| Desktop/browser parity addressed | ✅ | No platform-specific runtime dependency; browser verification remains required after implementation. |
| Aspect-ratio-safe layout addressed | ✅ | Mode button and top/bottom slots must use existing safe-area GameView layout constants/models. |
| Data changes explicit | ✅ | Selected mode persistence is explicit; no destructive data changes are planned. |
| Framework constraints recorded | ✅ | Bevy ECS scheduling/order risks documented in research and implementation approach. |

## Complexity Tracking

No constitution violations require justification.
